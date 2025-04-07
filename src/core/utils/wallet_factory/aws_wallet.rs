use alloy_primitives::{Address, ChainId, B256};
use alloy_signer::Signer;
use alloy_signer_aws::AwsSigner;
use aws_sdk_kms::Client;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, info};

#[derive(Debug, Error)]
pub enum AwsWalletError {
    #[error("Signer error: {0}")]
    SignerError(#[from] alloy_signer::Error),
    #[error("AWS signer error: {0}")]
    AwsSignerError(#[from] alloy_signer_aws::AwsSignerError),
    #[error("Failed to initialize AWS KMS client: {0}")]
    AwsClientError(String),
    #[error("Failed to sign with AWS KMS: {0}")]
    SigningError(String),
}

pub type Result<T> = std::result::Result<T, AwsWalletError>;

/// KMS wallet implementation using AWS KMS for signing
///
/// This wallet implementation provides functionality for:
/// - Creating wallets from AWS KMS keys
/// - Signing messages, hashes, and decryption responses using AWS KMS
///
/// It serves as an alternative to the local wallet implementation,
/// allowing for more secure key management using AWS KMS.
#[derive(Clone, Debug)]
pub struct KmsAwsWallet {
    signer: Arc<AwsSigner>,
    chain_id: Option<ChainId>,
}

impl KmsAwsWallet {
    /// Create a new wallet from an AWS KMS key ID
    ///
    /// This method requires an AWS KMS client and a key ID.
    /// The key must be an ECC_SECG_P256K1 key type with SIGN_VERIFY key usage.
    pub async fn new(
        kms_client: Client,
        key_id: String,
        chain_id: Option<ChainId>,
    ) -> Result<Self> {
        debug!("Creating AWS KMS wallet with key ID: {}", key_id);

        let signer = AwsSigner::new(kms_client, key_id, chain_id)
            .await
            .map_err(AwsWalletError::AwsSignerError)?;

        info!("Created AWS KMS wallet with address: {}", signer.address());

        Ok(Self {
            signer: Arc::new(signer),
            chain_id,
        })
    }

    /// Get the wallet's address
    pub fn address(&self) -> Address {
        debug!("Getting AWS KMS wallet address");
        self.signer.address()
    }

    /// Sign a message asynchronously
    pub async fn sign_message_async(&self, message: &[u8]) -> Result<Vec<u8>> {
        debug!("Signing message with AWS KMS");
        let signature = self
            .signer
            .sign_message(message)
            .await
            .map_err(AwsWalletError::SignerError)?;

        // Convert the signature to bytes
        let bytes = signature.as_bytes();
        Ok(bytes.to_vec())
    }

    /// Sign a hash asynchronously
    pub async fn sign_hash_async(&self, hash: &B256) -> Result<Vec<u8>> {
        debug!("Signing hash with AWS KMS");
        let signature = self
            .signer
            .sign_hash(hash)
            .await
            .map_err(AwsWalletError::SignerError)?;

        // Convert the signature to bytes
        let bytes = signature.as_bytes();
        Ok(bytes.to_vec())
    }

    /// Sign a decryption response asynchronously
    ///
    /// This method combines the request ID and result into a single message
    /// and signs it using the AWS KMS key.
    pub async fn sign_decryption_response_async(
        &self,
        id: &[u8],
        result: &[u8],
    ) -> Result<Vec<u8>> {
        debug!("Signing decryption response with AWS KMS");

        // Combine the ID and result into a single message
        let mut message = Vec::with_capacity(id.len() + result.len());
        message.extend_from_slice(id);
        message.extend_from_slice(result);

        self.sign_message_async(&message).await
    }

    /// Get the chain ID
    pub fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }

    /// Set the chain ID
    pub fn set_chain_id(&mut self, chain_id: Option<ChainId>) {
        self.chain_id = chain_id;
        if let Some(chain_id) = chain_id {
            Arc::get_mut(&mut self.signer)
                .expect("Failed to get mutable reference to AWS signer")
                .set_chain_id(Some(chain_id));
        } else {
            Arc::get_mut(&mut self.signer)
                .expect("Failed to get mutable reference to AWS signer")
                .set_chain_id(None);
        }
    }
}
