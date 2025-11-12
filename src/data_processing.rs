use crate::database;

pub fn today_total_screen_time() -> anyhow::Result<String> {
    let rt = tokio::runtime::Runtime::new()?;
    let data = rt.block_on(database::get_today_active_screen("sqlite://daemon/test.db"))?;

    let hours = data.num_hours();
    let minutes = data.num_minutes() % 60;

    Ok(format!("{:02}h {:02}m", hours, minutes))
}
