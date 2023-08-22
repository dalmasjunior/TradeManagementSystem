//! This module provides a function to convert a Unix timestamp to a NaiveDateTime
//!
//! The `timestamp_to_naive_date_time` function takes a Unix timestamp as input and returns a `NaiveDateTime` object.
//! The function utilizes the `chrono` crate to perform the conversion.
//!
//! # Examples
//!
//! ```
//! use chrono::{prelude::DateTime, NaiveDateTime};
//! use chrono::Utc;
//! use std::time::{UNIX_EPOCH, Duration};
//!
//! pub fn timestamp_to_naive_date_time(time: i64) -> NaiveDateTime {
//!     let d = UNIX_EPOCH + Duration::from_secs(time as u64);
//!     let datetime = DateTime::<Utc>::from(d);
//!     datetime.naive_utc()
//! }
//!
//! let unix_timestamp = 1629620736; // Example Unix timestamp
//! let naive_date_time = timestamp_to_naive_date_time(unix_timestamp);
//!
//! println!("Unix Timestamp: {}", unix_timestamp);
//! println!("Converted NaiveDateTime: {}", naive_date_time);
//! ```

use chrono::{prelude::DateTime, NaiveDateTime};
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

pub fn timestamp_to_naive_date_time(time: i64) -> NaiveDateTime {
    let d = UNIX_EPOCH + Duration::from_secs(time as u64);
    let datetime = DateTime::<Utc>::from(d);
    datetime.naive_utc()
}