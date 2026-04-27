use infra::errors::ApiError;

use crate::{domain::errors::DomainError, infrastructure::errors::InfraError};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Infra(#[from] InfraError),
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::Domain(err) => match err {
                DomainError::UserNotFound(msg) => ApiError::NotFound(msg),
                DomainError::InvalidCredentials => ApiError::Unauthorized,
                DomainError::TokenExpired(_) => ApiError::Unauthorized,
                DomainError::UserAlreadyExists(msg) => ApiError::Conflict(msg),
                DomainError::InvalidEmail(msg) | DomainError::InvalidUsername(msg) => {
                    ApiError::UnprocessableEntity(msg)
                }
                DomainError::Internal(err) => {
                    tracing::error!(error = ?err, "internal error");
                    ApiError::Internal
                }
            },
            AuthError::Infra(err) => {
                tracing::error!(error = ?err, "internal error");
                ApiError::Internal
            }
        }
    }
}
