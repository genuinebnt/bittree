use anyhow::anyhow;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretBox};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::user::User;
use crate::domain::errors::{DomainError, Result};
use crate::domain::repository::AuthRepository;
use crate::domain::types::{DateTimeWithTimezone, Email, UserId, Username};

#[derive(Debug)]
pub struct PostgresAuthRepository {
    pool: PgPool,
}

impl PostgresAuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn create_user(
        &self,
        username: crate::domain::types::Username,
        password: secrecy::SecretBox<String>,
        email: crate::domain::types::Email,
    ) -> Result<()> {
        let conflict = sqlx::query!(
            r#"
            SELECT username, email 
            FROM users 
            WHERE username = $1 OR email = $2 
            LIMIT 1
            "#,
            username.as_ref(),
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(anyhow!(e)))?;

        if let Some(row) = conflict {
            if row.username == username.as_ref() {
                return Err(DomainError::UserAlreadyExists(
                    "Username already taken".into(),
                ));
            } else {
                return Err(DomainError::UserAlreadyExists(
                    "Email already registered".into(),
                ));
            }
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.expose_secret().as_bytes(), &salt)
            .map_err(|e| DomainError::Internal(anyhow!(e)))?
            .to_string();

        sqlx::query!(
            "INSERT INTO users(username, password_hash, email) VALUES ($1, $2, $3)",
            username.as_ref() as &str,
            password_hash,
            email.as_ref() as &str,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(anyhow!(e)))?;

        Ok(())
    }

    async fn find_by_email(&self, email: Email) -> Result<User> {
        let user = sqlx::query_as!(
            UserRow,
            "SELECT * from users where email = $1 LIMIT 1",
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(anyhow!(e)))?
        .ok_or_else(|| DomainError::UserNotFound(email.as_ref().to_string()))?
        .try_into()?;

        Ok(user)
    }

    async fn find_by_id(&self, user_id: UserId) -> Result<User> {
        let user = sqlx::query_as!(
            UserRow,
            "SELECT * FROM users WHERE id = $1 LIMIT 1",
            user_id.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(anyhow!(e)))?
        .ok_or_else(|| DomainError::UserNotFound(user_id.as_ref().to_string()))?
        .try_into()?;

        Ok(user)
    }

    async fn validate_credentials(
        &self,
        username: Username,
        password: SecretBox<String>,
    ) -> Result<bool> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.expose_secret().as_bytes(), &salt)
            .map_err(|e| DomainError::Internal(anyhow!(e)))?
            .to_string();

        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 AND password_hash = $2)",
            username.as_ref() as &str,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(anyhow!(e)))?;

        if !exists.unwrap_or(false) {
            return Err(DomainError::UserNotFound(username.as_ref().to_string()));
        }

        Ok(true)
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    email_verified: bool,
    is_active: bool,
    created_at: DateTimeWithTimezone,
    updated_at: DateTimeWithTimezone,
}

impl TryFrom<UserRow> for User {
    type Error = DomainError;

    fn try_from(row: UserRow) -> std::result::Result<Self, Self::Error> {
        Ok(User {
            id: UserId::new(row.id),
            username: Username::parse(row.username)?,
            email: Email::parse(row.email)?,
            password_hash: row.password_hash,
            email_verified: row.email_verified,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}
