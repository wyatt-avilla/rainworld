use crate::database;
use crate::hardware::{HardwareInterface, HardwareInterfaceError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntervalReadingError {
    #[error("Couldn't read sensors ({0})")]
    SensorReading(shared::esp32::Error),

    #[error("Couldn't send request to hardware ({0})")]
    HardwareContact(HardwareInterfaceError),

    #[error("Couldn't convert reading into LineProtocol")]
    Serialization,

    #[error("Couldn't write to database ({0})")]
    Write(database::DatabaseClientError),
}

pub async fn read_sensor_and_store(
    hardware: Arc<HardwareInterface>,
    database: Arc<database::Client>,
    table_name: &str,
) -> Result<(), IntervalReadingError> {
    let reading = hardware
        .get_reading()
        .await
        .map_err(IntervalReadingError::HardwareContact)?
        .into_iter()
        .collect::<Result<Vec<shared::plant::PlantWithReadings>, _>>()
        .map_err(IntervalReadingError::SensorReading)?;

    let line_protocols = database::LineProtocol::from(table_name, &reading)
        .map_err(|_| IntervalReadingError::Serialization)?;

    database
        .write(line_protocols)
        .await
        .map_err(IntervalReadingError::Write)?;

    Ok(())
}

pub async fn read_sensor_and_store_every_n_seconds(
    hardware: Arc<HardwareInterface>,
    database: Arc<database::Client>,
    table_name: String,
    seconds: u64,
) -> ! {
    let mut duration = tokio::time::interval(tokio::time::Duration::from_secs(seconds));
    loop {
        duration.tick().await;
        match read_sensor_and_store(hardware.clone(), database.clone(), &table_name).await {
            Ok(()) => {}
            Err(e) => {
                log::error!("Error while reading sensors ({e})");
            }
        }
    }
}
