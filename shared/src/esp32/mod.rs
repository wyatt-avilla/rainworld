mod error;
mod plant;
mod plant_status;
mod scientific_plant_name;

pub use error::Esp32Error;
pub use plant::Plant;
pub use plant_status::PlantStatus;
pub use scientific_plant_name::ScientificPlantName;

pub const ESP32_ENDPOINT: &str = "/api";

pub type APIResponse = Vec<plant_status::PlantStatus>;
