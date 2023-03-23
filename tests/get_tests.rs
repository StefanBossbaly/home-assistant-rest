use std::vec;

use homeassistant_rest_rs::Client;
use mockito::{Mock, ServerGuard};

fn create_mock_server(server: &mut ServerGuard, endpoint: &str) -> Mock {
    server
        .mock("GET", endpoint)
        .match_header("content-type", "application/json")
        .match_header("Authorization", "Bearer test_token")
}

#[tokio::test]
async fn test_good_api_status_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/")
        .with_body(r#"{"message": "API running."}"#)
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let api_status = client.get_api_status().await?;

    assert_eq!(api_status.message, "API running.");

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_bad_api_status_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/")
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let api_status = client.get_api_status().await;

    assert!(api_status.is_err());

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_config_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/config")
        .with_body(
            r#"
        {
            "components":[
               "sensor.cpuspeed",
               "frontend",
               "config.core",
               "http",
               "map",
               "api",
               "sun",
               "config",
               "discovery",
               "conversation",
               "recorder",
               "group",
               "sensor",
               "websocket_api",
               "automation",
               "config.automation",
               "config.customize"
            ],
            "config_dir":"/home/ha/.homeassistant",
            "elevation":510,
            "latitude":45.8781529,
            "location_name":"Home",
            "longitude":8.458853651,
            "time_zone":"Europe/Zurich",
            "unit_system":{
               "length":"km",
               "mass":"g",
               "temperature":"\u00b0C",
               "volume":"L"
            },
            "version":"0.56.2",
            "whitelist_external_dirs":[
               "/home/ha/.homeassistant/www",
               "/home/ha/.homeassistant/"
            ]
         }"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let config = client.get_config().await?;

    assert_eq!(
        config.components,
        vec![
            "sensor.cpuspeed",
            "frontend",
            "config.core",
            "http",
            "map",
            "api",
            "sun",
            "config",
            "discovery",
            "conversation",
            "recorder",
            "group",
            "sensor",
            "websocket_api",
            "automation",
            "config.automation",
            "config.customize",
        ]
    );
    assert_eq!(config.config_dir, "/home/ha/.homeassistant");
    assert_eq!(config.elevation, 510);
    assert_eq!(config.latitude, 45.878_155);
    assert_eq!(config.location_name, "Home");
    assert_eq!(config.longitude, 8.458_854);
    assert_eq!(config.time_zone, "Europe/Zurich");
    assert_eq!(config.unit_system.length, "km");
    assert_eq!(config.unit_system.mass, "g");
    assert_eq!(config.unit_system.temperature, "\u{00b0}C");
    assert_eq!(config.unit_system.volume, "L");
    assert_eq!(config.version, "0.56.2");
    assert_eq!(
        config.whitelist_external_dirs,
        vec!["/home/ha/.homeassistant/www", "/home/ha/.homeassistant/"]
    );

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_events_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/events")
        .with_body(
            r#"
        [
            {
                "event": "state_changed",
                "listener_count": 5
            },
            {
                "event": "time_changed",
                "listener_count": 2
            }
        ]"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let events = client.get_events().await?;

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event, "state_changed");
    assert_eq!(events[0].listener_count, 5);
    assert_eq!(events[1].event, "time_changed");
    assert_eq!(events[1].listener_count, 2);

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_services_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/services")
        .with_body(
            r#"
        [
            {
              "domain": "browser",
              "services": [
                "browse_url"
              ]
            },
            {
              "domain": "keyboard",
              "services": [
                "volume_up",
                "volume_down"
              ]
            }
        ]"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let services = client.get_services().await?;

    assert_eq!(services.len(), 2);
    assert_eq!(services[0].domain, "browser");
    assert_eq!(services[0].services, vec!["browse_url"]);
    assert_eq!(services[1].domain, "keyboard");
    assert_eq!(services[1].services, vec!["volume_up", "volume_down"]);

    mock_server.assert_async().await;

    Ok(())
}
