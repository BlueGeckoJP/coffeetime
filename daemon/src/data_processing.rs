use crate::database;

pub async fn exec_start(database_url: &str) -> anyhow::Result<()> {
    database::db_write_exec_start(database_url).await?;

    Ok(())
}

pub async fn exec_stop(database_url: &str) -> anyhow::Result<()> {
    database::db_write_exec_stop(database_url).await?;

    Ok(())
}

pub async fn before_sleep(database_url: &str) -> anyhow::Result<()> {
    database::db_write_before_sleep(database_url).await?;

    Ok(())
}

pub async fn after_sleep(database_url: &str) -> anyhow::Result<()> {
    database::db_write_after_sleep(database_url).await?;

    Ok(())
}
