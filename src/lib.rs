//! # OpenTelemetry Log Integration
//!
//! This crate provides integration with OpenTelemetry for logging purposes. It allows you to
//! initialize and manage loggers that are compatible with the OpenTelemetry SDK.
//!
//! ## Features
//!
//! - Initialize loggers with specific tags, levels, and exporter endpoints.
//! - Automatically manage the lifecycle of loggers, ensuring proper shutdown.
//!
//! ## Usage
//!
//! Add this crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! opentelemetry-log = "0.1"
//! ```
//!
//! Import and use the `Opentelemetry` struct to manage your loggers:
//!
//! ```rust
//! use opentelemetry_log::Opentelemetry;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut otel = Opentelemetry::new();
//!     otel.init_log("my_app", "info", "http://localhost:4317").unwrap(); // Please do not unwrap in production code
//!     // Your application logic here
//! }
//! ```
//!
//! ## Modules
//!
//! - `log`: Contains the log initialization logic.
//!
//! ## Structs
//!
//! - `Opentelemetry`: Main struct for managing OpenTelemetry loggers.
//!
//! ## Traits
//!
//! - `Default`: Provides a default implementation for `Opentelemetry`.
//! - `Drop`: Ensures proper shutdown of loggers when `Opentelemetry` instances are dropped.
//!
//! ## Errors
//!
//! This crate uses the `anyhow` crate for error handling. Ensure you handle errors appropriately
//! when initializing and using loggers.
pub mod log;
pub use anyhow;

use std::sync::Arc;

use opentelemetry_sdk::logs as sdklogs;

/// Main struct for managing OpenTelemetry loggers, when you init the logger
/// remember to keep this alive for all the lifetime of the application.
///
/// An example can be found in the `examples` directory.
#[derive(Debug, Clone)]
pub struct Opentelemetry {
    pub(crate) logger: Option<Arc<sdklogs::LoggerProvider>>,
}

impl Default for Opentelemetry {
    fn default() -> Self {
        Self::new()
    }
}

impl Opentelemetry {
    pub fn new() -> Self {
        Opentelemetry { logger: None }
    }

    /// Initialize a new logger with the provided tag, level, and exporter endpoint.
    /// this is assuming tat your application is using `log` crate
    pub fn init_log(
        &mut self,
        tag: &str,
        level: &str,
        exporter_endpoint: &str,
    ) -> anyhow::Result<()> {
        log::init(self, tag.to_owned(), level, exporter_endpoint)?;
        Ok(())
    }
}

impl Drop for Opentelemetry {
    fn drop(&mut self) {
        let Some(Err(err)) = self.logger.as_ref().map(|log| log.shutdown()) else {
            return;
        };
        eprintln!("Failed to shutdown logger: {:?}", err);
    }
}
