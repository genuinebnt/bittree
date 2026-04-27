use infra::config::{ApplicationSettings, CacheSettings, DatabaseSettings, MessagingSettings};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthServiceSettings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub cache: CacheSettings,
    pub messaging: MessagingSettings,
}
