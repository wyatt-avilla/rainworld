use serde::{Deserialize, Serialize};
use thiserror::Error;

pub static ESP32_ENDPOINT: &str = "/api";

pub type APIResponse = Vec<PlantStatus>;

#[derive(Serialize, Deserialize, Clone)]
pub enum ScientificPlantName {
    FicusElastica,
    MonsteraDeliciosa,
    DieffenbachiaReflector,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Plant {
    pub id: u16,
    pub name: String,
    pub scientific_name: ScientificPlantName,
}

#[derive(Serialize, Deserialize)]
pub struct PlantStatus {
    pub plant: Plant,
    pub moisture_level: Result<u16, Esp32Error>,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Esp32Error {
    #[error("Couldn't read from moisture sensor")]
    SensorError,
}
