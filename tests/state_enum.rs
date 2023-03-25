use homeassistant_rest_rs::{
    deserialize::{deserialize_optional_state_enum, deserialize_state_enum},
    responses::StateEnum,
};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct TestStruct {
    #[serde(deserialize_with = "deserialize_state_enum")]
    state: StateEnum,
}

#[derive(Deserialize, Debug)]
struct OptionTestStruct {
    #[serde(deserialize_with = "deserialize_optional_state_enum")]
    state: Option<StateEnum>,
}

#[test]
fn test_bool_true() {
    let value: TestStruct = serde_json::from_str(r#"{"state": true}"#).unwrap();
    assert_eq!(value.state, StateEnum::Boolean(true));
}

#[test]
fn test_bool_false() {
    let value: TestStruct = serde_json::from_str(r#"{"state": false}"#).unwrap();
    assert_eq!(value.state, StateEnum::Boolean(false));
}

#[test]
fn test_interger_1() {
    let value: TestStruct = serde_json::from_str(r#"{"state": 123}"#).unwrap();
    assert_eq!(value.state, StateEnum::Integer(123));
}

#[test]
fn test_interger_2() {
    let value: TestStruct = serde_json::from_str(r#"{"state": -123}"#).unwrap();
    assert_eq!(value.state, StateEnum::Integer(-123));
}

#[test]
fn test_decimal_1() {
    let value: TestStruct = serde_json::from_str(r#"{"state": 3.141519}"#).unwrap();
    assert_eq!(value.state, StateEnum::Decimal(3.141519));
}

#[test]
fn test_decimal_2() {
    let value: TestStruct = serde_json::from_str(r#"{"state": -3.141519}"#).unwrap();
    assert_eq!(value.state, StateEnum::Decimal(-3.141519));
}

#[test]
fn test_string_1() {
    let value: TestStruct = serde_json::from_str(r#"{"state": "Hello World!"}"#).unwrap();
    assert_eq!(value.state, StateEnum::String("Hello World!".to_owned()));
}

#[test]
fn test_string_2() {
    let value: TestStruct = serde_json::from_str(r#"{"state": "0xDEADBEEF"}"#).unwrap();
    assert_eq!(value.state, StateEnum::String("0xDEADBEEF".to_owned()));
}

#[test]
fn test_bool_string_true() {
    let value: TestStruct = serde_json::from_str(r#"{"state": "true"}"#).unwrap();
    assert_eq!(value.state, StateEnum::Boolean(true));
}

#[test]
fn test_bool_string_false() {
    let value: TestStruct = serde_json::from_str(r#"{"state": "false"}"#).unwrap();
    assert_eq!(value.state, StateEnum::Boolean(false));
}

#[test]
fn test_interger_string_1() {
    let value: TestStruct = serde_json::from_str(r#"{"state": "123"}"#).unwrap();
    assert_eq!(value.state, StateEnum::Integer(123));
}

#[test]
fn test_interger_string_2() {
    let value: TestStruct = serde_json::from_str(r#"{"state": "-123"}"#).unwrap();
    assert_eq!(value.state, StateEnum::Integer(-123));
}

#[test]
fn test_option_null() {
    let value: OptionTestStruct = serde_json::from_str(r#"{"state": null}"#).unwrap();
    assert_eq!(value.state, Option::None);
}

#[test]
fn test_option_some() {
    let value: OptionTestStruct = serde_json::from_str(r#"{"state": "Hi Again"}"#).unwrap();
    assert_eq!(value.state, Some(StateEnum::String("Hi Again".to_owned())));
}
