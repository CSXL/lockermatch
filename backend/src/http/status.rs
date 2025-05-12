use axum::{extract::Query, response::Json, routing::get, Router};
use chrono::Utc;
use log::{debug, info, warn};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::http::Error;

#[derive(Debug, Deserialize)]
pub struct StatusParams {
    error: Option<bool>,
}

pub fn router() -> Router {
    debug!("Setting up status routes");
    Router::new().route("/status", get(status))
}

// Using axum's Result type which works with IntoResponse
pub async fn status(Query(params): Query<StatusParams>) -> Result<Json<Value>, Error> {
    debug!("Status endpoint called with params: {:?}", params);

    // Simulate an error if requested via query param
    if params.error.unwrap_or(false) {
        warn!("Client requested error simulation");
        return Err(anyhow::anyhow!("Simulated error requested by client")
            .context("Error processing status request")
            .into());
    }

    let timestamp = Utc::now().to_rfc3339();
    info!("Status check successful at {}", timestamp);

    let response = json!({
        "status": "ok",
        "timestamp": timestamp
    });

    Ok(Json(response))
}
