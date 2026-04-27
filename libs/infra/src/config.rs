use config::ConfigError;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::num::NonZeroU64;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "engine", rename_all = "lowercase")]
pub enum DatabaseSettings {
    Postgres {
        username: String,
        password: SecretBox<String>,
        hostname: String,
        port: u16,
        database_name: String,
        require_ssl: bool,
        min_connections: u32,
        max_connections: u32,
        connect_timeout_seconds: u32,
        idle_timeout_seconds: u64,
    },
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        match self {
            Self::Postgres {
                username,
                password,
                hostname,
                port,
                database_name,
                ..
            } => format!(
                "{}://{}:{}@{}:{}/{}",
                "postgres",
                username,
                password.expose_secret(),
                hostname,
                port,
                database_name,
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CacheSettings {
    pub local: Option<LocalCacheSettings>,
    pub distributed: Option<DistributedCacheSettings>,
}

#[derive(Debug, Deserialize)]
pub struct LocalCacheSettings {
    pub max_capacity: NonZeroU64,
    pub ttl_seconds: Option<NonZeroU64>,
    pub tti_seconds: Option<NonZeroU64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "engine", rename_all = "lowercase")]
pub enum DistributedCacheSettings {
    Redis {
        hostname: String,
        port: u16,
        password: Option<SecretBox<String>>,
        database_name: Option<u8>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "engine", rename_all = "lowercase")]
pub enum MessagingSettings {
    Nats {
        hostname: String,
        port: u16,
        password: Option<SecretBox<String>>,
        database_name: Option<u16>,
    },
}

/// Load configuration for type `T` from `<path>/base.yaml`, then
/// override with `APP__`-prefixed environment variables.
///
/// # Example
/// ```ignore
/// let cfg = get_configuration::<MyServiceSettings>("config")?;
/// ```
pub fn get_configuration<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let config_dir = base_path.join(path);

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("config.yaml")))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<T>()
}
