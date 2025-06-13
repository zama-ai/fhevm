use crate::error::{Error, Result};
use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use fhevm_gateway_rust_bindings::decryption::Decryption;
use std::sync::Arc;
use tracing::{debug, info};

/// Adapter for decryption operations
#[derive(Clone)]
pub struct DecryptionAdapter<P> {
    decryption_address: Address,
    provider: Arc<P>,
}

impl<P: Provider + Clone> DecryptionAdapter<P> {
    /// Create a new decryption adapter
    pub fn new(decryption_address: Address, provider: Arc<P>) -> Self {
        Self {
            decryption_address,
            provider,
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

        let contract = Decryption::new(self.decryption_address, self.provider.clone());

        let call_builder = contract.publicDecryptionResponse(id, result, signature.into());

        // Estimate gas and add an 80% buffer
        let estimated_gas = call_builder
            .estimate_gas()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        let gas_limit = estimated_gas * 18 / 10; // 80% buffer
        info!(
            ?estimated_gas,
            ?gas_limit,
            "Gas estimated for public decryption response"
        );

        let mut call = call_builder.into_transaction_request();
        call.gas = Some(gas_limit);

        let tx = self
            .provider()
            .send_transaction(call)
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

        let contract = Decryption::new(self.decryption_address, self.provider.clone());

        // Create and send transaction
        let call_builder = contract.userDecryptionResponse(id, result, signature.into());

        // Estimate gas and add an 80% buffer
        let estimated_gas = call_builder
            .estimate_gas()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        let gas_limit = estimated_gas * 18 / 10; // 80% buffer
        info!(
            ?estimated_gas,
            ?gas_limit,
            "Gas estimated for user decryption response"
        );

        let mut call = call_builder.into_transaction_request();
        call.gas = Some(gas_limit);

        let tx = self
            .provider()
            .send_transaction(call)
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
