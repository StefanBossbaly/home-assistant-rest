use std::collections::HashMap;

use crate::deserialize::{
    deserialize_date, deserialize_datetime, deserialize_optional_datetime,
    deserialize_optional_state_enum,
};
use chrono::{DateTime, FixedOffset, NaiveDate};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
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

#[derive(Deserialize, Debug)]
pub struct ApiStatus {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct UnitSystemConfig {
    pub length: String,
    pub mass: String,
    pub temperature: String,
    pub volume: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub components: Vec<String>,
    pub config_dir: String,
    pub elevation: i32,
    pub latitude: f32,
    pub location_name: String,
    pub longitude: f32,
    pub time_zone: String,
    pub unit_system: UnitSystemConfig,
    pub version: String,
    pub whitelist_external_dirs: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub event: String,
    pub listener_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    pub domain: String,
    pub services: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct History {
    #[serde(default)]
    pub attributes: Option<HashMap<String, String>>,

    #[serde(default)]
    pub entity_id: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub last_changed: Option<DateTime<FixedOffset>>,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub last_updated: Option<DateTime<FixedOffset>>,

    #[serde(deserialize_with = "deserialize_optional_state_enum")]
    pub state: Option<StateEnum>,
}

#[derive(Deserialize, Debug)]
pub struct Logbook {
    #[serde(default)]
    pub domain: Option<String>,
    pub entity_id: String,

    #[serde(default)]
    pub message: Option<String>,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub when: Option<DateTime<FixedOffset>>,
}

#[derive(Deserialize, Debug)]
pub struct State {
    pub attributes: HashMap<String, String>,
    pub entity_id: String,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_changed: DateTime<FixedOffset>,

    #[serde(deserialize_with = "deserialize_optional_state_enum")]
    pub state: Option<StateEnum>,
}

#[derive(Deserialize, Debug)]
pub struct Calendar {
    pub entity_id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub enum DateVariant {
    #[serde(
        rename(deserialize = "dateTime"),
        deserialize_with = "deserialize_datetime"
    )]
    DataTime(DateTime<FixedOffset>),
    #[serde(rename(deserialize = "date"), deserialize_with = "deserialize_date")]
    Date(NaiveDate),
}

#[derive(Deserialize, Debug)]
pub struct CalendarEvent {
    pub summary: String,

    pub start: DateVariant,
    pub end: DateVariant,

    #[serde(default)]
    pub location: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub uid: Option<String>,

    #[serde(default)]
    pub recurrence_id: Option<String>,

    #[serde(default)]
    pub rrule: Option<String>,
}
