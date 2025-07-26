use esp_idf_hal::{
    adc::{
        attenuation::DB_11,
        oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
        ADC1,
    },
    gpio::{ADCPin, Gpio32},
    io::{EspIOError, Write},
    peripheral::Peripheral,
    sys::EspError,
};
use esp_idf_svc::http::server::{EspHttpConnection, Request};
use thiserror::Error;

pub trait MoistureSensorPin {}

impl MoistureSensorPin for Gpio32 {}

pub fn moisture_value_from(
    adc: &AdcDriver<'_, ADC1>,
    pin: impl MoistureSensorPin + Peripheral<P = impl ADCPin<Adc = ADC1>>,
) -> Result<u16, EspError> {
    let config = AdcChannelConfig {
        attenuation: DB_11,
        ..Default::default()
    };

    let mut adc_pin = AdcChannelDriver::new(adc, pin, &config)?;

    adc.read(&mut adc_pin)
}

#[derive(Error, Debug)]
pub enum MockPlantsError {
    #[error("Couldn't convert request into response")]
    Connection(EspIOError),

    #[error("Couldn't write buffer")]
    Write(EspIOError),

    #[error("Couldn't serialize")]
    Serialization(serde_json::Error),
}

pub fn mock_plants(request: Request<&mut EspHttpConnection<'_>>) -> Result<(), MockPlantsError> {
    let plant = shared::PlantStatus {
        scientific_name: shared::PlantName::MonsteraDeliciosa,
        moisture_level: 10,
    };

    let json = serde_json::to_string(&[&plant]).map_err(MockPlantsError::Serialization)?;

    request
        .into_response(200, Some("OK"), &[("Content-Type", "application/json")])
        .map_err(MockPlantsError::Connection)?
        .write_all(json.as_bytes())
        .map_err(MockPlantsError::Write)
}
