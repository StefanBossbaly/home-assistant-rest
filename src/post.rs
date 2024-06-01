use std::collections::HashMap;

use crate::{errors, StateEnum};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// Represents a POST request that will be made to the home assistant server
pub struct Request<S: Serialize> {
    pub endpoint: String,
    pub body: S,
}

pub trait Requestable {
    type S: Serialize;
    fn into_request(self) -> Result<Request<Self::S>, errors::Error>;
}

#[derive(Serialize, Debug)]
pub struct StateRequestBody {
    pub state: String,
    pub attributes: HashMap<String, String>,
}

pub struct StateParams {
    pub entity_id: String,
    pub state: String,
    pub attributes: HashMap<String, String>,
}

impl Requestable for StateParams {
    type S = StateRequestBody;
    fn into_request(self) -> Result<Request<Self::S>, errors::Error> {
        let body = StateRequestBody {
            state: self.state,
            attributes: self.attributes,
        };

        Ok(Request {
            endpoint: format!("/api/states/{}", self.entity_id),
            body,
        })
    }
}

/// The context of the trigger that caused the state change
///
/// https://github.com/home-assistant/core/blob/2a4686e1b7703e33271f40aeca0325659166c6fb/homeassistant/core.py#L1250
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StateContextStateResponse {
    pub id: String,
    pub parent_id: Option<String>,
    pub user_id: Option<String>,
}

/// Response from the `/api/states/<entity_id>` endpoint
///
/// https://github.com/home-assistant/core/blob/2a4686e1b7703e33271f40aeca0325659166c6fb/homeassistant/core.py#L1741
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StateResponse {
    /// The unique identifier of the entity
    pub entity_id: String,

    /// The current state of the entity
    pub state: Option<StateEnum>,

    /// Extra information about the entity and the state
    pub attributes: HashMap<String, serde_json::Value>,

    /// The last time the entity changed state
    pub last_changed: Option<DateTime<FixedOffset>>,

    /// The last time the state was reported
    pub last_reported: Option<DateTime<FixedOffset>>,

    /// The last time the state or attributes were changed
    pub last_updated: Option<DateTime<FixedOffset>>,

    pub context: StateContextStateResponse,
}

pub struct EventParams {
    pub event_type: String,
    pub event_data: Option<serde_json::Value>,
}

impl Requestable for EventParams {
    type S = Option<serde_json::Value>;

    fn into_request(self) -> Result<Request<Self::S>, errors::Error> {
        Ok(Request {
            endpoint: format!("/api/events/{}", self.event_type),
            body: self.event_data,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct EventResponse {
    pub message: String,
}

pub struct TemplateParams {
    pub template: String,
}

#[derive(Serialize, Debug)]
pub struct TemplateRequestBody {
    pub template: String,
}

impl Requestable for TemplateParams {
    type S = TemplateRequestBody;
    fn into_request(self) -> Result<Request<Self::S>, errors::Error> {
        let body = TemplateRequestBody {
            template: self.template,
        };

        Ok(Request {
            endpoint: "/api/template".to_owned(),
            body,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct CheckConfigResponse {
    pub errors: Option<String>,
    pub result: String,
}
