use serde::{Serialize, Deserialize};
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
#[derive(PartialEq)]
pub enum RotateType {
    // delete old history files by the total files stored
    #[serde(rename = "history_count")]
    HistoryCount,
    // by days, delete old history files which are older than the stored time
    #[serde(rename = "stored_time")]
    StoredTime,
    // by MB, delete old history files by the total size of the history directory
    #[serde(rename = "total_size")]
    TotalSize,
}


#[derive(Deserialize)]
pub struct Settings  {
    pub rotate_type: RotateType,
    pub rotate_count: u32,
    pub rotate_time: u32,
    pub rotate_size: u32,
}

impl Settings  {
    pub fn new() -> Self {
        Settings  {
            rotate_type: RotateType::HistoryCount,
            rotate_count: 100,
            rotate_time: 30,
            rotate_size: 200,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let settings: Settings = serde_json::from_str(&contents).unwrap();
        settings
    }
}
#[derive(Deserialize)]
pub struct Config {
    pub settings: Settings,
}

impl Config{
    pub fn new() -> Self {
        let config_str: std::io::Result<String> = fs::read_to_string("config.json");
        let config: Config = serde_json::from_str(&config_str.unwrap()).unwrap();
        config
    }
}

