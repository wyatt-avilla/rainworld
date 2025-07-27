use esp_idf_hal::{
    adc::{oneshot::AdcDriver, ADC1},
    gpio::Pins,
    sys::EspError,
};
use shared::{Plant, ScientificPlantName};

use crate::sensors::{AnyMoistureSensor, MoistureSensor};

pub struct PlantWithHardware<'s> {
    pub plant: Plant,
    pub moisture_sensor: AnyMoistureSensor<'s>,
}

pub fn plant_hardware_associations<'a>(
    driver: &'a AdcDriver<'a, ADC1>,
    pins: Pins,
) -> Result<Vec<PlantWithHardware<'a>>, EspError> {
    let mut ids = 1..;

    let v = vec![
        PlantWithHardware {
            plant: Plant {
                id: ids.next().unwrap(),
                name: String::from("fake name 1"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            moisture_sensor: AnyMoistureSensor::Gpio32(MoistureSensor::new(driver, pins.gpio32)?),
        },
        PlantWithHardware {
            plant: Plant {
                id: ids.next().unwrap(),
                name: String::from("fake name 1"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            moisture_sensor: AnyMoistureSensor::Gpio33(MoistureSensor::new(driver, pins.gpio33)?),
        },
    ];

    Ok(v)
}
