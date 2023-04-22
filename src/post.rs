use std::collections::HashMap;

use serde::Serialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Request<S: Serialize> {
    pub endpoint: String,
    pub body: S,
}

pub trait Requestable {
    type S: Serialize;
    fn into_request(self) -> Result<Request<Self::S>>;
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
    fn into_request(self) -> Result<Request<Self::S>> {
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

pub struct TemplateParams {
    pub template: String,
}

#[derive(Serialize, Debug)]
pub struct TemplateRequestBody {
    pub template: String,
}

impl Requestable for TemplateParams {
    type S = TemplateRequestBody;
    fn into_request(self) -> Result<Request<Self::S>> {
        let body = TemplateRequestBody {
            template: self.template,
        };

        Ok(Request {
            endpoint: "/api/template".to_owned(),
            body,
        })
    }
}
