use std::fs;

use chrono::{DateTime, Duration, TimeZone, Utc};

pub fn get_utc_start_of_today() -> DateTime<Utc> {
    let local_now = chrono::Local::now();
    let start_of_day = local_now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let local_start_of_day = chrono::Local.from_local_datetime(&start_of_day).unwrap();
    local_start_of_day.with_timezone(&Utc)
}

fn get_uptime() -> Option<f64> {
    let file_content = fs::read_to_string("/proc/uptime").ok()?;
    let mut splitted = file_content.split_whitespace();
    let uptime_str = splitted.next()?;
    uptime_str.parse().ok()
}

pub fn get_humanized_uptime() -> String {
    let uptime = get_uptime();
    if let Some(uptime) = uptime {
        let duration = Duration::seconds(uptime as i64);
        let days = duration.num_days();
        let hours = duration.num_days() % 24;
        let minutes = duration.num_minutes() & 60;
        format!("{}d {}h {}m", days, hours, minutes)
    } else {
        String::from("-d -h -m")
    }
}
