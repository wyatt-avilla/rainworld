use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlantWithReadings {
    pub plant: Plant,
    pub readings: Readings,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Plant {
    pub id: u16,
    pub name: String,
    pub scientific_name: ScientificPlantName,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ScientificPlantName {
    FicusElastica,
    MonsteraDeliciosa,
    DieffenbachiaReflector,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Readings {
    pub moisture_level: u16,
}

// who needs strum anyway
impl Display for ScientificPlantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ScientificPlantName::FicusElastica => "Ficus Elastica",
            ScientificPlantName::MonsteraDeliciosa => "Monstera Deliciosa",
            ScientificPlantName::DieffenbachiaReflector => "Dieffenbachia Reflector",
        })
    }
}
