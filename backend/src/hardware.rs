use thiserror::Error;

#[derive(Debug, Error)]
pub enum HardwareInterfaceError {
    #[error("Couldn't build http request ({0})")]
    HttpRequestBuild(reqwest::Error),

    #[error("Couldn't send GET request ({0})")]
    HttpRequestGet(reqwest::Error),

    #[error("Couldn't deserialize response ({0})")]
    DeserializeError(reqwest::Error),
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

    pub async fn get_reading(&self) -> Result<shared::esp32::APIResponse, HardwareInterfaceError> {
        let request = self
            .http_client
            .get(&self.read_moisture_url)
            .build()
            .map_err(HardwareInterfaceError::HttpRequestBuild)?;

        self.http_client
            .execute(request)
            .await
            .map_err(HardwareInterfaceError::HttpRequestGet)?
            .json::<shared::esp32::APIResponse>()
            .await
            .map_err(HardwareInterfaceError::DeserializeError)
    }
}
