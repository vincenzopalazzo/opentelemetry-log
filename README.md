# opentelemetry-log

[![CI](https://github.com/vincent/open-source/opentelemetry-log/actions/workflows/ci.yml/badge.svg)](https://github.com/vincent/open-source/opentelemetry-log/actions/workflows/ci.yml)
[![Latest Version](https://img.shields.io/crates/v/opentelemetry-log.svg)](https://crates.io/crates/opentelemetry-log)
[![License](https://img.shields.io/crates/l/opentelemetry-log.svg)](https://github.com/vincent/open-source/opentelemetry-log/blob/main/LICENSE)

A minimal and simple OpenTelemetry log adapter that allows you to export your Rust logs to an OpenTelemetry collector.

## Features

- Export Rust logs to an OpenTelemetry collector
- Minimal and simple adapter
- Easy integration with existing logging (just `log` for now) frameworks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
opentelemetry-log = "0.1"
```

## Usage

```rust
use opentelemetry_common::Opentelemetry;

fn main() {
    let mut manager = Opentelemetry::new();
    manager.init_log("example", &args.level, &url)?;
    // Your application code
}
```

## License

This project is licensed under the GNU General Public License. See the [LICENSE](LICENSE) file for details.

## How to Deploy OpenTelemetry with Grafana

How to inspect the logs is something that is dependent on the user, but if you are starting from scratch and 
you want to learn how to work with Grafana and OpenTelemetry, I suggest starting from here: https://github.com/grafana/docker-otel-lgtm