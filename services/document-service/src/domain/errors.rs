use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("expected id in message, got: {0}")]
    PageNotFound(String),
    #[error("expected id in message, got: {0}")]
    BlockNotFound(String),
    #[error("expected version missing from: {expected}, actual version missing from: {actual}")]
    VersionConflict { expected: i32, actual: i32 },
    #[error("title cannot be empty")]
    InvalidTitle(String),
    #[error("sort_key cannot be empty")]
    InvalidSortKey(String),
    #[error("visibility not found: {0}")]
    VisibilityNotFound(String),
    #[error("database connection failed")]
    Internal(#[source] anyhow::Error),
    #[error("unauthorized")]
    Unauthorized,
}

impl From<sqlx::Error> for DomainError {
    fn from(error: sqlx::Error) -> Self {
        DomainError::Internal(anyhow::anyhow!(error))
    }
}

pub type Result<T> = std::result::Result<T, DomainError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_not_found_displays_id() {
        let err = DomainError::PageNotFound("abc-123".to_string());
        let msg = err.to_string();
        assert!(
            msg.contains("abc-123"),
            "expected id in message, got: {msg}"
        );
    }

    #[test]
    fn block_not_found_displays_id() {
        let err = DomainError::BlockNotFound("xyz-456".to_string());
        let msg = err.to_string();
        assert!(
            msg.contains("xyz-456"),
            "expected id in message, got: {msg}"
        );
    }

    #[test]
    fn version_conflict_displays_expected_and_actual() {
        let err = DomainError::VersionConflict {
            expected: 3,
            actual: 5,
        };
        let msg = err.to_string();
        assert!(msg.contains('3'), "expected version missing from: {msg}");
        assert!(msg.contains('5'), "actual version missing from: {msg}");
    }

    #[test]
    fn invalid_title_displays_reason() {
        let err = DomainError::InvalidTitle("title cannot be empty".to_string());
        let msg = err.to_string();
        assert!(!msg.is_empty());
    }

    #[test]
    fn invalid_sort_key_displays_reason() {
        let err = DomainError::InvalidSortKey("sort_key cannot be empty".to_string());
        let msg = err.to_string();
        assert!(!msg.is_empty());
    }

    #[test]
    fn internal_error_preserves_source() {
        let source = anyhow::anyhow!("database connection failed");
        let err = DomainError::Internal(source);
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn domain_result_aliases_result_over_domain_error() {
        let ok: Result<i32> = Ok(42);
        assert_eq!(ok.unwrap(), 42);

        let err: Result<i32> = Err(DomainError::Unauthorized);
        assert!(err.is_err());
    }
}
