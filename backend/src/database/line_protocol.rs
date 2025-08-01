use super::field::Field;
use super::tag::Tag;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LineProtocolError {
    #[error("Malformed sensor data ({0})")]
    Sensor(shared::esp32::Esp32Error),
}

pub struct LineProtocol {
    table: String,
    tags: HashSet<Tag>,
    fields: HashSet<Field>,
    timestamp: u64,
}

impl LineProtocol {
    pub fn new(
        table_name: &str,
        tags: impl Iterator<Item = Tag>,
        fields: impl Iterator<Item = Field>,
        timestamp: u64,
    ) -> Self {
        LineProtocol {
            table: table_name.to_string(),
            tags: tags.collect(),
            fields: fields.collect(),
            timestamp,
        }
    }

    pub fn from(
        table_name: &str,
        plant_statuses: &shared::esp32::APIResponse,
        timestamp: u64,
    ) -> Result<Vec<Self>, LineProtocolError> {
        plant_statuses
            .iter()
            .map(|status| {
                Ok(LineProtocol::new(
                    table_name,
                    Tag::vec_from(&status.plant).into_iter(),
                    Field::vec_from(status)
                        .map_err(LineProtocolError::Sensor)?
                        .into_iter(),
                    timestamp,
                ))
            })
            .collect()
    }
}
