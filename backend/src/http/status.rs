use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use chrono::Utc;
use log::{debug, info, warn};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::http::Error;
use crate::redis::{RedisOperations, RedisPool};

#[derive(Debug, Deserialize)]
pub struct StatusParams {
    error: Option<bool>,
}

/// Create the base router without Redis functionality
pub fn base_router() -> Router {
    debug!("Setting up base status routes");
    Router::new()
        .route("/status", get(status))
}

/// Create a router with Redis state
pub fn with_redis_router(redis_pool: Arc<RedisPool>) -> Router {
    debug!("Setting up router with Redis support");
    Router::new()
        .route("/status", get(status))
        .route("/redis/status", get(redis_status_handler))
        .with_state(redis_pool)
}

/// Handler function with explicit Redis state
async fn redis_status_handler(
    query: Query<StatusParams>,
    state: State<Arc<RedisPool>>,
) -> Result<Json<Value>, Error> {
    redis_status(query, state).await
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

/// Status endpoint that also checks Redis connection
pub async fn redis_status(
    Query(params): Query<StatusParams>,
    State(redis_pool): State<Arc<RedisPool>>,
) -> Result<Json<Value>, Error> {
    debug!("Redis status endpoint called with params: {:?}", params);

    // Simulate an error if requested via query param
    if params.error.unwrap_or(false) {
        warn!("Client requested error simulation");
        return Err(anyhow::anyhow!("Simulated error requested by client")
            .context("Error processing redis status request")
            .into());
    }

    let timestamp = Utc::now().to_rfc3339();
    // Store the current timestamp in Redis
    let key = "last_status_check";
    redis_pool.set(key, &timestamp).await?;
    // Retrieve and increment the hit counter
    let mut conn = redis_pool.get_connection()?;
    let hits: i64 = redis::cmd("INCR")
        .arg("status_hits")
        .query(&mut conn)
        .map_err(Error::from)?;
    info!("Redis status check successful at {} (hit count: {})", timestamp, hits);

    let response = json!({
        "status": "ok",
        "redis_status": "connected",
        "timestamp": timestamp,
        "hit_count": hits
    });

    Ok(Json(response))
}
