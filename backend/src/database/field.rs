pub mod strs {
    pub const MOISTURE_LEVEL: &str = "moisture_level";
}

#[derive(Hash, Eq, PartialEq)]
pub struct Field {
    key: String,
    value: FieldValue,
}

impl Field {
    pub fn new(key: &str, value: FieldValue) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }

    pub fn vec_from(
        status: &shared::esp32::PlantStatus,
    ) -> Result<Vec<Self>, shared::esp32::Esp32Error> {
        Ok(vec![Self::new(
            strs::MOISTURE_LEVEL,
            FieldValue::UInteger16(status.moisture_level.clone()?),
        )])
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum FieldValue {
    UInteger16(u16),
    String(String),
}

impl FieldValue {
    pub fn serialize(&self) -> String {
        match self {
            FieldValue::UInteger16(uint) => format!("{uint}u"),
            FieldValue::String(s) => format!("\"{s}\""),
        }
    }
}
