use crate::{core::utils::wallet::KmsWallet, gwl2_contracts::IDecryptionManager};
use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use std::sync::Arc;
use tracing::{debug, info};

use crate::error::{Error, Result};

/// Adapter for decryption operations
#[derive(Clone)]
pub struct DecryptionAdapter<P: Provider + Clone> {
    decryption_address: Address,
    provider: Arc<P>,
    wallet: Arc<KmsWallet>,
}

impl<P: Provider + Clone> DecryptionAdapter<P> {
    /// Create a new decryption adapter
    pub fn new(decryption_address: Address, provider: Arc<P>, wallet: KmsWallet) -> Self {
        Self {
            decryption_address,
            provider,
            wallet: Arc::new(wallet),
        }
    }

    /// Get the provider
    pub fn provider(&self) -> &Arc<P> {
        &self.provider
    }

    /// Send a public decryption response
    pub async fn send_public_decryption_response(
        &self,
        id: U256,
        result: Bytes,
        signature: Vec<u8>,
    ) -> Result<()> {
        if signature.len() != 65 {
            return Err(Error::Contract(format!(
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            )));
        }

        info!(
            decryption_id = ?id,
            signature = ?signature,
            "Using Core's EIP-712 signature for public decryption"
        );

        debug!(
            decryption_id = ?id,
            result_len = result.len(),
            signature = ?signature,
            "Sending public decryption response"
        );

        let contract = IDecryptionManager::new(self.decryption_address, self.provider.clone());

        // Create and send transaction
        let call = contract.publicDecryptionResponse(id, result, signature.into());
        let tx = call
            .from(self.wallet.address())
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(decryption_id = ?id, "🎯 Public Decryption response sent with tx receipt: {:?}", receipt);
        Ok(())
    }

    /// Send a user decryption response
    pub async fn send_user_decryption_response(
        &self,
        id: U256,
        result: Bytes,
        signature: Vec<u8>,
    ) -> Result<()> {
        if signature.len() != 65 {
            return Err(Error::Contract(format!(
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            )));
        }

        info!(
            decryption_id = ?id,
            signature = ?signature,
            "Using Core's EIP-712 signature for user decryption"
        );

        debug!(
            decryption_id = ?id,
            result_len = result.len(),
            signature = ?signature,
            "Sending user decryption response"
        );

        let contract = IDecryptionManager::new(self.decryption_address, self.provider.clone());

        // Create and send transaction
        let call = contract.userDecryptionResponse(id, result, signature.into());
        let tx = call
            .from(self.wallet.address())
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(decryption_id = ?id, "🎯 User Decryption response sent with tx receipt: {:?}", receipt);
        Ok(())
    }
}
