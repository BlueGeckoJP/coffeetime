use sea_orm::{Database, EntityTrait, QuerySelect};

pub async fn get_database_data(
    database_url: &str,
    limit: Option<u64>,
) -> anyhow::Result<Vec<shared_entities::Model>> {
    let db = Database::connect(database_url).await?;

    let data = shared_entities::Entity::find()
        .limit(limit)
        .all(&db)
        .await?;

    Ok(data)
}
