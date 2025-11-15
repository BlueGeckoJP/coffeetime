use chrono::{DateTime, Duration, Utc};
use sea_orm::sea_query::Condition;
use sea_orm::{ColumnTrait, Database, EntityTrait, ModelTrait, QueryFilter};
use shared_entities::entity::session::{Column as SessionColumn, Entity as SessionEntity};
use shared_entities::entity::sleep_period::Entity as SleepPeriodEntity;

use crate::utils::get_utc_start_of_today;

pub async fn get_today_active_screen(database_url: &str) -> anyhow::Result<Duration> {
    let db = Database::connect(database_url).await?;
    let start_of_day = get_utc_start_of_today();
    let start_of_next_day = start_of_day + Duration::days(1);

    // Include sessions that either start today, or that end today
    // (including ongoing sessions that have not yet finished)
    let sessions = SessionEntity::find()
        .filter(
            Condition::all()
                .add(SessionColumn::StartTime.lt(start_of_next_day))
                .add(
                    Condition::any()
                        .add(SessionColumn::EndTime.is_null())
                        .add(SessionColumn::EndTime.gte(start_of_day)),
                ),
        )
        .all(&db)
        .await?;

    let mut total_screen_time = Duration::seconds(0);

    for session in sessions {
        let end_time = session.end_time.unwrap_or_else(Utc::now);

        // Clip session to today's interval (start_of_day, start_of_next_day)
        let overlap_start = if session.start_time < start_of_day {
            start_of_day
        } else {
            session.start_time
        };
        let overlap_end = if end_time > start_of_next_day {
            start_of_next_day
        } else {
            end_time
        };

        if overlap_end <= overlap_start {
            continue; // no overlap with today
        }

        let mut session_duration = overlap_end - overlap_start;

        let sleep_periods = session.find_related(SleepPeriodEntity).all(&db).await?;

        for period in sleep_periods {
            let sleep_start = period.sleep_start;
            let sleep_end = period.sleep_end.unwrap_or_else(Utc::now);

            // Clip sleep to the session's overlap interval
            let sleep_overlap_start = if sleep_start < overlap_start {
                overlap_start
            } else {
                sleep_start
            };
            let sleep_overlap_end = if sleep_end > overlap_end {
                overlap_end
            } else {
                sleep_end
            };

            if sleep_overlap_end > sleep_overlap_start {
                let sleep_duration = sleep_overlap_end - sleep_overlap_start;
                session_duration -= sleep_duration;
            }
        }

        total_screen_time += session_duration;
    }

    Ok(total_screen_time)
}

pub async fn get_last_seven_days_active_screen(
    database_url: &str,
) -> anyhow::Result<Vec<Duration>> {
    let db = Database::connect(database_url).await?;
    let start_of_day = get_utc_start_of_today();
    let each_start_of_day: Vec<DateTime<Utc>> = (0..7)
        .map(|days_ago| start_of_day - Duration::days(days_ago))
        .collect();
    let mut result = Vec::with_capacity(7);

    for day_start in each_start_of_day {
        let sessions = SessionEntity::find()
            .filter(
                Condition::all()
                    .add(SessionColumn::StartTime.lt(day_start + Duration::days(1)))
                    .add(
                        Condition::any()
                            .add(SessionColumn::EndTime.is_null())
                            .add(SessionColumn::EndTime.gte(day_start)),
                    ),
            )
            .all(&db)
            .await?;

        let mut total_screen_time = Duration::seconds(0);

        for session in sessions {
            let end_time = session.end_time.unwrap_or_else(Utc::now);
            let next_day_start = day_start + Duration::days(1);

            // Clip session to today's interval
            let overlap_start = if session.start_time < day_start {
                day_start
            } else {
                session.start_time
            };
            let overlap_end = if end_time > next_day_start {
                next_day_start
            } else {
                end_time
            };

            if overlap_end <= overlap_start {
                continue; // no overlap with this day
            }

            let mut session_duration = overlap_end - overlap_start;

            // Subtract sleep periods within the session
            let sleep_periods = session.find_related(SleepPeriodEntity).all(&db).await?;

            for period in sleep_periods {
                let sleep_start = period.sleep_start;
                let sleep_end = period.sleep_end.unwrap_or_else(Utc::now);

                // Clip sleep to the session's overlap interval
                let sleep_overlap_start = if sleep_start < overlap_start {
                    overlap_start
                } else {
                    sleep_start
                };
                let sleep_overlap_end = if sleep_end > overlap_end {
                    overlap_end
                } else {
                    sleep_end
                };

                if sleep_overlap_end > sleep_overlap_start {
                    let sleep_duration = sleep_overlap_end - sleep_overlap_start;
                    session_duration -= sleep_duration;
                }
            }

            total_screen_time += session_duration;
        }

        result.push(total_screen_time);
    }

    Ok(result)
}

pub fn get_db_path(database_path: Option<String>) -> String {
    let share_directory = dirs::home_dir().unwrap().join(".local/share/coffeetime");
    let default_db_path = share_directory.join("coffeetime.db");
    std::fs::create_dir_all(share_directory).unwrap();

    match database_path {
        Some(path) => format!("sqlite://{}", path),
        None => format!("sqlite://{}", default_db_path.to_string_lossy()),
    }
}
