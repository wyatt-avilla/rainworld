use esp_idf_hal::io::EspIOError;
use esp_idf_hal::sys::EspError;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::http::Method;
use thiserror::Error;

use crate::sensors;

#[derive(Error, Debug)]
pub enum ServerCreationError {
    #[error("Couldn't register a handler function for the server")]
    FunctionRegistration(EspError),

    #[error("Couldn't create new EspHttpServer")]
    Init(EspIOError),
}

pub fn new_server() -> Result<EspHttpServer<'static>, ServerCreationError> {
    let mut server =
        EspHttpServer::new(&Configuration::default()).map_err(ServerCreationError::Init)?;

    server
        .fn_handler(shared::ESP32_ENDPOINT, Method::Get, sensors::mock_plants)
        .map_err(ServerCreationError::FunctionRegistration)?;

    Ok(server)
}
