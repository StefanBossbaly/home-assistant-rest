use std::fmt;

use chrono::{DateTime, FixedOffset};
use serde::de;

pub fn deserialize_optional_datetime<'a, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: de::Deserializer<'a>,
{
    deserializer.deserialize_option(OptionDateTimeRfc3339Visitor)
}

pub fn deserialize_datetime<'a, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: de::Deserializer<'a>,
{
    deserializer.deserialize_option(DateTimeRfc3339Visitor)
}

struct OptionDateTimeRfc3339Visitor;

impl<'de> de::Visitor<'de> for OptionDateTimeRfc3339Visitor {
    type Value = Option<DateTime<FixedOffset>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null or a rfc3339 encoded data time string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Some(d.deserialize_str(DateTimeRfc3339Visitor)?))
    }
}

struct DateTimeRfc3339Visitor;

pub fn deserialize<'a, D>(d: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: de::Deserializer<'a>,
{
    d.deserialize_str(DateTimeRfc3339Visitor)
}

impl<'a> de::Visitor<'a> for DateTimeRfc3339Visitor {
    type Value = DateTime<FixedOffset>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a rfc3339 encoded data time string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match DateTime::parse_from_rfc3339(value) {
            Ok(date_time) => Ok(date_time),
            Err(e) => Err(E::custom(format!("Error {} parsing timestamp {}", e, value))),
        }
    }
}