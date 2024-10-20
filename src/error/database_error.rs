use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Operation timed out")]
    Timeout,
    #[error("Transaction already in progress")]
    TransactionAlreadyInProgress,
    #[error("No transaction in progress")]
    NoTransactionInProgress,
    #[error("Failed to acquire lock")]
    LockError,
    #[error("Operation failed")]
    OperationFailed,
}
