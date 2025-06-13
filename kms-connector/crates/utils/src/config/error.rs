use alloy::signers::aws::AwsSignerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config deserialization failed: {0}")]
    ConfigDeserialization(#[from] config::ConfigError),
    #[error("{0} is not configured")]
    EmptyField(String),
    #[error("{0}")]
    InvalidConfig(String),

    #[error("AWS signer error: {0}")]
    InvalidAwsSigner(Box<AwsSignerError>),
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
}

impl From<AwsSignerError> for Error {
    fn from(value: AwsSignerError) -> Self {
        Self::InvalidAwsSigner(Box::new(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
