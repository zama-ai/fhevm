//! Yellowstone gRPC reconstruction path for the Solana host listener.
//!
//! Two operational limits live here, stated in the listener's own docs and not only in the
//! decisions log (DD-025/DD-028):
//!
//! - **No reorg unwind.** Blocks are ingested at confirmed commitment and never rolled back;
//!   work scheduled from a minority fork is wasted, never reverted. This is safe only because
//!   scheduling is decoupled from authorization — the KMS re-checks live on-chain state before
//!   releasing any plaintext (INVARIANTS #31/#32).
//! - **Version pairing.** Handle re-derivation uses the program crate's `computed_*` functions
//!   (INVARIANTS #28) and hashes the followed `--program-id`, not the crate's compiled
//!   `declare_id!`. Instruction layout still has no runtime handshake: deploy the listener from
//!   the same rev as the program (INVARIANTS #33).
//!
//! A checkpoint older than the provider's replay window is caught up from an archive RPC with
//! `getBlock` and `getTransaction` (`archive`), then the stream resumes from it.
//!
//! Each sealed block is applied in one database transaction: its compute rows, the
//! leaves its writes sealed (`database::solana_leaves`) and the resume checkpoint.
//! Compute rows and leaves both carry the result handles each `FheExecutedEvent`
//! emitted, so they name the chain's handles even where re-derivation disagrees.
//! A step whose handle this listener does not re-derive is held back: its row is
//! inserted as a terminal error, and the tfhe-worker drains everything that depends
//! on it. The rest of the block is ingested normally.

use std::fmt;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use futures_util::stream::StreamExt;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use anchor_lang::prelude::Pubkey;

use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::{Channel, ClientTlsConfig};
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, Message as TransactionMessage,
    SubscribeRequest, SubscribeUpdateTransactionInfo, TransactionStatusMeta,
};
use zama_solana_transaction::{
    CompiledInstruction as CanonicalCompiledInstruction,
    InnerInstructionGroup as CanonicalInnerInstructionGroup,
};

use crate::database::solana_leaves::{
    load_block_leaves, load_encrypted_store_histories, reduce_block_leaves,
    store_block_leaves, store_checkpoint, EncryptedStoreWrite,
    StoredCheckpoint, TransactionStoreWrites,
};
use crate::database::tfhe_event_propagate::{Database, TransactionId};
use crate::solana_adapter::{
    hold_back_computations, insert_solana_block_records, HeldBackComputation,
    SolanaBlockMeta, SolanaIngestStats,
};
use crate::solana_grpc_source::{
    build_subscribe_request, BlockValidator, SealDecision, SealedBlock,
};
use crate::solana_reconstruct::HandleMismatch;

mod archive;
mod metrics;
mod rpc_block;
#[cfg(test)]
mod wire_fixtures;

pub use metrics::track_confirmed_slot;

const MAX_DECODING_MESSAGE_SIZE: usize = 64 * 1024 * 1024;
const SOLANA_GRPC_INGEST_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IngestFailureKind {
    Retryable,
    Fatal,
}

#[derive(Debug)]
struct IngestFailure {
    kind: IngestFailureKind,
    error: anyhow::Error,
}

impl IngestFailure {
    fn retryable(error: impl Into<anyhow::Error>) -> Self {
        Self {
            kind: IngestFailureKind::Retryable,
            error: error.into(),
        }
    }

    fn fatal(error: impl Into<anyhow::Error>) -> Self {
        Self {
            kind: IngestFailureKind::Fatal,
            error: error.into(),
        }
    }

    fn context(self, context: &'static str) -> Self {
        Self {
            kind: self.kind,
            error: self.error.context(context),
        }
    }

    fn kind(&self) -> IngestFailureKind {
        self.kind
    }

    fn into_error(self) -> anyhow::Error {
        self.error
    }
}

impl fmt::Display for IngestFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug)]
struct FatalListenerError(anyhow::Error);

impl FatalListenerError {
    fn new(error: anyhow::Error) -> Self {
        Self(error)
    }

    fn into_inner(self) -> anyhow::Error {
        self.0
    }
}

impl fmt::Display for FatalListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for FatalListenerError {}

#[derive(Clone)]
pub struct SolanaGrpcListenerConfig {
    /// Yellowstone gRPC endpoint, e.g. `http://poc-solana-validator:10000`.
    pub grpc_url: String,
    /// Optional `x-token` auth metadata (None for a local validator).
    pub x_token: Option<String>,
    /// Base58 zama-host program id to follow: instruction filter and the id hashed into
    /// reconstructed handles (not the id this crate was compiled with).
    pub program_id: Pubkey,
    /// On-chain HostConfig chain_id used in handle derivation (distinct from the
    /// coprocessor host-chain id). Used by the reconstruction path.
    pub chain_id: u64,
    /// Shared scheduler cap; zero disables the slow lane.
    pub dependent_ops_max_per_chain: u32,
}

/// Hand-written so the `x-token` never reaches a log through a `{:?}` of the config.
impl fmt::Debug for SolanaGrpcListenerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SolanaGrpcListenerConfig")
            .field("grpc_url", &self.grpc_url)
            .field("x_token", &self.x_token.as_ref().map(|_| "[REDACTED]"))
            .field("program_id", &self.program_id)
            .field("chain_id", &self.chain_id)
            .field(
                "dependent_ops_max_per_chain",
                &self.dependent_ops_max_per_chain,
            )
            .finish()
    }
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

/// Connects, subscribes, and ingests until `cancel` fires. Reconnects with a
/// `from_slot` cursor on stream errors; inserts are idempotent so replay is safe. When the
/// stream can no longer replay from the checkpoint, catches up from `archive` first.
pub async fn run(
    db: &Database,
    archive: &RpcClient,
    config: &SolanaGrpcListenerConfig,
    start: StartPosition,
    cancel: CancellationToken,
) -> Result<()> {
    info!(
        program_id = %config.program_id,
        grpc_url = %config.grpc_url,
        "Starting Solana host listener (Yellowstone gRPC transport)"
    );
    metrics::record_start(config.chain_id, &start);
    let mut progress = IngestionProgress::from(start);

    loop {
        if cancel.is_cancelled() {
            return Ok(());
        }
        let start = progress.subscription_start();
        let err = match subscribe_loop(
            db,
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
                    db,
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
        match err.downcast::<FatalListenerError>() {
            Ok(fatal) => {
                let err = fatal.into_inner();
                error!(error = format!("{err:#}"), checkpoint = ?progress.applied, retry_cursor = ?progress.retry, "gRPC listener stopped on fail-closed ingestion error");
                return Err(err);
            }
            Err(err) => {
                error!(error = format!("{err:#}"), checkpoint = ?progress.applied, retry_cursor = ?progress.retry, "ingestion interrupted; resuming inclusively from the checkpoint");
                metrics::inc_reconnects(config.chain_id);
                tokio::select! {
                    _ = cancel.cancelled() => return Ok(()),
                    _ = tokio::time::sleep(Duration::from_secs(2)) => {}
                }
            }
        }
    }
}

/// Dynamic execution indices follow the host's canonical fixed account prefix.
const FHE_EXECUTE_REMAINING_BASE: usize = zama_host::FHE_EXECUTE_FIXED_ACCOUNTS;

fn fhe_execute_dynamic_account(
    accounts: &[[u8; 32]],
    remaining_index: u8,
) -> Option<[u8; 32]> {
    accounts
        .get(FHE_EXECUTE_REMAINING_BASE + usize::from(remaining_index))
        .copied()
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
) -> Result<Vec<crate::solana_reconstruct::DecodedInstruction>> {
    resolved
        .into_iter()
        .filter(|instruction| instruction.program_id == program.to_bytes())
        .map(|instruction| {
            Ok(crate::solana_reconstruct::DecodedInstruction {
                program: bs58::encode(instruction.program_id).into_string(),
                data: instruction.data,
                accounts: instruction.accounts,
                top_level_index: u32::try_from(instruction.top_level_index)
                    .context("top-level instruction index exceeds u32")?,
                is_inner: instruction.stack_height != 1,
            })
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
    db: &Database,
    config: &SolanaGrpcListenerConfig,
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

    loop {
        tokio::select! {
            _ = cancel.cancelled() => return Ok(StreamEnd::Cancelled),
            // Block meta is emitted for every produced slot, including slots with zero
            // matching transactions. Prolonged silence therefore means the stream stalled.
            msg = tokio::time::timeout(Duration::from_secs(30), stream.message()) => {
                let msg = msg.map_err(|_| anyhow!("grpc stream idle for 30s; reconnecting"))?;
                let msg = match msg {
                    Ok(message) => message,
                    Err(status) if is_resume && is_replay_window_passed(&status) => {
                        return Ok(StreamEnd::ReplayWindowPassed(status));
                    }
                    Err(status) if is_resume && is_replay_unsupported(&status) => {
                        return Err(FatalListenerError::new(anyhow!(
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
                        let slot = update.slot;
                        update
                            .transaction
                            .ok_or_else(|| anyhow!("transaction update in slot {slot} has no transaction"))
                            .and_then(|info| prepare_transaction(info, &config.program_id))
                            .and_then(|prepared| match prepared {
                                Some(transaction) => validator.transaction(slot, transaction),
                                None => Ok(()),
                            })
                            .map_err(|error| {
                                FatalListenerError::new(error.context(
                                    "validate Solana transaction",
                                ))
                            })?;
                    }
                    Some(UpdateOneof::BlockMeta(meta)) => {
                        let decision = validator.block_meta(meta).map_err(|error| {
                            FatalListenerError::new(error.context(
                                "validate sealed Solana block",
                            ))
                        })?;
                        if let SealDecision::Process(block, transactions) = decision {
                            let prepared = PreparedBlock {
                                block,
                                transactions,
                            };
                            if !apply_prepared_block(
                                db,
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
                    }
                    Some(UpdateOneof::Ping(_)) => debug!("grpc ping"),
                    _ => {}
                }
            }
        }
    }
}

/// A sealed block with its host transactions, in the transport-neutral form reconstruction reads.
#[derive(Debug)]
struct PreparedBlock {
    block: SealedBlock,
    transactions: Vec<PreparedTransaction>,
}

/// A successful transaction reduced to its host program's instructions.
#[derive(Debug)]
pub(crate) struct PreparedTransaction {
    pub(crate) signature: Signature,
    pub(crate) index: u64,
    pub(crate) instructions: Vec<crate::solana_reconstruct::DecodedInstruction>,
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
            resolve_transaction_instructions(message, meta)?,
            program,
        )?,
    }))
}

/// Applies one prepared block and advances the checkpoint. Returns `false` when cancelled.
async fn apply_prepared_block(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    prepared: &PreparedBlock,
    progress: &mut IngestionProgress,
    cancel: &CancellationToken,
) -> Result<bool> {
    progress.observe_unapplied(prepared.block.checkpoint());
    match ingest_block(db, config, prepared, cancel).await {
        Ok(BlockIngestOutcome::Complete) => {
            progress.commit(prepared.block.checkpoint());
            metrics::record_applied(config.chain_id, &prepared.block);
            Ok(true)
        }
        Ok(BlockIngestOutcome::Cancelled) => Ok(false),
        Err(err) if err.kind() == IngestFailureKind::Retryable => Err(err
            .into_error()
            .context("retryable sealed block ingest failure")),
        Err(err) => Err(FatalListenerError::new(
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
    use super::{
        is_replay_unsupported, is_replay_window_passed, BlockCheckpoint,
        IngestionProgress,
    };
    use crate::solana_grpc_listener::StartPosition;
    use crate::solana_grpc_source::{BlockValidator, SealDecision};
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
        let update = SubscribeUpdateBlockMeta {
            slot: 5,
            blockhash: bs58::encode([5; 32]).into_string(),
            parent_slot: 4,
            parent_blockhash: bs58::encode([4; 32]).into_string(),
            ..Default::default()
        };
        // A tip start skips its first slot, so slot 5 is the first it applies.
        let mut first_validator = BlockValidator::new(StartPosition::Tip);
        assert!(matches!(
            first_validator
                .block_meta(SubscribeUpdateBlockMeta {
                    slot: 4,
                    blockhash: bs58::encode([4; 32]).into_string(),
                    parent_slot: 3,
                    parent_blockhash: bs58::encode([3; 32]).into_string(),
                    ..Default::default()
                })
                .unwrap(),
            SealDecision::Skip
        ));
        let SealDecision::Process(first, _) =
            first_validator.block_meta(update.clone()).unwrap()
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
        let SealDecision::Process(retried, _) =
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
        let update = SubscribeUpdateBlockMeta {
            slot: 5,
            blockhash: bs58::encode([5; 32]).into_string(),
            parent_slot: 4,
            parent_blockhash: bs58::encode([4; 32]).into_string(),
            ..Default::default()
        };
        let mut validator = BlockValidator::new(progress.subscription_start());
        let SealDecision::Process(block, _) =
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
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    prepared: &PreparedBlock,
    cancel: &CancellationToken,
) -> std::result::Result<BlockIngestOutcome, IngestFailure> {
    let result = tokio::select! {
        _ = cancel.cancelled() => return Ok(BlockIngestOutcome::Cancelled),
        result = tokio::time::timeout(
            SOLANA_GRPC_INGEST_TIMEOUT,
            apply_block(db, config, prepared),
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

/// Applies one sealed block in one database transaction: every covered
/// transaction's compute rows, the leaves the block sealed, and the checkpoint.
async fn apply_block(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    prepared: &PreparedBlock,
) -> std::result::Result<(), IngestFailure> {
    let sealed_block = &prepared.block;
    let mut reconstructed = Vec::new();
    for transaction in &prepared.transactions {
        let outcome = reconstruct_records_for_insert(
            config,
            &transaction.instructions,
            sealed_block.slot,
        )
        .map_err(|err| {
            IngestFailure::fatal(err).context("reconstruct Solana host records")
        })?;
        if let ReconstructionOutcome::Complete(records) = outcome {
            reconstructed.push((transaction, records));
        }
    }

    // Same cutover/schema-reset write boundary as the EVM ingest path: a retired stack takes no
    // more rows, so drop the block instead of failing the subscription.
    let Some(mut db_tx) = db
        .new_transaction()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("open db tx"))?
    else {
        info!(
            slot = sealed_block.slot,
            "Cutover completed - skipping Solana block on retired stack"
        );
        return Ok(());
    };

    let mut records_by_transaction = Vec::new();
    let mut leaf_sources = Vec::new();
    let mut held_back = Vec::new();
    let mut check_failures = Vec::new();
    for (transaction, records) in reconstructed {
        let transaction_id = TransactionId::from(transaction.signature);
        records_by_transaction.push((transaction_id, records.records));
        if !records.leaf_sources.is_empty() {
            leaf_sources.push(TransactionStoreWrites {
                transaction_index: transaction.index,
                sources: records.leaf_sources,
            });
        }
        for failure in records.check_failures {
            held_back.push(HeldBackComputation {
                transaction_id,
                output_handle: failure.mismatch.emitted,
                reason: failure.describe(sealed_block.slot),
            });
            check_failures.push((transaction.signature, failure));
        }
    }
    let stats = if records_by_transaction.is_empty() {
        SolanaIngestStats::default()
    } else {
        let block_timestamp =
            sealed_block_timestamp(sealed_block).ok_or_else(|| {
                IngestFailure::fatal(anyhow!(
                    "missing or invalid block time for slot {}",
                    sealed_block.slot
                ))
            })?;
        let block = SolanaBlockMeta {
            block_number: sealed_block.slot,
            block_timestamp,
            block_hash: sealed_block.block_hash,
            parent_hash: sealed_block.parent_block_hash,
        };
        insert_solana_block_records(
            db,
            &mut db_tx,
            records_by_transaction,
            block,
            config.dependent_ops_max_per_chain,
        )
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("insert_solana_block_records")
        })?
    };
    let held_rows = hold_back_computations(&mut db_tx, &held_back)
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("hold back computations")
        })?;
    if held_rows != held_back.len() as u64 {
        return Err(IngestFailure::fatal(anyhow!(
            "slot {}: {} held-back steps but {held_rows} computation rows",
            sealed_block.slot,
            held_back.len()
        )));
    }

    let mut touched: Vec<[u8; 32]> = leaf_sources
        .iter()
        .flat_map(|transaction| transaction.sources.iter())
        .map(|source| source.encrypted_store)
        .collect();
    touched.sort_unstable();
    touched.dedup();
    let existing = load_encrypted_store_histories(&mut db_tx, &touched)
        .await
        .map_err(|err| {
        IngestFailure::retryable(err).context("load encrypted store histories")
    })?;
    // The chain accepted every write; a write this record cannot follow means the
    // record diverged from chain state, and continuing would seal wrong leaves.
    let reduction =
        reduce_block_leaves(sealed_block.slot, &leaf_sources, existing)
            .map_err(|err| {
                IngestFailure::fatal(err).context("reduce Solana leaves")
            })?;
    // A replayed slot must reproduce its recorded leaves exactly; anything else means
    // the record and the chain disagree about history the proofs already cover.
    if !reduction.replayed.is_empty() {
        let recorded = load_block_leaves(
            &mut db_tx,
            sealed_block.slot,
            reduction.replayed.keys().copied(),
        )
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("load replayed Solana leaves")
        })?;
        if recorded != reduction.replayed {
            return Err(IngestFailure::fatal(anyhow!(
                "replayed slot {} does not reproduce its recorded leaves",
                sealed_block.slot
            )));
        }
    }
    store_block_leaves(&mut db_tx, sealed_block.slot, &reduction)
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("store Solana leaves")
        })?;
    store_checkpoint(
        &mut db_tx,
        &StoredCheckpoint {
            slot: sealed_block.slot,
            block_hash: sealed_block.block_hash,
        },
    )
    .await
    .map_err(|err| IngestFailure::retryable(err).context("store checkpoint"))?;
    db_tx
        .commit()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("commit db tx"))?;

    for (signature, failure) in &check_failures {
        error!(
            signature = %signature,
            "{}; the step is held back",
            failure.describe(sealed_block.slot)
        );
    }
    if !check_failures.is_empty() {
        metrics::add_handle_check_failures(
            config.chain_id,
            check_failures.len(),
        );
    }

    if stats.inserted_rows > 0 || !reduction.leaves.is_empty() {
        info!(
            slot = sealed_block.slot,
            tfhe_events = stats.tfhe_events,
            material_requests = stats.material_requests,
            inserted_rows = stats.inserted_rows,
            leaves = reduction.leaves.len(),
            encrypted_stores = reduction.states.len(),
            "ingested Solana host records (gRPC)"
        );
    }
    Ok(())
}

fn unix_to_pdt(ts: i64) -> Option<PrimitiveDateTime> {
    let dt = OffsetDateTime::from_unix_timestamp(ts).ok()?;
    Some(PrimitiveDateTime::new(dt.date(), dt.time()))
}

/// Geyser's block time is the bank's `Clock::unix_timestamp`, the value every handle in the
/// block was derived with.
fn sealed_block_timestamp(block: &SealedBlock) -> Option<PrimitiveDateTime> {
    block.block_time.and_then(unix_to_pdt)
}

/// One covered transaction, rebuilt off-chain: the compute rows to insert, the
/// leaf sources its host instructions sealed, in on-chain order, and the steps
/// whose emitted handle re-derivation did not reproduce.
#[derive(Debug, Default)]
struct ReconstructedTransaction {
    records: Vec<crate::solana_adapter::SolanaHostRecord>,
    leaf_sources: Vec<EncryptedStoreWrite>,
    check_failures: Vec<HandleCheckFailure>,
}

/// A [`HandleMismatch`] located in its transaction: `execution_index` counts the
/// transaction's `fhe_execute` invocations from zero.
#[derive(Debug)]
struct HandleCheckFailure {
    execution_index: usize,
    mismatch: HandleMismatch,
}

impl HandleCheckFailure {
    /// The held-back row's `error_message` and the alert's log line. The row's `transaction_id`
    /// already names the transaction; leaving the base58 signature out keeps the message from
    /// ever spelling the tfhe-worker's retry marker, which would make the hold-back retryable.
    fn describe(&self, slot: u64) -> String {
        format!(
            "solana handle check failed: slot {slot}, execution {}, step {}: emitted 0x{}, re-derived 0x{}",
            self.execution_index,
            self.mismatch.step_index,
            hex::encode(self.mismatch.emitted),
            hex::encode(self.mismatch.derived),
        )
    }
}

#[derive(Debug)]
enum ReconstructionOutcome {
    Complete(ReconstructedTransaction),
    NotCovered,
}

/// Rebuilds the ingestable record set off-chain from a transaction's instructions.
/// Covers `fhe_execute` (one op record per step, plus a material request and
/// history append for each state output), decoded from the same ordered
/// instruction list together with each execution's `FheExecutedEvent`.
fn reconstruct_records_for_insert(
    config: &SolanaGrpcListenerConfig,
    instructions: &[crate::solana_reconstruct::DecodedInstruction],
    slot: u64,
) -> Result<ReconstructionOutcome> {
    use crate::solana_adapter::{material_request, SolanaHostRecord};
    use crate::solana_reconstruct::{
        decode_fhe_execute_args, decode_fhe_executed_event,
        decode_make_store_handle_public, is_fhe_execute_instruction,
        is_make_store_handle_public_instruction, reconstruct_fhe_execute,
        MAKE_STATE_ENCRYPTED_STORE_INDEX,
    };

    // The deployment this listener follows, not the id the crate was compiled with: handles
    // hash the program id, so a listener built for one profile can still derive another's.
    let program_id = config.program_id;
    let host_program = program_id.to_string();
    let host_instructions = instructions
        .iter()
        .filter(|ix| ix.program == host_program)
        .collect::<Vec<_>>();
    for ix in &host_instructions {
        if is_fhe_execute_instruction(&ix.data)
            && decode_fhe_execute_args(&ix.data).is_none()
        {
            anyhow::bail!(
                "reconstruct: known fhe_execute discriminator has undecodable arguments in slot {slot}"
            );
        }
        if is_make_store_handle_public_instruction(&ix.data)
            && decode_make_store_handle_public(&ix.data).is_none()
        {
            anyhow::bail!(
                "reconstruct: known make_store_handle_public discriminator has undecodable arguments in slot {slot}"
            );
        }
    }
    let has_fhe_execute = host_instructions
        .iter()
        .any(|ix| is_fhe_execute_instruction(&ix.data));
    let has_state_public = host_instructions
        .iter()
        .any(|ix| is_make_store_handle_public_instruction(&ix.data));
    if !has_fhe_execute && !has_state_public {
        return Ok(ReconstructionOutcome::NotCovered);
    }

    let mut reconstructed = ReconstructedTransaction::default();
    let mut produced_in_tx = std::collections::HashSet::new();
    let mut execution_index = 0;

    for (instruction_index, ix) in instructions.iter().enumerate() {
        if ix.program != host_program {
            continue;
        }
        if let Some(execution) = decode_fhe_execute_args(&ix.data) {
            // The execution's own event CPI follows it, before the next execution. Only
            // this program can sign its event authority, so a host instruction carrying
            // the event tag was emitted by the host.
            let mut events = instructions[instruction_index + 1..]
                .iter()
                .take_while(|later| {
                    later.program != host_program
                        || !is_fhe_execute_instruction(&later.data)
                })
                .filter(|later| later.program == host_program)
                .filter_map(|later| decode_fhe_executed_event(&later.data));
            let (Some(event), None) = (events.next(), events.next()) else {
                anyhow::bail!(
                    "reconstruct: fhe_execute in slot {slot} is not followed by exactly one \
                     FheExecutedEvent of version {}",
                    zama_host::EVENT_VERSION
                );
            };
            let Some(steps) = reconstruct_fhe_execute(
                &execution,
                &event,
                program_id,
                config.chain_id,
                &mut produced_in_tx,
            ) else {
                anyhow::bail!(
                    "reconstruct: fhe_execute in slot {slot} and its FheExecutedEvent \
                     do not describe the same steps"
                );
            };
            reconstructed.check_failures.extend(
                steps.mismatches.into_iter().map(|mismatch| {
                    HandleCheckFailure {
                        execution_index,
                        mismatch,
                    }
                }),
            );
            execution_index += 1;
            reconstructed.records.extend(steps.records);
            for output in steps.store_outputs {
                let Some(encrypted_store) = fhe_execute_dynamic_account(
                    &ix.accounts,
                    output.store_index,
                ) else {
                    anyhow::bail!(
                        "reconstruct: fhe_execute state output \
                         out of range in slot {slot}; remaining_index={}, \
                         accounts={}, handle={}",
                        output.store_index,
                        ix.accounts.len(),
                        bs58::encode(output.handle).into_string()
                    );
                };
                reconstructed
                    .records
                    .push(SolanaHostRecord::MaterialRequest(material_request(
                        output.handle,
                    )));
                reconstructed.leaf_sources.push(EncryptedStoreWrite {
                    encrypted_store,
                    previous_leaf_count: output.previous_leaf_count,
                    handle: output.handle,
                    allowed_keys: output.allowed_keys,
                    make_public: output.make_public,
                });
            }
            continue;
        }
        if is_make_store_handle_public_instruction(&ix.data) {
            let Some((_key, handle, previous_leaf_count)) =
                decode_make_store_handle_public(&ix.data)
            else {
                anyhow::bail!(
                    "reconstruct: malformed make_store_handle_public instruction in slot {slot}"
                );
            };
            let Some(encrypted_store) =
                ix.accounts.get(MAKE_STATE_ENCRYPTED_STORE_INDEX).copied()
            else {
                anyhow::bail!(
                    "reconstruct: make_store_handle_public account index {MAKE_STATE_ENCRYPTED_STORE_INDEX} out of range in slot {slot}; accounts={}",
                    ix.accounts.len()
                );
            };
            reconstructed
                .records
                .push(SolanaHostRecord::MaterialRequest(material_request(
                    handle,
                )));
            reconstructed.leaf_sources.push(EncryptedStoreWrite {
                encrypted_store,
                previous_leaf_count,
                handle,
                allowed_keys: Vec::new(),
                make_public: true,
            });
        }
    }
    Ok(ReconstructionOutcome::Complete(reconstructed))
}

#[cfg(test)]
mod account_resolution_tests {
    use super::{resolve_transaction_instructions, validated_account_keys};
    use yellowstone_grpc_proto::prelude::{
        Message as TransactionMessage, TransactionError, TransactionStatusMeta,
    };

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
    use super::wire_fixtures::{app_transaction, Compiled, Transaction};
    use super::{prepare_transaction, MAX_DECODING_MESSAGE_SIZE};
    use crate::solana_grpc_listener::StartPosition;
    use crate::solana_grpc_source::{BlockValidator, SealDecision};
    use solana_sdk::pubkey::Pubkey;
    use yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, SubscribeUpdate,
        SubscribeUpdateBlockMeta, SubscribeUpdateTransaction,
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
        let host = ZAMA_HOST.parse::<Pubkey>().unwrap().to_bytes();
        Transaction {
            signature: [signature; 64],
            static_keys: vec![[1; 32], [8; 32], host],
            loaded_writable: vec![],
            loaded_readonly: vec![],
            top_level: vec![Compiled {
                program_id_index: 1,
                accounts: vec![0, 2],
                data: vec![1],
                stack_height: None,
            }],
            inner_groups: vec![(
                0,
                (1..INSTRUCTION_TRACE)
                    .map(|_| Compiled {
                        program_id_index: 1,
                        accounts: vec![2],
                        data: vec![0xAB; CPI_DATA],
                        stack_height: Some(2),
                    })
                    .collect(),
            )],
        }
    }

    fn meta(
        slot: u64,
        executed_transaction_count: u64,
    ) -> SubscribeUpdateBlockMeta {
        SubscribeUpdateBlockMeta {
            slot,
            blockhash: bs58::encode([slot as u8; 32]).into_string(),
            parent_slot: slot - 1,
            parent_blockhash: bs58::encode([slot as u8 - 1; 32]).into_string(),
            executed_transaction_count,
            ..Default::default()
        }
    }

    #[test]
    fn a_slot_past_the_decoding_limit_streams_in_bounded_messages() {
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

        // Slot 4 is the tip start's skipped first slot; slot 5 carries the junk, then one
        // host transaction.
        let mut validator = BlockValidator::new(StartPosition::Tip);
        assert!(matches!(
            validator.block_meta(meta(4, 0)).unwrap(),
            SealDecision::Skip
        ));
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

        let SealDecision::Process(block, transactions) =
            validator.block_meta(meta(5, host_index + 1)).unwrap()
        else {
            panic!("slot 5 is applied")
        };
        assert_eq!(block.slot, 5);
        assert_eq!(transactions.len(), junk_count + 1);
        let with_instructions: Vec<_> = transactions
            .iter()
            .filter(|transaction| !transaction.instructions.is_empty())
            .map(|transaction| transaction.index)
            .collect();
        assert_eq!(with_instructions, [host_index]);
    }
}

/// Transactions and configuration shared by the listener's tests and the `getBlock` tests.
#[cfg(test)]
mod test_support {
    use super::{
        reconstruct_records_for_insert, ReconstructedTransaction,
        ReconstructionOutcome, SolanaGrpcListenerConfig,
    };
    use anchor_lang::{AnchorSerialize, Discriminator};
    use std::collections::HashSet;
    use zama_host::state::{FheExecuteArgs, FheExecuteStep};
    use zama_host::{
        FheExecuteRandomSeed, FheExecutedEvent, HandleDerivationContext,
    };

    use crate::solana_reconstruct::{
        decode_fhe_execute_args, event_cpi_data, event_with_derived_results,
        DecodedInstruction,
    };

    // A valid pubkey that is not the compiled-in `zama_host::ID`: derivation must follow the
    // configured deployment.
    pub(super) const ZAMA_HOST: &str =
        "7DYCAhqwQSKqqL1h8V1XmY1BTcMWxrASQYKNMy87jeg3";
    pub(super) const STATE: [u8; 32] = [0x22; 32];

    pub(super) fn config() -> SolanaGrpcListenerConfig {
        SolanaGrpcListenerConfig {
            grpc_url: "http://127.0.0.1:1".to_owned(),
            x_token: None,
            program_id: ZAMA_HOST.parse().unwrap(),
            chain_id: zama_host::SOLANA_POC_CHAIN_ID,
            dependent_ops_max_per_chain: 0,
        }
    }

    pub(super) fn encoded_execution(args: FheExecuteArgs) -> Vec<u8> {
        let mut data =
            zama_host::instruction::FheExecute::DISCRIMINATOR.to_vec();
        args.serialize(&mut data).unwrap();
        data
    }

    pub(super) fn context() -> HandleDerivationContext {
        HandleDerivationContext {
            program_id: ZAMA_HOST.parse().unwrap(),
            chain_id: config().chain_id,
            previous_bank_hash: [0x44; 32],
            unix_timestamp: 1_700_000_000,
        }
    }

    pub(super) fn event_instruction(
        execution: &DecodedInstruction,
        event: &FheExecutedEvent,
    ) -> DecodedInstruction {
        DecodedInstruction {
            data: event_cpi_data(event),
            accounts: vec![],
            ..execution.clone()
        }
    }
    /// Follows every host `fhe_execute` with the event a host would emit for it.
    pub(super) fn with_events(
        instructions: impl IntoIterator<Item = DecodedInstruction>,
    ) -> Vec<DecodedInstruction> {
        let mut produced = HashSet::new();
        let mut transaction = Vec::new();
        for instruction in instructions {
            let execution = (instruction.program == ZAMA_HOST)
                .then(|| decode_fhe_execute_args(&instruction.data))
                .flatten();
            transaction.push(instruction.clone());
            if let Some(execution) = execution {
                let seeds = execution
                    .steps
                    .iter()
                    .enumerate()
                    .filter(|(_, step)| {
                        matches!(
                            step,
                            FheExecuteStep::Rand { .. }
                                | FheExecuteStep::RandBounded { .. }
                        )
                    })
                    .map(|(index, _)| FheExecuteRandomSeed {
                        step_index: index as u16,
                        seed: [7; 16],
                    })
                    .collect();
                let event = event_with_derived_results(
                    &execution,
                    &context(),
                    seeds,
                    &produced,
                );
                produced.extend(event.results.iter().copied());
                transaction.push(event_instruction(&instruction, &event));
            }
        }
        transaction
    }

    pub(super) fn reconstruct(
        instructions: &[DecodedInstruction],
    ) -> anyhow::Result<ReconstructedTransaction> {
        match reconstruct_records_for_insert(&config(), instructions, 42)? {
            ReconstructionOutcome::Complete(reconstructed) => Ok(reconstructed),
            other => panic!("expected a covered transaction, got {other:?}"),
        }
    }
}

#[cfg(test)]
mod fhe_execute_acl_tests {
    use super::test_support::{
        config, context, encoded_execution, event_instruction, reconstruct,
        with_events, STATE, ZAMA_HOST,
    };
    use super::{
        fhe_execute_dynamic_account, HandleCheckFailure,
        SolanaGrpcListenerConfig, FHE_EXECUTE_REMAINING_BASE,
    };
    use anchor_lang::{AnchorSerialize, Discriminator};
    use zama_host::state::{FheExecuteArgs, FheExecuteStep};

    use crate::database::solana_leaves::EncryptedStoreWrite;
    use crate::solana_reconstruct::{DecodedInstruction, HandleMismatch};

    #[test]
    fn debug_redacts_yellowstone_x_token() {
        let secret = "yellowstone-secret-token";
        let config = SolanaGrpcListenerConfig {
            x_token: Some(secret.to_owned()),
            ..config()
        };
        let rendered = format!("{config:?}");
        assert!(!rendered.contains(secret), "{rendered}");
        assert!(rendered.contains("[REDACTED]"), "{rendered}");
    }

    #[test]
    fn dynamic_account_index_is_relative_to_remaining_accounts() {
        let accounts: Vec<[u8; 32]> = (0..13).map(|n| [n; 32]).collect();
        assert_eq!(fhe_execute_dynamic_account(&accounts, 0), Some([11; 32]));
        assert_eq!(fhe_execute_dynamic_account(&accounts, 1), Some([12; 32]));
        assert_eq!(fhe_execute_dynamic_account(&accounts[..11], 0), None);
    }

    #[test]
    fn executed_event_is_paired_with_its_own_host_execution() {
        let execute = DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                effects: vec![],

                returned_results: Vec::new(),
                account_count: 0,
                dictionary: vec![],
                steps: vec![FheExecuteStep::Rand { fhe_type: 5 }],
            }),
            accounts: vec![],
            top_level_index: 0,
            is_inner: true,
        };
        let [_, event] = &with_events([execute.clone()])[..] else {
            panic!("one execution, one event")
        };
        let foreign = |instruction: &DecodedInstruction| DecodedInstruction {
            program: "foreign-program".to_owned(),
            ..instruction.clone()
        };

        // Another program's instructions sharing the discriminators are ignored.
        let covered = [
            execute.clone(),
            foreign(&execute),
            foreign(event),
            event.clone(),
        ];
        assert_eq!(reconstruct(&covered).unwrap().records.len(), 1);

        // The next host execution ends the search, and an execution has exactly one event.
        for unpaired in [
            vec![execute.clone(), execute.clone(), event.clone()],
            vec![execute.clone(), foreign(event)],
            vec![execute, event.clone(), event.clone()],
        ] {
            let error = reconstruct(&unpaired).unwrap_err().to_string();
            assert!(error.contains("exactly one FheExecutedEvent"), "{error}");
        }
    }

    #[test]
    fn check_failures_name_the_execution_and_keep_the_emitted_handle() {
        let execute = |plaintext| DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                effects: vec![],
                returned_results: Vec::new(),
                account_count: 0,
                dictionary: vec![],
                steps: vec![FheExecuteStep::TrivialEncrypt {
                    plaintext,
                    fhe_type: 5,
                }],
            }),
            accounts: vec![],
            top_level_index: 0,
            is_inner: true,
        };
        let mut instructions =
            with_events([execute([1; 32]), execute([2; 32])]);
        let mut event = crate::solana_reconstruct::decode_fhe_executed_event(
            &instructions[3].data,
        )
        .unwrap();
        let derived = event.results[0];
        event.results[0] = [0xEE; 32];
        instructions[3] = event_instruction(&instructions[2], &event);

        let reconstructed = reconstruct(&instructions).unwrap();
        let [HandleCheckFailure {
            execution_index: 1,
            mismatch,
        }] = &reconstructed.check_failures[..]
        else {
            panic!("{:?}", reconstructed.check_failures)
        };
        assert_eq!(
            mismatch,
            &HandleMismatch {
                step_index: 0,
                emitted: [0xEE; 32],
                derived,
            }
        );
        assert!(reconstructed.records.iter().any(|record| matches!(
            record,
            crate::solana_adapter::SolanaHostRecord::TrivialEncrypt(op)
                if op.result == [0xEE; 32]
        )));
    }

    #[test]
    fn store_slot_preimage_depends_on_prior_calls_in_the_reconstruction() {
        use crate::solana_adapter::SolanaHostRecord;
        use zama_host::{
            ExecutionResultRef, FheBinaryOpCode, FheExecuteEffect,
            FheExecuteOperand, SlotWrite,
        };
        let context = context();
        let handle =
            zama_host::computed_eval_trivial_handle([7; 32], 5, &context);
        let mut accounts = vec![[0; 32]; FHE_EXECUTE_REMAINING_BASE];
        accounts.push(STATE);
        let first = DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            accounts,
            top_level_index: 0,
            is_inner: true,
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: vec![[9; 32]],
                steps: vec![FheExecuteStep::TrivialEncrypt {
                    plaintext: [7; 32],
                    fhe_type: 5,
                }],
                effects: vec![FheExecuteEffect {
                    result: ExecutionResultRef {
                        step_index: 0,
                        output_index: 0,
                    },
                    store_index: 0,
                    previous_leaf_count: 0,
                    slot: Some(SlotWrite {
                        key_index: 0,
                        previous_handle_index: None,
                    }),
                    allow_indexes: vec![],
                    make_public: false,
                    grants: vec![],
                }],
                returned_results: vec![],
            }),
        };
        let second = DecodedInstruction {
            top_level_index: 1,
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: vec![handle, [9; 32], [0; 32]],
                steps: vec![FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::StoreSlot {
                        handle_index: 0,
                        key_index: 1,
                        store_index: 0,
                    },
                    rhs: FheExecuteOperand::Scalar { value_index: 2 },
                    output_fhe_type: 5,
                }],
                effects: vec![],
                returned_results: vec![],
            }),
            ..first.clone()
        };
        for (instructions, boundary) in
            [(vec![first, second.clone()], 0), (vec![second], 1)]
        {
            let rebuilt = reconstruct(&with_events(instructions)).unwrap();
            let result = rebuilt
                .records
                .iter()
                .find_map(|record| match record {
                    SolanaHostRecord::FheBinaryOp(op) => Some(op.result),
                    _ => None,
                })
                .unwrap();
            let mut mask = [0; 32];
            mask[31] = boundary;
            assert_eq!(
                result,
                zama_host::computed_eval_handle(
                    FheBinaryOpCode::Add,
                    handle,
                    [0; 32],
                    true,
                    5,
                    mask,
                    &context
                )
            );
        }
    }

    #[test]
    fn store_output_reconstructs_address_cursor_and_leaves() {
        let args = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 4,
                slot: None,
                allow_indexes: vec![0],
                make_public: true,
                grants: vec![],
            }],

            returned_results: Vec::new(),
            account_count: 1,
            dictionary: vec![[0x33; 32]],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7; 32],
                fhe_type: 5,
            }],
        };
        let mut accounts: Vec<[u8; 32]> = (0..12).map(|n| [n; 32]).collect();
        accounts[FHE_EXECUTE_REMAINING_BASE] = STATE;
        let reconstructed = reconstruct(&with_events([DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data: encoded_execution(args),
            accounts,
            top_level_index: 0,
            is_inner: true,
        }]))
        .unwrap();
        let handle = reconstructed.leaf_sources[0].handle;
        assert_eq!(
            reconstructed.leaf_sources,
            vec![EncryptedStoreWrite {
                encrypted_store: STATE,
                previous_leaf_count: 4,
                handle,
                allowed_keys: vec![[0x33; 32]],
                make_public: true,
            }]
        );
    }

    #[test]
    fn grants_only_output_reconstructs_without_decrypt_permissions() {
        let args = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 4,
                slot: None,
                allow_indexes: vec![],
                make_public: false,
                grants: vec![zama_host::ResultGrant {
                    consumer_store_index: 0,
                }],
            }],

            returned_results: Vec::new(),
            account_count: 2,
            dictionary: vec![],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7; 32],
                fhe_type: 5,
            }],
        };
        let mut accounts: Vec<[u8; 32]> = (0..13).map(|n| [n; 32]).collect();
        accounts[FHE_EXECUTE_REMAINING_BASE] = STATE;
        let reconstructed = reconstruct(&with_events([DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data: encoded_execution(args),
            accounts,
            top_level_index: 0,
            is_inner: true,
        }]))
        .unwrap();
        assert!(reconstructed.leaf_sources.is_empty());
        assert_eq!(reconstructed.records.len(), 1);
    }

    #[test]
    fn store_output_with_missing_account_fails_ingest() {
        let args = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 0,
                slot: None,
                allow_indexes: vec![],
                make_public: true,
                grants: vec![],
            }],

            returned_results: Vec::new(),
            account_count: 1,
            dictionary: vec![],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7; 32],
                fhe_type: 5,
            }],
        };
        let error = reconstruct(&with_events([DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data: encoded_execution(args),
            accounts: vec![[0; 32]; FHE_EXECUTE_REMAINING_BASE],
            top_level_index: 0,
            is_inner: false,
        }]))
        .unwrap_err();
        assert!(error.to_string().contains("state output out of range"));
    }

    #[test]
    fn state_public_instruction_reconstructs_exact_public_leaf() {
        let args = zama_host::instruction::MakeStoreHandlePublic {
            key: [0x11; 32],
            handle: [0x22; 32],
            previous_leaf_count: 8,
        };
        let mut data =
            zama_host::instruction::MakeStoreHandlePublic::DISCRIMINATOR
                .to_vec();
        args.serialize(&mut data).unwrap();
        let mut accounts = vec![[0; 32]; 6];
        accounts[2] = STATE;
        let instruction = DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data,
            accounts,
            top_level_index: 0,
            is_inner: true,
        };
        let reconstructed = reconstruct(&[instruction]).unwrap();
        assert_eq!(
            reconstructed.leaf_sources,
            vec![EncryptedStoreWrite {
                encrypted_store: STATE,
                previous_leaf_count: 8,
                handle: [0x22; 32],
                allowed_keys: vec![],
                make_public: true,
            }]
        );
    }
}

#[cfg(test)]
mod apply_block_tests {
    use super::test_support::{
        config, context, encoded_execution, event_instruction, with_events,
        STATE, ZAMA_HOST,
    };
    use super::FHE_EXECUTE_REMAINING_BASE;
    use super::{apply_block, PreparedBlock, PreparedTransaction};
    use crate::database::solana_leaves::load_checkpoint;
    use crate::database::tfhe_event_propagate::Database;
    use crate::solana_grpc_source::SealedBlock;
    use crate::solana_reconstruct::{
        decode_fhe_executed_event, DecodedInstruction,
    };
    use fhevm_engine_common::chain_id::ChainId;
    use serial_test::serial;
    use solana_sdk::signature::Signature;
    use sqlx::Row;
    use test_harness::instance::{setup_test_db, ImportMode};
    use zama_host::state::{
        FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand, FheExecuteStep,
    };

    /// The value added to [`two_steps`]'s first result.
    const SCALAR: [u8; 32] = [3; 32];

    /// The handle a tampered event emits for the first step instead of the derived one.
    const WRONG: [u8; 32] = [0xEE; 32];

    fn handle_check_failures() -> f64 {
        let label = config().chain_id.to_string();
        prometheus::gather()
            .iter()
            .find(|family| {
                family.name()
                    == "coprocessor_solana_host_listener_handle_check_failures_total"
            })
            .and_then(|family| {
                family
                    .get_metric()
                    .iter()
                    .find(|metric| {
                        metric.get_label().iter().any(|pair| pair.value() == label)
                    })
                    .map(|metric| metric.get_counter().value())
            })
            .unwrap_or(0.0)
    }

    fn execution(
        steps: Vec<FheExecuteStep>,
        dictionary: Vec<[u8; 32]>,
    ) -> DecodedInstruction {
        DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                effects: vec![],
                returned_results: vec![],
                account_count: 0,
                dictionary,
                steps,
            }),
            accounts: vec![],
            top_level_index: 0,
            is_inner: true,
        }
    }

    /// Stores step `step_index`'s result into [`STATE`], allowing the dictionary key at
    /// `key_index`: one historical-access leaf.
    fn storing(
        mut instruction: DecodedInstruction,
        step_index: u8,
        key_index: u8,
        previous_leaf_count: u64,
    ) -> DecodedInstruction {
        let mut args = crate::solana_reconstruct::decode_fhe_execute_args(
            &instruction.data,
        )
        .unwrap();
        args.account_count = 1;
        args.effects = vec![zama_host::FheExecuteEffect {
            result: zama_host::ExecutionResultRef {
                step_index,
                output_index: 0,
            },
            store_index: 0,
            previous_leaf_count,
            slot: None,
            allow_indexes: vec![key_index],
            make_public: false,
            grants: vec![],
        }];
        instruction.data = encoded_execution(args);
        instruction.accounts = vec![[0; 32]; FHE_EXECUTE_REMAINING_BASE];
        instruction.accounts.push(STATE);
        instruction
    }

    /// A trivial encryption of `plaintext` followed by its sum with [`SCALAR`], the first
    /// dictionary entry.
    fn two_steps(
        plaintext: [u8; 32],
        dictionary: Vec<[u8; 32]>,
    ) -> DecodedInstruction {
        assert_eq!(dictionary[0], SCALAR);
        execution(
            vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext,
                    fhe_type: 5,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                },
            ],
            dictionary,
        )
    }

    /// Makes the event of a [`two_steps`] execution emit [`WRONG`] for its first step, and the
    /// honest derivation of the sum from it. Returns the consumer's handle and the handle the
    /// first step really derives to. The consumer's operand was produced in the transaction, so
    /// its mask is zero.
    fn tamper(instructions: &mut [DecodedInstruction]) -> ([u8; 32], [u8; 32]) {
        let consumer = zama_host::computed_eval_handle(
            FheBinaryOpCode::Add,
            WRONG,
            SCALAR,
            true,
            5,
            [0; 32],
            &context(),
        );
        let mut event =
            decode_fhe_executed_event(&instructions[1].data).unwrap();
        let derived = event.results[0];
        event.results = vec![WRONG, consumer];
        instructions[1] = event_instruction(&instructions[0], &event);
        (consumer, derived)
    }

    fn sealed(
        slot: u64,
        block_hash: [u8; 32],
        parent_block_hash: [u8; 32],
    ) -> SealedBlock {
        SealedBlock {
            slot,
            block_hash,
            parent_slot: slot - 1,
            parent_block_hash,
            block_time: Some(1_700_000_000),
            block_height: Some(slot - 2),
            executed_transaction_count: 1,
        }
    }

    /// The repair: rewind the checkpoint and revert the rows past a slot with the operator
    /// scripts, then replay. The consumer the worker drained comes back as a fresh row, the
    /// replay reproduces the recorded leaves, and the checkpoint returns to the tip.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial(handle_check_failures)]
    async fn a_reverted_slot_replays_with_fresh_rows_and_its_recorded_leaves() {
        let key = [0x33; 32];
        let slot_41 = PreparedBlock {
            block: sealed(41, [0x41; 32], [0x40; 32]),
            transactions: vec![PreparedTransaction {
                signature: Signature::from([1; 64]),
                index: 0,
                instructions: with_events([storing(
                    execution(
                        vec![FheExecuteStep::TrivialEncrypt {
                            plaintext: [1; 32],
                            fhe_type: 5,
                        }],
                        vec![key],
                    ),
                    0,
                    0,
                    0,
                )]),
            }],
        };
        let mut tampered = with_events([storing(
            two_steps([2; 32], vec![SCALAR, key]),
            1,
            1,
            1,
        )]);
        let (consumer, _) = tamper(&mut tampered);
        let slot_42 = PreparedBlock {
            block: sealed(42, [0x42; 32], [0x41; 32]),
            transactions: vec![PreparedTransaction {
                signature: Signature::from([2; 64]),
                index: 0,
                instructions: tampered,
            }],
        };

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let chain_id = ChainId::from_canonical_u64(config().chain_id);
        let db = Database::new(&instance.db_url, chain_id, 100)
            .await
            .unwrap();
        let pool = db.pool().await;
        for block in [&slot_41, &slot_42] {
            apply_block(&db, &config(), block)
                .await
                .map_err(|failure| failure.into_error())
                .unwrap();
        }
        // The worker drained the consumer of the held step.
        sqlx::query("UPDATE computations SET is_error = true, error_message = 'drained' WHERE output_handle = $1")
            .bind(consumer.to_vec())
            .execute(&pool)
            .await
            .unwrap();
        let leaves = || async {
            sqlx::query("SELECT leaf_index, commitment, block_slot FROM solana_encrypted_state_leaves ORDER BY leaf_index")
                .fetch_all(&pool)
                .await
                .unwrap()
                .iter()
                .map(|row| {
                    (
                        row.get::<i64, _>("leaf_index"),
                        row.get::<Vec<u8>, _>("commitment"),
                        row.get::<i64, _>("block_slot"),
                    )
                })
                .collect::<Vec<_>>()
        };
        let recorded_leaves = leaves().await;
        assert_eq!(recorded_leaves.len(), 2);

        sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, 'solana', '') ON CONFLICT DO NOTHING")
            .bind(chain_id.as_i64())
            .execute(&pool)
            .await
            .unwrap();
        let revert = test_harness::db_utils::revert_coprocessor_db_state_sql(
            chain_id.as_i64(),
            41,
        );
        // A refused script leaves its session in the aborted transaction, as psql would
        // before exiting, so it gets a connection of its own.
        let mut session: sqlx::PgConnection =
            sqlx::Connection::connect(instance.db_url()).await.unwrap();
        let refused = sqlx::raw_sql(&revert)
            .execute(&mut session)
            .await
            .unwrap_err();
        drop(session);
        assert!(
            refused
                .to_string()
                .contains("rewind_solana_listener_checkpoint.sql"),
            "{refused}"
        );
        let rewind = include_str!(
            "../../db-migration/db-scripts/rewind_solana_listener_checkpoint.sql"
        )
        .lines()
        .filter(|line| !line.starts_with("\\set "))
        .collect::<Vec<_>>()
        .join("\n")
        .replace(":'slot'", "41")
        .replace(":'block_hash'", &format!("'{}'", hex::encode([0x41; 32])));
        sqlx::raw_sql(&rewind).execute(&pool).await.unwrap();
        sqlx::raw_sql(&revert).execute(&pool).await.unwrap();
        let checkpoint = load_checkpoint(&pool).await.unwrap().unwrap();
        assert_eq!((checkpoint.slot, checkpoint.block_hash), (41, [0x41; 32]));
        let reverted: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM computations WHERE block_number = 42",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(reverted, 0);

        apply_block(&db, &config(), &slot_42)
            .await
            .map_err(|failure| failure.into_error())
            .unwrap();
        let consumer_errored: bool = sqlx::query_scalar(
            "SELECT is_error FROM computations WHERE output_handle = $1",
        )
        .bind(consumer.to_vec())
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(
            !consumer_errored,
            "the replay re-inserts the drained consumer fresh"
        );
        assert_eq!(leaves().await, recorded_leaves);
        assert_eq!(load_checkpoint(&pool).await.unwrap().unwrap().slot, 42);
    }

    /// A block whose second transaction emits a wrong handle for its first step: that step
    /// is held back, its consumer and the other transaction are ingested, the checkpoint
    /// advances and the alert counter moves.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial(handle_check_failures)]
    async fn a_wrong_emitted_handle_holds_back_only_its_step() {
        let honest = with_events([execution(
            vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [1; 32],
                fhe_type: 5,
            }],
            vec![],
        )]);
        let mut tampered = with_events([two_steps([2; 32], vec![SCALAR])]);
        let (consumer, derived) = tamper(&mut tampered);

        let block = PreparedBlock {
            block: SealedBlock {
                executed_transaction_count: 2,
                ..sealed(42, [5; 32], [4; 32])
            },
            transactions: vec![
                PreparedTransaction {
                    signature: Signature::from([1; 64]),
                    index: 0,
                    instructions: honest,
                },
                PreparedTransaction {
                    signature: Signature::from([2; 64]),
                    index: 1,
                    instructions: tampered,
                },
            ],
        };
        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let db = Database::new(
            &instance.db_url,
            ChainId::from_canonical_u64(config().chain_id),
            100,
        )
        .await
        .unwrap();
        let failures_before = handle_check_failures();

        apply_block(&db, &config(), &block)
            .await
            .map_err(|failure| failure.into_error())
            .unwrap();

        let pool = db.pool().await;
        let rows = sqlx::query(
            "SELECT output_handle, transaction_id, is_error, error_message FROM computations",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        let mut held = Vec::new();
        let mut ingested = Vec::new();
        for row in &rows {
            let handle = row.get::<Vec<u8>, _>("output_handle");
            if row.get::<bool, _>("is_error") {
                held.push((
                    handle,
                    row.get::<Vec<u8>, _>("transaction_id"),
                    row.get::<Option<String>, _>("error_message").unwrap(),
                ));
            } else {
                ingested.push(handle);
            }
        }
        assert_eq!(
            held,
            vec![(
                WRONG.to_vec(),
                vec![2; 64],
                format!(
                    "solana handle check failed: slot 42, execution 0, step 0: emitted 0x{}, re-derived 0x{}",
                    hex::encode(WRONG),
                    hex::encode(derived)
                )
            )]
        );
        assert_eq!(ingested.len(), 2, "the honest step and the consumer");
        assert!(ingested.contains(&consumer.to_vec()));
        assert_eq!(load_checkpoint(&pool).await.unwrap().unwrap().slot, 42);
        assert_eq!(handle_check_failures() - failures_before, 1.0);
    }
}
