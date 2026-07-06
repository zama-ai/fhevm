//! Yellowstone gRPC transport for the Solana host listener (Phase 1).
//!
//! Drop-in transport alternative to [`crate::solana_listener`]: instead of
//! RPC-polling `getSignaturesForAddress`, it subscribes to a Yellowstone gRPC
//! endpoint for transactions touching the tracked program, then feeds the SAME
//! shared decoder ([`decode_solana_transaction_events`]) and inserter
//! ([`insert_solana_events`]) the RPC path uses. On-chain events are still
//! emitted (`emit_cpi!`/`emit!`); only the transport differs, so ingested rows
//! match the RPC path by construction.
//!
//! gRPC transaction updates do not carry block time, so we track a
//! `slot -> block_time` map from `blocks_meta` updates and fall back to a single
//! RPC `getBlockTime` on a miss.

use std::collections::HashMap;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use futures_util::stream::StreamExt;
use solana_client::nonblocking::rpc_client::RpcClient;
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio_util::sync::CancellationToken;
#[cfg(feature = "solana-reconstruct")]
use tracing::warn;
use tracing::{debug, error, info};

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
    decode_solana_transaction_events, insert_solana_events,
    solana_transaction_id, SolanaBlockMeta,
};
use crate::solana_slot_hashes::{
    clock_unix_timestamp, previous_bank_hash_from_slot_hashes, CLOCK_SYSVAR,
    SLOT_HASHES_SYSVAR,
};

const MAX_DECODING_MESSAGE_SIZE: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct SolanaGrpcListenerConfig {
    /// Yellowstone gRPC endpoint, e.g. `http://poc-solana-validator:10000`.
    pub grpc_url: String,
    /// Optional `x-token` auth metadata (None for a local validator).
    pub x_token: Option<String>,
    /// RPC endpoint used only as a `getBlockTime` fallback for block timestamps.
    pub rpc_fallback_url: String,
    /// Base58 program id whose CPI/log events are ingested (the zama-host id).
    pub program_id: String,
    /// Commitment level for the subscription.
    pub commitment: CommitmentLevel,
    /// On-chain HostConfig chain_id used in handle derivation (distinct from the
    /// coprocessor host-chain id). Used by the reconstruction path.
    pub chain_id: u64,
    /// True when the on-chain HostConfig enables zero-birth entropy (test/PoC):
    /// derivation uses previous_bank_hash=[0;32] instead of the SlotHashes value.
    pub zero_birth_entropy: bool,
    /// When true, ingest events REBUILT off-chain from instructions (compute
    /// events + acl_record_bound fetches) instead of the emit-decoded events —
    /// the swap that lets on-chain emits eventually be removed. When false, ingest
    /// stays emit-based and reconstruction only runs as a shadow-compare.
    pub reconstruct: bool,
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

    // Confirmed (not the RpcClient default of finalized) so on-chain reads —
    // getBlockTime and the acl_record read for commit_handle_material — see
    // recently-created state, matching the gRPC subscription commitment.
    let rpc = RpcClient::new_with_commitment(
        config.rpc_fallback_url.clone(),
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
    #[cfg(feature = "solana-reconstruct")]
    let mut encrypted_value_tracker =
        crate::solana_reconstruct::EncryptedValueLineageTracker::new();
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
            #[cfg(feature = "solana-reconstruct")]
            &mut encrypted_value_tracker,
            &mut from_slot,
            &cancel,
        )
        .await
        {
            Ok(()) => return Ok(()), // cancelled
            Err(err) => {
                error!(error = %err, from_slot = ?from_slot, "gRPC subscription dropped; reconnecting");
                tokio::select! {
                    _ = cancel.cancelled() => return Ok(()),
                    _ = tokio::time::sleep(Duration::from_secs(2)) => {}
                }
            }
        }
    }
}

/// Resolve a durable `fhe_eval` step's output `EncryptedValue` PDA from the
/// instruction's accounts.
///
/// `remaining_index` (the program's `output_encrypted_value_index`) is relative to
/// `remaining_accounts`, which follow the 7 named `fhe_eval` accounts — payer,
/// compute_subject, app_account_authority, host_config, system_program, then
/// `#[event_cpi]`'s event_authority + program (see `FheEval` in fhe_eval.rs). Returns
/// `None` when the index is out of range; the caller treats that as a hard problem, since
/// the durable output would otherwise never be marked allowed and never materialize.
#[cfg(feature = "solana-reconstruct")]
fn fhe_eval_durable_encrypted_value(
    accounts: &[[u8; 32]],
    remaining_index: u16,
) -> Option<[u8; 32]> {
    const FHE_EVAL_REMAINING_BASE: usize = 7;
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
    #[cfg(feature = "solana-reconstruct")]
    encrypted_value_tracker: &mut crate::solana_reconstruct::EncryptedValueLineageTracker,
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
                            if info.is_vote {
                                continue;
                            }
                            if let Err(err) = ingest_transaction(
                                db, rpc, config, slot, &info, slot_time,
                                &*slot_bank_hash, &*slot_clock_ts,
                                #[cfg(feature = "solana-reconstruct")]
                                encrypted_value_tracker,
                            )
                            .await
                            {
                                // Do not advance from_slot past a failed ingest.
                                error!(slot, error = %err, "failed to ingest gRPC transaction");
                                continue;
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
        commitment: Some(config.commitment as i32),
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
    #[cfg(feature = "solana-reconstruct")]
    encrypted_value_tracker: &mut crate::solana_reconstruct::EncryptedValueLineageTracker,
) -> Result<()> {
    #[cfg(not(feature = "solana-reconstruct"))]
    let _ = (slot_bank_hash, slot_clock_ts); // only used by the shadow-compare
    let meta = info
        .meta
        .as_ref()
        .ok_or_else(|| anyhow!("tx has no meta"))?;
    let tx = info
        .transaction
        .as_ref()
        .ok_or_else(|| anyhow!("tx has no transaction"))?;
    let message = tx
        .message
        .as_ref()
        .ok_or_else(|| anyhow!("tx has no message"))?;

    // Resolved account-key list: static keys ++ ALT writable ++ ALT readonly,
    // the order `program_id_index` indexes into (matches solana_listener).
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
                    anyhow!(
                        "program_id_index {} out of range",
                        ix.program_id_index
                    )
                })?;
            inner.push((program.clone(), ix.data.clone()));
        }
    }

    // Reconstruction needs each instruction's RESOLVED accounts by address, so
    // build the account-key list in raw 32-byte form (same order as the base58
    // list: static keys ++ ALT writable ++ ALT readonly).
    #[cfg(feature = "solana-reconstruct")]
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
    #[cfg(feature = "solana-reconstruct")]
    let all_instructions: Vec<
        crate::solana_reconstruct::DecodedInstruction,
    > = {
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

    // emit! events arrive as `Program data:` log lines.
    let logs: Vec<String> = if meta.log_messages_none {
        Vec::new()
    } else {
        meta.log_messages.clone()
    };

    let emit_events = decode_solana_transaction_events(
        &logs,
        inner.iter().map(|(p, d)| (p.as_str(), d.as_slice())),
        &config.program_id,
    )
    .context("decode host events")?;

    // Choose the event set to ingest. In `--reconstruct` mode, rebuild it off-chain
    // (compute events + acl_record_bound fetches) from the instructions and ingest
    // THAT in place of emit-decode — the swap that lets on-chain emits be removed
    // (#7). With emits off, emit-decode yields nothing, so reconstruction is decided
    // here, BEFORE the empty gate below; we fall back to emit-decode only when
    // reconstruction produces nothing (e.g. instruction families not yet covered).
    #[cfg(feature = "solana-reconstruct")]
    let events = if config.reconstruct {
        match reconstruct_events_for_insert(
            config,
            &all_instructions,
            slot,
            slot_bank_hash,
            slot_clock_ts,
            rpc,
            encrypted_value_tracker,
        )
        .await
        {
            Some(reconstructed) if !reconstructed.is_empty() => {
                info!(
                    slot,
                    reconstructed = reconstructed.len(),
                    emit_decoded = emit_events.len(),
                    "ingesting reconstructed events (emit-decode swapped out)"
                );
                reconstructed
            }
            Some(_) | None => {
                if !emit_events.is_empty() {
                    warn!(
                        slot,
                        "reconstruction unavailable/empty; falling back to emit-decode"
                    );
                }
                emit_events
            }
        }
    } else {
        let mut events = emit_events;
        events.extend(
            crate::solana_reconstruct::decode_encrypted_value_fetch_events(
                &all_instructions,
                &config.program_id,
                encrypted_value_tracker,
            ),
        );
        events
    };
    #[cfg(not(feature = "solana-reconstruct"))]
    let events = emit_events;

    if events.is_empty() {
        return Ok(());
    }

    let block_timestamp = match slot_time.get(&slot) {
        Some(ts) => *ts,
        None => {
            let ts = rpc
                .get_block_time(slot)
                .await
                .context("getBlockTime fallback")?;
            let pdt = unix_to_pdt(ts)
                .ok_or_else(|| anyhow!("invalid block_time {ts}"))?;
            slot_time.insert(slot, pdt);
            pdt
        }
    };
    let block = SolanaBlockMeta {
        block_number: slot,
        block_timestamp,
    };

    let transaction_id = solana_transaction_id(&info.signature);
    let mut db_tx = db.new_transaction().await.context("open db tx")?;
    let stats =
        insert_solana_events(db, &mut db_tx, events, transaction_id, block)
            .await
            .context("insert_solana_events")?;
    db_tx.commit().await.context("commit db tx")?;

    info!(
        slot,
        signature = %bs58::encode(&info.signature).into_string(),
        tfhe_events = stats.tfhe_events,
        acl_events = stats.acl_events,
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
#[cfg(feature = "solana-reconstruct")]
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

/// Rebuilds the ingestable event set off-chain from a transaction's instructions,
/// for `--reconstruct` mode (the swap that replaces emit-decoding). Covers
/// `fhe_eval`: one compute event per step, plus an `acl_record_bound` allow-fetch
/// for each `Durable` step (handle = the step result, ACL record = the step's
/// `remaining_accounts` entry) — matching what the program's bind emits, so
/// `is_allowed` and the fetch row land identically.
///
/// `EncryptedValue` lifecycle instructions are decoded separately from the same
/// ordered instruction list and appended to the reconstructed event set. Missing
/// slot derivation context only suppresses `fhe_eval` recomputation; lifecycle
/// fetches do not need it.
#[cfg(feature = "solana-reconstruct")]
async fn reconstruct_events_for_insert(
    config: &SolanaGrpcListenerConfig,
    instructions: &[crate::solana_reconstruct::DecodedInstruction],
    slot: u64,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
    _rpc: &RpcClient,
    encrypted_value_tracker: &mut crate::solana_reconstruct::EncryptedValueLineageTracker,
) -> Option<Vec<crate::solana_adapter::SolanaHostEvent>> {
    use crate::solana_adapter::SolanaHostEvent;
    use crate::solana_reconstruct::{
        decode_fhe_eval_args, reconstruct_acl_record_bound_fetch,
        reconstruct_fhe_eval_steps_with_hints,
    };

    // compute_subject is the 2nd named fhe_eval account. (Durable EncryptedValue
    // PDAs live in remaining_accounts; resolved via
    // fhe_eval_durable_encrypted_value.)
    const COMPUTE_SUBJECT_INDEX: usize = 1;

    let mut events =
        crate::solana_reconstruct::decode_encrypted_value_fetch_events(
            instructions,
            &config.program_id,
            encrypted_value_tracker,
        );

    let Some(ctx) =
        reconstruct_context(config, slot, slot_bank_hash, slot_clock_ts)
    else {
        return Some(events);
    };

    // Pre-instruction MMR leaf counts are intentionally not fetched here. When
    // the program emits an inner update instruction, its explicit new_handle is
    // used as the repair path for superseding durable outputs.
    let lineage_leaf_counts: HashMap<[u8; 32], u64> = HashMap::new();

    for (index, ix) in instructions.iter().enumerate() {
        if ix.program != config.program_id {
            continue;
        }
        if let Some(plan) = decode_fhe_eval_args(&ix.data) {
            let subject = ix
                .accounts
                .get(COMPUTE_SUBJECT_INDEX)
                .copied()
                .unwrap_or([0u8; 32]);
            let mut handle_hints = durable_output_handle_hints_for_fhe_eval(
                index,
                ix,
                &plan,
                instructions,
                &config.program_id,
            );
            let Some(steps) = reconstruct_fhe_eval_steps_with_hints(
                &plan,
                subject,
                &ctx,
                &lineage_leaf_counts,
                &mut handle_hints,
            ) else {
                continue;
            };
            for step in steps {
                let handle = compute_result_handle(&step.event);
                events.push(step.event);
                if let (Some(index), Some(handle)) =
                    (step.durable_encrypted_value_index, handle)
                {
                    if let Some(encrypted_value) =
                        fhe_eval_durable_encrypted_value(&ix.accounts, index)
                    {
                        events.push(SolanaHostEvent::FinalizedAccountFetch(
                            reconstruct_acl_record_bound_fetch(
                                encrypted_value,
                                handle,
                            ),
                        ));
                    } else {
                        // The durable bind marks the output handle allowed; dropping
                        // it silently leaves the handle unmaterialized and starves
                        // every later consumer. A miss here means the account layout
                        // drifted from the program — surface it loudly.
                        warn!(
                            slot,
                            remaining_index = index,
                            accounts = ix.accounts.len(),
                            handle = %bs58::encode(handle).into_string(),
                            "reconstruct: fhe_eval durable bind encrypted_value out of range; \
                             output handle will not be allowed/materialized"
                        );
                    }
                }
            }
        }
    }
    Some(events)
}

#[cfg(feature = "solana-reconstruct")]
fn durable_output_handle_hints_for_fhe_eval(
    ix_position: usize,
    fhe_eval_ix: &crate::solana_reconstruct::DecodedInstruction,
    plan: &zama_host::state::FheEvalArgs,
    instructions: &[crate::solana_reconstruct::DecodedInstruction],
    program_id: &str,
) -> crate::solana_reconstruct::DurableOutputHandleHints {
    use crate::solana_reconstruct::{
        decode_encrypted_value_instruction, decode_fhe_eval_args,
        encrypted_value_bound_handle, DurableOutputHandleHints,
        ENCRYPTED_VALUE_ACCOUNT_INDEX,
    };
    use std::collections::{HashMap, VecDeque};
    use zama_host::state::{FheEvalOutput, FheEvalStep};

    let mut bound_handles_by_account: HashMap<[u8; 32], VecDeque<[u8; 32]>> =
        HashMap::new();
    for ix in &instructions[ix_position + 1..] {
        if ix.top_level_index != fhe_eval_ix.top_level_index || !ix.is_inner {
            break;
        }
        if ix.program == program_id && decode_fhe_eval_args(&ix.data).is_some()
        {
            break;
        }
        if ix.program != program_id {
            continue;
        }
        let Some(instruction) = decode_encrypted_value_instruction(&ix.data)
        else {
            continue;
        };
        let Some(handle) = encrypted_value_bound_handle(&instruction) else {
            continue;
        };
        let Some(account) =
            ix.accounts.get(ENCRYPTED_VALUE_ACCOUNT_INDEX).copied()
        else {
            continue;
        };
        bound_handles_by_account
            .entry(account)
            .or_default()
            .push_back(handle);
    }

    let mut hints = DurableOutputHandleHints::default();
    for step in &plan.steps {
        let output = match step {
            FheEvalStep::Binary { output, .. }
            | FheEvalStep::Ternary { output, .. }
            | FheEvalStep::TrivialEncrypt { output, .. }
            | FheEvalStep::Rand { output, .. } => output,
        };
        let FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            previous_handle: Some(_),
            ..
        } = output
        else {
            continue;
        };
        let Some(encrypted_value) = fhe_eval_durable_encrypted_value(
            &fhe_eval_ix.accounts,
            *output_encrypted_value_index,
        ) else {
            continue;
        };
        let Some(handles) = bound_handles_by_account.get_mut(&encrypted_value)
        else {
            continue;
        };
        let Some(handle) = handles.pop_front() else {
            continue;
        };
        hints.push(*output_encrypted_value_index, handle);
    }
    hints
}

#[cfg(feature = "solana-reconstruct")]
fn compute_result_handle(
    event: &crate::solana_adapter::SolanaHostEvent,
) -> Option<[u8; 32]> {
    use crate::solana_adapter::SolanaHostEvent as E;
    match event {
        E::FheBinaryOp(e) => Some(e.result),
        E::FheTernaryOp(e) => Some(e.result),
        E::TrivialEncrypt(e) => Some(e.result),
        E::FheRand(e) => Some(e.result),
        E::FinalizedAccountFetch(_) => None,
    }
}

#[cfg(all(test, feature = "solana-reconstruct"))]
mod fhe_eval_acl_tests {
    use super::{
        fhe_eval_durable_encrypted_value, reconstruct_events_for_insert,
        SolanaGrpcListenerConfig,
    };
    use anchor_lang::AnchorSerialize;
    use sha2::{Digest, Sha256};
    use solana_client::nonblocking::rpc_client::RpcClient;
    use std::collections::HashMap;
    use yellowstone_grpc_proto::prelude::CommitmentLevel;
    use zama_host::state::{
        FheBinaryOpCode as PgmBinaryOpCode, FheEvalArgs, FheEvalOperand,
        FheEvalOutput, FheEvalStep,
    };

    use crate::database::tfhe_event_propagate::Handle;
    use crate::solana_adapter::SolanaHostEvent;
    use crate::solana_reconstruct::{
        AllowSubjectsArgs, DecodedInstruction, EncryptedValueLineageTracker,
        EncryptedValueSubjectGrant, UpdateEncryptedValueArgs,
        ENCRYPTED_VALUE_ACCOUNT_INDEX,
    };

    fn acct(n: u8) -> [u8; 32] {
        [n; 32]
    }

    fn config() -> SolanaGrpcListenerConfig {
        SolanaGrpcListenerConfig {
            grpc_url: "http://127.0.0.1:1".to_owned(),
            x_token: None,
            rpc_fallback_url: "http://127.0.0.1:1".to_owned(),
            program_id: ZAMA_HOST.to_owned(),
            commitment: CommitmentLevel::Confirmed,
            chain_id: 12345,
            zero_birth_entropy: true,
            reconstruct: true,
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
        let mut accounts: Vec<[u8; 32]> = (0..8).map(acct).collect();
        accounts[1] = SUBJECT;
        accounts[7] = ENCRYPTED_VALUE;
        accounts
    }

    fn slot_context() -> (HashMap<u64, [u8; 32]>, HashMap<u64, i64>) {
        (HashMap::new(), HashMap::from([(42, 1_700_000_000)]))
    }

    #[test]
    fn durable_output_as_sole_remaining_account_resolves() {
        // The trivial-encrypt-eval shape: 7 named accounts (0..=6) + exactly one
        // remaining account, the durable output ACL record, at absolute index 7
        // (remaining_index 0). The off-by-one (base 8) read accounts.get(8) here, found
        // nothing, and silently dropped the allow-fetch — leaving the output handle
        // unmaterialized. This pins the base at 7.
        let accounts: Vec<[u8; 32]> = (0..8).map(acct).collect();
        assert_eq!(
            fhe_eval_durable_encrypted_value(&accounts, 0),
            Some(acct(7))
        );
    }

    #[test]
    fn output_after_input_acl_records_resolves() {
        // A durable input ACL record at 7 and the durable output at 8 (remaining_index 1).
        let accounts: Vec<[u8; 32]> = (0..9).map(acct).collect();
        assert_eq!(
            fhe_eval_durable_encrypted_value(&accounts, 1),
            Some(acct(8))
        );
    }

    #[test]
    fn missing_remaining_account_returns_none() {
        // Only the 7 named accounts, no remaining: a durable bind here is a layout drift
        // the caller must surface, not silently drop.
        let accounts: Vec<[u8; 32]> = (0..7).map(acct).collect();
        assert_eq!(fhe_eval_durable_encrypted_value(&accounts, 0), None);
    }

    #[tokio::test]
    async fn direct_allow_subjects_reconstructs_lifecycle_fetch() {
        let allow_data = encode_instruction(
            "allow_subjects",
            AllowSubjectsArgs {
                subjects: vec![EncryptedValueSubjectGrant {
                    subject: [7; 32],
                    role_flags: 1,
                }],
            },
        );
        let instructions =
            vec![decoded_ix(allow_data, encrypted_value_accounts(), 0, false)];
        let (slot_bank_hash, slot_clock_ts) = slot_context();
        let rpc = RpcClient::new("http://127.0.0.1:1".to_owned());
        let mut tracker = EncryptedValueLineageTracker::new();
        tracker.record(ENCRYPTED_VALUE, [4; 32]);

        let events = reconstruct_events_for_insert(
            &config(),
            &instructions,
            42,
            &slot_bank_hash,
            &slot_clock_ts,
            &rpc,
            &mut tracker,
        )
        .await
        .expect("reconstruction should return lifecycle events");

        assert!(events.iter().any(|event| {
            matches!(
                event,
                SolanaHostEvent::FinalizedAccountFetch(fetch)
                    if fetch.reason == "subject_allowed"
                        && fetch.account_key == ENCRYPTED_VALUE
                        && fetch.handle == Some(Handle::from([4; 32]))
            )
        }));
    }

    #[tokio::test]
    async fn superseding_fhe_eval_uses_inner_update_handle_hint() {
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
                },
            }],
        };
        let fhe_eval_data = encode_instruction("fhe_eval", plan);
        let update_data = encode_instruction(
            "update_encrypted_value",
            UpdateEncryptedValueArgs {
                new_handle: [9; 32],
                previous_handle: [8; 32],
                previous_subjects: vec![],
            },
        );
        let instructions = vec![
            decoded_ix(fhe_eval_data, fhe_eval_accounts(), 0, false),
            decoded_ix(update_data, encrypted_value_accounts(), 0, true),
        ];
        let (slot_bank_hash, slot_clock_ts) = slot_context();
        let rpc = RpcClient::new("http://127.0.0.1:1".to_owned());
        let mut tracker = EncryptedValueLineageTracker::new();
        tracker.record(ENCRYPTED_VALUE, [8; 32]);

        let events = reconstruct_events_for_insert(
            &config(),
            &instructions,
            42,
            &slot_bank_hash,
            &slot_clock_ts,
            &rpc,
            &mut tracker,
        )
        .await
        .expect("reconstruction should use inner update hint");

        assert!(events.iter().any(|event| {
            matches!(
                event,
                SolanaHostEvent::FheBinaryOp(op) if op.result == [9; 32]
            )
        }));
        assert!(events.iter().any(|event| {
            matches!(
                event,
                SolanaHostEvent::FinalizedAccountFetch(fetch)
                    if fetch.reason == "acl_record_bound"
                        && fetch.account_key == ENCRYPTED_VALUE
                        && fetch.handle == Some(Handle::from([9; 32]))
            )
        }));
        assert!(events.iter().any(|event| {
            matches!(
                event,
                SolanaHostEvent::FinalizedAccountFetch(fetch)
                    if fetch.reason == "handle_superseded"
                        && fetch.handle == Some(Handle::from([8; 32]))
            )
        }));
    }
}
