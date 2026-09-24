//! Catch-up from an archive RPC once the checkpoint has left Yellowstone's replay window.
//!
//! The archive lists the produced slots after the checkpoint with `getBlocks` and serves each
//! with `getBlock`, both at finalized commitment. A block is prepared like a streamed one
//! (`prepare_rpc_block`), must extend the checkpoint as the stream's validator requires, and is
//! applied through the same path, so the database ends as uninterrupted streaming leaves it.
//! Catch-up stops at the archive's finalized slot, well inside the stream's replay window.

use std::future::Future;

use anyhow::{bail, Context, Result};
use futures_util::stream::{self, StreamExt};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_config::RpcBlockConfig,
};
use solana_commitment_config::CommitmentConfig;
use solana_transaction_status_client_types::{
    TransactionDetails, UiConfirmedBlock, UiTransactionEncoding,
};
use tokio_util::sync::CancellationToken;
use tracing::info;

use super::rpc_block::prepare_rpc_block;
use super::{
    apply_prepared_block, metrics, FatalListenerError, IngestionProgress,
    SolanaGrpcListenerConfig,
};
use crate::database::tfhe_event_propagate::Database;
use crate::solana_grpc_source::SealedBlock;

/// Slots listed per `getBlocks` call, far below the RPC's 500,000-slot cap.
const SLOTS_PER_PAGE: u64 = 1_000;
/// `getBlock` calls in flight. Blocks are still applied one at a time, in slot order.
const BLOCKS_IN_FLIGHT: usize = 8;

/// The ledger reads catch-up needs, at finalized commitment.
pub(super) trait Archive: Sync {
    fn finalized_slot(&self) -> impl Future<Output = Result<u64>> + Send;
    /// The slots in `first..=last` that hold a block; skipped slots have none.
    fn produced_slots(
        &self,
        first: u64,
        last: u64,
    ) -> impl Future<Output = Result<Vec<u64>>> + Send;
    fn block(
        &self,
        slot: u64,
    ) -> impl Future<Output = Result<UiConfirmedBlock>> + Send;
}

impl Archive for RpcClient {
    async fn finalized_slot(&self) -> Result<u64> {
        self.get_slot_with_commitment(CommitmentConfig::finalized())
            .await
            .context("archive getSlot")
    }

    async fn produced_slots(&self, first: u64, last: u64) -> Result<Vec<u64>> {
        self.get_blocks_with_commitment(
            first,
            Some(last),
            CommitmentConfig::finalized(),
        )
        .await
        .with_context(|| format!("archive getBlocks {first}..={last}"))
    }

    async fn block(&self, slot: u64) -> Result<UiConfirmedBlock> {
        self.get_block_with_config(
            slot,
            RpcBlockConfig {
                encoding: Some(UiTransactionEncoding::Base64),
                transaction_details: Some(TransactionDetails::Full),
                rewards: Some(false),
                commitment: Some(CommitmentConfig::finalized()),
                // The listener's Solana crates decode legacy and v0 transactions only; a block
                // holding a v1 transaction is refused until fhevm-internal#2080.
                max_supported_transaction_version: Some(0),
            },
        )
        .await
        .with_context(|| format!("archive getBlock {slot}"))
    }
}

/// Applies every finalized block after the checkpoint, chasing the finalized slot until it is
/// reached. Returns `false` when cancelled.
pub(super) async fn catch_up(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    archive: &impl Archive,
    progress: &mut IngestionProgress,
    cancel: &CancellationToken,
) -> Result<bool> {
    let _active = metrics::ArchiveCatchUp::start(config.chain_id);
    let mut first = first_missing_slot(progress)?;
    let mut finalized = archive.finalized_slot().await?;
    if finalized < first {
        bail!(
            "archive finalized slot {finalized} is behind slot {first}, which the stream can no longer replay"
        );
    }
    info!(
        from_slot = first,
        finalized_slot = finalized,
        "catching up from the archive RPC"
    );
    while first <= finalized {
        let last = finalized.min(first + SLOTS_PER_PAGE - 1);
        let slots = archive.produced_slots(first, last).await?;
        let mut blocks = stream::iter(slots)
            .map(|slot| async move { (slot, archive.block(slot).await) })
            .buffered(BLOCKS_IN_FLIGHT);
        loop {
            let next = tokio::select! {
                _ = cancel.cancelled() => return Ok(false),
                next = blocks.next() => next,
            };
            let Some((slot, block)) = next else { break };
            let prepared =
                prepare_rpc_block(slot, block?).map_err(|error| {
                    FatalListenerError::new(
                        error.context("prepare archive Solana block"),
                    )
                })?;
            extends_checkpoint(progress, &prepared.block)
                .map_err(FatalListenerError::new)?;
            if !apply_prepared_block(db, config, &prepared, progress, cancel)
                .await?
            {
                return Ok(false);
            }
        }
        first = last + 1;
        finalized = archive.finalized_slot().await?;
    }
    info!(
        checkpoint = ?progress.applied,
        "caught up from the archive RPC; resuming the stream"
    );
    Ok(true)
}

/// An unapplied checkpoint is itself the first slot to rebuild; otherwise the one after it.
fn first_missing_slot(progress: &IngestionProgress) -> Result<u64> {
    match (&progress.retry, &progress.applied) {
        (Some(unapplied), _) => Ok(unapplied.slot),
        (None, Some(applied)) => Ok(applied.slot + 1),
        (None, None) => bail!("archive catch-up needs a checkpoint to extend"),
    }
}

/// The stream's ancestry rule: an unapplied checkpoint comes first and unchanged, and every
/// other block names the last applied one as its parent.
fn extends_checkpoint(
    progress: &IngestionProgress,
    block: &SealedBlock,
) -> Result<()> {
    if let Some(unapplied) = &progress.retry {
        if block.checkpoint() != *unapplied {
            bail!(
                "archive block at slot {} is not the unapplied checkpoint at slot {}",
                block.slot,
                unapplied.slot
            );
        }
        return Ok(());
    }
    let Some(applied) = &progress.applied else {
        bail!("archive catch-up needs a checkpoint to extend");
    };
    if block.parent_slot != applied.slot
        || block.parent_block_hash != applied.block_hash
    {
        bail!(
            "archive block ancestry mismatch at slot {} (parent slot {}, checkpoint slot {})",
            block.slot,
            block.parent_slot,
            applied.slot
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use anyhow::{anyhow, Result};
    use serde_json::{json, Value};
    use serial_test::serial;
    use solana_transaction_status_client_types::UiConfirmedBlock;
    use test_harness::instance::{setup_test_db, ImportMode};
    use tokio_util::sync::CancellationToken;
    use yellowstone_grpc_proto::prelude::{
        BlockHeight, SubscribeUpdateBlock, UnixTimestamp,
    };

    use super::super::test_support::config;
    use super::super::wire_fixtures::{app_transaction, Transaction};
    use super::super::{
        apply_prepared_block, prepare_block, BlockCheckpoint,
        FatalListenerError, IngestionProgress, StartPosition,
    };
    use super::{catch_up, extends_checkpoint, first_missing_slot, Archive};
    use crate::database::tfhe_event_propagate::Database;
    use crate::solana_grpc_source::{
        BlockValidator, SealDecision, SealedBlock,
    };
    use fhevm_engine_common::chain_id::ChainId;

    const BLOCK_TIME: i64 = 1_700_000_000;

    /// Every table block ingestion writes.
    const INGESTED_TABLES: &[&str] = &[
        "computations",
        "dependence_chain",
        "allowed_handles",
        "pbs_computations",
        "host_chain_blocks_valid",
        "solana_encrypted_states",
        "solana_encrypted_state_leaves",
        "solana_listener_checkpoint",
    ];

    fn hash(slot: u64) -> [u8; 32] {
        [slot as u8; 32]
    }

    fn checkpoint(slot: u64) -> BlockCheckpoint {
        BlockCheckpoint {
            slot,
            block_hash: hash(slot),
        }
    }

    /// One produced slot: its parent and its transactions, rendered for either transport.
    struct Slot {
        slot: u64,
        parent: u64,
        transactions: Vec<Transaction>,
    }

    impl Slot {
        fn rpc(&self) -> UiConfirmedBlock {
            serde_json::from_value(json!({
                "previousBlockhash": bs58::encode(hash(self.parent)).into_string(),
                "blockhash": bs58::encode(hash(self.slot)).into_string(),
                "parentSlot": self.parent,
                "transactions": self
                    .transactions
                    .iter()
                    .map(|transaction| transaction.rpc_json(Value::Null))
                    .collect::<Vec<_>>(),
                "blockTime": BLOCK_TIME,
                "blockHeight": self.slot - 2,
            }))
            .expect("getBlock JSON")
        }

        fn grpc(&self) -> SubscribeUpdateBlock {
            SubscribeUpdateBlock {
                slot: self.slot,
                blockhash: bs58::encode(hash(self.slot)).into_string(),
                parent_slot: self.parent,
                parent_blockhash: bs58::encode(hash(self.parent)).into_string(),
                block_time: Some(UnixTimestamp {
                    timestamp: BLOCK_TIME,
                }),
                block_height: Some(BlockHeight {
                    block_height: self.slot - 2,
                }),
                executed_transaction_count: self.transactions.len() as u64,
                transactions: self
                    .transactions
                    .iter()
                    .enumerate()
                    .map(|(index, transaction)| {
                        transaction.grpc_info(index as u64)
                    })
                    .collect(),
                ..Default::default()
            }
        }
    }

    /// The checkpoint at 40, then 41, a skipped 42, and 43 and 44 with one execution each.
    fn chain() -> Vec<Slot> {
        vec![
            Slot {
                slot: 40,
                parent: 39,
                transactions: vec![],
            },
            Slot {
                slot: 41,
                parent: 40,
                transactions: vec![app_transaction(1, [1; 32])],
            },
            Slot {
                slot: 43,
                parent: 41,
                transactions: vec![app_transaction(2, [2; 32])],
            },
            Slot {
                slot: 44,
                parent: 43,
                transactions: vec![app_transaction(3, [3; 32])],
            },
        ]
    }

    struct FakeArchive {
        finalized: u64,
        blocks: BTreeMap<u64, UiConfirmedBlock>,
    }

    impl Archive for FakeArchive {
        async fn finalized_slot(&self) -> Result<u64> {
            Ok(self.finalized)
        }

        async fn produced_slots(
            &self,
            first: u64,
            last: u64,
        ) -> Result<Vec<u64>> {
            Ok(self
                .blocks
                .range(first..=last)
                .map(|(slot, _)| *slot)
                .collect())
        }

        async fn block(&self, slot: u64) -> Result<UiConfirmedBlock> {
            self.blocks
                .get(&slot)
                .cloned()
                .ok_or_else(|| anyhow!("slot {slot} has no block"))
        }
    }

    /// Streams `slots` from `progress`'s start through the stream's validator and ingest path.
    async fn stream(
        db: &Database,
        slots: &[&Slot],
        progress: &mut IngestionProgress,
    ) {
        let mut validator = BlockValidator::new(progress.subscription_start());
        for slot in slots {
            if let SealDecision::Process(block) =
                validator.seal(slot.grpc()).unwrap()
            {
                let prepared = prepare_block(block).unwrap();
                assert!(apply_prepared_block(
                    db,
                    &config(),
                    &prepared,
                    progress,
                    &CancellationToken::new(),
                )
                .await
                .unwrap());
            }
        }
    }

    /// Every ingested row, without the columns the database stamps with the wall clock.
    async fn ingested_state(
        pool: &sqlx::PgPool,
    ) -> BTreeMap<&'static str, Vec<String>> {
        let mut state = BTreeMap::new();
        for table in INGESTED_TABLES {
            let columns: String = sqlx::query_scalar(
                "SELECT string_agg(quote_ident(column_name), ', ' ORDER BY ordinal_position) \
                 FROM information_schema.columns \
                 WHERE table_name = $1 AND data_type NOT LIKE 'timestamp%'",
            )
            .bind(table)
            .fetch_one(pool)
            .await
            .unwrap();
            let mut rows: Vec<String> = sqlx::query_scalar(&format!(
                "SELECT row_to_json(r)::text FROM (SELECT {columns} FROM {table}) r"
            ))
            .fetch_all(pool)
            .await
            .unwrap();
            rows.sort();
            state.insert(*table, rows);
        }
        state
    }

    async fn clear(pool: &sqlx::PgPool) {
        sqlx::raw_sql(&format!(
            "TRUNCATE {} RESTART IDENTITY CASCADE",
            INGESTED_TABLES.join(", ")
        ))
        .execute(pool)
        .await
        .unwrap();
    }

    /// The acceptance case of fhevm-internal#2085: from a checkpoint the stream can no longer
    /// replay, catch up from `getBlock` output across a skipped slot, stop at the archive's
    /// finalized slot, then hand back to the stream. The database ends as uninterrupted
    /// streaming leaves it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial(db)]
    async fn catching_up_from_the_archive_then_streaming_matches_uninterrupted_streaming(
    ) {
        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let db = Database::new(
            &instance.db_url,
            ChainId::from_canonical_u64(config().chain_id),
            100,
        )
        .await
        .unwrap();
        let pool = db.pool().await;
        let chain = chain();
        let at = |slot: u64| chain.iter().find(|s| s.slot == slot).unwrap();

        clear(&pool).await;
        let mut progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        stream(&db, &[at(40), at(41), at(43), at(44)], &mut progress).await;
        assert_eq!(progress.applied, Some(checkpoint(44)));
        let streamed = ingested_state(&pool).await;
        assert_eq!(streamed["computations"].len(), 3, "{streamed:#?}");

        clear(&pool).await;
        let archive = FakeArchive {
            finalized: 43,
            blocks: [41, 43, 44]
                .into_iter()
                .map(|slot| (slot, at(slot).rpc()))
                .collect(),
        };
        let mut progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        assert!(catch_up(
            &db,
            &config(),
            &archive,
            &mut progress,
            &CancellationToken::new()
        )
        .await
        .unwrap());
        assert_eq!(
            progress.applied,
            Some(checkpoint(43)),
            "catch-up stops at the finalized slot"
        );
        stream(&db, &[at(43), at(44)], &mut progress).await;
        assert_eq!(progress.applied, Some(checkpoint(44)));
        assert_eq!(ingested_state(&pool).await, streamed);

        // An archive behind the checkpoint leaves the listener retrying, not stopped.
        let behind = catch_up(
            &db,
            &config(),
            &archive,
            &mut progress,
            &CancellationToken::new(),
        )
        .await
        .unwrap_err();
        assert!(
            behind.downcast_ref::<FatalListenerError>().is_none(),
            "{behind:#}"
        );

        // A block of another fork stops the listener before it applies anything, as on the
        // stream.
        clear(&pool).await;
        let mut forked = at(41).rpc();
        forked.previous_blockhash = bs58::encode([0xEE; 32]).into_string();
        let fork = FakeArchive {
            finalized: 41,
            blocks: BTreeMap::from([(41, forked)]),
        };
        let mut progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        let mismatch = catch_up(
            &db,
            &config(),
            &fork,
            &mut progress,
            &CancellationToken::new(),
        )
        .await
        .unwrap_err();
        assert!(
            mismatch.downcast_ref::<FatalListenerError>().is_some(),
            "{mismatch:#}"
        );
        assert_eq!(progress.applied, Some(checkpoint(40)));
        assert!(ingested_state(&pool).await["computations"].is_empty());
    }

    fn sealed(
        slot: u64,
        parent: u64,
        parent_block_hash: [u8; 32],
    ) -> SealedBlock {
        SealedBlock {
            slot,
            block_hash: hash(slot),
            parent_slot: parent,
            parent_block_hash,
            block_time: Some(BLOCK_TIME),
            block_height: Some(slot - 2),
            executed_transaction_count: 0,
            transactions: vec![],
        }
    }

    #[test]
    fn an_archive_block_must_name_the_checkpoint_as_its_parent() {
        let progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        assert_eq!(first_missing_slot(&progress).unwrap(), 41);
        assert!(
            extends_checkpoint(&progress, &sealed(41, 40, hash(40))).is_ok()
        );
        assert!(
            extends_checkpoint(&progress, &sealed(42, 40, hash(40))).is_ok()
        );
        for (slot, parent, parent_hash) in
            [(41, 40, [0xEE; 32]), (42, 41, hash(41))]
        {
            let error = extends_checkpoint(
                &progress,
                &sealed(slot, parent, parent_hash),
            )
            .unwrap_err();
            assert!(
                format!("{error}").contains("ancestry mismatch"),
                "{error}"
            );
        }
    }

    #[test]
    fn an_unapplied_checkpoint_is_rebuilt_first_and_unchanged() {
        let progress =
            IngestionProgress::from(StartPosition::ReplayFrom(checkpoint(40)));
        assert_eq!(first_missing_slot(&progress).unwrap(), 40);
        assert!(
            extends_checkpoint(&progress, &sealed(40, 39, hash(39))).is_ok()
        );
        let changed = SealedBlock {
            block_hash: [0xEE; 32],
            ..sealed(40, 39, hash(39))
        };
        for block in [changed, sealed(41, 40, hash(40))] {
            assert!(extends_checkpoint(&progress, &block).is_err());
        }
    }
}
