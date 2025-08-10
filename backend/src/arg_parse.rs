use std::{io::Read, path::PathBuf};

use clap::Parser;

fn validate_nonempty_readable_auth_file(s: &str) -> Result<String, String> {
    let path = PathBuf::from(s);

    if !path.exists() {
        return Err(format!("Path '{s}' does not exist"));
    }

    if !path.is_file() {
        return Err(format!("Path '{s}' is not a file"));
    }

    match std::fs::File::open(&path) {
        Ok(mut file) => {
            let mut file_contents = String::new();
            let _ = file
                .read_to_string(&mut file_contents)
                .map_err(|e| format!("Cannot read file '{s}': {e}"))?;

            if file_contents.is_empty() {
                Err(format!("'{s}' is an empty file"))
            } else {
                Ok(file_contents.trim().to_string())
            }
        }
        Err(e) => Err(format!("Cannot read file '{s}': {e}")),
    }
}

/// Backend for rainworld
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port to run the server on
    #[arg(short, long, default_value_t = shared::backend::DEFAULT_SERVER_PORT)]
    pub port: u16,

    /// URL for accompanying ESP32
    #[arg(short, long)]
    pub esp32_url: String,

    /// Url for ``InfluxDB``
    #[arg(short, long)]
    pub influxdb_url: String,

    /// Path to file containing (only) an ``InfluxDB`` auth token
    #[arg(short('a'), long, value_parser = validate_nonempty_readable_auth_file)]
    pub influxdb_auth_token_file: String,

    /// Name of the ``InfluxDB`` database
    #[arg(short('d'), long, default_value_t = String::from("rainworld"))]
    pub influxdb_database_name: String,

    /// Interval (in seconds) between sensor readings
    #[arg(short, long, default_value_t = 300)]
    pub reading_interval_seconds: u64,

    /// Log level, one of (INFO, WARN, ERROR, DEBUG, TRACE)
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    pub log_level: tracing::Level,
}
