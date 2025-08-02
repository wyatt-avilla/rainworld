use super::field::Field;
use super::tag::Tag;
use itertools::Itertools;
use std::collections::HashSet;
use thiserror::Error;

pub trait LineProtocolElement {
    fn serialize(&self) -> String;
}

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
    pub fn escape(s: &str) -> String {
        s.replace(' ', "\\ ").replace(',', "\\,")
    }

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

    pub fn to_influx_string(&self, timestamp: u64) -> String {
        format!(
            "{},{} {} {}",
            self.table,
            self.tags
                .iter()
                .map(LineProtocolElement::serialize)
                .join(","),
            self.fields
                .iter()
                .map(LineProtocolElement::serialize)
                .join(","),
            timestamp,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::field::FieldValue;
    use super::Field;
    use super::LineProtocol;
    use super::Tag;

    #[test]
    pub fn single_tag_and_field() {
        let table_name = "fake_table_name";
        let tags = vec![Tag::new("tag_key_1", "tag_val_1")];
        let fields = vec![Field::new(
            "field_key_1",
            FieldValue::String("field_val_1".to_string()),
        )];
        let timestamp = 0u64;

        let line = LineProtocol::new(table_name, tags.into_iter(), fields.into_iter(), timestamp);

        assert_eq!(
            line.to_influx_string(timestamp),
            format!("fake_table_name,tag_key_1=tag_val_1 field_key_1=\"field_val_1\" {timestamp}")
        );
    }
}
