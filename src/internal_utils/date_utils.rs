use chrono::{DateTime, Duration, Utc};

pub fn check_date(created_at: &str, hour: i64) -> bool {
    let parsed_date = DateTime::parse_from_str(created_at, "%a %b %d %H:%M:%S %z %Y")
        .expect("Failed to parse date");

    let now = Utc::now();
    let hour_ago = now - Duration::hours(hour);

    return parsed_date > hour_ago;
}
