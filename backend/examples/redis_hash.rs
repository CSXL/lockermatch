use anyhow::{Context, Result};
use backend::{http::Error, init_env, init_logging, redis::RedisPool};
use log::{debug, info};
use std::collections::HashMap;

/// Example of working with a Redis hash
async fn hash_example(redis: &RedisPool) -> Result<(), Error> {
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
    info!("Starting Redis hash example...");

    // Load environment variables
    init_env().context("Failed to load environment variables")?;

    // Initialize Redis connection pool
    let redis_pool = RedisPool::init()
        .await
        .context("Failed to initialize Redis")?;

    // Run the example
    hash_example(&redis_pool)
        .await
        .context("Failed to run hash example")?;

    info!("Redis hash example completed successfully");
    Ok(())
}
