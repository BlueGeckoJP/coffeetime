use chrono::{DateTime, Duration, Utc};
use sea_orm::{ColumnTrait, Database, EntityTrait, ModelTrait, QueryFilter};
use shared_entities::entity::session::{Column as SessionColumn, Entity as SessionEntity};
use shared_entities::entity::sleep_period::Entity as SleepPeriodEntity;

use crate::utils::get_utc_start_of_today;

pub async fn get_today_active_screen(database_url: &str) -> anyhow::Result<Duration> {
    let db = Database::connect(database_url).await?;
    let start_of_day = get_utc_start_of_today();

    let sessions = SessionEntity::find()
        .filter(SessionColumn::StartTime.gte(start_of_day))
        .all(&db)
        .await?;

    let mut total_screen_time = Duration::seconds(0);

    for session in sessions {
        let end_time = session.end_time.unwrap_or_else(Utc::now);
        let mut session_duration = end_time - session.start_time;

        // Subtract sleep periods within the session
        let sleep_periods = session.find_related(SleepPeriodEntity).all(&db).await?;

        for period in sleep_periods {
            let sleep_end = period.sleep_end.unwrap_or_else(Utc::now);
            let sleep_duration = sleep_end - period.sleep_start;
            session_duration -= sleep_duration;
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
            .filter(SessionColumn::StartTime.gte(day_start))
            .filter(SessionColumn::StartTime.lt(day_start + Duration::days(1)))
            .all(&db)
            .await?;

        let mut total_screen_time = Duration::seconds(0);

        for session in sessions {
            let end_time = session.end_time.unwrap_or_else(Utc::now);
            let mut session_duration = end_time - session.start_time;

            // Subtract sleep periods within the session
            let sleep_periods = session.find_related(SleepPeriodEntity).all(&db).await?;

            for period in sleep_periods {
                let sleep_end = period.sleep_end.unwrap_or_else(Utc::now);
                let sleep_duration = sleep_end - period.sleep_start;
                session_duration -= sleep_duration;
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
