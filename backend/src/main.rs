use std::collections::BTreeSet;
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

#[derive(Clone)]
struct BundledStates {
    db: Arc<database::Client>,
    hardware: Arc<HardwareInterface>,
}

async fn now_readings(
    State(state): State<BundledStates>,
) -> Json<shared::backend::ReadingResponse> {
    Json(match state.hardware.get_reading().await {
        Ok(v) => v
            .into_iter()
            .map(|r| r.map_err(shared::backend::Error::Esp32))
            .collect::<Result<Vec<_>, _>>(),
        Err(e) => {
            log::error!("Error while trying to get reading ({e})");
            Err(e.into())
        }
    })
}

async fn historic_readings(
    State(state): State<BundledStates>,
) -> Json<shared::backend::ReadingResponse> {
    let resp = state.db.historic_readings().await;

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

async fn home_page_bundle(
    State(state): State<BundledStates>,
) -> Json<shared::backend::HomePageLoadResponse> {
    let historic_readings = match state.db.historic_readings().await {
        Err(e) => {
            log::error!("Error while trying to get reading ({e})");
            return Json(Err(e.into()));
        }
        Ok(v) => v,
    };

    let unique_ids = historic_readings
        .iter()
        .map(|pwr| pwr.plant.id)
        .collect::<BTreeSet<_>>();

    let current_readings = match state.hardware.get_reading().await {
        Ok(vr) => match vr.into_iter().collect::<Result<Vec<_>, _>>() {
            Ok(v) => v,
            Err(e) => {
                log::error!("Error while trying to get reading ({e})");
                return Json(Err(shared::backend::Error::Esp32(e)));
            }
        },
        Err(e) => {
            log::error!("Error while trying to get reading ({e})");
            return Json(Err(e.into()));
        }
    };

    let valve_statuses = match state.hardware.get_water_valve_statuses().await {
        Ok(r) => match r {
            Ok(vs) => vs,
            Err(e) => {
                log::error!("Error while trying to get valve status ({e})");
                return Json(Err(shared::backend::Error::Esp32(e)));
            }
        },
        Err(e) => {
            log::error!("Error while trying to get reading ({e})");
            return Json(Err(e.into()));
        }
    };

    Json(Ok(shared::backend::HomePageLoad {
        unique_ids,
        current_readings,
        historic_readings,
        valve_statuses,
    }))
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

    let bundled_states = BundledStates {
        db: db_client.clone(),
        hardware: hardware_interface.clone(),
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .route(shared::backend::READING_NOW_ENDPOINT, get(now_readings))
        .with_state(bundled_states.clone())
        .route(
            shared::backend::HISTORIC_READING_ENDPOINT,
            get(historic_readings),
        )
        .with_state(bundled_states.clone())
        .route(
            shared::backend::HOME_PAGE_DATA_ENDPOINT,
            get(home_page_bundle),
        )
        .with_state(bundled_states.clone());

    log::info!("Running server on port {}", args.port);
    log::info!("Expecting ESP32 at '{}'", args.esp32_url);
    log::info!("Expecting database at '{}'", args.influxdb_url);

    tokio::spawn(read_sensor_and_store_every_n_seconds(
        hardware_interface,
        db_client,
        database::TABLE_NAME.to_string(),
        args.reading_interval_seconds,
    ));

    axum::serve(listener, app).await?;

    Ok(())
}
