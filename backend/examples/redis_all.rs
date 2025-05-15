use anyhow::{Context, Result};
use backend::{
    http::Error,
    init_env, init_logging,
    redis::{RedisOperations, RedisPool},
};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Example struct to demonstrate serialization/deserialization with Redis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

/// Example of saving and retrieving a User from Redis
async fn user_example(redis: &RedisPool) -> Result<(), Error> {
    info!("Running user example...");

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

/// Example of storing a simple counter in Redis
async fn counter_example(redis: &RedisPool) -> Result<(), Error> {
    info!("Running counter example...");

    let counter_key = "visitor_count";

    // Increment counter using execute_command for direct command execution
    let count: i64 = redis
        .execute_command(&mut redis::cmd("INCR").arg(counter_key))
        .await?;

    info!("Visitor count: {}", count);

    Ok(())
}

/// Example of working with a Redis hash
async fn hash_example(redis: &RedisPool) -> Result<(), Error> {
    info!("Running hash example...");

    let hash_key = "product:12345";

    // Store multiple fields in a hash
    redis
        .execute_command::<()>(
            &mut redis::cmd("HSET")
                .arg(hash_key)
                .arg("name")
                .arg("Awesome Product")
                .arg("price")
                .arg(99.99)
                .arg("stock")
                .arg(42),
        )
        .await?;

    info!("Stored product data in Redis hash");

    // Get specific fields
    let name: String = redis
        .execute_command(&mut redis::cmd("HGET").arg(hash_key).arg("name"))
        .await?;

    let stock: i64 = redis
        .execute_command(&mut redis::cmd("HGET").arg(hash_key).arg("stock"))
        .await?;

    info!("Product '{}' has {} items in stock", name, stock);

    // Get all fields
    let hash_data: HashMap<String, String> = redis
        .execute_command(&mut redis::cmd("HGETALL").arg(hash_key))
        .await?;

    debug!("All product data: {:?}", hash_data);

    // Clean up
    redis
        .execute_command::<()>(&mut redis::cmd("DEL").arg(hash_key))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init_logging().context("Failed to initialize logging")?;
    info!("Starting Redis examples...");

    // Load environment variables
    init_env().context("Failed to load environment variables")?;

    // Initialize Redis connection pool
    let redis_pool = RedisPool::init()
        .await
        .context("Failed to initialize Redis")?;

    // Run all examples
    if let Err(e) = user_example(&redis_pool).await {
        warn!("User example failed: {}", e);
    }

    if let Err(e) = counter_example(&redis_pool).await {
        warn!("Counter example failed: {}", e);
    }

    if let Err(e) = hash_example(&redis_pool).await {
        warn!("Hash example failed: {}", e);
    }

    info!("Redis examples completed");
    Ok(())
}
