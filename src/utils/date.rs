use chrono::{Local, TimeZone};

pub fn format_timestamp(timestamp: i64, format: &str) -> String {
    if let Some(dt) = Local.timestamp_millis_opt(timestamp).single() {
        dt.format(format).to_string()
    } else {
        String::from("Invalid date")
    }
}
