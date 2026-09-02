use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy::sol_types::SolEventInterface;
use fhevm_engine_common::bridge::chain_id_from_handle;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::types::{
    Handle, COMPUTED_HANDLE_INDEX_MARKER, HANDLE_VERSION,
};
use sqlx::types::time::{OffsetDateTime, PrimitiveDateTime};
use tracing::{debug, error, info, warn};

use crate::cmd::block_history::{BlockHash, BlockSummary};
use crate::cmd::InfiniteLogIter;
use crate::contracts::{
    AclContract, BridgeContract, KMSGeneration, ProtocolConfig, TfheContract,
};
use crate::database::dependence_chains::dependence_chains;
use crate::database::synthetic_ops::{
    synthetic_logs, synthetic_transaction_hash, SyntheticContext,
    SYNTHETIC_BLOCK_OFFSET,
};
use crate::database::tfhe_event_propagate::{
    acl_result_handles, operand_boundary_mask_from_minted, tfhe_result_handle,
    Chain, ChainHash, Database, Handle as EventHandle, LogTfhe,
    TransactionHash,
};
use crate::kms_generation::insert_kms_generation_events_tx;
use crate::kms_generation::metrics::KMS_EVENT_DECODE_FAIL_COUNTER;
use crate::protocol_config::metrics::PROTOCOL_CONFIG_EVENT_DECODE_FAIL_COUNTER;

pub struct BlockLogs<T> {
    pub logs: Vec<T>,
    pub summary: BlockSummary,
    pub catchup: bool,
    pub finalized: bool,
}

#[derive(Clone, Debug)]
pub struct IngestOptions {
    pub dependence_by_connexity: bool,
    pub dependence_cross_block: bool,
    pub dependent_ops_max_per_chain: u32,
    /// Resolved once at startup from the listener's own `chain_id` and the
    /// configured `--canonical-protocol-config-chain-id`. When false, the listener silently
    /// skips `ProtocolConfig.CoprocessorUpgradeProposed` events.
    pub is_protocol_config_listener: bool,
}

/// Converts a block timestamp to a UTC `PrimitiveDateTime`.
///
/// # Parameters
/// - `timestamp`: Seconds since Unix epoch.
///
/// # Returns
/// A UTC `PrimitiveDateTime` suitable for database writes.
fn block_date_time_utc(timestamp: u64) -> PrimitiveDateTime {
    let offset = OffsetDateTime::from_unix_timestamp(timestamp as i64)
        .unwrap_or_else(|_| {
            error!(timestamp, "Invalid block timestamp, using now",);
            OffsetDateTime::now_utc()
        });
    PrimitiveDateTime::new(offset.date(), offset.time())
}

/// Derive the executor's operand-origin bits before any row is written.
///
/// The executor's transient minted set is transaction-scoped and journaled by
/// EVM reverts. Successful event logs are therefore the authoritative
/// off-chain reconstruction: inspect an operation's inputs first, then add
/// its result only when the event came from the executor itself. In
/// particular, bridge fallback synthesis creates a `TrivialEncrypt`-shaped
/// computation row but did not execute the executor, so it must not mint a
/// later operand in the same transaction.
/// Counts fail-closed rejections of operand-origin mask derivation. The
/// rejected block is retried (and rejected again) on every pass, so a
/// sustained nonzero rate on this counter means ingestion is STALLED on a
/// block whose logs the RPC provider serves malformed (missing/duplicate
/// log metadata) — page on it; the stall does not self-announce otherwise
/// and never resolves without a healthy refetch or an operator fix.
static OPERAND_MASK_DERIVATION_REJECTS_COUNTER: std::sync::LazyLock<
    prometheus::IntCounter,
> = std::sync::LazyLock::new(|| {
    prometheus::register_int_counter!(
        "coprocessor_host_listener_operand_mask_derivation_rejects_counter",
        "Fail-closed rejections of operand-origin mask derivation (block ingest stalled on malformed provider logs)"
    )
    .unwrap()
});

fn refuse_mask_derivation(reason: &'static str) -> sqlx::Error {
    OPERAND_MASK_DERIVATION_REJECTS_COUNTER.inc();
    error!(target: "host_listener", reason, "refusing operand-origin mask derivation; block ingest will stall and retry");
    sqlx::Error::Protocol(reason.into())
}

fn populate_operand_boundary_masks(
    logs: &mut [LogTfhe],
) -> Result<(), sqlx::Error> {
    let mask_bearing = |log: &LogTfhe| tfhe_result_handle(&log.event).is_some();
    for log in logs.iter().filter(|log| mask_bearing(log)) {
        if log.transaction_hash.is_none() {
            return Err(refuse_mask_derivation(
                "refusing computation event without transaction hash for operand-origin derivation",
            ));
        }
        if log.log_index.is_none() {
            return Err(refuse_mask_derivation(
                "refusing computation event without log index for operand-origin derivation",
            ));
        }
    }

    // `log_index` is globally ordered within the block. A stable order is
    // deterministic even if a malformed provider supplied duplicate indexes;
    // reject those instead of allowing the minted-set reconstruction to
    // depend on input delivery order.
    logs.sort_by_key(|log| log.log_index.unwrap_or(u64::MAX));
    let mut previous_index = None;
    for log in logs.iter().filter(|log| mask_bearing(log)) {
        let log_index =
            log.log_index.expect("validated mask-bearing log index");
        if previous_index == Some(log_index) {
            return Err(refuse_mask_derivation(
                "refusing duplicate computation log index for operand-origin derivation",
            ));
        }
        previous_index = Some(log_index);
    }

    let mut minted_by_transaction: HashMap<
        TransactionHash,
        HashSet<EventHandle>,
    > = HashMap::new();
    for log in logs.iter_mut().filter(|log| mask_bearing(log)) {
        let transaction_hash = log
            .transaction_hash
            .expect("validated mask-bearing transaction hash");
        let minted = minted_by_transaction.entry(transaction_hash).or_default();
        let mask = operand_boundary_mask_from_minted(&log.event, |handle| {
            minted.contains(handle)
        })
        .map_err(|reason| {
            OPERAND_MASK_DERIVATION_REJECTS_COUNTER.inc();
            error!(target: "host_listener", %reason, "refusing operand-origin mask derivation; block ingest will stall and retry");
            sqlx::Error::Protocol(reason)
        })?;
        log.operand_boundary_mask = Some(mask);

        // This happens strictly after the mask above, exactly as the executor
        // computes the preimage before calling `_markMinted`.
        if log.is_executor_minted {
            if let Some(result) = tfhe_result_handle(&log.event) {
                minted.insert(result);
            }
        }
    }
    Ok(())
}

fn propagate_slow_lane_to_dependents(
    chains: &[Chain],
    slow_dep_chain_ids: &mut HashSet<ChainHash>,
) {
    let mut dependents_by_dependency: HashMap<ChainHash, Vec<ChainHash>> =
        HashMap::new();
    for chain in chains {
        for dependency in &chain.split_dependencies {
            dependents_by_dependency
                .entry(*dependency)
                .or_default()
                .push(chain.hash);
        }
    }

    let mut queue: VecDeque<ChainHash> =
        slow_dep_chain_ids.iter().cloned().collect();
    while let Some(slow_dependency) = queue.pop_front() {
        let Some(dependents) = dependents_by_dependency.get(&slow_dependency)
        else {
            continue;
        };
        for dependent in dependents {
            if slow_dep_chain_ids.insert(*dependent) {
                queue.push_back(*dependent);
            }
        }
    }
}

/// Marks slow chains by counting inserted ops on linked split chains together.
///
/// In no-fork mode, one logical workload can be split into many small chains.
/// Here we connect chains through `split_dependencies`, sum their inserted-op
/// counts, and if the sum is above the cap we mark all linked chains as slow.
fn classify_slow_by_split_dependency_closure(
    chains: &[Chain],
    dependent_ops_by_chain: &HashMap<ChainHash, u64>,
    max_per_chain: u64,
) -> HashSet<ChainHash> {
    let chain_ids = chains
        .iter()
        .map(|chain| chain.hash)
        .collect::<HashSet<_>>();
    let mut neighbors: HashMap<ChainHash, HashSet<ChainHash>> =
        HashMap::with_capacity(chains.len());
    for chain in chains {
        neighbors.entry(chain.hash).or_default();
        for dependency in &chain.split_dependencies {
            if !chain_ids.contains(dependency) {
                continue;
            }
            neighbors.entry(chain.hash).or_default().insert(*dependency);
            neighbors.entry(*dependency).or_default().insert(chain.hash);
        }
    }

    let mut visited = HashSet::with_capacity(chains.len());
    let mut slow_dep_chain_ids = HashSet::new();
    for chain in chains {
        if visited.contains(&chain.hash) {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![chain.hash];
        visited.insert(chain.hash);
        while let Some(current) = stack.pop() {
            component.push(current);
            if let Some(next_neighbors) = neighbors.get(&current) {
                for next in next_neighbors {
                    if visited.insert(*next) {
                        stack.push(*next);
                    }
                }
            }
        }

        let component_ops =
            component.iter().fold(0_u64, |sum, dep_chain_id| {
                sum.saturating_add(
                    dependent_ops_by_chain
                        .get(dep_chain_id)
                        .copied()
                        .unwrap_or(0),
                )
            });
        if component_ops > max_per_chain {
            slow_dep_chain_ids.extend(component);
        }
    }
    slow_dep_chain_ids
}

/// pg_notify channel announcing a fully-ingested block.
///
/// Must stay in sync with `consensus_detector::NEW_BLOCK_CHANNEL`. Snake_case
/// per the channel-name convention.
const NEW_BLOCK_CHANNEL: &str = "event_new_block";

fn is_valid_fallback_dst_handle(
    dst_handle: &[u8; 32],
    chain_id: ChainId,
) -> bool {
    let embedded = chain_id_from_handle(dst_handle);
    if embedded != chain_id.as_u64() {
        warn!(
            dst_handle = ?dst_handle,
            embedded_chain_id = embedded,
            chain_id = %chain_id,
            "Ignoring FallbackGrantedPlaintext: dstHandle chain id does not match this chain"
        );
        return false;
    }
    if dst_handle[21] != COMPUTED_HANDLE_INDEX_MARKER {
        warn!(
            dst_handle = ?dst_handle,
            "Ignoring FallbackGrantedPlaintext: dstHandle is missing the computed-handle marker"
        );
        return false;
    }
    if dst_handle[31] != HANDLE_VERSION {
        warn!(
            dst_handle = ?dst_handle,
            "Ignoring FallbackGrantedPlaintext: dstHandle has an unexpected handle version"
        );
        return false;
    }
    // Restrict to the same allowlist the contract
    // enforces: Bool(0), Uint8(2), Uint16(3), Uint32(4), Uint64(5), Uint128(6),
    // Uint160(7), Uint256(8). Anything else is rejected.
    let to_type = dst_handle[30];
    if !matches!(to_type, 0 | 2..=8) {
        warn!(
            dst_handle = ?dst_handle,
            to_type,
            "Ignoring FallbackGrantedPlaintext: unsupported FheType in dstHandle"
        );
        return false;
    }
    true
}

#[allow(clippy::too_many_arguments)]
pub async fn ingest_block_logs(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
    kms_generation_contract_address: &Option<Address>,
    protocol_config_contract_address: &Option<Address>,
    confidential_bridge_address: &Option<Address>,
    options: IngestOptions,
) -> Result<(), sqlx::Error> {
    let Some(mut tx) = db.new_transaction().await? else {
        info!("cutover completed — host-listener skipping block ingest (retired stack)");
        return Ok(());
    };

    // Queue `pg_notify('event_new_block', ...)` at the top of the transaction so
    // postgres defers delivery until `tx.commit()` below succeeds. Same
    // "after all events committed" guarantee as emitting post-commit, but
    // atomic with the data — if the tx rolls back, the notification is
    // discarded too. JSON shape must match consensus_detector::NewBlockPayload.
    let new_block_payload = serde_json::json!({
        "chain_id": chain_id.as_u64() as i64,
        "block_height": block_logs.summary.number as i64,
        "block_hash": format!("{:#x}", block_logs.summary.hash),
    })
    .to_string();
    info!(
        channel = NEW_BLOCK_CHANNEL,
        payload = %new_block_payload,
        "Queuing new_block pg_notify in ingest transaction"
    );
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(NEW_BLOCK_CHANNEL)
        .bind(&new_block_payload)
        .execute(&mut *tx)
        .await?;

    // Only the listener watching the configured canonical chain decodes
    // `CoprocessorUpgradeProposed`; every other listener skips the channel.
    let is_protocol_config_listener = options.is_protocol_config_listener;

    let mut is_allowed = HashSet::<Handle>::new();
    let mut seen_fallback_handles = HashSet::<Handle>::new();
    let mut acl_event_log = vec![];
    let mut tfhe_event_log = vec![];
    let mut kms_gen_events = vec![];
    let block_hash = block_logs.summary.hash;
    let block_number = block_logs.summary.number;
    let mut catchup_insertion = 0;
    let block_timestamp = block_date_time_utc(block_logs.summary.timestamp);
    let mut at_least_one_insertion = false;
    // Per-block tallies persisted in host_chain_blocks_valid. Counted at decode
    // time, so an event that fails to insert (e.g. ON CONFLICT) still counts.
    let mut allow_event_count: i32 = 0;
    let mut fhe_event_count: i32 = 0;

    // GCS only: on one designated block of each chain's dry-run window, append a
    // deterministic synthetic transaction so a quiet chain can still anchor consensus. The
    // logs go through the normal decode below, so `fhe_event_count`, `is_allowed`, the
    // dependence chain and `schedule_order` are all derived by the production path. Blue
    // never injects, so `public` stays untouched. See `database::synthetic_ops`.
    let synthetic = if db.gcs_mode() {
        synthetic_logs_for_block(
            &mut tx,
            chain_id,
            block_number,
            block_hash,
            *tfhe_contract_address,
            *acl_contract_address,
            // Past every real log index in this block. `logs` can be a filtered subset of
            // the block's logs, so its length is not an upper bound on their indices.
            block_logs
                .logs
                .iter()
                .filter_map(|log| log.log_index)
                .max()
                .map_or(0, |max| max.saturating_add(1)),
        )
        .await?
    } else {
        vec![]
    };
    if !synthetic.is_empty() {
        info!(
            chain_id = chain_id.as_u64(),
            block_number,
            count = synthetic.len(),
            gcs_mode = db.gcs_mode(),
            "GCS: injecting synthetic consensus work into this block"
        );
    }

    for log in block_logs.logs.iter().chain(synthetic.iter()) {
        let current_address = Some(log.inner.address);
        let is_acl_address = &current_address == acl_contract_address;
        if acl_contract_address.is_none() || is_acl_address {
            if let Ok(event) =
                AclContract::AclContractEvents::decode_log(&log.inner)
            {
                allow_event_count = allow_event_count.saturating_add(1);
                let handles = acl_result_handles(&event);
                for handle in handles {
                    is_allowed.insert(handle.to_vec());
                }
                acl_event_log.push((event, log.transaction_hash));
                continue;
            }
        }

        let is_tfhe_address = &current_address == tfhe_contract_address;
        if tfhe_contract_address.is_none() || is_tfhe_address {
            if let Ok(event) =
                TfheContract::TfheContractEvents::decode_log(&log.inner)
            {
                fhe_event_count = fhe_event_count.saturating_add(1);
                let log = LogTfhe {
                    event,
                    transaction_hash: log.transaction_hash,
                    block_number,
                    block_hash,
                    block_timestamp,
                    // updated in the next loop and dependence_chains
                    is_allowed: false,
                    dependence_chain: Default::default(),
                    tx_depth_size: 0,
                    log_index: log.log_index,
                    operand_boundary_mask: None,
                    is_executor_minted: true,
                };
                tfhe_event_log.push(log);
                continue;
            }
        }

        let is_kms_gen_address =
            &current_address == kms_generation_contract_address;
        if is_kms_gen_address {
            if let Ok(event) =
                KMSGeneration::KMSGenerationEvents::decode_log(&log.inner)
            {
                kms_gen_events.push((event.data, log.clone()));
                continue;
            } else {
                KMS_EVENT_DECODE_FAIL_COUNTER.inc()
            }
        }

        let is_protocol_config_address = is_protocol_config_listener
            && protocol_config_contract_address
                .as_ref()
                .is_some_and(|addr| &log.inner.address == addr);
        if is_protocol_config_address {
            handle_protocol_config_log(&mut tx, chain_id, log).await?;
            continue;
        }

        let is_bridge_address = &current_address == confidential_bridge_address;
        if is_bridge_address {
            if let Ok(event) =
                BridgeContract::BridgeContractEvents::decode_log(&log.inner)
            {
                // A FallbackGrantedPlaintext becomes a synthetic TrivialEncrypt
                // computation so the normal pipeline materializes the ciphertext.
                // PBS is enqueued so its ct128/digest get computed and published.
                if let BridgeContract::BridgeContractEvents::FallbackGrantedPlaintext(e) =
                    &event.data
                {
                    let dst_handle = e.dstHandle;
                    if !is_valid_fallback_dst_handle(&dst_handle.0, chain_id) {
                        continue;
                    }
                    // Record the observation durably (keyed by block hash)
                    // regardless of the synthesis decision below: reorg
                    // cleanup and operators need the grant to survive even
                    // when this particular observation is suppressed.
                    db.record_fallback_grant_observation(
                        &mut tx,
                        dst_handle.as_slice(),
                        &e.plaintext.to_be_bytes::<32>(),
                        &log.transaction_hash,
                        block_number,
                        block_hash.as_ref(),
                    )
                    .await?;
                    // Materialization is finality-gated: once the async
                    // compute pipeline picks up the synthetic computation its
                    // ciphertext cannot be retracted on a reorg, and whether
                    // those bytes or a later bridge copy win the copy's
                    // ON CONFLICT would then depend on per-node fork
                    // visibility — a fleet consensus hazard. The observation
                    // above is durable either way; synthesis for a
                    // not-yet-final block happens when the block finalizes
                    // (see `synthesize_finalized_fallback_grants`).
                    if !block_logs.finalized {
                        info!(
                            dst_handle = ?dst_handle,
                            block_number,
                            "Deferring FallbackGrantedPlaintext synthesis until finality"
                        );
                        continue;
                    }
                    // The contract specifies that if multiple fallback events
                    // are emitted for the same handle, only the first one is
                    // the source of truth: skip duplicates within this block
                    // and grants from a different transaction. The SAME grant
                    // re-observed in another block context (fork sibling or
                    // canonical re-inclusion after a reorg) is re-synthesized
                    // idempotently (every insert no-ops on its conflict key).
                    // A handle materialized by a bridge association
                    // (ciphertext copy without a computations row) also stays
                    // write-once.
                    let first_in_block =
                        seen_fallback_handles.insert(dst_handle.to_vec());
                    if !first_in_block
                        || db
                            .fallback_grant_conflicts(
                                &mut tx,
                                dst_handle.as_slice(),
                                &log.transaction_hash,
                            )
                            .await?
                    {
                        warn!(
                            dst_handle = ?dst_handle,
                            "Ignoring FallbackGrantedPlaintext: dstHandle is already materialized"
                        );
                        continue;
                    }
                    // Force the handle allowed so the synthetic computation runs.
                    // governance ensures the handle is in the ACL.
                    is_allowed.insert(dst_handle.to_vec());
                    tfhe_event_log.push(LogTfhe {
                        event: alloy::primitives::Log {
                            address: log.inner.address,
                            data: TfheContract::TfheContractEvents::TrivialEncrypt(
                                TfheContract::TrivialEncrypt {
                                    caller: Address::ZERO,
                                    pt: e.plaintext,
                                    toType: dst_handle.0[30],
                                    result: dst_handle,
                                },
                            ),
                        },
                        transaction_hash: log.transaction_hash,
                        block_number,
                        block_hash,
                        block_timestamp,

                        // This is a placeholder. The real value can't be known yet
                        // because the is_allowed set is still being built from
                        // the rest of the block's logs. It is recomputed for
                        // every event in the loop right after this one.
                        is_allowed: false,

                        // Placeholders: dependence_chains() (called once the
                        // whole block is scanned) assigns the real dependence
                        // chain this op belongs to and its depth within it.
                        dependence_chain: Default::default(),
                        tx_depth_size: 0,

                        log_index: log.log_index,
                        operand_boundary_mask: None,
                        // This row is synthesized by the listener after a
                        // bridge event; the executor never called
                        // `_markMinted` for it.
                        is_executor_minted: false,
                    });
                    at_least_one_insertion |= db
                        .insert_pbs_computations(
                            &mut tx,
                            &[dst_handle.to_vec()],
                            log.transaction_hash.map(|h| h.to_vec()),
                            block_number,
                        )
                        .await?;
                } else {
                    at_least_one_insertion |= db
                        .handle_bridge_event(
                            &mut tx,
                            &event,
                            &log.transaction_hash,
                            block_number,
                            &block_logs.summary.hash,
                            &block_logs.summary.parent_hash,
                            block_logs.summary.timestamp,
                            acl_contract_address,
                        )
                        .await?;
                }
                continue;
            }
        }

        if is_acl_address
            || is_tfhe_address
            || is_kms_gen_address
            || is_protocol_config_address
            || is_bridge_address
        {
            error!(
                event_address = ?log.inner.address,
                acl_contract_address = ?acl_contract_address,
                tfhe_contract_address = ?tfhe_contract_address,
                kms_generation_contract_address = ?kms_generation_contract_address,
                confidential_bridge_address = ?confidential_bridge_address,
                log = ?log,
                "Cannot decode event",
            );
        }
    }
    for tfhe_log in tfhe_event_log.iter_mut() {
        tfhe_log.is_allowed =
            if let Some(result_handle) = tfhe_result_handle(&tfhe_log.event) {
                is_allowed.contains(&result_handle.to_vec())
            } else {
                false
            };
    }

    // Must happen before dependence grouping and database insertion. The
    // boundary mask is consensus-critical execution metadata, so ordering or
    // provenance gaps are fatal for this block instead of falling back to
    // database-local inference.
    populate_operand_boundary_masks(&mut tfhe_event_log)?;

    let chains = dependence_chains(
        &mut tfhe_event_log,
        &db.dependence_chain,
        &db.consumed_boundaries,
        &db.sealed_chains,
        options.dependence_by_connexity,
        options.dependence_cross_block,
    )
    .await;

    let slow_lane_enabled = options.dependent_ops_max_per_chain > 0;
    let mut dependent_ops_by_chain: HashMap<ChainHash, u64> = HashMap::new();
    for tfhe_log in tfhe_event_log {
        let inserted = db.insert_tfhe_event(&mut tx, &tfhe_log).await?;
        at_least_one_insertion |= inserted;
        // Count all newly inserted ops per chain to avoid underestimating
        // pressure from producer paths that are required by downstream work.
        if slow_lane_enabled && inserted {
            dependent_ops_by_chain
                .entry(tfhe_log.dependence_chain)
                .and_modify(|count| *count = count.saturating_add(1))
                .or_insert(1);
        }
        if block_logs.catchup && inserted {
            info!(tfhe_log = ?tfhe_log, "TFHE event missed before");
            catchup_insertion += 1;
        } else {
            info!(tfhe_log = ?tfhe_log, "TFHE event");
        }
    }

    // ACL events are processed only after every tfhe compute event for this
    // block has been inserted, so a handle produced *and* allowed within this
    // same block already has its computation row when the allow is recorded.
    for (event, transaction_hash) in acl_event_log {
        let inserted = db
            .handle_acl_event(
                &mut tx,
                &event,
                &transaction_hash,
                &block_logs.summary,
            )
            .await?;
        at_least_one_insertion |= inserted;
        if block_logs.catchup && inserted {
            info!(
                acl_event = ?event,
                ?transaction_hash,
                ?block_number,
                "ACL event missed before"
            );
            catchup_insertion += 1;
        } else {
            info!(
                acl_event = ?event,
                ?transaction_hash,
                ?block_number,
                "ACL event"
            );
        }
    }

    let mut slow_dep_chain_ids: HashSet<ChainHash> = HashSet::new();
    if slow_lane_enabled {
        let max_per_chain = u64::from(options.dependent_ops_max_per_chain);
        slow_dep_chain_ids = classify_slow_by_split_dependency_closure(
            &chains,
            &dependent_ops_by_chain,
            max_per_chain,
        );

        let parent_dep_chain_ids = chains
            .iter()
            .flat_map(|chain| {
                chain
                    .split_dependencies
                    .iter()
                    .map(|dependency| dependency.to_vec())
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let existing_slow_parents = db
            .find_slow_dep_chain_ids(&mut tx, &parent_dep_chain_ids)
            .await?;
        slow_dep_chain_ids.extend(existing_slow_parents);
        propagate_slow_lane_to_dependents(&chains, &mut slow_dep_chain_ids);

        let slow_marked_chains = chains
            .iter()
            .filter(|chain| slow_dep_chain_ids.contains(&chain.hash))
            .count() as u64;
        db.record_slow_lane_marked_chains(slow_marked_chains);
    }

    if catchup_insertion > 0 {
        if catchup_insertion == block_logs.logs.len() {
            info!(
                block_number,
                catchup_insertion, "Catchup inserted a full block"
            );
        } else {
            info!(block_number, catchup_insertion, "Catchup inserted events");
        }
    }
    insert_kms_generation_events_tx(
        &mut tx,
        kms_gen_events,
        chain_id,
        block_hash.as_ref(),
        block_number,
    )
    .await?;
    db.mark_block_as_valid(
        &mut tx,
        &block_logs.summary,
        block_logs.finalized,
        fhe_event_count,
        allow_event_count,
    )
    .await?;
    if at_least_one_insertion {
        db.update_dependence_chain(
            &mut tx,
            chains,
            block_timestamp,
            &block_logs.summary,
            &slow_dep_chain_ids,
        )
        .await?;
    }
    tx.commit().await
}

/// Synthetic logs for this block, or empty if it is not the designated one.
///
/// GCS-only, and only for the block at `start_block + SYNTHETIC_BLOCK_OFFSET` of *this*
/// chain's window, while the proposal is still dry-running. Reads the window from
/// `upgrade_state` inside the ingest transaction — one indexed read per block on the green
/// listener only.
///
/// Everything the derivation consumes (`proposal_id`, chain id, block number) comes from
/// on-chain data, so every operator produces byte-identical logs. See
/// `database::synthetic_ops`.
async fn synthetic_logs_for_block(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    chain_id: ChainId,
    block_number: u64,
    block_hash: BlockHash,
    tfhe_contract_address: Option<Address>,
    acl_contract_address: Option<Address>,
    log_index_base: u64,
) -> Result<Vec<Log>, sqlx::Error> {
    let Ok(block_number_i64) = i64::try_from(block_number) else {
        return Ok(vec![]);
    };
    let Ok(chain_id_i64) = i64::try_from(chain_id.as_u64()) else {
        return Ok(vec![]);
    };

    let row: Option<(Vec<u8>, String, i64)> = sqlx::query_as(
        "SELECT proposal_id, version, start_block
           FROM upgrade_state
          WHERE stack_role = 'GCS'
            AND status = 'in_progress'
            AND state IN ('UpgradeActivated', 'DryRunStarted')
            AND host_chain_id = $1
            AND proposal_id IS NOT NULL
            AND version IS NOT NULL
            AND start_block IS NOT NULL",
    )
    .bind(chain_id_i64)
    .fetch_optional(tx.as_mut())
    .await?;

    let Some((proposal_id, target_version, start_block)) = row else {
        return Ok(vec![]);
    };
    if block_number_i64 != start_block.saturating_add(SYNTHETIC_BLOCK_OFFSET) {
        return Ok(vec![]);
    }

    let ctx = SyntheticContext {
        proposal_id: &proposal_id,
        target_version: &target_version,
        chain_id: chain_id.as_u64(),
        block_number: block_number_i64,
        block_hash,
    };

    // Record the transaction hash so cutover can find this work and delete it before the merge.
    // The hash is keccak over the whole context, and `block_hash` is not persisted anywhere the
    // upgrade-controller can read unambiguously, so it cannot be recomputed there — the injector
    // is the only component that knows it.
    //
    // APPENDED, never overwritten. Injection happens at `start_block + 1` with no wait for
    // finality, so a reorg re-injects on the replacement block: different block hash, different
    // transaction hash, a second set of synthetic rows. Overwriting would forget the first set
    // and let it merge into `public` as live data.
    //
    // Same transaction as the log decoding that follows, so the marker and the work it points at
    // commit together or not at all.
    let synthetic_txn_hash = synthetic_transaction_hash(&ctx);
    let recorded = sqlx::query(
        "UPDATE upgrade_state u
            SET synthetic_txn_hashes = CASE
                    -- Offset-aligned membership test, not a `position()` substring search: a
                    -- 32-byte needle could straddle two stored hashes, and a false positive
                    -- there would skip the append and leak that fork's rows. Alignment makes
                    -- the comparison exact, and keeps re-ingesting the same block idempotent.
                    WHEN EXISTS (
                        SELECT 1
                          FROM generate_series(
                                   1, octet_length(u.synthetic_txn_hashes), 32
                               ) AS g(pos)
                         WHERE substring(u.synthetic_txn_hashes FROM g.pos FOR 32) = $1
                    )
                    THEN u.synthetic_txn_hashes
                    ELSE u.synthetic_txn_hashes || $1
                END,
                updated_at = NOW()
          WHERE u.stack_role = 'GCS'
            AND u.status = 'in_progress'
            AND u.state IN ('UpgradeActivated', 'DryRunStarted')
            AND u.host_chain_id = $2
            -- Pin to the proposal the hash was derived from. Without it, a proposal activated
            -- between the SELECT above and this UPDATE would be stamped with a hash computed
            -- from the previous one, and cutover would then delete nothing.
            AND u.proposal_id = $3",
    )
    .bind(synthetic_txn_hash.as_slice())
    .bind(chain_id_i64)
    .bind(&proposal_id)
    .execute(tx.as_mut())
    .await?
    .rows_affected();

    // No marker recorded means no injection. The row this UPDATE targets was read a few lines
    // above, but statements run at READ COMMITTED, so a concurrent FSM write — a rollback to
    // `PAUSED`, `transition_to_dry_run_started`, a replacement proposal — can move it in between
    // and leave the predicate matching nothing.
    //
    // Returning no logs is what keeps the invariant exact: cutover deletes synthetic work by
    // marker, so work injected without one would survive into `public` as live data. Skipping
    // this block costs nothing, because the trigger is a floor rather than an exact match and
    // the next tick re-evaluates.
    if recorded == 0 {
        warn!(
            chain_id = chain_id.as_u64(),
            block_number = block_number_i64,
            synthetic_txn_hash = %alloy::primitives::hex::encode(synthetic_txn_hash),
            "GCS: upgrade_state moved while injecting; skipping synthetic ops for this block"
        );
        return Ok(vec![]);
    }

    Ok(synthetic_logs(
        &ctx,
        tfhe_contract_address,
        acl_contract_address,
        log_index_base,
    ))
}

/// Channel name the upgrade-controller LISTENs on for `CoprocessorUpgradeProposed` events.
const UPGRADE_ACTIVATED_CHANNEL: &str = "event_upgrade_activated";

/// Decodes a log known to come from the configured ProtocolConfig contract on
/// the authority chain and dispatches it. Caller must pre-gate on
/// `is_protocol_config_listener && log.address == protocol_config_contract_address`.
async fn handle_protocol_config_log(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    chain_id: ChainId,
    log: &Log,
) -> Result<(), sqlx::Error> {
    match ProtocolConfig::ProtocolConfigEvents::decode_log(&log.inner) {
        Ok(event) => match &event.data {
            ProtocolConfig::ProtocolConfigEvents::CoprocessorUpgradeProposed(proposed) => {
                let Some(proposal_block) =
                    log.block_number.and_then(|b| i64::try_from(b).ok())
                else {
                    warn!(
                        proposal_id = %proposed.proposalId,
                        "Ignoring CoprocessorUpgradeProposed without a valid block number"
                    );
                    return Ok(());
                };
                notify_coprocessor_upgrade_proposed(tx, chain_id, proposed, proposal_block).await?;
            }
            _ => {
                // ProtocolConfigEvents has no Debug impl; topic0 identifies
                // the unhandled variant.
                warn!(
                    topic0 = ?log.topic0(),
                    block_number = ?log.block_number,
                    tx_hash = ?log.transaction_hash,
                    log_index = ?log.log_index,
                    "ProtocolConfig event decoded but no handler matched — likely a new variant added without updating host-listener",
                );
                PROTOCOL_CONFIG_EVENT_DECODE_FAIL_COUNTER.inc();
            }
        },
        Err(_) => {
            PROTOCOL_CONFIG_EVENT_DECODE_FAIL_COUNTER.inc();
        }
    }
    Ok(())
}

async fn notify_coprocessor_upgrade_proposed(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    chain_id: ChainId,
    event: &ProtocolConfig::CoprocessorUpgradeProposed,
    proposal_block: i64,
) -> Result<(), sqlx::Error> {
    let listener_chain_id = chain_id.as_u64();
    let Ok(listener_chain_id_i64) = i64::try_from(listener_chain_id) else {
        warn!(
            listener_chain_id,
            "Rejecting CoprocessorUpgradeProposed: listener chain id exceeds i64 range"
        );
        return Ok(());
    };

    if event.proposalId.is_zero() {
        warn!(
            chain_id = listener_chain_id,
            "Rejecting CoprocessorUpgradeProposed with proposalId == 0 — production contract guards against this; defense in depth against test mocks or future callers"
        );
        return Ok(());
    }

    let proposal_id_bytes = event.proposalId.to_be_bytes::<32>();
    let proposal_id_hex =
        format!("0x{}", alloy_primitives::hex::encode(proposal_id_bytes));

    // gwStartBlock is a single top-level field shared by every per-chain window.
    let Ok(gw_start_block) = i64::try_from(event.gwStartBlock) else {
        warn!(
            listener_chain_id,
            proposal_id = %proposal_id_hex,
            gw_start_block = event.gwStartBlock,
            "Rejecting CoprocessorUpgradeProposed: gwStartBlock exceeds i64 range"
        );
        return Ok(());
    };

    if event.chainUpgradeWindows.is_empty() {
        warn!(
            listener_chain_id,
            proposal_id = %proposal_id_hex,
            "CoprocessorUpgradeProposed carries no chain windows — nothing to activate"
        );
        return Ok(());
    }

    // Validate and materialize the complete set before touching durable state.
    // Rejecting the whole event avoids a partially installed proposal.
    let mut seen_chain_ids = HashSet::new();
    let mut windows = Vec::with_capacity(event.chainUpgradeWindows.len());
    for window in &event.chainUpgradeWindows {
        let (Ok(window_chain_id), Ok(start_block), Ok(end_block)) = (
            i64::try_from(window.chainId),
            i64::try_from(window.startBlock),
            i64::try_from(window.endBlock),
        ) else {
            warn!(
                listener_chain_id,
                proposal_id = %proposal_id_hex,
                window_chain_id = window.chainId,
                start_block = window.startBlock,
                end_block = window.endBlock,
                "Rejecting CoprocessorUpgradeProposed: chain/block field exceeds i64 range"
            );
            return Ok(());
        };
        if start_block > end_block {
            warn!(
                listener_chain_id,
                proposal_id = %proposal_id_hex,
                window_chain_id,
                start_block,
                end_block,
                "Rejecting CoprocessorUpgradeProposed: start_block is after end_block"
            );
            return Ok(());
        }
        if !seen_chain_ids.insert(window_chain_id) {
            warn!(
                listener_chain_id,
                proposal_id = %proposal_id_hex,
                window_chain_id,
                "Rejecting CoprocessorUpgradeProposed: duplicate host chain window"
            );
            return Ok(());
        }
        windows.push((window_chain_id, start_block, end_block));
    }
    windows.sort_unstable_by_key(|&(window_chain_id, _, _)| window_chain_id);

    let Some(&(_, canonical_start, canonical_end)) =
        windows.iter().find(|&&(window_chain_id, _, _)| {
            window_chain_id == listener_chain_id_i64
        })
    else {
        warn!(
            listener_chain_id,
            proposal_id = %proposal_id_hex,
            nb_windows = windows.len(),
            "Rejecting CoprocessorUpgradeProposed: proposal does not include the canonical listener chain"
        );
        return Ok(());
    };

    // ProtocolConfig ingestion replaces a proposal-wide row set. Serialize it
    // with controller transitions and concurrent listeners so readers can
    // never observe only a subset of the proposed chains.
    sqlx::query("LOCK TABLE upgrade_state IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut **tx)
        .await?;

    type ExistingWindow = (
        String,
        Option<Vec<u8>>,
        Option<i64>,
        i64,
        Option<i64>,
        Option<i64>,
        Option<String>,
        Option<i64>,
    );
    let existing: Vec<ExistingWindow> = sqlx::query_as(
        "SELECT status, proposal_id, proposal_block, host_chain_id,
                start_block, end_block, version, gw_start_block
           FROM upgrade_state
          WHERE stack_role = 'GCS'
          ORDER BY host_chain_id
          FOR UPDATE",
    )
    .fetch_all(&mut **tx)
    .await?;

    let same_attempt = !existing.is_empty()
        && existing.iter().all(
            |(_, existing_id, existing_block, _, _, _, _, _)| {
                existing_id.as_deref() == Some(&proposal_id_bytes[..])
                    && *existing_block == Some(proposal_block)
            },
        );
    if same_attempt {
        let same_windows = existing.len() == windows.len()
            && existing.iter().zip(&windows).all(
                |(
                    (
                        _,
                        _,
                        _,
                        existing_chain,
                        existing_start,
                        existing_end,
                        existing_version,
                        existing_gw_start,
                    ),
                    (chain, start, end),
                )| {
                    *existing_chain == *chain
                        && *existing_start == Some(*start)
                        && *existing_end == Some(*end)
                        && existing_version.as_deref()
                            == Some(event.softwareVersion.as_str())
                        && *existing_gw_start == Some(gw_start_block)
                },
            );
        if same_windows {
            debug!(
                proposal_id = %proposal_id_hex,
                proposal_block,
                "Ignoring exact replay of CoprocessorUpgradeProposed"
            );
        } else {
            warn!(
                proposal_id = %proposal_id_hex,
                proposal_block,
                "Rejecting CoprocessorUpgradeProposed: an existing attempt has different proposal data"
            );
        }
        return Ok(());
    }

    let can_replace = existing.is_empty()
        || existing.iter().all(
            |(status, existing_id, existing_block, _, _, _, _, _)| {
                existing_block.is_none_or(|block| proposal_block > block)
                    && (status == "failed"
                        || (status == "completed"
                            && existing_id.as_deref()
                                != Some(&proposal_id_bytes[..])))
            },
        );
    if !can_replace {
        warn!(
            proposal_id = %proposal_id_hex,
            proposal_block,
            "Rejected event_upgrade_activated: another proposal is active, completed, or newer"
        );
        return Ok(());
    }

    sqlx::query("DELETE FROM upgrade_state WHERE stack_role IN ('BCS', 'GCS')")
        .execute(&mut **tx)
        .await?;

    for &(window_chain_id, start_block, end_block) in &windows {
        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id,
                host_consensus_reached, gw_consensus_reached,
                gw_dry_run_started, proposal_block, last_error, updated_at
            )
            VALUES (
                'GCS', 'UpgradeActivated', 'in_progress', $1, $2,
                $3, $4, $5, $6, FALSE, FALSE, FALSE, $7, NULL, NOW()
            )
            "#,
        )
        .bind(&proposal_id_bytes[..])
        .bind(&event.softwareVersion)
        .bind(start_block)
        .bind(end_block)
        .bind(gw_start_block)
        .bind(window_chain_id)
        .bind(proposal_block)
        .execute(&mut **tx)
        .await?;
    }

    info!(
        proposal_id = %proposal_id_hex,
        software_version = %event.softwareVersion,
        chains = windows.len(),
        gw_start_block = event.gwStartBlock,
        "Persisted CoprocessorUpgradeProposed atomically, emitting pg_notify('event_upgrade_activated')"
    );

    // One wake-up for the complete proposal; the controller reconciles from
    // the durable per-chain rows.
    let payload = serde_json::json!({
        "proposal_id":    &proposal_id_hex,
        "chain_id":       listener_chain_id_i64,
        "start_block":    canonical_start,
        "end_block":      canonical_end,
        "gw_start_block": gw_start_block,
        "version":        &event.softwareVersion,
    });
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(UPGRADE_ACTIVATED_CHANNEL)
        .bind(payload.to_string())
        .execute(&mut **tx)
        .await?;

    Ok(())
}

/// Synthesizes the pending fallback-grant materializations for a block that
/// just finalized. Ingest records every `FallbackGrantedPlaintext`
/// observation durably but defers the synthetic TrivialEncrypt for
/// not-yet-final blocks: once the async compute pipeline materializes a
/// ciphertext it cannot be retracted on a reorg, and whether the fallback
/// bytes or a later bridge copy win the copy's ON CONFLICT would then depend
/// on per-node fork visibility — a fleet consensus hazard. Finalized blocks
/// are fleet-uniform, so synthesizing here is safe.
///
/// Idempotent: the computation insert no-ops on
/// `(output_handle, transaction_id)`, the PBS insert on its own key, and
/// `fallback_grant_conflicts` keeps the contract's first-grant-wins
/// semantics across transactions and against bridge-copied handles.
///
/// The synthetic op is dependency-free, so its dependence chain is a
/// singleton; the cross-block producer cache is deliberately not primed
/// (consumers in blocks ingested before finality already treated the handle
/// as an external producer).
///
/// Like every finality-gated feature (state-hash stamping, the bridge
/// src-finality gate, KMS activations), this runs only where finalization
/// runs: a consumer-mode (broker-fed) ingest must be paired with a
/// finalizing listener/poller or deferred grants never materialize.
pub async fn synthesize_finalized_fallback_grants(
    db: &Database,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    block_number: i64,
    block_hash: &BlockHash,
) -> Result<(), sqlx::Error> {
    use sqlx::Row;
    let rows = sqlx::query(
        "SELECT dst_handle, plaintext, transaction_id, created_at
           FROM fallback_granted_events
          WHERE dst_chain_id = $1 AND block_hash = $2
          ORDER BY id",
    )
    .bind(db.chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_all(&mut **tx)
    .await?;
    if rows.is_empty() {
        return Ok(());
    }
    let mut logs: Vec<LogTfhe> = Vec::with_capacity(rows.len());
    for (row_index, row) in rows.into_iter().enumerate() {
        let dst_handle: Vec<u8> = row.get("dst_handle");
        let plaintext: Vec<u8> = row.get("plaintext");
        let transaction_id: Option<Vec<u8>> = row.get("transaction_id");
        let created_at: OffsetDateTime = row.get("created_at");
        let Ok(handle_bytes) = <[u8; 32]>::try_from(dst_handle.as_slice())
        else {
            warn!(?dst_handle, "Skipping fallback grant with malformed handle");
            continue;
        };
        // Re-validated at synthesis time so a rule tightened between
        // observation and finality is enforced.
        if !is_valid_fallback_dst_handle(&handle_bytes, db.chain_id) {
            continue;
        }
        let transaction_hash = transaction_id
            .as_deref()
            .and_then(|t| <[u8; 32]>::try_from(t).ok())
            .map(alloy::primitives::FixedBytes::from);
        if db
            .fallback_grant_conflicts(tx, &handle_bytes, &transaction_hash)
            .await?
        {
            warn!(
                dst_handle = ?handle_bytes,
                "Skipping finalized FallbackGrantedPlaintext: dstHandle is already materialized"
            );
            continue;
        }
        // The computation row needs a transaction id (NOT NULL, part of the
        // primary key), but `fallback_granted_events.transaction_id` is
        // nullable and None is a normal value here. A grant observed without
        // one gets the deterministic zero sentinel: for a synthesized row
        // the id is bookkeeping — the handle and its bytes are what
        // consensus sees — and dropping the grant instead would trade a
        // block wedge for a silent per-handle liveness hole.
        let transaction_hash =
            transaction_hash.or(Some(alloy::primitives::FixedBytes::ZERO));
        // `created_at` (the observation's ingest time) stands in for the
        // block timestamp, which host_chain_blocks_valid does not record. It
        // only feeds scheduling hints (schedule_order, chain last_updated_at),
        // never consensus-compared data.
        let data = TfheContract::TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller: Address::ZERO,
                pt: alloy::primitives::U256::from_be_slice(&plaintext),
                toType: handle_bytes[30],
                result: alloy::primitives::FixedBytes::from(handle_bytes),
            },
        );
        // Derived per event from its shape with an EMPTY minted set, NOT
        // through the block-level derivation:
        // `fallback_granted_events.transaction_id` is nullable and None is a
        // normal value on this path (tolerated by the conflict check and the
        // pbs insert below), while the block-level pass fail-closes on a
        // missing transaction hash — one NULL row would wedge the
        // finalization loop forever to compute a value that cannot depend on
        // the missing input. The empty closure is correct by construction,
        // not just for today's shape: the executor never ran
        // (`is_executor_minted: false`), so nothing synthesized can be a
        // transaction-local operand — the TrivialEncrypt has no encrypted
        // operands and derives the all-zero mask, and if the synthesized
        // shape ever gained one, every bit would correctly derive as
        // boundary.
        let operand_boundary_mask =
            operand_boundary_mask_from_minted(&data, |_| false)
                .map_err(sqlx::Error::Protocol)?;
        logs.push(LogTfhe {
            event: alloy::primitives::Log {
                address: Address::ZERO,
                data,
            },
            transaction_hash,
            // Forced allowed, exactly like inline synthesis: governance
            // ensures the handle is in the ACL.
            is_allowed: true,
            block_number: block_number as u64,
            block_hash: *block_hash,
            block_timestamp: PrimitiveDateTime::new(
                created_at.date(),
                created_at.time(),
            ),
            tx_depth_size: 0,
            dependence_chain: Default::default(),
            // Deterministic (ORDER BY id = observation order); a real index
            // also keeps ensure_logs_order from warning on every pass.
            log_index: Some(row_index as u64),
            operand_boundary_mask: Some(operand_boundary_mask),
            // Synthesized by the listener from a finalized bridge grant; the
            // executor never ran, so `_markMinted` never recorded this
            // handle and it must not become a same-transaction minted
            // operand for a later real executor operation.
            is_executor_minted: false,
        });
    }
    if logs.is_empty() {
        return Ok(());
    }
    let block_timestamp = logs[0].block_timestamp;
    // Dependency-free singletons: connexity/cross-block grouping options
    // cannot change the outcome, so neither flag is threaded through here.
    let chains = dependence_chains(
        &mut logs,
        &db.dependence_chain,
        &db.consumed_boundaries,
        &db.sealed_chains,
        false,
        false,
    )
    .await;
    for log in &logs {
        let dst_handle = tfhe_result_handle(&log.event)
            .expect("synthetic TrivialEncrypt has a result handle");
        db.insert_tfhe_event(tx, log).await?;
        db.insert_pbs_computations(
            tx,
            &[dst_handle.to_vec()],
            log.transaction_hash.map(|h| h.to_vec()),
            block_number as u64,
        )
        .await?;
        info!(
            dst_handle = ?dst_handle,
            block_number,
            "Synthesized finalized FallbackGrantedPlaintext"
        );
    }
    let summary = BlockSummary {
        number: block_number as u64,
        hash: *block_hash,
        // Only `hash` and `number` are read by update_dependence_chain.
        parent_hash: BlockHash::ZERO,
        timestamp: 0,
    };
    db.update_dependence_chain(
        tx,
        chains,
        block_timestamp,
        &summary,
        &HashSet::new(),
    )
    .await?;
    Ok(())
}

pub async fn update_finalized_blocks(
    db: &mut Database,
    log_iter: &mut InfiniteLogIter,
    last_block_number: u64,
    finality_lag: u64,
) {
    let log_iter = &*log_iter;
    update_finalized_blocks_aux(
        db,
        last_block_number,
        finality_lag,
        |block_number| async move {
            log_iter
                .get_block_by_number(block_number)
                .await
                .map(|block| block.header.hash)
        },
    )
    .await;
}

pub async fn update_finalized_blocks_aux<GetBlockHash, GetBlockHashFuture>(
    db: &mut Database,
    last_block_number: u64,
    finality_lag: u64,
    mut get_block_hash_by_number: GetBlockHash,
) where
    GetBlockHash: FnMut(u64) -> GetBlockHashFuture,
    GetBlockHashFuture: Future<Output = anyhow::Result<BlockHash>>,
{
    info!(last_block_number, finality_lag, "Updating finalized blocks");
    let last_finalized_block = last_block_number.saturating_sub(finality_lag);

    // Read the candidate numbers in a short transaction, then resolve the
    // canonical hashes over RPC with NO transaction open: block fetches can
    // take seconds each, and holding the finalization transaction across the
    // round-trips kept its row locks pinned for the whole time.
    let blocks_number = {
        let mut tx = match db.new_transaction().await {
            Ok(Some(tx)) => tx,
            Ok(None) => {
                info!(
                    "cutover completed — skipping finalized-blocks lookup (retired stack)"
                );
                return;
            }
            Err(err) => {
                error!(
                    ?err,
                    "Failed to create transaction for finalized blocks update"
                );
                return;
            }
        };
        match Database::get_finalized_blocks_number(
            &mut tx,
            last_finalized_block as i64,
            db.chain_id,
        )
        .await
        {
            Ok(numbers) => numbers,
            Err(err) => {
                error!(
                    ?err,
                    last_finalized_block,
                    "Failed to fetch finalized blocks number"
                );
                return;
            }
        }
    };
    info!(?blocks_number, "Finalizing blocks");

    // Ascending: finalization verifies each block's parent linkage against
    // its finalized predecessor, so within one batch the predecessor must be
    // finalized first.
    let mut blocks_number: Vec<i64> = blocks_number.into_iter().collect();
    blocks_number.sort_unstable();

    let mut canonical = Vec::with_capacity(blocks_number.len());
    for block_number in blocks_number {
        match get_block_hash_by_number(block_number as u64).await {
            Ok(block_hash) => canonical.push((block_number, block_hash)),
            Err(err) => {
                error!(
                    block_number,
                    ?err,
                    "Failed to fetch block for finalization, \
                     stopping the batch at the gap"
                );
                // STOP, don't skip: a gap at this height would let the next
                // height's parent-linkage check pass vacuously (no finalized
                // predecessor), the same hazard the refusal branch below
                // stops the batch for. The fetched prefix is still safe to
                // finalize; the rest retries next pass.
                break;
            }
        }
    }
    if canonical.is_empty() {
        return;
    }

    let mut tx = match db.new_transaction().await {
        Ok(Some(tx)) => tx,
        Ok(None) => {
            info!("cutover completed — skipping finalized-blocks update (retired stack)");
            return;
        }
        Err(err) => {
            error!(
                ?err,
                "Failed to create transaction for finalized blocks update"
            );
            return;
        }
    };
    for (block_number, block_hash) in canonical {
        match db
            .update_block_as_finalized(&mut tx, block_number, &block_hash)
            .await
        {
            Ok(Some(orphaned_hashes)) => {
                // Orphaned work/ACL rows are deliberately left in place
                // (pre-wave1 semantics): handles are fork-scoped by
                // construction, so orphaned state is unreferenced on the
                // canonical branch and benign. Bridge/authorization event
                // rows are keyed by observation block instead and must be
                // retracted with the branch that carried them.
                if let Err(err) = db
                    .retract_orphaned_event_state(&mut tx, &orphaned_hashes)
                    .await
                {
                    error!(
                        block_number,
                        ?err,
                        "Failed to retract orphaned event state during finalization"
                    );
                    return;
                }
                // The block just became final: materialize its deferred
                // fallback grants in the same transaction, so the synthesis
                // is atomic with the finalization it is gated on.
                if let Err(err) = synthesize_finalized_fallback_grants(
                    db,
                    &mut tx,
                    block_number,
                    &block_hash,
                )
                .await
                {
                    error!(
                        block_number,
                        ?err,
                        "Failed to synthesize finalized fallback grants"
                    );
                    return;
                }
            }
            Ok(None) => {
                // Finalization refused (missing row / orphaned / parent
                // linkage contradiction). STOP the batch: the next height's
                // linkage check would pass vacuously without a finalized
                // predecessor, letting a stale or poisoned RPC finalize a
                // fork block right behind the refusal. Earlier blocks of
                // this batch stay finalized; the rest retries next pass.
                warn!(
                    block_number,
                    "Stopping finalization batch at refused block"
                );
                break;
            }
            Err(err) => {
                error!(
                    block_number,
                    ?err,
                    "Failed to update block as finalized"
                );
                return;
            }
        }
    }
    if let Err(err) = tx.commit().await {
        error!(?err, "Failed to commit finalized blocks update");
        return;
    }
    // Notify the database of the new block
    // Delayed delegation rely on this signal to reconsider ready delegation
    if let Err(err) = db.block_notification().await {
        error!(error = %err, "Error notifying listener for new block");
    }
    // Best-effort maintenance: drop old finalized block rows nothing
    // references anymore, so ancestry probes and the table itself stop
    // growing with chain history. Failures only delay pruning.
    match db
        .prune_finalized_block_history(last_finalized_block as i64)
        .await
    {
        Ok(0) => {}
        Ok(pruned) => info!(pruned, "Pruned finalized block history"),
        Err(err) => error!(?err, "Failed to prune finalized block history"),
    }
}

#[cfg(test)]
mod tests {
    use crate::contracts::TfheContract;
    use crate::contracts::TfheContract::TfheContractEvents;
    use crate::database::tfhe_event_propagate::ClearConst;
    use alloy::primitives::{Address, FixedBytes};

    use super::*;

    fn fixture_chain(hash: u8, dependencies: &[u8]) -> Chain {
        Chain {
            hash: FixedBytes::<32>::from([hash; 32]),
            dependencies: dependencies
                .iter()
                .map(|dep| FixedBytes::<32>::from([*dep; 32]))
                .collect(),
            split_dependencies: dependencies
                .iter()
                .map(|dep| FixedBytes::<32>::from([*dep; 32]))
                .collect(),
            outer_boundary_handles: vec![],
            dependents: vec![],
            allowed_handle: vec![],
            size: 1,
            before_size: 0,
            new_chain: true,
        }
    }

    fn mask_log(
        event: TfheContractEvents,
        tx: TransactionHash,
        log_index: Option<u64>,
        is_executor_minted: bool,
    ) -> LogTfhe {
        LogTfhe {
            event: alloy::primitives::Log {
                address: Address::ZERO,
                data: event,
            },
            transaction_hash: Some(tx),
            is_allowed: true,
            block_number: 1,
            block_hash: FixedBytes::ZERO,
            block_timestamp: PrimitiveDateTime::MIN,
            tx_depth_size: 0,
            dependence_chain: tx,
            log_index,
            operand_boundary_mask: None,
            is_executor_minted,
        }
    }

    fn handle(byte: u8) -> EventHandle {
        FixedBytes::from([byte; 32])
    }

    #[test]
    fn derives_executor_compatible_masks_and_leaves_scalar_bits_clear() {
        let tx = handle(0x11);
        let local = handle(0x21);
        let boundary = handle(0x22);
        let scalar = handle(0x23);
        let mut logs = vec![
            mask_log(
                TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller: Address::ZERO,
                        pt: ClearConst::from(7_u8),
                        toType: 5,
                        result: local,
                    },
                ),
                tx,
                Some(1),
                true,
            ),
            // `local` is executor-minted earlier in this transaction, while
            // `boundary` is not: bit 0 stays clear and bit 1 is set.
            mask_log(
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller: Address::ZERO,
                    lhs: local,
                    rhs: boundary,
                    scalarByte: FixedBytes::ZERO,
                    result: handle(0x24),
                }),
                tx,
                Some(2),
                true,
            ),
            // A scalar RHS occupies dependency position 1 but must retain a
            // zero boundary bit even though its bytes are not minted.
            mask_log(
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller: Address::ZERO,
                    lhs: boundary,
                    rhs: scalar,
                    scalarByte: FixedBytes::from([1]),
                    result: handle(0x25),
                }),
                tx,
                Some(3),
                true,
            ),
        ];

        populate_operand_boundary_masks(&mut logs)
            .expect("ordered logs derive masks");
        assert_eq!(logs[0].operand_boundary_mask.unwrap()[31], 0);
        assert_eq!(logs[1].operand_boundary_mask.unwrap()[31], 0b10);
        assert_eq!(logs[2].operand_boundary_mask.unwrap()[31], 0b01);
    }

    #[test]
    fn synthetic_bridge_trivial_encrypt_is_not_treated_as_executor_minted() {
        let tx = handle(0x31);
        let synthetic = handle(0x32);
        let boundary = handle(0x33);
        let mut logs = vec![
            mask_log(
                TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller: Address::ZERO,
                        pt: ClearConst::from(7_u8),
                        toType: 5,
                        result: synthetic,
                    },
                ),
                tx,
                Some(1),
                false,
            ),
            mask_log(
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller: Address::ZERO,
                    lhs: synthetic,
                    rhs: boundary,
                    scalarByte: FixedBytes::ZERO,
                    result: handle(0x34),
                }),
                tx,
                Some(2),
                true,
            ),
        ];

        populate_operand_boundary_masks(&mut logs)
            .expect("ordered logs derive masks");
        assert_eq!(
            logs[1].operand_boundary_mask.unwrap()[31],
            0b11,
            "fallback synthesis was not marked in executor transient storage"
        );
    }

    #[test]
    fn rejects_missing_mask_provenance() {
        let mut logs = vec![mask_log(
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller: Address::ZERO,
                pt: ClearConst::from(7_u8),
                toType: 5,
                result: handle(0x41),
            }),
            handle(0x40),
            None,
            true,
        )];

        assert!(populate_operand_boundary_masks(&mut logs).is_err());
    }

    #[test]
    fn propagates_slow_lane_transitively_on_known_dependencies() {
        let chains = vec![
            fixture_chain(1, &[]),
            fixture_chain(2, &[1]),
            fixture_chain(3, &[2]),
            fixture_chain(4, &[]),
        ];
        let mut slow_dep_chain_ids = HashSet::from([chains[0].hash]);

        propagate_slow_lane_to_dependents(&chains, &mut slow_dep_chain_ids);

        assert!(slow_dep_chain_ids.contains(&chains[0].hash));
        assert!(slow_dep_chain_ids.contains(&chains[1].hash));
        assert!(slow_dep_chain_ids.contains(&chains[2].hash));
        assert!(!slow_dep_chain_ids.contains(&chains[3].hash));
    }

    #[test]
    fn classifies_slow_by_split_dependency_closure_sum() {
        let chains = vec![
            fixture_chain(1, &[]),
            fixture_chain(2, &[1]),
            fixture_chain(3, &[2]),
            fixture_chain(4, &[]),
        ];
        let dependent_ops_by_chain = HashMap::from([
            (chains[0].hash, 30_u64),
            (chains[1].hash, 20_u64),
            (chains[2].hash, 20_u64),
            (chains[3].hash, 10_u64),
        ]);

        let slow_dep_chain_ids = classify_slow_by_split_dependency_closure(
            &chains,
            &dependent_ops_by_chain,
            64,
        );

        assert!(slow_dep_chain_ids.contains(&chains[0].hash));
        assert!(slow_dep_chain_ids.contains(&chains[1].hash));
        assert!(slow_dep_chain_ids.contains(&chains[2].hash));
        assert!(!slow_dep_chain_ids.contains(&chains[3].hash));
    }

    // 4 independent chains each with exactly max_per_chain ops.
    // Since they are disconnected, each represents its own component.
    #[test]
    fn classify_slow_disconnected_components_at_threshold_are_fast() {
        let chains = vec![
            fixture_chain(1, &[]),
            fixture_chain(2, &[]),
            fixture_chain(3, &[]),
            fixture_chain(4, &[]),
        ];
        let max = 64_u64;
        let dependent_ops_by_chain = HashMap::from([
            (chains[0].hash, max),
            (chains[1].hash, max),
            (chains[2].hash, max),
            (chains[3].hash, max),
        ]);

        let slow = classify_slow_by_split_dependency_closure(
            &chains,
            &dependent_ops_by_chain,
            max,
        );

        assert!(
            slow.is_empty(),
            "no chain should be slow at exactly the threshold"
        );
    }

    // Single chain with exactly max_per_chain ops is not slow.
    // One more dep makes it fast.
    #[test]
    fn classify_slow_single_chain_at_boundary() {
        let chains = vec![fixture_chain(1, &[])];
        let max = 64_u64;

        let at_boundary = classify_slow_by_split_dependency_closure(
            &chains,
            &HashMap::from([(chains[0].hash, max)]),
            max,
        );
        assert!(
            at_boundary.is_empty(),
            "exactly at threshold should be fast"
        );

        let over_boundary = classify_slow_by_split_dependency_closure(
            &chains,
            &HashMap::from([(chains[0].hash, max + 1)]),
            max,
        );
        assert!(
            over_boundary.contains(&chains[0].hash),
            "one over threshold should be slow"
        );
    }

    // Non linear: A -> B, A -> C, B -> D, C -> D
    // Mark A slow, verify B, C, D all become slow via propagate_slow_lane_to_dependents.
    #[test]
    fn propagate_slow_lane_non_linear_dependency() {
        let chain_a = fixture_chain(1, &[]);
        let chain_b = fixture_chain(2, &[1]);
        let chain_c = fixture_chain(3, &[1]);
        let chain_d = fixture_chain(4, &[2, 3]);
        let chains = vec![chain_a, chain_b, chain_c, chain_d];

        let mut slow = HashSet::from([chains[0].hash]);
        propagate_slow_lane_to_dependents(&chains, &mut slow);

        assert!(slow.contains(&chains[0].hash), "A should be slow");
        assert!(
            slow.contains(&chains[1].hash),
            "B should be slow (depends on A)"
        );
        assert!(
            slow.contains(&chains[2].hash),
            "C should be slow (depends on A)"
        );
        assert!(
            slow.contains(&chains[3].hash),
            "D should be slow (depends on B and C)"
        );
    }

    #[test]
    fn proposal_id_max_uint256_round_trips_to_hex() {
        let proposal_id = alloy::primitives::U256::MAX;
        let bytes = proposal_id.to_be_bytes::<32>();
        let hex = format!("0x{}", alloy_primitives::hex::encode(bytes));
        assert_eq!(hex, format!("0x{}", "ff".repeat(32)));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn upgrade_upsert_uses_single_canonical_row() {
        use alloy::primitives::U256;
        use sqlx::postgres::PgPoolOptions;
        use sqlx::Row;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");

        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id, proposal_block, updated_at
            )
            VALUES ('BCS', 'PAUSED', 'completed', $1, 'v1',
                    100, 200, 1, 1, 100, NOW())
            "#,
        )
        .bind(&U256::from(1u64).to_be_bytes::<32>()[..])
        .execute(&pool)
        .await
        .expect("seed legacy row");

        let event = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(2u64),
            softwareVersion: "v2".to_string(),
            chainUpgradeWindows: vec![ProtocolConfig::ChainUpgradeWindow {
                chainId: 1,
                startBlock: 100,
                endBlock: 200,
            }],
            gwStartBlock: 1,
        };

        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &event,
            300,
        )
        .await
        .expect("upsert ok");
        tx.commit().await.expect("commit");

        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &event,
            300,
        )
        .await
        .expect("duplicate upsert ok");
        tx.commit().await.expect("commit");

        let row = sqlx::query(
            "SELECT stack_role, proposal_id, state, status,
                    (SELECT COUNT(*) FROM upgrade_state) AS row_count
               FROM upgrade_state",
        )
        .fetch_one(&pool)
        .await
        .expect("row");
        assert_eq!(row.try_get::<String, _>("stack_role").unwrap(), "GCS");
        assert_eq!(row.try_get::<i64, _>("row_count").unwrap(), 1);
        assert_eq!(
            row.try_get::<Vec<u8>, _>("proposal_id").unwrap(),
            U256::from(2u64).to_be_bytes::<32>().to_vec()
        );
        assert_eq!(
            row.try_get::<String, _>("state").unwrap(),
            "UpgradeActivated"
        );
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "in_progress");
    }

    /// A proposal with N windows atomically seeds N upgrade_state rows.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn upgrade_seeds_one_row_per_chain_window() {
        use alloy::primitives::U256;
        use sqlx::postgres::PgPoolOptions;
        use sqlx::Row;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");

        let event = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(2u64),
            softwareVersion: "v2".to_string(),
            chainUpgradeWindows: vec![
                ProtocolConfig::ChainUpgradeWindow {
                    chainId: 1,
                    startBlock: 100,
                    endBlock: 200,
                },
                ProtocolConfig::ChainUpgradeWindow {
                    chainId: 2,
                    startBlock: 300,
                    endBlock: 400,
                },
            ],
            gwStartBlock: 5,
        };

        let mut tx = pool.begin().await.expect("tx");
        // The canonical listener (chain 1) decodes the event and seeds *all*
        // windows, not just its own.
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &event,
            300,
        )
        .await
        .expect("seed ok");
        tx.commit().await.expect("commit");

        let rows = sqlx::query(
            "SELECT host_chain_id, start_block, end_block, gw_start_block,
                    proposal_id, proposal_block, state, status
               FROM upgrade_state
              WHERE stack_role = 'GCS'
              ORDER BY host_chain_id",
        )
        .fetch_all(&pool)
        .await
        .expect("rows");
        assert_eq!(rows.len(), 2);

        let expected = [(1_i64, 100_i64, 200_i64), (2_i64, 300_i64, 400_i64)];
        for (row, (chain, start, end)) in rows.iter().zip(expected) {
            assert_eq!(row.try_get::<i64, _>("host_chain_id").unwrap(), chain);
            assert_eq!(row.try_get::<i64, _>("start_block").unwrap(), start);
            assert_eq!(row.try_get::<i64, _>("end_block").unwrap(), end);
            assert_eq!(row.try_get::<i64, _>("gw_start_block").unwrap(), 5);
            assert_eq!(row.try_get::<i64, _>("proposal_block").unwrap(), 300);
            assert_eq!(
                row.try_get::<Vec<u8>, _>("proposal_id").unwrap(),
                U256::from(2u64).to_be_bytes::<32>().to_vec()
            );
            assert_eq!(
                row.try_get::<String, _>("state").unwrap(),
                "UpgradeActivated"
            );
            assert_eq!(
                row.try_get::<String, _>("status").unwrap(),
                "in_progress"
            );
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn upgrade_upsert_resets_gw_dry_run_started_for_new_gcs_proposal() {
        use alloy::primitives::U256;
        use sqlx::postgres::PgPoolOptions;
        use sqlx::Row;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");

        // Seed a completed GCS row from a prior cycle with gw_dry_run_started TRUE.
        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id,
                gw_dry_run_started, proposal_block, updated_at
            )
            VALUES ('GCS', 'LIVE', 'completed', $1, 'v1', 100, 200, 1, 1, TRUE, 100, NOW())
            "#,
        )
        .bind(&U256::from(1u64).to_be_bytes::<32>()[..])
        .execute(&pool)
        .await
        .expect("seed");

        let event = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(2u64),
            softwareVersion: "v2".to_string(),
            chainUpgradeWindows: vec![ProtocolConfig::ChainUpgradeWindow {
                chainId: 1,
                startBlock: 300,
                endBlock: 400,
            }],
            gwStartBlock: 5,
        };

        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &event,
            300,
        )
        .await
        .expect("upsert ok");
        tx.commit().await.expect("commit");

        let row = sqlx::query(
            "SELECT state, gw_dry_run_started, gw_start_block FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("row");
        assert_eq!(
            row.try_get::<String, _>("state").unwrap(),
            "UpgradeActivated"
        );
        assert!(
            !row.try_get::<bool, _>("gw_dry_run_started").unwrap(),
            "gw_dry_run_started must be reset for the new proposal"
        );
        assert_eq!(row.try_get::<i64, _>("gw_start_block").unwrap(), 5);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn upgrade_upsert_replayed_completed_proposal_is_noop() {
        use alloy::primitives::U256;
        use sqlx::postgres::PgPoolOptions;
        use sqlx::Row;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");

        // A completed GCS cycle for proposal 1.
        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id, proposal_block, updated_at
            )
            VALUES ('GCS', 'LIVE', 'completed', $1, 'v1', 100, 200, 1, 1, 100, NOW())
            "#,
        )
        .bind(&U256::from(1u64).to_be_bytes::<32>()[..])
        .execute(&pool)
        .await
        .expect("seed");

        let event = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(1u64),
            softwareVersion: "v1".to_string(),
            chainUpgradeWindows: vec![ProtocolConfig::ChainUpgradeWindow {
                chainId: 1,
                startBlock: 100,
                endBlock: 200,
            }],
            gwStartBlock: 1,
        };

        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &event,
            200,
        )
        .await
        .expect("no-op ok");
        tx.commit().await.expect("commit");

        let row = sqlx::query(
            "SELECT state, status FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("row");
        assert_eq!(row.try_get::<String, _>("state").unwrap(), "LIVE");
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "completed");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn upgrade_upsert_accepts_any_newer_proposal_after_failure() {
        use alloy::primitives::U256;
        use sqlx::postgres::PgPoolOptions;
        use sqlx::Row;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");

        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id, proposal_block,
                host_consensus_reached, gw_consensus_reached,
                gw_dry_run_started, last_error, updated_at
            )
            VALUES ('GCS', 'PAUSED', 'failed', $1, 'v1', 100, 200, 1, 1, 100,
                    TRUE, TRUE, TRUE, 'timeout', NOW())
            "#,
        )
        .bind(&U256::from(1u64).to_be_bytes::<32>()[..])
        .execute(&pool)
        .await
        .expect("seed");

        let same_id = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(1u64),
            softwareVersion: "v2".to_string(),
            chainUpgradeWindows: vec![ProtocolConfig::ChainUpgradeWindow {
                chainId: 1,
                startBlock: 300,
                endBlock: 400,
            }],
            gwStartBlock: 5,
        };
        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &same_id,
            200,
        )
        .await
        .expect("same-id retry");
        tx.commit().await.expect("commit");

        let row = sqlx::query(
            "SELECT status, proposal_block, host_consensus_reached,
                    gw_consensus_reached, gw_dry_run_started, last_error
               FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("row");
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "in_progress");
        assert_eq!(row.try_get::<i64, _>("proposal_block").unwrap(), 200);
        assert!(!row.try_get::<bool, _>("host_consensus_reached").unwrap());
        assert!(!row.try_get::<bool, _>("gw_consensus_reached").unwrap());
        assert!(!row.try_get::<bool, _>("gw_dry_run_started").unwrap());
        assert!(row
            .try_get::<Option<String>, _>("last_error")
            .unwrap()
            .is_none());

        sqlx::query(
            "UPDATE upgrade_state
                SET state = 'PAUSED', status = 'failed', last_error = 'timeout'
              WHERE stack_role = 'GCS'",
        )
        .execute(&pool)
        .await
        .expect("mark failed");

        let other_id = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(2u64),
            softwareVersion: "v3".to_string(),
            chainUpgradeWindows: vec![ProtocolConfig::ChainUpgradeWindow {
                chainId: 1,
                startBlock: 500,
                endBlock: 600,
            }],
            gwStartBlock: 10,
        };
        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &other_id,
            300,
        )
        .await
        .expect("different-id retry");
        tx.commit().await.expect("commit");

        let proposal_id: Vec<u8> = sqlx::query_scalar(
            "SELECT proposal_id FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("proposal id");
        assert_eq!(proposal_id, U256::from(2u64).to_be_bytes::<32>());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn upgrade_upsert_does_not_replace_active_proposal() {
        use alloy::primitives::U256;
        use sqlx::postgres::PgPoolOptions;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");

        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id, proposal_block, updated_at
            )
            VALUES ('GCS', 'UpgradeActivated', 'in_progress', $1, 'v1',
                    100, 200, 1, 1, 100, NOW())
            "#,
        )
        .bind(&U256::from(1u64).to_be_bytes::<32>()[..])
        .execute(&pool)
        .await
        .expect("seed");

        let event = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(2u64),
            softwareVersion: "v2".to_string(),
            chainUpgradeWindows: vec![
                ProtocolConfig::ChainUpgradeWindow {
                    chainId: 1,
                    startBlock: 300,
                    endBlock: 400,
                },
                ProtocolConfig::ChainUpgradeWindow {
                    chainId: 2,
                    startBlock: 500,
                    endBlock: 600,
                },
            ],
            gwStartBlock: 5,
        };
        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &event,
            200,
        )
        .await
        .expect("upsert");
        tx.commit().await.expect("commit");

        let proposal_id: Vec<u8> = sqlx::query_scalar(
            "SELECT proposal_id FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("proposal id");
        assert_eq!(proposal_id, U256::from(1u64).to_be_bytes::<32>());
        let row_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("row count");
        assert_eq!(
            row_count, 1,
            "a rejected proposal must not install only its previously absent chain"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn upgrade_upsert_rejects_older_replayed_proposal() {
        use alloy::primitives::U256;
        use sqlx::postgres::PgPoolOptions;
        use sqlx::Row;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");

        // A completed cycle for proposal 2, observed at block 100.
        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id, proposal_block, updated_at
            )
            VALUES ('GCS', 'LIVE', 'completed', $1, 'v2', 100, 200, 1, 1, 100, NOW())
            "#,
        )
        .bind(&U256::from(2u64).to_be_bytes::<32>()[..])
        .execute(&pool)
        .await
        .expect("seed");

        // Replay the older proposal 1 from an earlier block (50).
        let event = ProtocolConfig::CoprocessorUpgradeProposed {
            proposalId: U256::from(1u64),
            softwareVersion: "v1".to_string(),
            chainUpgradeWindows: vec![ProtocolConfig::ChainUpgradeWindow {
                chainId: 1,
                startBlock: 10,
                endBlock: 20,
            }],
            gwStartBlock: 1,
        };

        let mut tx = pool.begin().await.expect("tx");
        notify_coprocessor_upgrade_proposed(
            &mut tx,
            ChainId::try_from(1_u64).expect("chain id"),
            &event,
            50,
        )
        .await
        .expect("no-op ok");
        tx.commit().await.expect("commit");

        // Row unchanged: the older replay did not re-arm the completed cycle.
        let row =
            sqlx::query("SELECT proposal_id, state, status FROM upgrade_state WHERE stack_role = 'GCS'")
                .fetch_one(&pool)
                .await
                .expect("row");
        assert_eq!(
            row.try_get::<Vec<u8>, _>("proposal_id").unwrap(),
            U256::from(2u64).to_be_bytes::<32>().to_vec()
        );
        assert_eq!(row.try_get::<String, _>("state").unwrap(), "LIVE");
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "completed");
    }
}
