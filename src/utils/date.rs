use chrono::{prelude::DateTime, NaiveDateTime};
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

pub fn timestamp_to_naive_date_time(time: i64) -> NaiveDateTime {
    let d = UNIX_EPOCH + Duration::from_secs(time as u64);
    let datetime = DateTime::<Utc>::from(d);
    datetime.naive_utc()
}