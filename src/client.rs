use log::debug;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use url::Url;

use crate::requests::{HistoryParams, LogbookParams, Queryable};
use crate::responses::{ApiStatus, Calendar, Config, Event, History, Logbook, Service, State};

pub struct Client {
    url: Url,
    token: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Client {
    pub fn new(url: &str, token: &str) -> Result<Self> {
        Ok(Client {
            url: Url::parse(url)?,
            token: token.to_owned(),
        })
    }

    fn build_get_request(&self, endpoint: &str) -> RequestBuilder {
        let mut url = self.url.clone();
        url.set_path(endpoint);

        reqwest::Client::new()
            .get(url)
            .bearer_auth(self.token.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
    }

    fn build_get_request_with_query(&self, endpoint: &str, query: &str) -> RequestBuilder {
        let mut url = self.url.clone();
        url.set_path(endpoint);
        url.set_query(Some(query));

        reqwest::Client::new()
            .get(url)
            .bearer_auth(self.token.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
    }

    async fn get_text_request(&self, endpoint: &str) -> Result<String> {
        let request = self
            .build_get_request(endpoint)
            .send()
            .await?
            .text()
            .await?;

        Ok(request)
    }

    async fn get_request<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let request = self
            .build_get_request(endpoint)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(request)
    }

    async fn get_request_with_query<T: DeserializeOwned, Q: Queryable>(
        &self,
        queryable: Q,
    ) -> Result<T> {
        let query = queryable.into_query();

        let query_string = serde_qs::to_string(&query.query)?;

        debug!(target: "homeassistant-rest", "Query string: {}", query_string);

        let request = self
            .build_get_request_with_query(&query.endpoint, &query_string)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(request)
    }

    pub async fn api_status(&self) -> Result<ApiStatus> {
        self.get_request::<ApiStatus>("/api/").await
    }

    pub async fn config(&self) -> Result<Config> {
        self.get_request::<Config>("/api/config").await
    }

    pub async fn events(&self) -> Result<Vec<Event>> {
        self.get_request::<Vec<Event>>("/api/events").await
    }

    pub async fn services(&self) -> Result<Vec<Service>> {
        self.get_request::<Vec<Service>>("/api/services").await
    }

    pub async fn history(&self, args: HistoryParams) -> Result<Vec<Vec<History>>> {
        self.get_request_with_query::<Vec<Vec<History>>, _>(args)
            .await
    }

    pub async fn logbook(&self, args: LogbookParams) -> Result<Vec<Logbook>> {
        self.get_request_with_query::<Vec<Logbook>, _>(args).await
    }

    pub async fn states(&self) -> Result<Vec<State>> {
        self.get_request::<Vec<State>>("/api/states").await
    }

    pub async fn error_log(&self) -> Result<String> {
        self.get_text_request("/api/error_log").await
    }

    pub async fn calendars(&self) -> Result<Vec<Calendar>> {
        self.get_request("/api/calendars").await
    }
}
