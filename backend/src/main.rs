use anyhow::Context;
use backend::{http, init_logging};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    init_logging().context("Failed to initialize logging")?;

    info!("Starting server...");

    match http::serve().await {
        Ok(_) => {
            info!("Server shutdown gracefully");
            Ok(())
        }
        Err(e) => {
            error!("Server error: {e}");
            Err(e)
        }
    }
}
