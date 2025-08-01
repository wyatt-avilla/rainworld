use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ScientificPlantName {
    FicusElastica,
    MonsteraDeliciosa,
    DieffenbachiaReflector,
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
