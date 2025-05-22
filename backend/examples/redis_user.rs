use anyhow::{Context, Result};
use backend::{
  http::Error,
  init_env, init_logging,
  redis::{RedisOperations, RedisPool},
};
use log::{debug, info};
use serde::{Deserialize, Serialize};

/// Example struct to demonstrate serialization/deserialization with Redis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub username: String,
  pub email: String,
}

/// Example of saving and retrieving a User from Redis
async fn user_example(redis: &RedisPool) -> Result<(), Error> {
  // Create a user
  let user = User {
    id: "user123".to_string(),
    username: "johndoe".to_string(),
    email: "john@example.com".to_string(),
  };

  // Serialize user to JSON
  let user_json = serde_json::to_string(&user)
    .map_err(|e| Error::RedisParseError(format!("Failed to serialize user: {}", e)))?;

  // Store in Redis with a TTL of 1 hour
  let key = format!("user:{}", user.id);
  redis.set_ex(&key, user_json, 3600).await?;
  info!("Stored user in Redis with key: {}", key);

  // Retrieve from Redis
  if redis.exists(&key).await? {
    let user_json: String = redis.get(&key).await?;

    // Deserialize from JSON
    let retrieved_user: User = serde_json::from_str(&user_json)
      .map_err(|e| Error::RedisParseError(format!("Failed to deserialize user: {}", e)))?;

    debug!("Retrieved user from Redis: {:?}", retrieved_user);
  } else {
    debug!("User not found in Redis");
  }

  // Delete from Redis
  redis.del(&key).await?;
  info!("Deleted user from Redis with key: {}", key);

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize logging
  init_logging().context("Failed to initialize logging")?;
  info!("Starting Redis user example...");

  // Load environment variables
  init_env().context("Failed to load environment variables")?;

  // Initialize Redis connection pool
  let redis_pool = RedisPool::init()
    .await
    .context("Failed to initialize Redis")?;

  // Run the example
  user_example(&redis_pool)
    .await
    .context("Failed to run user example")?;

  info!("Redis user example completed successfully");
  Ok(())
}
