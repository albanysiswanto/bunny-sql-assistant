use std::fs;
use std::io::{Error, ErrorKind};

const CONFIG_FILE: &str = ".bunny_db_url";

pub fn save_db_url(url: &str) {
    fs::write(CONFIG_FILE, url).expect("Failed to save the database connection.");
}

pub fn load_db_url() -> Result<String, Error> {
    fs::read_to_string(CONFIG_FILE)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            Error::new(
                ErrorKind::NotFound,
                format!("Configuration not found: {}", e),
            )
        })
}
