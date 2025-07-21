use serde::{Deserialize, Serialize};

pub static ESP32_ENDPOINT: &str = "/api";

#[derive(Serialize, Deserialize)]
pub enum PlantName {
    FicusElastica,
    MonsteraDeliciosa,
}

#[derive(Serialize, Deserialize)]
pub struct PlantStatus {
    pub scientific_name: PlantName,
    pub moisture_level: u32,
}
