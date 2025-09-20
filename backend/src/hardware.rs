use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum HardwareInterfaceError {
    #[error("Couldn't send GET request ({0})")]
    HttpRequestGet(String),

    #[error("Couldn't deserialize response ({0})")]
    DeserializeError(String),
}

pub struct HardwareInterface {
    http_client: reqwest::Client,
    read_moisture_url: String,
}

impl From<HardwareInterfaceError> for shared::backend::Error {
    fn from(e: HardwareInterfaceError) -> Self {
        match e {
            HardwareInterfaceError::HttpRequestGet(_) => shared::backend::Error::Http,
            HardwareInterfaceError::DeserializeError(_) => shared::backend::Error::Deserialize,
        }
    }
}

impl HardwareInterface {
    pub fn new(read_moisture_url: &str) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            read_moisture_url: read_moisture_url.to_string(),
        }
    }

    pub async fn get_reading(&self) -> Result<shared::esp32::Reading, HardwareInterfaceError> {
        self.http_client
            .get(&self.read_moisture_url)
            .send()
            .await
            .map_err(|e| HardwareInterfaceError::HttpRequestGet(e.to_string()))?
            .json::<shared::esp32::Reading>()
            .await
            .map_err(|e| HardwareInterfaceError::DeserializeError(e.to_string()))
    }

    pub async fn get_water_valve_statuses(
        &self,
    ) -> Result<shared::esp32::WaterValveStatuses, HardwareInterfaceError> {
        log::warn!("TODO: water valve statuses unimplemented");
        Ok(Ok(std::collections::HashMap::new()))
    }
}
