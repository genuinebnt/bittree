use crate::op::OpError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("crdt: invalid operation: {0}")]
    InvalidOp(#[from] OpError),
}
