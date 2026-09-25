//! Catch-up from an archive RPC once the checkpoint has left Yellowstone's replay window.
//!
//! The archive lists the produced slots after the checkpoint with `getBlocks`, lists each block's
//! transactions with `getBlock`, and serves each successful transaction naming the host with
//! `getTransaction`, all at finalized commitment. Each transaction is prepared like a streamed one
//! as it arrives (`prepare_rpc_transaction`); the block must extend the checkpoint as the stream's
//! validator requires, and is applied through the same path, so the database ends as
//! uninterrupted streaming leaves it.
//! Catch-up stops at the slot the archive had finalized when it started, well inside the
//! stream's replay window, so the stream takes over however fast the chain moves.

use std::future::Future;

use anyhow::{anyhow, bail, Context, Result};
use futures_util::stream::{self, StreamExt, TryStreamExt};
use solana_client::{
    client_error::{ClientError, ClientErrorKind},
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcBlockConfig, RpcTransactionConfig},
};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, TransactionDetails,
    UiConfirmedBlock, UiTransactionEncoding,
};
use tokio_util::sync::CancellationToken;
use tracing::info;

use super::rpc_block::{list_rpc_block, prepare_rpc_transaction};
use super::{
    apply_prepared_block, FatalListenerError, IngestionProgress, PreparedBlock,
    SolanaGrpcListenerConfig, StartPosition,
};
use crate::database::tfhe_event_propagate::Database;
use crate::solana_grpc_source::SealedBlock;

/// Slots listed per `getBlocks` call, far below the RPC's 500,000-slot cap.
const SLOTS_PER_PAGE: u64 = 1_000;
/// Blocks fetched at a time. Blocks are still applied one at a time, in slot order.
const BLOCKS_IN_FLIGHT: usize = 8;
/// `getTransaction` calls in flight for one block.
const TRANSACTIONS_IN_FLIGHT: usize = 8;

/// The ledger reads catch-up needs, at finalized commitment.
pub(super) trait Archive: Sync {
    fn finalized_slot(&self) -> impl Future<Output = Result<u64>> + Send;
    /// The slots in `first..=last` that hold a block; skipped slots have none.
    fn produced_slots(
        &self,
        first: u64,
        last: u64,
    ) -> impl Future<Output = Result<Vec<u64>>> + Send;
    /// The block with its transactions as account lists.
    fn block(
        &self,
        slot: u64,
    ) -> impl Future<Output = Result<UiConfirmedBlock>> + Send;
    fn transaction(
        &self,
        signature: Signature,
    ) -> impl Future<Output = Result<EncodedConfirmedTransactionWithStatusMeta>> + Send;
}

impl Archive for RpcClient {
    async fn finalized_slot(&self) -> Result<u64> {
        self.get_slot_with_commitment(CommitmentConfig::finalized())
            .await
            .map_err(without_url)
            .context("archive getSlot")
    }

    async fn produced_slots(&self, first: u64, last: u64) -> Result<Vec<u64>> {
        self.get_blocks_with_commitment(
            first,
            Some(last),
            CommitmentConfig::finalized(),
        )
        .await
        .map_err(without_url)
        .with_context(|| format!("archive getBlocks {first}..={last}"))
    }

    async fn block(&self, slot: u64) -> Result<UiConfirmedBlock> {
        self.get_block_with_config(
            slot,
            RpcBlockConfig {
                // An account list carries no transaction bytes, so no encoding applies.
                encoding: None,
                transaction_details: Some(TransactionDetails::Accounts),
                rewards: Some(false),
                commitment: Some(CommitmentConfig::finalized()),
                // The listener's Solana crates decode legacy and v0 transactions only; a block
                // holding a v1 transaction is refused until fhevm-internal#2080.
                max_supported_transaction_version: Some(0),
            },
        )
        .await
        .map_err(without_url)
        .with_context(|| format!("archive getBlock {slot}"))
    }

    async fn transaction(
        &self,
        signature: Signature,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta> {
        self.get_transaction_with_config(
            &signature,
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Base64),
                commitment: Some(CommitmentConfig::finalized()),
                max_supported_transaction_version: Some(0),
            },
        )
        .await
        .map_err(without_url)
        .with_context(|| format!("archive getTransaction {signature}"))
    }
}

/// Fetches and prepares the block at `slot`. A failed read is retried; a response that does not
/// decode stops the listener.
async fn fetch_block(
    archive: &impl Archive,
    slot: u64,
    program: &Pubkey,
) -> Result<PreparedBlock> {
    let fatal = |error: anyhow::Error| -> anyhow::Error {
        FatalListenerError::new(error.context("prepare archive Solana block"))
            .into()
    };
    let listing = list_rpc_block(slot, archive.block(slot).await?, program)
        .map_err(fatal)?;
    let transactions = stream::iter(listing.matching)
        .map(|listed| async move {
            let fetched = archive.transaction(listed.1).await?;
            prepare_rpc_transaction(slot, listed, fetched, program)
                .map_err(fatal)
        })
        .buffered(TRANSACTIONS_IN_FLIGHT)
        .try_collect()
        .await?;
    Ok(PreparedBlock {
        block: listing.block,
        transactions,
    })
}

/// A hosted archive URL usually carries its API key, and reqwest errors print the URL.
fn without_url(error: ClientError) -> ClientError {
    let ClientError { request, kind } = error;
    let kind = match *kind {
        ClientErrorKind::Reqwest(error) => {
            ClientErrorKind::Reqwest(error.without_url())
        }
        kind => kind,
    };
    ClientError {
        request,
        kind: Box::new(kind),
    }
}

/// Applies every block after the checkpoint up to the slot the archive has finalized now.
/// Returns `false` when cancelled.
pub(super) async fn catch_up(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    archive: &impl Archive,
    progress: &mut IngestionProgress,
    cancel: &CancellationToken,
) -> Result<bool> {
    let program = config.program_id;
    let mut first = first_missing_slot(&progress.subscription_start())?;
    let Some(target) =
        until_cancelled(cancel, archive.finalized_slot()).await?
    else {
        return Ok(false);
    };
    if target < first {
        bail!(
            "archive finalized slot {target} is behind slot {first}, which the stream can no longer replay"
        );
    }
    info!(
        from_slot = first,
        to_slot = target,
        "catching up from the archive RPC"
    );
    while first <= target {
        let last = target.min(first + SLOTS_PER_PAGE - 1);
        let Some(slots) =
            until_cancelled(cancel, archive.produced_slots(first, last))
                .await?
        else {
            return Ok(false);
        };
        let mut blocks = stream::iter(slots)
            .map(|slot| fetch_block(archive, slot, &program))
            .buffered(BLOCKS_IN_FLIGHT);
        loop {
            let next = tokio::select! {
                _ = cancel.cancelled() => return Ok(false),
                next = blocks.next() => next,
            };
            let Some(prepared) = next else { break };
            let prepared = prepared?;
            extends_checkpoint(
                &progress.subscription_start(),
                &prepared.block,
            )?;
            if !apply_prepared_block(db, config, &prepared, progress, cancel)
                .await?
            {
                return Ok(false);
            }
        }
        first = last + 1;
    }
    info!(
        checkpoint = ?progress.applied,
        "caught up from the archive RPC; resuming the stream"
    );
    Ok(true)
}

/// `None` when `cancel` fires first.
async fn until_cancelled<T>(
    cancel: &CancellationToken,
    future: impl Future<Output = Result<T>>,
) -> Result<Option<T>> {
    tokio::select! {
        _ = cancel.cancelled() => Ok(None),
        result = future => result.map(Some),
    }
}

/// An unapplied checkpoint is itself the first slot to rebuild; otherwise the one after it.
fn first_missing_slot(start: &StartPosition) -> Result<u64> {
    match start {
        StartPosition::ReplayFrom(unapplied) => Ok(unapplied.slot),
        StartPosition::Resume(applied) => Ok(applied.slot + 1),
        StartPosition::Tip => Err(no_checkpoint()),
    }
}

/// The stream's ancestry rule: an unapplied checkpoint comes first and unchanged, and every
/// other block names the last applied one as its parent. A block that descends from a slot the
/// archive did not list, the unapplied checkpoint included, means the archive skipped produced
/// slots, which is retried; any other mismatch is a fork.
fn extends_checkpoint(
    start: &StartPosition,
    block: &SealedBlock,
) -> Result<()> {
    let fork = match start {
        StartPosition::ReplayFrom(unapplied) if block.checkpoint() == *unapplied => {
            return Ok(())
        }
        StartPosition::ReplayFrom(unapplied)
            if block.slot > unapplied.slot
                && (block.parent_slot > unapplied.slot
                    || (block.parent_slot == unapplied.slot
                        && block.parent_block_hash == unapplied.block_hash)) =>
        {
            bail!(
                "the archive is missing slots {}..={} before slot {}",
                unapplied.slot,
                block.parent_slot,
                block.slot
            )
        }
        StartPosition::ReplayFrom(unapplied) => anyhow!(
            "archive block at slot {} is not the unapplied checkpoint at slot {}",
            block.slot,
            unapplied.slot
        ),
        StartPosition::Resume(applied)
            if block.parent_slot == applied.slot
                && block.parent_block_hash == applied.block_hash =>
        {
            return Ok(())
        }
        StartPosition::Resume(applied) if block.parent_slot > applied.slot => bail!(
            "the archive is missing slots {}..={} before slot {}",
            applied.slot + 1,
            block.parent_slot,
            block.slot
        ),
        StartPosition::Resume(applied) => anyhow!(
            "archive block ancestry mismatch at slot {} (parent slot {}, checkpoint slot {})",
            block.slot,
            block.parent_slot,
            applied.slot
        ),
        StartPosition::Tip => return Err(no_checkpoint()),
    };
    Err(FatalListenerError::new(fork).into())
}

/// The stream only refuses a replay, so a tip start never reaches catch-up.
fn no_checkpoint() -> anyhow::Error {
    FatalListenerError::new(anyhow!(
        "archive catch-up needs a checkpoint to extend"
    ))
    .into()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;

    use anyhow::{anyhow, Result};
    use serde_json::Value;
    use serial_test::serial;
    use solana_sdk::{pubkey::Pubkey, signature::Signature};
    use solana_transaction_status_client_types::{
        EncodedConfirmedTransactionWithStatusMeta, UiConfirmedBlock,
    };
    use test_harness::instance::{setup_test_db, ImportMode};
    use tokio_util::sync::CancellationToken;
    use yellowstone_grpc_proto::prelude::{
        BlockHeight, SubscribeUpdateBlockMeta, SubscribeUpdateTransaction,
        UnixTimestamp,
    };

    use super::super::test_support::{config, ZAMA_HOST};
    use super::super::wire_fixtures::{
        app_transaction, block_json, foreign_transaction,
        storing_app_transaction, Transaction, BLOCK_TIME,
    };
    use super::super::{
        accept_transaction, apply_prepared_block, BlockCheckpoint,
        FatalListenerError, IngestionProgress, StartPosition,
    };
    use super::{catch_up, extends_checkpoint, first_missing_slot, Archive};
    use crate::database::tfhe_event_propagate::Database;
    use crate::solana_grpc_source::{
        BlockValidator, SealDecision, SealedBlock,
    };
    use fhevm_engine_common::chain_id::ChainId;

    /// Every table block ingestion writes.
    const INGESTED_TABLES: &[&str] = &[
        "computations",
        "dependence_chain",
        "allowed_handles",
        "pbs_computations",
        "host_chain_blocks_valid",
        "solana_encrypted_states",
        "solana_encrypted_state_leaves",
        "solana_encrypted_state_nodes",
        "solana_listener_checkpoint",
    ];

    const STORE: [u8; 32] = [0x22; 32];
    const ALLOWED: [u8; 32] = [0x33; 32];

    fn hash(slot: u64) -> [u8; 32] {
        [slot as u8; 32]
    }

    fn checkpoint(slot: u64) -> BlockCheckpoint {
        BlockCheckpoint {
            slot,
            block_hash: hash(slot),
        }
    }

    fn is_fatal(error: &anyhow::Error) -> bool {
        error.downcast_ref::<FatalListenerError>().is_some()
    }

    /// One produced slot: its parent and its transactions, rendered for either transport.
    struct Slot {
        slot: u64,
        parent: u64,
        transactions: Vec<Transaction>,
    }

    impl Slot {
        /// The block as `getBlock` lists it, with `transactionDetails: "accounts"`.
        fn rpc(&self) -> UiConfirmedBlock {
            block_json(
                self.slot,
                self.parent,
                hash,
                self.transactions
                    .iter()
                    .map(|transaction| {
                        transaction.rpc_accounts_json(Value::Null)
                    })
                    .collect(),
            )
        }

        /// Each transaction's `getTransaction` JSON.
        fn rpc_transactions(
            &self,
        ) -> impl Iterator<Item = (Signature, serde_json::Value)> + '_ {
            self.transactions.iter().map(|transaction| {
                let response = serde_json::to_value(
                    transaction.rpc_transaction(self.slot),
                );
                (Signature::from(transaction.signature), response.unwrap())
            })
        }

        /// The slot as the stream delivers it: the transactions naming the host, at their index
        /// in the block, then the block meta.
        fn grpc(
            &self,
        ) -> (Vec<SubscribeUpdateTransaction>, SubscribeUpdateBlockMeta)
        {
            let host = ZAMA_HOST.parse::<Pubkey>().unwrap().to_bytes();
            let transactions = self
                .transactions
                .iter()
                .enumerate()
                .filter(|(_, transaction)| {
                    transaction.static_keys.contains(&host)
                })
                .map(|(index, transaction)| {
                    transaction.grpc_update(self.slot, index as u64)
                })
                .collect();
            let meta = SubscribeUpdateBlockMeta {
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
                ..Default::default()
            };
            (transactions, meta)
        }
    }

    /// The checkpoint at 40, then 41 (after another program's transaction), a skipped 42, and
    /// 43 and 44, with one execution each. 41 and 43 each store a leaf.
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
                transactions: vec![
                    foreign_transaction(9),
                    storing_app_transaction(1, [1; 32], STORE, ALLOWED, 0),
                ],
            },
            Slot {
                slot: 43,
                parent: 41,
                transactions: vec![storing_app_transaction(
                    2, [2; 32], STORE, ALLOWED, 1,
                )],
            },
            Slot {
                slot: 44,
                parent: 43,
                transactions: vec![app_transaction(3, [3; 32])],
            },
        ]
    }

    /// The chain keeps finalizing: each `finalized_slot` read is one slot later.
    struct FakeArchive {
        finalized: AtomicU64,
        blocks: BTreeMap<u64, UiConfirmedBlock>,
        transactions: BTreeMap<Signature, serde_json::Value>,
    }

    impl Archive for FakeArchive {
        async fn finalized_slot(&self) -> Result<u64> {
            // Yield as a real RPC call does, so a catch-up that never ends still lets the
            // test's timeout fire.
            tokio::task::yield_now().await;
            Ok(self.finalized.fetch_add(1, Ordering::Relaxed))
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

        async fn transaction(
            &self,
            signature: Signature,
        ) -> Result<EncodedConfirmedTransactionWithStatusMeta> {
            let response = self
                .transactions
                .get(&signature)
                .ok_or_else(|| anyhow!("no transaction {signature}"))?;
            Ok(serde_json::from_value(response.clone())?)
        }
    }

    /// Streams `slots` from `progress`'s start through the stream's preparation, validator and
    /// ingest path.
    async fn stream(
        db: &Database,
        slots: &[&Slot],
        progress: &mut IngestionProgress,
    ) {
        let host = ZAMA_HOST.parse::<Pubkey>().unwrap();
        let mut validator = BlockValidator::new(progress.subscription_start());
        for slot in slots {
            let (transactions, meta) = slot.grpc();
            for update in transactions {
                accept_transaction(&mut validator, update, &host).unwrap();
            }
            if let SealDecision::Process(prepared) =
                validator.block_meta(meta).unwrap()
            {
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

    async fn catch_up_from_40(
        db: &Database,
        archive: &FakeArchive,
    ) -> (Result<bool>, IngestionProgress) {
        let mut progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        // A catch-up chasing the moving finalized slot would never end.
        let result = tokio::time::timeout(
            Duration::from_secs(30),
            catch_up(
                db,
                &config(),
                archive,
                &mut progress,
                &CancellationToken::new(),
            ),
        )
        .await
        .expect("catch-up ends");
        (result, progress)
    }

    /// The acceptance case of fhevm-internal#2085: from a checkpoint the stream can no longer
    /// replay, catch up from `getBlock` and `getTransaction` output across a skipped slot, stop
    /// at the archive's finalized slot, then hand back to the stream. The database ends as
    /// uninterrupted streaming leaves it.
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
        let archive_of = |finalized: u64, slots: &[u64]| FakeArchive {
            finalized: AtomicU64::new(finalized),
            blocks: slots.iter().map(|slot| (*slot, at(*slot).rpc())).collect(),
            transactions: slots
                .iter()
                .flat_map(|slot| at(*slot).rpc_transactions())
                .collect(),
        };

        clear(&pool).await;
        let mut progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        stream(&db, &[at(40), at(41), at(43), at(44)], &mut progress).await;
        assert_eq!(progress.applied, Some(checkpoint(44)));
        let streamed = ingested_state(&pool).await;
        assert_eq!(streamed["computations"].len(), 3, "{streamed:#?}");
        assert_eq!(
            streamed["solana_encrypted_state_leaves"].len(),
            2,
            "{streamed:#?}"
        );

        clear(&pool).await;
        let archive = archive_of(43, &[41, 43, 44]);
        let (caught_up, mut progress) = catch_up_from_40(&db, &archive).await;
        assert!(caught_up.unwrap());
        assert_eq!(
            progress.applied,
            Some(checkpoint(43)),
            "catch-up stops at the slot finalized when it started"
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
        assert!(!is_fatal(&behind), "{behind:#}");

        // An archive without the history after the checkpoint is retried too, and applies
        // nothing past the gap.
        clear(&pool).await;
        let (gap, progress) =
            catch_up_from_40(&db, &archive_of(43, &[43])).await;
        let gap = gap.unwrap_err();
        assert!(!is_fatal(&gap), "{gap:#}");
        assert!(
            format!("{gap:#}").contains("missing slots 41..=41"),
            "{gap:#}"
        );
        assert_eq!(progress.applied, Some(checkpoint(40)));
        assert!(ingested_state(&pool).await["computations"].is_empty());

        // A block of another fork stops the listener before it applies anything, as on the
        // stream.
        let mut forked = at(41).rpc();
        forked.previous_blockhash = bs58::encode([0xEE; 32]).into_string();
        let fork = FakeArchive {
            finalized: AtomicU64::new(41),
            blocks: BTreeMap::from([(41, forked)]),
            transactions: at(41).rpc_transactions().collect(),
        };
        let (mismatch, progress) = catch_up_from_40(&db, &fork).await;
        let mismatch = mismatch.unwrap_err();
        assert!(is_fatal(&mismatch), "{mismatch:#}");
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
        }
    }

    #[test]
    fn an_archive_block_must_name_the_checkpoint_as_its_parent() {
        let start = StartPosition::Resume(checkpoint(40));
        assert_eq!(first_missing_slot(&start).unwrap(), 41);
        assert!(extends_checkpoint(&start, &sealed(41, 40, hash(40))).is_ok());
        assert!(extends_checkpoint(&start, &sealed(42, 40, hash(40))).is_ok());
        for (slot, parent, parent_hash) in
            [(41, 40, [0xEE; 32]), (41, 39, hash(39))]
        {
            let error =
                extends_checkpoint(&start, &sealed(slot, parent, parent_hash))
                    .unwrap_err();
            assert!(is_fatal(&error), "{error:#}");
        }
        let gap =
            extends_checkpoint(&start, &sealed(43, 42, hash(42))).unwrap_err();
        assert!(!is_fatal(&gap), "{gap:#}");
        assert!(
            format!("{gap:#}").contains("missing slots 41..=42"),
            "{gap:#}"
        );
    }

    #[test]
    fn an_unapplied_checkpoint_is_rebuilt_first_and_unchanged() {
        let start = StartPosition::ReplayFrom(checkpoint(40));
        assert_eq!(first_missing_slot(&start).unwrap(), 40);
        assert!(extends_checkpoint(&start, &sealed(40, 39, hash(39))).is_ok());
        let changed = SealedBlock {
            block_hash: [0xEE; 32],
            ..sealed(40, 39, hash(39))
        };
        // Another block at the checkpoint slot, or a later block whose chain skips it.
        for block in [
            changed,
            sealed(41, 39, hash(39)),
            sealed(41, 40, [0xEE; 32]),
        ] {
            let error = extends_checkpoint(&start, &block).unwrap_err();
            assert!(is_fatal(&error), "{error:#}");
        }
        // A later block descending from the checkpoint, or from a later slot, means the archive
        // left out the checkpoint block itself: retried, like any archive gap.
        for (block, missing) in [
            (sealed(41, 40, hash(40)), "missing slots 40..=40"),
            (sealed(43, 42, hash(42)), "missing slots 40..=42"),
        ] {
            let gap = extends_checkpoint(&start, &block).unwrap_err();
            assert!(!is_fatal(&gap), "{gap:#}");
            assert!(format!("{gap:#}").contains(missing), "{gap:#}");
        }
    }
}
