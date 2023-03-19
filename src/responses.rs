use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ApiStatus {
    pub message: String,
}

#[derive(Deserialize)]
pub struct UnitSystemConfig {
    pub length: String,
    pub mass: String,
    pub temperature: String,
    pub volume: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub components: Vec<String>,
    pub config_dir: String,
    pub elevation: i32,
    pub latitude: f32,
    pub location_name: String,
    pub longitude: f32,
    pub time_zone: String,
    pub unit_system: UnitSystemConfig,
    pub whitelist_external_dirs: Vec<String>,
}

#[derive(Deserialize)]
pub struct Event {
    pub name: String,
    pub listener_count: i32,
}

#[derive(Deserialize)]
pub struct Service {
    pub domain: String,
    pub services: Vec<String>,
}

#[derive(Deserialize)]
pub struct HistroyAttributes {
    pub friendly_name: String,
    pub unit_of_measurement: String,
}

#[derive(Deserialize)]
pub struct History {
    pub attributes: Option<HistroyAttributes>,
    pub entity_id: Option<String>,
    pub last_changed: String,
    pub last_updated: Option<String>,
    pub state: String,
}

#[derive(Deserialize)]
pub struct Logbook {
    pub domain: String,
    pub entity_id: String,
    pub message: String,
    pub name: String,
    pub when: String, // TODO: Change to timestamp
}