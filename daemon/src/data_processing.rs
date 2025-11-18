use crate::database;

pub async fn exec_start(database_url: &str) -> anyhow::Result<()> {
    database::create_session(database_url, chrono::Utc::now()).await?;

    Ok(())
}

pub async fn exec_stop(database_url: &str) -> anyhow::Result<()> {
    database::close_active_session(database_url).await?;

    Ok(())
}

pub async fn before_sleep(database_url: &str) -> anyhow::Result<()> {
    database::start_sleep_period(database_url, chrono::Utc::now()).await?;

    Ok(())
}

pub async fn after_sleep(database_url: &str) -> anyhow::Result<()> {
    database::end_sleep_period(database_url, chrono::Utc::now()).await?;

    Ok(())
}
