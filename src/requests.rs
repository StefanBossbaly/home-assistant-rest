use crate::serialize::serialize_optional_datetime;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use serde_derive::Serialize;

pub struct Query<T: Serialize> {
    pub endpoint: String,
    pub query: T,
}

pub trait Queryable {
    type QueryType: Serialize;
    fn into_query(self) -> Query<Self::QueryType>;
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

#[derive(Serialize, Debug)]
pub struct HistoryQuery {
    pub filter_entity_ids: Option<Vec<String>>,

    #[serde(serialize_with = "serialize_optional_datetime")]
    pub end_time: Option<DateTime<FixedOffset>>,
    pub minimal_response: bool,
    pub no_attributes: bool,
    pub significant_changes_only: bool,
}

impl Queryable for HistoryParams {
    type QueryType = HistoryQuery;

    fn into_query(self) -> Query<Self::QueryType> {
        let mut endpoint = String::from("/api/history/period");

        if let Some(start_time) = self.start_time {
            endpoint.push_str(format!("/{}", start_time.to_rfc3339()).as_str());
        }

        let query = HistoryQuery {
            filter_entity_ids: self.filter_entity_ids,
            end_time: self.end_time,
            minimal_response: self.minimal_response,
            no_attributes: self.no_attributes,
            significant_changes_only: self.significant_changes_only,
        };

        Query { endpoint, query }
    }
}

#[derive(Default)]
pub struct LogbookParams {
    pub entity: Option<String>,
    pub start_time: Option<DateTime<FixedOffset>>,
    pub end_time: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize, Debug)]
pub struct LogbookQuery {
    pub entity: Option<String>,

    #[serde(serialize_with = "serialize_optional_datetime")]
    pub end_time: Option<DateTime<FixedOffset>>,
}

impl Queryable for LogbookParams {
    type QueryType = LogbookQuery;

    fn into_query(self) -> Query<Self::QueryType> {
        let mut endpoint = String::from("/api/logbook");

        if let Some(start_time) = self.start_time {
            endpoint.push_str(format!("/{}", start_time.to_rfc3339()).as_str());
        }

        let query = LogbookQuery {
            entity: self.entity.clone(),
            end_time: self.end_time,
        };

        Query { endpoint, query }
    }
}

pub struct CalendarEventParams {
    pub entity_id: String,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}
