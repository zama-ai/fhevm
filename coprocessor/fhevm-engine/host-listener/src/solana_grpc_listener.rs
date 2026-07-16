//! Yellowstone gRPC reconstruction path for the Solana host listener.

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
use zama_solana_transaction::{
    CompiledInstruction as CanonicalCompiledInstruction,
    InnerInstructionGroup as CanonicalInnerInstructionGroup,
};

use crate::database::tfhe_event_propagate::Database;
use crate::solana_adapter::{
    insert_solana_events, solana_transaction_id, SolanaBlockMeta,
};
use crate::solana_grpc_source::{
    build_subscribe_request, BlockValidator, SealDecision, SealedBlock,
};

const MAX_DECODING_MESSAGE_SIZE: usize = 64 * 1024 * 1024;
const MAX_PENDING_CONTEXT_BLOCKS: usize = 256;
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

#[derive(Clone, Debug)]
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockCheckpoint {
    pub slot: u64,
    pub block_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StartPosition {
    Tip,
    Resume(BlockCheckpoint),
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
    let mut applied_checkpoint = match start {
        StartPosition::Tip => None,
        StartPosition::Resume(checkpoint) => Some(checkpoint),
    };
    let mut retry_cursor = None;

    loop {
        if cancel.is_cancelled() {
            return Ok(());
        }
        let (resume, resume_is_applied) =
            subscription_start(&applied_checkpoint, &retry_cursor);
        match subscribe_loop(
            db,
            config,
            resume,
            resume_is_applied,
            &mut applied_checkpoint,
            &mut retry_cursor,
            &cancel,
        )
        .await
        {
            Ok(()) => return Ok(()), // cancelled
            Err(err) => match err.downcast::<FatalListenerError>() {
                Ok(fatal) => {
                    let err = fatal.into_inner();
                    error!(error = %err, checkpoint = ?applied_checkpoint, retry_cursor = ?retry_cursor, "gRPC listener stopped on fail-closed ingestion error");
                    return Err(err);
                }
                Err(err) => {
                    error!(error = %err, checkpoint = ?applied_checkpoint, retry_cursor = ?retry_cursor, "gRPC subscription dropped; reconnecting inclusively");
                    tokio::select! {
                        _ = cancel.cancelled() => return Ok(()),
                        _ = tokio::time::sleep(Duration::from_secs(2)) => {}
                    }
                }
            },
        }
    }
}

fn subscription_start(
    applied_checkpoint: &Option<BlockCheckpoint>,
    retry_cursor: &Option<BlockCheckpoint>,
) -> (Option<BlockCheckpoint>, bool) {
    match retry_cursor {
        Some(checkpoint) => (Some(checkpoint.clone()), false),
        None => (applied_checkpoint.clone(), applied_checkpoint.is_some()),
    }
}

/// Resolve a durable `fhe_eval` step's output `EncryptedValue` PDA from the
/// instruction's accounts.
///
/// `remaining_index` (the program's `output_encrypted_value_index`) is relative to
/// `remaining_accounts`, which follow the 10 named `fhe_eval` accounts — payer,
/// compute_subject, app_account_authority, host_config, system_program, hcu_authority,
/// hcu_block_meter, hcu_trusted_app_record, then `#[event_cpi]`'s event_authority +
/// program (see `FheEval` in fhe_eval.rs). The two optional HCU accounts are always
/// present in the account list (as program-id placeholders when `None`): the event_cpi
/// pair follows them, so they can never be truncated off the tail. Returns `None` when
/// the index is out of range; the caller treats that as a hard problem, since the
/// durable output would otherwise never request ciphertext material.
fn fhe_eval_durable_encrypted_value(
    accounts: &[[u8; 32]],
    remaining_index: u16,
) -> Option<[u8; 32]> {
    const FHE_EVAL_REMAINING_BASE: usize = 10;
    accounts
        .get(FHE_EVAL_REMAINING_BASE + remaining_index as usize)
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
    resume: Option<BlockCheckpoint>,
    resume_is_applied: bool,
    applied_checkpoint: &mut Option<BlockCheckpoint>,
    retry_cursor: &mut Option<BlockCheckpoint>,
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

    let is_resume = resume.is_some();
    let start = resume
        .map(StartPosition::Resume)
        .unwrap_or(StartPosition::Tip);
    let request = build_subscribe_request(&config.program_id, &start);
    let mut validator = BlockValidator::new(start, resume_is_applied);
    let outbound = futures_util::stream::once(async move { request })
        .chain(futures_util::stream::pending::<SubscribeRequest>());

    let response = tokio::select! {
        _ = cancel.cancelled() => return Ok(()),
        result = client.subscribe(outbound) => result.context("subscribe")?,
    };
    let mut stream = response.into_inner();
    let mut pending_blocks = VecDeque::new();

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
                            applied_checkpoint,
                            retry_cursor,
                            cancel,
                        )
                        .await?;
                    }
                    Some(UpdateOneof::Block(block)) => {
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
                            let requires_context = block_requires_reconstruction_context(config, &block)
                                .map_err(|error| {
                                    FatalListenerError::new(error.context(
                                        "inspect sealed Solana block",
                                    ))
                                })?;
                            if retry_cursor.is_none() {
                                *retry_cursor = Some(block.checkpoint());
                            }
                            pending_blocks.push_back(PendingBlock {
                                block,
                                requires_context,
                            });
                            drain_pending_blocks(
                                db,
                                config,
                                &mut validator,
                                &mut pending_blocks,
                                applied_checkpoint,
                                retry_cursor,
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
    requires_context: bool,
}

async fn drain_pending_blocks(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    validator: &mut BlockValidator,
    pending_blocks: &mut VecDeque<PendingBlock>,
    applied_checkpoint: &mut Option<BlockCheckpoint>,
    retry_cursor: &mut Option<BlockCheckpoint>,
    cancel: &CancellationToken,
) -> Result<()> {
    while let Some(front) = pending_blocks.front_mut() {
        validator.refresh_context(&mut front.block);
        if front.requires_context
            && !block_has_reconstruction_context(&front.block)
        {
            return Ok(());
        }

        let pending = pending_blocks
            .pop_front()
            .expect("front was present immediately before pop");
        match ingest_block(db, config, &pending.block, cancel).await {
            Ok(BlockIngestOutcome::Complete) => {
                validator.commit(&pending.block);
                *applied_checkpoint = Some(pending.block.checkpoint());
                *retry_cursor = pending_blocks
                    .front()
                    .map(|pending| pending.block.checkpoint());
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
        is_terminal_replay_status, sealed_block_timestamp, subscription_start,
        BlockCheckpoint,
    };
    use crate::solana_grpc_source::SealedBlock;

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
    fn first_unapplied_block_is_an_inclusive_retry_cursor() {
        let applied = None;
        let retry = Some(BlockCheckpoint {
            slot: 5,
            block_hash: [5; 32],
        });

        let (resume, resume_is_applied) = subscription_start(&applied, &retry);

        assert_eq!(resume, retry);
        assert!(!resume_is_applied);
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

fn block_requires_reconstruction_context(
    config: &SolanaGrpcListenerConfig,
    block: &SealedBlock,
) -> Result<bool> {
    for transaction in &block.transactions {
        let meta = transaction
            .meta
            .as_ref()
            .ok_or_else(|| anyhow!("transaction has no status meta"))?;
        if meta.err.is_some() || transaction.is_vote {
            continue;
        }
        let tx = transaction.transaction.as_ref().ok_or_else(|| {
            anyhow!("successful transaction has no transaction")
        })?;
        let message = tx
            .message
            .as_ref()
            .ok_or_else(|| anyhow!("successful transaction has no message"))?;
        let instructions = decode_transaction_instructions(message, meta)?;
        if instructions.iter().any(|instruction| {
            instruction.program == config.program_id
                && crate::solana_reconstruct::is_fhe_eval_instruction(
                    &instruction.data,
                )
        }) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn block_has_reconstruction_context(block: &SealedBlock) -> bool {
    block.previous_bank_hash.is_some() && block.clock_unix_timestamp.is_some()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlockIngestOutcome {
    Complete,
    Cancelled,
}

async fn ingest_block(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    block: &SealedBlock,
    cancel: &CancellationToken,
) -> std::result::Result<BlockIngestOutcome, IngestFailure> {
    for transaction in &block.transactions {
        let meta = transaction.meta.as_ref().ok_or_else(|| {
            IngestFailure::fatal(anyhow!("transaction has no status meta"))
        })?;
        if meta.err.is_some() || transaction.is_vote {
            continue;
        }
        let result = tokio::select! {
            _ = cancel.cancelled() => return Ok(BlockIngestOutcome::Cancelled),
            result = tokio::time::timeout(
                SOLANA_GRPC_INGEST_TIMEOUT,
                ingest_transaction(db, config, block, transaction),
            ) => result,
        };
        match result {
            Ok(result) => result?,
            Err(_) => {
                return Err(IngestFailure::retryable(anyhow!(
                    "timed out ingesting Solana transaction {} in slot {}",
                    transaction.index,
                    block.slot
                )))
            }
        }
    }
    info!(
        slot = block.slot,
        parent_slot = block.parent_slot,
        block_height = ?block.block_height,
        executed_transaction_count = block.executed_transaction_count,
        matching_transaction_count = block.transactions.len(),
        "ingested sealed Solana block"
    );
    Ok(BlockIngestOutcome::Complete)
}

async fn ingest_transaction(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    sealed_block: &SealedBlock,
    info: &SubscribeUpdateTransactionInfo,
) -> std::result::Result<(), IngestFailure> {
    let meta = info
        .meta
        .as_ref()
        .ok_or_else(|| IngestFailure::fatal(anyhow!("tx has no meta")))?;
    let tx = info.transaction.as_ref().ok_or_else(|| {
        IngestFailure::fatal(anyhow!("tx has no transaction"))
    })?;
    let message = tx
        .message
        .as_ref()
        .ok_or_else(|| IngestFailure::fatal(anyhow!("tx has no message")))?;

    // Top-level + inner instruction invocations with resolved accounts (a
    // zama-host instruction is called directly as top-level, or via CPI as inner
    // for token flows); scanned by the reconstruction shadow-compare and ingest.
    let all_instructions = decode_transaction_instructions(message, meta)
        .map_err(IngestFailure::fatal)?;

    let slot_bank_hash = sealed_block
        .previous_bank_hash
        .map(|hash| HashMap::from([(sealed_block.slot, hash)]))
        .unwrap_or_default();
    let slot_clock_ts = sealed_block
        .clock_unix_timestamp
        .map(|timestamp| HashMap::from([(sealed_block.slot, timestamp)]))
        .unwrap_or_default();

    let events = match reconstruct_events_for_insert(
        config,
        &all_instructions,
        sealed_block.slot,
        &slot_bank_hash,
        &slot_clock_ts,
    )
    .await
    .map_err(|err| {
        IngestFailure::fatal(err).context("reconstruct Solana host events")
    })? {
        ReconstructionOutcome::Complete(events) => events,
        ReconstructionOutcome::NotCovered => Vec::new(),
    };

    if events.is_empty() {
        return Ok(());
    }

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

    let transaction_id = solana_transaction_id(&info.signature);
    let mut db_tx = db
        .new_transaction()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("open db tx"))?;
    let stats =
        insert_solana_events(db, &mut db_tx, events, transaction_id, block)
            .await
            .map_err(|err| {
                IngestFailure::retryable(err).context("insert_solana_events")
            })?;
    db_tx
        .commit()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("commit db tx"))?;

    info!(
        slot = sealed_block.slot,
        transaction_index = info.index,
        signature = %bs58::encode(&info.signature).into_string(),
        tfhe_events = stats.tfhe_events,
        material_requests = stats.material_requests,
        inserted_rows = stats.inserted_rows,
        "ingested Solana host events (gRPC)"
    );
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

#[derive(Debug)]
enum ReconstructionOutcome {
    Complete(Vec<crate::solana_adapter::SolanaHostEvent>),
    NotCovered,
}

/// Rebuilds the ingestable event set off-chain from a transaction's instructions. Covers
/// `fhe_eval`: one compute event per step, plus a material request for each
/// `Durable` step's result handle.
///
/// `EncryptedValue` lifecycle instructions are decoded separately from the same
/// ordered instruction list and appended to the reconstructed event set. Missing
/// slot derivation context only suppresses `fhe_eval` recomputation; lifecycle
/// material requests do not need it.
async fn reconstruct_events_for_insert(
    config: &SolanaGrpcListenerConfig,
    instructions: &[crate::solana_reconstruct::DecodedInstruction],
    slot: u64,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
) -> Result<ReconstructionOutcome> {
    use crate::solana_adapter::{material_request, SolanaHostEvent};
    use crate::solana_reconstruct::{
        decode_encrypted_value_instruction, decode_fhe_eval_args,
        encrypted_value_account_index, encrypted_value_material_requests,
        is_encrypted_value_instruction, is_fhe_eval_instruction,
        reconstruct_fhe_eval_steps,
    };

    // compute_subject is the 2nd named fhe_eval account. (Durable EncryptedValue
    // PDAs live in remaining_accounts; resolved via
    // fhe_eval_durable_encrypted_value.)
    const COMPUTE_SUBJECT_INDEX: usize = 1;

    let host_instructions = instructions
        .iter()
        .filter(|ix| ix.program == config.program_id)
        .collect::<Vec<_>>();
    for ix in &host_instructions {
        if is_fhe_eval_instruction(&ix.data)
            && decode_fhe_eval_args(&ix.data).is_none()
        {
            anyhow::bail!(
                "reconstruct: known fhe_eval discriminator has undecodable arguments in slot {slot}"
            );
        }
        if is_encrypted_value_instruction(&ix.data)
            && decode_encrypted_value_instruction(&ix.data).is_none()
        {
            anyhow::bail!(
                "reconstruct: known EncryptedValue lifecycle discriminator has undecodable arguments in slot {slot}"
            );
        }
    }
    let has_lifecycle = host_instructions
        .iter()
        .any(|ix| is_encrypted_value_instruction(&ix.data));
    let has_fhe_eval = host_instructions
        .iter()
        .any(|ix| is_fhe_eval_instruction(&ix.data));
    if !has_lifecycle && !has_fhe_eval {
        return Ok(ReconstructionOutcome::NotCovered);
    }

    let ctx = reconstruct_context(config, slot, slot_bank_hash, slot_clock_ts);
    if has_fhe_eval && ctx.is_none() {
        anyhow::bail!(
            "reconstruct: missing slot derivation context for covered fhe_eval in slot {slot}"
        );
    }

    let mut events = Vec::new();

    for ix in instructions.iter() {
        if ix.program != config.program_id {
            continue;
        }
        if let Some(plan) = decode_fhe_eval_args(&ix.data) {
            let ctx = ctx
                .as_ref()
                .expect("covered fhe_eval requires reconstruction context");
            let subject = ix
                .accounts
                .get(COMPUTE_SUBJECT_INDEX)
                .copied()
                .unwrap_or([0u8; 32]);
            // Durable output handles recompute from the plan's value_key + block
            // entropy alone (DD-015): no lineage leaf count, no handle hints.
            let Some(steps) = reconstruct_fhe_eval_steps(&plan, subject, ctx)
            else {
                anyhow::bail!(
                    "reconstruct: incomplete fhe_eval reconstruction in slot {slot}; \
                     malformed plan or missing handle context"
                );
            };
            for step in steps {
                let handle = compute_result_handle(&step.event);
                let previous_handle = step.previous_handle;
                events.push(step.event);
                if let (Some(index), Some(handle)) =
                    (step.durable_encrypted_value_index, handle)
                {
                    if fhe_eval_durable_encrypted_value(&ix.accounts, index)
                        .is_some()
                    {
                        events.push(SolanaHostEvent::MaterialRequest(
                            material_request(handle),
                        ));
                        if let Some(previous_handle) = previous_handle {
                            events.push(SolanaHostEvent::MaterialRequest(
                                material_request(previous_handle),
                            ));
                        }
                    } else {
                        anyhow::bail!(
                            "reconstruct: fhe_eval durable bind encrypted_value \
                             out of range in slot {slot}; remaining_index={index}, \
                             accounts={}, handle={}",
                            ix.accounts.len(),
                            bs58::encode(handle).into_string()
                        );
                    }
                }
            }
            continue;
        }

        if let Some(instruction) = decode_encrypted_value_instruction(&ix.data)
        {
            let encrypted_value_index =
                encrypted_value_account_index(&instruction);
            if ix.accounts.get(encrypted_value_index).is_none() {
                anyhow::bail!(
                    "reconstruct: EncryptedValue lifecycle account index {encrypted_value_index} \
                     out of range in slot {slot}; accounts={}",
                    ix.accounts.len()
                );
            }
            events.extend(
                encrypted_value_material_requests(&instruction)
                    .into_iter()
                    .map(SolanaHostEvent::MaterialRequest),
            );
        }
    }
    Ok(ReconstructionOutcome::Complete(events))
}

fn compute_result_handle(
    event: &crate::solana_adapter::SolanaHostEvent,
) -> Option<[u8; 32]> {
    use crate::solana_adapter::SolanaHostEvent as E;
    match event {
        E::FheBinaryOp(e) => Some(e.result),
        E::FheTernaryOp(e) => Some(e.result),
        E::TrivialEncrypt(e) => Some(e.result),
        E::FheRand(e) => Some(e.result),
        E::FheRandBounded(e) => Some(e.result),
        E::FheUnaryOp(e) => Some(e.result),
        E::FheSum(e) => Some(e.result),
        E::FheIsIn(e) => Some(e.result),
        E::FheMulDiv(e) => Some(e.result),
        E::MaterialRequest(_) => None,
    }
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
mod fhe_eval_acl_tests {
    use super::{
        fhe_eval_durable_encrypted_value, reconstruct_events_for_insert,
        ReconstructionOutcome, SolanaGrpcListenerConfig,
    };
    use anchor_lang::AnchorSerialize;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use zama_host::state::{
        FheBinaryOpCode as PgmBinaryOpCode, FheEvalArgs, FheEvalOperand,
        FheEvalOutput, FheEvalStep,
    };

    use crate::database::tfhe_event_propagate::Handle;
    use crate::solana_adapter::SolanaHostEvent;
    use crate::solana_reconstruct::{
        AllowSubjectsArgs, DecodedInstruction, EncryptedValueSubjectGrant,
        ENCRYPTED_VALUE_ACCOUNT_INDEX,
    };
    use zama_host::state::AclSubjectEntry;

    fn acct(n: u8) -> [u8; 32] {
        [n; 32]
    }

    fn config() -> SolanaGrpcListenerConfig {
        SolanaGrpcListenerConfig {
            grpc_url: "http://127.0.0.1:1".to_owned(),
            x_token: None,
            program_id: ZAMA_HOST.to_owned(),
            chain_id: zama_host::SOLANA_POC_CHAIN_ID,
        }
    }

    const ZAMA_HOST: &str = "ZamaHost11111111111111111111111111111111";
    const ENCRYPTED_VALUE: [u8; 32] = [0x22; 32];
    const SUBJECT: [u8; 32] = [0x33; 32];
    const PREVIOUS_BANK_HASH: [u8; 32] = [0x44; 32];

    fn discriminator(name: &str) -> [u8; 8] {
        let digest = Sha256::digest(format!("global:{name}").as_bytes());
        let mut out = [0u8; 8];
        out.copy_from_slice(&digest[..8]);
        out
    }

    fn encode_instruction(name: &str, args: impl AnchorSerialize) -> Vec<u8> {
        let mut data = discriminator(name).to_vec();
        args.serialize(&mut data).expect("serialize instruction");
        data
    }

    fn decoded_ix(
        data: Vec<u8>,
        accounts: Vec<[u8; 32]>,
        top_level_index: u32,
        is_inner: bool,
    ) -> DecodedInstruction {
        DecodedInstruction {
            program: ZAMA_HOST.to_owned(),
            data,
            accounts,
            top_level_index,
            is_inner,
        }
    }

    fn encrypted_value_accounts() -> Vec<[u8; 32]> {
        let mut accounts = vec![[0u8; 32]; 6];
        accounts[ENCRYPTED_VALUE_ACCOUNT_INDEX] = ENCRYPTED_VALUE;
        accounts
    }

    fn fhe_eval_accounts() -> Vec<[u8; 32]> {
        // 10 named FheEval accounts (0..=9, incl. the three HCU accounts and the
        // event-cpi pair); the durable output EncryptedValue is remaining_accounts[0]
        // at absolute index 10 (FHE_EVAL_REMAINING_BASE).
        let mut accounts: Vec<[u8; 32]> = (0..11).map(acct).collect();
        accounts[1] = SUBJECT;
        accounts[10] = ENCRYPTED_VALUE;
        accounts
    }

    fn fhe_eval_accounts_with_deny_record() -> Vec<[u8; 32]> {
        // 10 named FheEval accounts plus Anchor event-cpi accounts (0..=9). The
        // optional deny record is remaining_accounts[0] (index 10), and the durable
        // output is remaining_accounts[1] (index 11).
        let mut accounts: Vec<[u8; 32]> = (0..12).map(acct).collect();
        accounts[1] = SUBJECT;
        accounts[11] = ENCRYPTED_VALUE;
        accounts
    }

    fn slot_context() -> (HashMap<u64, [u8; 32]>, HashMap<u64, i64>) {
        (
            HashMap::from([(42, PREVIOUS_BANK_HASH)]),
            HashMap::from([(42, 1_700_000_000)]),
        )
    }

    /// The durable `Add` output handle the fhe_eval fixtures produce, derived
    /// exactly as the program does: the base handle, no per-output binding
    /// (durable == instruction-local, matching EVM). Matches `config()`
    /// (the Solana PoC host chain id, `PREVIOUS_BANK_HASH`), slot 42's clock ts,
    /// op_index 0, scalar rhs.
    fn derived_add_output_handle() -> [u8; 32] {
        zama_host::state::computed_eval_handle(
            PgmBinaryOpCode::Add,
            [3; 32],
            [1; 32],
            true,
            5,
            zama_host::SOLANA_POC_CHAIN_ID,
            PREVIOUS_BANK_HASH,
            1_700_000_000,
            [1; 32],
            0,
        )
    }

    fn complete_events(outcome: ReconstructionOutcome) -> Vec<SolanaHostEvent> {
        match outcome {
            ReconstructionOutcome::Complete(events) => events,
            ReconstructionOutcome::NotCovered => {
                panic!("expected reconstruction to cover transaction")
            }
        }
    }

    #[test]
    fn durable_output_as_sole_remaining_account_resolves() {
        // The trivial-encrypt-eval shape: 10 named accounts (0..=9, including the three
        // HCU accounts and the event_cpi pair) + exactly one remaining account, the
        // durable output EncryptedValue account, at absolute index 10 (remaining_index 0).
        // A stale base (7, the pre-HCU count) read accounts.get(7) here — the
        // trusted-app-record placeholder, not the durable EncryptedValue account. This
        // pins the base at 10.
        let accounts: Vec<[u8; 32]> = (0..11).map(acct).collect();
        assert_eq!(
            fhe_eval_durable_encrypted_value(&accounts, 0),
            Some(acct(10))
        );
    }

    #[test]
    fn output_after_input_acl_records_resolves() {
        // A durable input EncryptedValue account at 10 and the durable output at 11
        // (remaining_index 1).
        let accounts: Vec<[u8; 32]> = (0..12).map(acct).collect();
        assert_eq!(
            fhe_eval_durable_encrypted_value(&accounts, 1),
            Some(acct(11))
        );
    }

    #[test]
    fn durable_output_after_optional_deny_record_resolves() {
        let accounts = fhe_eval_accounts_with_deny_record();
        assert_eq!(
            fhe_eval_durable_encrypted_value(&accounts, 1),
            Some(ENCRYPTED_VALUE)
        );
    }

    #[test]
    fn missing_remaining_account_returns_none() {
        // Only the 10 named accounts, no remaining: a durable bind here is a layout drift
        // the caller must surface, not silently drop.
        let accounts: Vec<[u8; 32]> = (0..10).map(acct).collect();
        assert_eq!(fhe_eval_durable_encrypted_value(&accounts, 0), None);
    }

    #[tokio::test]
    async fn direct_allow_subjects_schedules_no_material() {
        let allow_data = encode_instruction(
            "allow_subjects",
            AllowSubjectsArgs {
                subjects: vec![EncryptedValueSubjectGrant { subject: [7; 32] }],
            },
        );
        let instructions =
            vec![decoded_ix(allow_data, encrypted_value_accounts(), 0, false)];
        let events = complete_events(
            reconstruct_events_for_insert(
                &config(),
                &instructions,
                42,
                &HashMap::new(),
                &HashMap::new(),
            )
            .await
            .expect("reconstruction should return lifecycle events"),
        );

        assert!(events.is_empty());
    }

    #[tokio::test]
    async fn known_but_undecodable_instructions_fail_ingest() {
        let malformed = [
            discriminator("fhe_eval").to_vec(),
            discriminator("make_handle_public").to_vec(),
        ];
        let (slot_bank_hash, slot_clock_ts) = slot_context();

        for data in malformed {
            let err = reconstruct_events_for_insert(
                &config(),
                &[decoded_ix(data, encrypted_value_accounts(), 0, false)],
                42,
                &slot_bank_hash,
                &slot_clock_ts,
            )
            .await
            .expect_err(
                "known discriminator with malformed args must fail ingest",
            );
            assert!(
                err.to_string().contains("undecodable arguments"),
                "got: {err}"
            );
        }
    }

    #[tokio::test]
    async fn lifecycle_with_missing_accounts_fails_ingest() {
        let allow_data = encode_instruction(
            "allow_subjects",
            AllowSubjectsArgs {
                subjects: vec![EncryptedValueSubjectGrant { subject: [7; 32] }],
            },
        );
        let (slot_bank_hash, slot_clock_ts) = slot_context();
        let err = reconstruct_events_for_insert(
            &config(),
            &[decoded_ix(allow_data, vec![[0u8; 32]], 0, false)],
            42,
            &slot_bank_hash,
            &slot_clock_ts,
        )
        .await
        .expect_err("covered lifecycle instruction with missing accounts must fail ingest");

        assert!(err.to_string().contains("account index 2 out of range"));
    }

    /// A superseding durable `fhe_eval` output recomputes its handle directly
    /// from the plan's output material + block entropy (DD-015) — no raw update
    /// handle hint and no lineage leaf count. The
    /// reconstructed compute result and current/historical material requests
    /// must all come from the `fhe_eval` instruction itself.
    #[tokio::test]
    async fn superseding_fhe_eval_derives_output_handle_without_hint() {
        let expected = derived_add_output_handle();

        let plan = FheEvalArgs {
            context_id: [1; 32],
            steps: vec![FheEvalStep::Binary {
                op: PgmBinaryOpCode::Add,
                lhs: FheEvalOperand::AllowedDurable {
                    handle: [3; 32],
                    encrypted_value_index: 0,
                },
                rhs: FheEvalOperand::Scalar([1; 32]),
                output_fhe_type: 5,
                output: FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index: 0,
                    output_app_account_authority_index: None,
                    output_acl_domain_key:
                        anchor_lang::prelude::Pubkey::new_from_array([8; 32]),
                    output_app_account:
                        anchor_lang::prelude::Pubkey::new_from_array([9; 32]),
                    output_encrypted_value_label: [10; 32],
                    output_subjects: vec![],
                    previous_handle: Some([8; 32]),
                    previous_subjects: Some(vec![]),
                    make_public: false,
                },
            }],
        };
        let fhe_eval_data = encode_instruction("fhe_eval", plan);
        let instructions =
            vec![decoded_ix(fhe_eval_data, fhe_eval_accounts(), 0, false)];
        let (slot_bank_hash, slot_clock_ts) = slot_context();
        let events = complete_events(
            reconstruct_events_for_insert(
                &config(),
                &instructions,
                42,
                &slot_bank_hash,
                &slot_clock_ts,
            )
            .await
            .expect(
                "reconstruction should derive the supersede handle directly",
            ),
        );

        assert!(events.iter().any(|event| {
            matches!(
                event,
                SolanaHostEvent::FheBinaryOp(op) if op.result == expected
            )
        }));
        let requested_handles = events
            .iter()
            .filter_map(|event| match event {
                SolanaHostEvent::MaterialRequest(request) => {
                    Some(request.handle)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            requested_handles,
            vec![Handle::from(expected), Handle::from([8; 32])],
            "supersession must request the current and previous handles exactly once"
        );
    }

    /// A durable output born public still requests material for its recomputed
    /// handle. The durable bind and public transition share that request.
    #[tokio::test]
    async fn born_public_fhe_eval_output_requests_material() {
        let plan = FheEvalArgs {
            context_id: [1; 32],
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: [7; 32],
                fhe_type: 5,
                output: FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index: 0,
                    output_app_account_authority_index: None,
                    output_acl_domain_key:
                        anchor_lang::prelude::Pubkey::new_from_array([8; 32]),
                    output_app_account:
                        anchor_lang::prelude::Pubkey::new_from_array([9; 32]),
                    output_encrypted_value_label: [10; 32],
                    output_subjects: vec![AclSubjectEntry::user(
                        anchor_lang::prelude::Pubkey::new_from_array(SUBJECT),
                    )],
                    // Fresh lineage (create), born publicly decryptable inline.
                    previous_handle: None,
                    previous_subjects: None,
                    make_public: true,
                },
            }],
        };
        let instructions = vec![decoded_ix(
            encode_instruction("fhe_eval", plan),
            fhe_eval_accounts(),
            0,
            false,
        )];
        let (slot_bank_hash, slot_clock_ts) = slot_context();
        let events = complete_events(
            reconstruct_events_for_insert(
                &config(),
                &instructions,
                42,
                &slot_bank_hash,
                &slot_clock_ts,
            )
            .await
            .expect("born-public create reconstruction should succeed"),
        );

        // The handle the trivial-encrypt bind recomputed for this output.
        let bound_handle = events
            .iter()
            .find_map(|event| match event {
                SolanaHostEvent::TrivialEncrypt(e) => Some(e.result),
                _ => None,
            })
            .expect("trivial-encrypt compute event with a result handle");

        let material_request = events
            .iter()
            .find(|event| {
                matches!(
                    event,
                    SolanaHostEvent::MaterialRequest(request)
                        if request.handle == Handle::from(bound_handle)
                )
            })
            .expect("material request for the born-public bound handle");
        assert!(matches!(
            material_request,
            SolanaHostEvent::MaterialRequest(_)
        ));
    }
}
