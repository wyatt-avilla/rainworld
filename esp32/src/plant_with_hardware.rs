use shared::plant::{Plant, PlantWithReadings, Readings};

use crate::sensors::AnyMoistureSensor;

pub struct PlantWithHardware<'a> {
    plant: Plant,
    moisture_sensor: AnyMoistureSensor<'a>,
}

impl<'a> PlantWithHardware<'a> {
    pub fn new(plant: Plant, moisture_sensor: AnyMoistureSensor<'a>) -> PlantWithHardware<'a> {
        Self {
            plant,
            moisture_sensor,
        }
    }

    pub fn status(&self) -> Result<PlantWithReadings, shared::esp32::Error> {
        let moisture_level = self
            .moisture_sensor
            .read()
            .map_err(|_| shared::esp32::Error::Moisture)?;

        Ok(PlantWithReadings {
            plant: self.plant.clone(),
            readings: Readings { moisture_level },
            time: std::time::SystemTime::now(),
        })
    }
}
