use chrono::{DateTime, Duration, Utc};

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    return start + Duration::seconds(1_000_000_000)
}
