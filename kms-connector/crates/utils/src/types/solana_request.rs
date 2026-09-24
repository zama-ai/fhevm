//! Solana user decryption: an sRFC-38 permit, its ed25519 signature, and the handles it is used
//! for. Built by the Gateway listener, the HTTP endpoint and the row reader, authorized by the
//! worker.

use crate::types::handle::extract_chain_id_from_handle;
use alloy::primitives::{B256, U256};
use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest_4;
use zama_solana_permit::{PermitFields, SIGNATURE_LEN, Signature};

pub use zama_solana_permit::PermitWireFields;
pub use zama_solana_request::{
    MAX_REQUEST_HANDLES, SolanaHandleEntryWire, SolanaUserDecryptRequestWire,
};

/// One handle and the unsigned claims that authorize it: the key whose allow leaf covers the
/// handle (the signer, or a delegator), and the encrypted store holding that leaf.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SolanaHandleEntry {
    pub handle: [u8; 32],
    pub owner_address: [u8; 32],
    pub encrypted_store: [u8; 32],
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
    pub fn new(
        decryption_id: U256,
        wire: &SolanaUserDecryptRequestWire,
    ) -> Result<Self, RequestFormError> {
        let permit = PermitFields::decode(&wire.permit)?;
        let signature: [u8; SIGNATURE_LEN] = wire
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| RequestFormError::SignatureWidth(wire.signature.len()))?;
        if !(1..=MAX_REQUEST_HANDLES).contains(&wire.handles.len()) {
            return Err(RequestFormError::HandleCount(wire.handles.len()));
        }
        let handles = wire
            .handles
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let field = |bytes: &[u8]| {
                    <[u8; 32]>::try_from(bytes).map_err(|_| RequestFormError::EntryWidth(index))
                };
                Ok(SolanaHandleEntry {
                    handle: field(&entry.handle)?,
                    owner_address: field(&entry.owner_address)?,
                    encrypted_store: field(&entry.encrypted_store)?,
                })
            })
            .collect::<Result<Vec<_>, RequestFormError>>()?;
        for entry in &handles {
            let chain_id = extract_chain_id_from_handle(&B256::from(entry.handle))
                .map_err(|e| RequestFormError::Handle(e.to_string()))?;
            if chain_id != permit.chain_id() {
                return Err(RequestFormError::ChainId {
                    permit: permit.chain_id(),
                    handle: chain_id,
                });
            }
        }
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
        self.handles.iter().map(|e| B256::from(e.handle)).collect()
    }

    pub fn extra_data(&self) -> Vec<u8> {
        self.permit.extra_data().to_extra_data()
    }
}

/// The Gateway budgets and charges the event's cleartext handles, transport key, window and
/// extra data, so those are the values stored. The worker verifies the signature over them.
impl TryFrom<UserDecryptionRequest_4> for SolanaUserDecryptionRequestV1 {
    type Error = anyhow::Error;

    fn try_from(event: UserDecryptionRequest_4) -> anyhow::Result<Self> {
        let request = zama_solana_request::decode_solana_request(&event.solanaRequest)?;
        anyhow::ensure!(
            request.handles.len() == event.ctHandles.len(),
            "event names {} handles, its Solana request {}",
            event.ctHandles.len(),
            request.handles.len()
        );
        let wire = SolanaUserDecryptRequestWire {
            permit: PermitWireFields {
                transport_key: event.publicKey.to_vec(),
                start_timestamp: event.requestValidity.startTimestamp.try_into()?,
                duration_seconds: event.requestValidity.durationSeconds.try_into()?,
                extra_data: event.extraData.to_vec(),
                ..request.permit
            },
            signature: request.signature,
            handles: event
                .ctHandles
                .iter()
                .zip(request.handles)
                .map(|(handle, entry)| SolanaHandleEntryWire {
                    handle: handle.to_vec(),
                    ..entry
                })
                .collect(),
        };
        Ok(Self::new(event.decryptionId, &wire)?)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum RequestFormError {
    #[error("permit: {0}")]
    Permit(#[from] zama_solana_permit::PermitError),
    #[error("signature is {0} bytes, expected {SIGNATURE_LEN}")]
    SignatureWidth(usize),
    #[error("request names {0} handles, expected 1 to {MAX_REQUEST_HANDLES}")]
    HandleCount(usize),
    #[error("handle entry {0} has a field that is not 32 bytes")]
    EntryWidth(usize),
    #[error("invalid handle: {0}")]
    Handle(String),
    #[error("permit chain id {permit} does not match handle chain id {handle}")]
    ChainId { permit: u64, handle: u64 },
}

#[cfg(all(test, feature = "tests"))]
mod tests {
    use super::*;
    use crate::tests::rand::{solana_user_decryption_event, solana_user_decryption_wire};
    use zama_solana_request::{decode_solana_request, encode_solana_request};

    #[test]
    fn gateway_cleartext_replaces_the_request_copies() {
        let mut event = solana_user_decryption_event(U256::from(1), B256::ZERO);
        let mut request = decode_solana_request(&event.solanaRequest).unwrap();
        request.permit.start_timestamp -= 1;
        request.permit.transport_key[0] ^= 1;
        request.handles[0].handle = vec![0xff; 32];
        event.solanaRequest = encode_solana_request(&request).unwrap().into();

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
    fn rejects_an_event_whose_request_names_another_handle_count() {
        let mut event = solana_user_decryption_event(U256::from(1), B256::ZERO);
        event.ctHandles.push(event.ctHandles[0]);
        assert!(SolanaUserDecryptionRequestV1::try_from(event).is_err());
    }

    /// An empty list authorizes nothing, and a list past the cap cannot be read in one
    /// `getMultipleAccounts` snapshot.
    #[test]
    fn rejects_a_handle_count_outside_one_snapshot() {
        let mut wire = solana_user_decryption_wire(B256::ZERO);
        let entry = wire.handles[0].clone();
        for count in [0, MAX_REQUEST_HANDLES + 1] {
            wire.handles = vec![entry.clone(); count];
            assert_eq!(
                SolanaUserDecryptionRequestV1::new(U256::ONE, &wire),
                Err(RequestFormError::HandleCount(count))
            );
        }
    }
    /// The permit names the one chain the request may decrypt on, as `contractsChainId` does for
    /// an EIP-712 request.
    #[test]
    fn rejects_a_handle_of_another_chain_than_the_permit() {
        let mut wire = solana_user_decryption_wire(B256::ZERO);
        let handle_chain = wire.permit.chain_id;
        wire.permit.chain_id += 1;
        assert_eq!(
            SolanaUserDecryptionRequestV1::new(U256::ONE, &wire),
            Err(RequestFormError::ChainId {
                permit: handle_chain + 1,
                handle: handle_chain,
            })
        );
    }
}
