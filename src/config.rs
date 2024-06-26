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
    #[serde(rename = "reserved")]
    Reserved,
}

impl Clone for RotateType {
    fn clone(&self) -> Self {
        match self {
            RotateType::HistoryCount => RotateType::HistoryCount,
            RotateType::StoredTime => RotateType::StoredTime,
            RotateType::TotalSize => RotateType::TotalSize,
            RotateType::Reserved => RotateType::Reserved,
        }
    }
}

#[derive(Deserialize)]
pub struct Settings  {
    pub rotate_type: RotateType,
    pub rotate_count: u32,
    pub rotate_time: u32,
    pub rotate_size: u32,
    pub enable_region_block: bool,
    pub white_region_code_list: Vec<String>,
}

impl Settings  {
    pub fn new() -> Self {
        Settings  {
            rotate_type: RotateType::HistoryCount,
            rotate_count: 100,
            rotate_time: 30,
            rotate_size: 200,
            enable_region_block: true,
            white_region_code_list: vec![String::from("SG")],
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let settings: Settings = serde_json::from_str(&contents).unwrap();
        settings
    }
    
    pub fn contains_region(&self, region: &str) -> bool {
        for white_region in &self.white_region_code_list {
            if white_region == region {
                return true;
            }
        }
        false
    }
}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Settings {
            rotate_type: self.rotate_type.clone(),
            rotate_count: self.rotate_count.clone(),
            rotate_time: self.rotate_time.clone(),
            rotate_size: self.rotate_size.clone(),
            enable_region_block: self.enable_region_block.clone(),
            white_region_code_list: self.white_region_code_list.clone(),
        }
    }
}


#[derive(Deserialize)]
pub struct Config {
    pub settings: Settings,
}

impl Config{
    fn build(settings: Settings) -> Self {
        Config {
            settings,
        }
    }

    pub fn new() -> Self {
        let config_string = match fs::read_to_string("./config/appsettings.json") {
            Ok(value) => value,
            Err(_) => {
                let default_config = Config {
                    settings: Settings::new(),
                };
                return default_config;
            }
        };
        let config: Config = match serde_json::from_str(&config_string) {
            Ok(value) => value,
            Err(_) => {
                let default_config = Config {
                    settings: Settings::new(),
                };
                return default_config;
            }
        };
        config
    }
}

// test module
#[cfg(test)]
mod tests {
    lazy_static! {
        pub static ref CONFIG_INSTANCE: Mutex<Config> = Mutex::new(Config::new());
    }
    use super::*;

    #[test]
    fn test_all_config_fields() {
        let config = match CONFIG_INSTANCE.lock(){
            Ok(value) => value,
            Err(_) => {
                assert!(false);
                return;
            }
        };
        assert!(config.settings.rotate_count > 0);
        assert!(config.settings.rotate_time > 0);
        assert!(config.settings.rotate_size > 0);
        assert_ne!(config.settings.rotate_type, RotateType::Reserved);
        assert!(config.settings.enable_region_block == true || config.settings.enable_region_block == false);
        assert!(config.settings.white_region_code_list.len() > 0);
    }
}