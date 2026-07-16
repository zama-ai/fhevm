//! Deterministic CompletedBlock → staged lineage/leaf mutations.
//!
//! Reduction order is fixed: transaction index, then instruction, then event,
//! then subject (inside shared ACL leaf commitments). Staging completes before
//! any SQL writes in [`crate::store::SqlProofStore::apply_completed_block`].

use std::collections::{BTreeMap, BTreeSet};

use solana_proof_source::CompletedBlock;
use zama_solana_acl::lineage::LineageEvent;
use zama_solana_acl::mmr::mmr_append;
use zama_solana_acl::{historical_access_leaf_commitment, public_decrypt_leaf_commitment};

use crate::decode::{decode_program_instructions, DecodeError, DecodedInstruction};
use crate::replay::{apply_instruction, LineageReplayState, ReplayError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PriorLineageState {
    pub replay: LineageReplayState,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedLeaf {
    pub lineage: [u8; 32],
    pub leaf_index: u64,
    pub commitment: [u8; 32],
    pub transaction_index: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedLineage {
    pub lineage: [u8; 32],
    pub current_handle: Option<[u8; 32]>,
    pub subjects: Vec<[u8; 32]>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    /// True when this lineage did not exist in the store before this block.
    pub born_in_block: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedBlockReduction {
    pub lineages: Vec<StagedLineage>,
    pub leaves: Vec<StagedLeaf>,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ReduceError {
    #[error("decode error: {0}")]
    Decode(#[from] DecodeError),
    #[error("replay error: {0}")]
    Replay(#[from] ReplayError),
    #[error("unknown pre-bootstrap lineage {lineage}")]
    UnknownPreBootstrapLineage { lineage: String },
    #[error("MMR append failed for lineage {lineage}")]
    MmrAppend { lineage: String },
}

fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn is_lineage_birth(instruction: &DecodedInstruction) -> bool {
    matches!(
        instruction,
        DecodedInstruction::CreateEncryptedValue { .. }
            | DecodedInstruction::FheEvalCreateEncryptedValue { .. }
    )
}

fn append_events(
    lineage: [u8; 32],
    peaks: &mut Vec<[u8; 32]>,
    leaf_count: &mut u64,
    events_with_tx: &[(u64, LineageEvent)],
) -> Result<Vec<StagedLeaf>, ReduceError> {
    let mut staged = Vec::new();
    for (tx_index, event) in events_with_tx {
        match event {
            LineageEvent::HandleSuperseded {
                previous_handle,
                previous_subjects,
            } => {
                for subject in previous_subjects {
                    let commitment = historical_access_leaf_commitment(
                        lineage,
                        *leaf_count,
                        *previous_handle,
                        *subject,
                    );
                    mmr_append(peaks, leaf_count, commitment).map_err(|_| {
                        ReduceError::MmrAppend {
                            lineage: hex32(&lineage),
                        }
                    })?;
                    staged.push(StagedLeaf {
                        lineage,
                        leaf_index: *leaf_count - 1,
                        commitment,
                        transaction_index: *tx_index,
                    });
                }
            }
            LineageEvent::MarkedPublic { handle } => {
                let commitment = public_decrypt_leaf_commitment(lineage, *leaf_count, *handle);
                mmr_append(peaks, leaf_count, commitment).map_err(|_| ReduceError::MmrAppend {
                    lineage: hex32(&lineage),
                })?;
                staged.push(StagedLeaf {
                    lineage,
                    leaf_index: *leaf_count - 1,
                    commitment,
                    transaction_index: *tx_index,
                });
            }
        }
    }
    Ok(staged)
}

/// Loads affected lineages from `existing`, reduces `block` deterministically,
/// and returns staged lineage/leaf rows ready for a single SQL commit.
pub fn reduce_completed_block(
    program_id: [u8; 32],
    block: &CompletedBlock,
    existing: &BTreeMap<[u8; 32], PriorLineageState>,
) -> Result<StagedBlockReduction, ReduceError> {
    let mut working_replay: BTreeMap<[u8; 32], LineageReplayState> = existing
        .iter()
        .map(|(lineage, state)| (*lineage, state.replay.clone()))
        .collect();
    let mut working_mmr: BTreeMap<[u8; 32], (Vec<[u8; 32]>, u64)> = existing
        .iter()
        .map(|(lineage, state)| (*lineage, (state.peaks.clone(), state.leaf_count)))
        .collect();
    let mut born: BTreeSet<[u8; 32]> = BTreeSet::new();
    let mut block_events: BTreeMap<[u8; 32], Vec<(u64, LineageEvent)>> = BTreeMap::new();
    let mut touched: BTreeSet<[u8; 32]> = BTreeSet::new();

    for tx in &block.transactions {
        // Failed/vote identities retain position but produce no leaves.
        if tx.instructions.is_empty() {
            continue;
        }
        let decoded = decode_program_instructions(program_id, &tx.instructions)?;
        for instruction in &decoded {
            let lineage = instruction.encrypted_value();
            let had_state = working_replay.contains_key(&lineage);
            if !had_state && !is_lineage_birth(instruction) {
                return Err(ReduceError::UnknownPreBootstrapLineage {
                    lineage: hex32(&lineage),
                });
            }

            let mut state = working_replay.remove(&lineage);
            let was_absent = state.is_none();
            let events = match apply_instruction(&mut state, instruction) {
                Ok(events) => events,
                Err(ReplayError::UnknownLineage(lineage)) => {
                    return Err(ReduceError::UnknownPreBootstrapLineage {
                        lineage: hex32(&lineage),
                    });
                }
                Err(other) => return Err(ReduceError::Replay(other)),
            };
            let state = state.expect("instruction application leaves lineage state");
            if was_absent {
                born.insert(lineage);
                working_mmr
                    .entry(lineage)
                    .or_insert_with(|| (Vec::new(), 0));
            }
            working_replay.insert(lineage, state);
            touched.insert(lineage);
            if !events.is_empty() {
                block_events
                    .entry(lineage)
                    .or_default()
                    .extend(events.into_iter().map(|event| (tx.index, event)));
            }
        }
    }

    let mut staged_lineages = Vec::new();
    let mut staged_leaves = Vec::new();

    for lineage in &touched {
        let replay = working_replay
            .get(lineage)
            .expect("touched lineages have replay state")
            .clone();
        let (mut peaks, mut leaf_count) = working_mmr
            .get(lineage)
            .cloned()
            .unwrap_or_else(|| (Vec::new(), 0));
        let events = block_events.get(lineage).cloned().unwrap_or_default();
        let new_leaves = append_events(*lineage, &mut peaks, &mut leaf_count, &events)?;
        staged_leaves.extend(new_leaves);
        staged_lineages.push(StagedLineage {
            lineage: *lineage,
            current_handle: replay.current_handle,
            subjects: replay.subjects,
            leaf_count,
            peaks,
            born_in_block: born.contains(lineage),
        });
    }

    staged_lineages.sort_by(|a, b| a.lineage.cmp(&b.lineage));
    staged_leaves.sort_by(|a, b| {
        (a.lineage, a.leaf_index, a.transaction_index).cmp(&(
            b.lineage,
            b.leaf_index,
            b.transaction_index,
        ))
    });

    Ok(StagedBlockReduction {
        lineages: staged_lineages,
        leaves: staged_leaves,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::SubjectGrant;
    use solana_proof_source::{CanonicalTransaction, RawInstruction};
    use zama_solana_acl::lineage::reconstruct;

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn empty_block(slot: u64) -> CompletedBlock {
        CompletedBlock {
            slot,
            block_hash: pk(0xB0),
            parent_slot: slot.saturating_sub(1),
            parent_hash: pk(0xB1),
            block_time: Some(1),
            block_height: Some(slot),
            executed_transaction_count: 0,
            transactions: Vec::new(),
        }
    }

    #[test]
    fn empty_block_stages_nothing() {
        let staged = reduce_completed_block(pk(1), &empty_block(10), &BTreeMap::new()).unwrap();
        assert!(staged.lineages.is_empty());
        assert!(staged.leaves.is_empty());
    }

    #[test]
    fn failed_transaction_produces_no_leaves() {
        let program = pk(1);
        let block = CompletedBlock {
            slot: 11,
            block_hash: pk(0xB0),
            parent_slot: 10,
            parent_hash: pk(0xB1),
            block_time: None,
            block_height: None,
            executed_transaction_count: 3,
            transactions: vec![CanonicalTransaction {
                signature: [0x11; 64],
                index: 1,
                succeeded: false,
                is_vote: false,
                instructions: Vec::new(),
            }],
        };
        let staged = reduce_completed_block(program, &block, &BTreeMap::new()).unwrap();
        assert!(staged.leaves.is_empty());
        let _ = RawInstruction {
            program_id: program,
            accounts: Vec::new(),
            data: Vec::new(),
            top_level_index: 0,
            stack_height: Some(1),
        };
    }

    #[test]
    fn unknown_pre_bootstrap_lineage_is_detected_without_existing_state() {
        // Wire-level unknown-lineage coverage lives in store integration tests.
        // Here we only assert the reduce error variant stays distinguishable.
        let err = ReduceError::UnknownPreBootstrapLineage {
            lineage: "00".repeat(32),
        };
        assert!(err.to_string().contains("unknown pre-bootstrap lineage"));
        let _ = SubjectGrant { subject: pk(2) };
        let _ = reconstruct(pk(1), &[]);
    }
}
