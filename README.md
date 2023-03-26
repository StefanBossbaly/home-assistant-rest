# Project Title

Home Assistant REST API Async Rust client.

## Project Description

This project provides developers with a standard and ergonomic Rust API for calling the various endpoints in
Home Assistant's REST API. It handles the serialization and deserialization of the requests and responses
and allows the developer to use the provided structures. Currently this project is still under active development
and the API might change at any time.

## API Status

| Endpoint                               | Request Type | Implemented | Tested |
| -------------------------------------- | ------------ | ----------- | ------ |
| `/api/`                                | GET          | ✅          | ✅     |
| `/api/config`                          | GET          | ✅          | ✅     |
| `/api/events`                          | GET          | ✅          | ✅     |
| `/api/services`                        | GET          | ✅          | ✅     |
| `/api/history/period/<timestamp>`      | GET          | ✅          | ❌     |
| `/api/logbook/<timestamp>`             | GET          | ✅          | ❌     |
| `/api/states`                          | GET          | ✅          | ✅     |
| `/api/states/<entity_id>`              | GET          | ❌          | ❌     |
| `/api/error_log`                       | GET          | ✅          | ✅     |
| `/api/camera_proxy/<camera entity_id>` | GET          | ❌          | ❌     |
| `/api/calendars`                       | GET          | ✅          | ✅     |
| `/api/calendars/<calendar entity_id>`  | GET          | ✅          | ❌     |
| `/api/states/<entity_id>`              | POST         | ✅          | ❌     |
| `/api/events/<event_type>`             | POST         | ❌          | ❌     |
| `/api/services/<domain>/<service>`     | POST         | ❌          | ❌     |
| `/api/template`                        | POST         | ✅          | ❌     |
| `/api/config/core/check_config`        | POST         | ❌          | ❌     |
| `/api/intent/handle`                   | POST         | ❌          | ❌     |

## Example

```rust
let base_url = "http://192.168.1.2:8123";
let token = "sdfef...";

let client = Client::new(base_url, token)?;
let api_status = client.api_status().await?;
dbg!(api_status);
```

## Differences between the specification and implementation

Home Assistant provides a [API specification](https://developers.home-assistant.io/docs/api/rest/) that lists in the various different endpoints, their parameters and their output. However when implementing this library I have come across a number of discrepancies between the specification and the my local Home Assistant instance. Here is a list of the differences I have found so far:

1. For the `/api/services` endpoint, the `services` attribute is listed in the example as a list of strings. However testing locally the type for the `services` attribute is actually a map.
2. For the `/api/camera_proxy/<camera entity_id>` endpoint, the only listed parameters are the camera id and the time. However looking at the developer tools panel, it shows that a unique token is also needed in order to retrieve the image.
3. Serialization of non-string types, especially for the `state` attribute. I have seen many entities that report their state as either an integer, decimal or boolean but when that state is serialized in the JSON response, the type is a string and not the underlying type. In order to work around this issue, when deserializing the state we attempt to deserialize into the underlying types first if that fails then we default to a string type. This is not ideal as it is costly to parse various different types. In the future I will consider adding an option that allows the user to opt-out of this "feature".

## Authors

Stefan Bossbaly

## License

This project is licensed under the MIT License - see the LICENSE file for details

## Acknowledgments

Inspiration, code snippets, etc.

- [Home Assistant REST API](https://developers.home-assistant.io/docs/api/rest/)
