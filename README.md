# Rainworld

![Rust Edition](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwyatt-avilla%2Frainworld%2Frefs%2Fheads%2Fmain%2Fbackend%2FCargo.toml&query=%24.package.edition&label=Rust%20Edition&color=%23F74C00)
![Clippy](https://img.shields.io/badge/Clippy-Check-green?logo=rust)
![Nix Flake Check](https://img.shields.io/static/v1?label=Nix%20Flake&message=Check&style=flat&logo=nixos&colorB=9173ff&logoColor=CAD3F5)

Houseplant remote management and automation with an ESP32 sensor node, Axum backend,
InfluxDB storage, and a Leptos frontend.

## Features

- Shared Rust types for the ESP32 firmware, backend API, and frontend
- ESP32 firmware for Wi-Fi connected soil moisture readings
- Backend polling loop that stores sensor readings in InfluxDB
- HTTP endpoints for current readings, historic readings, and homepage data
- Nix flakes for backend, frontend, and ESP32 development environments

## Use Case

Rainworld is a stack for monitoring houseplants (and planned to eventually support remote watering).
The ESP32 reads moisture sensors attached to individual plants and exposes the readings over HTTP.
The backend periodically polls the ESP32, writes readings to InfluxDB, and serves the data to clients.
The frontend is a Leptos/WASM UI for displaying plant state and controls.

## Installation

### Nix Compatible Systems

This repository exposes separate packages and dev shells for the backend,
frontend, and ESP32 firmware.

```sh
nix build .#backend
nix build .#frontend
nix build .#esp32
```

To enter a development shell:

```sh
nix develop .#backend
nix develop .#frontend
nix develop .#esp32
```

The backend shell includes helper commands for a local InfluxDB 3 instance:

```sh
start-influx
stop-influx
```

To flash the ESP32 firmware with Nix, plug in the board and run:

```sh
nix run .#esp32
```

### Manual

For non-Nix systems, install the Rust toolchain and the native dependencies
needed by the selected crate. The backend requires OpenSSL and pkg-config. The
ESP32 firmware requires the Espressif Rust toolchain and `espflash`.

Build the backend manually with:

```sh
cargo build --release -p backend
```

Build the frontend manually for WASM with:

```sh
rustup target add wasm32-unknown-unknown
cargo build --release -p frontend --target wasm32-unknown-unknown
```

For ESP32 firmware without Nix, use
[`espup`](https://github.com/esp-rs/espup) to install the Espressif Rust
toolchain, then build and flash the `esp32` package with
[`espflash`](https://github.com/esp-rs/espflash).

## Backend

The backend is built with [Axum](https://github.com/tokio-rs/axum). It polls the
ESP32 on a fixed interval, writes readings to InfluxDB line protocol, and serves
current and historic readings over HTTP.

### Configuration

The backend is configured through CLI arguments:

```txt
Usage: rainworld-backend [OPTIONS] --esp32-url <ESP32_URL> --influxdb-url <INFLUXDB_URL> --influxdb-auth-token-file <INFLUXDB_AUTH_TOKEN_FILE>

Options:
  -p, --port <PORT>
          Port to run the server on [default: 8877]
  -e, --esp32-url <ESP32_URL>
          URL for accompanying ESP32
  -i, --influxdb-url <INFLUXDB_URL>
          Url for ``InfluxDB``
  -a, --influxdb-auth-token-file <INFLUXDB_AUTH_TOKEN_FILE>
          Path to file containing (only) an ``InfluxDB`` auth token
  -d, --influxdb-database-name <INFLUXDB_DATABASE_NAME>
          Name of the ``InfluxDB`` database [default: rainworld]
  -r, --reading-interval-seconds <READING_INTERVAL_SECONDS>
          Interval (in seconds) between sensor readings [default: 300]
  -l, --log-level <LOG_LEVEL>
          Log level, one of (INFO, WARN, ERROR, DEBUG, TRACE) [default: INFO]
  -h, --help
          Print help
  -V, --version
          Print version
```

For example, with a local InfluxDB instance and an ESP32 at
`192.168.1.50`:

```sh
rainworld-backend \
  --esp32-url http://192.168.1.50/api/read \
  --influxdb-url http://localhost:8086 \
  --influxdb-auth-token-file ./influx.auth \
  --reading-interval-seconds 300
```

### API

| Endpoint                     | Description                                      |
| ---------------------------- | ------------------------------------------------ |
| `/`                          | Basic health-style JSON response                 |
| `/api/get/now_reading`       | Reads the ESP32 immediately and returns readings |
| `/api/get/historic_readings` | Returns readings stored in InfluxDB              |
| `/api/get/home_page`         | Bundles current, historic, and plant metadata    |

### Data Shape

The shared API types live in `shared/src`. A plant reading contains:

```rust
pub struct PlantWithReadings {
    pub plant: Plant,
    pub readings: Readings,
    pub time: SystemTime,
}
```

InfluxDB rows are written to the `rainworld_readings` measurement. Plant ID,
plant name, and scientific name are stored as tags, moisture level is stored as a
field.

## ESP32 Firmware

The firmware connects to Wi-Fi, syncs time with SNTP, reads configured ADC
moisture sensors, and exposes the readings at:

```txt
GET /api/read
```

Wi-Fi credentials are compiled from `.env` values:

```env
WIFI_SSID=my-network
WIFI_PASSWORD=my-password
```

Plant-to-pin associations are currently configured in `esp32/src/config.rs`.
The default configuration reads moisture sensors on GPIO 32, 33, 34, 35, and 39.

## Frontend

The frontend is a Leptos client-side WASM package. The Nix package builds the
WASM bundle, runs `wasm-bindgen`, optimizes the output with `wasm-opt`, and
emits a minimal `index.html` with the generated CSS.

```sh
nix build .#frontend
```

The current UI contains a graph canvas and plant control components; it is still
closer to an application scaffold than a finished dashboard.

## Repo Structure

```txt
.
|-- Cargo.toml       # Rust workspace
|-- flake.nix        # Nix packages, dev shells, and checks
|-- backend          # Axum API server and InfluxDB client
|   |-- Cargo.toml
|   `-- src
|-- esp32            # ESP32 firmware
|   |-- Cargo.toml
|   |-- sdkconfig.defaults
|   `-- src
|-- frontend         # Leptos/WASM frontend
|   |-- Cargo.toml
|   `-- src
`-- shared           # Shared API and plant data types
    |-- Cargo.toml
    `-- src
```

## Checks

The flake exposes formatting, linting, and dead-code checks:

```sh
nix build .#checks.x86_64-linux.formatting
nix build .#checks.x86_64-linux.linting
nix build .#checks.x86_64-linux.dead-code
```

The same checks are wired into the local pre-commit configuration.
