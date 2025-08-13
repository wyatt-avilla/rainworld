use serde::{Deserialize, Serialize};

pub const DEFAULT_SERVER_PORT: u16 = 8877;

pub const READING_NOW_ENDPOINT: &str = "/api/get_reading";

pub type Response = Result<Vec<super::plant::PlantWithReadings>, Error>;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("Error while sending HTTP request")]
    Http,

    #[error("Couldn't deserialize from database response JSON")]
    Deserialize,

    #[error("Internal error occurred while trying to query data")]
    Internal,
}
