//! # Async Home Assistant REST Client
//!
//! home-assistant-rest is a Home Assistant REST client for Rust.
//!
//! The Home Assistant REST specification is available [here](https://developers.home-assistant.io/docs/api/rest/).
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use home_assistant_rest::{get::StateEnum, Client};
//!
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let base_url = "REPLACE_WITH_BASE_URL"; // Replace with base url of Home Assistant instance
//!     let token = "REPLACE_WITH_TOKEN"; // Replace with token of Home Assistant instance
//!     let client = Client::new(base_url, token)?;
//!
//!     let api_status = client.get_api_status().await?;
//!     if api_status.message != "API running." {
//!         println!("API is NOT running");
//!     } else {
//!         println!("API is running, getting status of \"sun.sun\" entity");
//!         let state_entity = client.get_states_of_entity("sun.sun").await?;
//!
//!         if let Some(state) = state_entity.state {
//!             match state {
//!                 StateEnum::Boolean(x) => println!("Value is boolean with value {}", x),
//!                 StateEnum::Decimal(x) => println!("Value is decimal with value {}", x),
//!                 StateEnum::Integer(x) => println!("Value is integer with value {}", x),
//!                 StateEnum::String(x) => println!("Value is string with value \"{}\"", x),
//!             }
//!         } else {
//!             println!("Value was not provided");
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

mod client;
pub mod deserialize;
pub mod get;
pub mod post;
pub mod serialize;

pub use client::Client;
