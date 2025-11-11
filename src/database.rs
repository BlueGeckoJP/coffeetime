use chrono::{Local, TimeZone, Utc};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};

pub async fn get_today_data(database_url: &str) -> anyhow::Result<Vec<shared_entities::Model>> {
    let db = Database::connect(database_url).await?;

    let local_now = Local::now();
    let start_of_day = local_now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let local_start_of_day = Local.from_local_datetime(&start_of_day).unwrap();
    let start_of_day_utc = local_start_of_day.with_timezone(&Utc);

    let results = shared_entities::Entity::find()
        .filter(shared_entities::Column::Timestamp.gt(start_of_day_utc))
        .all(&db)
        .await?;

    Ok(results)
}
