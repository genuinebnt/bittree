use std::time::Duration;

use crate::infrastructure::errors::Result;
use infra::config::DatabaseSettings;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn bootstrap_db(cfg: &DatabaseSettings) -> Result<PgPool> {
    match cfg {
        DatabaseSettings::Postgres {
            require_ssl,
            min_connections,
            max_connections,
            connect_timeout_seconds,
            idle_timeout_seconds,
            ..
        } => {
            let url = cfg.connection_string();
            let pool = PgPoolOptions::new()
                .max_connections(*max_connections)
                .min_connections(*min_connections)
                .idle_timeout(Duration::from_secs(*idle_timeout_seconds))
                .connect_lazy(&url)?;

            Ok(pool)
        }
    }
}
