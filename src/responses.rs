use std::collections::HashMap;

use crate::deserialze::{deserialize_datetime, deserialize_optional_datetime};
use chrono::{DateTime, FixedOffset};
use serde_derive::Deserialize;

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
    pub state: String,
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
    pub state: String,
}
