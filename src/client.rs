use serde::de::DeserializeOwned;
use reqwest;

use crate::types::{ApiStatus, Config, Event, Service};

pub struct Client {
    url: String,
    token: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Client {
    pub fn new(url: String, token: String) -> Self {
        Client {
            url,
            token
        }
    }

    fn build_client(&self, endpoint: &str) -> reqwest::RequestBuilder {
        reqwest::Client::new()
            .get(format!("{}{}", self.url, endpoint))
            .header("Authorization", format!("Bearer {}", self.token))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
    }

    async fn get_request<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let request = self.build_client(endpoint)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(request)
    }

    pub async fn api_status(&self) -> Result<ApiStatus> {
        Ok(self.get_request::<ApiStatus>("/api/").await?)
    }

    pub async fn config(&self) -> Result<Config> {
        Ok(self.get_request::<Config>("/api/config").await?)
    }

    pub async fn events(&self) -> Result<Vec<Event>> {
        Ok(self.get_request::<Vec<Event>>("/api/events").await?)
    }

    pub async fn services(&self) -> Result<Vec<Service>> {
        Ok(self.get_request::<Vec<Service>>("/api/services").await?)
    }
}

