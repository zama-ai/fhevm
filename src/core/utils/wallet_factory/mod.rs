pub mod aws_wallet;
pub mod dev_wallet;

// Re-export the wallet types for easier access
pub use self::aws_wallet::KmsAwsWallet;
pub use self::dev_wallet::KmsWallet;

use crate::error::{Error, Result};
use alloy_primitives::{hex, ChainId};
use aws_config::{self, BehaviorVersion};
use aws_sdk_kms::Client as KmsClient;
use tracing::{info, warn};

/// Configuration for AWS KMS wallet
#[derive(Debug, Clone)]
pub struct AwsKmsConfig {
    pub key_id: String,
    pub region: Option<String>,
    pub endpoint: Option<String>,
}

/// Wallet factory for creating different types of wallets
pub struct WalletFactory;

impl WalletFactory {
    /// Initialize a wallet based on configuration
    ///
    /// This method tries different wallet initialization methods in the following order:
    /// 1. AWS KMS wallet (if AWS KMS config is provided)
    /// 2. Signing key file wallet (if signing key path is provided)
    /// 3. Private key wallet (if private key is provided)
    /// 4. Mnemonic wallet (fallback)
    pub async fn initialize_wallet(
        aws_kms_config: Option<AwsKmsConfig>,
        signing_key_path: Option<&str>,
        private_key: Option<&str>,
        mnemonic: &str,
        account_index: u32,
        chain_id: Option<ChainId>,
    ) -> Result<KmsWallet> {
        // Try AWS KMS wallet first if config is provided
        if let Some(aws_config) = aws_kms_config {
            if let Ok(wallet) = Self::create_aws_wallet(aws_config, chain_id).await {
                return Ok(wallet);
            }
            // If AWS KMS wallet creation fails, fall back to other methods
            warn!("Failed to initialize AWS KMS wallet, falling back to local wallet");
        }

        // Try signing key file if provided
        if let Some(path) = signing_key_path {
            info!("Using signing key from file: {}", path);
            return Ok(KmsWallet::from_signing_key_file(Some(path), chain_id)?);
        }

        // Try private key if provided
        if let Some(key) = private_key {
            info!("Using private key from configuration");
            return Ok(KmsWallet::from_private_key_str(key, chain_id)?);
        }

        // Fall back to mnemonic with account index
        info!("Using mnemonic with account index: {}", account_index);
        Ok(KmsWallet::from_mnemonic_with_index(
            mnemonic,
            account_index,
            chain_id,
        )?)
    }

    /// Create an AWS KMS wallet
    ///
    /// This method configures the AWS SDK and creates a KmsAwsWallet instance.
    /// If successful, it returns a KmsWallet with a placeholder private key for compatibility.
    async fn create_aws_wallet(
        aws_config: AwsKmsConfig,
        chain_id: Option<ChainId>,
    ) -> Result<KmsWallet> {
        info!("Using AWS KMS signer with key ID: {}", aws_config.key_id);

        // Configure AWS SDK
        let aws_config_builder = aws_config::defaults(BehaviorVersion::latest());

        // Apply region if specified
        let aws_config_builder = if let Some(region) = &aws_config.region {
            info!("Using AWS region: {}", region);
            aws_config_builder.region(aws_sdk_kms::config::Region::new(region.clone()))
        } else {
            aws_config_builder
        };

        // Apply endpoint if specified
        let aws_config_builder = if let Some(endpoint) = &aws_config.endpoint {
            info!("Using AWS endpoint: {}", endpoint);
            aws_config_builder.endpoint_url(endpoint)
        } else {
            aws_config_builder
        };

        // Load AWS config
        let aws_sdk_config = aws_config_builder.load().await;
        let kms_client = KmsClient::new(&aws_sdk_config);

        // Create AWS KMS wallet
        match KmsAwsWallet::new(kms_client, aws_config.key_id, chain_id).await {
            Ok(aws_wallet) => {
                info!(
                    "AWS KMS wallet created successfully with address: {:#x}",
                    aws_wallet.address()
                );

                // Convert to KmsWallet for compatibility
                // This is a temporary solution until we refactor the connector to use a trait
                // for wallet operations
                Ok(KmsWallet::from_private_key_str(
                    &format!("0x{}", hex::encode([1u8; 32])), // Placeholder private key
                    chain_id,
                )?)
            }
            Err(e) => {
                warn!("Failed to initialize AWS KMS wallet: {}", e);
                // Convert the error to a string and map it to our error type
                Err(Error::Config(format!(
                    "AWS KMS wallet initialization failed: {}",
                    e
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::ChainId;

    #[tokio::test]
    async fn test_initialize_wallet_with_mnemonic() {
        let wallet = WalletFactory::initialize_wallet(
            None,
            None,
            None,
            "test test test test test test test test test test test junk",
            0,
            Some(ChainId::from(1u64)),
        )
        .await
        .unwrap();

        assert!(!wallet.address().to_string().is_empty());
        assert_eq!(
            wallet.address().to_string(),
            "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
        );
    }

    #[tokio::test]
    async fn test_initialize_wallet_with_private_key() {
        let wallet = WalletFactory::initialize_wallet(
            None,
            None,
            Some("0x0000000000000000000000000000000000000000000000000000000000000001"),
            "test mnemonic",
            0,
            Some(ChainId::from(1u64)),
        )
        .await
        .unwrap();
        // See that mnemonic is ignored and private key is used
        // as per precedence rules
        assert_eq!(
            wallet.address().to_string(),
            "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf"
        );
    }
}
