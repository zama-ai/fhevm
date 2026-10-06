//! The leaf record of Solana encrypted stores, as the Solana access control RFC defines it.
//!
//! Every store write the host accepts may seal leaves: one historical-access leaf per allowed
//! key on the handle it installs, then one public-decrypt leaf when the handle is made public.
//! The indexer recomputes those leaves from the finalized instruction stream, from a block
//! before any store existed, so every store's record starts at leaf zero.
//!
//! The reduce rule ([`reduce_block_leaves`]) is a pure function over one block; the SQL steps
//! around it load the touched stores, persist the reduction, and move the checkpoint, all in
//! one transaction per block.
//!
//! The indexer skips the checkpoint block when its hash matches. An older slot or a
//! conflicting hash stops it. A wrong record is rebuilt from the start slot or restored from
//! a dump; moving the checkpoint back does not rewind the store cursors.
//!
//! Each append also records the MMR nodes it completes, so a proof reads its path by
//! position ([`load_proof`]) instead of rebuilding the mountain from every leaf. Node
//! `(height, index)` covers leaves `[index << height, (index + 1) << height)`: mountains
//! are aligned to their size, so a node's position never depends on the leaf count.
//! Height-0 path entries are leaf rows; only nodes of height 1 and above are stored. The same
//! positions give the record's peaks at any leaf count it holds ([`load_peaks`]), which the
//! indexer's store check compares with the chain's.

use std::collections::{BTreeMap, HashMap, HashSet};

use sqlx::Error as SqlxError;
use zama_solana_acl::{
    historical_access_leaf_commitment, mmr_append, mmr_leaf_node, mmr_node,
    public_decrypt_leaf_commitment, AclError, MmrProof,
};

use solana_host_follower::host::EncryptedStoreWrite;
use solana_host_follower::BlockCheckpoint;

type Transaction<'c> = sqlx::Transaction<'c, sqlx::Postgres>;

/// The leaf sources of one finalized transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionStoreWrites {
    pub transaction_index: u64,
    pub sources: Vec<EncryptedStoreWrite>,
}

/// The persisted MMR cursor of one encrypted store.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EncryptedStoreCursor {
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
}

/// A recorded store as the proof server reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServedStore {
    pub cursor: EncryptedStoreCursor,
    /// The store check found the record disagrees with the chain.
    pub quarantined: bool,
}

/// Persisted as the `leaf_kind` column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeafKind {
    HistoricalAccess = 0,
    PublicDecrypt = 1,
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

/// An MMR node of height 1 or more, over leaves `[index << height, (index + 1) << height)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedNode {
    pub encrypted_store: [u8; 32],
    pub height: u8,
    pub index: u64,
    pub node: [u8; 32],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BlockLeafReduction {
    /// Every store the block wrote, with its cursor after the block.
    pub stores: BTreeMap<[u8; 32], EncryptedStoreCursor>,
    pub leaves: Vec<StagedLeaf>,
    /// The nodes the block's appends completed.
    pub nodes: Vec<StagedNode>,
}

/// The finalized chain performed a write this record cannot follow: the record
/// diverged from chain state, and continuing would seal wrong leaves.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LeafReduceError {
    /// The record first sees a store that already holds leaves: the indexer started after the
    /// store was created, and the earlier leaves can never be recovered from later blocks.
    #[error("encrypted store {} already held {previous_leaf_count} leaves when the record first saw it: rebuild the record on an empty database with a --start-slot before the store was created", bs58::encode(encrypted_store).into_string())]
    UnrecordedHistory {
        encrypted_store: [u8; 32],
        previous_leaf_count: u64,
    },
    #[error("encrypted store {} declared previous leaf count {declared}, record holds {recorded}", bs58::encode(encrypted_store).into_string())]
    PreviousLeafCountMismatch {
        encrypted_store: [u8; 32],
        declared: u64,
        recorded: u64,
    },
    #[error("MMR append failed on encrypted store {}: {error:?}", bs58::encode(encrypted_store).into_string())]
    Mmr {
        encrypted_store: [u8; 32],
        error: AclError,
    },
}

/// Reduces the leaf sources of a block the record has not applied over the stores' recorded
/// cursors.
///
/// `existing` holds the recorded cursor of each store the block writes, and of no other. A store
/// first seen above leaf zero fails with [`LeafReduceError::UnrecordedHistory`].
pub fn reduce_block_leaves(
    transactions: &[TransactionStoreWrites],
    mut existing: BTreeMap<[u8; 32], EncryptedStoreCursor>,
) -> Result<BlockLeafReduction, LeafReduceError> {
    let mut reduction = BlockLeafReduction::default();
    for transaction in transactions {
        for write in &transaction.sources {
            let store = write.encrypted_store;
            if !existing.contains_key(&store) && write.previous_leaf_count != 0
            {
                return Err(LeafReduceError::UnrecordedHistory {
                    encrypted_store: store,
                    previous_leaf_count: write.previous_leaf_count,
                });
            }
            apply_write(
                existing.entry(store).or_default(),
                &mut reduction,
                write,
                transaction.transaction_index,
            )?;
        }
    }
    reduction.stores = existing;
    Ok(reduction)
}

/// The key of each leaf `write` seals, in order: one per allowed key, then `None` for the
/// public-decrypt leaf.
fn leaf_keys(
    write: &EncryptedStoreWrite,
) -> impl Iterator<Item = Option<[u8; 32]>> + '_ {
    let keyed = write.allowed_keys.iter().map(|key| Some(*key));
    keyed.chain(write.make_public.then_some(None))
}

fn apply_write(
    state: &mut EncryptedStoreCursor,
    reduction: &mut BlockLeafReduction,
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
    if state.peaks.len() != state.leaf_count.count_ones() as usize {
        return Err(LeafReduceError::Mmr {
            encrypted_store: account,
            error: AclError::MmrInconsistent,
        });
    }
    for key in leaf_keys(write) {
        let leaf = staged(
            account,
            state.leaf_count,
            write.handle,
            key,
            transaction_index,
        );
        reduction.nodes.extend(completed_nodes(
            account,
            &state.peaks,
            state.leaf_count,
            &leaf.commitment,
        ));
        mmr_append(&mut state.peaks, &mut state.leaf_count, leaf.commitment)
            .map_err(|error| LeafReduceError::Mmr {
                encrypted_store: account,
                error,
            })?;
        reduction.leaves.push(leaf);
    }
    Ok(())
}

/// The nodes that appending `commitment` as leaf `leaf_index` completes, lowest first. They
/// are the merges `mmr_append` performs: the new leaf node absorbs one peak, newest first,
/// per trailing one bit of `leaf_index`.
fn completed_nodes(
    encrypted_store: [u8; 32],
    peaks: &[[u8; 32]],
    leaf_index: u64,
    commitment: &[u8; 32],
) -> Vec<StagedNode> {
    let mut node = mmr_leaf_node(commitment);
    let merges = leaf_index.trailing_ones() as usize;
    peaks
        .iter()
        .rev()
        .take(merges)
        .zip(1u8..)
        .map(|(left, height)| {
            node = mmr_node(left, &node);
            StagedNode {
                encrypted_store,
                height,
                index: leaf_index >> height,
                node,
            }
        })
        .collect()
}

/// The positions of leaf `leaf_index`'s path siblings among `leaf_count` leaves, from
/// height 0 up to below its mountain's peak. `(0, i)` is leaf `i`; above it, `(k, j)` is
/// node `j` of height `k`. `None` when the leaf is not below `leaf_count`.
fn proof_path(leaf_index: u64, leaf_count: u64) -> Option<Vec<(u8, u64)>> {
    if leaf_index >= leaf_count {
        return None;
    }
    // The leaf's mountain is the one of the highest bit where its index and the count
    // differ: every larger mountain ends at or before the leaf, since each is aligned.
    let height =
        (u64::BITS - 1 - (leaf_index ^ leaf_count).leading_zeros()) as u8;
    Some((0..height).map(|k| (k, (leaf_index >> k) ^ 1)).collect())
}

/// The positions of the peaks of the first `leaf_count` leaves, oldest mountain first, in
/// [`proof_path`]'s notation. Each set bit of `leaf_count` is one mountain, aligned to its size.
fn peak_positions(leaf_count: u64) -> Vec<(u8, u64)> {
    let mut offset = 0;
    (0..u64::BITS as u8)
        .rev()
        .filter(|height| leaf_count & (1 << height) != 0)
        .map(|height| {
            let position = (height, offset >> height);
            offset += 1 << height;
            position
        })
        .collect()
}

/// The commitment of a historical-access leaf for `key`, or of the public-decrypt leaf
/// when `key` is `None`.
pub fn leaf_commitment(
    account: [u8; 32],
    leaf_index: u64,
    handle: [u8; 32],
    key: Option<[u8; 32]>,
) -> [u8; 32] {
    match key {
        Some(key) => {
            historical_access_leaf_commitment(account, leaf_index, handle, key)
        }
        None => public_decrypt_leaf_commitment(account, leaf_index, handle),
    }
}

/// A historical-access leaf for `key`, or the public-decrypt leaf when `key` is `None`.
fn staged(
    account: [u8; 32],
    leaf_index: u64,
    handle: [u8; 32],
    key: Option<[u8; 32]>,
    transaction_index: u64,
) -> StagedLeaf {
    let kind = match key {
        Some(_) => LeafKind::HistoricalAccess,
        None => LeafKind::PublicDecrypt,
    };
    StagedLeaf {
        encrypted_store: account,
        leaf_index,
        commitment: leaf_commitment(account, leaf_index, handle, key),
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

/// Locks and loads the recorded cursor of `stores`; stores without a row are absent from the
/// result.
pub async fn load_store_cursors(
    tx: &mut Transaction<'_>,
    stores: &[[u8; 32]],
) -> Result<BTreeMap<[u8; 32], EncryptedStoreCursor>, SqlxError> {
    if stores.is_empty() {
        return Ok(BTreeMap::new());
    }
    let keys: Vec<Vec<u8>> =
        stores.iter().map(|store| store.to_vec()).collect();
    let rows = sqlx::query!(
        r#"
        SELECT encrypted_store, leaf_count, peaks
        FROM encrypted_stores
        WHERE encrypted_store = ANY($1)
        FOR UPDATE
        "#,
        &keys,
    )
    .fetch_all(tx.as_mut())
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok((
                bytes32(&row.encrypted_store)?,
                EncryptedStoreCursor {
                    leaf_count: sql_u64(row.leaf_count, "leaf_count")?,
                    peaks: bytes32_vec(&row.peaks)?,
                },
            ))
        })
        .collect()
}

/// Persists a block's reduction: advanced store cursors upserted, leaves and nodes appended.
pub async fn store_block_leaves(
    tx: &mut Transaction<'_>,
    slot: u64,
    reduction: &BlockLeafReduction,
) -> Result<(), SqlxError> {
    for (store, cursor) in &reduction.stores {
        let leaf_count = sql_i64(cursor.leaf_count, "leaf_count")?;
        let peaks: Vec<Vec<u8>> =
            cursor.peaks.iter().map(|peak| peak.to_vec()).collect();
        sqlx::query!(
            r#"
            INSERT INTO encrypted_stores (encrypted_store, leaf_count, peaks)
            VALUES ($1, $2, $3)
            ON CONFLICT (encrypted_store) DO UPDATE SET
                leaf_count = EXCLUDED.leaf_count,
                peaks = EXCLUDED.peaks
            "#,
            &store[..],
            leaf_count,
            &peaks,
        )
        .execute(tx.as_mut())
        .await?;
    }
    let leaves = &reduction.leaves;
    if !leaves.is_empty() {
        sqlx::query!(
            r#"
            INSERT INTO leaves
                (encrypted_store, leaf_index, commitment, leaf_kind, handle,
                 allowed_key, block_slot, transaction_index)
            SELECT encrypted_store, leaf_index, commitment, leaf_kind, handle,
                   allowed_key, $7, transaction_index
            FROM UNNEST($1::BYTEA[], $2::BIGINT[], $3::BYTEA[], $4::SMALLINT[],
                        $5::BYTEA[], $6::BYTEA[], $8::BIGINT[])
                AS leaf(encrypted_store, leaf_index, commitment, leaf_kind, handle,
                        allowed_key, transaction_index)
            "#,
            &leaves
                .iter()
                .map(|leaf| leaf.encrypted_store.to_vec())
                .collect::<Vec<_>>(),
            &leaves
                .iter()
                .map(|leaf| sql_i64(leaf.leaf_index, "leaf_index"))
                .collect::<Result<Vec<_>, _>>()?,
            &leaves
                .iter()
                .map(|leaf| leaf.commitment.to_vec())
                .collect::<Vec<_>>(),
            &leaves
                .iter()
                .map(|leaf| leaf.kind as i16)
                .collect::<Vec<_>>(),
            &leaves
                .iter()
                .map(|leaf| leaf.handle.to_vec())
                .collect::<Vec<_>>(),
            &leaves
                .iter()
                .map(|leaf| leaf.key.map(|key| key.to_vec()))
                .collect::<Vec<_>>() as &[Option<Vec<u8>>],
            sql_i64(slot, "block_slot")?,
            &leaves
                .iter()
                .map(|leaf| sql_i64(
                    leaf.transaction_index,
                    "transaction_index"
                ))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .execute(tx.as_mut())
        .await?;
    }
    let nodes = &reduction.nodes;
    if !nodes.is_empty() {
        sqlx::query!(
            r#"
            INSERT INTO nodes (encrypted_store, height, node_index, node)
            SELECT * FROM UNNEST($1::BYTEA[], $2::SMALLINT[], $3::BIGINT[], $4::BYTEA[])
            "#,
            &nodes
                .iter()
                .map(|node| node.encrypted_store.to_vec())
                .collect::<Vec<_>>(),
            &nodes
                .iter()
                .map(|node| i16::from(node.height))
                .collect::<Vec<_>>(),
            &nodes
                .iter()
                .map(|node| sql_i64(node.index, "node_index"))
                .collect::<Result<Vec<_>, _>>()?,
            &nodes.iter().map(|node| node.node.to_vec()).collect::<Vec<_>>(),
        )
        .execute(tx.as_mut())
        .await?;
    }
    Ok(())
}

/// Moves the record's checkpoint to `checkpoint`, inside the transaction that applied its block,
/// so a restart resumes exactly after the recorded work.
pub async fn store_checkpoint(
    tx: &mut Transaction<'_>,
    checkpoint: &BlockCheckpoint,
) -> Result<(), SqlxError> {
    sqlx::query!(
        r#"
        INSERT INTO checkpoint (singleton, slot, block_hash)
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

/// Loads and locks the checkpoint until the executor's transaction ends.
pub async fn load_checkpoint<'e>(
    executor: impl sqlx::PgExecutor<'e>,
) -> Result<Option<BlockCheckpoint>, SqlxError> {
    let row = sqlx::query!(
        "SELECT slot, block_hash FROM checkpoint WHERE singleton = 1 FOR UPDATE"
    )
    .fetch_optional(executor)
    .await?;
    row.map(|row| {
        Ok(BlockCheckpoint {
            slot: sql_u64(row.slot, "checkpoint slot")?,
            block_hash: bytes32(&row.block_hash)?,
        })
    })
    .transpose()
}

/// The recorded cursor of `store` and whether it is quarantined, read outside
/// ingestion. Leaves and nodes are immutable once written and the cursor only
/// grows, so reads bounded by this `leaf_count` agree with it while later blocks
/// commit.
pub async fn load_served_store(
    pool: &sqlx::PgPool,
    store: [u8; 32],
) -> Result<Option<ServedStore>, SqlxError> {
    let row = sqlx::query!(
        r#"
        SELECT
            leaf_count,
            peaks,
            EXISTS (
                SELECT 1 FROM quarantined_stores AS q
                WHERE q.encrypted_store = s.encrypted_store
            ) AS "quarantined!"
        FROM encrypted_stores AS s
        WHERE encrypted_store = $1
        "#,
        &store[..],
    )
    .fetch_optional(pool)
    .await?;
    row.map(|row| {
        Ok(ServedStore {
            cursor: EncryptedStoreCursor {
                leaf_count: sql_u64(row.leaf_count, "leaf_count")?,
                peaks: bytes32_vec(&row.peaks)?,
            },
            quarantined: row.quarantined,
        })
    })
    .transpose()
}

/// Up to `limit` recorded stores after `after`, in address order: one page of the store check.
pub async fn load_store_page(
    pool: &sqlx::PgPool,
    after: Option<[u8; 32]>,
    limit: u16,
) -> Result<Vec<[u8; 32]>, SqlxError> {
    let rows = sqlx::query_scalar!(
        r#"
        SELECT encrypted_store
        FROM encrypted_stores
        WHERE $1::BYTEA IS NULL OR encrypted_store > $1
        ORDER BY encrypted_store
        LIMIT $2
        "#,
        after.as_ref().map(|store| &store[..]),
        i64::from(limit),
    )
    .fetch_all(pool)
    .await?;
    rows.iter().map(|store| bytes32(store)).collect()
}

/// The recorded leaf count of each of `stores` the record holds.
pub async fn load_leaf_counts(
    pool: &sqlx::PgPool,
    stores: &[[u8; 32]],
) -> Result<HashMap<[u8; 32], u64>, SqlxError> {
    let stores: Vec<&[u8]> = stores.iter().map(|store| &store[..]).collect();
    let rows = sqlx::query!(
        r#"
        SELECT encrypted_store, leaf_count
        FROM encrypted_stores
        WHERE encrypted_store = ANY($1::BYTEA[])
        "#,
        &stores as &[&[u8]],
    )
    .fetch_all(pool)
    .await?;
    rows.iter()
        .map(|row| {
            Ok((
                bytes32(&row.encrypted_store)?,
                sql_u64(row.leaf_count, "leaf_count")?,
            ))
        })
        .collect()
}

/// The stores the store check found disagreeing with the chain.
pub async fn load_quarantined_stores(
    pool: &sqlx::PgPool,
) -> Result<HashSet<[u8; 32]>, SqlxError> {
    let rows =
        sqlx::query_scalar!("SELECT encrypted_store FROM quarantined_stores")
            .fetch_all(pool)
            .await?;
    rows.iter().map(|store| bytes32(store)).collect()
}

/// Quarantines `store`, which disagrees with the chain.
pub async fn quarantine_store(
    pool: &sqlx::PgPool,
    store: [u8; 32],
) -> Result<(), SqlxError> {
    sqlx::query!(
        r#"
        INSERT INTO quarantined_stores (encrypted_store) VALUES ($1)
        ON CONFLICT (encrypted_store) DO NOTHING
        "#,
        &store[..],
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Lifts the quarantine of `store`, which matches the chain again or was closed.
pub async fn lift_quarantine(
    pool: &sqlx::PgPool,
    store: [u8; 32],
) -> Result<(), SqlxError> {
    sqlx::query!(
        "DELETE FROM quarantined_stores WHERE encrypted_store = $1",
        &store[..],
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// The index and commitment of the first leaf of `account` below `leaf_count` that records
/// `kind` for `handle` and `key`. The oldest match sits in the oldest mountain, whose path
/// changes least.
pub async fn find_leaf(
    pool: &sqlx::PgPool,
    account: [u8; 32],
    kind: LeafKind,
    handle: [u8; 32],
    key: Option<[u8; 32]>,
    leaf_count: u64,
) -> Result<Option<(u64, [u8; 32])>, SqlxError> {
    let leaf_count = sql_i64(leaf_count, "leaf_count")?;
    // Two statements, because `allowed_key IS NOT DISTINCT FROM $4` cannot use the
    // semantic index.
    let row = match key {
        Some(key) => sqlx::query!(
            r#"
            SELECT leaf_index, commitment
            FROM leaves
            WHERE encrypted_store = $1 AND leaf_kind = $2 AND handle = $3
              AND allowed_key = $4 AND leaf_index < $5
            ORDER BY leaf_index
            LIMIT 1
            "#,
            &account[..],
            kind as i16,
            &handle[..],
            &key[..],
            leaf_count,
        )
        .fetch_optional(pool)
        .await?
        .map(|row| (row.leaf_index, row.commitment)),
        None => sqlx::query!(
            r#"
            SELECT leaf_index, commitment
            FROM leaves
            WHERE encrypted_store = $1 AND leaf_kind = $2 AND handle = $3
              AND allowed_key IS NULL AND leaf_index < $4
            ORDER BY leaf_index
            LIMIT 1
            "#,
            &account[..],
            kind as i16,
            &handle[..],
            leaf_count,
        )
        .fetch_optional(pool)
        .await?
        .map(|row| (row.leaf_index, row.commitment)),
    };
    row.map(|(leaf_index, commitment)| {
        Ok((sql_u64(leaf_index, "leaf_index")?, bytes32(&commitment)?))
    })
    .transpose()
}

/// The authentication path of leaf `leaf_index` among `account`'s first `leaf_count`
/// leaves, read by position in one statement. `None` when the leaf is not below
/// `leaf_count` or a path row is missing; the caller still verifies the path.
pub async fn load_proof(
    pool: &sqlx::PgPool,
    account: [u8; 32],
    leaf_index: u64,
    leaf_count: u64,
) -> Result<Option<MmrProof>, SqlxError> {
    let Some(path) = proof_path(leaf_index, leaf_count) else {
        return Ok(None);
    };
    Ok(load_nodes_at(pool, account, &path)
        .await?
        .map(|siblings| MmrProof {
            leaf_index,
            siblings,
        }))
}

/// The record's peaks of `account`'s first `leaf_count` leaves, oldest mountain first, read
/// from the nodes those leaves completed. `None` when a peak row is missing, which a record
/// holding `leaf_count` leaves never lacks.
pub async fn load_peaks(
    pool: &sqlx::PgPool,
    account: [u8; 32],
    leaf_count: u64,
) -> Result<Option<Vec<[u8; 32]>>, SqlxError> {
    load_nodes_at(pool, account, &peak_positions(leaf_count)).await
}

/// The MMR nodes of `account` at `positions`, in their order, read in one statement. A
/// height-0 position is a leaf row, hashed into its node. The positions have distinct
/// heights, and at most one is a leaf. `None` when a row is missing.
async fn load_nodes_at(
    pool: &sqlx::PgPool,
    account: [u8; 32],
    positions: &[(u8, u64)],
) -> Result<Option<Vec<[u8; 32]>>, SqlxError> {
    // No leaf has a negative index, so -1 reads no leaf row.
    let mut leaf_index = -1;
    let mut heights = Vec::new();
    let mut indexes = Vec::new();
    for &(height, index) in positions {
        if height == 0 {
            leaf_index = sql_i64(index, "leaf_index")?;
        } else {
            heights.push(i16::from(height));
            indexes.push(sql_i64(index, "node_index")?);
        }
    }
    let rows = sqlx::query!(
        r#"
        SELECT 0::SMALLINT AS "height!", commitment AS "node!"
        FROM leaves
        WHERE encrypted_store = $1 AND leaf_index = $2
        UNION ALL
        SELECT node.height, node.node
        FROM nodes AS node
        JOIN UNNEST($3::SMALLINT[], $4::BIGINT[]) AS path(height, node_index)
            USING (height, node_index)
        WHERE node.encrypted_store = $1
        "#,
        &account[..],
        leaf_index,
        &heights,
        &indexes,
    )
    .fetch_all(pool)
    .await?;
    let mut by_height = BTreeMap::new();
    for row in rows {
        let node = bytes32(&row.node)?;
        by_height.insert(
            row.height,
            if row.height == 0 {
                mmr_leaf_node(&node)
            } else {
                node
            },
        );
    }
    Ok(positions
        .iter()
        .map(|&(height, _)| by_height.get(&i16::from(height)).copied())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use zama_solana_acl::{mmr_build_proof, mmr_peaks_from_leaves};

    const STORE: [u8; 32] = [0xAC; 32];
    const OWNER: [u8; 32] = [0xA1; 32];

    fn write(
        previous_leaf_count: u64,
        handle: [u8; 32],
        allowed_keys: Vec<[u8; 32]>,
        make_public: bool,
    ) -> EncryptedStoreWrite {
        EncryptedStoreWrite {
            encrypted_store: STORE,
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
            &[transaction(vec![
                write(0, [1; 32], vec![OWNER], false),
                write(1, [2; 32], vec![[0xB2; 32]], true),
            ])],
            BTreeMap::new(),
        )
        .unwrap();
        let cursor = &reduction.stores[&STORE];
        assert_eq!(cursor.leaf_count, 3);
        assert_eq!(cursor.peaks.len(), 2);
        assert_eq!(reduction.leaves.len(), 3);
        assert_eq!(reduction.leaves[0].key, Some(OWNER));
        assert_eq!(reduction.leaves[1].key, Some([0xB2; 32]));
        assert_eq!(reduction.leaves[2].kind, LeafKind::PublicDecrypt);
        assert_eq!(reduction.leaves[2].handle, [2; 32]);
    }

    /// A store first seen above leaf zero was created before the record started; its earlier
    /// leaves are lost, so the indexer stops instead of serving a partial history.
    #[test]
    fn a_store_first_seen_above_leaf_zero_is_fatal() {
        let error = reduce_block_leaves(
            &[transaction(vec![write(7, [1; 32], vec![OWNER], true)])],
            BTreeMap::new(),
        )
        .unwrap_err();
        assert_eq!(
            error,
            LeafReduceError::UnrecordedHistory {
                encrypted_store: STORE,
                previous_leaf_count: 7,
            }
        );
        assert!(error.to_string().contains("--start-slot"), "{error}");
    }

    #[test]
    fn rejects_a_count_gap() {
        let error = reduce_block_leaves(
            &[transaction(vec![write(3, [2; 32], vec![], false)])],
            BTreeMap::from([(
                STORE,
                EncryptedStoreCursor {
                    leaf_count: 4,
                    peaks: vec![[1; 32]],
                },
            )]),
        )
        .unwrap_err();
        assert_eq!(
            error,
            LeafReduceError::PreviousLeafCountMismatch {
                encrypted_store: STORE,
                declared: 3,
                recorded: 4,
            }
        );
    }

    #[test]
    fn zero_leaf_output_still_checks_continuity() {
        let reduction = reduce_block_leaves(
            &[transaction(vec![write(0, [1; 32], vec![], false)])],
            BTreeMap::new(),
        )
        .unwrap();
        assert_eq!(reduction.stores[&STORE].leaf_count, 0);
        assert!(reduction.leaves.is_empty());
    }

    #[test]
    fn zero_leaf_output_rejects_malformed_history() {
        let error = reduce_block_leaves(
            &[transaction(vec![write(0, [1; 32], vec![], false)])],
            BTreeMap::from([(
                STORE,
                EncryptedStoreCursor {
                    leaf_count: 0,
                    peaks: vec![[1; 32]],
                },
            )]),
        )
        .unwrap_err();
        assert_eq!(
            error,
            LeafReduceError::Mmr {
                encrypted_store: STORE,
                error: AclError::MmrInconsistent,
            }
        );
    }

    /// Growing a store one block at a time, the recorded nodes and leaves hold every
    /// path at every size, and the peaks of every smaller size: read by position, each equals
    /// the one rebuilt from all leaves.
    #[test]
    fn recorded_nodes_hold_every_proof_path_and_peaks() {
        let mut stores = BTreeMap::new();
        let mut commitments = Vec::new();
        let mut nodes = BTreeMap::new();
        for leaf_count in 1u64..=33 {
            let reduction = reduce_block_leaves(
                &[transaction(vec![write(
                    leaf_count - 1,
                    [leaf_count as u8; 32],
                    vec![OWNER],
                    false,
                )])],
                stores,
            )
            .unwrap();
            commitments
                .extend(reduction.leaves.iter().map(|leaf| leaf.commitment));
            nodes.extend(
                reduction
                    .nodes
                    .iter()
                    .map(|node| ((node.height, node.index), node.node)),
            );
            assert_eq!(
                nodes.len() as u64,
                leaf_count - u64::from(leaf_count.count_ones())
            );
            let node_at = |(height, index): (u8, u64)| match height {
                0 => mmr_leaf_node(&commitments[index as usize]),
                _ => nodes[&(height, index)],
            };
            for leaf_index in 0..leaf_count {
                let siblings = proof_path(leaf_index, leaf_count)
                    .unwrap()
                    .into_iter()
                    .map(node_at)
                    .collect();
                assert_eq!(
                    Some(MmrProof {
                        leaf_index,
                        siblings
                    }),
                    mmr_build_proof(&commitments, leaf_index)
                );
            }
            assert_eq!(proof_path(leaf_count, leaf_count), None);
            for size in 0..=leaf_count {
                let peaks: Vec<_> =
                    peak_positions(size).into_iter().map(node_at).collect();
                assert_eq!(
                    peaks,
                    mmr_peaks_from_leaves(&commitments[..size as usize])
                );
            }
            stores = reduction.stores;
        }
    }
}
