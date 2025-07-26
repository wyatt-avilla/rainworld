use std::error::Error;

use esp_idf_hal::io::EspIOError;
use esp_idf_hal::sys::EspError;
use esp_idf_svc::http::server::{Configuration, EspHttpConnection, EspHttpServer, Request};
use esp_idf_svc::http::Method;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerCreationError {
    #[error("Couldn't register a handler function for the server")]
    FunctionRegistration(EspError),

    #[error("Couldn't create new EspHttpServer")]
    Init(EspIOError),
}

pub fn new_server<E, GH>(get_handler: GH) -> Result<EspHttpServer<'static>, ServerCreationError>
where
    E: Error,
    GH: for<'r> Fn(Request<&mut EspHttpConnection<'r>>) -> Result<(), E> + Send + 'static,
{
    let mut server =
        EspHttpServer::new(&Configuration::default()).map_err(ServerCreationError::Init)?;

    server
        .fn_handler(shared::ESP32_ENDPOINT, Method::Get, get_handler)
        .map_err(ServerCreationError::FunctionRegistration)?;

    Ok(server)
}
