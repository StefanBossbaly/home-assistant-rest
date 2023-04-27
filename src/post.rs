use crate::deserialize::{deserialize_datetime, deserialize_optional_state_enum};
use crate::{errors, get::StateEnum};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Deserialize, Debug)]
pub struct StateContextStateResponse {
    pub id: String,
    pub parent_id: Option<String>,
    pub user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct StateResponse {
    pub entity_id: String,

    #[serde(deserialize_with = "deserialize_optional_state_enum")]
    pub state: Option<StateEnum>,

    pub attributes: HashMap<String, serde_json::Value>,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_changed: DateTime<FixedOffset>,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_updated: DateTime<FixedOffset>,

    pub context: StateContextStateResponse,
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
