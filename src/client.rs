use bytes::Bytes;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::requests::{
    CalendarParams, GetRequest, GetRequestable, HistoryParams, LogbookParams, PostRequest,
    PostRequestable, StateParams, TemplateParams,
};
use crate::responses::{
    ApiStatus, Calendar, CalendarEvent, Config, Event, History, Logbook, Service, State,
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

    fn build_get_request_with_query(&self, query_params: GetRequest) -> RequestBuilder {
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

    async fn post_text_request<S: Serialize>(&self, post_param: PostRequest<S>) -> Result<String> {
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

    async fn get_request_with_query<T: DeserializeOwned, Q: GetRequestable>(
        &self,
        queryable: Q,
    ) -> Result<T> {
        let query_params = queryable.into_get_request()?;

        let request = self
            .build_get_request_with_query(query_params)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(request)
    }

    pub async fn get_api_status(&self) -> Result<ApiStatus> {
        self.get_request::<ApiStatus>("/api/").await
    }

    pub async fn get_config(&self) -> Result<Config> {
        self.get_request::<Config>("/api/config").await
    }

    pub async fn get_events(&self) -> Result<Vec<Event>> {
        self.get_request::<Vec<Event>>("/api/events").await
    }

    pub async fn get_services(&self) -> Result<Vec<Service>> {
        self.get_request::<Vec<Service>>("/api/services").await
    }

    pub async fn get_history(&self, args: HistoryParams) -> Result<Vec<Vec<History>>> {
        self.get_request_with_query::<Vec<Vec<History>>, _>(args)
            .await
    }

    pub async fn get_logbook(&self, args: LogbookParams) -> Result<Vec<Logbook>> {
        self.get_request_with_query::<Vec<Logbook>, _>(args).await
    }

    pub async fn get_states(&self) -> Result<Vec<State>> {
        self.get_request::<Vec<State>>("/api/states").await
    }

    pub async fn get_error_log(&self) -> Result<String> {
        self.get_text_request("/api/error_log").await
    }

    pub async fn get_camera_proxy(&self) -> Result<Bytes> {
        unimplemented!()
    }

    pub async fn get_calendars(&self) -> Result<Vec<Calendar>> {
        self.get_request("/api/calendars").await
    }

    pub async fn get_calendars_of_entity(
        &self,
        params: CalendarParams,
    ) -> Result<Vec<CalendarEvent>> {
        self.get_request_with_query::<Vec<CalendarEvent>, _>(params)
            .await
    }

    pub async fn post_states(&self, params: StateParams) -> Result<String> {
        self.post_text_request(params.into_post_request()?).await
    }

    pub async fn post_events(&self) -> Result<()> {
        unimplemented!()
    }

    pub async fn post_template(&self, params: TemplateParams) -> Result<String> {
        self.post_text_request(params.into_post_request()?).await
    }

    pub async fn post_config_check(&self) -> Result<()> {
        unimplemented!()
    }

    pub async fn post_handle(&self) -> Result<()> {
        unimplemented!()
    }
}
