mod backend;
mod esp32;

pub use backend::BACKEND_SERVER_PORT;

pub use esp32::APIResponse;
pub use esp32::ESP32_ENDPOINT;
pub use esp32::Esp32Error;
pub use esp32::Plant;
pub use esp32::PlantStatus;
pub use esp32::ScientificPlantName;
