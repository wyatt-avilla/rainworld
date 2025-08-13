use super::field::Field;
use super::tag::Tag;
use itertools::Itertools;
use shared::plant::PlantWithReadings;
use std::collections::HashSet;

pub trait LineProtocolElement {
    fn serialize(&self) -> String;
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
        plants_with_readings: &[PlantWithReadings],
    ) -> Result<Vec<Self>, shared::esp32::Error> {
        plants_with_readings
            .iter()
            .map(|plant_with_readings| {
                Ok(LineProtocol::new(
                    table_name,
                    Tag::vec_from(&plant_with_readings.plant).into_iter(),
                    Field::vec_from(&plant_with_readings.readings).into_iter(),
                    plant_with_readings
                        .time
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                ))
            })
            .collect()
    }

    pub fn to_influx_string(&self) -> String {
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
            self.timestamp,
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
            line.to_influx_string(),
            format!("fake_table_name,tag_key_1=tag_val_1 field_key_1=\"field_val_1\" {timestamp}")
        );
    }
}
