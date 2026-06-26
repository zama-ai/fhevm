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
#![cfg(feature = "solana-grpc")]

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
            // BlockMeta arrives every slot, so prolonged total silence means the stream stalled
            // (e.g. the geyser plugin stopped sending after backpressure during an event burst)
            // without ever closing. Bound the await so a stall reconnects instead of hanging.
            msg = tokio::time::timeout(Duration::from_secs(30), stream.message()) => {
                let msg = msg.map_err(|_| anyhow!("grpc stream idle for 30s; reconnecting"))?;
                let Some(update) = msg.context("grpc stream")? else {
                    // Server closed the stream (e.g. the geyser plugin dropping a client that
                    // lagged during an event burst). This is NOT a cancellation — return an
                    // error so the outer loop reconnects (resuming from `from_slot`) instead of
                    // exiting silently and missing every later slot.
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

async fn ingest_transaction(
    db: &Database,
    rpc: &RpcClient,
    config: &SolanaGrpcListenerConfig,
    slot: u64,
    info: &SubscribeUpdateTransactionInfo,
    slot_time: &mut HashMap<u64, PrimitiveDateTime>,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
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
    let all_instructions: Vec<DecodedInstruction> = {
        let resolve = |idxs: &[u8]| -> Vec<[u8; 32]> {
            idxs.iter()
                .filter_map(|&i| account_keys_bytes.get(i as usize).copied())
                .collect()
        };
        let decode = |program_id_index: u32, data: &[u8], accounts: &[u8]| {
            DecodedInstruction {
                program: account_keys
                    .get(program_id_index as usize)
                    .cloned()
                    .unwrap_or_default(),
                data: data.to_vec(),
                accounts: resolve(accounts),
            }
        };
        message
            .instructions
            .iter()
            .map(|ix| decode(ix.program_id_index, &ix.data, &ix.accounts))
            .chain(meta.inner_instructions.iter().flat_map(|group| {
                group.instructions.iter().map(|ix| {
                    decode(ix.program_id_index, &ix.data, &ix.accounts)
                })
            }))
            .collect()
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
        emit_events
    };
    #[cfg(not(feature = "solana-reconstruct"))]
    let events = emit_events;

    // DIAG (temporary): surface every zama-host tx the reconstruct path scans — slot, signature,
    // the zama-host instruction discriminators present, and the reconstructed event count — so a tx
    // that reconstructs to zero events (otherwise invisible past the gate below) is logged. Used to
    // find why the consume/burn compute event is not reconstructed.
    #[cfg(feature = "solana-reconstruct")]
    if config.reconstruct {
        let zh_discs: Vec<String> = all_instructions
            .iter()
            .filter(|ix| ix.program == config.program_id)
            .map(|ix| {
                ix.data
                    .iter()
                    .take(8)
                    .map(|b| format!("{b:02x}"))
                    .collect::<String>()
            })
            .collect();
        info!(
            slot,
            signature = %bs58::encode(&info.signature).into_string(),
            zama_host_ixs = zh_discs.len(),
            discriminators = ?zh_discs,
            reconstructed = events.len(),
            // Full reconstructed payloads (handles/results as byte arrays) so a wrong/missing
            // handle in the consume/burn txs is visible and comparable to the emit-decode path.
            events = ?events,
            "DIAG: reconstruct tx scanned"
        );
    }

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

/// A decoded instruction invocation: program id (base58), instruction data, and
/// resolved account addresses (raw 32-byte) in the instruction's account order.
#[cfg(feature = "solana-reconstruct")]
struct DecodedInstruction {
    program: String,
    data: Vec<u8>,
    accounts: Vec<[u8; 32]>,
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
/// Returns `None` if the slot's derivation context is not yet cached. Direct
/// zama-host compute/bind instructions, `allow_for_decryption`,
/// `commit_handle_material`, and token fetches are not yet reconstructed for
/// ingestion (follow-ups); the caller falls back to emit-decode when this yields
/// nothing.
#[cfg(feature = "solana-reconstruct")]
async fn reconstruct_events_for_insert(
    config: &SolanaGrpcListenerConfig,
    instructions: &[DecodedInstruction],
    slot: u64,
    slot_bank_hash: &HashMap<u64, [u8; 32]>,
    slot_clock_ts: &HashMap<u64, i64>,
    rpc: &RpcClient,
) -> Option<Vec<crate::solana_adapter::SolanaHostEvent>> {
    use crate::generated::{decode_zama_host_instruction, ZamaHostInstruction};
    use crate::solana_adapter::SolanaHostEvent;
    use crate::solana_reconstruct::{
        decode_fhe_eval_args, parse_acl_record_handle,
        reconstruct_acl_record_bound_fetch, reconstruct_fhe_eval_steps,
        reconstruct_instruction_events,
    };
    use solana_sdk::pubkey::Pubkey;

    // fhe_eval named accounts: payer, compute_subject, app_account_authority,
    // host_config, system_program, instructions_sysvar, then event_cpi's
    // event_authority + program = 8; remaining_accounts (ACL records) follow.
    const COMPUTE_SUBJECT_INDEX: usize = 1;
    const FHE_EVAL_REMAINING_BASE: usize = 8;
    // commit_handle_material's acl_record (account 3): its emitted handle comes from
    // that record's account state, so it must be read on-chain.
    const COMMIT_ACL_RECORD_INDEX: usize = 3;

    let ctx = reconstruct_context(config, slot, slot_bank_hash, slot_clock_ts)?;

    // Pre-read acl_record.handle for any commit_handle_material in this tx: the
    // handle is account state, not in the instruction args. Restart-safe RPC read,
    // mirroring the HostConfig startup fetch.
    let mut acl_handles: HashMap<[u8; 32], [u8; 32]> = HashMap::new();
    for ix in instructions {
        if ix.program != config.program_id {
            continue;
        }
        let Some(ZamaHostInstruction::CommitHandleMaterial(_)) =
            decode_zama_host_instruction(&ix.data)
        else {
            continue;
        };
        let Some(acl_record) =
            ix.accounts.get(COMMIT_ACL_RECORD_INDEX).copied()
        else {
            continue;
        };
        if acl_handles.contains_key(&acl_record) {
            continue;
        }
        match rpc.get_account(&Pubkey::from(acl_record)).await {
            Ok(account) => {
                if let Some(handle) = parse_acl_record_handle(&account.data) {
                    acl_handles.insert(acl_record, handle);
                }
            }
            Err(err) => warn!(
                slot,
                error = %err,
                "reconstruct: failed to read acl_record for commit_handle_material"
            ),
        }
    }

    let mut events: Vec<SolanaHostEvent> = Vec::new();
    for ix in instructions {
        if ix.program != config.program_id {
            continue;
        }
        if let Some(plan) = decode_fhe_eval_args(&ix.data) {
            let subject = ix
                .accounts
                .get(COMPUTE_SUBJECT_INDEX)
                .copied()
                .unwrap_or([0u8; 32]);
            let Some(steps) = reconstruct_fhe_eval_steps(&plan, subject, &ctx)
            else {
                continue;
            };
            for step in steps {
                let handle = compute_result_handle(&step.event);
                events.push(step.event);
                if let (Some(index), Some(handle)) =
                    (step.durable_acl_record_index, handle)
                {
                    if let Some(acl_record) = ix
                        .accounts
                        .get(FHE_EVAL_REMAINING_BASE + index as usize)
                    {
                        events.push(SolanaHostEvent::FinalizedAccountFetch(
                            reconstruct_acl_record_bound_fetch(
                                *acl_record,
                                handle,
                            ),
                        ));
                    }
                }
            }
        } else if let Some(instruction) = decode_zama_host_instruction(&ix.data)
        {
            if let Some(evs) = reconstruct_instruction_events(
                &instruction,
                &ix.accounts,
                &ctx,
                &acl_handles,
            ) {
                events.extend(evs);
            }
        }
    }
    Some(events)
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
        E::FheRandBounded(e) => Some(e.result),
        E::FinalizedAccountFetch(_) | E::AclAllowed(_) => None,
    }
}
