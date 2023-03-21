use std::fmt;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::de;

const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn deserialize_optional_datetime<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error> {
    deserializer.deserialize_option(OptionDateTimeRfc3339Visitor)
}

pub fn deserialize_datetime<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error> {
    deserializer.deserialize_string(DateTimeRfc3339Visitor)
}

pub fn deserialize_date<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<NaiveDate, D::Error> {
    deserializer.deserialize_string(NaiveDateVistor)
}

struct OptionDateTimeRfc3339Visitor;

impl<'a> de::Visitor<'a> for OptionDateTimeRfc3339Visitor {
    type Value = Option<DateTime<FixedOffset>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null or a rfc3339 encoded data time string")
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D: de::Deserializer<'a>>(self, d: D) -> Result<Self::Value, D::Error> {
        Ok(Some(d.deserialize_str(DateTimeRfc3339Visitor)?))
    }
}

struct DateTimeRfc3339Visitor;

impl<'a> de::Visitor<'a> for DateTimeRfc3339Visitor {
    type Value = DateTime<FixedOffset>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a rfc3339 encoded data time string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match DateTime::parse_from_rfc3339(value) {
            Ok(date_time) => Ok(date_time),
            Err(e) => Err(E::custom(format!(
                "Error {} parsing timestamp {}",
                e, value
            ))),
        }
    }
}

struct NaiveDateVistor;

impl<'a> de::Visitor<'a> for NaiveDateVistor {
    type Value = NaiveDate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a trivally encoded date string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match NaiveDate::parse_from_str(value, DATE_FORMAT) {
            Ok(date) => Ok(date),
            Err(e) => Err(E::custom(format!(
                "Error {} parsing timestamp {}",
                e, value
            ))),
        }
    }
}
