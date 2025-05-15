use anyhow::{Context, Result};
use log::{debug, warn};

pub mod http;
pub mod redis;

/// Initialize the logging system using log4rs
pub fn init_logging() -> Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).context("Failed to initialize logging")
}

/// Initialize environment variables from .env file
pub fn init_env() -> Result<()> {
    match dotenv::dotenv() {
        Ok(path) => {
            debug!("Loaded .env file from: {}", path.display());
            Ok(())
        }
        Err(e) => {
            warn!("Could not load .env file: {}", e);
            // Not finding a .env file is not a critical error
            Ok(())
        }
    }
}
