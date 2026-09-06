//! The RFC 035 leaf record of Solana encrypted value accounts.
//!
//! Every write the host performs on an encrypted value account seals leaves: one
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

/// A `fhe_execute` persistent output, with its account resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedValueWrite {
    pub encrypted_value_account: [u8; 32],
    pub program: [u8; 32],
    pub encrypted_value_account_authority: [u8; 32],
    pub scope: [u8; 32],
    pub label: [u8; 32],
    /// The handle being replaced: `None` on create.
    pub previous_handle: Option<[u8; 32]>,
    pub handle: [u8; 32],
    pub allowed_keys: Vec<[u8; 32]>,
    pub make_public: bool,
}

/// One host instruction that seals leaves, in on-chain order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LeafSource {
    Write(EncryptedValueWrite),
    MakeHandlePublic {
        encrypted_value_account: [u8; 32],
        handle: [u8; 32],
    },
}

/// The leaf sources of one confirmed transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionLeafSources {
    pub transaction_index: u64,
    pub sources: Vec<LeafSource>,
}

/// The persisted state of one encrypted value account.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedValueState {
    pub program: [u8; 32],
    pub encrypted_value_account_authority: [u8; 32],
    pub scope: [u8; 32],
    pub label: [u8; 32],
    pub current_handle: [u8; 32],
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    /// False when the account was first seen through an update, so its earlier
    /// leaves were sealed before this record started: no leaf of it is stored,
    /// and no proof can be served for it.
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
    pub encrypted_value_account: [u8; 32],
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
    pub accounts: BTreeMap<[u8; 32], EncryptedValueState>,
    pub leaves: Vec<StagedLeaf>,
}

/// The confirmed chain performed a write this record cannot follow: the record
/// diverged from chain state, and continuing would seal wrong leaves.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LeafReduceError {
    #[error("create on encrypted value account {} already recorded", bs58::encode(encrypted_value_account).into_string())]
    CreateOnRecordedAccount { encrypted_value_account: [u8; 32] },
    #[error("encrypted value account {} replaced handle {} but the record holds {}", bs58::encode(encrypted_value_account).into_string(), hex::encode(previous_handle), hex::encode(recorded_handle))]
    PreviousHandleMismatch {
        encrypted_value_account: [u8; 32],
        previous_handle: [u8; 32],
        recorded_handle: [u8; 32],
    },
    #[error("encrypted value account {} made handle {} public but the record holds {}", bs58::encode(encrypted_value_account).into_string(), hex::encode(handle), hex::encode(recorded_handle))]
    PublicHandleMismatch {
        encrypted_value_account: [u8; 32],
        handle: [u8; 32],
        recorded_handle: [u8; 32],
    },
    #[error("MMR append failed on encrypted value account {}: {error:?}", bs58::encode(encrypted_value_account).into_string())]
    Mmr {
        encrypted_value_account: [u8; 32],
        error: AclError,
    },
}

/// Reduces one block's leaf sources over the accounts' prior states.
///
/// `existing` holds the recorded state of every account the block touches. An
/// account first seen through an update or a `make_handle_public` is recorded
/// with `history_complete = false`: its handle is tracked, no leaf is stored.
pub fn reduce_block_leaves(
    transactions: &[TransactionLeafSources],
    existing: BTreeMap<[u8; 32], EncryptedValueState>,
) -> Result<BlockLeafReduction, LeafReduceError> {
    let mut reduction = BlockLeafReduction {
        accounts: existing,
        leaves: Vec::new(),
    };
    for transaction in transactions {
        for source in &transaction.sources {
            match source {
                LeafSource::Write(write) => apply_write(
                    &mut reduction,
                    write,
                    transaction.transaction_index,
                )?,
                LeafSource::MakeHandlePublic {
                    encrypted_value_account,
                    handle,
                } => apply_make_public(
                    &mut reduction,
                    *encrypted_value_account,
                    *handle,
                    transaction.transaction_index,
                )?,
            }
        }
    }
    Ok(reduction)
}

fn apply_write(
    reduction: &mut BlockLeafReduction,
    write: &EncryptedValueWrite,
    transaction_index: u64,
) -> Result<(), LeafReduceError> {
    let account = write.encrypted_value_account;
    let state =
        match (reduction.accounts.get_mut(&account), write.previous_handle) {
            (Some(_), None) => {
                return Err(LeafReduceError::CreateOnRecordedAccount {
                    encrypted_value_account: account,
                })
            }
            (Some(state), Some(previous_handle)) => {
                if state.history_complete
                    && state.current_handle != previous_handle
                {
                    return Err(LeafReduceError::PreviousHandleMismatch {
                        encrypted_value_account: account,
                        previous_handle,
                        recorded_handle: state.current_handle,
                    });
                }
                state.current_handle = write.handle;
                state
            }
            (None, previous_handle) => reduction
                .accounts
                .entry(account)
                .or_insert(EncryptedValueState {
                    program: write.program,
                    encrypted_value_account_authority: write
                        .encrypted_value_account_authority,
                    scope: write.scope,
                    label: write.label,
                    current_handle: write.handle,
                    leaf_count: 0,
                    peaks: Vec::new(),
                    history_complete: previous_handle.is_none(),
                }),
        };
    if !state.history_complete {
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

fn apply_make_public(
    reduction: &mut BlockLeafReduction,
    account: [u8; 32],
    handle: [u8; 32],
    transaction_index: u64,
) -> Result<(), LeafReduceError> {
    // An account this record never saw created has an incomplete history; there
    // is nothing to store for it, and the host already checked the handle.
    let Some(state) = reduction.accounts.get_mut(&account) else {
        return Ok(());
    };
    if !state.history_complete {
        return Ok(());
    }
    if state.current_handle != handle {
        return Err(LeafReduceError::PublicHandleMismatch {
            encrypted_value_account: account,
            handle,
            recorded_handle: state.current_handle,
        });
    }
    append_leaf(
        state,
        &mut reduction.leaves,
        account,
        LeafKind::PublicDecrypt,
        handle,
        None,
        transaction_index,
    )
}

fn append_leaf(
    state: &mut EncryptedValueState,
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
            encrypted_value_account: account,
            error,
        },
    )?;
    leaves.push(StagedLeaf {
        encrypted_value_account: account,
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

/// Locks and loads the recorded state of `accounts`; accounts without a row are
/// absent from the result.
pub async fn load_encrypted_value_states(
    tx: &mut Transaction<'_>,
    accounts: &[[u8; 32]],
) -> Result<BTreeMap<[u8; 32], EncryptedValueState>, SqlxError> {
    if accounts.is_empty() {
        return Ok(BTreeMap::new());
    }
    let keys: Vec<Vec<u8>> =
        accounts.iter().map(|account| account.to_vec()).collect();
    let rows = sqlx::query!(
        r#"
        SELECT encrypted_value_account, program, encrypted_value_account_authority,
               scope, label, current_handle, leaf_count, peaks, history_complete
        FROM solana_encrypted_value_accounts
        WHERE encrypted_value_account = ANY($1)
        FOR UPDATE
        "#,
        &keys
    )
    .fetch_all(tx.as_mut())
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok((
                bytes32(&row.encrypted_value_account)?,
                EncryptedValueState {
                    program: bytes32(&row.program)?,
                    encrypted_value_account_authority: bytes32(
                        &row.encrypted_value_account_authority,
                    )?,
                    scope: bytes32(&row.scope)?,
                    label: bytes32(&row.label)?,
                    current_handle: bytes32(&row.current_handle)?,
                    leaf_count: row.leaf_count as u64,
                    peaks: bytes32_vec(&row.peaks)?,
                    history_complete: row.history_complete,
                },
            ))
        })
        .collect()
}

/// Persists a block's reduction: account states upserted, leaves appended.
pub async fn store_block_leaves(
    tx: &mut Transaction<'_>,
    slot: u64,
    reduction: &BlockLeafReduction,
) -> Result<(), SqlxError> {
    for (account, state) in &reduction.accounts {
        let peaks: Vec<Vec<u8>> =
            state.peaks.iter().map(|peak| peak.to_vec()).collect();
        sqlx::query!(
            r#"
            INSERT INTO solana_encrypted_value_accounts
                (encrypted_value_account, program, encrypted_value_account_authority,
                 scope, label, current_handle, leaf_count, peaks, history_complete, last_slot)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (encrypted_value_account) DO UPDATE SET
                current_handle = EXCLUDED.current_handle,
                leaf_count = EXCLUDED.leaf_count,
                peaks = EXCLUDED.peaks,
                last_slot = EXCLUDED.last_slot
            "#,
            &account[..],
            &state.program[..],
            &state.encrypted_value_account_authority[..],
            &state.scope[..],
            &state.label[..],
            &state.current_handle[..],
            state.leaf_count as i64,
            &peaks,
            state.history_complete,
            slot as i64,
        )
        .execute(tx.as_mut())
        .await?;
    }
    for leaf in &reduction.leaves {
        sqlx::query!(
            r#"
            INSERT INTO solana_encrypted_value_leaves
                (encrypted_value_account, leaf_index, commitment, leaf_kind, handle,
                 allowed_key, block_slot, transaction_index)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            &leaf.encrypted_value_account[..],
            leaf.leaf_index as i64,
            &leaf.commitment[..],
            leaf.kind as i16,
            &leaf.handle[..],
            leaf.key.as_ref().map(|key| key.to_vec()),
            slot as i64,
            leaf.transaction_index as i64,
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
        checkpoint.slot as i64,
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
            slot: row.slot as u64,
            block_hash: bytes32(&row.block_hash)?,
        })
    })
    .transpose()
}

/// One account's recorded leaves as a proof reader needs them: the state row and
/// the ordered leaves below its `leaf_count`. Leaves committed after the state
/// row was read are cut off by that bound, so the two always agree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedLeaves {
    pub state: EncryptedValueState,
    pub leaves: Vec<StagedLeaf>,
}

pub async fn load_recorded_leaves(
    pool: &sqlx::PgPool,
    account: [u8; 32],
) -> Result<Option<RecordedLeaves>, SqlxError> {
    let Some(row) = sqlx::query!(
        r#"
        SELECT program, encrypted_value_account_authority, scope, label,
               current_handle, leaf_count, peaks, history_complete
        FROM solana_encrypted_value_accounts
        WHERE encrypted_value_account = $1
        "#,
        &account[..]
    )
    .fetch_optional(pool)
    .await?
    else {
        return Ok(None);
    };
    let state = EncryptedValueState {
        program: bytes32(&row.program)?,
        encrypted_value_account_authority: bytes32(
            &row.encrypted_value_account_authority,
        )?,
        scope: bytes32(&row.scope)?,
        label: bytes32(&row.label)?,
        current_handle: bytes32(&row.current_handle)?,
        leaf_count: row.leaf_count as u64,
        peaks: bytes32_vec(&row.peaks)?,
        history_complete: row.history_complete,
    };
    let rows = sqlx::query!(
        r#"
        SELECT leaf_index, commitment, leaf_kind, handle, allowed_key, transaction_index
        FROM solana_encrypted_value_leaves
        WHERE encrypted_value_account = $1 AND leaf_index < $2
        ORDER BY leaf_index
        "#,
        &account[..],
        row.leaf_count,
    )
    .fetch_all(pool)
    .await?;
    let leaves = rows
        .into_iter()
        .map(|row| {
            Ok(StagedLeaf {
                encrypted_value_account: account,
                leaf_index: row.leaf_index as u64,
                commitment: bytes32(&row.commitment)?,
                kind: LeafKind::from_i16(row.leaf_kind).ok_or_else(|| {
                    SqlxError::Decode(
                        format!("unknown leaf_kind {}", row.leaf_kind).into(),
                    )
                })?,
                handle: bytes32(&row.handle)?,
                key: row.allowed_key.as_deref().map(bytes32).transpose()?,
                transaction_index: row.transaction_index as u64,
            })
        })
        .collect::<Result<Vec<_>, SqlxError>>()?;
    Ok(Some(RecordedLeaves { state, leaves }))
}

#[cfg(test)]
#[path = "../../../../../solana/test-fixtures/leaves/leaf_vectors.rs"]
mod leaf_vectors;

#[cfg(test)]
mod tests {
    use super::*;
    use zama_solana_acl::{
        mmr_peaks_from_leaves, reconstruct, EncryptedValueAccountEvent,
    };

    const ACCOUNT: [u8; 32] = [0xAC; 32];
    const PROGRAM: [u8; 32] = [0x01; 32];
    const AUTHORITY: [u8; 32] = [0x02; 32];
    const SCOPE: [u8; 32] = [0x03; 32];
    const LABEL: [u8; 32] = [0x04; 32];

    fn write(
        previous_handle: Option<[u8; 32]>,
        handle: [u8; 32],
        allowed_keys: Vec<[u8; 32]>,
        make_public: bool,
    ) -> LeafSource {
        LeafSource::Write(EncryptedValueWrite {
            encrypted_value_account: ACCOUNT,
            program: PROGRAM,
            encrypted_value_account_authority: AUTHORITY,
            scope: SCOPE,
            label: LABEL,
            previous_handle,
            handle,
            allowed_keys,
            make_public,
        })
    }

    fn transaction(
        index: u64,
        sources: Vec<LeafSource>,
    ) -> TransactionLeafSources {
        TransactionLeafSources {
            transaction_index: index,
            sources,
        }
    }

    fn events_of(leaves: &[StagedLeaf]) -> Vec<EncryptedValueAccountEvent> {
        leaves
            .iter()
            .map(|leaf| match leaf.key {
                Some(key) => EncryptedValueAccountEvent::Allowed {
                    handle: leaf.handle,
                    key,
                },
                None => EncryptedValueAccountEvent::MarkedPublic {
                    handle: leaf.handle,
                },
            })
            .collect()
    }

    #[test]
    fn create_then_update_seal_leaves_in_host_order() {
        let reduction = reduce_block_leaves(
            &[
                transaction(
                    0,
                    vec![write(
                        None,
                        [0x10; 32],
                        vec![[0xA1; 32], [0xA2; 32]],
                        false,
                    )],
                ),
                transaction(
                    3,
                    vec![
                        write(
                            Some([0x10; 32]),
                            [0x11; 32],
                            vec![[0xA1; 32]],
                            true,
                        ),
                        LeafSource::MakeHandlePublic {
                            encrypted_value_account: ACCOUNT,
                            handle: [0x11; 32],
                        },
                    ],
                ),
            ],
            BTreeMap::new(),
        )
        .expect("reduce");

        let state = &reduction.accounts[&ACCOUNT];
        assert!(state.history_complete);
        assert_eq!(state.current_handle, [0x11; 32]);
        assert_eq!(state.leaf_count, 5);
        assert_eq!(
            reduction
                .leaves
                .iter()
                .map(|leaf| (
                    leaf.leaf_index,
                    leaf.kind,
                    leaf.handle,
                    leaf.key,
                    leaf.transaction_index
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    0,
                    LeafKind::HistoricalAccess,
                    [0x10; 32],
                    Some([0xA1; 32]),
                    0
                ),
                (
                    1,
                    LeafKind::HistoricalAccess,
                    [0x10; 32],
                    Some([0xA2; 32]),
                    0
                ),
                (
                    2,
                    LeafKind::HistoricalAccess,
                    [0x11; 32],
                    Some([0xA1; 32]),
                    3
                ),
                (3, LeafKind::PublicDecrypt, [0x11; 32], None, 3),
                (4, LeafKind::PublicDecrypt, [0x11; 32], None, 3),
            ]
        );
        // Byte-identical to the shared crate's reconstruction from events.
        let expected = reconstruct(ACCOUNT, &events_of(&reduction.leaves));
        assert_eq!(
            reduction
                .leaves
                .iter()
                .map(|leaf| leaf.commitment)
                .collect::<Vec<_>>(),
            expected.leaves
        );
        assert_eq!(state.peaks, expected.peaks);
    }

    #[test]
    fn prior_state_continues_the_mmr_across_blocks() {
        let first = reduce_block_leaves(
            &[transaction(
                0,
                vec![write(None, [0x10; 32], vec![[0xA1; 32]], false)],
            )],
            BTreeMap::new(),
        )
        .expect("first block");
        let second = reduce_block_leaves(
            &[transaction(
                1,
                vec![write(
                    Some([0x10; 32]),
                    [0x11; 32],
                    vec![[0xA1; 32]],
                    true,
                )],
            )],
            first.accounts.clone(),
        )
        .expect("second block");
        assert_eq!(second.leaves.first().map(|leaf| leaf.leaf_index), Some(1));
        let all = first
            .leaves
            .iter()
            .chain(&second.leaves)
            .map(|leaf| leaf.commitment)
            .collect::<Vec<_>>();
        assert_eq!(
            second.accounts[&ACCOUNT].peaks,
            mmr_peaks_from_leaves(&all)
        );
        assert_eq!(second.accounts[&ACCOUNT].leaf_count, 3);
    }

    #[test]
    fn update_of_an_unseen_account_is_tracked_without_leaves() {
        let reduction = reduce_block_leaves(
            &[transaction(
                0,
                vec![
                    write(Some([0x10; 32]), [0x11; 32], vec![[0xA1; 32]], true),
                    LeafSource::MakeHandlePublic {
                        encrypted_value_account: ACCOUNT,
                        handle: [0x11; 32],
                    },
                ],
            )],
            BTreeMap::new(),
        )
        .expect("reduce");
        let state = &reduction.accounts[&ACCOUNT];
        assert!(!state.history_complete);
        assert_eq!(state.current_handle, [0x11; 32]);
        assert_eq!(state.leaf_count, 0);
        assert!(reduction.leaves.is_empty());
    }

    #[test]
    fn make_public_on_an_unseen_account_records_nothing() {
        let reduction = reduce_block_leaves(
            &[transaction(
                0,
                vec![LeafSource::MakeHandlePublic {
                    encrypted_value_account: ACCOUNT,
                    handle: [0x11; 32],
                }],
            )],
            BTreeMap::new(),
        )
        .expect("reduce");
        assert_eq!(reduction, BlockLeafReduction::default());
    }

    #[test]
    fn divergence_from_chain_fails_closed() {
        let created = reduce_block_leaves(
            &[transaction(0, vec![write(None, [0x10; 32], vec![], false)])],
            BTreeMap::new(),
        )
        .expect("create");

        let stale_update = reduce_block_leaves(
            &[transaction(
                0,
                vec![write(Some([0x99; 32]), [0x11; 32], vec![], false)],
            )],
            created.accounts.clone(),
        );
        assert_eq!(
            stale_update,
            Err(LeafReduceError::PreviousHandleMismatch {
                encrypted_value_account: ACCOUNT,
                previous_handle: [0x99; 32],
                recorded_handle: [0x10; 32],
            })
        );

        let wrong_public = reduce_block_leaves(
            &[transaction(
                0,
                vec![LeafSource::MakeHandlePublic {
                    encrypted_value_account: ACCOUNT,
                    handle: [0x99; 32],
                }],
            )],
            created.accounts.clone(),
        );
        assert_eq!(
            wrong_public,
            Err(LeafReduceError::PublicHandleMismatch {
                encrypted_value_account: ACCOUNT,
                handle: [0x99; 32],
                recorded_handle: [0x10; 32],
            })
        );

        let recreate = reduce_block_leaves(
            &[transaction(0, vec![write(None, [0x12; 32], vec![], false)])],
            created.accounts,
        );
        assert_eq!(
            recreate,
            Err(LeafReduceError::CreateOnRecordedAccount {
                encrypted_value_account: ACCOUNT,
            })
        );
    }

    fn hex32(value: &str) -> [u8; 32] {
        hex::decode(value)
            .expect("hex")
            .try_into()
            .expect("32 bytes")
    }

    /// The normative vectors are event lists; this folds them back into the host
    /// writes that would have sealed them and checks the reduce rule reproduces
    /// every leaf, the peaks and the leaf count byte for byte.
    #[test]
    fn replays_the_normative_leaf_vectors() {
        let file: leaf_vectors::LeafVectorFile = serde_json::from_str(
            &std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../solana/test-fixtures/leaves/leaves_v1.json"
            ))
            .expect("read vectors"),
        )
        .expect("parse vectors");
        assert_eq!(file.schema, leaf_vectors::LEAF_VECTOR_SCHEMA);

        for vector in &file.vectors {
            let account = hex32(&vector.encrypted_value_account);
            let mut sources: Vec<LeafSource> = Vec::new();
            let mut current: Option<[u8; 32]> = None;
            for event in &vector.events {
                match event {
                    leaf_vectors::LeafEvent::Allowed { handle, key } => {
                        let handle = hex32(handle);
                        let key = hex32(key);
                        match sources.last_mut() {
                            Some(LeafSource::Write(write))
                                if write.handle == handle
                                    && !write.make_public =>
                            {
                                write.allowed_keys.push(key)
                            }
                            _ => {
                                sources.push(LeafSource::Write(
                                    EncryptedValueWrite {
                                        encrypted_value_account: account,
                                        program: PROGRAM,
                                        encrypted_value_account_authority:
                                            AUTHORITY,
                                        scope: SCOPE,
                                        label: LABEL,
                                        previous_handle: current,
                                        handle,
                                        allowed_keys: vec![key],
                                        make_public: false,
                                    },
                                ));
                                current = Some(handle);
                            }
                        }
                    }
                    leaf_vectors::LeafEvent::MarkedPublic { handle } => {
                        let handle = hex32(handle);
                        match sources.last_mut() {
                            Some(LeafSource::Write(write))
                                if write.handle == handle
                                    && !write.make_public =>
                            {
                                write.make_public = true
                            }
                            _ => sources.push(LeafSource::MakeHandlePublic {
                                encrypted_value_account: account,
                                handle,
                            }),
                        }
                    }
                }
            }
            let reduction = reduce_block_leaves(
                &[transaction(0, sources)],
                BTreeMap::new(),
            )
            .unwrap_or_else(|error| panic!("{}: {error}", vector.id));
            let state = &reduction.accounts[&account];
            assert_eq!(
                reduction
                    .leaves
                    .iter()
                    .map(|leaf| hex::encode(leaf.commitment))
                    .collect::<Vec<_>>(),
                vector.leaves,
                "{}: leaves",
                vector.id
            );
            assert_eq!(
                state.leaf_count.to_string(),
                vector.leaf_count,
                "{}: leaf_count",
                vector.id
            );
            assert_eq!(
                state.peaks.iter().map(hex::encode).collect::<Vec<_>>(),
                vector.peaks,
                "{}: peaks",
                vector.id
            );
        }
    }
}
