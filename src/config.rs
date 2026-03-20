use std::{env, error::Error, fmt};

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_JOB_MAX_STEPS: u32 = 4096;
const DEFAULT_JOB_TICK_MS: u64 = 20;
const DEFAULT_FRONTEND_DIR: &str = "frontend/dist";

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub job_max_steps: u32,
    pub job_tick_ms: u64,
    pub frontend_dir: String,
}

#[derive(Debug, Clone)]
pub struct ConfigError {
    field: &'static str,
    value: String,
    reason: &'static str,
}

impl ConfigError {
    fn new(field: &'static str, value: String, reason: &'static str) -> Self {
        Self {
            field,
            value,
            reason,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}=`{}`: {}", self.field, self.value, self.reason)
    }
}

impl Error for ConfigError {}

fn env_or_default(name: &str, fallback: &str) -> String {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| fallback.to_owned())
}

fn parse_u16_env(name: &'static str, fallback: u16) -> Result<u16, ConfigError> {
    let raw = env_or_default(name, &fallback.to_string());
    raw.parse::<u16>()
        .map_err(|_| ConfigError::new(name, raw, "must be an unsigned 16-bit integer"))
}

fn parse_u32_env(name: &'static str, fallback: u32) -> Result<u32, ConfigError> {
    let raw = env_or_default(name, &fallback.to_string());
    raw.parse::<u32>()
        .map_err(|_| ConfigError::new(name, raw, "must be an unsigned integer"))
}

fn parse_u64_env(name: &'static str, fallback: u64) -> Result<u64, ConfigError> {
    let raw = env_or_default(name, &fallback.to_string());
    raw.parse::<u64>()
        .map_err(|_| ConfigError::new(name, raw, "must be an unsigned integer"))
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            host: env_or_default("HOST", DEFAULT_HOST),
            port: parse_u16_env("PORT", DEFAULT_PORT)?,
            job_max_steps: parse_u32_env("JOB_MAX_STEPS", DEFAULT_JOB_MAX_STEPS)?,
            job_tick_ms: parse_u64_env("JOB_TICK_MS", DEFAULT_JOB_TICK_MS)?,
            frontend_dir: env_or_default("FRONTEND_DIR", DEFAULT_FRONTEND_DIR),
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
