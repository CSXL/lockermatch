// TODO: finish error implementation

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use log::{debug, error};
use serde_json::json;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound,

    /// Return `422 Unprocessable Entity`
    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    /// Return `500 Internal Server Error` on Redis connection error
    #[error("failed to connect to Redis: {0}")]
    RedisConnection(String),

    /// Return `500 Internal Server Error` on Redis command error
    #[error("Redis command failed: {0}")]
    RedisCommand(String),

    /// Return `404 Not Found` on Redis key not found
    #[error("Redis key not found: {0}")]
    RedisKeyNotFound(String),

    /// Return `500 Internal Server Error` on Redis parsing error
    #[error("failed to parse Redis data: {0}")]
    RedisParseError(String),

    // Return `500 Internal Server Error` on an `anyhow::Error`
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    ///
    /// Try "Go to Usage" in an IDE for examples.
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    /// Convert a Redis error into our application Error
    pub fn from_redis_error(err: redis::RedisError) -> Self {
        match err.kind() {
            redis::ErrorKind::IoError => Self::RedisConnection(err.to_string()),
            redis::ErrorKind::ResponseError => {
                if err.to_string().contains("nil") {
                    Self::RedisKeyNotFound(err.to_string())
                } else {
                    Self::RedisCommand(err.to_string())
                }
            }
            redis::ErrorKind::TypeError | redis::ErrorKind::ClientError => {
                Self::RedisParseError(err.to_string())
            }
            _ => Self::Anyhow(anyhow::anyhow!(err)),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound | Self::RedisKeyNotFound(_) => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::RedisConnection(_) | Self::RedisCommand(_) | Self::RedisParseError(_) | Self::Anyhow(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();

        match &self {
            Error::Unauthorized => debug!("Unauthorized request: {}", self),
            Error::Forbidden => debug!("Forbidden request: {}", self),
            Error::NotFound => debug!("Not found: {}", self),
            Error::UnprocessableEntity { errors } => debug!("Validation errors: {:?}", errors),
            Error::RedisConnection(err) => error!("Redis connection error: {}", err),
            Error::RedisCommand(err) => error!("Redis command error: {}", err),
            Error::RedisKeyNotFound(key) => debug!("Redis key not found: {}", key),
            Error::RedisParseError(err) => error!("Redis parse error: {}", err),
            Error::Anyhow(e) => error!("Internal server error: {}", e),
        }

        let body = match self {
            Error::UnprocessableEntity { errors } => Json(json!({
                "errors": errors
            })),
            _ => Json(json!({
                "error": self.to_string()
            })),
        };

        (status, body).into_response()
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Self::from_redis_error(err)
    }
}
