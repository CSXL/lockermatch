use anyhow::Context;
use axum::Router;

mod error;
mod status;

pub async fn serve() -> anyhow::Result<()> {
    let app = api_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Failed to start server")
}

fn api_router() -> Router {
    status::router()
}
