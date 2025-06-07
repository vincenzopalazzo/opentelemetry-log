//! This module provides functionality to initialize and configure a logger that exports logs
//! using OpenTelemetry. The logger can be configured with different levels and exporter endpoints.
//!
//! # Functions
//!
//! - `init`: Initializes a new logger exported with OpenTelemetry. It sets up the logger provider,
//!   configures the log appender, and sets the global logger.
//! - `http_exporter`: Creates a new HTTP exporter builder for OpenTelemetry.
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::{LogExporter, Protocol, WithExportConfig};
use opentelemetry_sdk::logs::{BatchLogProcessor, LoggerProviderBuilder};
use opentelemetry_sdk::Resource;
use std::str::FromStr;
use std::sync::Arc;

use crate::Opentelemetry;

/// Initialize a new logger exported with open telemetry.
pub fn init(
    manager: &mut Opentelemetry,
    tag: String,
    level: &str,
    exporter_endpoint: &str,
) -> anyhow::Result<()> {
    // Create the OTLP exporter using the new reqwest client by default
    let exporter = LogExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint(format!("{exporter_endpoint}/v1/logs"))
        .build()?;

    // Create the batch processor
    let processor = BatchLogProcessor::builder(exporter).build();

    // Create the logger provider
    let logger_provider = LoggerProviderBuilder::default()
        .with_resource(
            Resource::builder()
                .with_attributes(vec![KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    tag,
                )])
                .build(),
        )
        .with_log_processor(processor)
        .build();

    manager.logger = Some(Arc::new(logger_provider.clone()));

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);

    // the install method set a global provider, that we can use now
    log::set_boxed_logger(Box::new(otel_log_appender)).map_err(|err| anyhow::anyhow!("{err}"))?;
    let level = log::Level::from_str(level)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}
