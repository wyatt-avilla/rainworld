use super::ScientificPlantName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Plant {
    pub id: u16,
    pub name: String,
    pub scientific_name: ScientificPlantName,
}
