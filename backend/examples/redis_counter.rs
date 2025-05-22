use anyhow::{Context, Result};
use backend::{http::Error, init_env, init_logging, redis::RedisPool};
use log::info;

/// Example of storing a simple counter in Redis
async fn counter_example(redis: &RedisPool) -> Result<(), Error> {
  let counter_key = "visitor_count";

  // Increment counter using execute_command for direct command execution
  let count: i64 = redis
    .execute_command(&mut redis::cmd("INCR").arg(counter_key))
    .await?;

  info!("Visitor count: {}", count);

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize logging
  init_logging().context("Failed to initialize logging")?;
  info!("Starting Redis counter example...");

  // Load environment variables
  init_env().context("Failed to load environment variables")?;

  // Initialize Redis connection pool
  let redis_pool = RedisPool::init()
    .await
    .context("Failed to initialize Redis")?;

  // Run the example
  counter_example(&redis_pool)
    .await
    .context("Failed to run counter example")?;

  info!("Redis counter example completed successfully");
  Ok(())
}
