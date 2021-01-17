use chrono::{DateTime, Utc, Duration};

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(i64::from(1_000_000_000))
}
