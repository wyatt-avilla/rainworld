use esp_idf_hal::{
    adc::{
        attenuation::DB_11,
        oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
        ADC1,
    },
    gpio::{ADCPin, Gpio32, Gpio33, Gpio34, Gpio35, Gpio39},
    sys::EspError,
};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;

pub struct MoistureSensor<'a, P: ADCPin<Adc = ADC1>> {
    driver: &'a AdcDriver<'a, ADC1>,
    channel: Mutex<CriticalSectionRawMutex, AdcChannelDriver<'a, P, &'a AdcDriver<'a, ADC1>>>,
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

pub enum AnyMoistureSensor<'a> {
    Gpio32(MoistureSensor<'a, Gpio32>),
    Gpio33(MoistureSensor<'a, Gpio33>),
    Gpio34(MoistureSensor<'a, Gpio34>),
    Gpio35(MoistureSensor<'a, Gpio35>),
    Gpio39(MoistureSensor<'a, Gpio39>),
}

impl AnyMoistureSensor<'_> {
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
