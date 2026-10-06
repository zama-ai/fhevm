//! Catch-up from an archive RPC once the checkpoint has left Yellowstone's replay window.
//!
//! The archive lists the produced slots after the checkpoint with `getBlocks`, lists each block's
//! transactions with `getBlock`, and serves each successful transaction naming the host with
//! `getTransaction`, all at finalized commitment. Each transaction is prepared like a streamed one
//! as it arrives (`prepare_rpc_transaction`); the block must extend the checkpoint as the stream's
//! validator requires, and is applied through the same path, so the database ends as
//! uninterrupted streaming leaves it.
//! Catch-up stops at the slot the archive had finalized when it started, and the stream resumes
//! from there. When Yellowstone lags the archive node and has not reached that slot, the stream's
//! replay does not begin at the checkpoint slot, which is fatal ("inclusive replay did not begin
//! at checkpoint slot"); a restart heals it once Yellowstone has caught up.

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
    apply_prepared_block, BlockCheckpoint, BlockSink, FatalIngestError,
    FollowerConfig, IngestionProgress, PreparedBlock, StartPosition,
};
use crate::source::SealedBlock;

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
                // The RPC refuses the whole block when one transaction is of a later version.
                max_supported_transaction_version: Some(1),
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
                encoding: Some(UiTransactionEncoding::Json),
                commitment: Some(CommitmentConfig::finalized()),
                max_supported_transaction_version: Some(1),
            },
        )
        .await
        .map_err(without_url)
        .with_context(|| format!("archive getTransaction {signature}"))
    }
}

/// Fetches and prepares the block at `slot`. A failed read is retried; a response that does not
/// decode stops the follower.
async fn fetch_block(
    archive: &impl Archive,
    slot: u64,
    program: &Pubkey,
) -> Result<PreparedBlock> {
    let fatal = |error: anyhow::Error| -> anyhow::Error {
        FatalIngestError::new(error.context("prepare archive Solana block"))
            .into()
    };
    let listing = list_rpc_block(slot, archive.block(slot).await?, program)
        .map_err(fatal)?;
    let transactions = stream::iter(listing.matching)
        .map(|listed @ (_, signature)| async move {
            let fetched = archive.transaction(signature).await?;
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

/// The finalized block at `slot`, as an inclusive start for [`run`](super::run). Fails when
/// `slot` holds no finalized block, so a start slot can never quietly become the tip.
pub async fn block_checkpoint(
    archive: &RpcClient,
    slot: u64,
) -> Result<BlockCheckpoint> {
    let block = archive
        .get_block_with_config(
            slot,
            RpcBlockConfig {
                encoding: None,
                transaction_details: Some(TransactionDetails::None),
                rewards: Some(false),
                commitment: Some(CommitmentConfig::finalized()),
                max_supported_transaction_version: Some(1),
            },
        )
        .await
        .map_err(without_url)
        .with_context(|| format!("fetch start block {slot}"))?;
    let block_hash = block
        .blockhash
        .parse::<solana_sdk::hash::Hash>()
        .with_context(|| format!("parse the block hash of slot {slot}"))?;
    Ok(BlockCheckpoint {
        slot,
        block_hash: block_hash.to_bytes(),
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
    sink: &impl BlockSink,
    config: &FollowerConfig,
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
            if !apply_prepared_block(sink, config, &prepared, progress, cancel)
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
        StartPosition::ReplayFrom(unapplied) if block.checkpoint() == *unapplied => return Ok(()),
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
    Err(FatalIngestError::new(fork).into())
}

/// The stream only refuses a replay, so a tip start never reaches catch-up.
fn no_checkpoint() -> anyhow::Error {
    FatalIngestError::new(anyhow!(
        "archive catch-up needs a checkpoint to extend"
    ))
    .into()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;
    use std::time::Duration;

    use anyhow::{anyhow, Result};
    use serde_json::Value;
    use solana_client::{
        client_error::{ClientErrorKind, Result as ClientResult},
        nonblocking::rpc_client::RpcClient,
        rpc_client::RpcClientConfig,
        rpc_request::{RpcError, RpcRequest, RpcResponseErrorData},
        rpc_sender::{RpcSender, RpcTransportStats},
    };
    use solana_commitment_config::CommitmentConfig;
    use solana_sdk::{pubkey::Pubkey, signature::Signature};
    use solana_transaction_status_client_types::{
        EncodedConfirmedTransactionWithStatusMeta, UiConfirmedBlock,
    };
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
        accept_transaction, apply_prepared_block, BlockCheckpoint, BlockSink,
        FatalIngestError, IngestFailure, IngestionProgress, PreparedBlock,
        PreparedTransaction, StartPosition,
    };
    use super::{
        block_checkpoint, catch_up, extends_checkpoint, fetch_block,
        first_missing_slot, Archive,
    };
    use crate::host::{host_operations, DecodedInstruction};
    use crate::source::{BlockValidator, SealDecision, SealedBlock};

    /// Records every block it is handed, in order: two sinks fed the same chain hold equal
    /// records.
    #[derive(Default)]
    struct RecordingSink(Mutex<Vec<PreparedBlock>>);

    impl BlockSink for RecordingSink {
        async fn apply(
            &self,
            block: &PreparedBlock,
        ) -> std::result::Result<(), IngestFailure> {
            self.0.lock().unwrap().push(block.clone());
            Ok(())
        }
    }

    impl RecordingSink {
        fn blocks(&self) -> Vec<PreparedBlock> {
            self.0.lock().unwrap().clone()
        }
    }

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
        error.downcast_ref::<FatalIngestError>().is_some()
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
        sink: &RecordingSink,
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
                    sink,
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

    async fn catch_up_from_40(
        sink: &RecordingSink,
        archive: &FakeArchive,
    ) -> (Result<bool>, IngestionProgress) {
        let mut progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        // A catch-up chasing the moving finalized slot would never end.
        let result = tokio::time::timeout(
            Duration::from_secs(30),
            catch_up(
                sink,
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
    /// at the archive's finalized slot, then hand back to the stream. The sink is handed exactly
    /// the blocks uninterrupted streaming hands it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn catching_up_from_the_archive_then_streaming_matches_uninterrupted_streaming(
    ) {
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

        let streaming = RecordingSink::default();
        let mut progress =
            IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
        stream(&streaming, &[at(40), at(41), at(43), at(44)], &mut progress)
            .await;
        assert_eq!(progress.applied, Some(checkpoint(44)));
        let streamed = streaming.blocks();
        let operations = streamed
            .iter()
            .flat_map(|block| &block.transactions)
            .map(|transaction| {
                host_operations(&transaction.instructions, 0).unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(operations.iter().flatten().count(), 3, "{streamed:#?}");
        assert_eq!(
            operations
                .iter()
                .flatten()
                .flat_map(|operation| operation.store_writes())
                .count(),
            2,
            "{streamed:#?}"
        );

        let catching_up = RecordingSink::default();
        let archive = archive_of(43, &[41, 43, 44]);
        let (caught_up, mut progress) =
            catch_up_from_40(&catching_up, &archive).await;
        assert!(caught_up.unwrap());
        assert_eq!(
            progress.applied,
            Some(checkpoint(43)),
            "catch-up stops at the slot finalized when it started"
        );
        stream(&catching_up, &[at(43), at(44)], &mut progress).await;
        assert_eq!(progress.applied, Some(checkpoint(44)));
        assert_eq!(catching_up.blocks(), streamed);

        // An archive behind the checkpoint leaves the follower retrying, not stopped.
        let behind = catch_up(
            &catching_up,
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
        let gapped = RecordingSink::default();
        let (gap, progress) =
            catch_up_from_40(&gapped, &archive_of(43, &[43])).await;
        let gap = gap.unwrap_err();
        assert!(!is_fatal(&gap), "{gap:#}");
        assert!(
            format!("{gap:#}").contains("missing slots 41..=41"),
            "{gap:#}"
        );
        assert_eq!(progress.applied, Some(checkpoint(40)));
        assert!(gapped.blocks().is_empty());

        // A block of another fork stops the follower before it applies anything, as on the
        // stream.
        let mut forked = at(41).rpc();
        forked.previous_blockhash = bs58::encode([0xEE; 32]).into_string();
        let fork = FakeArchive {
            finalized: AtomicU64::new(41),
            blocks: BTreeMap::from([(41, forked)]),
            transactions: at(41).rpc_transactions().collect(),
        };
        let forking = RecordingSink::default();
        let (mismatch, progress) = catch_up_from_40(&forking, &fork).await;
        let mismatch = mismatch.unwrap_err();
        assert!(is_fatal(&mismatch), "{mismatch:#}");
        assert_eq!(progress.applied, Some(checkpoint(40)));
        assert!(forking.blocks().is_empty());
    }

    /// Devnet slot 506183762 as its RPC served it: a block of legacy, v0 and v1 transactions,
    /// listed with `transactionDetails: "accounts"`, and each of its two v1 calls into
    /// `DEVNET_PROGRAM` fetched in both `json` and `base64`.
    const DEVNET_SLOT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../solana/test-fixtures/rpc/devnet_v1_slot_506183762.json"
    ));
    const DEVNET_PROGRAM: &str = "BiSoNHVpsVZW2F7rx2eQ59yQwKxzU5NvBcmKshCSUypi";

    /// Serves the captured slot, and refuses what devnet refuses: a response holding a v1
    /// transaction, to a request that does not support version 1. It also refuses the
    /// `transactionDetails: "none"` listing below version 1, which a real node answers, so every
    /// archive read is held to version 1.
    struct DevnetNode(Value);

    #[async_trait::async_trait]
    impl RpcSender for DevnetNode {
        async fn send(
            &self,
            request: RpcRequest,
            params: Value,
        ) -> ClientResult<Value> {
            let config = &params[1];
            let response = match request {
                RpcRequest::GetBlock if params[0] == self.0["slot"] => {
                    match config["transactionDetails"].as_str() {
                        Some("accounts") => Some(self.0["getBlock"].clone()),
                        Some("none") => {
                            let mut block = self.0["getBlock"].clone();
                            block
                                .as_object_mut()
                                .unwrap()
                                .remove("transactions");
                            Some(block)
                        }
                        _ => None,
                    }
                }
                RpcRequest::GetTransaction => params[0]
                    .as_str()
                    .and_then(|signature| {
                        self.0["getTransaction"].get(signature)
                    })
                    .and_then(|encodings| {
                        encodings
                            .get(config["encoding"].as_str().unwrap_or("json"))
                    })
                    .cloned(),
                _ => None,
            };
            let error = |code, message: String| {
                ClientErrorKind::RpcError(RpcError::RpcResponseError {
                    code,
                    message,
                    data: RpcResponseErrorData::Empty,
                })
                .into()
            };
            let Some(response) = response else {
                return Err(error(
                    -32601,
                    format!("the capture holds no {request} {params}"),
                ));
            };
            if config["maxSupportedTransactionVersion"]
                .as_u64()
                .is_none_or(|version| version < 1)
            {
                return Err(error(
                    -32015,
                    "Transaction version (1) is not supported by the requesting client".into(),
                ));
            }
            Ok(response)
        }

        fn get_transport_stats(&self) -> RpcTransportStats {
            RpcTransportStats::default()
        }

        fn url(&self) -> String {
            "devnet capture".into()
        }
    }

    /// fhevm-internal#2111: the archive's own reads rebuild a block holding v1 transactions.
    #[tokio::test]
    async fn the_archive_rebuilds_a_devnet_block_holding_v1_transactions() {
        let archive = RpcClient::new_sender(
            DevnetNode(serde_json::from_str(DEVNET_SLOT).unwrap()),
            RpcClientConfig::with_commitment(CommitmentConfig::finalized()),
        );
        let prepared = fetch_block(
            &archive,
            506_183_762,
            &DEVNET_PROGRAM.parse().unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(prepared.block.executed_transaction_count, 11);
        assert_eq!(
            block_checkpoint(&archive, 506_183_762).await.unwrap(),
            BlockCheckpoint {
                slot: 506_183_762,
                block_hash: prepared.block.block_hash,
            }
        );
        // Payer, a program account and the Clock sysvar.
        let accounts = [
            "2dbChKurXcZj4iDQPdEPyYHJGaiAqJno2omaPjHCQ7EN",
            "AWVYnCT2ZdLsWZf1X9KXatZhC2TyruRM22y8KZqVeupr",
            "SysvarC1ock11111111111111111111111111111111",
        ]
        .map(|key| key.parse::<Pubkey>().unwrap().to_bytes())
        .to_vec();
        let call = |signature: &str, index, data| PreparedTransaction {
            signature: signature.parse().unwrap(),
            index,
            instructions: vec![DecodedInstruction {
                data: hex::decode(data).unwrap(),
                accounts: accounts.clone(),
            }],
        };
        assert_eq!(
            prepared.transactions,
            [
                call(
                    "38x9pAHxtWYiuCZ3qVNFszCooj2D6TdWDRdBXdnKAcFcT4Sg8aBjtMHgudoPETr4wDXQ2eZ14MUh13FNwyFMRURH",
                    5,
                    "0af09b9211b754da18000000000080cdcb4e5a1f77000000000000000002000200",
                ),
                call(
                    "e4uvnv3G8GtKhwEveT6RhGYBQkSHKim8oh5mN8fgXATnBJ7YdYpL7mUnX1ScnFmWEeeMdXfApvmv27Zam34XAEj",
                    10,
                    "0aa9982018b754da180000000000a416b3cda02077000000000000000002000200",
                ),
            ]
        );
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

    /// A sink's failure kind decides what the follower does: a retryable failure replays the
    /// block, a fatal one stops the follower. Neither commits the block.
    #[tokio::test]
    async fn a_sink_failure_is_retried_or_stops_the_follower_by_its_kind() {
        struct FailingSink(fn() -> IngestFailure);

        impl BlockSink for FailingSink {
            async fn apply(
                &self,
                _: &PreparedBlock,
            ) -> std::result::Result<(), IngestFailure> {
                Err((self.0)())
            }
        }

        let block = PreparedBlock {
            block: sealed(41, 40, hash(40)),
            transactions: vec![],
        };
        for (sink, fatal) in [
            (
                FailingSink(|| {
                    IngestFailure::retryable(anyhow!("database down"))
                }),
                false,
            ),
            (
                FailingSink(|| {
                    IngestFailure::fatal(anyhow!("record diverged"))
                }),
                true,
            ),
        ] {
            let mut progress =
                IngestionProgress::from(StartPosition::Resume(checkpoint(40)));
            let error = apply_prepared_block(
                &sink,
                &config(),
                &block,
                &mut progress,
                &CancellationToken::new(),
            )
            .await
            .unwrap_err();
            assert_eq!(is_fatal(&error), fatal, "{error:#}");
            assert_eq!(progress.applied, Some(checkpoint(40)));
            assert_eq!(progress.retry, Some(checkpoint(41)));
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
