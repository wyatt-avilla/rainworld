use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_svc::http::server::{EspHttpConnection, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MockPlantsError {
    #[error("Couldn't convert request into response")]
    Connection(EspIOError),

    #[error("Couldn't write buffer")]
    Write(EspIOError),

    #[error("Couldn't serialize")]
    Serialization(serde_json::Error),
}

pub fn mock_plants(request: Request<&mut EspHttpConnection<'_>>) -> Result<(), MockPlantsError> {
    let plant = shared::PlantStatus {
        scientific_name: shared::PlantName::MonsteraDeliciosa,
        moisture_level: 10,
    };

    let json = serde_json::to_string(&[&plant]).map_err(MockPlantsError::Serialization)?;

    request
        .into_response(200, Some("OK"), &[("Content-Type", "application/json")])
        .map_err(MockPlantsError::Connection)?
        .write_all(json.as_bytes())
        .map_err(MockPlantsError::Write)
}
