//! Solana decryption requests in their typed form. A user decryption is an sRFC-38 permit, its
//! ed25519 signature, and the handles it is used for; a public decryption is handles, each with
//! the encrypted store its public-decrypt leaf is proven against. Built by the Gateway listener,
//! the HTTP endpoint and the row reader, authorized by the worker.

use alloy::primitives::{B256, U256};
use fhevm_gateway_bindings::decryption::Decryption;
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

/// The Gateway types the handles, transport key, window and extra data it budgets and charges;
/// the blob carries the rest.
impl TryFrom<Decryption::SolanaUserDecryptionRequest> for SolanaUserDecryptionRequestV1 {
    type Error = anyhow::Error;

    fn try_from(event: Decryption::SolanaUserDecryptionRequest) -> anyhow::Result<Self> {
        let blob = zama_solana_request::decode_solana_request(&event.solanaRequest)?;
        let fields = SolanaUserDecryptFields {
            handles: event.ctHandles.iter().map(|h| h.0).collect(),
            transport_key: event.publicKey.to_vec(),
            start_timestamp: event.requestValidity.startTimestamp.try_into()?,
            duration_seconds: event.requestValidity.durationSeconds.try_into()?,
            extra_data: event.extraData.to_vec(),
        };
        Ok(Self::new(event.decryptionId, fields, blob)?)
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

impl TryFrom<Decryption::SolanaPublicDecryptionRequest> for SolanaPublicDecryptionRequest {
    type Error = SolanaRequestError;

    fn try_from(event: Decryption::SolanaPublicDecryptionRequest) -> Result<Self, Self::Error> {
        Self::new(
            event.decryptionId,
            &event.ctHandles,
            &event.encryptedStores,
            event.extraData.to_vec(),
        )
    }
}

#[cfg(all(test, feature = "tests"))]
mod tests {
    use super::*;
    use crate::tests::rand::{rand_solana_handle, solana_user_decryption_event};

    #[test]
    fn an_event_types_its_request_from_the_event_fields_and_the_blob() {
        let event = solana_user_decryption_event(U256::from(1), rand_solana_handle());
        let stored = SolanaUserDecryptionRequestV1::try_from(event.clone()).unwrap();
        let permit = stored.request.permit();
        assert_eq!(stored.ct_handles(), event.ctHandles);
        assert_eq!(
            U256::from(permit.start_timestamp()),
            event.requestValidity.startTimestamp
        );
        assert_eq!(
            &permit.transport_key().as_bytes()[..],
            event.publicKey.as_ref()
        );
        assert_eq!(stored.extra_data(), event.extraData.to_vec());
    }

    #[test]
    fn a_public_event_pairs_each_handle_with_its_store() {
        let event = Decryption::SolanaPublicDecryptionRequest {
            decryptionId: U256::from(1),
            ctHandles: vec![rand_solana_handle(), rand_solana_handle()],
            extraData: vec![0x00].into(),
            encryptedStores: vec![B256::repeat_byte(3), B256::repeat_byte(4)],
        };
        let request = SolanaPublicDecryptionRequest::try_from(event.clone()).unwrap();
        assert_eq!(request.ct_handles(), event.ctHandles);
        assert_eq!(request.encrypted_stores(), event.encryptedStores);
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
    fn rejects_an_event_whose_blob_names_another_entry_count() {
        let mut event = solana_user_decryption_event(U256::from(1), rand_solana_handle());
        event.ctHandles.push(event.ctHandles[0]);
        let error = SolanaUserDecryptionRequestV1::try_from(event).unwrap_err();
        assert_eq!(
            error.downcast_ref::<SolanaRequestError>(),
            Some(&SolanaRequestError::EntryCount {
                handles: 2,
                entries: 1
            })
        );
    }
}
