#![allow(dead_code)] // TODO: remove this

use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use clap::Parser;
use hardware::{HardwareInterface, HardwareInterfaceError};
use serde_json::{Value, json};

mod arg_parse;
mod database;
mod hardware;

async fn get_reading_handler(
    State(state): State<Arc<HardwareInterface>>,
) -> Json<Result<shared::esp32::APIResponse, HardwareInterfaceError>> {
    let reading = state.get_reading().await;
    if let Err(e) = &reading {
        log::error!("Error while trying to get reading ({e})");
    }
    Json(reading)
}

async fn root_handler() -> Json<Value> {
    let j = json!({
        "message": "hello world"
    });

    axum::Json(j)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = arg_parse::Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    let hardware_interface = Arc::new(HardwareInterface::new(&args.esp32_url));

    let db_client = Arc::new(database::Client::new(
        &args.influxdb_database_name,
        &args.influxdb_url,
        &args.influxdb_auth_token_file,
    )?);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port).as_str()).await?;

    let app = Router::new()
        .route("/", get(root_handler))
        .route(
            shared::backend::READING_NOW_ENDPOINT,
            get(get_reading_handler),
        )
        .with_state(hardware_interface)
        .with_state(db_client);

    log::info!("Running server on port {}", args.port);
    log::info!("Expecting ESP32 at '{}'", args.esp32_url);
    log::info!("Expecting database at '{}'", args.influxdb_url);

    axum::serve(listener, app).await?;

    Ok(())
}
