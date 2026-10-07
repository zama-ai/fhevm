//! Follows the zama-host program on a Solana cluster through Yellowstone gRPC, and hands each
//! sealed block, reduced to the host's instructions, to a [`BlockSink`].
//!
//! - **Finalized only.** Blocks are followed at finalized commitment, so none is rolled back and
//!   nothing is unwound (INVARIANTS #32). A sink still authorizes nothing from them: the KMS
//!   re-checks live on-chain state before releasing any plaintext (INVARIANTS #31).
//! - **Resume.** The sink commits each block together with its checkpoint. A restart resumes
//!   from that checkpoint, and Yellowstone replays inclusively from it. A checkpoint older than
//!   the provider's replay window is caught up from an archive RPC with `getBlock` and
//!   `getTransaction` (`archive`), then the stream resumes from it.
//! - **Failures.** A sink reports each failure as retryable, which resumes from the checkpoint,
//!   or fatal, which stops the follower.

use std::fmt;
use std::future::Future;
use std::time::Duration;

use anchor_lang::prelude::Pubkey;
use anyhow::{anyhow, ensure, Context, Result};
use futures_util::stream::StreamExt;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::{Channel, ClientTlsConfig};
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, Message as TransactionMessage,
    SubscribeRequest, SubscribeUpdateTransaction,
    SubscribeUpdateTransactionInfo, TransactionStatusMeta,
};
use zama_solana_transaction::{
    CompiledInstruction as CanonicalCompiledInstruction,
    InnerInstructionGroup as CanonicalInnerInstructionGroup,
};

use crate::host::DecodedInstruction;
use crate::source::{
    build_subscribe_request, BlockValidator, SealDecision, SealedBlock,
};

mod archive;
mod metrics;
mod rpc_block;
#[cfg(test)]
pub(crate) mod wire_fixtures;

pub use archive::block_checkpoint;
pub use metrics::track_finalized_slot;

const MAX_DECODING_MESSAGE_SIZE: usize = 64 * 1024 * 1024;
const SOLANA_GRPC_INGEST_TIMEOUT: Duration = Duration::from_secs(60);
const STREAM_STALL_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Copy, Debug)]
enum IngestFailureKind {
    Retryable,
    Fatal,
}

#[derive(Debug)]
pub struct IngestFailure {
    kind: IngestFailureKind,
    error: anyhow::Error,
}

impl IngestFailure {
    pub fn retryable(error: impl Into<anyhow::Error>) -> Self {
        Self {
            kind: IngestFailureKind::Retryable,
            error: error.into(),
        }
    }

    pub fn fatal(error: impl Into<anyhow::Error>) -> Self {
        Self {
            kind: IngestFailureKind::Fatal,
            error: error.into(),
        }
    }

    pub fn context(self, context: &'static str) -> Self {
        Self {
            kind: self.kind,
            error: self.error.context(context),
        }
    }

    /// Whether the follower stops on this failure instead of replaying the block.
    pub fn is_fatal(&self) -> bool {
        matches!(self.kind, IngestFailureKind::Fatal)
    }

    pub(crate) fn into_error(self) -> anyhow::Error {
        self.error
    }
}

impl fmt::Display for IngestFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug)]
struct FatalIngestError(anyhow::Error);

impl FatalIngestError {
    fn new(error: anyhow::Error) -> Self {
        Self(error)
    }

    fn into_inner(self) -> anyhow::Error {
        self.0
    }
}

impl fmt::Display for FatalIngestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for FatalIngestError {}

#[derive(Clone)]
pub struct FollowerConfig {
    /// Yellowstone gRPC endpoint, e.g. `http://poc-solana-validator:10000`.
    pub grpc_url: String,
    /// Optional `x-token` auth metadata (None for a local validator).
    pub x_token: Option<String>,
    /// Base58 zama-host program id to follow (not the id this crate was compiled with).
    pub program_id: Pubkey,
    /// On-chain HostConfig chain id, the `host_chain_id` label of the follower's metrics.
    pub chain_id: u64,
}

/// Hand-written so the `x-token` never reaches a log through a `{:?}` of the config.
impl fmt::Debug for FollowerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FollowerConfig")
            .field("grpc_url", &self.grpc_url)
            .field("x_token", &self.x_token.as_ref().map(|_| "[REDACTED]"))
            .field("program_id", &self.program_id)
            .field("chain_id", &self.chain_id)
            .finish()
    }
}

/// Commits each sealed block the follower hands it, with the block's checkpoint, atomically.
///
/// The follower may hand the same block again: `Resume` and `ReplayFrom` replay inclusively, and
/// an `apply` dropped on cancel or timeout may already have committed. A block the sink already
/// holds must therefore leave its state unchanged, and `apply` must be safe to drop at any await.
/// A [`host::host_operations`](crate::host::host_operations) error means the chain accepted
/// something this crate cannot decode, so a sink reports it as fatal.
pub trait BlockSink: Sync {
    fn apply(
        &self,
        block: &PreparedBlock,
    ) -> impl Future<Output = std::result::Result<(), IngestFailure>> + Send;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockCheckpoint {
    pub slot: u64,
    pub block_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StartPosition {
    Tip,
    /// Verify the committed checkpoint without applying it again.
    Resume(BlockCheckpoint),
    /// Inclusive bootstrap anchor: this block has not been committed locally.
    ReplayFrom(BlockCheckpoint),
}

#[derive(Debug, Default)]
struct IngestionProgress {
    applied: Option<BlockCheckpoint>,
    retry: Option<BlockCheckpoint>,
}

impl From<StartPosition> for IngestionProgress {
    fn from(start: StartPosition) -> Self {
        match start {
            StartPosition::Tip => Self::default(),
            StartPosition::Resume(checkpoint) => Self {
                applied: Some(checkpoint),
                retry: None,
            },
            StartPosition::ReplayFrom(checkpoint) => Self {
                applied: None,
                retry: Some(checkpoint),
            },
        }
    }
}

impl IngestionProgress {
    fn subscription_start(&self) -> StartPosition {
        match &self.retry {
            Some(checkpoint) => StartPosition::ReplayFrom(checkpoint.clone()),
            None => self
                .applied
                .clone()
                .map(StartPosition::Resume)
                .unwrap_or(StartPosition::Tip),
        }
    }

    fn observe_unapplied(&mut self, checkpoint: BlockCheckpoint) {
        if self.retry.is_none() {
            self.retry = Some(checkpoint);
        }
    }

    fn commit(&mut self, checkpoint: BlockCheckpoint) {
        self.applied = Some(checkpoint);
        self.retry = None;
    }
}

/// Connects, subscribes, and hands sealed blocks to `sink` until `cancel` fires. Reconnects with
/// a `from_slot` cursor on stream errors. When the stream can no longer replay from the
/// checkpoint, catches up from `archive` first.
pub async fn run(
    sink: &impl BlockSink,
    archive: &RpcClient,
    config: &FollowerConfig,
    start: StartPosition,
    cancel: CancellationToken,
) -> Result<()> {
    info!(
        program_id = %config.program_id,
        grpc_url = %config.grpc_url,
        "Starting Solana host follower (Yellowstone gRPC transport)"
    );
    metrics::record_start(config.chain_id, &start);
    let mut progress = IngestionProgress::from(start);

    loop {
        if cancel.is_cancelled() {
            return Ok(());
        }
        let start = progress.subscription_start();
        let err = match subscribe_loop(
            sink,
            config,
            start,
            &mut progress,
            &cancel,
        )
        .await
        {
            Ok(StreamEnd::Cancelled) => return Ok(()),
            Ok(StreamEnd::ReplayWindowPassed(status)) => {
                warn!(%status, checkpoint = ?progress.applied, retry_cursor = ?progress.retry, "Yellowstone can no longer replay from the checkpoint; catching up from the archive RPC");
                metrics::set_archive_catch_up(config.chain_id, true);
                let caught_up = archive::catch_up(
                    sink,
                    config,
                    archive,
                    &mut progress,
                    &cancel,
                )
                .await;
                metrics::set_archive_catch_up(config.chain_id, false);
                match caught_up {
                    Ok(true) => continue,
                    Ok(false) => return Ok(()),
                    Err(err) => err,
                }
            }
            Err(err) => err,
        };
        match err.downcast::<FatalIngestError>() {
            Ok(fatal) => {
                let err = fatal.into_inner();
                error!(error = format!("{err:#}"), checkpoint = ?progress.applied, retry_cursor = ?progress.retry, "Solana host follower stopped on fail-closed ingestion error");
                return Err(err);
            }
            Err(err) => {
                error!(error = format!("{err:#}"), checkpoint = ?progress.applied, retry_cursor = ?progress.retry, "ingestion interrupted; resuming inclusively from the checkpoint");
                metrics::record_interruption(config.chain_id);
                tokio::select! {
                    _ = cancel.cancelled() => return Ok(()),
                    _ = tokio::time::sleep(Duration::from_secs(2)) => {}
                }
            }
        }
    }
}

fn validated_account_keys<'a>(
    keys: impl IntoIterator<Item = &'a Vec<u8>>,
) -> Result<Vec<[u8; 32]>> {
    keys.into_iter()
        .enumerate()
        .map(|(index, key)| {
            <[u8; 32]>::try_from(key.as_slice()).map_err(|_| {
                anyhow!(
                    "account key {index} has invalid length {}, expected 32 bytes",
                    key.len()
                )
            })
        })
        .collect()
}

/// The host program's instructions, from either wire format's resolution: reconstruction reads
/// no other, so a transaction that merely lists the host keeps none of its other instructions'
/// data.
fn host_instructions(
    resolved: Vec<zama_solana_transaction::ResolvedInstruction>,
    program: &Pubkey,
) -> Vec<DecodedInstruction> {
    resolved
        .into_iter()
        .filter(|instruction| instruction.program_id == program.to_bytes())
        .map(|instruction| DecodedInstruction {
            data: instruction.data,
            accounts: instruction.accounts,
        })
        .collect()
}

fn resolve_transaction_instructions(
    message: &TransactionMessage,
    meta: &TransactionStatusMeta,
) -> Result<Vec<zama_solana_transaction::ResolvedInstruction>> {
    if meta.err.is_some() {
        return Ok(Vec::new());
    }
    let static_keys = validated_account_keys(&message.account_keys)?;
    let loaded_writable_keys =
        validated_account_keys(&meta.loaded_writable_addresses)?;
    let loaded_readonly_keys =
        validated_account_keys(&meta.loaded_readonly_addresses)?;
    let top_level = message
        .instructions
        .iter()
        .map(|instruction| CanonicalCompiledInstruction {
            program_id_index: instruction.program_id_index as usize,
            account_indices: instruction
                .accounts
                .iter()
                .map(|index| *index as usize)
                .collect(),
            data: instruction.data.clone(),
            stack_height: None,
        })
        .collect::<Vec<_>>();
    // A node that did not record inner instructions would hide every CPI into the host.
    ensure!(
        !meta.inner_instructions_none,
        "transaction meta has no inner instructions"
    );
    let inner_groups = meta
        .inner_instructions
        .iter()
        .map(|group| CanonicalInnerInstructionGroup {
            top_level_index: group.index as usize,
            instructions: group
                .instructions
                .iter()
                .map(|instruction| CanonicalCompiledInstruction {
                    program_id_index: instruction.program_id_index as usize,
                    account_indices: instruction
                        .accounts
                        .iter()
                        .map(|index| *index as usize)
                        .collect(),
                    data: instruction.data.clone(),
                    stack_height: instruction.stack_height,
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    zama_solana_transaction::resolve_transaction(
        &static_keys,
        &loaded_writable_keys,
        &loaded_readonly_keys,
        top_level,
        inner_groups,
    )
    .map_err(anyhow::Error::from)
}

/// Why a subscription ended without an error.
#[derive(Debug)]
enum StreamEnd {
    Cancelled,
    /// The provider refused to replay from the checkpoint: it has left the replay window.
    ReplayWindowPassed(tonic::Status),
}

async fn subscribe_loop(
    sink: &impl BlockSink,
    config: &FollowerConfig,
    start: StartPosition,
    progress: &mut IngestionProgress,
    cancel: &CancellationToken,
) -> Result<StreamEnd> {
    let endpoint = Channel::from_shared(config.grpc_url.clone())
        .context("invalid grpc url")?;
    // from_shared leaves tls unset. Attach rustls when the parsed URI is https so hosted
    // Yellowstone handshakes; plaintext http (local e2e geyser) stays as-is.
    let endpoint = if endpoint.uri().scheme_str() == Some("https") {
        endpoint
            .tls_config(ClientTlsConfig::new().with_webpki_roots())
            .context("configure grpc tls")?
    } else {
        endpoint
    };
    let channel = tokio::select! {
        _ = cancel.cancelled() => return Ok(StreamEnd::Cancelled),
        result = endpoint.connect() => result.context("connect grpc endpoint")?,
    };

    let token: Option<MetadataValue<Ascii>> = config
        .x_token
        .as_ref()
        .map(|t| t.parse())
        .transpose()
        .context("invalid x-token")?;

    let mut client = GeyserClient::with_interceptor(
        channel,
        move |mut req: tonic::Request<()>| {
            if let Some(token) = &token {
                req.metadata_mut().insert("x-token", token.clone());
            }
            Ok(req)
        },
    )
    .max_decoding_message_size(MAX_DECODING_MESSAGE_SIZE);

    let is_resume = !matches!(start, StartPosition::Tip);
    let request = build_subscribe_request(&config.program_id, &start);
    let mut validator = BlockValidator::new(start);
    let outbound = futures_util::stream::once(async move { request })
        .chain(futures_util::stream::pending::<SubscribeRequest>());

    let response = tokio::select! {
        _ = cancel.cancelled() => return Ok(StreamEnd::Cancelled),
        result = client.subscribe(outbound) => result.context("subscribe")?,
    };
    let mut stream = response.into_inner();

    // Block meta is emitted for every produced slot, including slots with no host transaction,
    // while Yellowstone pings every few seconds whatever its feed does: only a block meta shows
    // the stream advancing. The deadline restarts after the block is applied, so a slow apply is
    // not taken for a stall.
    let mut stall_deadline = tokio::time::Instant::now() + STREAM_STALL_TIMEOUT;
    loop {
        tokio::select! {
            _ = cancel.cancelled() => return Ok(StreamEnd::Cancelled),
            _ = tokio::time::sleep_until(stall_deadline) => {
                return Err(anyhow!(
                    "no block meta for {}s; reconnecting",
                    STREAM_STALL_TIMEOUT.as_secs()
                ));
            }
            msg = stream.message() => {
                let msg = match msg {
                    Ok(message) => message,
                    Err(status) if is_resume && is_replay_window_passed(&status) => {
                        return Ok(StreamEnd::ReplayWindowPassed(status));
                    }
                    Err(status) if is_resume && is_replay_unsupported(&status) => {
                        return Err(FatalIngestError::new(anyhow!(
                            "Yellowstone provider cannot replay from a slot: {status}"
                        )).into());
                    }
                    Err(status) => return Err(anyhow!(status).context("grpc stream")),
                };
                let Some(update) = msg else {
                    // A None message means the server closed the stream. This is NOT a
                    // cancellation (handled above) — return an error so the outer loop reconnects
                    // and resumes from `from_slot`, rather than exiting silently and missing every
                    // later slot.
                    return Err(anyhow!("grpc stream closed by server"));
                };
                match update.update_oneof {
                    Some(UpdateOneof::Transaction(update)) => {
                        accept_transaction(&mut validator, update, &config.program_id)
                            .map_err(|error| {
                                FatalIngestError::new(error.context(
                                    "validate Solana transaction",
                                ))
                            })?;
                    }
                    Some(UpdateOneof::BlockMeta(meta)) => {
                        let decision = validator.block_meta(meta).map_err(|error| {
                            FatalIngestError::new(error.context(
                                "validate sealed Solana block",
                            ))
                        })?;
                        if let SealDecision::Process(prepared) = decision {
                            if !apply_prepared_block(
                                sink,
                                config,
                                &prepared,
                                progress,
                                cancel,
                            )
                            .await?
                            {
                                return Ok(StreamEnd::Cancelled);
                            }
                        }
                        stall_deadline =
                            tokio::time::Instant::now() + STREAM_STALL_TIMEOUT;
                    }
                    Some(UpdateOneof::Ping(_)) => debug!("grpc ping"),
                    _ => {}
                }
            }
        }
    }
}

/// A sealed block with its host transactions, in the transport-neutral form reconstruction reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedBlock {
    pub block: SealedBlock,
    pub transactions: Vec<PreparedTransaction>,
}

/// A successful transaction reduced to its host program's instructions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedTransaction {
    pub signature: Signature,
    pub index: u64,
    pub instructions: Vec<DecodedInstruction>,
}

/// Prepares a streamed transaction and holds it for its slot.
fn accept_transaction(
    validator: &mut BlockValidator,
    update: SubscribeUpdateTransaction,
    program: &Pubkey,
) -> Result<()> {
    let slot = update.slot;
    let info = update.transaction.ok_or_else(|| {
        anyhow!("transaction update in slot {slot} has no transaction")
    })?;
    match prepare_transaction(info, program)? {
        Some(transaction) => validator.transaction(slot, transaction),
        None => Ok(()),
    }
}

/// Prepares a streamed transaction when it arrives. A failed or vote transaction changes no
/// output and is dropped; the subscription leaves them out already.
fn prepare_transaction(
    info: SubscribeUpdateTransactionInfo,
    program: &Pubkey,
) -> Result<Option<PreparedTransaction>> {
    let signature = Signature::try_from(info.signature.as_slice())
        .context("invalid Solana signature")?;
    let meta = info
        .meta
        .as_ref()
        .ok_or_else(|| anyhow!("transaction {signature} has no status meta"))?;
    if meta.err.is_some() || info.is_vote {
        return Ok(None);
    }
    let message = info
        .transaction
        .as_ref()
        .and_then(|transaction| transaction.message.as_ref())
        .ok_or_else(|| {
            anyhow!("successful transaction {signature} has no message")
        })?;
    Ok(Some(PreparedTransaction {
        signature,
        index: info.index,
        instructions: host_instructions(
            resolve_transaction_instructions(message, meta)
                .with_context(|| format!("transaction {signature}"))?,
            program,
        ),
    }))
}

/// Applies one prepared block and advances the checkpoint. Returns `false` when cancelled.
async fn apply_prepared_block(
    sink: &impl BlockSink,
    config: &FollowerConfig,
    prepared: &PreparedBlock,
    progress: &mut IngestionProgress,
    cancel: &CancellationToken,
) -> Result<bool> {
    progress.observe_unapplied(prepared.block.checkpoint());
    match ingest_block(sink, prepared, cancel).await {
        Ok(BlockIngestOutcome::Complete) => {
            progress.commit(prepared.block.checkpoint());
            metrics::record_applied(config.chain_id, &prepared.block);
            Ok(true)
        }
        Ok(BlockIngestOutcome::Cancelled) => Ok(false),
        Err(err) if !err.is_fatal() => Err(err
            .into_error()
            .context("retryable sealed block ingest failure")),
        Err(err) => Err(FatalIngestError::new(
            err.into_error()
                .context("fatal sealed block ingest failure"),
        )
        .into()),
    }
}

/// The provider keeps no replay history at all, so catching up could never hand back to it.
fn is_replay_unsupported(status: &tonic::Status) -> bool {
    status.message() == "from_slot is not supported"
}

/// The requested slot is older than the provider's replay window.
fn is_replay_window_passed(status: &tonic::Status) -> bool {
    let message = status.message();
    message.starts_with("broadcast from ")
        && message.contains(" is not available")
}

#[cfg(test)]
mod replay_status_tests {
    use super::StartPosition;
    use super::{
        is_replay_unsupported, is_replay_window_passed, BlockCheckpoint,
        IngestionProgress, PreparedBlock,
    };
    use crate::source::testing::{following, meta};
    use crate::source::{BlockValidator, SealDecision};
    use yellowstone_grpc_proto::prelude::SubscribeUpdateBlockMeta;

    #[test]
    fn classifies_replay_refusals_without_treating_transport_as_one() {
        let passed = tonic::Status::internal(
            "broadcast from 7 is not available, last available: 12",
        );
        assert!(is_replay_window_passed(&passed));
        assert!(!is_replay_unsupported(&passed));
        let unsupported = tonic::Status::internal("from_slot is not supported");
        assert!(is_replay_unsupported(&unsupported));
        assert!(!is_replay_window_passed(&unsupported));
        for transport in [
            tonic::Status::unavailable("connection reset"),
            tonic::Status::internal("failed to send from_slot request"),
        ] {
            assert!(!is_replay_window_passed(&transport));
            assert!(!is_replay_unsupported(&transport));
        }
    }

    #[test]
    fn retryable_first_block_replays_as_unapplied_then_commits() {
        let update = meta(5, 0);
        let SealDecision::Process(PreparedBlock { block: first, .. }) =
            following(5).block_meta(update.clone()).unwrap()
        else {
            panic!()
        };
        let checkpoint = first.checkpoint();
        let mut progress = IngestionProgress::default();
        progress.observe_unapplied(checkpoint.clone());

        // A retryable apply failure does not mutate progress.
        let start = progress.subscription_start();
        assert_eq!(start, StartPosition::ReplayFrom(checkpoint.clone()));
        assert!(progress.applied.is_none());

        let mut retry_validator = BlockValidator::new(start);
        let SealDecision::Process(PreparedBlock { block: retried, .. }) =
            retry_validator.block_meta(update).unwrap()
        else {
            panic!()
        };
        progress.commit(retried.checkpoint());

        assert_eq!(progress.applied, Some(checkpoint));
        assert!(progress.retry.is_none());
    }

    #[test]
    fn bootstrap_anchor_survives_disconnect_and_is_processed_before_checkpointing(
    ) {
        let checkpoint = BlockCheckpoint {
            slot: 5,
            block_hash: [5; 32],
        };
        let mut progress = IngestionProgress::from(StartPosition::ReplayFrom(
            checkpoint.clone(),
        ));
        // Disconnects before the first block leave the inclusive, unapplied cursor intact.
        for _ in 0..2 {
            assert_eq!(
                progress.subscription_start(),
                StartPosition::ReplayFrom(checkpoint.clone())
            );
            assert!(progress.applied.is_none());
        }
        let update = meta(5, 0);
        let mut validator = BlockValidator::new(progress.subscription_start());
        let SealDecision::Process(PreparedBlock { block, .. }) =
            validator.block_meta(update.clone()).unwrap()
        else {
            panic!("bootstrap block must be applied")
        };
        progress.commit(block.checkpoint());
        let start = progress.subscription_start();
        assert_eq!(start, StartPosition::Resume(checkpoint));
        let mut restarted = BlockValidator::new(start);
        assert!(matches!(
            restarted.block_meta(update).unwrap(),
            SealDecision::Skip
        ));
    }

    #[test]
    fn bootstrap_rejects_a_provider_that_skips_or_changes_the_anchor() {
        for (slot, hash) in [(6, [6; 32]), (5, [9; 32])] {
            let mut validator = BlockValidator::new(StartPosition::ReplayFrom(
                BlockCheckpoint {
                    slot: 5,
                    block_hash: [5; 32],
                },
            ));
            assert!(validator
                .block_meta(SubscribeUpdateBlockMeta {
                    slot,
                    blockhash: bs58::encode(hash).into_string(),
                    parent_slot: 4,
                    parent_blockhash: bs58::encode([4; 32]).into_string(),
                    ..Default::default()
                })
                .is_err());
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlockIngestOutcome {
    Complete,
    Cancelled,
}

async fn ingest_block(
    sink: &impl BlockSink,
    prepared: &PreparedBlock,
    cancel: &CancellationToken,
) -> std::result::Result<BlockIngestOutcome, IngestFailure> {
    let result = tokio::select! {
        _ = cancel.cancelled() => return Ok(BlockIngestOutcome::Cancelled),
        result = tokio::time::timeout(
            SOLANA_GRPC_INGEST_TIMEOUT,
            sink.apply(prepared),
        ) => result,
    };
    match result {
        Ok(result) => result?,
        Err(_) => {
            return Err(IngestFailure::retryable(anyhow!(
                "timed out ingesting Solana block in slot {}",
                prepared.block.slot
            )))
        }
    }
    info!(
        slot = prepared.block.slot,
        parent_slot = prepared.block.parent_slot,
        block_height = ?prepared.block.block_height,
        executed_transaction_count = prepared.block.executed_transaction_count,
        host_transaction_count = prepared.transactions.len(),
        "ingested sealed Solana block"
    );
    Ok(BlockIngestOutcome::Complete)
}

#[cfg(test)]
mod account_resolution_tests {
    use super::test_support::ZAMA_HOST;
    use super::wire_fixtures::app_transaction;
    use super::{
        prepare_transaction, resolve_transaction_instructions,
        validated_account_keys,
    };
    use yellowstone_grpc_proto::prelude::{
        Message as TransactionMessage, TransactionError, TransactionStatusMeta,
    };

    #[test]
    fn a_node_without_inner_instructions_is_refused() {
        let mut info = app_transaction(8, [2; 32]).grpc_info(0);
        let meta = info.meta.as_mut().unwrap();
        meta.inner_instructions.clear();
        meta.inner_instructions_none = true;

        let error = prepare_transaction(info, &ZAMA_HOST.parse().unwrap())
            .expect_err("a stream without inner instructions hides host CPIs");

        assert!(
            format!("{error:#}").contains("no inner instructions"),
            "{error:#}"
        );
    }

    #[test]
    fn rejects_malformed_account_key_length() {
        let err = validated_account_keys([&vec![1; 32], &vec![2; 31]])
            .expect_err("short account keys must fail closed");

        assert!(err.to_string().contains(
            "account key 1 has invalid length 31, expected 32 bytes"
        ));
    }

    #[test]
    fn failed_transaction_is_ignored_before_instruction_decoding() {
        let message = TransactionMessage {
            account_keys: vec![vec![1; 31]],
            ..Default::default()
        };
        let meta = TransactionStatusMeta {
            err: Some(TransactionError { err: vec![1] }),
            ..Default::default()
        };

        let instructions = resolve_transaction_instructions(&message, &meta)
            .expect("failed transactions are valid chain history");

        assert!(instructions.is_empty());
    }
}

#[cfg(test)]
mod slot_size_tests {
    use super::test_support::ZAMA_HOST;
    use super::wire_fixtures::{
        app_transaction, foreign_transaction, Compiled, Transaction,
    };
    use super::PreparedBlock;
    use super::{prepare_transaction, MAX_DECODING_MESSAGE_SIZE};
    use crate::source::testing::{following, meta};
    use crate::source::SealDecision;
    use solana_sdk::pubkey::Pubkey;
    use yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, SubscribeUpdate,
        SubscribeUpdateTransaction,
    };
    use yellowstone_grpc_proto::prost::Message;

    /// Agave's bounds on one transaction's message: 64 instructions in its trace, 10 KiB of
    /// data per CPI and 10,000 bytes of logs.
    const INSTRUCTION_TRACE: usize = 64;
    const CPI_DATA: usize = 10 * 1024;
    const LOG_BYTES: usize = 10_000;

    /// A transaction that lists the host and fills its message to Agave's bounds through
    /// another program.
    fn junk(signature: u8) -> Transaction {
        let mut transaction = foreign_transaction(signature);
        transaction
            .static_keys
            .push(ZAMA_HOST.parse::<Pubkey>().unwrap().to_bytes());
        transaction.inner_groups = vec![(
            0,
            (1..INSTRUCTION_TRACE)
                .map(|_| Compiled {
                    program_id_index: 1,
                    accounts: vec![2],
                    data: vec![0xAB; CPI_DATA],
                    stack_height: Some(2),
                })
                .collect(),
        )];
        transaction
    }

    /// A message carries one transaction because the subscription asks for transactions, not
    /// blocks (`request_subscribes_to_host_transactions_and_block_meta`). This checks that a
    /// transaction at Agave's bounds stays far below the decoding limit, and that a slot whose
    /// junk adds up past it still yields its host transaction.
    #[test]
    fn a_transaction_message_stays_far_below_the_decoding_limit() {
        let host = ZAMA_HOST.parse::<Pubkey>().unwrap();
        let junk_info = |index: u64| {
            let mut info = junk(index as u8).grpc_info(index);
            info.meta.as_mut().unwrap().log_messages =
                vec!["x".repeat(LOG_BYTES)];
            info
        };
        let message_size = SubscribeUpdate {
            update_oneof: Some(UpdateOneof::Transaction(
                SubscribeUpdateTransaction {
                    transaction: Some(junk_info(0)),
                    slot: 5,
                    ..Default::default()
                },
            )),
            ..Default::default()
        }
        .encoded_len();
        assert!(
            message_size < MAX_DECODING_MESSAGE_SIZE / 64,
            "one transaction message is {message_size} bytes"
        );
        // Enough junk that one message holding the slot would pass the decoding limit.
        let junk_count = MAX_DECODING_MESSAGE_SIZE / message_size + 1;

        // Slot 5 carries the junk, then one host transaction.
        let mut validator = following(5);
        for index in 0..junk_count as u64 {
            let prepared = prepare_transaction(junk_info(index), &host)
                .unwrap()
                .unwrap();
            assert!(
                prepared.instructions.is_empty(),
                "the junk keeps no instruction"
            );
            validator.transaction(5, prepared).unwrap();
        }
        let host_index = junk_count as u64;
        let prepared = prepare_transaction(
            app_transaction(0xF0, [7; 32]).grpc_info(host_index),
            &host,
        )
        .unwrap()
        .unwrap();
        validator.transaction(5, prepared).unwrap();

        let SealDecision::Process(PreparedBlock {
            block,
            transactions,
        }) = validator.block_meta(meta(5, host_index + 1)).unwrap()
        else {
            panic!("slot 5 is applied")
        };
        assert_eq!(block.slot, 5);
        assert_eq!(transactions.len(), junk_count + 1);
        assert!(!transactions[host_index as usize].instructions.is_empty());
    }
}

#[cfg(test)]
mod config_tests {
    use super::FollowerConfig;

    #[test]
    fn debug_redacts_yellowstone_x_token() {
        let config = FollowerConfig {
            x_token: Some("secret".to_owned()),
            ..super::test_support::config()
        };
        let rendered = format!("{config:?}");
        assert!(!rendered.contains("secret"), "{rendered}");
        assert!(rendered.contains("[REDACTED]"), "{rendered}");
    }
}

/// Configuration shared by the follower's tests.
#[cfg(test)]
pub(crate) mod test_support {
    use super::FollowerConfig;

    // A valid pubkey that is not the compiled-in `zama_host::ID`: following must use the
    // configured deployment.
    pub(crate) const ZAMA_HOST: &str =
        "7DYCAhqwQSKqqL1h8V1XmY1BTcMWxrASQYKNMy87jeg3";

    pub(crate) fn config() -> FollowerConfig {
        FollowerConfig {
            grpc_url: "http://127.0.0.1:1".to_owned(),
            x_token: None,
            program_id: ZAMA_HOST.parse().unwrap(),
            chain_id: zama_host::SOLANA_POC_CHAIN_ID,
        }
    }
}
