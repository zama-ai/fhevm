use crate::config::{AwsKmsConfig, BlockchainConfig};
use alloy::{
    hex::decode,
    network::{EthereumWallet, IntoWallet},
    primitives::{Address, ChainId},
    signers::{
        Signer,
        aws::AwsSigner,
        k256::ecdsa::SigningKey,
        local::{MnemonicBuilder, PrivateKeySigner, coins_bip39::English},
    },
};
use anyhow::anyhow;
use aws_config::BehaviorVersion;
use aws_sdk_kms::Client as KmsClient;
use tracing::{debug, info};

#[derive(Clone, Debug)]
pub struct Wallet {
    signer: WalletSigner,
}

#[derive(Clone, Debug)]
enum WalletSigner {
    Local(PrivateKeySigner),
    AwsKms(AwsSigner),
}

impl Wallet {
    pub async fn from_config(config: &BlockchainConfig) -> anyhow::Result<Self> {
        if let Some(aws_config) = &config.aws_kms_config {
            debug!("Building wallet using AWS KMS configuration...");
            Self::from_aws_kms(aws_config.clone(), Some(config.gateway_chain_id)).await
        } else if let Some(mnemonic) = &config.mnemonic {
            debug!("Building wallet using mnemonic...");
            Self::from_mnemonic_with_index(
                mnemonic,
                config.mnemonic_index,
                Some(config.gateway_chain_id),
            )
        } else if let Some(private_key) = &config.private_key {
            debug!("Building wallet using private key...");
            Self::from_private_key_str(private_key, Some(config.gateway_chain_id))
        } else {
            Err(anyhow!(
                "Either aws_kms or private_key should be configured"
            ))
        }
        .inspect(|w| info!("Wallet built successfully, with address: {}!", w.address()))
    }

    fn from_mnemonic_with_index(
        phrase: &str,
        account_index: usize,
        chain_id: Option<ChainId>,
    ) -> anyhow::Result<Self> {
        let derivation_path = format!("m/44'/60'/0'/0/{}", account_index);

        let signer = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(&derivation_path)?
            .build()?
            .with_chain_id(chain_id);

        Ok(Self {
            signer: WalletSigner::Local(signer),
        })
    }

    fn from_private_key_str(private_key: &str, chain_id: Option<ChainId>) -> anyhow::Result<Self> {
        let private_key = private_key.trim_start_matches("0x");

        let bytes = decode(private_key).map_err(|e| anyhow!("Invalid hex encoding: {e}"))?;
        if bytes.len() != 32 {
            return Err(anyhow!(
                "Private key must be 32 bytes, got {} bytes",
                bytes.len()
            ));
        }

        let signing_key = SigningKey::from_bytes(bytes.as_slice().into())
            .map_err(|e| anyhow!("Invalid private key: {e}"))?;
        let signer = PrivateKeySigner::from_signing_key(signing_key).with_chain_id(chain_id);

        Ok(Self {
            signer: WalletSigner::Local(signer),
        })
    }

    async fn from_aws_kms(
        aws_kms_config: AwsKmsConfig,
        chain_id: Option<ChainId>,
    ) -> anyhow::Result<Self> {
        info!(
            "Creating wallet from AWS KMS with key ID: {}",
            aws_kms_config.key_id
        );

        let mut config_loader = aws_config::defaults(BehaviorVersion::latest());

        if let Some(region) = aws_kms_config.region {
            debug!("Using AWS region: {}", region);
            config_loader = config_loader.region(aws_config::Region::new(region));
        }

        if let Some(endpoint) = aws_kms_config.endpoint {
            debug!("Using AWS endpoint: {}", endpoint);
            config_loader = config_loader.endpoint_url(endpoint);
        }

        let config = config_loader.load().await;
        let kms_client = KmsClient::new(&config);
        let aws_signer = AwsSigner::new(kms_client, aws_kms_config.key_id, chain_id).await?;

        info!(
            "Created wallet from AWS KMS with address: {}",
            aws_signer.address()
        );
        Ok(Self {
            signer: WalletSigner::AwsKms(aws_signer),
        })
    }

    pub fn address(&self) -> Address {
        match &self.signer {
            WalletSigner::Local(signer) => signer.address(),
            WalletSigner::AwsKms(signer) => signer.address(),
        }
    }
}

impl IntoWallet for Wallet {
    type NetworkWallet = EthereumWallet;

    fn into_wallet(self) -> Self::NetworkWallet {
        match self.signer {
            WalletSigner::Local(wallet) => wallet.into(),
            WalletSigner::AwsKms(wallet) => wallet.into(),
        }
    }
}
