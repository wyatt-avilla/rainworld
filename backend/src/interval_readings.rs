use crate::database;
use crate::hardware::{HardwareInterface, HardwareInterfaceError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntervalReadingError {
    #[error("Couldn't read sensors ({0})")]
    SensorReading(HardwareInterfaceError),

    #[error("Couldn't convert reading into LineProtocol ({0})")]
    Serialization(database::LineProtocolError),

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
        .map_err(IntervalReadingError::SensorReading)?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let line_protocols = database::LineProtocol::from(table_name, &reading, timestamp)
        .map_err(IntervalReadingError::Serialization)?;

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
