#![allow(dead_code, unused_imports)] // TODO: remove once IHTTPZ is ready
use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use std::sync::Arc;

use crate::{
    core::utils::wallet::KmsWallet,
    error::{Error, Result},
};

/// Adapter for HTTPZ key management operations
pub struct HTTPZAdapter<P: Provider + Clone> {
    httpz_address: Address,
    provider: Arc<P>,
    wallet: Arc<KmsWallet>,
}

impl<P: Provider + Clone> HTTPZAdapter<P> {
    /// Create a new HTTPZ adapter
    pub fn new(httpz_address: Address, provider: Arc<P>, wallet: KmsWallet) -> Self {
        Self {
            httpz_address,
            provider,
            wallet: Arc::new(wallet),
        }
    }

    // TODO: uncomment when IHTTPZ is ready

    // /// Send a preprocess keygen response
    // pub async fn send_preprocess_keygen_response(&self, pre_key_id: U256) -> Result<()> {
    //     let contract = IHTTPZ::new(self.httpz_address, self.provider.clone());

    //     let call = contract.preprocessKeygenResponse(pre_key_id);
    //     let _ = call
    //         .from(self.wallet.address())
    //         .send()
    //         .await
    //         .map_err(|e| Error::Contract(e.to_string()))?;

    //     Ok(())
    // }

    // /// Send a preprocess KSK generation response
    // pub async fn send_preprocess_kskgen_response(&self, pre_ksk_id: U256) -> Result<()> {
    //     let contract = IHTTPZ::new(self.httpz_address, self.provider.clone());

    //     let call = contract.preprocessKskgenResponse(pre_ksk_id);
    //     let _ = call
    //         .from(self.wallet.address())
    //         .send()
    //         .await
    //         .map_err(|e| Error::Contract(e.to_string()))?;

    //     Ok(())
    // }

    // /// Send a keygen response
    // pub async fn send_keygen_response(&self, keygen_id: U256) -> Result<()> {
    //     let contract = IHTTPZ::new(self.httpz_address, self.provider.clone());

    //     let call = contract.keygenResponse(keygen_id);
    //     let _ = call
    //         .from(self.wallet.address())
    //         .send()
    //         .await
    //         .map_err(|e| Error::Contract(e.to_string()))?;

    //     Ok(())
    // }

    // /// Send a CRS generation response
    // pub async fn send_crsgen_response(&self, crs_id: U256) -> Result<()> {
    //     let contract = IHTTPZ::new(self.httpz_address, self.provider.clone());

    //     let call = contract.crsgenResponse(crs_id);
    //     let _ = call
    //         .from(self.wallet.address())
    //         .send()
    //         .await
    //         .map_err(|e| Error::Contract(e.to_string()))?;

    //     Ok(())
    // }

    // /// Send a KSK generation response
    // pub async fn send_kskgen_response(&self, ksk_id: U256) -> Result<()> {
    //     let contract = IHTTPZ::new(self.httpz_address, self.provider.clone());

    //     let call = contract.kskgenResponse(ksk_id);
    //     let _ = call
    //         .from(self.wallet.address())
    //         .send()
    //         .await
    //         .map_err(|e| Error::Contract(e.to_string()))?;

    //     Ok(())
    // }

    // /// Send an activate key response
    // pub async fn send_activate_key_response(&self, key_id: U256) -> Result<()> {
    //     let contract = IHTTPZ::new(self.httpz_address, self.provider.clone());

    //     let call = contract.activateKeyResponse(key_id);
    //     let _ = call
    //         .from(self.wallet.address())
    //         .send()
    //         .await
    //         .map_err(|e| Error::Contract(e.to_string()))?;

    //     Ok(())
    // }
}
