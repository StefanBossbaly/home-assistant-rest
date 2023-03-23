# Project Title

Homeassistant REST API Async Rust client.

## Project Description

This project provides developers with a standard and ergnomic Rust API for calling the various endpoints in
Home Assistant's REST API. It handles the serialization and deserialization of the resquests and responses
and allows the developer to use the provided structures. Currently this project is still under active developement
and the API might change at any time.

## API Status

| Endpoint                              | Request Type | Implemented        | Tested             |
| ------------------------------------- | ------------ | ------------------ | ------------------ |
| `/api/`                               | GET          | ✅                 | ✅                 |
| `/api/config`                         | GET          | ✅                 | ❌                 |
| `/api/events`                         | GET          | ✅                 | ❌                 |
| `/api/services`                       | GET          | ✅                 | ❌                 |
| `/api/history/period/<timestamp>`     | GET          | ✅                 | ❌                 |
| `/api/logbook/<timestamp>`            | GET          | ✅                 | ❌                 |
| `/api/states`                         | GET          | ✅                 | ❌                 |
| `/api/states/<entity_id>`             | GET          | ✅                 | ❌                 |
| `/api/error_log`                      | GET          | ✅                 | ❌                 |
| `/api/camera_proxy/<camera entity_id>`| GET          | ✅                 | ❌                 |
| `/api/calendars`                      | GET          | ✅                 | ❌                 |
| `/api/calendars/<calendar entity_id>` | GET          | ✅                 | ❌                 |
| `/api/states/<entity_id>`             | POST         | ✅                 | ❌                 |
| `/api/events/<event_type>`            | POST         | ❌                 | ❌                 |
| `/api/services/<domain>/<service>`    | POST         | ❌                 | ❌                 |
| `/api/template`                       | POST         | ✅                 | ❌                 |
| `/api/config/core/check_config`       | POST         | ❌                 | ❌                 |
| `/api/intent/handle`                  | POST         | ❌                 | ❌                 |

## Example

```rust
let base_url = "http://192.168.1.2:8123";
let token = "sdfef...";

let client = Client::new(base_url, token)?;
let api_status = client.api_status().await?;
dbg!(api_status);
```

## Authors

Stefan Bossbaly

## License

This project is licensed under the MIT License - see the LICENSE file for details

## Acknowledgments

Inspiration, code snippets, etc.

* [homeassistant](https://developers.home-assistant.io/docs/api/rest/)
