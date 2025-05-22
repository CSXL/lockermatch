use anyhow::Context;
use log::{debug, info};
use std::sync::Arc;

mod error;
mod status;

// Re-export our custom Error type
pub use error::Error;

pub async fn serve(redis_pool: Option<Arc<crate::redis::RedisPool>>) -> anyhow::Result<()> {
  let app = if let Some(pool) = redis_pool {
    debug!("Initializing router with Redis support");
    status::with_redis_router(pool)
  } else {
    debug!("Initializing router without Redis support");
    status::base_router()
  };

  info!("Starting HTTP server on 0.0.0.0:3000");
  debug!("Initializing API router");

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .context("Failed to bind to port 3000")?;

  info!("Server is listening on 0.0.0.0:3000");
  info!("Press Ctrl+C to stop the server");

  axum::serve(listener, app)
    .await
    .context("Failed to start server")
}
