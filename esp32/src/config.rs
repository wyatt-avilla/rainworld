use esp_idf_hal::{
    adc::{oneshot::AdcDriver, ADC1},
    gpio::Pins,
    sys::EspError,
};
use shared::plant::{Plant, ScientificPlantName};

use crate::plant_with_hardware::PlantWithHardware;
use crate::sensors::{AnyMoistureSensor, MoistureSensor};

pub fn plant_hardware_associations(
    driver: &'static mut AdcDriver<'static, ADC1>,
    pins: Pins,
) -> Result<Vec<PlantWithHardware<'static>>, EspError> {
    let mut ids = 1..;

    let v = vec![
        PlantWithHardware::new(
            Plant {
                id: ids.next().unwrap(),
                name: String::from("Top Shelf Monstera"),
                scientific_name: ScientificPlantName::MonsteraDeliciosa,
            },
            AnyMoistureSensor::Gpio32(MoistureSensor::new(driver, pins.gpio32)?),
        ),
        PlantWithHardware::new(
            Plant {
                id: ids.next().unwrap(),
                name: String::from("Middle Shelf Rubber Tree"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            AnyMoistureSensor::Gpio33(MoistureSensor::new(driver, pins.gpio33)?),
        ),
        PlantWithHardware::new(
            Plant {
                id: ids.next().unwrap(),
                name: String::from("Middle Shelf Monstera"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            AnyMoistureSensor::Gpio34(MoistureSensor::new(driver, pins.gpio34)?),
        ),
        PlantWithHardware::new(
            Plant {
                id: ids.next().unwrap(),
                name: String::from("Middle Shelf Camo"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            AnyMoistureSensor::Gpio35(MoistureSensor::new(driver, pins.gpio35)?),
        ),
        PlantWithHardware::new(
            Plant {
                id: ids.next().unwrap(),
                name: String::from("Bottom Shelf Monstera"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            AnyMoistureSensor::Gpio39(MoistureSensor::new(driver, pins.gpio39)?),
        ),
    ];

    Ok(v)
}
