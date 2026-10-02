//! The Merkle indexer's sink for the Solana host follower: records the leaves each sealed block's
//! store writes seal, with the resume checkpoint, in one database transaction per block.

use anyhow::anyhow;
use solana_host_follower::host::host_operations;
use solana_host_follower::{
    BlockCheckpoint, BlockSink, IngestFailure, PreparedBlock,
};
use sqlx::PgPool;
use tracing::info;

use crate::store::{
    load_block_leaves, load_recorded_through, load_store_cursors,
    reduce_block_leaves, replay_block_leaves, store_block_leaves,
    store_checkpoint, TransactionStoreWrites,
};

pub struct MerkleIndexerSink {
    pool: PgPool,
}

impl MerkleIndexerSink {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl BlockSink for MerkleIndexerSink {
    async fn apply(&self, block: &PreparedBlock) -> Result<(), IngestFailure> {
        apply_block(&self.pool, block).await
    }
}

/// Where the indexer starts, before the start block is fetched.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexerStart {
    /// After the block the record last applied.
    Resume(BlockCheckpoint),
    /// At this slot, inclusively, on an empty record.
    From(u64),
}

impl IndexerStart {
    /// Resumes from the record's checkpoint, else starts at `start_slot`. An empty record
    /// never starts at the tip: a store created before the start block can never be recorded.
    pub fn resolve(
        checkpoint: Option<BlockCheckpoint>,
        start_slot: Option<u64>,
    ) -> anyhow::Result<Self> {
        match (checkpoint, start_slot) {
            (Some(checkpoint), _) => Ok(Self::Resume(checkpoint)),
            (None, Some(slot)) => Ok(Self::From(slot)),
            (None, None) => Err(anyhow!(
                "the record is empty and no --start-slot is set: set it to a slot before the \
                 first encrypted store was created, such as the zama-host deployment slot"
            )),
        }
    }
}

/// Applies one sealed block: its leaves, nodes and store cursors, and the checkpoint. A block
/// the record already applied must reproduce the leaves recorded at its slot and writes nothing
/// else.
async fn apply_block(
    pool: &PgPool,
    prepared: &PreparedBlock,
) -> Result<(), IngestFailure> {
    let block = &prepared.block;
    let mut writes = Vec::new();
    for transaction in &prepared.transactions {
        let operations = host_operations(&transaction.instructions, block.slot)
            .map_err(|err| {
                IngestFailure::fatal(err).context("decode host operations")
            })?;
        let sources: Vec<_> = operations
            .iter()
            .flat_map(|operation| operation.store_writes().iter().cloned())
            .collect();
        if !sources.is_empty() {
            writes.push(TransactionStoreWrites {
                transaction_index: transaction.index,
                sources,
            });
        }
    }

    let mut db_tx = pool
        .begin()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("open db tx"))?;
    let recorded_through =
        load_recorded_through(&mut db_tx).await.map_err(|err| {
            IngestFailure::retryable(err).context("load checkpoint")
        })?;
    let mut recorded_leaves = 0;
    if recorded_through.is_some_and(|through| block.slot <= through) {
        // Anything but the recorded leaves means the record and the chain disagree about
        // history the proofs already cover.
        let replayed = replay_block_leaves(&writes).map_err(|err| {
            IngestFailure::fatal(err).context("replay leaves")
        })?;
        let recorded = load_block_leaves(&mut db_tx, block.slot)
            .await
            .map_err(|err| {
                IngestFailure::retryable(err).context("load recorded leaves")
            })?;
        if replayed != recorded {
            return Err(IngestFailure::fatal(anyhow!(
                "replayed slot {} does not reproduce its recorded leaves",
                block.slot
            )));
        }
    } else {
        let mut written: Vec<[u8; 32]> = writes
            .iter()
            .flat_map(|transaction| transaction.sources.iter())
            .map(|write| write.encrypted_store)
            .collect();
        written.sort_unstable();
        written.dedup();
        let existing =
            load_store_cursors(&mut db_tx, &written)
                .await
                .map_err(|err| {
                    IngestFailure::retryable(err)
                        .context("load encrypted stores")
                })?;
        let reduction =
            reduce_block_leaves(&writes, existing).map_err(|err| {
                IngestFailure::fatal(err).context("reduce leaves")
            })?;
        store_block_leaves(&mut db_tx, block.slot, &reduction)
            .await
            .map_err(|err| {
                IngestFailure::retryable(err).context("store leaves")
            })?;
        recorded_leaves = reduction.leaves.len();
    }
    store_checkpoint(&mut db_tx, &block.checkpoint())
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("store checkpoint")
        })?;
    db_tx
        .commit()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("commit db tx"))?;

    if recorded_leaves > 0 {
        info!(
            slot = block.slot,
            leaves = recorded_leaves,
            "recorded Solana leaves"
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHECKPOINT: BlockCheckpoint = BlockCheckpoint {
        slot: 41,
        block_hash: [0x41; 32],
    };

    #[test]
    fn the_checkpoint_wins_over_the_start_slot() {
        for start_slot in [None, Some(7)] {
            assert_eq!(
                IndexerStart::resolve(Some(CHECKPOINT), start_slot).unwrap(),
                IndexerStart::Resume(CHECKPOINT)
            );
        }
    }

    #[test]
    fn an_empty_record_starts_at_the_start_slot() {
        assert_eq!(
            IndexerStart::resolve(None, Some(7)).unwrap(),
            IndexerStart::From(7)
        );
    }

    #[test]
    fn an_empty_record_without_a_start_slot_does_not_start() {
        let error = IndexerStart::resolve(None, None).unwrap_err();
        assert!(error.to_string().contains("--start-slot"), "{error}");
    }
}
