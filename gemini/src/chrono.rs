use chrono::{DateTime, Duration, NaiveDateTime, Utc};

pub trait FromMilliseconds {
    fn from_milliseconds(ms: i64) -> Self;
}

impl FromMilliseconds for DateTime<Utc> {
    fn from_milliseconds(ms: i64) -> Self {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(Duration::milliseconds(ms).num_seconds(), 0),
            Utc,
        )
    }
}
