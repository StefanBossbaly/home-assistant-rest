use std::fmt;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::de;

use crate::responses::StateEnum;

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

pub fn deserialize_optional_state_enum<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<StateEnum>, D::Error> {
    deserializer.deserialize_option(OptionStateEnumVisitor)
}

pub fn deserialize_state_enum<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<StateEnum, D::Error> {
    deserializer.deserialize_any(StateEnumVisitor)
}

struct OptionStateEnumVisitor;

impl<'a> de::Visitor<'a> for OptionStateEnumVisitor {
    type Value = Option<StateEnum>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null, bool, integer, decimal or string valu")
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D: de::Deserializer<'a>>(self, d: D) -> Result<Self::Value, D::Error> {
        Ok(Some(d.deserialize_str(StateEnumVisitor)?))
    }
}

struct StateEnumVisitor;

impl<'a> de::Visitor<'a> for StateEnumVisitor {
    type Value = StateEnum;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "bool, integer, decimal or string value")
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(StateEnum::Boolean(v))
    }

    fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v as i64))
    }

    fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v as i64))
    }

    fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v as i64))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v))
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v as i64))
    }

    fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v as i64))
    }

    fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v as i64))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(StateEnum::Integer(v as i64))
    }

    fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
        Ok(StateEnum::Decimal(v as f64))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(StateEnum::Decimal(v))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        // Attmmpt to parse bool first
        if let Ok(bool_value) = value.parse::<bool>() {
            return Ok(StateEnum::Boolean(bool_value));
        }

        // Next attempt to parse integer
        if let Ok(int_value) = value.parse::<i64>() {
            return Ok(StateEnum::Integer(int_value));
        }

        // Finally attempt to parse float
        if let Ok(decimal_value) = value.parse::<f64>() {
            return Ok(StateEnum::Decimal(decimal_value));
        }

        Ok(StateEnum::String(value))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        // Attempt to parse bool first
        if let Ok(bool_value) = value.parse::<bool>() {
            return Ok(StateEnum::Boolean(bool_value));
        }

        // Next attempt to parse integer
        if let Ok(int_value) = value.parse::<i64>() {
            return Ok(StateEnum::Integer(int_value));
        }

        // Finally attempt to parse float
        if let Ok(decimal_value) = value.parse::<f64>() {
            return Ok(StateEnum::Decimal(decimal_value));
        }

        Ok(StateEnum::String(value.to_owned()))
    }
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
