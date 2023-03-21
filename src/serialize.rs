use chrono::{DateTime, FixedOffset};
use serde::Serializer;

#[allow(dead_code)]
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn serialize_optional_datetime<S: Serializer>(
    date: &Option<DateTime<FixedOffset>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match date {
        None => serializer.serialize_none(),
        Some(value) => serializer.serialize_some(value),
    }
}

pub fn serialize_datetime<S: Serializer>(
    date: &DateTime<FixedOffset>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&date.to_rfc3339())
}
