use chrono::{DateTime, Utc};
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, Database, DbConn, EntityTrait, ExprTrait,
    QueryFilter, Schema,
};
use shared_entities::entity::session::{
    ActiveModel as SessionActiveModel, Column as SessionColumn, Entity as SessionEntity,
};
use shared_entities::entity::sleep_period::{
    ActiveModel as SleepPeriodActiveModel, Column as SleepPeriodColumn, Entity as SleepPeriodEntity,
};

async fn setup_database(database_url: &str) -> anyhow::Result<DbConn> {
    let db = Database::connect(database_url).await?;
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(
        schema
            .create_table_from_entity(SessionEntity)
            .if_not_exists(),
    )
    .await?;
    db.execute(
        schema
            .create_table_from_entity(SleepPeriodEntity)
            .if_not_exists(),
    )
    .await?;

    Ok(db)
}

pub async fn create_session(database_url: &str, start_time: DateTime<Utc>) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;

    let session = SessionActiveModel {
        start_time: ActiveValue::Set(start_time),
        end_time: ActiveValue::Set(None),
        ..Default::default()
    };
    session.insert(&db).await?;
    Ok(())
}

pub async fn close_active_session(database_url: &str) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;

    // find the active session ( end_time is NULL )
    if let Some(session) = SessionEntity::find()
        .filter(Expr::col(SessionColumn::EndTime).is_null())
        .one(&db)
        .await?
    {
        let mut active_model: SessionActiveModel = session.into();
        active_model.end_time = ActiveValue::Set(Some(Utc::now()));
        active_model.update(&db).await?;
    }

    Ok(())
}

pub async fn start_sleep_period(
    database_url: &str,
    sleep_start: DateTime<Utc>,
) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;

    // find the active session ( end_time is NULL )
    if let Some(session) = SessionEntity::find()
        .filter(Expr::col(SessionColumn::EndTime).is_null())
        .one(&db)
        .await?
    {
        let sleep_period = SleepPeriodActiveModel {
            session_id: ActiveValue::Set(session.id),
            sleep_start: ActiveValue::Set(sleep_start),
            sleep_end: ActiveValue::Set(None),
            ..Default::default()
        };
        sleep_period.insert(&db).await?;
    }

    Ok(())
}

pub async fn end_sleep_period(database_url: &str, sleep_end: DateTime<Utc>) -> anyhow::Result<()> {
    let db = setup_database(database_url).await?;

    // find the active sleep period ( sleep_end is NULL )
    if let Some(period) = SleepPeriodEntity::find()
        .filter(Expr::col(SleepPeriodColumn::SleepEnd).is_null())
        .one(&db)
        .await?
    {
        let mut active_model: SleepPeriodActiveModel = period.into();
        active_model.sleep_end = ActiveValue::Set(Some(sleep_end));
        active_model.update(&db).await?;
    }

    Ok(())
}
