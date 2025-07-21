use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_hal::sys::EspError;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::http::Method;
use thiserror::Error;

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
        .fn_handler("/index.html", Method::Get, |request| {
            request
                .into_ok_response()?
                .write_all(b"<html><body>Hello world!</body></html>")
        })
        .map_err(ServerCreationError::FunctionRegistration)?;

    Ok(server)
}
