use crate::config::BlockchainConfig;
use alloy::{
    hex::decode,
    network::{EthereumWallet, IntoWallet},
    primitives::{Address, ChainId},
    signers::{Signer, aws::AwsSigner, k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use anyhow::anyhow;
use tracing::{debug, info};

#[derive(Clone, Debug)]
pub struct Wallet {
    signer: WalletSigner,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum WalletSigner {
    Local(PrivateKeySigner),
    AwsKms(AwsSigner),
}

impl Wallet {
    pub async fn from_config(config: &BlockchainConfig) -> anyhow::Result<Self> {
        debug!("Building wallet using private key...");
        let wallet =
            Self::from_private_key_str(&config.private_key, Some(config.gateway_chain_id))?;
        info!(
            "Wallet built successfully, with address: {}!",
            wallet.address()
        );
        Ok(wallet)
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
