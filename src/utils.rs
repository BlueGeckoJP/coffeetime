use chrono::{DateTime, TimeZone, Utc};

pub fn get_utc_start_of_today() -> DateTime<Utc> {
    let local_now = chrono::Local::now();
    let start_of_day = local_now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let local_start_of_day = chrono::Local.from_local_datetime(&start_of_day).unwrap();
    local_start_of_day.with_timezone(&Utc)
}
