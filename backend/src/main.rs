use anyhow::Context;
use backend::{http, init_env, init_logging, redis::RedisPool};
use log::{error, info, warn};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    init_logging().context("Failed to initialize logging")?;

    info!("Starting server...");

    // Load environment variables
    init_env().context("Failed to load environment variables")?;

    // Initialize Redis connection pool
    let redis_pool = match RedisPool::init().await {
        Ok(pool) => {
            info!("Redis connection established");
            Some(Arc::new(pool))
        }
        Err(e) => {
            warn!("Failed to initialize Redis connection: {}", e);
            warn!("Server will start without Redis support");
            None
        }
    };

    match http::serve(redis_pool).await {
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
