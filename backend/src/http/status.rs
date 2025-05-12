use axum::{extract::Query, response::Json, routing::get, Router};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::http::Error;

#[derive(Debug, Deserialize)]
pub struct StatusParams {
    error: Option<bool>,
}

pub fn router() -> Router {
    Router::new().route("/status", get(status))
}

// Using axum's Result type which works with IntoResponse
pub async fn status(Query(params): Query<StatusParams>) -> Result<Json<Value>, Error> {
    // Simulate an error if requested via query param
    if params.error.unwrap_or(false) {
        return Err(anyhow::anyhow!("Simulated error requested by client")
            .context("Error processing status request")
            .into());
    }

    let timestamp = Utc::now().to_rfc3339();
    let response = json!({
        "status": "ok",
        "timestamp": timestamp
    });

    Ok(Json(response))
}
