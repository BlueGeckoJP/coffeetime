use chrono::{DateTime, Local, Utc};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};

use crate::utils::get_utc_start_of_today;

pub async fn get_today_data(database_url: &str) -> anyhow::Result<Vec<shared_entities::Model>> {
    let db = Database::connect(database_url).await?;

    let start_of_day_utc = get_utc_start_of_today();

    let results = shared_entities::Entity::find()
        .filter(shared_entities::Column::Timestamp.gt(start_of_day_utc))
        .all(&db)
        .await?;

    Ok(results)
}

pub async fn get_period_data(
    database_url: &str,
    start_timestamp: DateTime<Local>,
    end_timestamp: DateTime<Local>,
) -> anyhow::Result<Vec<shared_entities::Model>> {
    let db = Database::connect(database_url).await?;

    let utc_start = start_timestamp.with_timezone(&Utc);
    let utc_end = end_timestamp.with_timezone(&Utc);

    let results = shared_entities::Entity::find()
        .filter(shared_entities::Column::Timestamp.gte(utc_start))
        .filter(shared_entities::Column::Timestamp.lte(utc_end))
        .all(&db)
        .await?;

    Ok(results)
}
