use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Esp32Error {
    #[error("Couldn't read from moisture sensor")]
    SensorError,
}
