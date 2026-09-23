//! The RFC 035 leaf record of Solana encrypted store accounts.
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
//!
//! Repair replays slots the record already holds: the operator reverts the compute rows
//! and the checkpoint, and leaves stay. A replayed block's leaves are recomputed and
//! must equal the recorded ones exactly ([`load_block_leaves`]).

use std::collections::{BTreeMap, BTreeSet};

use sqlx::Error as SqlxError;
use zama_solana_acl::{
    historical_access_leaf_commitment, mmr_append,
    public_decrypt_leaf_commitment, AclError,
};

use crate::database::tfhe_event_propagate::Transaction;

/// A `fhe_execute` state output, with its account resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedStoreWrite {
    pub encrypted_store: [u8; 32],
    pub previous_leaf_count: u64,
    pub handle: [u8; 32],
    pub allowed_keys: Vec<[u8; 32]>,
    pub make_public: bool,
}

/// The leaf sources of one confirmed transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionStoreWrites {
    pub transaction_index: u64,
    pub sources: Vec<EncryptedStoreWrite>,
}

/// The persisted proof-history cursor of one encrypted store account.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedStoreHistory {
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    /// False when the first observed output declared a nonzero prior count. The
    /// numeric cursor advances, but no leaf or proof can be served.
    pub history_complete: bool,
    /// Slot of the last block that wrote the account.
    pub last_slot: u64,
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
    pub encrypted_store: [u8; 32],
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
    /// Every account the block advanced, with its state after the block.
    pub states: BTreeMap<[u8; 32], EncryptedStoreHistory>,
    pub leaves: Vec<StagedLeaf>,
    /// Accounts whose record already holds this block, with the leaves the block
    /// recomputes for them. They must equal the recorded leaves of the same slot.
    pub replayed: BTreeMap<[u8; 32], Vec<StagedLeaf>>,
}

/// The confirmed chain performed a write this record cannot follow: the record
/// diverged from chain state, and continuing would seal wrong leaves.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LeafReduceError {
    #[error("encrypted store {} declared previous leaf count {declared}, record holds {recorded}", bs58::encode(encrypted_store).into_string())]
    PreviousLeafCountMismatch {
        encrypted_store: [u8; 32],
        declared: u64,
        recorded: u64,
    },
    #[error("leaf count overflow on encrypted store {}", bs58::encode(encrypted_store).into_string())]
    LeafCountOverflow { encrypted_store: [u8; 32] },
    #[error("MMR append failed on encrypted store {}: {error:?}", bs58::encode(encrypted_store).into_string())]
    Mmr {
        encrypted_store: [u8; 32],
        error: AclError,
    },
}

/// Reduces the leaf sources of the block at `slot` over the accounts' prior states.
///
/// `existing` holds the recorded cursor of every state the block touches. A
/// state first seen above leaf zero is tracked with `history_complete = false`:
/// its cursor advances, but no leaf or proof is stored.
///
/// An account whose last recorded write is at or after `slot` already holds this
/// block: its writes are replayed below the cursor instead of appended.
pub fn reduce_block_leaves(
    slot: u64,
    transactions: &[TransactionStoreWrites],
    mut existing: BTreeMap<[u8; 32], EncryptedStoreHistory>,
) -> Result<BlockLeafReduction, LeafReduceError> {
    let mut reduction = BlockLeafReduction::default();
    let mut replay_cursors = BTreeMap::new();
    let mut advanced = BTreeSet::new();
    let replaying: BTreeSet<[u8; 32]> = existing
        .iter()
        .filter(|(_, recorded)| slot <= recorded.last_slot)
        .map(|(account, _)| *account)
        .collect();
    for transaction in transactions {
        for write in &transaction.sources {
            let account = write.encrypted_store;
            match existing.get(&account) {
                Some(recorded) if replaying.contains(&account) => replay_write(
                    &mut reduction,
                    &mut replay_cursors,
                    recorded,
                    write,
                    transaction.transaction_index,
                )?,
                _ => {
                    let state = existing.entry(account).or_insert_with(|| {
                        EncryptedStoreHistory {
                            leaf_count: write.previous_leaf_count,
                            peaks: Vec::new(),
                            history_complete: write.previous_leaf_count == 0,
                            last_slot: slot,
                        }
                    });
                    state.last_slot = slot;
                    apply_write(
                        state,
                        &mut reduction.leaves,
                        write,
                        transaction.transaction_index,
                    )?;
                    advanced.insert(account);
                }
            }
        }
    }
    reduction.states = existing
        .into_iter()
        .filter(|(account, _)| advanced.contains(account))
        .collect();
    Ok(reduction)
}

/// Recomputes a write the record already holds. Its leaves must fit below the
/// recorded cursor and follow the block's previous replayed write on the account.
fn replay_write(
    reduction: &mut BlockLeafReduction,
    replay_cursors: &mut BTreeMap<[u8; 32], u64>,
    recorded: &EncryptedStoreHistory,
    write: &EncryptedStoreWrite,
    transaction_index: u64,
) -> Result<(), LeafReduceError> {
    let account = write.encrypted_store;
    let cursor = *replay_cursors
        .entry(account)
        .or_insert(write.previous_leaf_count);
    if write.previous_leaf_count != cursor {
        return Err(LeafReduceError::PreviousLeafCountMismatch {
            encrypted_store: account,
            declared: write.previous_leaf_count,
            recorded: cursor,
        });
    }
    let end = leaf_count_after(write, write.previous_leaf_count)?;
    if end > recorded.leaf_count {
        return Err(LeafReduceError::PreviousLeafCountMismatch {
            encrypted_store: account,
            declared: write.previous_leaf_count,
            recorded: recorded.leaf_count,
        });
    }
    replay_cursors.insert(account, end);
    let leaves = reduction.replayed.entry(account).or_default();
    if recorded.history_complete {
        for (leaf_index, key) in
            (write.previous_leaf_count..).zip(leaf_keys(write))
        {
            leaves.push(staged(
                account,
                leaf_index,
                write.handle,
                key,
                transaction_index,
            ));
        }
    }
    Ok(())
}

/// The key of each leaf `write` seals, in order: one per allowed key, then `None` for the
/// public-decrypt leaf.
fn leaf_keys(
    write: &EncryptedStoreWrite,
) -> impl Iterator<Item = Option<[u8; 32]>> + '_ {
    let keyed = write.allowed_keys.iter().map(|key| Some(*key));
    keyed.chain(write.make_public.then_some(None))
}

fn leaf_count_after(
    write: &EncryptedStoreWrite,
    leaf_count: u64,
) -> Result<u64, LeafReduceError> {
    u64::try_from(write.allowed_keys.len())
        .ok()
        .and_then(|count| count.checked_add(u64::from(write.make_public)))
        .and_then(|count| leaf_count.checked_add(count))
        .ok_or(LeafReduceError::LeafCountOverflow {
            encrypted_store: write.encrypted_store,
        })
}

fn apply_write(
    state: &mut EncryptedStoreHistory,
    leaves: &mut Vec<StagedLeaf>,
    write: &EncryptedStoreWrite,
    transaction_index: u64,
) -> Result<(), LeafReduceError> {
    let account = write.encrypted_store;
    if state.leaf_count != write.previous_leaf_count {
        return Err(LeafReduceError::PreviousLeafCountMismatch {
            encrypted_store: account,
            declared: write.previous_leaf_count,
            recorded: state.leaf_count,
        });
    }
    if state.history_complete
        && state.peaks.len() != state.leaf_count.count_ones() as usize
    {
        return Err(LeafReduceError::Mmr {
            encrypted_store: account,
            error: AclError::MmrInconsistent,
        });
    }
    if !state.history_complete {
        state.leaf_count = leaf_count_after(write, state.leaf_count)?;
        return Ok(());
    }
    for key in leaf_keys(write) {
        let leaf = staged(
            account,
            state.leaf_count,
            write.handle,
            key,
            transaction_index,
        );
        mmr_append(&mut state.peaks, &mut state.leaf_count, leaf.commitment)
            .map_err(|error| LeafReduceError::Mmr {
                encrypted_store: account,
                error,
            })?;
        leaves.push(leaf);
    }
    Ok(())
}

/// A historical-access leaf for `key`, or the public-decrypt leaf when `key` is `None`.
fn staged(
    account: [u8; 32],
    leaf_index: u64,
    handle: [u8; 32],
    key: Option<[u8; 32]>,
    transaction_index: u64,
) -> StagedLeaf {
    let (kind, commitment) = match key {
        Some(key) => (
            LeafKind::HistoricalAccess,
            historical_access_leaf_commitment(account, leaf_index, handle, key),
        ),
        None => (
            LeafKind::PublicDecrypt,
            public_decrypt_leaf_commitment(account, leaf_index, handle),
        ),
    };
    StagedLeaf {
        encrypted_store: account,
        leaf_index,
        commitment,
        kind,
        handle,
        key,
        transaction_index,
    }
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
pub async fn load_encrypted_store_histories(
    tx: &mut Transaction<'_>,
    accounts: &[[u8; 32]],
) -> Result<BTreeMap<[u8; 32], EncryptedStoreHistory>, SqlxError> {
    if accounts.is_empty() {
        return Ok(BTreeMap::new());
    }
    let keys: Vec<Vec<u8>> =
        accounts.iter().map(|account| account.to_vec()).collect();
    let rows = sqlx::query!(
        r#"
        SELECT encrypted_state, leaf_count, peaks, history_complete, last_slot
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
                EncryptedStoreHistory {
                    leaf_count: sql_u64(row.leaf_count, "leaf_count")?,
                    peaks: bytes32_vec(&row.peaks)?,
                    history_complete: row.history_complete,
                    last_slot: sql_u64(row.last_slot, "last_slot")?,
                },
            ))
        })
        .collect()
}

/// Persists a block's reduction: advanced state cursors upserted, leaves appended.
pub async fn store_block_leaves(
    tx: &mut Transaction<'_>,
    slot: u64,
    reduction: &BlockLeafReduction,
) -> Result<(), SqlxError> {
    for (account, state) in &reduction.states {
        let leaf_count = sql_i64(state.leaf_count, "leaf_count")?;
        let last_slot = sql_i64(state.last_slot, "last_slot")?;
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
            &leaf.encrypted_store[..],
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
    pub state: EncryptedStoreHistory,
    pub leaves: Vec<StagedLeaf>,
}

pub async fn load_recorded_leaves(
    pool: &sqlx::PgPool,
    account: [u8; 32],
) -> Result<Option<RecordedLeaves>, SqlxError> {
    let Some(row) = sqlx::query!(
        r#"
        SELECT leaf_count, peaks, history_complete, last_slot
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
    let state = EncryptedStoreHistory {
        leaf_count: sql_u64(row.leaf_count, "leaf_count")?,
        peaks: bytes32_vec(&row.peaks)?,
        history_complete: row.history_complete,
        last_slot: sql_u64(row.last_slot, "last_slot")?,
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
            leaf_row(
                &account,
                row.leaf_index,
                &row.commitment,
                row.leaf_kind,
                &row.handle,
                row.allowed_key.as_deref(),
                row.transaction_index,
            )
        })
        .collect::<Result<Vec<_>, SqlxError>>()?;
    Ok(Some(RecordedLeaves { state, leaves }))
}

/// The leaves `slot` recorded for `accounts`, grouped by account in leaf order. A
/// replayed block's [`BlockLeafReduction::replayed`] must equal this exactly.
pub async fn load_block_leaves(
    tx: &mut Transaction<'_>,
    slot: u64,
    accounts: impl IntoIterator<Item = [u8; 32]>,
) -> Result<BTreeMap<[u8; 32], Vec<StagedLeaf>>, SqlxError> {
    let mut recorded: BTreeMap<[u8; 32], Vec<StagedLeaf>> = accounts
        .into_iter()
        .map(|account| (account, Vec::new()))
        .collect();
    if recorded.is_empty() {
        return Ok(recorded);
    }
    let keys: Vec<Vec<u8>> =
        recorded.keys().map(|account| account.to_vec()).collect();
    let rows = sqlx::query!(
        r#"
        SELECT encrypted_state, leaf_index, commitment, leaf_kind, handle, allowed_key,
               transaction_index
        FROM solana_encrypted_state_leaves
        WHERE encrypted_state = ANY($1) AND block_slot = $2
        ORDER BY encrypted_state, leaf_index
        "#,
        &keys,
        sql_i64(slot, "block_slot")?,
    )
    .fetch_all(tx.as_mut())
    .await?;
    for row in rows {
        let leaf = leaf_row(
            &row.encrypted_state,
            row.leaf_index,
            &row.commitment,
            row.leaf_kind,
            &row.handle,
            row.allowed_key.as_deref(),
            row.transaction_index,
        )?;
        recorded.entry(leaf.encrypted_store).or_default().push(leaf);
    }
    Ok(recorded)
}

fn leaf_row(
    encrypted_store: &[u8],
    leaf_index: i64,
    commitment: &[u8],
    leaf_kind: i16,
    handle: &[u8],
    allowed_key: Option<&[u8]>,
    transaction_index: i64,
) -> Result<StagedLeaf, SqlxError> {
    Ok(StagedLeaf {
        encrypted_store: bytes32(encrypted_store)?,
        leaf_index: sql_u64(leaf_index, "leaf_index")?,
        commitment: bytes32(commitment)?,
        kind: LeafKind::from_i16(leaf_kind).ok_or_else(|| {
            SqlxError::Decode(format!("unknown leaf_kind {leaf_kind}").into())
        })?,
        handle: bytes32(handle)?,
        key: allowed_key.map(bytes32).transpose()?,
        transaction_index: sql_u64(transaction_index, "transaction_index")?,
    })
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
    ) -> EncryptedStoreWrite {
        EncryptedStoreWrite {
            encrypted_store: STATE,
            previous_leaf_count,
            handle,
            allowed_keys,
            make_public,
        }
    }

    fn transaction(
        sources: Vec<EncryptedStoreWrite>,
    ) -> TransactionStoreWrites {
        TransactionStoreWrites {
            transaction_index: 3,
            sources,
        }
    }

    #[test]
    fn appends_allows_then_public_and_advances_across_outputs() {
        let reduction = reduce_block_leaves(
            10,
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
            10,
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
            11,
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
            EncryptedStoreHistory {
                leaf_count: 4,
                peaks: vec![[1; 32]],
                history_complete: true,
                last_slot: 10,
            },
            EncryptedStoreHistory {
                leaf_count: 4,
                peaks: vec![],
                history_complete: false,
                last_slot: 10,
            },
        ] {
            let error = reduce_block_leaves(
                11,
                &[transaction(vec![write(3, [2; 32], vec![], false)])],
                BTreeMap::from([(STATE, history)]),
            )
            .unwrap_err();
            assert_eq!(
                error,
                LeafReduceError::PreviousLeafCountMismatch {
                    encrypted_store: STATE,
                    declared: 3,
                    recorded: 4,
                }
            );
        }
    }

    #[test]
    fn zero_leaf_output_still_checks_continuity() {
        let reduction = reduce_block_leaves(
            10,
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
            11,
            &[transaction(vec![write(0, [1; 32], vec![], false)])],
            BTreeMap::from([(
                STATE,
                EncryptedStoreHistory {
                    leaf_count: 0,
                    peaks: vec![[1; 32]],
                    history_complete: true,
                    last_slot: 10,
                },
            )]),
        )
        .unwrap_err();
        assert_eq!(
            error,
            LeafReduceError::Mmr {
                encrypted_store: STATE,
                error: AclError::MmrInconsistent,
            }
        );
    }

    /// A slot the record already holds is recomputed below the cursor, not appended:
    /// the state row is left alone and the leaves come back for comparison.
    #[test]
    fn replaying_a_recorded_slot_recomputes_its_leaves_without_advancing() {
        let block_10 = [transaction(vec![
            write(0, [1; 32], vec![OWNER], false),
            write(1, [2; 32], vec![OWNER], true),
        ])];
        let first =
            reduce_block_leaves(10, &block_10, BTreeMap::new()).unwrap();
        let second = reduce_block_leaves(
            11,
            &[transaction(vec![write(3, [3; 32], vec![OWNER], false)])],
            first.states.clone(),
        )
        .unwrap();

        let replay = reduce_block_leaves(10, &block_10, second.states).unwrap();
        assert!(replay.states.is_empty());
        assert!(replay.leaves.is_empty());
        assert_eq!(replay.replayed, BTreeMap::from([(STATE, first.leaves)]));
    }

    #[test]
    fn a_replay_must_fit_the_recorded_history() {
        let recorded = BTreeMap::from([(
            STATE,
            EncryptedStoreHistory {
                leaf_count: 3,
                peaks: vec![[1; 32], [2; 32]],
                history_complete: true,
                last_slot: 11,
            },
        )]);
        for writes in [
            // Past the recorded cursor.
            vec![write(2, [1; 32], vec![OWNER, OWNER], false)],
            // Not contiguous with the block's previous write on the account.
            vec![
                write(0, [1; 32], vec![OWNER], false),
                write(2, [2; 32], vec![OWNER], false),
            ],
        ] {
            assert!(matches!(
                reduce_block_leaves(
                    10,
                    &[transaction(writes)],
                    recorded.clone()
                ),
                Err(LeafReduceError::PreviousLeafCountMismatch { .. })
            ));
        }
    }
}
