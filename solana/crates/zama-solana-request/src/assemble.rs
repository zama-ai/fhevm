//! Building the typed request from the two carriers it travels in.
//!
//! The Gateway's `solanaUserDecryptionRequest` types the fields it budgets and charges
//! ([`SolanaUserDecryptFields`]) and carries everything else as one opaque blob
//! ([`SolanaRequestBlob`]). The HTTP paths carry the same fields in the same roles. Every
//! consumer joins the two here, so no fact is carried twice and no copy can disagree with another.

use crate::host_chain::{handle_chain_id, is_solana_host_chain_id};
use crate::request::{HandleEntry, SolanaUserDecryptRequest, MAX_REQUEST_HANDLES};
use borsh::{BorshDeserialize, BorshSerialize};
use zama_solana_permit::{PermitError, PermitFields, PermitWireFields, Signature, SIGNATURE_LEN};

/// The request fields the Gateway entry types itself: the handles it budgets, the transport key
/// and validity window it records, and the extra data it routes on.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SolanaUserDecryptFields {
    /// Ciphertext handles, in request order.
    pub handles: Vec<[u8; 32]>,
    /// Claimed transport key the shares are encrypted to.
    pub transport_key: Vec<u8>,
    /// Signed validity start, Unix seconds.
    pub start_timestamp: u64,
    /// Signed validity length, seconds.
    pub duration_seconds: u64,
    /// Claimed versioned extra data (KMS context routing).
    pub extra_data: Vec<u8>,
}

/// What the opaque blob carries: the signed permit fields the Gateway does not type, the
/// signature, and one claim per handle, in handle order. Its borsh encoding is the blob's
/// canonical body ([`crate::codec`]), so the field order below is the layout.
#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub struct SolanaRequestBlob {
    /// The requester's Ed25519 public key.
    pub user_address: [u8; 32],
    /// The `program ‖ scope` pairs the permit is restricted to; empty means unrestricted.
    pub allowed_scopes: Vec<[u8; 64]>,
    /// The program id of the host deployment the permit is for.
    pub verifying_program_id: [u8; 32],
    /// Ed25519 signature over the reconstructed envelope.
    pub signature: [u8; SIGNATURE_LEN],
    /// One entry per handle, in handle order.
    pub entries: Vec<SolanaEntryClaims>,
}

/// The unsigned claims for one handle. See [`HandleEntry`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub struct SolanaEntryClaims {
    pub owner_address: [u8; 32],
    pub encrypted_store: [u8; 32],
}

/// Why a Solana request is refused before any chain state is read.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SolanaRequestError {
    /// A request names at least one handle.
    #[error("solana request names no handles")]
    NoHandles,
    /// More handles than one account snapshot can authorize.
    #[error("solana request names {0} handles, expected at most {MAX_REQUEST_HANDLES}")]
    TooManyHandles(usize),
    /// A public decryption names one encrypted store per handle.
    #[error("solana request names {handles} handles but {stores} encrypted stores")]
    StoreCount { handles: usize, stores: usize },
    /// A user decryption names one entry per handle.
    #[error("solana request names {handles} handles but carries {entries} entries")]
    EntryCount { handles: usize, entries: usize },
    /// The permit names one chain, so every handle must be on it.
    #[error("solana request handle {index} is on chain {other}, handle 0 on chain {first}")]
    MixedChains {
        first: u64,
        index: usize,
        other: u64,
    },
    /// The handles name a chain that is not a Solana host chain.
    #[error("solana request handles are on chain {0}, which is not a Solana host chain")]
    NotSolanaChain(u64),
    #[error("permit: {0}")]
    Permit(#[from] PermitError),
}

impl SolanaUserDecryptRequest {
    /// Joins the Gateway fields and the blob, and types the permit.
    ///
    /// The permit's chain id is not carried by either part: every handle embeds its chain, so it
    /// is derived here and the handles must agree. A signer who signed another chain id then fails
    /// signature verification, like any other field it did not sign.
    pub fn assemble(
        fields: SolanaUserDecryptFields,
        blob: SolanaRequestBlob,
    ) -> Result<Self, SolanaRequestError> {
        check_handle_count(fields.handles.len())?;
        if fields.handles.len() != blob.entries.len() {
            return Err(SolanaRequestError::EntryCount {
                handles: fields.handles.len(),
                entries: blob.entries.len(),
            });
        }
        let chain_id = solana_chain_id(&fields.handles)?;
        let permit = PermitFields::decode(&PermitWireFields {
            user_address: blob.user_address.to_vec(),
            transport_key: fields.transport_key,
            allowed_scopes: blob.allowed_scopes.iter().map(|s| s.to_vec()).collect(),
            start_timestamp: fields.start_timestamp,
            duration_seconds: fields.duration_seconds,
            verifying_program_id: blob.verifying_program_id.to_vec(),
            chain_id,
            extra_data: fields.extra_data,
        })?;
        let entries = fields
            .handles
            .into_iter()
            .zip(blob.entries)
            .map(|(handle, claims)| HandleEntry {
                handle,
                owner_address: claims.owner_address,
                encrypted_store: claims.encrypted_store,
            })
            .collect();
        Ok(Self {
            permit,
            signature: Signature::new(blob.signature),
            entries,
        })
    }
}

/// Checks a Solana public decryption: one encrypted store per handle, within the cap, every
/// handle on one Solana chain. Returns that chain.
pub fn public_request_chain_id(
    handles: &[[u8; 32]],
    encrypted_stores: usize,
) -> Result<u64, SolanaRequestError> {
    check_handle_count(handles.len())?;
    if handles.len() != encrypted_stores {
        return Err(SolanaRequestError::StoreCount {
            handles: handles.len(),
            stores: encrypted_stores,
        });
    }
    solana_chain_id(handles)
}

fn check_handle_count(handles: usize) -> Result<(), SolanaRequestError> {
    if handles > MAX_REQUEST_HANDLES {
        return Err(SolanaRequestError::TooManyHandles(handles));
    }
    Ok(())
}

/// The Solana chain every handle names.
fn solana_chain_id(handles: &[[u8; 32]]) -> Result<u64, SolanaRequestError> {
    let (first, rest) = handles.split_first().ok_or(SolanaRequestError::NoHandles)?;
    let first = handle_chain_id(first);
    for (index, handle) in rest.iter().enumerate() {
        let other = handle_chain_id(handle);
        if other != first {
            return Err(SolanaRequestError::MixedChains {
                first,
                index: index + 1,
                other,
            });
        }
    }
    if !is_solana_host_chain_id(first) {
        return Err(SolanaRequestError::NotSolanaChain(first));
    }
    Ok(first)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host_chain::solana_host_chain_id;
    use zama_solana_permit::{
        KMS_ROUTING_EXTRA_DATA_LEN, KMS_ROUTING_VERSION_BYTE, TRANSPORT_KEY_LEN,
    };

    fn handle(chain_id: u64, seed: u8) -> [u8; 32] {
        let mut handle = [seed; 32];
        handle[22..30].copy_from_slice(&chain_id.to_be_bytes());
        handle
    }

    /// A request whose every field is well formed and distinct.
    fn parts(
        handles: Vec<[u8; 32]>,
        entries: usize,
    ) -> (SolanaUserDecryptFields, SolanaRequestBlob) {
        let mut extra_data = vec![7; KMS_ROUTING_EXTRA_DATA_LEN];
        extra_data[0] = KMS_ROUTING_VERSION_BYTE;
        let fields = SolanaUserDecryptFields {
            handles,
            transport_key: vec![2; TRANSPORT_KEY_LEN],
            start_timestamp: 4,
            duration_seconds: 5,
            extra_data,
        };
        let entry = |seed: u8| SolanaEntryClaims {
            owner_address: [seed; 32],
            encrypted_store: [seed.wrapping_add(1); 32],
        };
        let blob = SolanaRequestBlob {
            user_address: [1; 32],
            allowed_scopes: vec![[3; 64]],
            verifying_program_id: [6; 32],
            signature: [9; 64],
            entries: (0..entries).map(|i| entry(i as u8 * 2)).collect(),
        };
        (fields, blob)
    }

    #[test]
    fn every_field_lands_in_its_place_and_the_chain_comes_from_the_handles() {
        let chain = solana_host_chain_id(7);
        let (fields, blob) = parts(vec![handle(chain, 0xa1), handle(chain, 0xa2)], 2);
        let request = SolanaUserDecryptRequest::assemble(fields.clone(), blob.clone()).unwrap();
        let permit = request.permit();
        assert_eq!(permit.user_address().as_bytes(), &blob.user_address);
        assert_eq!(
            &permit.transport_key().as_bytes()[..],
            &fields.transport_key[..]
        );
        assert_eq!(
            permit.allowed_scopes().as_slice()[0].as_bytes(),
            &blob.allowed_scopes[0]
        );
        assert_eq!(permit.start_timestamp(), fields.start_timestamp);
        assert_eq!(permit.duration_seconds(), fields.duration_seconds);
        assert_eq!(
            permit.verifying_program_id().as_bytes(),
            &blob.verifying_program_id
        );
        assert_eq!(permit.chain_id(), chain);
        assert_eq!(permit.extra_data().to_extra_data(), fields.extra_data);
        assert_eq!(request.signature().as_bytes(), &blob.signature);
        for (index, entry) in request.entries().iter().enumerate() {
            assert_eq!(entry.handle, fields.handles[index]);
            assert_eq!(entry.owner_address, blob.entries[index].owner_address);
            assert_eq!(entry.encrypted_store, blob.entries[index].encrypted_store);
        }
    }

    #[test]
    fn refuses_parts_that_do_not_make_one_request() {
        let chain = solana_host_chain_id(7);
        let other = solana_host_chain_id(8);
        let cases = [
            (parts(vec![], 0), SolanaRequestError::NoHandles),
            (
                parts(
                    vec![handle(chain, 1); MAX_REQUEST_HANDLES + 1],
                    MAX_REQUEST_HANDLES + 1,
                ),
                SolanaRequestError::TooManyHandles(MAX_REQUEST_HANDLES + 1),
            ),
            (
                parts(vec![handle(chain, 1)], 2),
                SolanaRequestError::EntryCount {
                    handles: 1,
                    entries: 2,
                },
            ),
            (
                parts(
                    vec![handle(chain, 1), handle(chain, 2), handle(other, 3)],
                    3,
                ),
                SolanaRequestError::MixedChains {
                    first: chain,
                    index: 2,
                    other,
                },
            ),
            (
                parts(vec![handle(31_337, 1)], 1),
                SolanaRequestError::NotSolanaChain(31_337),
            ),
        ];
        for ((fields, blob), expected) in cases {
            assert_eq!(
                SolanaUserDecryptRequest::assemble(fields, blob),
                Err(expected)
            );
        }
    }

    #[test]
    fn a_permit_field_is_decoded_by_the_permit_rules() {
        let chain = solana_host_chain_id(7);
        let (mut fields, blob) = parts(vec![handle(chain, 1)], 1);
        fields.transport_key.pop();
        assert!(matches!(
            SolanaUserDecryptRequest::assemble(fields, blob),
            Err(SolanaRequestError::Permit(_))
        ));
    }

    #[test]
    fn a_public_request_names_one_store_per_handle_on_one_solana_chain() {
        let chain = solana_host_chain_id(7);
        assert_eq!(
            public_request_chain_id(&[handle(chain, 1), handle(chain, 2)], 2),
            Ok(chain)
        );
        let cases = [
            (vec![], 0, SolanaRequestError::NoHandles),
            (
                vec![handle(chain, 1); MAX_REQUEST_HANDLES + 1],
                MAX_REQUEST_HANDLES + 1,
                SolanaRequestError::TooManyHandles(MAX_REQUEST_HANDLES + 1),
            ),
            (
                vec![handle(chain, 1), handle(chain, 2)],
                1,
                SolanaRequestError::StoreCount {
                    handles: 2,
                    stores: 1,
                },
            ),
            (
                vec![handle(chain, 1), handle(chain + 1, 2)],
                2,
                SolanaRequestError::MixedChains {
                    first: chain,
                    index: 1,
                    other: chain + 1,
                },
            ),
            (
                vec![handle(31_337, 1)],
                1,
                SolanaRequestError::NotSolanaChain(31_337),
            ),
        ];
        for (handles, stores, expected) in cases {
            assert_eq!(public_request_chain_id(&handles, stores), Err(expected));
        }
    }
}
