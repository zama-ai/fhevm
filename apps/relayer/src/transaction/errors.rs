use crate::transaction::sender::TransactionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionServiceError {
    #[error("Transaction failed: {0}")]
    Failed(String),

    #[error("Transaction timeout after {0} seconds")]
    Timeout(u64),

    #[error("Gas estimation failed: {0}")]
    GasEstimation(String),

    #[error("Nonce error: {0}")]
    NonceError(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

impl From<&TransactionError> for TransactionServiceError {
    fn from(err: &TransactionError) -> Self {
        match err {
            TransactionError::InvalidPrivateKey(msg) => {
                Self::Failed(format!("Invalid private key: {msg}"))
            }
            TransactionError::InvalidAddress(msg) => {
                Self::Failed(format!("Invalid address: {msg}"))
            }
            TransactionError::RpcError(msg) => Self::Network(msg.to_string()),
            TransactionError::TransactionFailed(msg) => {
                if msg.contains("nonce too low") {
                    Self::NonceError(msg.to_string())
                } else {
                    Self::Failed(msg.to_string())
                }
            }
            TransactionError::TransactionTimeout(secs) => Self::Timeout(*secs),
            TransactionError::GasEstimationFailed(msg) => Self::GasEstimation(msg.to_string()),
            TransactionError::MonitoringTimeout(secs) => Self::Timeout(*secs), // Transaction may still succeed but monitoring timed out
            TransactionError::ReceiptNotFound(attempts) => {
                Self::Failed(format!("Receipt not found after {attempts} attempts"))
            }
            TransactionError::InsufficientConfirmations { required, actual } => Self::Failed(
                format!("Insufficient confirmations: required {required}, got {actual}"),
            ),
            TransactionError::NetworkError(msg) => Self::Network(msg.to_string()),
            TransactionError::TransportError(e) => Self::Network(e.to_string()),
            TransactionError::InvalidChainId(msg) => Self::Failed(msg.to_string()),
        }
    }
}

impl From<TransactionError> for TransactionServiceError {
    fn from(err: TransactionError) -> Self {
        Self::from(&err)
    }
}
