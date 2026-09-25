//! The one way to build a full request from its two carriers.
//!
//! A Solana user-decryption request travels in two parts. The Gateway's host-generic entry types
//! the fields it budgets and charges ([`SolanaUserDecryptFields`]) and carries everything else as one
//! opaque blob ([`SolanaRequestBlob`]). Every consumer that needs the full request — the
//! connector's Gateway listener, HTTP endpoint and row reader, and the relayer — joins the two
//! here, so no fact is carried twice and no copy can disagree with another.

use crate::wire::{SolanaHandleEntryWire, SolanaUserDecryptRequestWire, MAX_REQUEST_HANDLES};
use borsh::{BorshDeserialize, BorshSerialize};
use zama_solana_permit::PermitWireFields;

/// The request fields the Gateway entry types itself: the handles it budgets, the transport key
/// and validity window it records, and the extra data it routes on. The HTTP path carries the
/// same fields in the same roles.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SolanaUserDecryptFields {
    /// Claimed 32-byte ciphertext handles, in request order.
    pub handles: Vec<Vec<u8>>,
    /// The transport key the shares are encrypted to.
    pub transport_key: Vec<u8>,
    /// Signed validity start, Unix seconds.
    pub start_timestamp: u64,
    /// Signed validity length, seconds.
    pub duration_seconds: u64,
    /// The versioned extra data (KMS context routing).
    pub extra_data: Vec<u8>,
}

/// What the opaque blob carries: the signed permit fields the Gateway does not type, the
/// signature, and one claim per handle, in handle order. Its borsh encoding is the blob's
/// canonical body ([`crate::codec`]), so the field order below is the layout.
#[derive(Clone, PartialEq, Eq, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct SolanaRequestBlob {
    /// Claimed 32-byte Ed25519 public key of the requester.
    pub user_address: Vec<u8>,
    /// The `(program, scope)` pairs the permit is restricted to; empty means unrestricted.
    pub allowed_scopes: Vec<Vec<u8>>,
    /// Claimed 32-byte program id of the host deployment the permit is for.
    pub verifying_program_id: Vec<u8>,
    /// Claimed Ed25519 signature over the reconstructed envelope.
    pub signature: Vec<u8>,
    /// One entry per handle, in handle order.
    pub entries: Vec<SolanaEntryClaims>,
}

/// The unsigned claims for one handle.
#[derive(Clone, PartialEq, Eq, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct SolanaEntryClaims {
    /// See [`SolanaHandleEntryWire::owner_address`].
    pub owner_address: Vec<u8>,
    /// See [`SolanaHandleEntryWire::encrypted_store`].
    pub encrypted_store: Vec<u8>,
}

/// Why the two carriers do not make one request.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SolanaRequestAssemblyError {
    /// A request authorizes at least one handle.
    #[error("solana request names no handles")]
    NoHandles,
    /// More handles than one account snapshot can authorize.
    #[error("solana request names {0} handles, expected at most {MAX_REQUEST_HANDLES}")]
    TooManyHandles(usize),
    /// A public decryption names one encrypted store per handle.
    #[error("solana request names {handles} handles but {stores} encrypted stores")]
    StoreCount {
        /// Handles in the request.
        handles: usize,
        /// Encrypted stores in the request.
        stores: usize,
    },
    /// Each handle needs exactly one entry.
    #[error("solana request names {handles} handles but carries {entries} entries")]
    EntryCount {
        /// Handles typed by the Gateway fields.
        handles: usize,
        /// Entries in the blob.
        entries: usize,
    },
    /// A handle is not 32 bytes, so it names no chain.
    #[error("solana request handle {index} is {len} bytes, expected 32")]
    HandleWidth {
        /// Position in request order.
        index: usize,
        /// Received width.
        len: usize,
    },
    /// The permit names one chain, so every handle must be on it.
    #[error("solana request handle {index} is on chain {other}, handle 0 on chain {first}")]
    MixedChains {
        /// Chain of the first handle.
        first: u64,
        /// Position of the first disagreeing handle.
        index: usize,
        /// Its chain.
        other: u64,
    },
}

/// Joins the Gateway fields and the blob into the full request.
///
/// The permit's chain id is not carried by either part: every handle embeds its chain in bytes
/// 22..30, so it is derived here and the handles must agree. A signer who signed another chain
/// id then fails signature verification, like any other field it did not sign.
pub fn assemble_solana_request(
    gateway: SolanaUserDecryptFields,
    blob: SolanaRequestBlob,
) -> Result<SolanaUserDecryptRequestWire, SolanaRequestAssemblyError> {
    let SolanaUserDecryptFields {
        handles,
        transport_key,
        start_timestamp,
        duration_seconds,
        extra_data,
    } = gateway;
    let SolanaRequestBlob {
        user_address,
        allowed_scopes,
        verifying_program_id,
        signature,
        entries,
    } = blob;

    check_handle_count(handles.len())?;
    if handles.len() != entries.len() {
        return Err(SolanaRequestAssemblyError::EntryCount {
            handles: handles.len(),
            entries: entries.len(),
        });
    }
    let chain_id = common_chain_id(&handles)?;

    Ok(SolanaUserDecryptRequestWire {
        permit: PermitWireFields {
            user_address,
            transport_key,
            allowed_scopes,
            start_timestamp,
            duration_seconds,
            verifying_program_id,
            chain_id,
            extra_data,
        },
        signature,
        handles: handles
            .into_iter()
            .zip(entries)
            .map(|(handle, entry)| SolanaHandleEntryWire {
                handle,
                owner_address: entry.owner_address,
                encrypted_store: entry.encrypted_store,
            })
            .collect(),
    })
}

/// Checks a Solana public decryption: one encrypted store per handle, within the cap, every
/// handle on one chain. Returns that chain.
pub fn public_request_chain_id<H: AsRef<[u8]>>(
    handles: &[H],
    encrypted_stores: usize,
) -> Result<u64, SolanaRequestAssemblyError> {
    check_handle_count(handles.len())?;
    if handles.len() != encrypted_stores {
        return Err(SolanaRequestAssemblyError::StoreCount {
            handles: handles.len(),
            stores: encrypted_stores,
        });
    }
    common_chain_id(handles)
}

fn check_handle_count(handles: usize) -> Result<(), SolanaRequestAssemblyError> {
    if handles > MAX_REQUEST_HANDLES {
        return Err(SolanaRequestAssemblyError::TooManyHandles(handles));
    }
    Ok(())
}

/// The chain every handle names.
fn common_chain_id<H: AsRef<[u8]>>(handles: &[H]) -> Result<u64, SolanaRequestAssemblyError> {
    let mut chains = handles.iter().enumerate().map(|(index, handle)| {
        let handle = handle.as_ref();
        let handle: &[u8; 32] =
            handle
                .try_into()
                .map_err(|_| SolanaRequestAssemblyError::HandleWidth {
                    index,
                    len: handle.len(),
                })?;
        Ok((index, handle_chain_id(handle)))
    });
    let (_, first) = chains
        .next()
        .ok_or(SolanaRequestAssemblyError::NoHandles)??;
    for chain in chains {
        let (index, other) = chain?;
        if other != first {
            return Err(SolanaRequestAssemblyError::MixedChains {
                first,
                index,
                other,
            });
        }
    }
    Ok(first)
}

/// Bytes 22..30 of a handle, big-endian: the host chain id (`HandleOps.sol`).
fn handle_chain_id(handle: &[u8; 32]) -> u64 {
    let mut chain = [0u8; 8];
    chain.copy_from_slice(&handle[22..30]);
    u64::from_be_bytes(chain)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handle(chain_id: u64, seed: u8) -> Vec<u8> {
        let mut handle = vec![seed; 32];
        handle[22..30].copy_from_slice(&chain_id.to_be_bytes());
        handle
    }

    fn parts(
        handles: Vec<Vec<u8>>,
        entries: usize,
    ) -> (SolanaUserDecryptFields, SolanaRequestBlob) {
        let gateway = SolanaUserDecryptFields {
            handles,
            transport_key: vec![2; 32],
            start_timestamp: 4,
            duration_seconds: 5,
            extra_data: vec![8; 65],
        };
        let entry = |seed: u8| SolanaEntryClaims {
            owner_address: vec![seed; 32],
            encrypted_store: vec![seed.wrapping_add(1); 32],
        };
        let blob = SolanaRequestBlob {
            user_address: vec![1; 32],
            allowed_scopes: vec![vec![3; 64]],
            verifying_program_id: vec![6; 32],
            signature: vec![9; 64],
            entries: (0..entries).map(|i| entry(i as u8)).collect(),
        };
        (gateway, blob)
    }

    #[test]
    fn every_field_lands_in_its_place_and_the_chain_comes_from_the_handles() {
        let (gateway, blob) = parts(vec![handle(7, 0xa1), handle(7, 0xa2)], 2);
        let wire = assemble_solana_request(gateway.clone(), blob.clone()).unwrap();
        assert_eq!(
            wire.permit,
            PermitWireFields {
                user_address: blob.user_address,
                transport_key: gateway.transport_key,
                allowed_scopes: blob.allowed_scopes,
                start_timestamp: gateway.start_timestamp,
                duration_seconds: gateway.duration_seconds,
                verifying_program_id: blob.verifying_program_id,
                chain_id: 7,
                extra_data: gateway.extra_data,
            }
        );
        assert_eq!(wire.signature, blob.signature);
        for (index, entry) in wire.handles.iter().enumerate() {
            assert_eq!(entry.handle, gateway.handles[index]);
            assert_eq!(entry.owner_address, blob.entries[index].owner_address);
            assert_eq!(entry.encrypted_store, blob.entries[index].encrypted_store);
        }
    }

    #[test]
    fn refuses_parts_that_do_not_make_one_request() {
        let cases = [
            (parts(vec![], 0), SolanaRequestAssemblyError::NoHandles),
            (
                parts(
                    vec![handle(7, 1); MAX_REQUEST_HANDLES + 1],
                    MAX_REQUEST_HANDLES + 1,
                ),
                SolanaRequestAssemblyError::TooManyHandles(MAX_REQUEST_HANDLES + 1),
            ),
            (
                parts(vec![handle(7, 1)], 2),
                SolanaRequestAssemblyError::EntryCount {
                    handles: 1,
                    entries: 2,
                },
            ),
            (
                parts(vec![handle(7, 1), vec![0; 31]], 2),
                SolanaRequestAssemblyError::HandleWidth { index: 1, len: 31 },
            ),
            (
                parts(vec![handle(7, 1), handle(7, 2), handle(8, 3)], 3),
                SolanaRequestAssemblyError::MixedChains {
                    first: 7,
                    index: 2,
                    other: 8,
                },
            ),
        ];
        for ((gateway, blob), expected) in cases {
            assert_eq!(assemble_solana_request(gateway, blob), Err(expected));
        }
    }

    #[test]
    fn a_public_request_names_one_store_per_handle_on_one_chain() {
        assert_eq!(
            public_request_chain_id(&[handle(7, 1), handle(7, 2)], 2),
            Ok(7)
        );
        let cases = [
            (vec![], 0, SolanaRequestAssemblyError::NoHandles),
            (
                vec![handle(7, 1); MAX_REQUEST_HANDLES + 1],
                MAX_REQUEST_HANDLES + 1,
                SolanaRequestAssemblyError::TooManyHandles(MAX_REQUEST_HANDLES + 1),
            ),
            (
                vec![handle(7, 1), handle(7, 2)],
                1,
                SolanaRequestAssemblyError::StoreCount {
                    handles: 2,
                    stores: 1,
                },
            ),
            (
                vec![handle(7, 1), handle(8, 2)],
                2,
                SolanaRequestAssemblyError::MixedChains {
                    first: 7,
                    index: 1,
                    other: 8,
                },
            ),
        ];
        for (handles, stores, expected) in cases {
            assert_eq!(public_request_chain_id(&handles, stores), Err(expected));
        }
    }
}
