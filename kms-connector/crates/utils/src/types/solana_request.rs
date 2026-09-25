//! Solana decryption requests in their typed form. A user decryption is an sRFC-38 permit, its
//! ed25519 signature, and the handles it is used for; a public decryption is handles, each with
//! the encrypted store its public-decrypt leaf is proven against. Built by the Gateway listener,
//! the HTTP endpoint and the row reader, authorized by the worker.

use crate::types::handle::extract_chain_id_from_handle;
use alloy::primitives::{B256, U256};
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest_2, UserDecryptionRequest_4,
};
use solana_pubkey::Pubkey;
use zama_solana_permit::{PermitFields, SIGNATURE_LEN, Signature};
use zama_solana_request::assemble_solana_request;

pub use zama_solana_request::{
    MAX_REQUEST_HANDLES, SolanaEntryClaims, SolanaGatewayFields, SolanaRequestBlob,
};

/// One handle and the unsigned claims that authorize it: the key whose allow leaf covers the
/// handle (the signer, or a delegator), and the encrypted store holding that leaf.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SolanaHandleEntry {
    pub handle: B256,
    pub owner_address: Pubkey,
    pub encrypted_store: Pubkey,
}

/// A Solana user decryption request whose fields have their typed form: a decoded permit, a
/// 64-byte signature, and 1 to [`MAX_REQUEST_HANDLES`] handles of the permit's chain.
///
/// The signature is not verified here: the worker verifies it on every attempt.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaUserDecryptionRequestV1 {
    pub decryption_id: U256,
    permit: PermitFields,
    signature: Signature,
    handles: Vec<SolanaHandleEntry>,
}

impl SolanaUserDecryptionRequestV1 {
    /// Joins the Gateway-typed fields and the blob, then types every field. The permit's chain id
    /// is the one all handles embed.
    pub fn new(
        decryption_id: U256,
        gateway: SolanaGatewayFields,
        blob: SolanaRequestBlob,
    ) -> Result<Self, RequestFormError> {
        let wire = assemble_solana_request(gateway, blob)?;
        let permit = PermitFields::decode(&wire.permit)?;
        let signature: [u8; SIGNATURE_LEN] = wire
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| RequestFormError::SignatureWidth(wire.signature.len()))?;
        let handles = wire
            .handles
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let field = |bytes: &[u8]| {
                    <[u8; 32]>::try_from(bytes).map_err(|_| RequestFormError::EntryWidth(index))
                };
                Ok(SolanaHandleEntry {
                    handle: B256::from(field(&entry.handle)?),
                    owner_address: Pubkey::new_from_array(field(&entry.owner_address)?),
                    encrypted_store: Pubkey::new_from_array(field(&entry.encrypted_store)?),
                })
            })
            .collect::<Result<Vec<_>, RequestFormError>>()?;
        Ok(Self {
            decryption_id,
            permit,
            signature: Signature::new(signature),
            handles,
        })
    }

    pub fn permit(&self) -> &PermitFields {
        &self.permit
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// In request order; duplicates are kept and each one is authorized.
    pub fn handles(&self) -> &[SolanaHandleEntry] {
        &self.handles
    }

    pub fn ct_handles(&self) -> Vec<B256> {
        self.handles.iter().map(|e| e.handle).collect()
    }

    pub fn extra_data(&self) -> Vec<u8> {
        self.permit.extra_data().to_extra_data()
    }
}

/// The Gateway types the handles, transport key, window and extra data it budgets and charges;
/// the blob carries the rest.
impl TryFrom<UserDecryptionRequest_4> for SolanaUserDecryptionRequestV1 {
    type Error = anyhow::Error;

    fn try_from(event: UserDecryptionRequest_4) -> anyhow::Result<Self> {
        let blob = zama_solana_request::decode_solana_request(&event.solanaRequest)?;
        let gateway = SolanaGatewayFields {
            handles: event.ctHandles.iter().map(|h| h.to_vec()).collect(),
            transport_key: event.publicKey.to_vec(),
            start_timestamp: event.requestValidity.startTimestamp.try_into()?,
            duration_seconds: event.requestValidity.durationSeconds.try_into()?,
            extra_data: event.extraData.to_vec(),
        };
        Ok(Self::new(event.decryptionId, gateway, blob)?)
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
    ) -> Result<Self, PublicRequestFormError> {
        if ct_handles.len() != encrypted_stores.len() {
            return Err(PublicRequestFormError::StoreCount {
                handles: ct_handles.len(),
                stores: encrypted_stores.len(),
            });
        }
        if ct_handles.is_empty() || ct_handles.len() > MAX_REQUEST_HANDLES {
            return Err(PublicRequestFormError::HandleCount(ct_handles.len()));
        }
        let chain_of = |handle: &B256| {
            extract_chain_id_from_handle(handle).expect("a 32-byte handle embeds a chain id")
        };
        let chain_id = chain_of(&ct_handles[0]);
        for handle in ct_handles {
            let other = chain_of(handle);
            if other != chain_id {
                return Err(PublicRequestFormError::MixedChains {
                    first: chain_id,
                    other,
                });
            }
        }
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

impl TryFrom<PublicDecryptionRequest_2> for SolanaPublicDecryptionRequest {
    type Error = PublicRequestFormError;

    fn try_from(event: PublicDecryptionRequest_2) -> Result<Self, Self::Error> {
        Self::new(
            event.decryptionId,
            &event.ctHandles,
            &event.encryptedStores,
            event.extraData.to_vec(),
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum PublicRequestFormError {
    #[error("{handles} handles name {stores} encrypted stores")]
    StoreCount { handles: usize, stores: usize },
    #[error("{0} handles, expected 1 to {MAX_REQUEST_HANDLES}")]
    HandleCount(usize),
    #[error("handles of chains {first} and {other} in one request")]
    MixedChains { first: u64, other: u64 },
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum RequestFormError {
    #[error(transparent)]
    Assembly(#[from] zama_solana_request::SolanaRequestAssemblyError),
    #[error("permit: {0}")]
    Permit(#[from] zama_solana_permit::PermitError),
    #[error("signature is {0} bytes, expected {SIGNATURE_LEN}")]
    SignatureWidth(usize),
    #[error("handle entry {0} has a field that is not 32 bytes")]
    EntryWidth(usize),
}

#[cfg(all(test, feature = "tests"))]
mod tests {
    use super::*;
    use crate::tests::rand::solana_user_decryption_event;
    use zama_solana_request::SolanaRequestAssemblyError;

    #[test]
    fn an_event_types_its_request_from_the_event_fields_and_the_blob() {
        let event = solana_user_decryption_event(U256::from(1), B256::ZERO);
        let stored = SolanaUserDecryptionRequestV1::try_from(event.clone()).unwrap();
        assert_eq!(stored.ct_handles(), event.ctHandles);
        assert_eq!(
            U256::from(stored.permit().start_timestamp()),
            event.requestValidity.startTimestamp
        );
        assert_eq!(
            &stored.permit().transport_key().as_bytes()[..],
            event.publicKey.as_ref()
        );
        assert_eq!(stored.extra_data(), event.extraData.to_vec());
    }

    #[test]
    fn a_public_event_pairs_each_handle_with_its_store() {
        let event = PublicDecryptionRequest_2 {
            decryptionId: U256::from(1),
            ctHandles: vec![B256::with_last_byte(1), B256::with_last_byte(2)],
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
    fn a_public_request_names_one_store_per_handle() {
        let handles = [B256::with_last_byte(1), B256::with_last_byte(2)];
        for stores in [&[B256::ZERO][..], &[B256::ZERO; 3][..]] {
            assert_eq!(
                SolanaPublicDecryptionRequest::new(U256::ONE, &handles, stores, vec![]),
                Err(PublicRequestFormError::StoreCount {
                    handles: 2,
                    stores: stores.len()
                })
            );
        }
    }

    #[test]
    fn a_public_request_names_handles_of_one_chain() {
        let mut other_chain = B256::ZERO;
        other_chain[29] = 1;
        let handles = [B256::ZERO, other_chain];
        assert_eq!(
            SolanaPublicDecryptionRequest::new(U256::ONE, &handles, &handles, vec![]),
            Err(PublicRequestFormError::MixedChains { first: 0, other: 1 })
        );
    }

    #[test]
    fn a_public_request_carries_one_to_the_handle_cap() {
        for count in [0, MAX_REQUEST_HANDLES + 1] {
            let handles = vec![B256::ZERO; count];
            assert_eq!(
                SolanaPublicDecryptionRequest::new(U256::ONE, &handles, &handles, vec![]),
                Err(PublicRequestFormError::HandleCount(count))
            );
        }
        let at_cap = vec![B256::ZERO; MAX_REQUEST_HANDLES];
        assert!(SolanaPublicDecryptionRequest::new(U256::ONE, &at_cap, &at_cap, vec![]).is_ok());
    }

    #[test]
    fn rejects_an_event_whose_blob_names_another_entry_count() {
        let mut event = solana_user_decryption_event(U256::from(1), B256::ZERO);
        event.ctHandles.push(event.ctHandles[0]);
        let error = SolanaUserDecryptionRequestV1::try_from(event).unwrap_err();
        assert_eq!(
            error.downcast_ref::<RequestFormError>(),
            Some(&RequestFormError::Assembly(
                SolanaRequestAssemblyError::EntryCount {
                    handles: 2,
                    entries: 1
                }
            ))
        );
    }
}
