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

impl HardwareInterface {
    pub fn new(read_moisture_url: &str) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            read_moisture_url: read_moisture_url.to_string(),
        }
    }

    pub async fn get_reading(&self) -> Result<shared::esp32::Response, HardwareInterfaceError> {
        self.http_client
            .get(&self.read_moisture_url)
            .send()
            .await
            .map_err(|e| HardwareInterfaceError::HttpRequestGet(e.to_string()))?
            .json::<shared::esp32::Response>()
            .await
            .map_err(|e| HardwareInterfaceError::DeserializeError(e.to_string()))
    }
}
