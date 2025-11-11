use crate::database;
use chrono::Utc;
use shared_entities::EventType;

pub fn today_total_screen_time() -> anyhow::Result<String> {
    let data = tokio::runtime::Runtime::new()?
        .block_on(database::get_today_data("sqlite://daemon/test.db"))?;

    let mut total_duration = chrono::Duration::seconds(0);
    let mut i = 0;

    while i < data.len() {
        if data[i].event_type == EventType::ExecStart {
            let start_time = data[i].timestamp;

            // Find the corresponding ExecStop event
            let mut found_stop = false;
            for (j, event) in data.iter().enumerate().skip(i + 1) {
                if event.event_type == EventType::ExecStop {
                    let stop_time = event.timestamp;
                    total_duration += stop_time - start_time;
                    found_stop = true;
                    i = j + 1;
                    break;
                }
            }

            // If no corresponding ExecStop event is found, assume the session is ongoing
            if !found_stop {
                let now = Utc::now();
                total_duration += now - start_time;
                break;
            }
        } else {
            i += 1;
        }
    }

    let hours = total_duration.num_hours();
    let minutes = total_duration.num_minutes() % 60;

    Ok(format!("{:02}h {:02}m", hours, minutes))
}
