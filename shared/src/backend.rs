use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

pub const DEFAULT_SERVER_PORT: u16 = 8877;

pub const READING_NOW_ENDPOINT: &str = "/api/get/now_reading";
pub const HISTORIC_READING_ENDPOINT: &str = "/api/get/historic_readings";
pub const HOME_PAGE_DATA_ENDPOINT: &str = "/api/get/home_page";

pub type ReadingResponse = Result<Vec<super::plant::PlantWithReadings>, Error>;

pub type HomePageLoadResponse = Result<HomePageLoad, Error>;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("Error while sending HTTP request")]
    Http,

    #[error("Couldn't deserialize from database response JSON")]
    Deserialize,

    #[error("Internal error occurred while trying to query data")]
    Internal,

    #[error("Error with ESP32")]
    Esp32(super::esp32::Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HomePageLoad {
    pub unique_ids: BTreeSet<super::plant::ID>,
    pub current_readings: Vec<super::plant::PlantWithReadings>,
    pub historic_readings: Vec<super::plant::PlantWithReadings>,
    pub valve_statuses: HashMap<super::plant::ID, super::esp32::WaterValveStatus>,
}
