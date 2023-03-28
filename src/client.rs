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

    async fn get_request_with_query<T: DeserializeOwned, Q: get::Parameters>(
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

    pub async fn get_api_status(&self) -> Result<get::ApiStatusResponse> {
        self.get_request::<get::ApiStatusResponse>("/api/").await
    }

    pub async fn get_config(&self) -> Result<get::ConfigResponse> {
        self.get_request::<get::ConfigResponse>("/api/config").await
    }

    pub async fn get_events(&self) -> Result<get::EventsResponse> {
        self.get_request::<get::EventsResponse>("/api/events").await
    }

    pub async fn get_services(&self) -> Result<get::ServicesResponse> {
        self.get_request::<get::ServicesResponse>("/api/services")
            .await
    }

    pub async fn get_history(&self, params: get::HistoryParams) -> Result<get::HistoryResponse> {
        self.get_request_with_query::<get::HistoryResponse, _>(params)
            .await
    }

    pub async fn get_logbook(&self, params: get::LogbookParams) -> Result<get::LogbookResponse> {
        self.get_request_with_query::<get::LogbookResponse, _>(params)
            .await
    }

    pub async fn get_states(&self) -> Result<get::StatesResponse> {
        self.get_request::<get::StatesResponse>("/api/states").await
    }

    pub async fn get_states_of_entity(&self, entity_id: &str) -> Result<get::StatesEntityResponse> {
        self.get_request::<get::StatesEntityResponse>(&format!("/api/states/{}", entity_id))
            .await
    }

    pub async fn get_error_log(&self) -> Result<String> {
        self.get_text_request("/api/error_log").await
    }

    pub async fn get_camera_proxy(&self) -> Result<Bytes> {
        unimplemented!()
    }

    pub async fn get_calendars(&self) -> Result<get::CalendarsResponse> {
        self.get_request("/api/calendars").await
    }

    pub async fn get_calendars_of_entity(
        &self,
        params: get::CalendarsParams,
    ) -> Result<get::CalendarsEntityResponse> {
        self.get_request_with_query::<get::CalendarsEntityResponse, _>(params)
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
