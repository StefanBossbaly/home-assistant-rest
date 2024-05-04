use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to parse the URL: {0}")]
    UrlParseFailed(#[from] url::ParseError),

    #[error("Unable to send the request: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Unable to deserialize the received value: {0}")]
    DeserializeFailed(#[from] serde_json::error::Error),
}
