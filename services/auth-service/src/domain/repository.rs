use crate::domain::errors::Result;
use crate::domain::{
    entities::user::User,
    types::{Email, TokenId, UserId, Username},
};
use async_trait::async_trait;
use secrecy::SecretBox;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait]
pub trait AuthRepository: Send + Sync + 'static + Debug {
    async fn create_user(
        &self,
        username: Username,
        password: SecretBox<String>,
        email: Email,
    ) -> Result<()>;
    async fn find_by_email(&self, email: Email) -> Result<User>;
    async fn find_by_id(&self, user_id: UserId) -> Result<User>;
}

pub type DynAuthRepository = Arc<dyn AuthRepository>;

#[async_trait]
pub trait SessionRepository: Send + Sync + 'static + Debug {
    async fn store_token(&self, token: TokenId) -> Result<()>;
    async fn revoke_token(&self, token: TokenId) -> Result<()>;
    async fn is_revoked(&self, token: &TokenId) -> Result<bool>;
}

pub type DynSessionRepository = Arc<dyn SessionRepository>;
