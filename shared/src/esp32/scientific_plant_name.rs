use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ScientificPlantName {
    FicusElastica,
    MonsteraDeliciosa,
    DieffenbachiaReflector,
}
