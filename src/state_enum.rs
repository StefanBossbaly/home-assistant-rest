use std::fmt;

use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub enum StateEnum {
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
    String(String),
}

impl std::cmp::Eq for StateEnum {}

impl std::cmp::PartialEq for StateEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StateEnum::Integer(x), StateEnum::Integer(y)) => *x == *y,
            (StateEnum::Decimal(x), StateEnum::Decimal(y)) => *x == *y,
            (StateEnum::Boolean(x), StateEnum::Boolean(y)) => *x == *y,
            (StateEnum::String(x), StateEnum::String(y)) => *x == *y,
            _ => false,
        }
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

impl<'de> Deserialize<'de> for StateEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StateEnumVisitor)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_bool_true() {
        let value: StateEnum = serde_json::from_value(json!(true)).unwrap();
        assert_eq!(value, StateEnum::Boolean(true));
    }

    #[test]
    fn test_bool_false() {
        let value: StateEnum = serde_json::from_value(json!(false)).unwrap();
        assert_eq!(value, StateEnum::Boolean(false));
    }

    #[test]
    fn test_positive_integer() {
        let value: StateEnum = serde_json::from_value(json!(123)).unwrap();
        assert_eq!(value, StateEnum::Integer(123));
    }

    #[test]
    fn test_negative_integer() {
        let value: StateEnum = serde_json::from_value(json!(-123)).unwrap();
        assert_eq!(value, StateEnum::Integer(-123));
    }

    #[test]
    fn test_positive_decimal() {
        let value: StateEnum = serde_json::from_value(json!(3.141519)).unwrap();
        assert_eq!(value, StateEnum::Decimal(3.141519));
    }

    #[test]
    fn test_negative_decimal() {
        let value: StateEnum = serde_json::from_value(json!(-3.141519)).unwrap();
        assert_eq!(value, StateEnum::Decimal(-3.141519));
    }

    #[test]
    fn test_string_1() {
        let value: StateEnum = serde_json::from_value(json!("Hello World!")).unwrap();
        assert_eq!(value, StateEnum::String("Hello World!".to_owned()));
    }

    #[test]
    fn test_string_2() {
        let value: StateEnum = serde_json::from_value(json!("0xDEADBEEF")).unwrap();
        assert_eq!(value, StateEnum::String("0xDEADBEEF".to_owned()));
    }

    #[test]
    fn test_bool_string_true() {
        let value: StateEnum = serde_json::from_value(json!("true")).unwrap();
        assert_eq!(value, StateEnum::Boolean(true));
    }

    #[test]
    fn test_bool_string_false() {
        let value: StateEnum = serde_json::from_value(json!("false")).unwrap();
        assert_eq!(value, StateEnum::Boolean(false));
    }

    #[test]
    fn test_integer_string_1() {
        let value: StateEnum = serde_json::from_value(json!("123")).unwrap();
        assert_eq!(value, StateEnum::Integer(123));
    }

    #[test]
    fn test_integer_string_2() {
        let value: StateEnum = serde_json::from_value(json!("-123")).unwrap();
        assert_eq!(value, StateEnum::Integer(-123));
    }

    #[test]
    fn test_option_null() {
        let value: Option<StateEnum> = serde_json::from_value(json!(null)).unwrap();
        assert_eq!(value, Option::None);
    }

    #[test]
    fn test_option_some() {
        let value: Option<StateEnum> = serde_json::from_value(json!("Hi Again!")).unwrap();
        assert_eq!(value, Some(StateEnum::String("Hi Again!".to_owned())));
    }

    #[test]
    fn test_invalid_type_array() {
        let value: Result<StateEnum, _> = serde_json::from_value(json!([1, 2, 3]));
        assert!(value.is_err());
    }

    #[test]
    fn test_invalid_type_map() {
        let value: Result<StateEnum, _> = serde_json::from_value(json!({"Hello": "World"}));
        assert!(value.is_err());
    }
}
