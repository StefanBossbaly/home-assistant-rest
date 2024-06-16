//! Home Assistant REST Client

use crate::{
    errors, get,
    post::{self, Requestable},
};

use std::fmt::Display;

use bytes::Bytes;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

/// Represents a connection to a Home Assistant instance
pub struct Client {
    url: Url,
    token: String,
}

type Result<T> = std::result::Result<T, errors::Error>;

#[cfg(feature = "serde_debugging")]
type DebuggingResult<T> = std::result::Result<T, errors::DebuggingError>;

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

    fn build_post_request(&self, endpoint: &str) -> RequestBuilder {
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

    fn build_post_request_with_query<S: Serialize>(
        &self,
        query_params: post::Request<S>,
    ) -> RequestBuilder {
        let mut url = self.url.clone();
        url.set_path(&query_params.endpoint);

        reqwest::Client::new()
            .post(url)
            .bearer_auth(self.token.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&query_params.body)
    }

    async fn post_text_request<S: Serialize>(
        &self,
        post_param: post::Request<S>,
    ) -> Result<String> {
        let request = self
            .build_post_request(&post_param.endpoint)
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

    /// Sends a GET request to the specified endpoint and returns the response as a deserialized object
    async fn get_request<S, D>(&self, endpoint: S) -> Result<D>
    where
        S: AsRef<str>,
        D: DeserializeOwned,
    {
        let request = self
            .build_get_request(endpoint.as_ref())
            .send()
            .await?
            .json::<D>()
            .await?;

        Ok(request)
    }

    /// Same as [`get_request`] but using `serde_path_to_error` as the deserializer adapter
    ///
    /// [`get_request`]: Client::get_request
    #[cfg(feature = "serde_debugging")]
    async fn get_request_with_debugging<S, D>(&self, endpoint: S) -> DebuggingResult<D>
    where
        S: AsRef<str>,
        D: DeserializeOwned,
    {
        let bytes = self
            .build_get_request(endpoint.as_ref())
            .send()
            .await?
            .bytes()
            .await?;

        let response: D = serde_path_to_error::deserialize(
            &mut serde_json::Deserializer::from_slice(bytes.as_ref()),
        )
        .map_err(|err| errors::DebuggingError::DeserializeFailed {
            error: err,
            response: std::str::from_utf8(bytes.as_ref())
                .unwrap_or("Failed to convert bytes to string")
                .to_string(),
        })?;

        Ok(response)
    }

    /// Sends a POST request to the specified endpoint and returns the response as a deserialized object
    async fn post_request<D>(&self, endpoint: &str) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let request = self
            .build_post_request(endpoint)
            .send()
            .await?
            .json::<D>()
            .await?;

        Ok(request)
    }

    /// Same as [`post_request`] but using `serde_path_to_error` as the deserializer adapter
    ///
    /// [`post_request`]: Client::post_request
    #[cfg(feature = "serde_debugging")]
    async fn post_request_with_debugging<S, D>(&self, endpoint: S) -> DebuggingResult<D>
    where
        S: AsRef<str>,
        D: DeserializeOwned,
    {
        let bytes = self
            .build_post_request(endpoint.as_ref())
            .send()
            .await?
            .bytes()
            .await?;

        let response: D = serde_path_to_error::deserialize(
            &mut serde_json::Deserializer::from_slice(bytes.as_ref()),
        )
        .map_err(|err| errors::DebuggingError::DeserializeFailed {
            error: err,
            response: std::str::from_utf8(bytes.as_ref())
                .unwrap_or("Failed to convert bytes to string")
                .to_string(),
        })?;

        Ok(response)
    }

    async fn get_request_with_query<D, Q>(&self, queryable: Q) -> Result<D>
    where
        D: DeserializeOwned,
        Q: get::Parameters,
    {
        let query_params = queryable.into_request();

        let request = self
            .build_get_request_with_query(query_params)
            .send()
            .await?
            .json::<D>()
            .await?;

        Ok(request)
    }

    #[cfg(feature = "serde_debugging")]
    async fn get_request_with_query_and_debugging<D, Q>(&self, queryable: Q) -> DebuggingResult<D>
    where
        D: DeserializeOwned,
        Q: get::Parameters,
    {
        let query_params = queryable.into_request();

        let bytes = self
            .build_get_request_with_query(query_params)
            .send()
            .await?
            .bytes()
            .await?;

        let response: D = serde_path_to_error::deserialize(
            &mut serde_json::Deserializer::from_slice(bytes.as_ref()),
        )
        .map_err(|err| errors::DebuggingError::DeserializeFailed {
            error: err,
            response: std::str::from_utf8(bytes.as_ref())
                .unwrap_or("Failed to convert bytes to string")
                .to_string(),
        })?;

        Ok(response)
    }

    async fn post_request_with_query<S, D>(&self, request: post::Request<S>) -> Result<D>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        let request = self
            .build_post_request_with_query(request)
            .send()
            .await?
            .json::<D>()
            .await?;

        Ok(request)
    }

    #[cfg(feature = "serde_debugging")]
    async fn post_request_with_query_and_debugging<S, D>(
        &self,
        request: post::Request<S>,
    ) -> DebuggingResult<D>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        let bytes = self
            .build_post_request_with_query(request)
            .send()
            .await?
            .bytes()
            .await?;

        let response: D = serde_path_to_error::deserialize(
            &mut serde_json::Deserializer::from_slice(bytes.as_ref()),
        )
        .map_err(|err| errors::DebuggingError::DeserializeFailed {
            error: err,
            response: std::str::from_utf8(bytes.as_ref())
                .unwrap_or("Failed to convert bytes to string")
                .to_string(),
        })?;

        Ok(response)
    }

    /// Calls the `/api/` endpoint which returns the status of the Home Assistant API
    ///
    /// This function will return a [`get::ApiStatusResponse`] which contains the status of the API.
    /// If the API is up and running, the `message` field will be `API running.`. Any other message indicates
    /// that the API is not running or an error has occurred.
    pub async fn get_api_status(&self) -> Result<get::ApiStatusResponse> {
        self.get_request("/api/").await
    }

    /// Same as [`get_api_status`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_api_status`]: Client::get_api_status
    #[cfg(feature = "serde_debugging")]
    pub async fn get_api_status_with_debugging(&self) -> DebuggingResult<get::ApiStatusResponse> {
        self.get_request_with_debugging("/api/").await
    }

    /// Calls the `/api/config` endpoint which returns the current configuration of the Home Assistant instance
    pub async fn get_config(&self) -> Result<get::ConfigResponse> {
        self.get_request("/api/config").await
    }

    /// Same as [`get_config`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_config`]: Client::get_config
    #[cfg(feature = "serde_debugging")]
    pub async fn get_config_with_debugging(&self) -> DebuggingResult<get::ConfigResponse> {
        self.get_request_with_debugging("/api/config").await
    }

    /// Calls the `/api/events` endpoint which returns an array of event objects
    pub async fn get_events(&self) -> Result<get::EventsResponse> {
        self.get_request("/api/events").await
    }

    /// Same as [`get_events`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_events`]: Client::get_events
    #[cfg(feature = "serde_debugging")]
    pub async fn get_events_with_debugging(&self) -> DebuggingResult<get::EventsResponse> {
        self.get_request_with_debugging("/api/events").await
    }

    /// Calls the `/api/services` endpoint which returns an array of service objects
    pub async fn get_services(&self) -> Result<get::ServicesResponse> {
        self.get_request("/api/services").await
    }

    /// Same as [`get_services`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_services`]: Client::get_services
    #[cfg(feature = "serde_debugging")]
    pub async fn get_services_with_debugging(&self) -> DebuggingResult<get::ServicesResponse> {
        self.get_request_with_debugging("/api/services").await
    }

    /// Calls the `/api/history/period/<timestamp>` which returns an array of state changes in the past
    pub async fn get_history(&self, params: get::HistoryParams) -> Result<get::HistoryResponse> {
        self.get_request_with_query(params).await
    }

    /// Same as [`get_history`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_history`]: Client::get_history
    #[cfg(feature = "serde_debugging")]
    pub async fn get_history_with_debugging(
        &self,
        params: get::HistoryParams,
    ) -> DebuggingResult<get::HistoryResponse> {
        self.get_request_with_query_and_debugging(params).await
    }

    /// Calls the `/api/logbook/<timestamp>` which returns an array of logbook entries
    pub async fn get_logbook(&self, params: get::LogbookParams) -> Result<get::LogbookResponse> {
        self.get_request_with_query(params).await
    }

    /// Same as [`get_logbook`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_logbook`]: Client::get_logbook
    #[cfg(feature = "serde_debugging")]
    pub async fn get_logbook_with_debugging(
        &self,
        params: get::LogbookParams,
    ) -> DebuggingResult<get::LogbookResponse> {
        self.get_request_with_query_and_debugging(params).await
    }

    /// Calls the `/api/states` which return an array of state objects.
    pub async fn get_states(&self) -> Result<get::StatesResponse> {
        self.get_request("/api/states").await
    }

    /// Same as [`get_logbook`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_logbook`]: Client::get_logbook
    #[cfg(feature = "serde_debugging")]
    pub async fn get_states_with_debugging(&self) -> DebuggingResult<get::StatesResponse> {
        self.get_request_with_debugging("/api/states").await
    }

    /// Calls the `/api/states/<entity_id>` which returns a state object for the specifies `entity_id`
    pub async fn get_states_of_entity<D>(&self, entity_id: D) -> Result<get::StatesEntityResponse>
    where
        D: Display,
    {
        self.get_request(&format!("/api/states/{}", entity_id))
            .await
    }

    /// Same as [`get_states_of_entity`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_states_of_entity`]: Client::get_states_of_entity
    #[cfg(feature = "serde_debugging")]
    pub async fn get_states_of_entity_with_debugging<D>(
        &self,
        entity_id: D,
    ) -> DebuggingResult<get::StatesEntityResponse>
    where
        D: Display,
    {
        self.get_request_with_debugging(&format!("/api/states/{}", entity_id))
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

    /// Same as [`get_calendars`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_calendars`]: Client::get_calendars
    #[cfg(feature = "serde_debugging")]
    pub async fn get_calendars_with_debugging(&self) -> DebuggingResult<get::CalendarsResponse> {
        self.get_request_with_debugging("/api/calendars").await
    }

    /// Calls the `/api/calendars/<calendar entity_id>` endpoint which returns a list of calendar events for the specified entity.
    pub async fn get_calendars_of_entity(
        &self,
        params: get::CalendarsParams,
    ) -> Result<get::CalendarsEntityResponse> {
        self.get_request_with_query(params).await
    }

    /// Same as [`get_calendars_of_entity`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`get_calendars_of_entity`]: Client::get_calendars_of_entity
    #[cfg(feature = "serde_debugging")]
    pub async fn get_calendars_of_entity_with_debugging(
        &self,
        params: get::CalendarsParams,
    ) -> DebuggingResult<get::CalendarsEntityResponse> {
        self.get_request_with_query_and_debugging(params).await
    }

    // Calls the `/api/states/<entity_id>` endpoint which updates or creates a state.
    pub async fn post_states(&self, params: post::StateParams) -> Result<post::StateResponse> {
        self.post_request_with_query(params.into_request()).await
    }

    /// Same as [`post_states`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`post_states`]: Client::post_states
    #[cfg(feature = "serde_debugging")]
    pub async fn post_states_with_debugging(
        &self,
        params: post::StateParams,
    ) -> DebuggingResult<post::StateResponse> {
        self.post_request_with_query_and_debugging(params.into_request())
            .await
    }

    /// Calls the `/api/events/<event_type>` endpoint which fires an event.
    pub async fn post_events(&self, params: post::EventParams) -> Result<post::EventResponse> {
        let request = params.into_request();
        let builder = self.build_post_request(&request.endpoint);

        Ok(match request.body {
            Some(data) => {
                builder
                    .json(&data)
                    .send()
                    .await?
                    .json::<post::EventResponse>()
                    .await?
            }
            None => builder.send().await?.json::<post::EventResponse>().await?,
        })
    }

    /// Same as [`post_events`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`post_events`]: Client::post_events
    #[cfg(feature = "serde_debugging")]
    pub async fn post_events_with_debugging(
        &self,
        params: post::EventParams,
    ) -> DebuggingResult<post::EventResponse> {
        let request = params.into_request();
        let builder = self.build_post_request(&request.endpoint);

        let bytes = match request.body {
            Some(data) => builder.json(&data).send().await?.bytes().await?,
            None => builder.send().await?.bytes().await?,
        };

        let response = serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(
            bytes.as_ref(),
        ))
        .map_err(|err| errors::DebuggingError::DeserializeFailed {
            error: err,
            response: std::str::from_utf8(bytes.as_ref())
                .unwrap_or("Failed to convert bytes to string")
                .to_string(),
        })?;

        Ok(response)
    }

    /// Calls the `/api/services/<domain>/<service>` endpoint which calls a service. Currently unimplemented.
    pub async fn post_service(&self) -> Result<()> {
        unimplemented!()
    }

    /// Calls the `/api/template` endpoint which renders a Home Assistant template.
    pub async fn post_template(&self, params: post::TemplateParams) -> Result<String> {
        self.post_text_request(params.into_request()).await
    }

    /// Calls the `/api/config/core/check_config` endpoint which triggers a check of the current configuration. Currently unimplemented.
    pub async fn post_config_check(&self) -> Result<post::CheckConfigResponse> {
        self.post_request("/api/config/core/check_config").await
    }

    /// Same as [`post_config_check`] but using [`serde_path_to_error`] as the deserialize adapter
    ///
    /// [`post_config_check`]: Client::post_config_check
    #[cfg(feature = "serde_debugging")]
    pub async fn post_config_check_with_debugging(
        &self,
    ) -> DebuggingResult<post::CheckConfigResponse> {
        self.post_request_with_debugging("/api/config/core/check_config")
            .await
    }

    /// Calls the `/api/intent/handle` endpoint which handles an intent. Currently unimplemented.
    pub async fn post_handle(&self) -> Result<()> {
        unimplemented!()
    }
}
