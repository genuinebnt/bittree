use std::sync::Arc;

use crate::infrastructure::{database::bootstrap::bootstrap_db, errors::Result};
use infra::config::DatabaseSettings;

use crate::{
    domain::repository::DynAuthRepository,
    infrastructure::database::postgres::PostgresAuthRepository,
};

pub struct RepoProvider {
    pub auth_repo: DynAuthRepository,
}

pub async fn from_config(cfg: &DatabaseSettings) -> Result<RepoProvider> {
    let pool = bootstrap_db(cfg).await?;
    let repo = PostgresAuthRepository::new(pool);
    let auth_repo: DynAuthRepository = Arc::new(repo) as DynAuthRepository;
    Ok(RepoProvider { auth_repo })
}
