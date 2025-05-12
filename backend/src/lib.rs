use anyhow::{Context, Result};

pub mod http;

/// Initialize the logging system using log4rs
pub fn init_logging() -> Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).context("Failed to initialize logging")
}
