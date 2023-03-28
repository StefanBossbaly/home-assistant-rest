use bytes::Bytes;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::{
    get,
    post::{self, Requestable},
};

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

    fn build_put_request(&self, endpoint: &str) -> RequestBuilder {
        let mut url = self.url.clone();
        url.set_path(endpoint);

        reqwest::Client::new()
            .post(url)
            .bearer_auth(self.token.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
    }

    fn build_get_request_with_query(&self, query_params: get::Request) -> RequestBuilder {
        let mut url = self.url.clone();
        url.set_path(&query_params.endpoint);

        let mut query_string = String::new();
        let mut first_time = true;

        for (key, value) in query_params.query {
            if first_time {
                query_string.push_str(format!("{}={}", key, value).as_str());
            } else {
                query_string.push_str(format!("&{}={}", key, value).as_str());
            }
            first_time = false;
        }

        url.set_query(Some(&query_string));

        reqwest::Client::new()
            .get(url)
            .bearer_auth(self.token.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
    }

    async fn post_text_request<S: Serialize>(
        &self,
        post_param: post::Request<S>,
    ) -> Result<String> {
        let request = self
            .build_put_request(&post_param.endpoint)
            .json(&post_param.body)
            .send()
            .await?
            .text()
            .await?;

        Ok(request)
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

    #[allow(dead_code)]
    async fn get_binary_request(&self, endpoint: &str) -> Result<Bytes> {
        let request = self
            .build_get_request(endpoint)
            .send()
            .await?
            .bytes()
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

    async fn get_request_with_query<T: DeserializeOwned, Q: get::Requestable>(
        &self,
        queryable: Q,
    ) -> Result<T> {
        let query_params = queryable.into_request()?;

        let request = self
            .build_get_request_with_query(query_params)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(request)
    }

    pub async fn get_api_status(&self) -> Result<get::ApiStatus> {
        self.get_request::<get::ApiStatus>("/api/").await
    }

    pub async fn get_config(&self) -> Result<get::Config> {
        self.get_request::<get::Config>("/api/config").await
    }

    pub async fn get_events(&self) -> Result<Vec<get::Event>> {
        self.get_request::<Vec<get::Event>>("/api/events").await
    }

    pub async fn get_services(&self) -> Result<Vec<get::Service>> {
        self.get_request::<Vec<get::Service>>("/api/services").await
    }

    pub async fn get_history(&self, args: get::HistoryParams) -> Result<Vec<Vec<get::History>>> {
        self.get_request_with_query::<Vec<Vec<get::History>>, _>(args)
            .await
    }

    pub async fn get_logbook(&self, args: get::LogbookParams) -> Result<Vec<get::Logbook>> {
        self.get_request_with_query::<Vec<get::Logbook>, _>(args)
            .await
    }

    pub async fn get_states(&self) -> Result<Vec<get::State>> {
        self.get_request::<Vec<get::State>>("/api/states").await
    }

    pub async fn get_state(&self, entity_id: &str) -> Result<get::StateEntity> {
        self.get_request::<get::StateEntity>(&format!("/api/state/{}", entity_id))
            .await
    }

    pub async fn get_error_log(&self) -> Result<String> {
        self.get_text_request("/api/error_log").await
    }

    pub async fn get_camera_proxy(&self) -> Result<Bytes> {
        unimplemented!()
    }

    pub async fn get_calendars(&self) -> Result<Vec<get::Calendar>> {
        self.get_request("/api/calendars").await
    }

    pub async fn get_calendars_of_entity(
        &self,
        params: get::CalendarParams,
    ) -> Result<Vec<get::CalendarEvent>> {
        self.get_request_with_query::<Vec<get::CalendarEvent>, _>(params)
            .await
    }

    pub async fn post_states(&self, params: post::StateParams) -> Result<String> {
        self.post_text_request(params.into_request()?).await
    }

    pub async fn post_events(&self) -> Result<()> {
        unimplemented!()
    }

    pub async fn post_template(&self, params: post::TemplateParams) -> Result<String> {
        self.post_text_request(params.into_request()?).await
    }

    pub async fn post_config_check(&self) -> Result<()> {
        unimplemented!()
    }

    pub async fn post_handle(&self) -> Result<()> {
        unimplemented!()
    }
}
