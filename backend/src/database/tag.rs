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

    pub fn vec_from(plant: &shared::esp32::Plant) -> Vec<Self> {
        vec![
            Self::new(strs::ID, &plant.id.to_string()),
            Self::new(strs::NAME, &plant.name),
            Self::new(strs::SCIENTIFIC_NAME, &plant.scientific_name.to_string()),
        ]
    }
}
