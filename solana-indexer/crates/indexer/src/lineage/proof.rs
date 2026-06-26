//! Reconstructs a lineage's leaf list and builds a mode-prefixed inclusion proof.
//!
//! Output bytes = 1 mode-prefix byte (`0x01` historical / `0x02` public) ‖
//! `Borsh(MmrProof)`. The prefix is chosen from the kind of the event that
//! produced the leaf at `leaf_index` (Rotation -> historical, MarkedPublic ->
//! public), which the ordered event log preserves.
//!
//! Cross-check: when an RPC client is available, fetch the live PDA account,
//! decode its `(peaks, leaf_count)` and call `build_verified_proof`; on RPC
//! failure/timeout, fall back to the unverified proof with `verified = false`
//! and a logged warning.

use borsh::BorshSerialize;
use zama_solana_acl::lineage::{reconstruct, LineageEvent, ReconstructedLineage};
use zama_solana_acl::{decode_account, MmrProof};

use crate::store::repositories::lineage_repo::EventRow;

/// Mode prefix byte: leaf came from a `rotate_encrypted_value`.
pub const MODE_HISTORICAL: u8 = 0x01;
/// Mode prefix byte: leaf came from a `mark_encrypted_value_public`.
pub const MODE_PUBLIC: u8 = 0x02;

#[derive(Debug, thiserror::Error)]
pub enum ProofError {
    #[error("leaf_index {0} out of range (leaf_count {1})")]
    LeafIndexOutOfRange(u64, u64),
    #[error("lineage reconstruction failed: {0:?}")]
    Reconstruct(zama_solana_acl::LineageError),
    /// Serialising the built `MmrProof` to bytes failed — an I/O/allocation fault,
    /// NOT a peaks divergence.
    #[error("proof serialisation failed: {0}")]
    Encode(#[source] anyhow::Error),
}

/// A built proof plus the metadata the API returns.
pub struct BuiltProof {
    /// `mode_prefix ‖ Borsh(MmrProof)`.
    pub bytes: Vec<u8>,
    pub leaf_count: u64,
    /// Whether the reconstruction was cross-checked against the live on-chain
    /// `(peaks, leaf_count)`. `false` means the RPC check could not be performed.
    pub verified: bool,
}

/// Picks the mode prefix for the leaf at `leaf_index` by walking the events in
/// the same order `reconstruct` lays out leaves.
fn mode_prefix_for_leaf(events: &[LineageEvent], leaf_index: u64) -> Option<u8> {
    let mut idx: u64 = 0;
    for event in events {
        match event {
            LineageEvent::Rotation {
                subjects_before_rotation,
                ..
            } => {
                let n = subjects_before_rotation.len() as u64;
                if leaf_index < idx + n {
                    return Some(MODE_HISTORICAL);
                }
                idx += n;
            }
            LineageEvent::MarkedPublic { .. } => {
                if leaf_index == idx {
                    return Some(MODE_PUBLIC);
                }
                idx += 1;
            }
        }
    }
    None
}

fn encode_proof(mode: u8, proof: &MmrProof) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(1 + 32);
    bytes.push(mode);
    proof.serialize(&mut bytes)?;
    Ok(bytes)
}

/// Reconstructs from `event_rows`, then builds the mode-prefixed proof for
/// `leaf_index`. `on_chain` is the live `(peaks, leaf_count)` from the PDA, when a
/// cross-check could be performed; pass `None` to skip verification.
pub fn build(
    pda: [u8; 32],
    event_rows: &[EventRow],
    leaf_index: u64,
    on_chain: Option<(Vec<[u8; 32]>, u64)>,
) -> Result<BuiltProof, ProofError> {
    let events: Vec<LineageEvent> = event_rows.iter().map(|r| r.event.clone()).collect();
    let lineage: ReconstructedLineage =
        reconstruct(pda, &events).map_err(ProofError::Reconstruct)?;

    if leaf_index >= lineage.leaf_count {
        return Err(ProofError::LeafIndexOutOfRange(
            leaf_index,
            lineage.leaf_count,
        ));
    }

    let mode = mode_prefix_for_leaf(&events, leaf_index).ok_or(ProofError::LeafIndexOutOfRange(
        leaf_index,
        lineage.leaf_count,
    ))?;

    let (proof, verified) = match on_chain {
        Some((peaks, leaf_count)) => {
            // Cross-check against the chain; a divergence surfaces as a
            // reconstruction error rather than a doomed proof handed downstream.
            let proof = lineage
                .build_verified_proof(&peaks, leaf_count, leaf_index)
                .map_err(ProofError::Reconstruct)?;
            (proof, true)
        }
        None => {
            let proof = lineage
                .build_proof(leaf_index)
                .ok_or(ProofError::LeafIndexOutOfRange(
                    leaf_index,
                    lineage.leaf_count,
                ))?;
            (proof, false)
        }
    };

    let bytes = encode_proof(mode, &proof).map_err(ProofError::Encode)?;

    Ok(BuiltProof {
        bytes,
        leaf_count: lineage.leaf_count,
        verified,
    })
}

/// Decodes the on-chain `(peaks, leaf_count)` from raw PDA account data, for the
/// build_verified_proof cross-check. Returns `None` if the account cannot be
/// decoded as an `EncryptedValueAcl`.
pub fn on_chain_peaks_from_account(data: &[u8]) -> Option<(Vec<[u8; 32]>, u64)> {
    let acl = decode_account(data).ok()?;
    Some((acl.peaks, acl.leaf_count))
}
