use std::time::SystemTime;

use chrono::DateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InfluxResponse {
    time: String,
    id: String,
    name: String,
    scientific_name: String,
    moisture_level: u16,
}

impl TryInto<shared::plant::PlantWithReadings> for InfluxResponse {
    type Error = String;

    fn try_into(self) -> Result<shared::plant::PlantWithReadings, Self::Error> {
        let time: SystemTime = DateTime::parse_from_rfc3339(format!("{}z", self.time).as_str())
            .map_err(|e| format!("Couldn't parse time ({e})"))?
            .into();

        let id: u16 = self
            .id
            .parse()
            .map_err(|e| format!("Couldn't parse ID ({e})"))?;

        let scientific_name: shared::plant::ScientificPlantName = self
            .scientific_name
            .as_str()
            .try_into()
            .map_err(|e| format!("Couldn't parse scientific name ({e})"))?;

        Ok(shared::plant::PlantWithReadings {
            plant: shared::plant::Plant {
                id,
                name: self.name.clone(),
                scientific_name,
            },
            readings: shared::plant::Readings {
                moisture_level: self.moisture_level,
            },
            time,
        })
    }
}
