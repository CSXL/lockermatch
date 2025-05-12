use anyhow::Context;
use axum::Router;

mod error;
mod status;

// Re-export our custom Error type
pub use error::Error;

pub async fn serve() -> anyhow::Result<()> {
    let app = api_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to bind to port 3000")?;

    axum::serve(listener, app)
        .await
        .context("Failed to start server")
}

fn api_router() -> Router {
    status::router()
}
