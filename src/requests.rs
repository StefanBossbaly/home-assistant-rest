use std::collections::HashMap;

use crate::serialize::serialize_optional_datetime;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use serde_derive::Serialize;

pub struct GetRequest {
    pub endpoint: String,
    pub query: Vec<(String, String)>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait GetRequestable {
    fn into_get_request(self) -> Result<GetRequest>;
}

pub struct PostRequest<S: Serialize> {
    pub endpoint: String,
    pub body: S,
}

pub trait PostRequestable {
    type S: Serialize;
    fn into_post_request(self) -> Result<PostRequest<Self::S>>;
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

impl GetRequestable for HistoryParams {
    fn into_get_request(self) -> Result<GetRequest> {
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

        Ok(GetRequest { endpoint, query })
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

impl GetRequestable for LogbookParams {
    fn into_get_request(self) -> Result<GetRequest> {
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

        Ok(GetRequest { endpoint, query })
    }
}

pub struct CalendarParams {
    pub entity_id: String,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}

impl GetRequestable for CalendarParams {
    fn into_get_request(self) -> Result<GetRequest> {
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

        Ok(GetRequest { endpoint, query })
    }
}

pub struct StateParams {
    pub entity_id: String,
    pub state: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct StateRequestBody {
    pub state: String,
    pub attributes: HashMap<String, String>,
}

impl PostRequestable for StateParams {
    type S = StateRequestBody;
    fn into_post_request(self) -> Result<PostRequest<Self::S>> {
        let body = StateRequestBody {
            state: self.state,
            attributes: self.attributes,
        };

        Ok(PostRequest {
            endpoint: format!("/api/states/{}", self.entity_id),
            body,
        })
    }
}

pub struct TemplateParams {
    pub template: String,
}

#[derive(Serialize, Debug)]
pub struct TemplateRequestBody {
    pub template: String,
}

impl PostRequestable for TemplateParams {
    type S = TemplateRequestBody;
    fn into_post_request(self) -> Result<PostRequest<Self::S>> {
        let body = TemplateRequestBody {
            template: self.template,
        };

        Ok(PostRequest {
            endpoint: "/api/template".to_owned(),
            body,
        })
    }
}
