use serde::de::DeserializeOwned;

use url::Url;

use crate::responses::{ApiStatus, Config, Event, Service, History, Logbook, State};
use crate::requests::{HistoryQueryParams, Queryable, LogbookParams};

pub struct Client {
    url: Url,
    token: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Client {
    pub fn new(url: &str, token: &str) -> Result<Self> {
        Ok(Client {
            url: Url::parse(url)?,
            token: token.to_owned()
        })
    }

    async fn get_request<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let mut url = self.url.clone();
        url.set_path(endpoint);

        let request = reqwest::Client::new()
            .get(url)
            .bearer_auth(self.token.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(request)
    }

    async fn get_request_with_query<T: DeserializeOwned, Q: Queryable>(&self, queryable: Q) -> Result<T> {
        let query = queryable.generate_query();

        let mut url = self.url.clone();
        url.set_path(&query.endpoint);

        let request = reqwest::Client::new()
            .get(url)
            .bearer_auth(self.token.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .query(&query.query_params)
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

    pub async fn history(&self, args: HistoryQueryParams) -> Result<Vec<Vec<History>>> {
        self.get_request_with_query::<Vec<Vec<History>>, _>(args).await
    }

    pub async fn logbook(&self, args: LogbookParams) -> Result<Vec<Logbook>> {
        self.get_request_with_query::<Vec<Logbook>, _>(args).await
    }

    pub async fn states(&self) -> Result<Vec<State>> {
        self.get_request::<Vec<State>>("/api/states").await
    }

}

