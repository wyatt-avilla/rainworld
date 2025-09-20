use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use clap::Parser;
use database::DatabaseClientError;
use hardware::HardwareInterface;
use interval_readings::read_sensor_and_store_every_n_seconds;
use serde_json::{Value, json};

mod arg_parse;
mod database;
mod hardware;
mod interval_readings;

const TABLE_NAME: &str = "rainworld_readings";

async fn now_readings(
    State(hardware): State<Arc<HardwareInterface>>,
) -> Json<shared::backend::ReadingResponse> {
    Json(match hardware.get_reading().await {
        Ok(v) => v
            .into_iter()
            .map(|r| r.map_err(shared::backend::Error::Esp32))
            .collect::<Result<Vec<_>, _>>(),
        Err(e) => Err(match e {
            hardware::HardwareInterfaceError::HttpRequestGet(_) => shared::backend::Error::Http,
            hardware::HardwareInterfaceError::DeserializeError(_) => {
                shared::backend::Error::Deserialize
            }
        }),
    })
}

async fn historic_readings(
    State(db): State<Arc<database::Client>>,
) -> Json<shared::backend::ReadingResponse> {
    let resp = db
        .query(format!("select * from {TABLE_NAME}").as_str())
        .await;

    if let Err(e) = &resp {
        log::error!("Error while trying to get reading ({e})");
    }

    let resp = resp.map_err(|e| match e {
        DatabaseClientError::Http(_) => shared::backend::Error::Http,
        DatabaseClientError::Deserialize(_) => shared::backend::Error::Deserialize,
        _ => shared::backend::Error::Internal,
    });

    Json(resp)
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
        &args.influxdb_database_name.clone(),
        &args.influxdb_url,
        &args.influxdb_auth_token_file,
    )?);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port).as_str()).await?;

    let app = Router::new()
        .route("/", get(root_handler))
        .route(shared::backend::READING_NOW_ENDPOINT, get(now_readings))
        .with_state(hardware_interface.clone())
        .route("/api/read_table", get(historic_readings))
        .with_state(db_client.clone());

    log::info!("Running server on port {}", args.port);
    log::info!("Expecting ESP32 at '{}'", args.esp32_url);
    log::info!("Expecting database at '{}'", args.influxdb_url);

    tokio::spawn(read_sensor_and_store_every_n_seconds(
        hardware_interface,
        db_client,
        TABLE_NAME.to_string(),
        args.reading_interval_seconds,
    ));

    axum::serve(listener, app).await?;

    Ok(())
}
