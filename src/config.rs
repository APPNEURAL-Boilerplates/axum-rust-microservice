use std::{env, num::ParseIntError};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub app_env: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub request_timeout_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        Ok(Self {
            app_name: env_string("APP_NAME", "axum-microservice"),
            app_env: env_string("APP_ENV", "local"),
            host: env_string("HOST", "0.0.0.0"),
            port: env_u16("PORT", 8080)?,
            log_level: env_string("LOG_LEVEL", "info"),
            request_timeout_seconds: env_u64("REQUEST_TIMEOUT_SECONDS", 15)?,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn test() -> Self {
        Self {
            app_name: "axum-microservice-test".to_string(),
            app_env: "test".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            log_level: "debug".to_string(),
            request_timeout_seconds: 15,
        }
    }
}

fn env_string(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_u16(key: &'static str, default: u16) -> Result<u16, ConfigError> {
    env::var(key).map_or(Ok(default), |value| {
        value
            .parse::<u16>()
            .map_err(|source| ConfigError::InvalidNumber { key, source })
    })
}

fn env_u64(key: &'static str, default: u64) -> Result<u64, ConfigError> {
    env::var(key).map_or(Ok(default), |value| {
        value
            .parse::<u64>()
            .map_err(|source| ConfigError::InvalidNumber { key, source })
    })
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid numeric environment variable {key}")]
    InvalidNumber {
        key: &'static str,
        #[source]
        source: ParseIntError,
    },
}
