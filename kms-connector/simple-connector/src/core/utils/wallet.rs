use alloy::hex::decode;
use alloy::network::{EthereumWallet, IntoWallet};
use alloy::primitives::{Address, ChainId};
use alloy::signers::Signer;
use alloy::signers::local::PrivateKeySigner;
use thiserror::Error;
use tracing::{debug, info};

// Import AWS KMS signer
use alloy::signers::aws::AwsSigner;
use aws_config::BehaviorVersion;
use aws_sdk_kms::Client as KmsClient;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    #[error("AWS KMS error: {0}")]
    AwsKmsError(#[from] Box<alloy::signers::aws::AwsSignerError>),
}

pub type Result<T> = std::result::Result<T, WalletError>;

/// KMS wallet used by `alloy::Provider` for signing decryption responses.
///
/// This wallet implementation provides functionality for:
/// - Creating wallets from private key strings
/// - Creating wallets from AWS KMS keys
#[derive(Clone, Debug)]
pub struct KmsWallet {
    /// The signer implementation - either local or AWS KMS
    signer: WalletSigner,
}

/// Internal enum to hold either a local or AWS KMS signer
#[derive(Clone, Debug)]
enum WalletSigner {
    /// Local signer using a private key
    Local(PrivateKeySigner),
    /// AWS KMS signer
    AwsKms(AwsSigner),
}

impl KmsWallet {
    /// Create a new wallet from a private key string
    ///
    /// The private key string should be a hexadecimal string with or without '0x' prefix.
    /// This method is particularly useful for testing or when the private key is stored
    /// as a string in a secure environment variable or configuration.
    pub fn from_private_key_str(private_key: &str, chain_id: Option<ChainId>) -> Result<Self> {
        debug!("Creating wallet from private key string");

        // Remove 0x prefix if present
        let private_key = private_key.trim_start_matches("0x");

        // Convert hex string to bytes
        let bytes = decode(private_key)
            .map_err(|e| WalletError::InvalidPrivateKey(format!("Invalid hex encoding: {e}")))?;

        // Ensure the key is the correct length
        if bytes.len() != 32 {
            return Err(WalletError::InvalidPrivateKey(format!(
                "Private key must be 32 bytes, got {} bytes",
                bytes.len()
            )));
        }

        // Create a signing key from the bytes
        let signing_key =
            alloy::signers::k256::ecdsa::SigningKey::from_bytes(bytes.as_slice().into())
                .map_err(|e| WalletError::InvalidPrivateKey(format!("Invalid private key: {e}")))?;

        // Create signer from the signing key
        let signer = PrivateKeySigner::from_signing_key(signing_key).with_chain_id(chain_id);

        info!("Created wallet from private key string");
        Ok(Self {
            signer: WalletSigner::Local(signer),
        })
    }

    /// Create a new wallet from AWS KMS configuration
    pub async fn from_aws_kms(
        key_id: String,
        region: Option<String>,
        endpoint: Option<String>,
        chain_id: Option<ChainId>,
    ) -> Result<Self> {
        info!("Creating wallet from AWS KMS with key ID: {}", key_id);

        // Create AWS config builder
        let mut config_loader = aws_config::defaults(BehaviorVersion::latest());

        // Add region if specified
        if let Some(region) = region {
            debug!("Using AWS region: {}", region);
            config_loader = config_loader.region(aws_config::Region::new(region));
        }

        // Add endpoint if specified
        if let Some(endpoint) = endpoint {
            debug!("Using AWS endpoint: {}", endpoint);
            config_loader = config_loader.endpoint_url(endpoint);
        }

        // Load AWS config
        let config = config_loader.load().await;

        // Create KMS client
        let kms_client = KmsClient::new(&config);

        // Create AWS KMS signer
        let aws_signer = AwsSigner::new(kms_client, key_id, chain_id)
            .await
            .map_err(Box::new)?;

        info!(
            "Created wallet from AWS KMS with address: {}",
            aws_signer.address()
        );
        Ok(Self {
            signer: WalletSigner::AwsKms(aws_signer),
        })
    }

    /// Get the wallet's address
    pub fn address(&self) -> Address {
        debug!("Getting wallet address");
        match &self.signer {
            WalletSigner::Local(signer) => signer.address(),
            WalletSigner::AwsKms(signer) => signer.address(),
        }
    }
}

impl IntoWallet for KmsWallet {
    type NetworkWallet = EthereumWallet;

    fn into_wallet(self) -> Self::NetworkWallet {
        match self.signer {
            WalletSigner::Local(wallet) => wallet.into(),
            WalletSigner::AwsKms(wallet) => wallet.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CHAIN_ID: u64 = 1337;

    #[tokio::test]
    async fn test_wallet_from_private_key_str() {
        // Test private key (this is a test key, never use in production)
        let private_key = "8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f";

        // Create wallet from private key string
        let wallet = KmsWallet::from_private_key_str(private_key, Some(TEST_CHAIN_ID)).unwrap();

        // Expected address for this private key
        let expected_address =
            Address::parse_checksummed("0x63FaC9201494f0bd17B9892B9fae4d52fe3BD377", None).unwrap();

        // Verify the address matches
        assert_eq!(wallet.address(), expected_address);

        // Test with 0x prefix
        let wallet_with_prefix =
            KmsWallet::from_private_key_str(&format!("0x{private_key}"), Some(TEST_CHAIN_ID))
                .unwrap();
        assert_eq!(wallet_with_prefix.address(), expected_address);
    }

    #[test]
    fn test_wallet_from_private_key_str_invalid() {
        // Test with invalid hex string
        let result = KmsWallet::from_private_key_str("not a hex string", Some(TEST_CHAIN_ID));
        assert!(result.is_err());

        // Test with wrong length
        let result = KmsWallet::from_private_key_str("deadbeef", Some(TEST_CHAIN_ID));
        assert!(result.is_err());
    }

    impl KmsWallet {
        pub fn random(chain_id: Option<ChainId>) -> Result<Self> {
            let signer = PrivateKeySigner::random().with_chain_id(chain_id);
            info!("Created random wallet");
            Ok(Self {
                signer: WalletSigner::Local(signer),
            })
        }
    }
}
