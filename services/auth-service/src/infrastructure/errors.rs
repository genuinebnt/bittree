#[derive(Debug, thiserror::Error)]
pub enum InfraError {
    #[error("configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, InfraError>;
