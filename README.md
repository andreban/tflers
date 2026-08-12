# tflers

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

An asynchronous Rust client library for the [Transport for London (TfL) Unified API](https://api-portal.tfl.gov.uk/).

## Features

- **Asynchronous API**: Built on top of `reqwest` and `tokio`.
- **Strongly Typed Entities**: Full deserialization of TfL presentation entities including `StopPoint`, `Prediction` (`Arrival`), `Identifier`, `LineGroup`, `LineModeGroup`, `Crowding`, `AdditionalProperties`, and search responses.
- **SMS Code Resolution**: Easily resolve 5-digit bus stop SMS shortcodes to NaPTAN stop points (`stop_point_by_sms`).
- **Flexible Date Types**: Support for `chrono` and `time` via feature flags, defaulting to `String`.

## Installation

Add `tflers` to your `Cargo.toml`:

```toml
[dependencies]
tflers = "0.1.0"
```

To enable date/time support with `chrono` or `time`:

```toml
# Use chrono for DateTime types
tflers = { version = "0.1.0", features = ["chrono"] }

# Or use time for OffsetDateTime types
tflers = { version = "0.1.0", features = ["time"] }
```

## Quick Start

### Fetching a StopPoint

```rust,no_run
use tflers::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let stop_point = client.stop_point("490004105N").await?;

    println!("Stop Name: {:?}", stop_point.common_name);
    println!("Modes: {:?}", stop_point.modes);

    Ok(())
}
```

### Resolving SMS Shortcode to StopPoint

```rust,no_run
use tflers::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Resolves 5-digit bus stop code (e.g. 58015) via 302 redirect
    let stop_point = client.stop_point_by_sms("58015").await?;

    println!("Resolved Stop ID: {:?}", stop_point.id);
    println!("Common Name: {:?}", stop_point.common_name);

    Ok(())
}
```

### Fetching Live Arrival Predictions

```rust,no_run
use tflers::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let arrivals = client.arrivals("490004105N").await?;

    for arrival in arrivals {
        println!(
            "Line: {:?} -> Towards: {:?} (in {:?} seconds)",
            arrival.line_name, arrival.towards, arrival.time_to_station
        );
    }

    Ok(())
}
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `default` | Disables date library dependencies. Timestamps deserialize as `String`. |
| `chrono` | Enables [`chrono`](https://crates.io/crates/chrono) support (`chrono::DateTime<chrono::Utc>`). |
| `time` | Enables [`time`](https://crates.io/crates/time) support (`time::OffsetDateTime`). |

## Examples

Run any of the included examples with Cargo:

```bash
cargo run --example sms_to_naptan
cargo run --example arrivals
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
