use std::{fmt::Display, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlantWithReadings {
    pub plant: Plant,
    pub readings: Readings,
    pub time: SystemTime,
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

mod literals {
    pub const FICUS_ELASTICA: &str = "Ficus Elastica";
    pub const MONSTERA_DELICIOSA: &str = "Monstera Deliciosa";
    pub const DIEFFENBACHIA_REFLECTOR: &str = "Dieffenbachia Reflector";
}

// who needs strum anyway
impl Display for ScientificPlantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ScientificPlantName::FicusElastica => literals::FICUS_ELASTICA,
            ScientificPlantName::MonsteraDeliciosa => literals::MONSTERA_DELICIOSA,
            ScientificPlantName::DieffenbachiaReflector => literals::DIEFFENBACHIA_REFLECTOR,
        })
    }
}

impl TryFrom<&str> for ScientificPlantName {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            literals::FICUS_ELASTICA => Ok(ScientificPlantName::FicusElastica),
            literals::MONSTERA_DELICIOSA => Ok(ScientificPlantName::MonsteraDeliciosa),
            literals::DIEFFENBACHIA_REFLECTOR => Ok(ScientificPlantName::DieffenbachiaReflector),
            _ => Err(String::from("Unknown ScientificPlantName")),
        }
    }
}
