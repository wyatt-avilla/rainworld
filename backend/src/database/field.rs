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

    pub fn vec_from(readings: &shared::plant::Readings) -> Vec<Self> {
        vec![Self::new(
            strs::MOISTURE_LEVEL,
            FieldValue::UInteger16(readings.moisture_level),
        )]
    }
}

impl super::line_protocol::LineProtocolElement for Field {
    fn serialize(&self) -> String {
        format!("{}={}", self.key, self.value.serialize())
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum FieldValue {
    UInteger16(u16),
    #[allow(dead_code)]
    String(String),
}

impl super::line_protocol::LineProtocolElement for FieldValue {
    fn serialize(&self) -> String {
        match self {
            FieldValue::UInteger16(uint) => format!("{uint}u"),
            FieldValue::String(s) => {
                format!("\"{}\"", super::line_protocol::LineProtocol::escape(s))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::line_protocol::LineProtocolElement;

    #[test]
    pub fn field_value_one_space_escape() {
        let val = super::FieldValue::String(String::from("with space"));
        assert_eq!(val.serialize(), "\"with\\ space\"");
    }

    #[test]
    pub fn field_value_two_space_escape() {
        let val = super::FieldValue::String(String::from("with two spaces"));
        assert_eq!(val.serialize(), "\"with\\ two\\ spaces\"");
    }

    #[test]
    pub fn field_value_one_comma_escape() {
        let val = super::FieldValue::String(String::from("with,comma"));
        assert_eq!(val.serialize(), "\"with\\,comma\"");
    }

    #[test]
    pub fn field_value_two_comma_escape() {
        let val = super::FieldValue::String(String::from("with,two,commas"));
        assert_eq!(val.serialize(), "\"with\\,two\\,commas\"");
    }

    #[test]
    pub fn field_value_mixed_escape() {
        let val = super::FieldValue::String(String::from("with a,space and, some  commas"));
        assert_eq!(
            val.serialize(),
            "\"with\\ a\\,space\\ and\\,\\ some\\ \\ commas\""
        );
    }
}
