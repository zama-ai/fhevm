//! Solana decryption requests in their typed form. A user decryption is an sRFC-38 permit, its
//! ed25519 signature, and the handles it is used for; a public decryption is handles, each with
//! the encrypted store its public-decrypt leaf is proven against. Built by the HTTP endpoint and
//! the row reader, authorized by the worker.

use alloy::primitives::{B256, U256};
use solana_pubkey::Pubkey;
use zama_solana_request::{SolanaUserDecryptRequest, public_request_chain_id};

pub use zama_solana_request::{
    MAX_REQUEST_HANDLES, SolanaEntryClaims, SolanaRequestBlob, SolanaRequestError,
    SolanaUserDecryptFields,
};

/// A Solana user decryption and the Gateway decryption id it answers. The signature is not
/// verified here: the worker verifies it on every attempt.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaUserDecryptionRequestV1 {
    pub decryption_id: U256,
    pub request: SolanaUserDecryptRequest,
}

impl SolanaUserDecryptionRequestV1 {
    pub fn new(
        decryption_id: U256,
        fields: SolanaUserDecryptFields,
        blob: SolanaRequestBlob,
    ) -> Result<Self, SolanaRequestError> {
        Ok(Self {
            decryption_id,
            request: SolanaUserDecryptRequest::assemble(fields, blob)?,
        })
    }

    pub fn ct_handles(&self) -> Vec<B256> {
        self.request
            .entries()
            .iter()
            .map(|e| B256::from(e.handle))
            .collect()
    }

    pub fn extra_data(&self) -> Vec<u8> {
        self.request.permit().extra_data().to_extra_data()
    }
}

/// One handle of a Solana public decryption and the encrypted store that holds its history. A
/// store is not derivable from a handle, so the request names it; a wrong one fails the proof and
/// never widens access.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SolanaPublicHandle {
    pub handle: B256,
    pub encrypted_store: Pubkey,
}

/// A Solana public decryption: 1 to [`MAX_REQUEST_HANDLES`] handles of one chain, each with its
/// encrypted store, and the KMS routing `extra_data` the Gateway pinned the context from.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaPublicDecryptionRequest {
    pub decryption_id: U256,
    chain_id: u64,
    handles: Vec<SolanaPublicHandle>,
    extra_data: Vec<u8>,
}

impl SolanaPublicDecryptionRequest {
    /// Pairs each handle with the store at the same position.
    pub fn new(
        decryption_id: U256,
        ct_handles: &[B256],
        encrypted_stores: &[B256],
        extra_data: Vec<u8>,
    ) -> Result<Self, SolanaRequestError> {
        let handles: Vec<[u8; 32]> = ct_handles.iter().map(|h| h.0).collect();
        let chain_id = public_request_chain_id(&handles, encrypted_stores.len())?;
        let handles = ct_handles
            .iter()
            .zip(encrypted_stores)
            .map(|(handle, store)| SolanaPublicHandle {
                handle: *handle,
                encrypted_store: Pubkey::new_from_array(store.0),
            })
            .collect();
        Ok(Self {
            decryption_id,
            chain_id,
            handles,
            extra_data,
        })
    }

    /// The chain every handle embeds.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// In request order; duplicates are kept and each one is proven.
    pub fn handles(&self) -> &[SolanaPublicHandle] {
        &self.handles
    }

    pub fn ct_handles(&self) -> Vec<B256> {
        self.handles.iter().map(|e| e.handle).collect()
    }

    pub fn encrypted_stores(&self) -> Vec<B256> {
        self.handles
            .iter()
            .map(|e| B256::from(e.encrypted_store.to_bytes()))
            .collect()
    }

    pub fn extra_data(&self) -> &[u8] {
        &self.extra_data
    }
}

#[cfg(all(test, feature = "tests"))]
mod tests {
    use super::*;
    use crate::tests::rand::{rand_solana_handle, solana_user_decryption_parts};

    #[test]
    fn a_public_request_pairs_each_handle_with_its_store() {
        let handles = [rand_solana_handle(), rand_solana_handle()];
        let stores = [B256::repeat_byte(3), B256::repeat_byte(4)];
        let request =
            SolanaPublicDecryptionRequest::new(U256::ONE, &handles, &stores, vec![0x00]).unwrap();
        assert_eq!(request.ct_handles(), handles);
        assert_eq!(request.encrypted_stores(), stores);
        assert_eq!(
            request.handles()[1].encrypted_store,
            Pubkey::new_from_array([4; 32])
        );
    }

    #[test]
    fn a_public_request_is_checked_by_the_shared_rules() {
        let handles = [rand_solana_handle(), rand_solana_handle()];
        assert_eq!(
            SolanaPublicDecryptionRequest::new(U256::ONE, &handles, &[B256::ZERO], vec![]),
            Err(SolanaRequestError::StoreCount {
                handles: 2,
                stores: 1
            })
        );
        let at_cap = vec![rand_solana_handle(); MAX_REQUEST_HANDLES];
        assert!(SolanaPublicDecryptionRequest::new(U256::ONE, &at_cap, &at_cap, vec![]).is_ok());
    }

    #[test]
    fn rejects_a_blob_that_names_another_entry_count() {
        let (mut fields, blob) = solana_user_decryption_parts(rand_solana_handle());
        fields.handles.push(fields.handles[0]);
        assert_eq!(
            SolanaUserDecryptionRequestV1::new(U256::ONE, fields, blob),
            Err(SolanaRequestError::EntryCount {
                handles: 2,
                entries: 1
            })
        );
    }
}
