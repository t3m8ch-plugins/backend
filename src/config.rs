use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub plugins_dir: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let plugins_dir = env::var("PLUGINS_DIR")
            .with_context(|| "PLUGINS_DIR environment variable is required")?;

        let host = env::var("HOST")
            .unwrap_or_else(|_| "localhost".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .with_context(|| "PORT must be a valid number")?;

        let log_level = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            plugins_dir,
            host,
            port,
            log_level,
        })
    }

    pub fn addr(&self) -> (String, u16) {
        (self.host.clone(), self.port)
    }
}