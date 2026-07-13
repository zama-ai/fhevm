//! Yellowstone gRPC reconstruction path for the Solana host listener.
//!
//! gRPC transaction updates do not carry block time, so we track a
//! `slot -> block_time` map from `blocks_meta` updates and fall back to a single
//! RPC `getBlockTime` on a miss.

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use futures_util::stream::StreamExt;
use solana_client::nonblocking::rpc_client::RpcClient;
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Channel;
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterAccounts, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterTransactions, SubscribeUpdateTransactionInfo,
};

use crate::database::tfhe_event_propagate::Database;
use crate::solana_adapter::{
    insert_solana_events, solana_transaction_id, SolanaBlockMeta,
};
use crate::solana_slot_hashes::{
    clock_unix_timestamp, previous_bank_hash_from_slot_hashes, CLOCK_SYSVAR,
    SLOT_HASHES_SYSVAR,
};

const MAX_DECODING_MESSAGE_SIZE: usize = 64 * 1024 * 1024;
const SOLANA_RPC_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const SOLANA_GRPC_INGEST_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IngestFailureKind {
    Permanent,
    Retryable,
    Fatal,
}

#[derive(Debug)]
struct IngestFailure {
    kind: IngestFailureKind,
    error: anyhow::Error,
}

impl IngestFailure {
    fn permanent(error: impl Into<anyhow::Error>) -> Self {
        Self {
            kind: IngestFailureKind::Permanent,
            error: error.into(),
        }
    }

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CursorDecision {
    Advance,
    RetrySameCursor,
    FatalSameCursor,
}

fn cursor_decision_for_ingest_failure(
    kind: IngestFailureKind,
) -> CursorDecision {
    match kind {
        IngestFailureKind::Permanent => CursorDecision::Advance,
        IngestFailureKind::Retryable => CursorDecision::RetrySameCursor,
        IngestFailureKind::Fatal => CursorDecision::FatalSameCursor,
    }
}

fn apply_cursor_decision(
    from_slot: &mut Option<u64>,
    slot: u64,
    decision: CursorDecision,
) {
    if decision == CursorDecision::Advance {
        *from_slot = Some(slot);
    }
}

#[derive(Clone, Debug)]
pub struct SolanaGrpcListenerConfig {
    /// Yellowstone gRPC endpoint, e.g. `http://poc-solana-validator:10000`.
    pub grpc_url: String,
    /// Optional `x-token` auth metadata (None for a local validator).
    pub x_token: Option<String>,
    /// RPC endpoint used only as a `getBlockTime` fallback for block timestamps.
    pub rpc_fallback_url: String,
    /// Base58 zama-host program id whose instructions are reconstructed.
    pub program_id: String,
    /// On-chain HostConfig chain_id used in handle derivation (distinct from the
    /// coprocessor host-chain id). Used by the reconstruction path.
    pub chain_id: u64,
    /// True when the on-chain HostConfig enables zero-birth entropy (test/PoC):
    /// derivation uses previous_bank_hash=[0;32] instead of the SlotHashes value.
    pub zero_birth_entropy: bool,
}

/// Connects, subscribes, and ingests until `cancel` fires. Reconnects with a
/// `from_slot` cursor on stream errors; inserts are idempotent so replay is safe.
pub async fn run(
    db: &Database,
    config: &SolanaGrpcListenerConfig,
    cancel: CancellationToken,
) -> Result<()> {
    info!(
        program_id = %config.program_id,
        grpc_url = %config.grpc_url,
        "Starting Solana host listener (Yellowstone gRPC transport)"
    );
    // Authorization is revalidated by KMS before plaintext release. Confirmed
    // data is sufficient for eager ciphertext-material preparation.
    let rpc = RpcClient::new_with_timeout_and_commitment(
        config.rpc_fallback_url.clone(),
        SOLANA_RPC_REQUEST_TIMEOUT,
        solana_commitment_config::CommitmentConfig::confirmed(),
    );
    // TODO(unbounded-cache): these per-slot maps are insert/get only — never
    // evicted — so they grow without bound in this long-lived process. Only recent
    // slots are ever read (txs arrive near the tip), so prune on insert (retain
    // slots >= newest - WINDOW) or use a bounded/LRU map.
    let mut slot_time: HashMap<u64, PrimitiveDateTime> = HashMap::new();
    // `slot -> previous_bank_hash` sourced from the SlotHashes sysvar stream;
    // consumed by the reconstruction path (recompute of trivial/rand/fhe_eval).
    let mut slot_bank_hash: HashMap<u64, [u8; 32]> = HashMap::new();
    // `slot -> Clock.unix_timestamp` from the Clock sysvar stream (the value the
    // program uses in handle derivation, which differs from getBlockTime).
    let mut slot_clock_ts: HashMap<u64, i64> = HashMap::new();
    let mut from_slot: Option<u64> = None;

    loop {
        if cancel.is_cancelled() {
            return Ok(());
        }
        match subscribe_loop(
            db,
            &rpc,
            config,
            &mut slot_time,
            &mut slot_bank_hash,
            &mut slot_clock_ts,
            &mut from_slot,
            &cancel,
        )
        .await
        {
            Ok(()) => return Ok(()), // cancelled
            Err(err) => match err.downcast::<FatalListenerError>() {
                Ok(fatal) => {
                    let err = fatal.into_inner();
                    error!(error = %err, from_slot = ?from_slot, "gRPC listener stopped on fail-closed ingestion error");
                    return Err(err);
                }
                Err(err) => {
                    error!(error = %err, from_slot = ?from_slot, "gRPC subscription dropped; reconnecting");
                    tokio::select! {
                        _ = cancel.cancelled() => return Ok(()),
                        _ = tokio::time::sleep(Duration::from_secs(2)) => {}
                    }
                }
            },
        }
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

#[allow(clippy::too_many_arguments)]
async fn subscribe_loop(
    db: &Database,
    rpc: &RpcClient,
    config: &SolanaGrpcListenerConfig,
    slot_time: &mut HashMap<u64, PrimitiveDateTime>,
    slot_bank_hash: &mut HashMap<u64, [u8; 32]>,
    slot_clock_ts: &mut HashMap<u64, i64>,
    from_slot: &mut Option<u64>,
    cancel: &CancellationToken,
) -> Result<()> {
    let channel = Channel::from_shared(config.grpc_url.clone())
        .context("invalid grpc url")?
        .connect()
        .await
        .context("connect grpc endpoint")?;

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

    let request = build_subscribe_request(config, *from_slot);
    let outbound = futures_util::stream::once(async move { request })
        .chain(futures_util::stream::pending::<SubscribeRequest>());

    let mut stream = client
        .subscribe(outbound)
        .await
        .context("subscribe")?
        .into_inner();

    loop {
        tokio::select! {
            _ = cancel.cancelled() => return Ok(()),
            // The subscription requests blocks_meta (see build_subscribe_request), which the
            // validator emits every slot, so prolonged total silence means the stream stalled
            // rather than the chain being idle. Bound the await so a stall reconnects instead of
            // hanging forever waiting on a stream that will never produce again.
            msg = tokio::time::timeout(Duration::from_secs(30), stream.message()) => {
                let msg = msg.map_err(|_| anyhow!("grpc stream idle for 30s; reconnecting"))?;
                let Some(update) = msg.context("grpc stream")? else {
                    // A None message means the server closed the stream. This is NOT a
                    // cancellation (handled above) — return an error so the outer loop reconnects
                    // and resumes from `from_slot`, rather than exiting silently and missing every
                    // later slot.
                    return Err(anyhow!("grpc stream closed by server"));
                };
                match update.update_oneof {
                    Some(UpdateOneof::BlockMeta(bm)) => {
                        if let Some(ts) = bm.block_time.and_then(|t| unix_to_pdt(t.timestamp)) {
                            slot_time.insert(bm.slot, ts);
                        }
                    }
                    Some(UpdateOneof::Account(acc)) => {
                        // Cache per-slot derivation inputs from the sysvar stream:
                        // SlotHashes -> previous_bank_hash, Clock -> unix_timestamp.
                        if let Some(info) = acc.account {
                            let pubkey =
                                bs58::encode(&info.pubkey).into_string();
                            if pubkey == SLOT_HASHES_SYSVAR {
                                if let Some(prev) =
                                    previous_bank_hash_from_slot_hashes(
                                        &info.data, acc.slot,
                                    )
                                {
                                    slot_bank_hash.insert(acc.slot, prev);
                                }
                            } else if pubkey == CLOCK_SYSVAR {
                                if let Some(ts) =
                                    clock_unix_timestamp(&info.data)
                                {
                                    slot_clock_ts.insert(acc.slot, ts);
                                }
                            }
                        }
                    }
                    Some(UpdateOneof::Transaction(txu)) => {
                        let slot = txu.slot;
                        if let Some(info) = txu.transaction {
                            let signature =
                                bs58::encode(&info.signature).into_string();
                            if info.is_vote {
                                *from_slot = Some(slot);
                                continue;
                            }
                            match tokio::time::timeout(
                                SOLANA_GRPC_INGEST_TIMEOUT,
                                ingest_transaction(
                                    db, rpc, config, slot, &info, slot_time,
                                    &*slot_bank_hash, &*slot_clock_ts,
                                ),
                            )
                            .await
                            {
                                Ok(Ok(())) => {}
                                Ok(Err(err)) => {
                                    let decision =
                                        cursor_decision_for_ingest_failure(
                                            err.kind(),
                                        );
                                    apply_cursor_decision(
                                        from_slot, slot, decision,
                                    );
                                    match decision {
                                        CursorDecision::Advance => {
                                            error!(
                                                slot,
                                                signature = %signature,
                                                error = %err,
                                                "permanently failed to ingest gRPC transaction; skipping"
                                            );
                                            continue;
                                        }
                                        CursorDecision::RetrySameCursor => {
                                            error!(
                                                slot,
                                                signature = %signature,
                                                from_slot = ?from_slot,
                                                error = %err,
                                                "retryable gRPC transaction ingest failure; reconnecting without advancing cursor"
                                            );
                                            return Err(err
                                                .into_error()
                                                .context("retryable gRPC transaction ingest failure"));
                                        }
                                        CursorDecision::FatalSameCursor => {
                                            error!(
                                                slot,
                                                signature = %signature,
                                                from_slot = ?from_slot,
                                                error = %err,
                                                "fatal gRPC transaction ingest failure; stopping without advancing cursor"
                                            );
                                            return Err(FatalListenerError::new(
                                                err.into_error(),
                                            )
                                            .into());
                                        }
                                    }
                                }
                                Err(_) => {
                                    warn!(
                                        slot,
                                        signature = %signature,
                                        timeout = ?SOLANA_GRPC_INGEST_TIMEOUT,
                                        from_slot = ?from_slot,
                                        "timed out ingesting gRPC transaction; reconnecting without advancing cursor"
                                    );
                                    return Err(anyhow!(
                                        "timed out ingesting gRPC transaction {signature} in slot {slot}"
                                    ));
                                }
                            }
                        }
                        *from_slot = Some(slot);
                    }
                    Some(UpdateOneof::Ping(_)) => debug!("grpc ping"),
                    _ => {}
                }
            }
        }
    }
}

fn build_subscribe_request(
    config: &SolanaGrpcListenerConfig,
    from_slot: Option<u64>,
) -> SubscribeRequest {
    let mut transactions = HashMap::new();
    transactions.insert(
        "zama_host".to_string(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            signature: None,
            account_include: vec![config.program_id.clone()],
            account_exclude: vec![],
            account_required: vec![],
        },
    );

    let mut blocks_meta = HashMap::new();
    blocks_meta.insert("meta".to_string(), SubscribeRequestFilterBlocksMeta {});

    // Stream the SlotHashes + Clock sysvar accounts to source
    // `previous_bank_hash` and `unix_timestamp` per slot for handle reconstruction.
    let mut accounts = HashMap::new();
    accounts.insert(
        "sysvars".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![
                SLOT_HASHES_SYSVAR.to_string(),
                CLOCK_SYSVAR.to_string(),
            ],
            owner: vec![],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    SubscribeRequest {
        accounts,
        slots: HashMap::new(),
        transactions,
        transactions_status: HashMap::new(),
        blocks: HashMap::new(),
        blocks_meta,
        entry: HashMap::new(),
        commitment: Some(CommitmentLevel::Confirmed as i32),
        accounts_data_slice: vec![],
        ping: None,
        from_slot,
    }
}

#[allow(clippy::too_many_arguments)]
async fn ingest_transaction(
    db: &Database,
    rpc: &RpcClient,
    config: &SolanaGrpcListenerConfig,
    slot: u64,
    info: &SubscribeUpdateTransactionInfo,
    slot_time: &mut HashMap<u64, PrimitiveDateTime>,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
) -> std::result::Result<(), IngestFailure> {
    let meta = info
        .meta
        .as_ref()
        .ok_or_else(|| IngestFailure::permanent(anyhow!("tx has no meta")))?;
    let tx = info.transaction.as_ref().ok_or_else(|| {
        IngestFailure::permanent(anyhow!("tx has no transaction"))
    })?;
    let message = tx.message.as_ref().ok_or_else(|| {
        IngestFailure::permanent(anyhow!("tx has no message"))
    })?;

    // Resolved account-key list: static keys ++ ALT writable ++ ALT readonly,
    // the order `program_id_index` indexes into.
    let mut account_keys: Vec<String> = message
        .account_keys
        .iter()
        .map(|k| bs58::encode(k).into_string())
        .collect();
    account_keys.extend(
        meta.loaded_writable_addresses
            .iter()
            .map(|k| bs58::encode(k).into_string()),
    );
    account_keys.extend(
        meta.loaded_readonly_addresses
            .iter()
            .map(|k| bs58::encode(k).into_string()),
    );

    // Inner instructions == CPI frames carrying emit_cpi! event bytes.
    let mut inner: Vec<(String, Vec<u8>)> = Vec::new();
    for group in &meta.inner_instructions {
        for ix in &group.instructions {
            let program = account_keys
                .get(ix.program_id_index as usize)
                .ok_or_else(|| {
                    IngestFailure::permanent(anyhow!(
                        "program_id_index {} out of range",
                        ix.program_id_index
                    ))
                })?;
            inner.push((program.clone(), ix.data.clone()));
        }
    }

    // Reconstruction needs each instruction's RESOLVED accounts by address, so
    // build the account-key list in raw 32-byte form (same order as the base58
    // list: static keys ++ ALT writable ++ ALT readonly).
    let account_keys_bytes: Vec<[u8; 32]> = message
        .account_keys
        .iter()
        .chain(meta.loaded_writable_addresses.iter())
        .chain(meta.loaded_readonly_addresses.iter())
        .map(|k| <[u8; 32]>::try_from(k.as_slice()).unwrap_or([0u8; 32]))
        .collect();

    // Top-level + inner instruction invocations with resolved accounts (a
    // zama-host instruction is called directly as top-level, or via CPI as inner
    // for token flows); scanned by the reconstruction shadow-compare and ingest.
    let all_instructions: Vec<crate::solana_reconstruct::DecodedInstruction> = {
        let resolve = |idxs: &[u8]| -> Vec<[u8; 32]> {
            idxs.iter()
                .filter_map(|&i| account_keys_bytes.get(i as usize).copied())
                .collect()
        };
        let decode = |top_level_index: u32,
                      is_inner: bool,
                      program_id_index: u32,
                      data: &[u8],
                      accounts: &[u8]| {
            crate::solana_reconstruct::DecodedInstruction {
                program: account_keys
                    .get(program_id_index as usize)
                    .cloned()
                    .unwrap_or_default(),
                data: data.to_vec(),
                accounts: resolve(accounts),
                top_level_index,
                is_inner,
            }
        };
        let mut inner_by_index: HashMap<u32, Vec<_>> = HashMap::new();
        for group in &meta.inner_instructions {
            inner_by_index.insert(
                group.index,
                group
                    .instructions
                    .iter()
                    .map(|ix| {
                        decode(
                            group.index,
                            true,
                            ix.program_id_index,
                            &ix.data,
                            &ix.accounts,
                        )
                    })
                    .collect(),
            );
        }
        let mut ordered = Vec::new();
        for (index, ix) in message.instructions.iter().enumerate() {
            let index = index as u32;
            ordered.push(decode(
                index,
                false,
                ix.program_id_index,
                &ix.data,
                &ix.accounts,
            ));
            if let Some(inner) = inner_by_index.remove(&index) {
                ordered.extend(inner);
            }
        }
        ordered.extend(inner_by_index.into_values().flatten());
        ordered
    };

    let events = match reconstruct_events_for_insert(
        config,
        &all_instructions,
        slot,
        slot_bank_hash,
        slot_clock_ts,
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

    let block_timestamp = match slot_time.get(&slot) {
        Some(ts) => *ts,
        None => {
            let ts = rpc.get_block_time(slot).await.map_err(|err| {
                IngestFailure::retryable(err).context("getBlockTime fallback")
            })?;
            let pdt = unix_to_pdt(ts).ok_or_else(|| {
                IngestFailure::permanent(anyhow!("invalid block_time {ts}"))
            })?;
            slot_time.insert(slot, pdt);
            pdt
        }
    };
    let block = SolanaBlockMeta {
        block_number: slot,
        block_timestamp,
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
        slot,
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

/// Builds the handle-derivation context for `slot` from the streamed sysvars,
/// applying the zero-birth-entropy rule. Returns `None` until both the Clock and
/// (in production mode) the SlotHashes value for the slot have been cached.
fn reconstruct_context(
    config: &SolanaGrpcListenerConfig,
    slot: u64,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
) -> Option<crate::solana_reconstruct::ReconstructContext> {
    let unix_timestamp = slot_clock_ts.get(&slot).copied()?;
    // Zero-birth-entropy (test/PoC) configs derive with previous_bank_hash=[0;32]
    // (host_config.zero_birth_entropy_allowed()); production uses the
    // SlotHashes-sourced value for the tx's slot.
    let previous_bank_hash = if config.zero_birth_entropy {
        [0u8; 32]
    } else {
        slot_bank_hash.get(&slot).copied()?
    };
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
mod ingest_cursor_tests {
    use super::{
        apply_cursor_decision, cursor_decision_for_ingest_failure,
        CursorDecision, IngestFailureKind,
    };

    #[test]
    fn retryable_ingest_failure_keeps_cursor_for_replay() {
        let mut from_slot = Some(40);
        let decision =
            cursor_decision_for_ingest_failure(IngestFailureKind::Retryable);

        apply_cursor_decision(&mut from_slot, 42, decision);

        assert_eq!(decision, CursorDecision::RetrySameCursor);
        assert_eq!(from_slot, Some(40));
    }

    #[test]
    fn permanent_ingest_failure_advances_cursor() {
        let mut from_slot = Some(40);
        let decision =
            cursor_decision_for_ingest_failure(IngestFailureKind::Permanent);

        apply_cursor_decision(&mut from_slot, 42, decision);

        assert_eq!(decision, CursorDecision::Advance);
        assert_eq!(from_slot, Some(42));
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
            rpc_fallback_url: "http://127.0.0.1:1".to_owned(),
            program_id: ZAMA_HOST.to_owned(),
            chain_id: 12345,
            zero_birth_entropy: true,
        }
    }

    const ZAMA_HOST: &str = "ZamaHost11111111111111111111111111111111";
    const ENCRYPTED_VALUE: [u8; 32] = [0x22; 32];
    const SUBJECT: [u8; 32] = [0x33; 32];

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
        (HashMap::new(), HashMap::from([(42, 1_700_000_000)]))
    }

    /// The durable `Add` output handle the fhe_eval fixtures produce, derived
    /// exactly as the program does: the base handle, no per-output binding
    /// (durable == instruction-local, matching EVM). Matches `config()`
    /// (chain_id 12345, zero_birth_entropy → previous_bank_hash [0;32]), slot
    /// 42's clock ts, op_index 0, scalar rhs.
    fn derived_add_output_handle() -> [u8; 32] {
        zama_host::state::computed_eval_handle(
            PgmBinaryOpCode::Add,
            [3; 32],
            [1; 32],
            true,
            5,
            12345,
            [0; 32],
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
