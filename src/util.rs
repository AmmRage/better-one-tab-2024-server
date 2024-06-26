use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::sync::Mutex;
use rand::Rng;
use crate::models::user::User;
use crate::Config;
use crate::config::RotateType;
use crate::ip::Ips;

lazy_static! {
    pub static ref INSTANCE: Mutex<Config> = Mutex::new(Config::new());
    pub static ref IPS_INSTANCE: Mutex<Ips> = Mutex::new(Ips::new());
}

pub fn read_lines_from_file(filename: &str) -> Result<Vec<User>, std::io::Error> {
    // 打开文件并创建一个 BufReader 来缓冲读取
    let file = File::open(format!("./data/{}", filename))?;
    let reader = BufReader::new(file);

    // 准备一个 Vec 来存储行
    let mut lines: Vec<User> = Vec::new();

    // 遍历每一行并存储到 Vec 中
    for line in reader.lines() {
        let line = line?; // 检查并处理每一行读取可能的错误
        if line.is_empty() {
            continue;
        }
        let parts = line.split(",").collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        }
        lines.push(User {
            username: String::from(parts[0]),
            password: String::from(parts[1]),
        });
    }

    Ok(lines) // 返回 Vec
}

pub fn generate_random_string(length: usize) -> String {
    // 定义字符集
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789";
    let charset_len = charset.len();

    // 创建随机生成器
    let mut rng = rand::thread_rng();

    // 生成随机字符串
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset_len);
            charset[idx] as char
        })
        .collect();

    random_string
}

pub fn save_token_to_file(filename: String, token: String) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("./data/{}", filename))?;
    file.write_all(token.as_bytes())?;
    Ok(())
}

fn get_total_files_count_and_remove_old_files() -> Result<u32, std::io::Error> {
    let instance = INSTANCE.lock().unwrap();
    let mut removed_count = 0;
    if instance.settings.rotate_type == RotateType::HistoryCount {
        let mut files = std::fs::read_dir("./data/history")?
            .map(|res| res.map(|e| e.path()))
            .filter(|res| res.is_ok() && res.as_ref().unwrap().is_dir())
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        files.sort();
        if files.len() > instance.settings.rotate_count as usize {
            for i in 0..files.len() - instance.settings.rotate_count as usize {
                let path = files[i].as_path();
                std::fs::remove_dir_all(path)?;
                removed_count += 1;
            }
        }
    }
    Ok(removed_count)
}
fn get_dir_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let mut size = 0;
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            size += get_dir_size(entry.path())?;
        } else {
            size += metadata.len();
        }
    }
    Ok(size)
}
fn get_total_files_size_and_remove_old_files() -> Result<u32, std::io::Error> {
    let instance = INSTANCE.lock().unwrap();
    let mut removed_count = 0;
    if instance.settings.rotate_type == RotateType::TotalSize {
        let mut files = std::fs::read_dir("./data/history")?
            .map(|res| res.map(|e| e.path()))
            .filter(|res| res.is_ok() && res.as_ref().unwrap().is_dir())
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        files.sort();
        let mut total_size = 0;
        let mut path_size_map: std::collections::HashMap<&Path, u64> = std::collections::HashMap::new();

        for file in &files {
            let dir_size = get_dir_size(file.clone())?;
            path_size_map.insert(file.as_path(), dir_size);
            total_size += dir_size;
        }

        let rotate_size_in_bytes = instance.settings.rotate_size * 1024 * 1024;

        if total_size > rotate_size_in_bytes as u64 {

            for file in &files {
                std::fs::remove_dir_all(file.clone())?;
                removed_count += 1;
                let size = path_size_map.get(file.as_path()).unwrap();
                total_size -= size;
                if total_size <= rotate_size_in_bytes as u64 {
                    return Ok(removed_count);
                }
            }
        }
    }
    Ok(removed_count)
}

fn get_total_files_time_and_remove_old_files() -> Result<u32, std::io::Error> {
    let instance = INSTANCE.lock().unwrap();
    let mut removed_count = 0;
    if instance.settings.rotate_type == RotateType::StoredTime {
        let mut files = std::fs::read_dir("./data/history")?
            .map(|res| res.map(|e| e.path()))
            .filter(|res| res.is_ok() && res.as_ref().unwrap().is_dir())
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        files.sort();
        let now = chrono::Utc::now().timestamp();

        let store_time_in_seconds = instance.settings.rotate_time * 24 * 60 * 60;

        for file in &files {
            let filename = file.file_name().unwrap();
            let dir_timestamp = i64::from_str_radix(filename.to_str().unwrap(), 10).unwrap();

            if now - dir_timestamp > store_time_in_seconds as i64 {
                std::fs::remove_dir_all(file.clone())?;
                removed_count += 1;
                println!("delete: {:?}", filename);
            }
            else{
                println!("keep: {:?}", filename);
            }
        }
    }
    Ok(removed_count)
}

fn move_file_to_history(filename: &String) -> Result<(), std::io::Error> {

    if !std::path::Path::new(&format!("./data/{}", filename)).exists() {
        return Ok(());
    }

    let unix_time = chrono::Utc::now().timestamp();
    let history_dir = format!("./data/history/{}", unix_time);
    if !std::path::Path::new(&history_dir).exists() {
        std::fs::create_dir(history_dir)?;
        std::fs::copy(format!("./data/{}", filename), format!("./data/history/{}/{}", unix_time, filename))?;
    }

    get_total_files_count_and_remove_old_files()?;
    get_total_files_time_and_remove_old_files()?;
    get_total_files_size_and_remove_old_files()?;

    Ok(())
}


pub fn save_tabs_to_file(filename: &String, tabs: String) -> Result<(), std::io::Error> {
    move_file_to_history(filename)?;
    let mut file = File::create(&format!("./data/{}", filename))?;
    file.write_all(tabs.as_bytes())?;
    Ok(())
}

pub fn get_tabs_from_file(filename: String) -> Result<String, std::io::Error> {
    let file = File::open(format!("./data/{}", filename));
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            return Ok(contents);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn try_get_username_token(username: &String, token: String) -> bool {
    let filename = format!("{}.txt", username);
    let file = File::open(format!("./data/{}", filename));
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            if contents == token {
                return true;
            }
        }
        Err(_) => {}
    }
    false
}

pub fn remove_user_token(username: &String) -> Result<(), std::io::Error> {
    let filename = format!("{}.txt", username);
    std::fs::remove_file(format!("./data/{}", filename))?;
    Ok(())
}

// test module
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_total_files_count_and_remove_old_files() {
        let deleted = match get_total_files_count_and_remove_old_files() {
            Ok(value) => value,
            Err(_) => {
                assert!(false);
                return;
            }
        };
        assert_eq!(deleted, 0);
    }
    #[test]
    fn test_get_total_files_size_and_remove_old_files() {
        let deleted = match get_total_files_size_and_remove_old_files() {
            Ok(value) => value,
            Err(_) => {
                assert!(false);
                return;
            }
        };
        assert_eq!(deleted, 0);
    }
    #[test]
    fn test_get_total_files_time_and_remove_old_files() {
        let deleted = match get_total_files_time_and_remove_old_files() {
            Ok(value) => value,
            Err(_) => {
                assert!(false);
                return;
            }
        };
        assert_eq!(deleted, 0);
    }
}