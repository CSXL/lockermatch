use log::{debug, info};
use redis::{Client, Connection, RedisResult};
use std::env;

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
        
        Self { url, username, password }
    }
}

/// Redis connection pool
#[derive(Clone)]
pub struct RedisPool {
    client: Client,
    config: RedisConfig,
}

impl std::fmt::Debug for RedisPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisPool")
            .field("client", &self.client)
            .field("config", &self.config)
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
        })
    }

    /// Get a Redis connection from the pool
    pub fn get_connection(&self) -> Result<Connection, Error> {
        // We'll just get a new connection each time
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
                    .map_err(|e| Error::RedisConnection(format!("Redis authentication failed: {}", e)))?;
            }
        } else if let Some(password) = &self.config.password {
            debug!("Authenticating to Redis with password only");
            redis::cmd("AUTH")
                .arg(password)
                .query::<()>(&mut conn)
                .map_err(|e| Error::RedisConnection(format!("Redis authentication failed: {}", e)))?;
        }
            
        debug!("Redis connection established");
        Ok(conn)
    }

    /// Initialize the Redis connection pool
    pub async fn init() -> Result<Self, Error> {
        let config = RedisConfig::default();
        info!("Initializing Redis connection pool with URL: {}", config.url);
        let pool = Self::new(config)?;
        
        // Test the connection
        let mut conn = pool.get_connection()?;
        let ping: RedisResult<String> = redis::cmd("PING").query(&mut conn);
        
        match ping {
            Ok(response) => {
                info!("Redis connection test successful: {}", response);
                Ok(pool)
            }
            Err(e) => Err(Error::RedisConnection(format!("Redis connection test failed: {}", e))),
        }
    }
}

/// Helper trait to simplify Redis operations
#[async_trait::async_trait]
pub trait RedisOperations {
    /// Get a value from Redis
    async fn get<T: redis::FromRedisValue + Send>(&self, key: &str) -> Result<T, Error>;
    
    /// Set a value in Redis
    async fn set<T: redis::ToRedisArgs + Send + Sync>(&self, key: &str, value: T) -> Result<(), Error>;
    
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
        let mut conn = self.get_connection()?;
        redis::cmd("GET").arg(key).query(&mut conn).map_err(Error::from)
    }
    
    async fn set<T: redis::ToRedisArgs + Send + Sync>(&self, key: &str, value: T) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        redis::cmd("SET").arg(key).arg(value).query(&mut conn).map_err(Error::from)
    }
    
    async fn set_ex<T: redis::ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
        ttl_seconds: u64,
    ) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        redis::cmd("SETEX").arg(key).arg(ttl_seconds).arg(value).query(&mut conn).map_err(Error::from)
    }
    
    async fn del(&self, key: &str) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        redis::cmd("DEL").arg(key).query(&mut conn).map_err(Error::from)
    }
    
    async fn exists(&self, key: &str) -> Result<bool, Error> {
        let mut conn = self.get_connection()?;
        redis::cmd("EXISTS").arg(key).query(&mut conn).map_err(Error::from)
    }
} 