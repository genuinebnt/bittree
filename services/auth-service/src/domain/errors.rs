use crate::domain::types::DateTimeWithTimezone;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User with identified '{0}' already exists")]
    UserAlreadyExists(String),
    #[error("User '{0}' not found")]
    UserNotFound(String),
    #[error("Session expired at '{0}'")]
    TokenExpired(DateTimeWithTimezone),
    #[error("'{0}' is not a valid email address")]
    InvalidEmail(String),
    #[error("'{0}' is not a valid username")]
    InvalidUsername(String),
    #[error("An unexpected internal error occured")]
    Internal(#[source] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, DomainError>;
