use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub github_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    #[serde(default)]
    pub github_client_id: String,
    #[serde(default)]
    pub github_client_secret: String,
    #[serde(default)]
    pub github_redirect_url: String,
    #[serde(default)]
    pub admin_github_ids: Vec<i64>,
    #[serde(default = "default_session_ttl_seconds")]
    pub session_ttl_seconds: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            github_client_id: String::new(),
            github_client_secret: String::new(),
            github_redirect_url: String::new(),
            admin_github_ids: Vec::new(),
            session_ttl_seconds: default_session_ttl_seconds(),
        }
    }
}

fn default_session_ttl_seconds() -> u64 {
    60 * 60 * 24 * 7
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let mut builder = ConfigBuilder::builder();

        let path = format!("{}/src/config/toml/{env}.toml", env!("CARGO_MANIFEST_DIR"));
        let contents = std::fs::read_to_string(&path).map_err(|e| {
            ConfigError::Message(format!("Failed to read config file {}: {}", path, e))
        })?;

        builder = builder.add_source(File::from_str(&contents, config::FileFormat::Toml));
        builder = builder.add_source(Environment::with_prefix("APP").separator("__"));

        let config = builder.build()?.try_deserialize()?;

        Ok(config)
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address")
    }
}
