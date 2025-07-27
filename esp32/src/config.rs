use esp_idf_hal::{
    adc::{oneshot::AdcDriver, ADC1},
    gpio::Pins,
    sys::EspError,
};
use shared::{Plant, ScientificPlantName};

use crate::plant_with_hardware::PlantWithHardware;
use crate::sensors::{AnyMoistureSensor, MoistureSensor};

pub fn plant_hardware_associations<'a>(
    driver: &'a AdcDriver<'a, ADC1>,
    pins: Pins,
) -> Result<Vec<PlantWithHardware<'a>>, EspError> {
    let mut ids = 1..;

    let v = vec![
        PlantWithHardware::new(
            Plant {
                id: ids.next().unwrap(),
                name: String::from("fake name 1"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            AnyMoistureSensor::Gpio32(MoistureSensor::new(driver, pins.gpio32)?),
        ),
        PlantWithHardware::new(
            Plant {
                id: ids.next().unwrap(),
                name: String::from("fake name 1"),
                scientific_name: ScientificPlantName::FicusElastica,
            },
            AnyMoistureSensor::Gpio33(MoistureSensor::new(driver, pins.gpio33)?),
        ),
    ];

    Ok(v)
}
