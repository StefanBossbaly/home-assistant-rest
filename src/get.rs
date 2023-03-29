use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde_derive::Deserialize;

use crate::deserialize::{
    deserialize_date, deserialize_datetime, deserialize_optional_datetime,
    deserialize_optional_state_enum,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Request {
    pub endpoint: String,
    pub query: Vec<(String, String)>,
}

pub trait Parameters {
    fn into_request(self) -> Result<Request>;
}

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
pub struct ApiStatusResponse {
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
pub struct ConfigResponse {
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

pub type EventsResponse = Vec<EventEntry>;

#[derive(Deserialize, Debug)]
pub struct EventEntry {
    pub event: String,
    pub listener_count: i32,
}

pub type ServicesResponse = Vec<ServiceEntry>;

#[derive(Deserialize, Debug)]
pub struct ServiceEntry {
    pub domain: String,
    pub services: Vec<String>,
}

#[derive(Default)]
pub struct HistoryParams {
    pub filter_entity_ids: Option<Vec<String>>,
    pub start_time: Option<DateTime<FixedOffset>>,
    pub end_time: Option<DateTime<FixedOffset>>,
    pub minimal_response: bool,
    pub no_attributes: bool,
    pub significant_changes_only: bool,
}

impl Parameters for HistoryParams {
    fn into_request(self) -> Result<Request> {
        let mut query = Vec::new();
        let mut endpoint = String::from("/api/history/period");

        if let Some(start_time) = self.start_time {
            endpoint.push_str(format!("/{}", start_time.to_rfc3339()).as_str());
        }

        if let Some(ref filter_entity_ids) = self.filter_entity_ids {
            query.push(("filter_entity_ids".to_owned(), filter_entity_ids.join(",")));
        }

        if let Some(ref end_time) = self.end_time {
            query.push(("end_time".to_owned(), end_time.to_rfc3339()));
        }

        if self.minimal_response {
            query.push(("minimal_response".to_owned(), "true".to_owned()));
        }

        if self.no_attributes {
            query.push(("no_attributes".to_owned(), "true".to_owned()));
        }

        if self.significant_changes_only {
            query.push(("significant_changes_only".to_owned(), "true".to_owned()));
        }

        Ok(Request { endpoint, query })
    }
}

pub type HistoryResponse = Vec<Vec<HistoryEntry>>;

#[derive(Deserialize, Debug)]
pub struct HistoryEntry {
    #[serde(default)]
    pub attributes: Option<HashMap<String, serde_json::Value>>,

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

#[derive(Default)]
pub struct LogbookParams {
    pub entity: Option<String>,
    pub start_time: Option<DateTime<FixedOffset>>,
    pub end_time: Option<DateTime<FixedOffset>>,
}

impl Parameters for LogbookParams {
    fn into_request(self) -> Result<Request> {
        let mut query = Vec::new();
        let mut endpoint = String::from("/api/logbook");

        if let Some(start_time) = self.start_time {
            endpoint.push_str(format!("/{}", start_time.to_rfc3339()).as_str());
        }

        if let Some(ref entity) = self.entity {
            query.push(("entity".to_owned(), entity.to_owned()));
        }

        if let Some(ref end_time) = self.end_time {
            query.push(("end_time".to_owned(), end_time.to_rfc3339()));
        }

        Ok(Request { endpoint, query })
    }
}

pub type LogbookResponse = Vec<LogbookEntry>;

#[derive(Deserialize, Debug)]
pub struct LogbookEntry {
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

pub type StatesResponse = Vec<StateEntry>;

#[derive(Deserialize, Debug)]
pub struct StateEntry {
    pub attributes: HashMap<String, serde_json::Value>,
    pub entity_id: String,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_changed: DateTime<FixedOffset>,

    #[serde(deserialize_with = "deserialize_optional_state_enum")]
    pub state: Option<StateEnum>,
}

#[derive(Deserialize, Debug)]
pub struct StatesEntityResponse {
    pub attributes: HashMap<String, serde_json::Value>,
    pub entity_id: String,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_changed: DateTime<FixedOffset>,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_updated: DateTime<FixedOffset>,

    #[serde(deserialize_with = "deserialize_optional_state_enum")]
    pub state: Option<StateEnum>,
}

#[derive(Default)]
pub struct CalendarsParams {
    pub entity_id: String,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}

impl Parameters for CalendarsParams {
    fn into_request(self) -> Result<Request> {
        let mut query = Vec::new();
        let endpoint = format!("/api/calendars/{}", &self.entity_id);

        query.push((
            "start".to_owned(),
            self.start_time
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        ));
        query.push((
            "end".to_owned(),
            self.end_time
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        ));

        Ok(Request { endpoint, query })
    }
}

pub type CalendarsResponse = Vec<CalendarEntry>;

#[derive(Deserialize, Debug)]
pub struct CalendarEntry {
    pub entity_id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub enum DateVariant {
    #[serde(
        rename(deserialize = "dateTime"),
        deserialize_with = "deserialize_datetime"
    )]
    DateTime(DateTime<FixedOffset>),

    #[serde(rename(deserialize = "date"), deserialize_with = "deserialize_date")]
    Date(NaiveDate),
}

impl std::cmp::Eq for DateVariant {}

impl std::cmp::PartialEq for DateVariant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DateVariant::DateTime(x), DateVariant::DateTime(y)) => *x == *y,
            (DateVariant::Date(x), DateVariant::Date(y)) => *x == *y,
            _ => false,
        }
    }
}

pub type CalendarsEntityResponse = Vec<CalendarsEntityEntry>;

#[derive(Deserialize, Debug)]
pub struct CalendarsEntityEntry {
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
