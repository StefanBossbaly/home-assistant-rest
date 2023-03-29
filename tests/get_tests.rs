use std::vec;

use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use homeassistant_rest_rs::{
    get::{self, CalendarsParams, DateVariant, StateEnum},
    Client,
};
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
        .match_query("")
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
        .match_query("")
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
        .match_query("")
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
        .match_query("")
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
        .match_query("")
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

#[tokio::test]
async fn test_good_history_period_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    // TODO: Figure out why match_query doesn't work
    let mock_server = create_mock_server(
        &mut server,
        "/api/history/period/2016-12-29T11:22:33+02:00?end_time=2016-12-30T10:11:22+02:00",
    )
    // .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
    //     "end_time".to_owned(),
    //     "2016-12-30T10:11:22+02:00".to_owned(),
    // )]))
    .with_status(200)
    .with_body(
        r#"
    [
        [
            {
                "attributes": {
                    "friendly_name": "Weather Temperature",
                    "unit_of_measurement": "\u00b0C"
                },
                "entity_id": "sensor.weather_temperature",
                "last_changed": "2016-02-06T22:15:00+00:00",
                "last_updated": "2016-02-06T22:15:00+00:00",
                "state": "-3.9"
            },
            {
                "attributes": {
                    "friendly_name": "Weather Temperature",
                    "unit_of_measurement": "\u00b0C"
                },
                "entity_id": "sensor.weather_temperature",
                "last_changed": "2016-02-06T22:15:00+00:00",
                "last_updated": "2016-02-06T22:15:00+00:00",
                "state": "-1.9"
            }
        ]
    ]"#,
    )
    .create_async()
    .await;

    let start_time = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2016, 12, 29, 11, 22, 33)
        .unwrap();

    let end_time = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2016, 12, 30, 10, 11, 22)
        .unwrap();

    let client = Client::new(server.url().as_str(), "test_token")?;

    let params = get::HistoryParams {
        start_time: Some(start_time),
        end_time: Some(end_time),
        ..get::HistoryParams::default()
    };
    let history = client.get_history(params).await?;

    assert_eq!(history.len(), 1);
    assert_eq!(history[0].len(), 2);
    assert_eq!(
        history[0][0].entity_id,
        Some("sensor.weather_temperature".to_owned())
    );
    assert_eq!(history[0][0].state, Some(get::StateEnum::Decimal(-3.9)));

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_logbook_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    // TODO: Figure out why match_query doesn't work
    let mock_server = create_mock_server(&mut server, "/api/logbook/2023-03-27T23:42:00+00:00")
        .with_status(200)
        .with_body(
            r#"
        [
            {
                "context_user_id": null,
                "domain": "alarm_control_panel",
                "entity_id": "alarm_control_panel.area_001",
                "message": "changed to disarmed",
                "name": "Security",
                "when": "2020-06-20T16:44:26.127295+00:00"
            },
            {
                "context_user_id": null,
                "domain": "homekit",
                "entity_id": "alarm_control_panel.area_001",
                "message": "send command alarm_arm_night for Security",
                "name": "HomeKit",
                "when": "2020-06-21T02:59:05.759645+00:00"
            },
            {
                "context_user_id": null,
                "domain": "alarm_control_panel",
                "entity_id": "alarm_control_panel.area_001",
                "message": "changed to armed_night",
                "name": "Security",
                "when": "2020-06-21T02:59:06.015463+00:00"
            }
        ]"#,
        )
        .create_async()
        .await;

    let start_time = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2023, 3, 27, 23, 42, 00)
        .unwrap();

    let client = Client::new(server.url().as_str(), "test_token")?;
    let params = get::LogbookParams {
        start_time: Some(start_time),
        ..get::LogbookParams::default()
    };
    let logbook = client.get_logbook(params).await?;

    let timezone = FixedOffset::east_opt(0).unwrap();

    // Ensure we have 3 logbook entries
    assert_eq!(logbook.len(), 3);

    // First Logbook Entry
    assert_eq!(logbook[0].domain, Some("alarm_control_panel".to_owned()));
    assert_eq!(
        logbook[0].entity_id,
        "alarm_control_panel.area_001".to_owned()
    );
    assert_eq!(logbook[0].message, Some("changed to disarmed".to_owned()));
    assert_eq!(logbook[0].name, Some("Security".to_owned()));
    assert_eq!(
        logbook[0].when,
        Some(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 6, 20).unwrap(),
                NaiveTime::from_hms_nano_opt(16, 44, 26, 127_295_000).unwrap()
            )
            .and_local_timezone(timezone)
            .unwrap()
        )
    );

    // Second Logbook Entry
    assert_eq!(logbook[1].domain, Some("homekit".to_owned()));
    assert_eq!(
        logbook[1].entity_id,
        "alarm_control_panel.area_001".to_owned()
    );
    assert_eq!(
        logbook[1].message,
        Some("send command alarm_arm_night for Security".to_owned())
    );
    assert_eq!(logbook[1].name, Some("HomeKit".to_owned()));
    assert_eq!(
        logbook[1].when,
        Some(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 6, 21).unwrap(),
                NaiveTime::from_hms_nano_opt(2, 59, 5, 759_645_000).unwrap()
            )
            .and_local_timezone(timezone)
            .unwrap()
        )
    );

    // Third Logbook Entry
    assert_eq!(logbook[2].domain, Some("alarm_control_panel".to_owned()));
    assert_eq!(
        logbook[2].entity_id,
        "alarm_control_panel.area_001".to_owned()
    );
    assert_eq!(
        logbook[2].message,
        Some("changed to armed_night".to_owned())
    );
    assert_eq!(logbook[2].name, Some("Security".to_owned()));
    assert_eq!(
        logbook[2].when,
        Some(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 6, 21).unwrap(),
                NaiveTime::from_hms_nano_opt(2, 59, 6, 15_463_000).unwrap()
            )
            .and_local_timezone(timezone)
            .unwrap()
        )
    );

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_states_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/states")
        .match_query("")
        .with_body(
            r#"
        [
            {
                "attributes": {},
                "entity_id": "sun.sun",
                "last_changed": "2016-05-30T21:43:32.418320+00:00",
                "state": "below_horizon"
            },
            {
                "attributes": {},
                "entity_id": "process.Dropbox",
                "last_changed": "2017-05-30T21:43:32.418320+00:00",
                "state": "on"
            }
        ]"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let states = client.get_states().await?;

    let timezone = FixedOffset::east_opt(0).unwrap();

    assert_eq!(states.len(), 2);
    assert!(states[0].attributes.is_empty());
    assert_eq!(states[0].entity_id, "sun.sun");
    assert_eq!(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2016, 5, 30).unwrap(),
            NaiveTime::from_hms_nano_opt(21, 43, 32, 418_320_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap(),
        states[0].last_changed
    );
    assert_eq!(
        states[0].state,
        Some(StateEnum::String("below_horizon".to_owned()))
    );

    assert!(states[1].attributes.is_empty());
    assert_eq!(states[1].entity_id, "process.Dropbox");
    assert_eq!(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2017, 5, 30).unwrap(),
            NaiveTime::from_hms_nano_opt(21, 43, 32, 418_320_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap(),
        states[1].last_changed
    );
    assert_eq!(states[1].state, Some(StateEnum::String("on".to_owned())));

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_state_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();
    let mock_server = create_mock_server(&mut server, "/api/states/sun.sun")
        .with_body(
            r#"
        {
            "attributes":{
                "azimuth":336.34,
                "elevation":-17.67,
                "friendly_name":"Sun",
                "next_rising":"2016-05-31T03:39:14+00:00",
                "next_setting":"2016-05-31T19:16:42+00:00"
            },
            "entity_id":"sun.sun",
            "last_changed":"2016-05-30T21:43:29.204838+00:00",
            "last_updated":"2016-05-30T21:50:30.529465+00:00",
            "state":"below_horizon"
        }"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let state = client.get_states_of_entity("sun.sun").await?;

    let timezone = FixedOffset::east_opt(0).unwrap();

    assert_eq!(state.attributes["azimuth"], 336.34);
    assert_eq!(state.attributes["elevation"], -17.67);
    assert_eq!(state.attributes["friendly_name"], "Sun");
    assert_eq!(state.attributes["next_rising"], "2016-05-31T03:39:14+00:00");
    assert_eq!(
        state.attributes["next_setting"],
        "2016-05-31T19:16:42+00:00"
    );
    assert_eq!(state.entity_id, "sun.sun");
    assert_eq!(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2016, 5, 30).unwrap(),
            NaiveTime::from_hms_nano_opt(21, 43, 29, 204_838_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap(),
        state.last_changed
    );
    assert_eq!(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2016, 5, 30).unwrap(),
            NaiveTime::from_hms_nano_opt(21, 50, 30, 529_465_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap(),
        state.last_updated
    );

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_error_log_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/error_log")
        .with_body(
            r#"15-12-20 11:02:50 homeassistant.components.recorder: Found unfinished sessions
15-12-20 11:03:03 netdisco.ssdp: Error fetching description at http://192.168.1.1:8200/rootDesc.xml
15-12-20 11:04:36 homeassistant.components.alexa: Received unknown intent HelpIntent"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let log = client.get_error_log().await?;

    assert_eq!(
        log,
        r#"15-12-20 11:02:50 homeassistant.components.recorder: Found unfinished sessions
15-12-20 11:03:03 netdisco.ssdp: Error fetching description at http://192.168.1.1:8200/rootDesc.xml
15-12-20 11:04:36 homeassistant.components.alexa: Received unknown intent HelpIntent"#
    );

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_calendars_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/calendars")
        .match_query("")
        .with_body(
            r#"
        [
            {
                "entity_id": "calendar.holidays",
                "name": "National Holidays"
            },
            {
                "entity_id": "calendar.personal",
                "name": "Personal Calendar"
            }
        ]"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let calendards = client.get_calendars().await?;

    assert_eq!(calendards.len(), 2);
    assert_eq!(calendards[0].entity_id, "calendar.holidays");
    assert_eq!(calendards[0].name, "National Holidays");
    assert_eq!(calendards[1].entity_id, "calendar.personal");
    assert_eq!(calendards[1].name, "Personal Calendar");

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_good_calendars_entity_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    // TODO: Figure out why match_query doesn't work
    let mock_server = create_mock_server(&mut server, "/api/calendars/calendar.holidays?start=2022-05-01T07:00:00.000Z&end=2022-06-12T07:00:00.000Z")
        .with_body(
            r#"
        [
            {
                "summary": "Cinco de Mayo",
                "start": {
                    "date": "2022-05-05"
                },
                "end": {
                    "date": "2022-05-06"
                }
            },
            {
                "summary": "Birthday Party",
                "start": {
                    "dateTime": "2022-05-06T20:00:00-07:00"
                },
                "end": {
                    "dateTime": "2022-05-06T23:00:00-07:00"
                },
                "description": "Don't forget to bring balloons",
                "location": "Brian's House"
            }
            ]"#,
        )
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;

    let request_timezone = FixedOffset::east_opt(0).unwrap();

    let calendar_params = CalendarsParams {
        entity_id: "calendar.holidays".to_owned(),

        start_time: NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 5, 1).unwrap(),
            NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
        )
        .and_local_timezone(request_timezone)
        .unwrap(),

        end_time: NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 6, 12).unwrap(),
            NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
        )
        .and_local_timezone(request_timezone)
        .unwrap(),
    };
    let calendar_entries = client.get_calendars_of_entity(calendar_params).await?;

    assert_eq!(calendar_entries.len(), 2);

    // Check the first calendar entry
    assert_eq!(calendar_entries[0].summary, "Cinco de Mayo");
    assert_eq!(
        calendar_entries[0].start,
        DateVariant::Date(NaiveDate::from_ymd_opt(2022, 5, 5).unwrap())
    );
    assert_eq!(
        calendar_entries[0].end,
        DateVariant::Date(NaiveDate::from_ymd_opt(2022, 5, 6).unwrap())
    );
    assert_eq!(calendar_entries[0].description, None);
    assert_eq!(calendar_entries[0].location, None);

    // Check the second calendar entry
    let timezone = FixedOffset::west_opt(7 * 60 * 60).unwrap();
    assert_eq!(calendar_entries[1].summary, "Birthday Party");
    assert_eq!(
        calendar_entries[1].start,
        DateVariant::DateTime(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 5, 6).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap()
            )
            .and_local_timezone(timezone)
            .unwrap(),
        )
    );
    assert_eq!(
        calendar_entries[1].end,
        DateVariant::DateTime(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 5, 6).unwrap(),
                NaiveTime::from_hms_opt(23, 0, 0).unwrap()
            )
            .and_local_timezone(timezone)
            .unwrap(),
        )
    );
    assert_eq!(
        calendar_entries[1].description,
        Some("Don't forget to bring balloons".to_owned())
    );
    assert_eq!(
        calendar_entries[1].location,
        Some("Brian's House".to_owned())
    );

    mock_server.assert_async().await;

    Ok(())
}
