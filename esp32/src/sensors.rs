use esp_idf_hal::{
    adc::{
        attenuation::DB_11,
        oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
        ADC1,
    },
    gpio::{ADCPin, Gpio32, Gpio33, Gpio34, Gpio35, Gpio39},
    io::{EspIOError, Write},
    sys::EspError,
};
use esp_idf_svc::http::server::{EspHttpConnection, Request};
use thiserror::Error;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;

pub struct MoistureSensor<'d, P: ADCPin<Adc = ADC1>> {
    driver: &'d AdcDriver<'d, ADC1>,
    channel: Mutex<CriticalSectionRawMutex, AdcChannelDriver<'d, P, &'d AdcDriver<'d, ADC1>>>,
}

impl<P> MoistureSensor<'_, P>
where
    P: ADCPin<Adc = ADC1>,
{
    pub fn new<'a>(
        driver: &'a AdcDriver<'a, ADC1>,
        pin: P,
    ) -> Result<MoistureSensor<'a, P>, EspError> {
        let config = AdcChannelConfig {
            attenuation: DB_11,
            ..Default::default()
        };

        let channel = Mutex::new(AdcChannelDriver::new(driver, pin, &config)?);

        Ok(MoistureSensor { driver, channel })
    }

    pub fn read(&self) -> Result<u16, EspError> {
        unsafe { self.channel.lock_mut(|channel| self.driver.read(channel)) }
    }
}

pub enum AnyMoistureSensor<'d> {
    Gpio32(MoistureSensor<'d, Gpio32>),
    Gpio33(MoistureSensor<'d, Gpio33>),
    Gpio34(MoistureSensor<'d, Gpio34>),
    Gpio35(MoistureSensor<'d, Gpio35>),
    Gpio39(MoistureSensor<'d, Gpio39>),
}

impl<'d> AnyMoistureSensor<'d> {
    pub fn read(&self) -> Result<u16, EspError> {
        match self {
            AnyMoistureSensor::Gpio32(sensor) => sensor.read(),
            AnyMoistureSensor::Gpio33(sensor) => sensor.read(),
            AnyMoistureSensor::Gpio34(sensor) => sensor.read(),
            AnyMoistureSensor::Gpio35(sensor) => sensor.read(),
            AnyMoistureSensor::Gpio39(sensor) => sensor.read(),
        }
    }
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
    let plant = shared::Plant {
        id: 0,
        name: "fake_name".to_string(),
        scientific_name: shared::ScientificPlantName::MonsteraDeliciosa,
    };

    let json = serde_json::to_string(&[&plant]).map_err(MockPlantsError::Serialization)?;

    request
        .into_response(200, Some("OK"), &[("Content-Type", "application/json")])
        .map_err(MockPlantsError::Connection)?
        .write_all(json.as_bytes())
        .map_err(MockPlantsError::Write)
}
