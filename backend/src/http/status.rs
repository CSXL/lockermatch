use axum::{response::Json, routing::get, Router};
use chrono::Utc;
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new().route("/status", get(status))
}

pub async fn status() -> Json<Value> {
    let timestamp = Utc::now().to_rfc3339();
    let response = json!({
        "status": "ok",
        "timestamp": timestamp
    });

    Json(response)
}
