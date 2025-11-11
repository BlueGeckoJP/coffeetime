use sea_orm::{ActiveModelTrait, ActiveValue, Database, DbConn};

use crate::entities::data::EventType;

pub async fn exec_start(database_url: &str) -> anyhow::Result<()> {
    let db: DbConn = Database::connect(database_url).await?;

    let data = crate::entities::data::ActiveModel {
        event_type: ActiveValue::Set(EventType::ExecStart),
        timestamp: ActiveValue::Set(chrono::Utc::now()),
        ..Default::default()
    };
    data.insert(&db).await?;

    Ok(())
}

pub async fn exec_stop(database_url: &str) -> anyhow::Result<()> {
    let db: DbConn = Database::connect(database_url).await?;

    let data = crate::entities::data::ActiveModel {
        event_type: ActiveValue::Set(EventType::ExecStop),
        timestamp: ActiveValue::Set(chrono::Utc::now()),
        ..Default::default()
    };
    data.insert(&db).await?;

    Ok(())
}

pub async fn before_sleep(database_url: &str) -> anyhow::Result<()> {
    let db: DbConn = Database::connect(database_url).await?;

    let data = crate::entities::data::ActiveModel {
        event_type: ActiveValue::Set(EventType::BeforeSleep),
        timestamp: ActiveValue::Set(chrono::Utc::now()),
        ..Default::default()
    };
    data.insert(&db).await?;

    Ok(())
}

pub async fn after_sleep(database_url: &str) -> anyhow::Result<()> {
    let db: DbConn = Database::connect(database_url).await?;

    let data = crate::entities::data::ActiveModel {
        event_type: ActiveValue::Set(EventType::AfterSleep),
        timestamp: ActiveValue::Set(chrono::Utc::now()),
        ..Default::default()
    };
    data.insert(&db).await?;

    Ok(())
}
