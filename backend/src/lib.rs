use anyhow::{Context, Result};
use log::{debug, warn};
use std::sync::atomic::AtomicBool;
pub mod http;
pub mod redis;
pub mod student;

static LOGGING_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize the logging system using log4rs
pub fn init_logging() -> Result<()> {
    if logging_initialized() {
        return Ok(());
    }
    let result = log4rs::init_file("log4rs.yaml", Default::default())
        .context("Failed to initialize logging");
    if result.is_ok() {
        set_logging_initialized();
    }
    result
}

/// Check if the logging system is initialized
pub fn logging_initialized() -> bool {
    LOGGING_INITIALIZED.load(std::sync::atomic::Ordering::Relaxed)
}

/// Set the logging system to initialized
pub fn set_logging_initialized() {
    LOGGING_INITIALIZED.store(true, std::sync::atomic::Ordering::Relaxed);
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
