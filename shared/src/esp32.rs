use serde::{Deserialize, Serialize};

pub const READ_ENDPOINT: &str = "/api/read";

pub type Response = Vec<Result<super::plant::PlantWithReadings, Error>>;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("Couldn't read from moisture sensor")]
    Moisture,
}
