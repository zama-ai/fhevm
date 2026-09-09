//! The RFC 035 leaf record of Solana encrypted state accounts.
//!
//! Every state output the host accepts may seal leaves: one
//! historical-access leaf per allowed key on the handle it installs, then one
//! public-decrypt leaf when the handle is made public. The listener recomputes
//! those leaves from the confirmed instruction stream and keeps them next to the
//! compute rows they were derived with, in the same database transaction, so the
//! two records can never disagree about which blocks were applied.
//!
//! The reduce rule ([`reduce_block_leaves`]) is a pure function over one block, in
//! the style of `dependence_chains.rs`; the SQL steps around it load the touched
//! accounts, persist the reduction, and move the checkpoint.

use std::collections::BTreeMap;

use sqlx::Error as SqlxError;
use zama_solana_acl::{
    historical_access_leaf_commitment, mmr_append,
    public_decrypt_leaf_commitment, AclError,
};

use crate::database::tfhe_event_propagate::Transaction;

/// A `fhe_execute` state output, with its account resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedStateWrite {
    pub encrypted_state: [u8; 32],
    pub previous_leaf_count: u64,
    pub handle: [u8; 32],
    pub allowed_keys: Vec<[u8; 32]>,
    pub make_public: bool,
}

/// One host instruction that seals leaves, in on-chain order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LeafSource {
    State(EncryptedStateWrite),
}

/// The leaf sources of one confirmed transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionLeafSources {
    pub transaction_index: u64,
    pub sources: Vec<LeafSource>,
}

/// The persisted proof-history cursor of one encrypted state account.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedStateHistory {
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    /// False when the first observed output declared a nonzero prior count. The
    /// numeric cursor advances, but no leaf or proof can be served.
    pub history_complete: bool,
}

/// Persisted as the `leaf_kind` column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeafKind {
    HistoricalAccess = 0,
    PublicDecrypt = 1,
}

impl LeafKind {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            0 => Some(Self::HistoricalAccess),
            1 => Some(Self::PublicDecrypt),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedLeaf {
    pub encrypted_state: [u8; 32],
    pub leaf_index: u64,
    pub commitment: [u8; 32],
    pub kind: LeafKind,
    pub handle: [u8; 32],
    /// The allowed key of a historical-access leaf; `None` for public-decrypt.
    pub key: Option<[u8; 32]>,
    pub transaction_index: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BlockLeafReduction {
    /// Every account the block touched, with its state after the block.
    pub states: BTreeMap<[u8; 32], EncryptedStateHistory>,
    pub leaves: Vec<StagedLeaf>,
}

/// The confirmed chain performed a write this record cannot follow: the record
/// diverged from chain state, and continuing would seal wrong leaves.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LeafReduceError {
    #[error("encrypted state {} declared previous leaf count {declared}, record holds {recorded}", bs58::encode(encrypted_state).into_string())]
    PreviousLeafCountMismatch {
        encrypted_state: [u8; 32],
        declared: u64,
        recorded: u64,
    },
    #[error("leaf count overflow on encrypted state {}", bs58::encode(encrypted_state).into_string())]
    LeafCountOverflow { encrypted_state: [u8; 32] },
    #[error("MMR append failed on encrypted state {}: {error:?}", bs58::encode(encrypted_state).into_string())]
    Mmr {
        encrypted_state: [u8; 32],
        error: AclError,
    },
}

/// Reduces one block's leaf sources over the accounts' prior states.
///
/// `existing` holds the recorded cursor of every state the block touches. A
/// state first seen above leaf zero is tracked with `history_complete = false`:
/// its cursor advances, but no leaf or proof is stored.
pub fn reduce_block_leaves(
    transactions: &[TransactionLeafSources],
    existing: BTreeMap<[u8; 32], EncryptedStateHistory>,
) -> Result<BlockLeafReduction, LeafReduceError> {
    let mut reduction = BlockLeafReduction {
        states: existing,
        leaves: Vec::new(),
    };
    for transaction in transactions {
        for source in &transaction.sources {
            let LeafSource::State(write) = source;
            apply_write(&mut reduction, write, transaction.transaction_index)?;
        }
    }
    Ok(reduction)
}

fn apply_write(
    reduction: &mut BlockLeafReduction,
    write: &EncryptedStateWrite,
    transaction_index: u64,
) -> Result<(), LeafReduceError> {
    let account = write.encrypted_state;
    let state = reduction.states.entry(account).or_insert_with(|| {
        EncryptedStateHistory {
            leaf_count: write.previous_leaf_count,
            peaks: Vec::new(),
            history_complete: write.previous_leaf_count == 0,
        }
    });
    if state.leaf_count != write.previous_leaf_count {
        return Err(LeafReduceError::PreviousLeafCountMismatch {
            encrypted_state: account,
            declared: write.previous_leaf_count,
            recorded: state.leaf_count,
        });
    }
    if state.history_complete
        && state.peaks.len() != state.leaf_count.count_ones() as usize
    {
        return Err(LeafReduceError::Mmr {
            encrypted_state: account,
            error: AclError::MmrInconsistent,
        });
    }
    if !state.history_complete {
        let appended = u64::try_from(write.allowed_keys.len())
            .ok()
            .and_then(|count| count.checked_add(u64::from(write.make_public)))
            .and_then(|count| state.leaf_count.checked_add(count))
            .ok_or(LeafReduceError::LeafCountOverflow {
                encrypted_state: account,
            })?;
        state.leaf_count = appended;
        return Ok(());
    }
    for key in &write.allowed_keys {
        append_leaf(
            state,
            &mut reduction.leaves,
            account,
            LeafKind::HistoricalAccess,
            write.handle,
            Some(*key),
            transaction_index,
        )?;
    }
    if write.make_public {
        append_leaf(
            state,
            &mut reduction.leaves,
            account,
            LeafKind::PublicDecrypt,
            write.handle,
            None,
            transaction_index,
        )?;
    }
    Ok(())
}

fn append_leaf(
    state: &mut EncryptedStateHistory,
    leaves: &mut Vec<StagedLeaf>,
    account: [u8; 32],
    kind: LeafKind,
    handle: [u8; 32],
    key: Option<[u8; 32]>,
    transaction_index: u64,
) -> Result<(), LeafReduceError> {
    let leaf_index = state.leaf_count;
    let commitment = match key {
        Some(key) => {
            historical_access_leaf_commitment(account, leaf_index, handle, key)
        }
        None => public_decrypt_leaf_commitment(account, leaf_index, handle),
    };
    mmr_append(&mut state.peaks, &mut state.leaf_count, commitment).map_err(
        |error| LeafReduceError::Mmr {
            encrypted_state: account,
            error,
        },
    )?;
    leaves.push(StagedLeaf {
        encrypted_state: account,
        leaf_index,
        commitment,
        kind,
        handle,
        key,
        transaction_index,
    });
    Ok(())
}

// --- SQL -----------------------------------------------------------------------

fn bytes32(bytes: &[u8]) -> Result<[u8; 32], SqlxError> {
    bytes.try_into().map_err(|_| {
        SqlxError::Decode(
            format!("expected 32 bytes, found {}", bytes.len()).into(),
        )
    })
}

fn bytes32_vec(items: &[Vec<u8>]) -> Result<Vec<[u8; 32]>, SqlxError> {
    items.iter().map(|item| bytes32(item)).collect()
}

fn sql_i64(value: u64, field: &str) -> Result<i64, SqlxError> {
    i64::try_from(value).map_err(|_| {
        SqlxError::Protocol(format!("{field} exceeds PostgreSQL BIGINT"))
    })
}

fn sql_u64(value: i64, field: &str) -> Result<u64, SqlxError> {
    u64::try_from(value)
        .map_err(|_| SqlxError::Decode(format!("{field} is negative").into()))
}

/// Locks and loads the recorded state of `accounts`; accounts without a row are
/// absent from the result.
pub async fn load_encrypted_state_histories(
    tx: &mut Transaction<'_>,
    accounts: &[[u8; 32]],
) -> Result<BTreeMap<[u8; 32], EncryptedStateHistory>, SqlxError> {
    if accounts.is_empty() {
        return Ok(BTreeMap::new());
    }
    let keys: Vec<Vec<u8>> =
        accounts.iter().map(|account| account.to_vec()).collect();
    let rows = sqlx::query!(
        r#"
        SELECT encrypted_state, leaf_count, peaks, history_complete
        FROM solana_encrypted_states
        WHERE encrypted_state = ANY($1)
        FOR UPDATE
        "#,
        &keys,
    )
    .fetch_all(tx.as_mut())
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok((
                bytes32(&row.encrypted_state)?,
                EncryptedStateHistory {
                    leaf_count: sql_u64(row.leaf_count, "leaf_count")?,
                    peaks: bytes32_vec(&row.peaks)?,
                    history_complete: row.history_complete,
                },
            ))
        })
        .collect()
}

/// Persists a block's reduction: state cursors upserted, leaves appended.
pub async fn store_block_leaves(
    tx: &mut Transaction<'_>,
    slot: u64,
    reduction: &BlockLeafReduction,
) -> Result<(), SqlxError> {
    for (account, state) in &reduction.states {
        let leaf_count = sql_i64(state.leaf_count, "leaf_count")?;
        let last_slot = sql_i64(slot, "last_slot")?;
        let peaks: Vec<Vec<u8>> =
            state.peaks.iter().map(|peak| peak.to_vec()).collect();
        sqlx::query!(
            r#"
            INSERT INTO solana_encrypted_states
                (encrypted_state, leaf_count, peaks, history_complete, last_slot)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (encrypted_state) DO UPDATE SET
                leaf_count = EXCLUDED.leaf_count,
                peaks = EXCLUDED.peaks,
                last_slot = EXCLUDED.last_slot
            "#,
            &account[..],
            leaf_count,
            &peaks,
            state.history_complete,
            last_slot,
        )
        .execute(tx.as_mut())
        .await?;
    }
    for leaf in &reduction.leaves {
        let leaf_index = sql_i64(leaf.leaf_index, "leaf_index")?;
        let block_slot = sql_i64(slot, "block_slot")?;
        let transaction_index =
            sql_i64(leaf.transaction_index, "transaction_index")?;
        sqlx::query!(
            r#"
            INSERT INTO solana_encrypted_state_leaves
                (encrypted_state, leaf_index, commitment, leaf_kind, handle,
                 allowed_key, block_slot, transaction_index)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            &leaf.encrypted_state[..],
            leaf_index,
            &leaf.commitment[..],
            leaf.kind as i16,
            &leaf.handle[..],
            leaf.key.as_ref().map(|key| key.to_vec()),
            block_slot,
            transaction_index,
        )
        .execute(tx.as_mut())
        .await?;
    }
    Ok(())
}

/// The last sealed block the listener applied, moved inside each block's
/// transaction so a restart resumes exactly after the recorded work.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredCheckpoint {
    pub slot: u64,
    pub block_hash: [u8; 32],
}

pub async fn store_checkpoint(
    tx: &mut Transaction<'_>,
    checkpoint: &StoredCheckpoint,
) -> Result<(), SqlxError> {
    sqlx::query!(
        r#"
        INSERT INTO solana_listener_checkpoint (singleton, slot, block_hash)
        VALUES (1, $1, $2)
        ON CONFLICT (singleton) DO UPDATE SET
            slot = EXCLUDED.slot,
            block_hash = EXCLUDED.block_hash,
            updated_at = NOW()
        "#,
        sql_i64(checkpoint.slot, "checkpoint slot")?,
        &checkpoint.block_hash[..],
    )
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn load_checkpoint(
    pool: &sqlx::PgPool,
) -> Result<Option<StoredCheckpoint>, SqlxError> {
    let row = sqlx::query!(
        "SELECT slot, block_hash FROM solana_listener_checkpoint WHERE singleton = 1"
    )
    .fetch_optional(pool)
    .await?;
    row.map(|row| {
        Ok(StoredCheckpoint {
            slot: sql_u64(row.slot, "checkpoint slot")?,
            block_hash: bytes32(&row.block_hash)?,
        })
    })
    .transpose()
}

/// One state's recorded leaves as a proof reader needs them: the cursor row and
/// the ordered leaves below its `leaf_count`. Leaves committed after the state
/// row was read are cut off by that bound, so the two always agree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedLeaves {
    pub state: EncryptedStateHistory,
    pub leaves: Vec<StagedLeaf>,
}

pub async fn load_recorded_leaves(
    pool: &sqlx::PgPool,
    account: [u8; 32],
) -> Result<Option<RecordedLeaves>, SqlxError> {
    let Some(row) = sqlx::query!(
        r#"
        SELECT leaf_count, peaks, history_complete
        FROM solana_encrypted_states
        WHERE encrypted_state = $1
        "#,
        &account[..],
    )
    .fetch_optional(pool)
    .await?
    else {
        return Ok(None);
    };
    let state = EncryptedStateHistory {
        leaf_count: sql_u64(row.leaf_count, "leaf_count")?,
        peaks: bytes32_vec(&row.peaks)?,
        history_complete: row.history_complete,
    };
    let leaf_count = sql_i64(state.leaf_count, "leaf_count")?;
    let rows = sqlx::query!(
        r#"
        SELECT leaf_index, commitment, leaf_kind, handle, allowed_key, transaction_index
        FROM solana_encrypted_state_leaves
        WHERE encrypted_state = $1 AND leaf_index < $2
        ORDER BY leaf_index
        "#,
        &account[..],
        leaf_count,
    )
    .fetch_all(pool)
    .await?;
    let leaves = rows
        .into_iter()
        .map(|row| {
            Ok(StagedLeaf {
                encrypted_state: account,
                leaf_index: sql_u64(row.leaf_index, "leaf_index")?,
                commitment: bytes32(&row.commitment)?,
                kind: LeafKind::from_i16(row.leaf_kind).ok_or_else(|| {
                    SqlxError::Decode(
                        format!("unknown leaf_kind {}", row.leaf_kind).into(),
                    )
                })?,
                handle: bytes32(&row.handle)?,
                key: row.allowed_key.as_deref().map(bytes32).transpose()?,
                transaction_index: sql_u64(
                    row.transaction_index,
                    "transaction_index",
                )?,
            })
        })
        .collect::<Result<Vec<_>, SqlxError>>()?;
    Ok(Some(RecordedLeaves { state, leaves }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATE: [u8; 32] = [0xAC; 32];
    const OWNER: [u8; 32] = [0xA1; 32];

    fn write(
        previous_leaf_count: u64,
        handle: [u8; 32],
        allowed_keys: Vec<[u8; 32]>,
        make_public: bool,
    ) -> LeafSource {
        LeafSource::State(EncryptedStateWrite {
            encrypted_state: STATE,
            previous_leaf_count,
            handle,
            allowed_keys,
            make_public,
        })
    }

    fn transaction(sources: Vec<LeafSource>) -> TransactionLeafSources {
        TransactionLeafSources {
            transaction_index: 3,
            sources,
        }
    }

    #[test]
    fn appends_allows_then_public_and_advances_across_outputs() {
        let reduction = reduce_block_leaves(
            &[transaction(vec![
                write(0, [1; 32], vec![OWNER], false),
                write(1, [2; 32], vec![[0xB2; 32]], true),
            ])],
            BTreeMap::new(),
        )
        .unwrap();
        let history = &reduction.states[&STATE];
        assert!(history.history_complete);
        assert_eq!(history.leaf_count, 3);
        assert_eq!(history.peaks.len(), 2);
        assert_eq!(reduction.leaves.len(), 3);
        assert_eq!(reduction.leaves[0].key, Some(OWNER));
        assert_eq!(reduction.leaves[1].key, Some([0xB2; 32]));
        assert_eq!(reduction.leaves[2].kind, LeafKind::PublicDecrypt);
        assert_eq!(reduction.leaves[2].handle, [2; 32]);
    }

    #[test]
    fn first_seen_mid_history_stays_incomplete_but_tracks_continuity() {
        let first = reduce_block_leaves(
            &[transaction(vec![write(7, [1; 32], vec![OWNER], true)])],
            BTreeMap::new(),
        )
        .unwrap();
        let history = &first.states[&STATE];
        assert!(!history.history_complete);
        assert_eq!(history.leaf_count, 9);
        assert!(history.peaks.is_empty());
        assert!(first.leaves.is_empty());

        let second = reduce_block_leaves(
            &[transaction(vec![write(9, [2; 32], vec![OWNER], false)])],
            first.states,
        )
        .unwrap();
        assert_eq!(second.states[&STATE].leaf_count, 10);
        assert!(!second.states[&STATE].history_complete);
        assert!(second.leaves.is_empty());
    }

    #[test]
    fn rejects_count_gap_for_complete_and_incomplete_histories() {
        for history in [
            EncryptedStateHistory {
                leaf_count: 4,
                peaks: vec![[1; 32]],
                history_complete: true,
            },
            EncryptedStateHistory {
                leaf_count: 4,
                peaks: vec![],
                history_complete: false,
            },
        ] {
            let error = reduce_block_leaves(
                &[transaction(vec![write(3, [2; 32], vec![], false)])],
                BTreeMap::from([(STATE, history)]),
            )
            .unwrap_err();
            assert_eq!(
                error,
                LeafReduceError::PreviousLeafCountMismatch {
                    encrypted_state: STATE,
                    declared: 3,
                    recorded: 4,
                }
            );
        }
    }

    #[test]
    fn zero_leaf_output_still_checks_continuity() {
        let reduction = reduce_block_leaves(
            &[transaction(vec![write(0, [1; 32], vec![], false)])],
            BTreeMap::new(),
        )
        .unwrap();
        assert_eq!(reduction.states[&STATE].leaf_count, 0);
        assert!(reduction.states[&STATE].history_complete);
        assert!(reduction.leaves.is_empty());
    }

    #[test]
    fn zero_leaf_output_rejects_malformed_complete_history() {
        let error = reduce_block_leaves(
            &[transaction(vec![write(0, [1; 32], vec![], false)])],
            BTreeMap::from([(
                STATE,
                EncryptedStateHistory {
                    leaf_count: 0,
                    peaks: vec![[1; 32]],
                    history_complete: true,
                },
            )]),
        )
        .unwrap_err();
        assert_eq!(
            error,
            LeafReduceError::Mmr {
                encrypted_state: STATE,
                error: AclError::MmrInconsistent,
            }
        );
    }
}
