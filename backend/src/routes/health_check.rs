use axum::response::Json;
use chrono::Utc;
use serde_json::{json, Value};

pub async fn health_check() -> Json<Value> {
    let timestamp = Utc::now().to_rfc3339();
    let response = json!({
        "status": "ok",
        "timestamp": timestamp
    });

    Json(response)
}
