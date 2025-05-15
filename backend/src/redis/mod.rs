use log::{debug, info};
use redis::{Client, Connection, RedisResult};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::http::Error;

pub mod examples;

/// Redis connection configuration
#[derive(Debug, Clone)]
pub struct RedisConfig {
    /// Redis connection URL (redis://...)
    pub url: String,
    /// Redis username (optional)
    pub username: Option<String>,
    /// Redis password (optional)
    pub password: Option<String>,
}

impl Default for RedisConfig {
    fn default() -> Self {
        // Get Redis URL from environment or use default
        let url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        // Get authentication details if provided
        let username = env::var("REDIS_USERNAME").ok().filter(|s| !s.is_empty());
        let password = env::var("REDIS_PASSWORD").ok().filter(|s| !s.is_empty());

        if username.is_some() || password.is_some() {
            debug!("Redis authentication credentials found");
        }

        Self {
            url,
            username,
            password,
        }
    }
}

/// Redis connection pool with shared connection
#[derive(Clone)]
pub struct RedisPool {
    client: Client,
    config: RedisConfig,
    connection: Arc<Mutex<Option<Connection>>>,
}

impl std::fmt::Debug for RedisPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisPool")
            .field("client", &self.client)
            .field("config", &self.config)
            .field("connection", &"<Redis Connection>")
            .finish()
    }
}

impl RedisPool {
    /// Create a new Redis connection pool with the given configuration
    pub fn new(config: RedisConfig) -> Result<Self, Error> {
        debug!("Creating Redis client with URL: {}", config.url);
        let client = Client::open(config.url.clone())
            .map_err(|e| Error::RedisConnection(format!("Failed to create Redis client: {}", e)))?;

        Ok(Self {
            client,
            config,
            connection: Arc::new(Mutex::new(None)),
        })
    }

    /// Create a new authenticated connection to Redis
    fn create_connection(&self) -> Result<Connection, Error> {
        debug!("Creating new Redis connection");
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| Error::RedisConnection(format!("Failed to connect to Redis: {}", e)))?;

        // Apply authentication if needed
        if let Some(username) = &self.config.username {
            if let Some(password) = &self.config.password {
                debug!("Authenticating to Redis with username");
                redis::cmd("AUTH")
                    .arg(username)
                    .arg(password)
                    .query::<()>(&mut conn)
                    .map_err(|e| {
                        Error::RedisConnection(format!("Redis authentication failed: {}", e))
                    })?;
            }
        } else if let Some(password) = &self.config.password {
            debug!("Authenticating to Redis with password only");
            redis::cmd("AUTH")
                .arg(password)
                .query::<()>(&mut conn)
                .map_err(|e| {
                    Error::RedisConnection(format!("Redis authentication failed: {}", e))
                })?;
        }

        debug!("Redis connection established");
        Ok(conn)
    }

    /// Get a Redis connection from the pool or create a new one
    pub async fn get_connection(&self) -> Result<Connection, Error> {
        let mut conn_guard = self.connection.lock().await;

        // Check if we already have a connection
        match conn_guard.take() {
            Some(mut conn) => {
                // Test if the connection is still valid with a PING
                let ping_result: Result<String, redis::RedisError> =
                    redis::cmd("PING").query(&mut conn);

                if ping_result.is_ok() {
                    debug!("Reusing existing Redis connection");
                    // Return the connection for use
                    Ok(conn)
                } else {
                    debug!("Existing connection is invalid, creating new one");
                    // Connection is not valid, create a new one
                    let conn = self.create_connection()?;
                    Ok(conn)
                }
            }
            None => {
                // No connection exists, create a new one
                debug!("No existing connection, creating new one");
                let conn = self.create_connection()?;
                Ok(conn)
            }
        }
    }

    /// Initialize the Redis connection pool and establish an initial connection
    pub async fn init() -> Result<Self, Error> {
        let config = RedisConfig::default();
        info!(
            "Initializing Redis connection pool with URL: {}",
            config.url
        );
        let pool = Self::new(config)?;

        // Test the connection to make sure Redis is available
        {
            let mut conn = pool.create_connection()?;

            // Test the connection with PING
            let ping_result = redis::cmd("PING").query::<String>(&mut conn).map_err(|e| {
                Error::RedisConnection(format!("Redis connection test failed: {}", e))
            })?;

            info!("Redis connection test successful: {}", ping_result);

            // Store the initial connection in the pool
            let mut conn_guard = pool.connection.lock().await;
            *conn_guard = Some(conn);
        }

        Ok(pool)
    }

    /// Execute a Redis command with automatic connection management
    pub async fn execute_command<T: redis::FromRedisValue>(
        &self,
        cmd: &mut redis::Cmd,
    ) -> Result<T, Error> {
        // Get a connection from the pool
        let mut conn = self.get_connection().await?;
        // Execute the command
        let result = cmd.query(&mut conn).map_err(Error::from)?;
        // Return the connection to the pool
        let mut conn_guard = self.connection.lock().await;
        *conn_guard = Some(conn);
        Ok(result)
    }
}

/// Helper trait to simplify Redis operations
#[async_trait::async_trait]
pub trait RedisOperations {
    /// Get a value from Redis
    async fn get<T: redis::FromRedisValue + Send>(&self, key: &str) -> Result<T, Error>;

    /// Set a value in Redis
    async fn set<T: redis::ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
    ) -> Result<(), Error>;

    /// Set a value in Redis with an expiration (in seconds)
    async fn set_ex<T: redis::ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
        ttl_seconds: u64,
    ) -> Result<(), Error>;

    /// Delete a key from Redis
    async fn del(&self, key: &str) -> Result<(), Error>;

    /// Check if a key exists in Redis
    async fn exists(&self, key: &str) -> Result<bool, Error>;
}

#[async_trait::async_trait]
impl RedisOperations for RedisPool {
    async fn get<T: redis::FromRedisValue + Send>(&self, key: &str) -> Result<T, Error> {
        self.execute_command(&mut redis::cmd("GET").arg(key)).await
    }

    async fn set<T: redis::ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
    ) -> Result<(), Error> {
        self.execute_command(&mut redis::cmd("SET").arg(key).arg(value))
            .await
    }

    async fn set_ex<T: redis::ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
        ttl_seconds: u64,
    ) -> Result<(), Error> {
        self.execute_command(&mut redis::cmd("SETEX").arg(key).arg(ttl_seconds).arg(value))
            .await
    }

    async fn del(&self, key: &str) -> Result<(), Error> {
        self.execute_command(&mut redis::cmd("DEL").arg(key)).await
    }

    async fn exists(&self, key: &str) -> Result<bool, Error> {
        self.execute_command(&mut redis::cmd("EXISTS").arg(key))
            .await
    }
}
