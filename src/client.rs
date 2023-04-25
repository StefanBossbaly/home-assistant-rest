//! Home Assistant REST Client

use bytes::Bytes;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::{
    get,
    post::{self, Requestable},
};

/// Represents a connection to a Home Assistant instance
pub struct Client {
    url: Url,
    token: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Client {
    /// Creates a new instance of the client
    ///
    /// This function will not attempt to connect to the Home Assistant instance. It will only
    /// ensure that the URL is valid. The user must check the status of the API by calling the
    /// [`get_api_status`](crate::Client::get_api_status) function.
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

        if !query_params.query.is_empty() {
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
        }

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

    /// Calls the `/api/` endpoint which returns the status of the Home Assistant API
    ///
    /// This function will return a [`get::ApiStatusResponse`] which contains the status of the API.
    /// If the API is up and running, the `message` field will be `API running.`. Any other message indicates
    /// that the API is not running or an error has occurred.
    pub async fn get_api_status(&self) -> Result<get::ApiStatusResponse> {
        self.get_request::<get::ApiStatusResponse>("/api/").await
    }

    /// Calls the `/api/config` endpoint which returns the current configuration of the Home Assistant instance
    pub async fn get_config(&self) -> Result<get::ConfigResponse> {
        self.get_request::<get::ConfigResponse>("/api/config").await
    }

    /// Calls the `/api/events` endpoint which returns an array of event objects
    pub async fn get_events(&self) -> Result<get::EventsResponse> {
        self.get_request::<get::EventsResponse>("/api/events").await
    }

    /// Calls the `/api/services` endpoint which returns an array of service objects
    pub async fn get_services(&self) -> Result<get::ServicesResponse> {
        self.get_request::<get::ServicesResponse>("/api/services")
            .await
    }

    /// Calls the `/api/history/period/<timestamp>` which returns an array of state changes in the past
    pub async fn get_history(&self, params: get::HistoryParams) -> Result<get::HistoryResponse> {
        self.get_request_with_query::<get::HistoryResponse, _>(params)
            .await
    }

    /// Calls the `/api/logbook/<timestamp>` which returns an array of logbook entries
    pub async fn get_logbook(&self, params: get::LogbookParams) -> Result<get::LogbookResponse> {
        self.get_request_with_query::<get::LogbookResponse, _>(params)
            .await
    }

    /// Calls the `/api/states` which return an array of state objects.
    pub async fn get_states(&self) -> Result<get::StatesResponse> {
        self.get_request::<get::StatesResponse>("/api/states").await
    }

    /// Calls the `/api/states/<entity_id>` which returns a state object for the specifies `entity_id`
    pub async fn get_states_of_entity(&self, entity_id: &str) -> Result<get::StatesEntityResponse> {
        self.get_request::<get::StatesEntityResponse>(&format!("/api/states/{}", entity_id))
            .await
    }

    /// Calls the `/api/error_log` which returns all errors logged during the current session as a plaintext response.
    pub async fn get_error_log(&self) -> Result<String> {
        self.get_text_request("/api/error_log").await
    }

    /// Calls the `/api/camera_proxy/<camera entity_id>`. Still a work in progress. Currently unimplemented.
    pub async fn get_camera_proxy(&self) -> Result<Bytes> {
        unimplemented!()
    }

    /// Calls the `/api/calendars` endpoint which returns an array of calendar entities.
    pub async fn get_calendars(&self) -> Result<get::CalendarsResponse> {
        self.get_request("/api/calendars").await
    }

    /// Calls the `/api/calendars/<calendar entity_id>` endpoint which returns a list of calendar events for the specified entity.
    pub async fn get_calendars_of_entity(
        &self,
        params: get::CalendarsParams,
    ) -> Result<get::CalendarsEntityResponse> {
        self.get_request_with_query::<get::CalendarsEntityResponse, _>(params)
            .await
    }

    // Calls the `/api/states/<entity_id>` endpoint which updates or creates a state.
    pub async fn post_states(&self, params: post::StateParams) -> Result<String> {
        self.post_text_request(params.into_request()?).await
    }

    /// Calls the `/api/events/<event_type>` endpoint which fires an event. Currently unimplemented.
    pub async fn post_events(&self) -> Result<()> {
        unimplemented!()
    }

    /// Calls the `/api/services/<domain>/<service>` endpoint which calls a service. Currently unimplemented.
    pub async fn post_service(&self) -> Result<()> {
        unimplemented!()
    }

    /// Calls the `/api/template` endpoint which renders a Home Assistant template.
    pub async fn post_template(&self, params: post::TemplateParams) -> Result<String> {
        self.post_text_request(params.into_request()?).await
    }

    /// Calls the `/api/config/core/check_config` endpoint which triggers a check of the current configuration. Currently unimplemented.
    pub async fn post_config_check(&self) -> Result<()> {
        unimplemented!()
    }

    /// Calls the `/api/intent/handle` endpoint which handles an intent. Currently unimplemented.
    pub async fn post_handle(&self) -> Result<()> {
        unimplemented!()
    }
}
