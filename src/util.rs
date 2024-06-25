use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::Mutex;
use rand::Rng;
use crate::models::user::User;
use crate::Config;
use crate::config::RotateType;

lazy_static! {
    pub static ref INSTANCE: Mutex<Config> = Mutex::new(Config::new());
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

fn get_total_files_count_and_remove_old_files() -> Result<(), std::io::Error> {
    let instance = INSTANCE.lock().unwrap();
    if instance.settings.rotate_type == RotateType::HistoryCount {
        let mut files = std::fs::read_dir("./data/history")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        files.sort();
        if files.len() > instance.settings.rotate_count as usize {
            std::fs::remove_file(files[0].clone())?;
        }
    }
    Ok(())
}

fn get_total_files_size_and_remove_old_files() -> Result<(), std::io::Error> {
    let instance = INSTANCE.lock().unwrap();
    if instance.settings.rotate_type == RotateType::TotalSize {
        let mut files = std::fs::read_dir("./data/history")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        files.sort();
        let mut total_size = 0;
        for file in &files {
            let metadata = std::fs::metadata(file.clone())?;
            total_size += metadata.len();
        }

        if total_size > instance.settings.rotate_size as u64 {

            for file in &files {
                let metadata = std::fs::metadata(file.clone())?;
                std::fs::remove_file(files[0].clone())?;
                total_size -= metadata.len();
                if total_size <= instance.settings.rotate_size as u64 {
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

fn get_total_files_time_and_remove_old_files() -> Result<(), std::io::Error> {
    let instance = INSTANCE.lock().unwrap();
    if instance.settings.rotate_type == RotateType::StoredTime {
        let mut files = std::fs::read_dir("./data/history")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        files.sort();
        let now = chrono::Utc::now().timestamp();
        for file in &files {
            println!("file: {:?}", file);
            // if duration.num_days() > instance.settings.rotate_time as i64 {
            //     std::fs::remove_file(files[0].clone())?;
            // }
        }
    }
    Ok(())
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


pub fn save_tabs_to_file(filename: String, tabs: String) -> Result<(), std::io::Error> {
    move_file_to_history(&filename)?;
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
