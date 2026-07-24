//! Deterministic CompletedBlock → staged encrypted_value_account/leaf mutations.
//!
//! Reduction order is fixed: transaction index, then instruction, then event,
//! then subject (inside shared ACL leaf commitments). Staging completes before
//! any SQL writes in [`crate::store::SqlProofStore::apply_completed_block`].

use std::collections::{BTreeMap, BTreeSet};

use solana_proof_source::CompletedBlock;
use zama_solana_acl::mmr::mmr_append;
use zama_solana_acl::value_account::EncryptedValueAccountEvent;
use zama_solana_acl::{historical_access_leaf_commitment, public_decrypt_leaf_commitment};

use crate::decode::{decode_program_instructions, DecodeError, DecodedInstruction};
use crate::replay::{apply_instruction, LineageReplayState, ReplayError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PriorLineageState {
    pub replay: LineageReplayState,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
}

/// Which semantic leaf a staged row is, so the proof service can resolve
/// `(encrypted_value_account, handle[, subject], kind) -> leaf_index` without recomputing hashes.
/// Persisted as the `leaf_kind` SMALLINT (0 / 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeafKind {
    /// `ZAMA_HIST_ACCESS_LEAF_V1`, keyed by (handle, subject).
    HistoricalAccess,
    /// `ZAMA_PUBLIC_DECRYPT_LEAF_V1`, keyed by (handle); no subject.
    PublicDecrypt,
}

impl LeafKind {
    pub fn as_i16(self) -> i16 {
        match self {
            LeafKind::HistoricalAccess => 0,
            LeafKind::PublicDecrypt => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedLeaf {
    pub encrypted_value_account: [u8; 32],
    pub leaf_index: u64,
    pub commitment: [u8; 32],
    pub transaction_index: u64,
    /// Semantic classification of the leaf, persisted for indexed resolution.
    pub kind: LeafKind,
    /// The ciphertext handle sealed into the leaf preimage.
    pub handle: [u8; 32],
    /// The authorized subject (historical-access leaves only; `None` for public-decrypt).
    pub subject: Option<[u8; 32]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedEncryptedValueAccount {
    pub encrypted_value_account: [u8; 32],
    pub current_handle: Option<[u8; 32]>,
    pub subjects: Vec<[u8; 32]>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    /// True when this encrypted_value_account did not exist in the store before this block.
    pub born_in_block: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedBlockReduction {
    pub encrypted_value_accounts: Vec<StagedEncryptedValueAccount>,
    pub leaves: Vec<StagedLeaf>,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ReduceError {
    #[error("decode error: {0}")]
    Decode(#[from] DecodeError),
    #[error("replay error: {0}")]
    Replay(#[from] ReplayError),
    #[error("unknown pre-bootstrap encrypted_value_account {encrypted_value_account}")]
    UnknownPreBootstrapLineage { encrypted_value_account: String },
    #[error("MMR append failed for encrypted_value_account {encrypted_value_account}")]
    MmrAppend { encrypted_value_account: String },
}

fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn is_encrypted_value_account_birth(instruction: &DecodedInstruction) -> bool {
    matches!(
        instruction,
        DecodedInstruction::CreateEncryptedValue { .. }
            | DecodedInstruction::FheEvalCreateEncryptedValue { .. }
    )
}

fn append_events(
    encrypted_value_account: [u8; 32],
    peaks: &mut Vec<[u8; 32]>,
    leaf_count: &mut u64,
    events_with_tx: &[(u64, EncryptedValueAccountEvent)],
) -> Result<Vec<StagedLeaf>, ReduceError> {
    let mut staged = Vec::new();
    for (tx_index, event) in events_with_tx {
        match event {
            EncryptedValueAccountEvent::HandleSuperseded {
                previous_handle,
                previous_subjects,
            } => {
                for subject in previous_subjects {
                    let commitment = historical_access_leaf_commitment(
                        encrypted_value_account,
                        *leaf_count,
                        *previous_handle,
                        *subject,
                    );
                    mmr_append(peaks, leaf_count, commitment).map_err(|_| {
                        ReduceError::MmrAppend {
                            encrypted_value_account: hex32(&encrypted_value_account),
                        }
                    })?;
                    staged.push(StagedLeaf {
                        encrypted_value_account,
                        leaf_index: *leaf_count - 1,
                        commitment,
                        transaction_index: *tx_index,
                        kind: LeafKind::HistoricalAccess,
                        handle: *previous_handle,
                        subject: Some(*subject),
                    });
                }
            }
            EncryptedValueAccountEvent::MarkedPublic { handle } => {
                let commitment =
                    public_decrypt_leaf_commitment(encrypted_value_account, *leaf_count, *handle);
                mmr_append(peaks, leaf_count, commitment).map_err(|_| ReduceError::MmrAppend {
                    encrypted_value_account: hex32(&encrypted_value_account),
                })?;
                staged.push(StagedLeaf {
                    encrypted_value_account,
                    leaf_index: *leaf_count - 1,
                    commitment,
                    transaction_index: *tx_index,
                    kind: LeafKind::PublicDecrypt,
                    handle: *handle,
                    subject: None,
                });
            }
        }
    }
    Ok(staged)
}

/// Loads affected encrypted_value_accounts from `existing`, reduces `block` deterministically,
/// and returns staged encrypted_value_account/leaf rows ready for a single SQL commit.
pub fn reduce_completed_block(
    program_id: [u8; 32],
    block: &CompletedBlock,
    existing: &BTreeMap<[u8; 32], PriorLineageState>,
) -> Result<StagedBlockReduction, ReduceError> {
    let mut working_replay: BTreeMap<[u8; 32], LineageReplayState> = existing
        .iter()
        .map(|(encrypted_value_account, state)| (*encrypted_value_account, state.replay.clone()))
        .collect();
    let mut working_mmr: BTreeMap<[u8; 32], (Vec<[u8; 32]>, u64)> = existing
        .iter()
        .map(|(encrypted_value_account, state)| {
            (
                *encrypted_value_account,
                (state.peaks.clone(), state.leaf_count),
            )
        })
        .collect();
    let mut born: BTreeSet<[u8; 32]> = BTreeSet::new();
    let mut block_events: BTreeMap<[u8; 32], Vec<(u64, EncryptedValueAccountEvent)>> =
        BTreeMap::new();
    let mut touched: BTreeSet<[u8; 32]> = BTreeSet::new();

    for tx in &block.transactions {
        // Failed/vote identities retain position (store inserts them) but must
        // never produce MMR leaves — even if a caller handed us instructions.
        if !tx.succeeded || tx.is_vote {
            continue;
        }
        if tx.instructions.is_empty() {
            continue;
        }
        let decoded = decode_program_instructions(program_id, &tx.instructions)?;
        for instruction in &decoded {
            let encrypted_value_account = instruction.encrypted_value();
            let had_state = working_replay.contains_key(&encrypted_value_account);
            if !had_state && !is_encrypted_value_account_birth(instruction) {
                return Err(ReduceError::UnknownPreBootstrapLineage {
                    encrypted_value_account: hex32(&encrypted_value_account),
                });
            }

            let mut state = working_replay.remove(&encrypted_value_account);
            let was_absent = state.is_none();
            let events = match apply_instruction(&mut state, instruction) {
                Ok(events) => events,
                Err(ReplayError::UnknownLineage(encrypted_value_account)) => {
                    return Err(ReduceError::UnknownPreBootstrapLineage {
                        encrypted_value_account: hex32(&encrypted_value_account),
                    });
                }
                Err(other) => return Err(ReduceError::Replay(other)),
            };
            let state =
                state.expect("instruction application leaves encrypted_value_account state");
            if was_absent {
                born.insert(encrypted_value_account);
                working_mmr
                    .entry(encrypted_value_account)
                    .or_insert_with(|| (Vec::new(), 0));
            }
            working_replay.insert(encrypted_value_account, state);
            touched.insert(encrypted_value_account);
            if !events.is_empty() {
                block_events
                    .entry(encrypted_value_account)
                    .or_default()
                    .extend(events.into_iter().map(|event| (tx.index, event)));
            }
        }
    }

    let mut staged_encrypted_value_accounts = Vec::new();
    let mut staged_leaves = Vec::new();

    for encrypted_value_account in &touched {
        let replay = working_replay
            .get(encrypted_value_account)
            .expect("touched encrypted_value_accounts have replay state")
            .clone();
        let (mut peaks, mut leaf_count) = working_mmr
            .get(encrypted_value_account)
            .cloned()
            .unwrap_or_else(|| (Vec::new(), 0));
        let events = block_events
            .get(encrypted_value_account)
            .cloned()
            .unwrap_or_default();
        let new_leaves = append_events(
            *encrypted_value_account,
            &mut peaks,
            &mut leaf_count,
            &events,
        )?;
        staged_leaves.extend(new_leaves);
        staged_encrypted_value_accounts.push(StagedEncryptedValueAccount {
            encrypted_value_account: *encrypted_value_account,
            current_handle: replay.current_handle,
            subjects: replay.subjects,
            leaf_count,
            peaks,
            born_in_block: born.contains(encrypted_value_account),
        });
    }

    staged_encrypted_value_accounts
        .sort_by(|a, b| a.encrypted_value_account.cmp(&b.encrypted_value_account));
    staged_leaves.sort_by(|a, b| {
        (a.encrypted_value_account, a.leaf_index, a.transaction_index).cmp(&(
            b.encrypted_value_account,
            b.leaf_index,
            b.transaction_index,
        ))
    });

    Ok(StagedBlockReduction {
        encrypted_value_accounts: staged_encrypted_value_accounts,
        leaves: staged_leaves,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::SubjectGrant;
    use solana_proof_source::{CanonicalTransaction, RawInstruction};
    use zama_solana_acl::value_account::reconstruct;

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
        assert!(staged.encrypted_value_accounts.is_empty());
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
                // Even with instructions present, failed txs must stage zero leaves.
                instructions: vec![RawInstruction {
                    program_id: program,
                    accounts: vec![pk(0xA), pk(0xB), pk(0xE), pk(0xC), pk(0xD)],
                    data: vec![0u8; 8],
                    top_level_index: 0,
                    stack_height: Some(1),
                }],
            }],
        };
        let staged = reduce_completed_block(program, &block, &BTreeMap::new()).unwrap();
        assert!(staged.leaves.is_empty());
        assert!(staged.encrypted_value_accounts.is_empty());
    }

    #[test]
    fn vote_transaction_produces_no_leaves() {
        let program = pk(1);
        let block = CompletedBlock {
            slot: 12,
            block_hash: pk(0xB0),
            parent_slot: 11,
            parent_hash: pk(0xB1),
            block_time: None,
            block_height: None,
            executed_transaction_count: 1,
            transactions: vec![CanonicalTransaction {
                signature: [0x22; 64],
                index: 0,
                succeeded: true,
                is_vote: true,
                instructions: vec![RawInstruction {
                    program_id: program,
                    accounts: Vec::new(),
                    data: Vec::new(),
                    top_level_index: 0,
                    stack_height: Some(1),
                }],
            }],
        };
        let staged = reduce_completed_block(program, &block, &BTreeMap::new()).unwrap();
        assert!(staged.leaves.is_empty());
        assert!(staged.encrypted_value_accounts.is_empty());
    }

    #[test]
    fn unknown_pre_bootstrap_encrypted_value_account_is_detected_without_existing_state() {
        // Wire-level unknown-encrypted_value_account coverage lives in store integration tests.
        // Here we only assert the reduce error variant stays distinguishable.
        let err = ReduceError::UnknownPreBootstrapLineage {
            encrypted_value_account: "00".repeat(32),
        };
        assert!(err
            .to_string()
            .contains("unknown pre-bootstrap encrypted_value_account"));
        let _ = SubjectGrant { subject: pk(2) };
        let _ = reconstruct(pk(1), &[]);
    }
}
