//! This module provides functionality to initialize and configure a logger that exports logs
//! using OpenTelemetry. The logger can be configured with different levels and exporter endpoints.
//!
//! # Functions
//!
//! - `init`: Initializes a new logger exported with OpenTelemetry. It sets up the logger provider,
//!   configures the log appender, and sets the global logger.
//! - `http_exporter`: Creates a new HTTP exporter builder for OpenTelemetry.
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::HttpExporterBuilder;
use opentelemetry_otlp::Protocol;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::logs::BatchConfigBuilder;
use opentelemetry_sdk::Resource;

use crate::Opentelemetry;

/// Initialize a new logger exported with open telemetry.
pub fn init(
    manager: &mut Opentelemetry,
    tag: String,
    level: &str,
    exporter_endpoint: &str,
) -> anyhow::Result<()> {
    // FIXME: make this configurable from the API level
    let batch_config = BatchConfigBuilder::default()
        .with_max_export_timeout(Duration::from_secs(1))
        .with_max_queue_size(10_000)
        .with_max_export_batch_size(100_000);
    let batch_config = batch_config.build();

    let logger_provider = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            tag,
        )]))
        .with_batch_config(batch_config)
        .with_exporter(
            http_exporter()
                .with_protocol(Protocol::HttpBinary) //can be changed to `Protocol::HttpJson` to export in JSON format
                .with_endpoint(format!("{exporter_endpoint}/v1/logs")),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;
    manager.logger = Some(Arc::new(logger_provider.clone()));

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);

    // the install method set a global provider, that we can use now
    log::set_boxed_logger(Box::new(otel_log_appender)).map_err(|err| anyhow::anyhow!("{err}"))?;
    let level = log::Level::from_str(level)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

fn http_exporter() -> HttpExporterBuilder {
    opentelemetry_otlp::new_exporter().http()
}
