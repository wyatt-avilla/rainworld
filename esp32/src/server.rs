use std::sync::Arc;

use esp_idf_hal::io::EspIOError;
use esp_idf_hal::sys::EspError;
use esp_idf_svc::http::server::{Configuration, EspHttpConnection, EspHttpServer, Request};
use esp_idf_svc::http::Method;
use thiserror::Error;

use crate::plant_with_hardware::PlantWithHardware;

#[derive(Error, Debug)]
pub enum ServerCreationError {
    #[error("Couldn't register a handler function for the server")]
    FunctionRegistration(EspError),

    #[error("Couldn't create new EspHttpServer")]
    Init(EspIOError),
}

fn get_handler(
    plants_with_hardware: Arc<Vec<PlantWithHardware<'_>>>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), EspIOError> + use<'_> {
    let handler = move |request: Request<&mut EspHttpConnection<'_>>| -> Result<(), EspIOError> {
        let statuses: shared::esp32::Response = plants_with_hardware
            .iter()
            .map(PlantWithHardware::status)
            .collect();

        match serde_json::to_string(&statuses) {
            Ok(json) => {
                request
                    .into_response(200, Some("OK"), &[("Content-Type", "application/json")])?
                    .write(json.as_bytes())?;
                Ok(())
            }
            Err(e) => {
                request
                    .into_status_response(500)?
                    .write(e.to_string().as_bytes())?;
                Ok(())
            }
        }
    };

    handler
}

pub fn new_server(
    plants_with_hardware: &Arc<Vec<PlantWithHardware<'static>>>,
) -> Result<EspHttpServer<'static>, ServerCreationError> {
    let mut server = EspHttpServer::new(&Configuration {
        stack_size: 10000,
        ..Configuration::default()
    })
    .map_err(ServerCreationError::Init)?;

    let plants_clone = plants_with_hardware.clone();

    server
        .fn_handler(
            shared::esp32::READ_ENDPOINT,
            Method::Get,
            get_handler(plants_clone),
        )
        .map_err(ServerCreationError::FunctionRegistration)?;

    Ok(server)
}
