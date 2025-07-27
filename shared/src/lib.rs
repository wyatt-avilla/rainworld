use serde::{Deserialize, Serialize};

pub static ESP32_ENDPOINT: &str = "/api";

#[derive(Serialize, Deserialize, Clone)]
pub enum ScientificPlantName {
    FicusElastica,
    MonsteraDeliciosa,
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
    pub moisture_level: u16,
}
