use serde::{Serialize, Deserialize};
use std::fs;
use std::sync::Mutex;
use log::{error, info};

pub struct Ip  {
    pub low: u32,
    pub high: u32,
    pub region: String,
}

pub struct Ips {
    pub ip_list: Vec<Ip>
}

impl Ips{
    pub fn new() -> Self {
        let contents = match fs::read_to_string("./config/dbip-country-ipv4-num.csv"){
            Ok(value) => value,
            Err(_) => {
                error!("Failed to read ip list file");
                return Self {
                    ip_list: Vec::new()
                }
            }
        };
        let lines = contents.split("\r\n").collect::<Vec<&str>>();
        let mut ip_list: Vec<Ip> = Vec::new();
        for line in lines {
            let parts = line.split(",").collect::<Vec<&str>>();
            if parts.len() != 3 {
                continue;
            }
            ip_list.push(Ip {
                low: parts[0].parse::<u32>().unwrap(),
                high: parts[1].parse::<u32>().unwrap(),
                region: String::from(parts[2]),
            });
        }
        info!("Loaded {} ip regions", ip_list.len());
        Self {
            ip_list
        }
    }

    pub fn get_region(&self, ip: u32) -> String {
        for i in 0..self.ip_list.len() {
            if ip >= self.ip_list[i].low && ip <= self.ip_list[i].high {
                return self.ip_list[i].region.clone();
            }
        }
        String::from("unknown")
    }
}

// test module
#[cfg(test)]
mod tests {
    lazy_static! {
        pub static ref IPS_INSTANCE: Mutex<Ips> = Mutex::new(Ips::new());
    }
    use super::*;

    #[test]
    fn test_es() {
        assert_eq!(IPS_INSTANCE.lock().unwrap().get_region(37347328), "ES");
    }

    #[test]
    fn test_sg() {
        assert_eq!(IPS_INSTANCE.lock().unwrap().get_region(37459967), "SG");
    }
}