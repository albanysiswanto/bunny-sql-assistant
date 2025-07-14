use std::fs;
use std::io::{Error, ErrorKind};

const CONFIG_FILE: &str = ".bunny_db_config";

#[derive(Debug)]
pub struct DbConfig {
    pub db_type: String,
    pub url: String,
}

pub fn save_db_config(db_type: &str, url: &str) {
    let content = format!("type={}\nurl={}", db_type.trim(), url.trim());
    fs::write(CONFIG_FILE, content).expect("Failed to save the database configuration.");
}

pub fn load_db_config() -> Result<DbConfig, Error> {
    let content = fs::read_to_string(CONFIG_FILE).map_err(|e| {
        Error::new(
            ErrorKind::NotFound,
            format!("Configuration not found: {}", e),
        )
    })?;

    let mut db_type = None;
    let mut url = None;

    for line in content.lines() {
        if let Some(stripped) = line.strip_prefix("type=") {
            db_type = Some(stripped.trim().to_string());
        } else if let Some(stripped) = line.strip_prefix("url=") {
            url = Some(stripped.trim().to_string());
        }
    }

    match (db_type, url) {
        (Some(t), Some(u)) => Ok(DbConfig { db_type: t, url: u }),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid database config format.",
        )),
    }
}
