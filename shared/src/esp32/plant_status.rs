use super::Esp32Error;
use super::Plant;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlantStatus {
    pub plant: Plant,
    pub moisture_level: Result<u16, Esp32Error>,
}
