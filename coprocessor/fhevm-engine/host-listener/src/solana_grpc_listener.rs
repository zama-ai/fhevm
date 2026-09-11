//! Yellowstone gRPC reconstruction path for the Solana host listener.
//!
//! Two operational limits live here, stated in the listener's own docs and not only in the
//! decisions log (DD-025/DD-028):
//!
//! - **No reorg unwind.** Blocks are ingested at confirmed commitment and never rolled back;
//!   work scheduled from a minority fork is wasted, never reverted. This is safe only because
//!   scheduling is decoupled from authorization — the KMS re-checks live on-chain state before
//!   releasing any plaintext (INVARIANTS #31/#32).
//! - **Version pairing.** Handle re-derivation is byte-identical to the program because the
//!   listener links the program crate itself (INVARIANTS #28) — which silently assumes the
//!   deployed program and the running listener were built from the same revision. There is no
//!   runtime handshake; the operational rule is to deploy both from the same rev
//!   (INVARIANTS #33).
//!
//! Each sealed block is applied in one database transaction: its compute rows, the
//! leaves its writes sealed (`database::solana_leaves`) and the resume checkpoint.
//! A leaf handle is derived from the same slot context as the compute row that
//! produced it, so the two records are built from the same input and committed
//! together.

use crate::database::transaction_id::TransactionId;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use futures_util::stream::StreamExt;
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Channel;
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, Message as TransactionMessage,
    SubscribeRequest, SubscribeUpdateTransactionInfo, TransactionStatusMeta,
};
use yellowstone_grpc_proto::prost::Message as _;
use zama_solana_transaction::{
    CompiledInstruction as CanonicalCompiledInstruction,
    InnerInstructionGroup as CanonicalInnerInstructionGroup,
};

use crate::database::solana_leaves::{
    load_encrypted_store_histories, reduce_block_leaves, store_block_leaves,
    store_checkpoint, EncryptedStoreWrite, StoredCheckpoint,
    TransactionStoreWrites,
};
use crate::database::tfhe_event_propagate::Database;
use crate::solana_adapter::{
    insert_solana_block_records, SolanaBlockMeta, SolanaIngestStats,
};
use crate::solana_grpc_source::{
    build_subscribe_request, BlockValidator, SealDecision, SealedBlock,
};

const MAX_DECODING_MESSAGE_SIZE: usize = 64 * 1024 * 1024;
const MAX_PENDING_CONTEXT_BLOCKS: usize = 256;
// A single decoded gRPC message is capped at 64 MiB. Keeping the cumulative
// encoded size of queued blocks within the same bound prevents the 256-block
// count limit from multiplying the transport's worst-case allocation.
const MAX_PENDING_BLOCK_BYTES: usize = MAX_DECODING_MESSAGE_SIZE;
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
    /// Base58 zama-host program id whose instructions are reconstructed.
    pub program_id: String,
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

    fn commit(
        &mut self,
        checkpoint: BlockCheckpoint,
        next_unapplied: Option<BlockCheckpoint>,
    ) {
        self.applied = Some(checkpoint);
        self.retry = next_unapplied;
    }
}

/// Connects, subscribes, and ingests until `cancel` fires. Reconnects with a
/// `from_slot` cursor on stream errors; inserts are idempotent so replay is safe.
pub async fn run(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    start: StartPosition,
    cancel: CancellationToken,
) -> Result<()> {
    info!(
        program_id = %config.program_id,
        grpc_url = %config.grpc_url,
        "Starting Solana host listener (Yellowstone gRPC transport)"
    );
    let mut progress = IngestionProgress::from(start);

    loop {
        if cancel.is_cancelled() {
            return Ok(());
        }
        let start = progress.subscription_start();
        match subscribe_loop(db, config, start, &mut progress, &cancel).await {
            Ok(()) => return Ok(()), // cancelled
            Err(err) => match err.downcast::<FatalListenerError>() {
                Ok(fatal) => {
                    let err = fatal.into_inner();
                    error!(error = %err, checkpoint = ?progress.applied, retry_cursor = ?progress.retry, "gRPC listener stopped on fail-closed ingestion error");
                    return Err(err);
                }
                Err(err) => {
                    error!(error = %err, checkpoint = ?progress.applied, retry_cursor = ?progress.retry, "gRPC subscription dropped; reconnecting inclusively");
                    tokio::select! {
                        _ = cancel.cancelled() => return Ok(()),
                        _ = tokio::time::sleep(Duration::from_secs(2)) => {}
                    }
                }
            },
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

fn decode_transaction_instructions(
    message: &TransactionMessage,
    meta: &TransactionStatusMeta,
) -> Result<Vec<crate::solana_reconstruct::DecodedInstruction>> {
    resolve_transaction_instructions(message, meta)?
        .into_iter()
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

async fn subscribe_loop(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    start: StartPosition,
    progress: &mut IngestionProgress,
    cancel: &CancellationToken,
) -> Result<()> {
    let endpoint = Channel::from_shared(config.grpc_url.clone())
        .context("invalid grpc url")?;
    let channel = tokio::select! {
        _ = cancel.cancelled() => return Ok(()),
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
        _ = cancel.cancelled() => return Ok(()),
        result = client.subscribe(outbound) => result.context("subscribe")?,
    };
    let mut stream = response.into_inner();
    let mut pending_blocks = VecDeque::new();
    let mut pending_encoded_bytes = 0;

    loop {
        tokio::select! {
            _ = cancel.cancelled() => return Ok(()),
            // Sealed blocks are emitted for every produced slot, including slots with zero
            // matching transactions. Prolonged silence therefore means the stream stalled.
            msg = tokio::time::timeout(Duration::from_secs(30), stream.message()) => {
                let msg = msg.map_err(|_| anyhow!("grpc stream idle for 30s; reconnecting"))?;
                let msg = match msg {
                    Ok(message) => message,
                    Err(status) if is_resume && is_terminal_replay_status(&status) => {
                        return Err(FatalListenerError::new(anyhow!(
                            "inclusive Yellowstone replay unavailable: {status}"
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
                    Some(UpdateOneof::Account(acc)) => {
                        validator.observe_account(acc).map_err(|error| {
                            FatalListenerError::new(error.context(
                                "validate Solana sysvar update",
                            ))
                        })?;
                        drain_pending_blocks(
                            db,
                            config,
                            &mut validator,
                            &mut pending_blocks,
                            &mut pending_encoded_bytes,
                            progress,
                            cancel,
                        )
                        .await?;
                    }
                    Some(UpdateOneof::Block(block)) => {
                        let encoded_len = block.encoded_len();
                        let decision = validator.seal(block).map_err(|error| {
                            FatalListenerError::new(error.context(
                                "validate sealed Solana block",
                            ))
                        })?;
                        if let SealDecision::Process(block) = decision {
                            if pending_blocks.len()
                                == MAX_PENDING_CONTEXT_BLOCKS
                            {
                                return Err(FatalListenerError::new(anyhow!(
                                    "sealed Solana blocks exceeded {MAX_PENDING_CONTEXT_BLOCKS} pending context slots"
                                )).into());
                            }
                            let pending = prepare_block(config, block, encoded_len).map_err(|error| {
                                FatalListenerError::new(error.context(
                                    "prepare sealed Solana block",
                                ))
                            })?;
                            pending_encoded_bytes = checked_pending_bytes(
                                pending_encoded_bytes,
                                pending.encoded_len,
                            ).map_err(FatalListenerError::new)?;
                            progress.observe_unapplied(pending.block.checkpoint());
                            pending_blocks.push_back(pending);
                            drain_pending_blocks(
                                db,
                                config,
                                &mut validator,
                                &mut pending_blocks,
                                &mut pending_encoded_bytes,
                                progress,
                                cancel,
                            )
                            .await?;
                        }
                    }
                    Some(UpdateOneof::Ping(_)) => debug!("grpc ping"),
                    _ => {}
                }
            }
        }
    }
}

#[derive(Debug)]
struct PendingBlock {
    block: SealedBlock,
    transactions: Vec<PreparedTransaction>,
    requirement: ContextRequirement,
    matching_transaction_count: usize,
    encoded_len: usize,
}

#[derive(Debug)]
struct PreparedTransaction {
    info: SubscribeUpdateTransactionInfo,
    instructions: Vec<crate::solana_reconstruct::DecodedInstruction>,
    requirement: ContextRequirement,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ContextRequirement {
    slot_hashes: bool,
    clock: bool,
}

impl ContextRequirement {
    fn union(self, other: Self) -> Self {
        Self {
            slot_hashes: self.slot_hashes || other.slot_hashes,
            clock: self.clock || other.clock,
        }
    }

    fn is_satisfied_by(self, block: &SealedBlock) -> bool {
        (!self.slot_hashes || block.previous_bank_hash.is_some())
            && (!self.clock || block.clock_unix_timestamp.is_some())
    }
}

fn prepare_block(
    config: &SolanaGrpcListenerConfig,
    mut block: SealedBlock,
    encoded_len: usize,
) -> Result<PendingBlock> {
    let matching_transaction_count = block.transactions.len();
    let mut requirement = ContextRequirement::default();
    let mut transactions = Vec::new();
    for info in std::mem::take(&mut block.transactions) {
        let meta = info
            .meta
            .as_ref()
            .ok_or_else(|| anyhow!("transaction has no status meta"))?;
        if meta.err.is_some() || info.is_vote {
            continue;
        }
        let tx = info.transaction.as_ref().ok_or_else(|| {
            anyhow!("successful transaction has no transaction")
        })?;
        let message = tx
            .message
            .as_ref()
            .ok_or_else(|| anyhow!("successful transaction has no message"))?;
        let instructions = decode_transaction_instructions(message, meta)?;
        let transaction_requirement = transaction_context_requirement(
            config,
            &instructions,
            block.block_time.is_some(),
        );
        requirement = requirement.union(transaction_requirement);
        transactions.push(PreparedTransaction {
            info,
            instructions,
            requirement: transaction_requirement,
        });
    }
    Ok(PendingBlock {
        block,
        transactions,
        requirement,
        matching_transaction_count,
        encoded_len,
    })
}

fn transaction_context_requirement(
    config: &SolanaGrpcListenerConfig,
    instructions: &[crate::solana_reconstruct::DecodedInstruction],
    has_block_time: bool,
) -> ContextRequirement {
    use crate::solana_reconstruct::{
        is_fhe_execute_instruction, is_make_store_handle_public_instruction,
    };

    let mut requirement = ContextRequirement::default();
    for instruction in instructions
        .iter()
        .filter(|instruction| instruction.program == config.program_id)
    {
        if is_fhe_execute_instruction(&instruction.data) {
            requirement.slot_hashes = true;
            requirement.clock = true;
        } else if !has_block_time
            && is_make_store_handle_public_instruction(&instruction.data)
        {
            requirement.clock = true;
        }
    }
    requirement
}

fn checked_pending_bytes(current: usize, added: usize) -> Result<usize> {
    let total = current
        .checked_add(added)
        .ok_or_else(|| anyhow!("pending sealed block byte count overflow"))?;
    if total > MAX_PENDING_BLOCK_BYTES {
        anyhow::bail!(
            "sealed Solana blocks exceeded {MAX_PENDING_BLOCK_BYTES} pending encoded bytes"
        );
    }
    Ok(total)
}

async fn drain_pending_blocks(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    validator: &mut BlockValidator,
    pending_blocks: &mut VecDeque<PendingBlock>,
    pending_encoded_bytes: &mut usize,
    progress: &mut IngestionProgress,
    cancel: &CancellationToken,
) -> Result<()> {
    while let Some(front) = pending_blocks.front_mut() {
        validator.refresh_context(&mut front.block);
        if !front.requirement.is_satisfied_by(&front.block) {
            return Ok(());
        }

        let pending = pending_blocks
            .pop_front()
            .expect("front was present immediately before pop");
        *pending_encoded_bytes -= pending.encoded_len;
        match ingest_block(db, config, &pending, cancel).await {
            Ok(BlockIngestOutcome::Complete) => {
                validator.commit(
                    &pending.block,
                    pending.requirement.slot_hashes,
                    pending.requirement.clock,
                );
                progress.commit(
                    pending.block.checkpoint(),
                    pending_blocks
                        .front()
                        .map(|pending| pending.block.checkpoint()),
                );
            }
            Ok(BlockIngestOutcome::Cancelled) => return Ok(()),
            Err(err) if err.kind() == IngestFailureKind::Retryable => {
                return Err(err
                    .into_error()
                    .context("retryable sealed block ingest failure"));
            }
            Err(err) => {
                return Err(FatalListenerError::new(
                    err.into_error()
                        .context("fatal sealed block ingest failure"),
                )
                .into());
            }
        }
    }
    Ok(())
}

fn is_terminal_replay_status(status: &tonic::Status) -> bool {
    let message = status.message();
    message == "from_slot is not supported"
        || message.starts_with("broadcast from ")
            && message.contains(" is not available")
}

#[cfg(test)]
mod replay_status_tests {
    use super::{
        checked_pending_bytes, is_terminal_replay_status,
        sealed_block_timestamp, BlockCheckpoint, IngestionProgress,
        MAX_PENDING_BLOCK_BYTES,
    };
    use crate::solana_grpc_listener::StartPosition;
    use crate::solana_grpc_source::{
        BlockValidator, SealDecision, SealedBlock,
    };
    use yellowstone_grpc_proto::prelude::SubscribeUpdateBlock;

    #[test]
    fn classifies_replay_gaps_without_treating_transport_as_terminal() {
        for message in [
            "from_slot is not supported",
            "broadcast from 7 is not available, last available: 12",
        ] {
            assert!(is_terminal_replay_status(&tonic::Status::internal(
                message
            )));
        }
        assert!(!is_terminal_replay_status(&tonic::Status::unavailable(
            "connection reset"
        )));
        assert!(!is_terminal_replay_status(&tonic::Status::internal(
            "failed to send from_slot request"
        )));
    }

    #[test]
    fn retryable_first_block_replays_as_unapplied_then_commits() {
        let update = SubscribeUpdateBlock {
            slot: 5,
            blockhash: bs58::encode([5; 32]).into_string(),
            parent_slot: 4,
            parent_blockhash: bs58::encode([4; 32]).into_string(),
            ..Default::default()
        };
        let mut first_validator = BlockValidator::new(StartPosition::Tip);
        let SealDecision::Process(first) =
            first_validator.seal(update.clone()).unwrap()
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
        let SealDecision::Process(retried) =
            retry_validator.seal(update).unwrap()
        else {
            panic!()
        };
        retry_validator.commit(&retried, false, false);
        progress.commit(retried.checkpoint(), None);

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
        let update = SubscribeUpdateBlock {
            slot: 5,
            blockhash: bs58::encode([5; 32]).into_string(),
            parent_slot: 4,
            parent_blockhash: bs58::encode([4; 32]).into_string(),
            ..Default::default()
        };
        let mut validator = BlockValidator::new(progress.subscription_start());
        let SealDecision::Process(block) =
            validator.seal(update.clone()).unwrap()
        else {
            panic!("bootstrap block must be applied")
        };
        progress.commit(block.checkpoint(), None);
        let start = progress.subscription_start();
        assert_eq!(start, StartPosition::Resume(checkpoint));
        let mut restarted = BlockValidator::new(start);
        assert!(matches!(
            restarted.seal(update).unwrap(),
            SealDecision::Replay
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
                .seal(SubscribeUpdateBlock {
                    slot,
                    blockhash: bs58::encode(hash).into_string(),
                    parent_slot: 4,
                    parent_blockhash: bs58::encode([4; 32]).into_string(),
                    ..Default::default()
                })
                .is_err());
        }
    }

    #[test]
    fn pending_byte_budget_fails_without_large_allocations() {
        assert_eq!(
            checked_pending_bytes(MAX_PENDING_BLOCK_BYTES - 1, 1).unwrap(),
            MAX_PENDING_BLOCK_BYTES
        );
        assert!(checked_pending_bytes(MAX_PENDING_BLOCK_BYTES - 1, 2).is_err());
    }

    #[test]
    fn clock_timestamp_is_the_block_time_fallback() {
        let block = SealedBlock {
            slot: 5,
            block_hash: [5; 32],
            parent_slot: 4,
            parent_block_hash: [4; 32],
            block_time: None,
            block_height: None,
            executed_transaction_count: 0,
            transactions: vec![],
            previous_bank_hash: None,
            clock_unix_timestamp: Some(1_700_000_000),
        };

        assert_eq!(
            sealed_block_timestamp(&block),
            super::unix_to_pdt(1_700_000_000)
        );
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
    pending: &PendingBlock,
    cancel: &CancellationToken,
) -> std::result::Result<BlockIngestOutcome, IngestFailure> {
    for transaction in &pending.transactions {
        if !transaction.requirement.is_satisfied_by(&pending.block) {
            return Err(IngestFailure::fatal(anyhow!(
                "missing required Solana reconstruction context for transaction {} in slot {}",
                transaction.info.index,
                pending.block.slot
            )));
        }
    }
    let result = tokio::select! {
        _ = cancel.cancelled() => return Ok(BlockIngestOutcome::Cancelled),
        result = tokio::time::timeout(
            SOLANA_GRPC_INGEST_TIMEOUT,
            apply_block(db, config, pending),
        ) => result,
    };
    match result {
        Ok(result) => result?,
        Err(_) => {
            return Err(IngestFailure::retryable(anyhow!(
                "timed out ingesting Solana block in slot {}",
                pending.block.slot
            )))
        }
    }
    info!(
        slot = pending.block.slot,
        parent_slot = pending.block.parent_slot,
        block_height = ?pending.block.block_height,
        executed_transaction_count = pending.block.executed_transaction_count,
        matching_transaction_count = pending.matching_transaction_count,
        "ingested sealed Solana block"
    );
    Ok(BlockIngestOutcome::Complete)
}

/// Applies one sealed block in one database transaction: every covered
/// transaction's compute rows, the leaves the block sealed, and the checkpoint.
async fn apply_block(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    pending: &PendingBlock,
) -> std::result::Result<(), IngestFailure> {
    let sealed_block = &pending.block;
    let slot_bank_hash = sealed_block
        .previous_bank_hash
        .map(|hash| HashMap::from([(sealed_block.slot, hash)]))
        .unwrap_or_default();
    let slot_clock_ts = sealed_block
        .clock_unix_timestamp
        .map(|timestamp| HashMap::from([(sealed_block.slot, timestamp)]))
        .unwrap_or_default();

    let mut reconstructed = Vec::new();
    for transaction in &pending.transactions {
        let outcome = reconstruct_records_for_insert(
            config,
            &transaction.instructions,
            sealed_block.slot,
            &slot_bank_hash,
            &slot_clock_ts,
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
    for (transaction, records) in reconstructed {
        records_by_transaction.push((
            TransactionId::SolanaSignature(
                solana_sdk::signature::Signature::try_from(
                    transaction.info.signature.as_slice(),
                )
                .map_err(|err| {
                    IngestFailure::fatal(err)
                        .context("invalid Solana signature")
                })?,
            ),
            records.records,
        ));
        if !records.leaf_sources.is_empty() {
            leaf_sources.push(TransactionStoreWrites {
                transaction_index: transaction.info.index,
                sources: records.leaf_sources,
            });
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
        reduce_block_leaves(&leaf_sources, existing).map_err(|err| {
            IngestFailure::fatal(err).context("reduce Solana leaves")
        })?;
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

fn sealed_block_timestamp(block: &SealedBlock) -> Option<PrimitiveDateTime> {
    block
        .block_time
        .or(block.clock_unix_timestamp)
        .and_then(unix_to_pdt)
}

/// Builds the handle-derivation context for `slot` from the streamed sysvars,
/// Returns `None` until both the Clock and SlotHashes value for the slot have been cached.
fn reconstruct_context(
    config: &SolanaGrpcListenerConfig,
    slot: u64,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
) -> Option<crate::solana_reconstruct::ReconstructContext> {
    let unix_timestamp = slot_clock_ts.get(&slot).copied()?;
    let previous_bank_hash = slot_bank_hash.get(&slot).copied()?;
    Some(crate::solana_reconstruct::ReconstructContext {
        chain_id: config.chain_id,
        previous_bank_hash,
        unix_timestamp,
    })
}

/// One covered transaction, rebuilt off-chain: the compute rows to insert and the
/// leaf sources its host instructions sealed, in on-chain order.
#[derive(Debug, Default)]
struct ReconstructedTransaction {
    records: Vec<crate::solana_adapter::SolanaHostRecord>,
    leaf_sources: Vec<EncryptedStoreWrite>,
}

#[derive(Debug)]
enum ReconstructionOutcome {
    Complete(ReconstructedTransaction),
    NotCovered,
}

/// Rebuilds the ingestable record set off-chain from a transaction's instructions.
/// Covers `fhe_execute` (one op record per step, plus a material request and
/// history append for each state output), decoded from the same ordered
/// instruction list.
fn reconstruct_records_for_insert(
    config: &SolanaGrpcListenerConfig,
    instructions: &[crate::solana_reconstruct::DecodedInstruction],
    slot: u64,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
) -> Result<ReconstructionOutcome> {
    use crate::solana_adapter::{material_request, SolanaHostRecord};
    use crate::solana_reconstruct::{
        decode_fhe_execute_args, decode_fhe_execute_random_seeds_event,
        decode_make_store_handle_public, is_fhe_execute_instruction,
        is_make_store_handle_public_instruction, reconstruct_fhe_execute,
        MAKE_STATE_ENCRYPTED_STORE_INDEX,
    };

    let host_instructions = instructions
        .iter()
        .filter(|ix| ix.program == config.program_id)
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

    let ctx = reconstruct_context(config, slot, slot_bank_hash, slot_clock_ts);
    if has_fhe_execute && ctx.is_none() {
        anyhow::bail!(
            "reconstruct: missing slot derivation context for covered fhe_execute in slot {slot}"
        );
    }

    let mut reconstructed = ReconstructedTransaction::default();
    let mut produced_in_tx = std::collections::HashSet::new();

    for (instruction_index, ix) in instructions.iter().enumerate() {
        if ix.program != config.program_id {
            continue;
        }
        if let Some(execution) = decode_fhe_execute_args(&ix.data) {
            let ctx = ctx
                .as_ref()
                .expect("covered fhe_execute requires reconstruction context");
            let random_seeds = instructions[instruction_index + 1..]
                .iter()
                .take_while(|later| {
                    later.program != config.program_id
                        || !is_fhe_execute_instruction(&later.data)
                })
                .filter(|later| later.program == config.program_id)
                .find_map(|later| {
                    decode_fhe_execute_random_seeds_event(&later.data)
                })
                .map(|event| event.seeds)
                .unwrap_or_default();
            // Deterministic output handles are reconstructed from the operation
            // and operands. Random outputs use the host-signed seed batch.
            let Some(steps) = reconstruct_fhe_execute(
                &execution,
                &random_seeds,
                ctx,
                &mut produced_in_tx,
            ) else {
                anyhow::bail!(
                    "reconstruct: incomplete fhe_execute reconstruction in slot {slot}; \
                     malformed execution or missing handle context"
                );
            };
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
        CompiledInstruction, InnerInstruction, InnerInstructions,
        Message as TransactionMessage, TransactionError, TransactionStatusMeta,
    };

    mod shared_fixtures {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../solana/test-fixtures/transaction_decoding.rs"
        ));
    }

    use shared_fixtures::{
        fixture_key, transaction_decoding_fixtures, ExpectedInstruction,
        ExpectedOutcome,
    };

    #[test]
    fn rejects_malformed_account_key_length() {
        let err =
            validated_account_keys([&fixture_key(1).to_vec(), &vec![2; 31]])
                .expect_err("short account keys must fail closed");

        assert!(err.to_string().contains(
            "account key 1 has invalid length 31, expected 32 bytes"
        ));
    }

    #[test]
    fn shared_transaction_decoding_contract() {
        for fixture in transaction_decoding_fixtures() {
            let top_level: Vec<CompiledInstruction> = fixture
                .top_level
                .iter()
                .map(|instruction| CompiledInstruction {
                    program_id_index: instruction.program_id_index,
                    accounts: instruction.accounts.clone(),
                    data: instruction.data.clone(),
                })
                .collect();
            let inner_groups: Vec<InnerInstructions> = fixture
                .inner_groups
                .iter()
                .map(|group| InnerInstructions {
                    index: group.index,
                    instructions: group
                        .instructions
                        .iter()
                        .map(|instruction| InnerInstruction {
                            program_id_index: instruction.program_id_index,
                            accounts: instruction.accounts.clone(),
                            data: instruction.data.clone(),
                            stack_height: instruction.stack_height,
                        })
                        .collect(),
                })
                .collect();

            let message = TransactionMessage {
                account_keys: fixture
                    .static_account_tags
                    .iter()
                    .copied()
                    .map(|tag| fixture_key(tag).to_vec())
                    .collect(),
                instructions: top_level,
                ..Default::default()
            };
            let meta = TransactionStatusMeta {
                inner_instructions: inner_groups,
                loaded_writable_addresses: fixture
                    .loaded_writable_account_tags
                    .iter()
                    .copied()
                    .map(|tag| fixture_key(tag).to_vec())
                    .collect(),
                loaded_readonly_addresses: fixture
                    .loaded_readonly_account_tags
                    .iter()
                    .copied()
                    .map(|tag| fixture_key(tag).to_vec())
                    .collect(),
                ..Default::default()
            };

            let decoded = resolve_transaction_instructions(&message, &meta);
            match &fixture.expected {
                ExpectedOutcome::Accept { instructions } => {
                    let actual: Vec<ExpectedInstruction> = decoded
                        .unwrap_or_else(|error| {
                            panic!("{}: {error}", fixture.name)
                        })
                        .into_iter()
                        .map(|instruction| ExpectedInstruction {
                            program: instruction.program_id,
                            accounts: instruction.accounts,
                            data: instruction.data,
                            top_level_index: u32::try_from(
                                instruction.top_level_index,
                            )
                            .unwrap(),
                            stack_height: instruction.stack_height,
                        })
                        .collect();
                    let expected = instructions
                        .iter()
                        .map(|instruction| instruction.resolve())
                        .collect::<Vec<_>>();
                    assert_eq!(actual, expected, "{}", fixture.name);
                }
                ExpectedOutcome::Reject => {
                    assert!(decoded.is_err(), "{}", fixture.name);
                }
            }
        }
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
mod fhe_execute_acl_tests {
    use super::{
        fhe_execute_dynamic_account, reconstruct_records_for_insert,
        transaction_context_requirement, ReconstructionOutcome,
        SolanaGrpcListenerConfig, FHE_EXECUTE_REMAINING_BASE,
    };
    use anchor_lang::{AnchorSerialize, Discriminator};
    use std::collections::HashMap;
    use zama_host::state::{FheExecuteArgs, FheExecuteStep};

    use crate::database::solana_leaves::EncryptedStoreWrite;
    use crate::solana_reconstruct::DecodedInstruction;

    const ZAMA_HOST: &str = "ZamaHost11111111111111111111111111111111";
    const STATE: [u8; 32] = [0x22; 32];

    fn config() -> SolanaGrpcListenerConfig {
        SolanaGrpcListenerConfig {
            grpc_url: "http://127.0.0.1:1".to_owned(),
            x_token: None,
            program_id: ZAMA_HOST.to_owned(),
            chain_id: zama_host::SOLANA_POC_CHAIN_ID,
            dependent_ops_max_per_chain: 0,
        }
    }

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

    fn encoded_execution(args: FheExecuteArgs) -> Vec<u8> {
        let mut data =
            zama_host::instruction::FheExecute::DISCRIMINATOR.to_vec();
        args.serialize(&mut data).unwrap();
        data
    }

    #[test]
    fn dynamic_account_index_is_relative_to_remaining_accounts() {
        let accounts: Vec<[u8; 32]> = (0..13).map(|n| [n; 32]).collect();
        assert_eq!(fhe_execute_dynamic_account(&accounts, 0), Some([11; 32]));
        assert_eq!(fhe_execute_dynamic_account(&accounts, 1), Some([12; 32]));
        assert_eq!(fhe_execute_dynamic_account(&accounts[..11], 0), None);
    }

    #[test]
    fn random_seed_lookup_respects_program_discriminator_namespace() {
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
        let event = zama_host::FheExecuteRandomSeedsEvent {
            version: zama_host::EVENT_VERSION,
            seeds: vec![zama_host::FheExecuteRandomSeed {
                step_index: 0,
                seed: [7; 16],
            }],
        };
        let seed_event = DecodedInstruction {
            data: anchor_lang::event::EVENT_IX_TAG_LE
                .iter()
                .copied()
                .chain(anchor_lang::Event::data(&event))
                .collect(),
            ..execute.clone()
        };
        for same_program in [false, true] {
            let mut collision = execute.clone();
            if !same_program {
                collision.program = "foreign-program".to_owned();
            }
            let outcome = reconstruct_records_for_insert(
                &config(),
                &[execute.clone(), collision, seed_event.clone()],
                42,
                &HashMap::from([(42, [0x44; 32])]),
                &HashMap::from([(42, 1_700_000_000)]),
            );
            if same_program {
                assert!(outcome
                    .unwrap_err()
                    .to_string()
                    .contains("incomplete fhe_execute reconstruction"));
            } else {
                assert!(matches!(
                    outcome.unwrap(),
                    ReconstructionOutcome::Complete(_)
                ));
            }
        }
    }

    #[test]
    fn store_slot_preimage_depends_on_prior_calls_in_the_reconstruction() {
        use crate::solana_adapter::SolanaHostRecord;
        use zama_host::{
            ExecutionResultRef, FheBinaryOpCode, FheExecuteEffect,
            FheExecuteOperand, SlotWrite,
        };
        let context = zama_host::HandleDerivationContext {
            chain_id: config().chain_id,
            previous_bank_hash: [0x44; 32],
            unix_timestamp: 1_700_000_000,
        };
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
            let ReconstructionOutcome::Complete(rebuilt) =
                reconstruct_records_for_insert(
                    &config(),
                    &instructions,
                    42,
                    &HashMap::from([(42, context.previous_bank_hash)]),
                    &HashMap::from([(42, context.unix_timestamp)]),
                )
                .unwrap()
            else {
                panic!("covered execution")
            };
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
        let instructions = [DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data: encoded_execution(args),
            accounts,
            top_level_index: 0,
            is_inner: true,
        }];
        let outcome = reconstruct_records_for_insert(
            &config(),
            &instructions,
            42,
            &HashMap::from([(42, [0x44; 32])]),
            &HashMap::from([(42, 1_700_000_000)]),
        )
        .unwrap();
        let ReconstructionOutcome::Complete(reconstructed) = outcome else {
            panic!("expected covered transaction")
        };
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
        let outcome = reconstruct_records_for_insert(
            &config(),
            &[DecodedInstruction {
                program: ZAMA_HOST.to_owned(),
                data: encoded_execution(args),
                accounts,
                top_level_index: 0,
                is_inner: true,
            }],
            42,
            &HashMap::from([(42, [0x44; 32])]),
            &HashMap::from([(42, 1_700_000_000)]),
        )
        .unwrap();
        let ReconstructionOutcome::Complete(reconstructed) = outcome else {
            panic!("expected covered transaction")
        };
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
        let error = reconstruct_records_for_insert(
            &config(),
            &[DecodedInstruction {
                program: ZAMA_HOST.to_owned(),
                data: encoded_execution(args),
                accounts: vec![[0; 32]; FHE_EXECUTE_REMAINING_BASE],
                top_level_index: 0,
                is_inner: false,
            }],
            42,
            &HashMap::from([(42, [0x44; 32])]),
            &HashMap::from([(42, 1_700_000_000)]),
        )
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
        assert!(
            !transaction_context_requirement(
                &config(),
                std::slice::from_ref(&instruction),
                true,
            )
            .clock
        );
        assert!(
            transaction_context_requirement(
                &config(),
                std::slice::from_ref(&instruction),
                false,
            )
            .clock
        );
        let outcome = reconstruct_records_for_insert(
            &config(),
            &[instruction],
            42,
            &HashMap::new(),
            &HashMap::new(),
        )
        .unwrap();
        let ReconstructionOutcome::Complete(reconstructed) = outcome else {
            panic!("expected covered transaction")
        };
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
