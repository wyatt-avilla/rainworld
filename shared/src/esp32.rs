use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub const READ_ENDPOINT: &str = "/api/read";

pub type Reading = Vec<Result<super::plant::PlantWithReadings, Error>>;
pub type WaterValveStatuses = Result<HashMap<super::plant::ID, WaterValveStatus>, Error>;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("Couldn't read from moisture sensor")]
    Moisture,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum WaterValveStatus {
    Open,
    Closed,
}
