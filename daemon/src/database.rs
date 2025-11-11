use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, Database, DbConn, Schema};
use shared_entities::EventType;

async fn setup_database(database_url: &str) -> anyhow::Result<DbConn> {
    let db = Database::connect(database_url).await?;

    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let mut create_table_stmt = schema.create_table_from_entity(shared_entities::Entity);

    db.execute(create_table_stmt.if_not_exists()).await?;

    Ok(db)
}

async fn insert_event(db: &DbConn, event_type: EventType) -> anyhow::Result<()> {
    let data = shared_entities::ActiveModel {
        event_type: ActiveValue::Set(event_type),
        timestamp: ActiveValue::Set(chrono::Utc::now()),
        ..Default::default()
    };
    data.insert(db).await?;

    Ok(())
}

pub async fn db_write_exec_start(database_url: &str) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;
    insert_event(&db, EventType::ExecStart).await?;

    Ok(())
}

pub async fn db_write_exec_stop(database_url: &str) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;
    insert_event(&db, EventType::ExecStop).await?;

    Ok(())
}

pub async fn db_write_before_sleep(database_url: &str) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;
    insert_event(&db, EventType::BeforeSleep).await?;

    Ok(())
}

pub async fn db_write_after_sleep(database_url: &str) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;
    insert_event(&db, EventType::AfterSleep).await?;

    Ok(())
}
