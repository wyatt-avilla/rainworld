pub mod strs {
    pub const ID: &str = "id";
    pub const NAME: &str = "name";
    pub const SCIENTIFIC_NAME: &str = "scientific_name";
}

#[derive(Hash, Eq, PartialEq)]
pub struct Tag {
    key: String,
    value: String,
}

impl Tag {
    pub fn new(key: &str, value: &str) -> Self {
        Tag {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn vec_from(plant: &shared::plant::Plant) -> Vec<Self> {
        vec![
            Self::new(strs::ID, &plant.id.to_string()),
            Self::new(strs::NAME, &plant.name),
            Self::new(strs::SCIENTIFIC_NAME, &plant.scientific_name.to_string()),
        ]
    }
}

impl super::line_protocol::LineProtocolElement for Tag {
    fn serialize(&self) -> String {
        let esc = super::line_protocol::LineProtocol::escape;
        format!("{}={}", esc(&self.key), esc(&self.value))
    }
}
