use shared::{Plant, PlantStatus};

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

    pub fn status(&self) -> PlantStatus {
        PlantStatus {
            plant: self.plant.clone(),
            moisture_level: self
                .moisture_sensor
                .read()
                .map_err(|_| shared::Esp32Error::SensorError),
        }
    }
}
