pub mod verifier;
use std::io;

use aws_sdk_s3::{error::SdkError, operation::get_object::GetObjectError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    #[error("S3 error: {0}")]
    SdkError(#[from] SdkError<GetObjectError>),

    #[error("Invalid CRS bytes {0}")]
    InvalidCrsBytes(String),

    #[error("Invalid Input bytes {0}")]
    InvalidInputBytes(String),

    #[error("Invalid Proof: {0}")]
    InvalidProof(i64),
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_verify_proofs() {
        //crate::verifier::verify_proof(&[], &[]).await.unwrap();
    }
}
