use crate::database;

pub fn today_total_screen_time() -> anyhow::Result<String> {
    let rt = tokio::runtime::Runtime::new()?;
    let data = rt.block_on(database::get_today_active_screen("sqlite://daemon/test.db"))?;

    let hours = data.num_hours();
    let minutes = data.num_minutes() % 60;

    Ok(format!("{:02}h {:02}m", hours, minutes))
}

pub fn last_seven_days_screen_time_f64() -> anyhow::Result<Vec<f64>> {
    let rt = tokio::runtime::Runtime::new()?;
    let data = rt.block_on(database::get_last_seven_days_active_screen(
        "sqlite://daemon/test.db",
    ))?;

    let mut result = Vec::with_capacity(7);
    for duration in data {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        result.push(hours as f64 + minutes as f64 / 60.0);
    }

    Ok(result)
}
