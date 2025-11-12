use chrono::{Duration, Utc};
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
