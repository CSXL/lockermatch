use anyhow::Context;
use axum::Router;
use log::{debug, info};

mod error;
mod status;

// Re-export our custom Error type
pub use error::Error;

pub async fn serve() -> anyhow::Result<()> {
    let app = api_router();

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

fn api_router() -> Router {
    debug!("Configuring API routes");
    status::router()
}
