#![cfg(feature = "serde_debugging")]
use anyhow::Context;
use chrono::{FixedOffset, TimeZone};
use home_assistant_rest::{get, Client};

fn get_env_vars() -> anyhow::Result<(String, String)> {
    let server = std::env::var("LIVE_ENDPOINT_URL")
        .context("Could not read LIVE_ENDPOINT_URL env variable")?;
    let token = std::env::var("LIVE_ENDPOINT_TOKEN")
        .context("Could not read LIVE_ENDPOINT_TOKEN env variable")?;

    Ok((server, token))
}

#[tokio::test]
async fn live_endpoint_api_status() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let client = Client::new(&server, &token)?;
    let api_status = client.get_api_status_with_debugging().await?;

    dbg!(api_status);

    Ok(())
}

#[tokio::test]
async fn live_endpoint_config() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let client = Client::new(&server, &token)?;
    let _config = client.get_config_with_debugging().await?;

    Ok(())
}

#[tokio::test]
async fn live_endpoint_get_events() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let client = Client::new(&server, &token)?;
    let _events = client.get_events_with_debugging().await?;

    Ok(())
}

#[tokio::test]
async fn live_endpoint_get_services() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let client = Client::new(&server, &token)?;
    let services = client.get_services_with_debugging().await?;

    dbg!(services);

    Ok(())
}

#[tokio::test]
async fn live_endpoint_get_history() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let start_time = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2016, 12, 29, 11, 22, 33)
        .unwrap();

    let end_time = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2016, 12, 30, 10, 11, 22)
        .unwrap();

    let client = Client::new(&server, &token)?;
    let _history = client
        .get_history_with_debugging(get::HistoryParams {
            start_time: Some(start_time),
            end_time: Some(end_time),
            ..get::HistoryParams::default()
        })
        .await?;

    Ok(())
}

#[tokio::test]
async fn live_endpoint_get_logbook() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let start_time = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2016, 12, 29, 11, 22, 33)
        .unwrap();

    let end_time = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2016, 12, 30, 10, 11, 22)
        .unwrap();

    let client = Client::new(&server, &token)?;

    let _logbook = client
        .get_logbook_with_debugging(get::LogbookParams {
            start_time: Some(start_time),
            end_time: Some(end_time),
            ..get::LogbookParams::default()
        })
        .await?;

    Ok(())
}

#[tokio::test]
async fn live_endpoint_get_states() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let client = Client::new(&server, &token)?;

    let _states = client.get_states_with_debugging().await?;

    Ok(())
}

#[tokio::test]
async fn live_endpoint_get_error_log() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let client = Client::new(&server, &token)?;

    let _error_log = client.get_error_log().await?;

    Ok(())
}

#[tokio::test]
async fn live_endpoint_get_calendars() -> anyhow::Result<()> {
    let (server, token) = get_env_vars()?;

    let client = Client::new(&server, &token)?;

    let _error_log = client.get_calendars_with_debugging().await?;

    Ok(())
}
