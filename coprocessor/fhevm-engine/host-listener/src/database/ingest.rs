use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy::sol_types::SolEventInterface;
use fhevm_engine_common::branch::{
    advance_settled_height, read_settled_height,
};
use fhevm_engine_common::bridge::chain_id_from_handle;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::types::{
    Handle, COMPUTED_HANDLE_INDEX_MARKER, HANDLE_VERSION,
};
use sqlx::types::time::{OffsetDateTime, PrimitiveDateTime};
use tracing::{error, info, warn};

use crate::cmd::block_history::{BlockHash, BlockSummary};
use crate::cmd::InfiniteLogIter;
use crate::contracts::{
    AclContract, BridgeContract, KMSGeneration, ProtocolConfig, TfheContract,
};
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    acl_result_handles, settlement_cutover_block, tfhe_result_handle, Chain,
    ChainHash, Database, LogTfhe,
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

fn settlement_candidate_block(
    last_block_number: u64,
    finality_lag: u64,
    settlement_finality_lag: u64,
) -> u64 {
    last_block_number.saturating_sub(finality_lag.max(settlement_finality_lag))
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
/// Same-block components can still be connected to prior-block components.
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

    for log in &block_logs.logs {
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
                    // The contract specifies that if multiple fallback events
                    // are emitted for the same handle, only the first one is
                    // the source of truth: skip duplicates within this block
                    // and grants from a different transaction. The SAME grant
                    // re-observed in another block context (fork sibling or
                    // canonical re-inclusion after a reorg) is synthesized
                    // again for its own context, so cleanup of one fork never
                    // erases the grant from the surviving fork. A handle
                    // materialized by a bridge association (ciphertext copy
                    // without a computations row) also stays write-once.
                    let first_in_block =
                        seen_fallback_handles.insert(dst_handle.to_vec());
                    if !first_in_block
                        || db
                            .fallback_grant_conflicts(
                                &mut tx,
                                dst_handle.as_slice(),
                                &log.transaction_hash,
                                block_hash.as_ref(),
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
                    });
                    at_least_one_insertion |= db
                        .insert_pbs_computations(
                            &mut tx,
                            &[dst_handle.to_vec()],
                            log.transaction_hash.map(|h| h.to_vec()),
                            block_number,
                            block_hash.as_ref(),
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

    let chains = dependence_chains(
        &mut tfhe_event_log,
        &db.dependence_chain,
        options.dependence_cross_block,
    )
    .await;

    let slow_lane_enabled = options.dependent_ops_max_per_chain > 0;
    let mut dependent_ops_by_chain: HashMap<ChainHash, u64> = HashMap::new();
    let insertions = db
        .insert_tfhe_events_batch(&mut tx, &tfhe_event_log)
        .await?;
    for (tfhe_log, inserted) in tfhe_event_log.into_iter().zip(insertions) {
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
    // block has been inserted into computations_branch. handle_acl_event
    // resolves each allowed handle's producer block by matching
    // computations_branch against the current-branch ancestry (which includes
    // this block); a handle produced *and* allowed within this same block only
    // has its producer row once the loop above has run. Resolving ACL events
    // earlier would miss the same-block producer and fall back to branchless,
    // spuriously incrementing host_listener_unresolved_producer_block_total.
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
                notify_coprocessor_upgrade_proposed(tx, chain_id, proposed).await?;
            }
            other => {
                warn!(
                    ?other,
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
) -> Result<(), sqlx::Error> {
    let listener_chain_id = chain_id.as_u64();

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

    let Some(window) = event
        .chainUpgradeWindows
        .iter()
        .find(|w| w.chainId == listener_chain_id)
    else {
        warn!(
            listener_chain_id,
            proposal_id = %proposal_id_hex,
            nb_windows = event.chainUpgradeWindows.len(),
            "CoprocessorUpgradeProposed does not include this chain — authority listener will not emit event_upgrade_activated"
        );
        return Ok(());
    };

    info!(
        proposal_id = %proposal_id_hex,
        software_version = %event.softwareVersion,
        chain_id = listener_chain_id,
        start_block = window.startBlock,
        end_block = window.endBlock,
        gw_start_block = event.gwStartBlock,
        "Decoded CoprocessorUpgradeProposed, emitting pg_notify('event_upgrade_activated')"
    );

    let payload = serde_json::json!({
        "proposal_id":        &proposal_id_hex,
        "chain_id":           listener_chain_id as i64,
        "start_block":        window.startBlock as i64,
        "end_block":          window.endBlock as i64,
        "gw_start_block":     event.gwStartBlock as i64,
        "version":            &event.softwareVersion,
    });

    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(UPGRADE_ACTIVATED_CHANNEL)
        .bind(payload.to_string())
        .execute(&mut **tx)
        .await
        .map(|_| ())
}

pub async fn update_finalized_blocks(
    db: &mut Database,
    log_iter: &mut InfiniteLogIter,
    last_block_number: u64,
    finality_lag: u64,
    settlement_finality_lag: u64,
) {
    let log_iter = &*log_iter;
    update_finalized_blocks_aux(
        db,
        last_block_number,
        finality_lag,
        settlement_finality_lag,
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
    settlement_finality_lag: u64,
    mut get_block_hash_by_number: GetBlockHash,
) where
    GetBlockHash: FnMut(u64) -> GetBlockHashFuture,
    GetBlockHashFuture: Future<Output = anyhow::Result<BlockHash>>,
{
    info!(
        last_block_number,
        finality_lag,
        settlement_finality_lag,
        effective_settlement_lag = finality_lag.max(settlement_finality_lag),
        "Updating finalized blocks"
    );
    let last_finalized_block = last_block_number.saturating_sub(finality_lag);
    let settlement_candidate_height = settlement_candidate_block(
        last_block_number,
        finality_lag,
        settlement_finality_lag,
    );

    // Read the candidate numbers in a short transaction, then resolve the
    // canonical hashes over RPC with NO transaction open: block fetches can
    // take seconds each, and holding the finalization transaction across the
    // round-trips kept its row locks pinned for the whole time.
    let (pending_blocks, revalidation_blocks, current_settled_height) = {
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
        let current_settled_height =
            match read_settled_height(&mut tx, db.chain_id.as_i64()).await {
                Ok(height) => height,
                Err(err) => {
                    error!(?err, "Failed to read settlement frontier");
                    return;
                }
            };
        let first_post_cutover_block = settlement_cutover_block().max(0);
        let first_unsettled_block = current_settled_height
            .saturating_add(1)
            .max(first_post_cutover_block);
        let pending_blocks = match Database::get_pending_blocks_to_finalize(
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
                    "Failed to fetch pending blocks for finalization"
                );
                return;
            }
        };
        let revalidation_blocks =
            match Database::get_finalized_blocks_to_revalidate(
                &mut tx,
                db.chain_id,
                first_unsettled_block,
                settlement_candidate_height as i64,
            )
            .await
            {
                Ok(numbers) => numbers,
                Err(err) => {
                    error!(
                        ?err,
                        settlement_candidate_height,
                        "Failed to fetch finalized blocks for canonical revalidation"
                    );
                    return;
                }
            };
        (pending_blocks, revalidation_blocks, current_settled_height)
    };
    info!(?pending_blocks, ?revalidation_blocks, "Finalizing blocks");

    // Ascending: finalization verifies each block's parent linkage against
    // its finalized predecessor, so within one batch the predecessor must be
    // finalized first.
    let first_unsettled_block = current_settled_height
        .saturating_add(1)
        .max(settlement_cutover_block().max(0));
    // Catch-up persists blocks containing relevant logs, not every empty host
    // block, so the valid-block table can be sparse after a restart. Validate
    // every stored checkpoint before settlement crosses it, using the 11th
    // selected row as lookahead for a bounded batch of 10. Gaps with no stored
    // row are not themselves work and do not freeze settlement.
    const FINALIZATION_BATCH_SIZE: usize = 10;
    let settlement_cap_for_batch = |blocks: &mut Vec<i64>| {
        blocks.sort_unstable();
        let first_deferred_block = blocks.get(FINALIZATION_BATCH_SIZE).copied();
        let cap = if first_deferred_block.is_some_and(|block_number| {
            block_number <= settlement_candidate_height as i64
        }) {
            blocks[..FINALIZATION_BATCH_SIZE]
                .iter()
                .rev()
                .copied()
                .find(|block_number| {
                    *block_number >= first_unsettled_block
                        && *block_number <= settlement_candidate_height as i64
                })
                .unwrap_or(current_settled_height)
        } else {
            settlement_candidate_height as i64
        };
        blocks.truncate(FINALIZATION_BATCH_SIZE);
        cap
    };
    let mut pending_blocks: Vec<i64> = pending_blocks.into_iter().collect();
    let mut revalidation_blocks: Vec<i64> =
        revalidation_blocks.into_iter().collect();
    let pending_cap = settlement_cap_for_batch(&mut pending_blocks);
    let revalidation_cap = settlement_cap_for_batch(&mut revalidation_blocks);
    let mut verified_settlement_candidate = pending_cap.min(revalidation_cap);

    let mut fetch_heights: Vec<i64> = pending_blocks
        .iter()
        .copied()
        .chain(revalidation_blocks.iter().copied())
        .collect();
    fetch_heights.sort_unstable();
    fetch_heights.dedup();

    let mut canonical =
        std::collections::HashMap::with_capacity(fetch_heights.len());
    for block_number in fetch_heights {
        match get_block_hash_by_number(block_number as u64).await {
            Ok(block_hash) => {
                canonical.insert(block_number, block_hash);
            }
            Err(err) => {
                // The stop skips every batched block from here up, so nothing
                // above this height was revalidated this pass. Clamping below
                // the frontier just means no advance this pass.
                verified_settlement_candidate = verified_settlement_candidate
                    .min(block_number.saturating_sub(1));
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

    // Advance settlement even when no blocks need finalization: asynchronous
    // cleanup or publication work may have unblocked an existing finalized row.
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
    // The revalidation and pending queues finalize independently, each in
    // ascending order, so a refusal in the revalidation window (a stale row
    // whose canonical sibling awaits re-ingestion) cannot starve newer
    // pending blocks — and vice versa. Cross-queue linkage safety needs no
    // shared stop: a block adjacent to a refused height still checks its own
    // parent linkage against the stale finalized row and refuses on
    // contradiction.
    'queue: for queue in [&revalidation_blocks, &pending_blocks] {
        for &block_number in queue.iter() {
            let Some(block_hash) = canonical.get(&block_number) else {
                // The hash fetch stopped below this height; already clamped.
                continue 'queue;
            };
            match db
                .update_block_as_finalized(&mut tx, block_number, block_hash)
                .await
            {
                Ok(Some(orphaned_hashes)) => {
                    if let Err(err) = db
                        .enqueue_orphaned_branch_cleanup(
                            &mut tx,
                            block_number,
                            block_hash.as_ref(),
                            &orphaned_hashes,
                        )
                        .await
                    {
                        error!(
                            block_number,
                            ?err,
                            "Failed to enqueue orphaned branch cleanup during finalization"
                        );
                        return;
                    }
                }
                Ok(None) => {
                    // The stop skips every queued block from here up, so
                    // nothing above this height was revalidated this pass.
                    // Clamping below the frontier just means no advance this
                    // pass.
                    verified_settlement_candidate =
                        verified_settlement_candidate
                            .min(block_number.saturating_sub(1));
                    // Finalization refused (missing row / orphaned / parent
                    // linkage contradiction). STOP this queue: the next
                    // height's linkage check would pass vacuously without a
                    // finalized predecessor, letting a stale or poisoned RPC
                    // finalize a fork block right behind the refusal. Earlier
                    // blocks of this queue stay finalized; the rest retries
                    // next pass.
                    warn!(
                        block_number,
                        "Stopping finalization queue at refused block"
                    );
                    continue 'queue;
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
    }
    // Settlement is monotonic, so never let it cross a stored post-cutover
    // checkpoint which this pass did not revalidate against the listener's
    // current chain view.
    match advance_settled_height(
        &mut tx,
        db.chain_id.as_i64(),
        verified_settlement_candidate,
        settlement_cutover_block(),
    )
    .await
    {
        Ok(settled_height) => {
            info!(
                settled_height,
                "Updated coprocessor branch settlement frontier"
            );
        }
        Err(err) => {
            error!(
                ?err,
                settlement_candidate_height,
                "Failed to update coprocessor branch settlement frontier"
            );
            return;
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
    use alloy::primitives::FixedBytes;
    use fhevm_engine_common::branch::read_settled_height;
    use test_harness::instance::ImportMode;

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
            dependents: vec![],
            allowed_handle: vec![],
            size: 1,
            before_size: 0,
            new_chain: true,
        }
    }

    #[test]
    fn settlement_candidate_uses_deeper_lag_than_indexing_finality() {
        assert_eq!(settlement_candidate_block(100, 15, 50), 50);
    }

    #[test]
    fn settlement_candidate_never_advances_ahead_of_indexing_finality() {
        assert_eq!(settlement_candidate_block(100, 50, 15), 50);
    }

    struct EnvGuard {
        key: &'static str,
        value: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self {
                key,
                value: previous,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.value {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    async fn finalized_blocks_can_advance_ahead_of_settlement_lag() {
        let _cutover = EnvGuard::set("FHEVM_BRANCH_CUTOVER_BLOCK", "1");
        let db_instance =
            test_harness::instance::setup_test_db(ImportMode::None)
                .await
                .expect("valid db instance");
        let chain_id = ChainId::try_from(42_u64).unwrap();
        let mut db = Database::new(&db_instance.db_url, chain_id, 128)
            .await
            .expect("database");
        let pool = db.pool.read().await.clone();
        sqlx::query("DELETE FROM coprocessor_settlement WHERE chain_id = $1")
            .bind(chain_id.as_i64())
            .execute(&pool)
            .await
            .expect("clear settlement row");
        sqlx::query("DELETE FROM host_chain_blocks_valid WHERE chain_id = $1")
            .bind(chain_id.as_i64())
            .execute(&pool)
            .await
            .expect("clear block rows");
        sqlx::query("DELETE FROM computations_branch WHERE host_chain_id = $1")
            .bind(chain_id.as_i64())
            .execute(&pool)
            .await
            .expect("clear branch computation rows");
        sqlx::query(
            "DELETE FROM pbs_computations_branch WHERE host_chain_id = $1",
        )
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await
        .expect("clear branch pbs rows");

        for block_number in 0_i64..=10 {
            sqlx::query(
                "INSERT INTO host_chain_blocks_valid \
                 (chain_id, block_hash, parent_hash, block_number, block_status) \
                 VALUES ($1, $2, $3, $4, 'pending')",
            )
            .bind(chain_id.as_i64())
            .bind(vec![block_number as u8; 32])
            .bind(if block_number == 0 {
                Vec::new()
            } else {
                vec![(block_number - 1) as u8; 32]
            })
            .bind(block_number)
            .execute(&pool)
            .await
            .expect("insert pending block");
        }

        update_finalized_blocks_aux(
            &mut db,
            10,
            1,
            4,
            |block_number| async move {
                Ok(BlockHash::from([block_number as u8; 32]))
            },
        )
        .await;

        let mut tx = db
            .new_transaction()
            .await
            .expect("settlement tx")
            .expect("new_transaction() returns Some on a live stack");
        let settled_height = read_settled_height(&mut tx, chain_id.as_i64())
            .await
            .expect("read settlement");
        tx.rollback().await.expect("rollback settlement tx");

        let block_statuses = sqlx::query_as::<_, (i64, String)>(
            "SELECT block_number, block_status::text FROM host_chain_blocks_valid \
             WHERE chain_id = $1 ORDER BY block_number",
        )
        .bind(chain_id.as_i64())
        .fetch_all(&pool)
        .await
        .expect("query block statuses");

        assert_eq!(block_statuses.len(), 11);
        for block_number in 0_i64..=9 {
            assert!(
                block_statuses
                    .contains(&(block_number, "finalized".to_string())),
                "block {block_number} should be finalized by indexing finality"
            );
        }
        assert!(
            block_statuses.contains(&(10, "pending".to_string())),
            "block 10 should remain pending"
        );

        assert_eq!(
            settled_height, 6,
            "settlement should use the stricter settlement_finality_lag"
        );
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    async fn settlement_waits_for_reingestion_after_finalized_branch_changes() {
        let _cutover = EnvGuard::set("FHEVM_BRANCH_CUTOVER_BLOCK", "1");
        let db_instance =
            test_harness::instance::setup_test_db(ImportMode::None)
                .await
                .expect("valid db instance");
        let chain_id = ChainId::try_from(44_u64).unwrap();
        let mut db = Database::new(&db_instance.db_url, chain_id, 128)
            .await
            .expect("database");
        let pool = db.pool.read().await.clone();

        sqlx::query(
            "INSERT INTO coprocessor_settlement(chain_id, settled_height) \
             VALUES ($1, 1)",
        )
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await
        .expect("insert settlement frontier");

        let mut parent_hash = Vec::new();
        for block_number in 0_i64..=6 {
            let block_hash = if block_number < 2 {
                vec![block_number as u8; 32]
            } else {
                vec![0x40 + block_number as u8; 32]
            };
            sqlx::query(
                "INSERT INTO host_chain_blocks_valid \
                 (chain_id, block_hash, parent_hash, block_number, block_status) \
                 VALUES ($1, $2, $3, $4, 'finalized')",
            )
            .bind(chain_id.as_i64())
            .bind(&block_hash)
            .bind(&parent_hash)
            .bind(block_number)
            .execute(&pool)
            .await
            .expect("insert old finalized branch");
            parent_hash = block_hash;
        }

        update_finalized_blocks_aux(
            &mut db,
            10,
            1,
            4,
            |block_number| async move {
                // The listener now observes a different branch from height 2,
                // but its canonical rows have not been replayed into the DB.
                Ok(BlockHash::from([0x80 + block_number as u8; 32]))
            },
        )
        .await;

        let mut tx = db
            .new_transaction()
            .await
            .expect("settlement tx")
            .expect("new_transaction() returns Some on a live stack");
        let settled_height = read_settled_height(&mut tx, chain_id.as_i64())
            .await
            .expect("read settlement");
        tx.rollback().await.expect("rollback settlement tx");

        assert_eq!(
            settled_height, 1,
            "settlement must not trust finalized hashes from the listener's old branch"
        );

        let mut canonical_parent_hash = vec![1_u8; 32];
        for block_number in 2_i64..=6 {
            let canonical_hash = vec![0x80 + block_number as u8; 32];
            sqlx::query(
                "INSERT INTO host_chain_blocks_valid \
                 (chain_id, block_hash, parent_hash, block_number, block_status) \
                 VALUES ($1, $2, $3, $4, 'pending')",
            )
            .bind(chain_id.as_i64())
            .bind(&canonical_hash)
            .bind(&canonical_parent_hash)
            .bind(block_number)
            .execute(&pool)
            .await
            .expect("replay canonical block");
            canonical_parent_hash = canonical_hash;
        }

        update_finalized_blocks_aux(
            &mut db,
            10,
            1,
            4,
            |block_number| async move {
                Ok(BlockHash::from([0x80 + block_number as u8; 32]))
            },
        )
        .await;
        assert_eq!(
            db.process_orphaned_branch_cleanup_jobs()
                .await
                .expect("clean old branch"),
            1
        );

        // Cleanup completion unblocks the same shared finalization path. Its
        // bounded revalidation is idempotent and may now advance settlement.
        update_finalized_blocks_aux(
            &mut db,
            10,
            1,
            4,
            |block_number| async move {
                Ok(BlockHash::from([0x80 + block_number as u8; 32]))
            },
        )
        .await;
        let mut tx = db
            .new_transaction()
            .await
            .expect("settlement tx")
            .expect("new_transaction() returns Some on a live stack");
        let settled_height = read_settled_height(&mut tx, chain_id.as_i64())
            .await
            .expect("read recovered settlement");
        tx.rollback().await.expect("rollback settlement tx");
        assert_eq!(settled_height, 6);
    }

    /// A fork-recovered listener can hold a stale finalized row whose
    /// canonical sibling was never ingested (the canonical block is empty and
    /// below every replay window until the next restart). Its refusal must
    /// stall only the revalidation queue: newer pending blocks keep
    /// finalizing, and settlement holds below the stale row until
    /// re-ingestion resolves it.
    #[tokio::test]
    #[serial_test::serial(db)]
    async fn stale_finalized_row_does_not_starve_pending_finalization() {
        let _cutover = EnvGuard::set("FHEVM_BRANCH_CUTOVER_BLOCK", "1");
        let db_instance =
            test_harness::instance::setup_test_db(ImportMode::None)
                .await
                .expect("valid db instance");
        let chain_id = ChainId::try_from(46_u64).unwrap();
        let mut db = Database::new(&db_instance.db_url, chain_id, 128)
            .await
            .expect("database");
        let pool = db.pool.read().await.clone();

        sqlx::query(
            "INSERT INTO coprocessor_settlement(chain_id, settled_height) \
             VALUES ($1, 2)",
        )
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await
        .expect("insert settlement frontier");

        let seed_block = |number: i64,
                          hash: Vec<u8>,
                          parent: Vec<u8>,
                          status: &'static str| {
            let pool = pool.clone();
            let chain_id = chain_id.as_i64();
            async move {
                sqlx::query(
                    "INSERT INTO host_chain_blocks_valid \
                     (chain_id, block_hash, parent_hash, block_number, block_status) \
                     VALUES ($1, $2, $3, $4, $5)",
                )
                .bind(chain_id)
                .bind(hash)
                .bind(parent)
                .bind(number)
                .bind(status)
                .execute(&pool)
                .await
                .expect("seed block row");
            }
        };

        // Shared canonical history up to the settlement frontier.
        seed_block(1, vec![1_u8; 32], vec![0_u8; 32], "finalized").await;
        seed_block(2, vec![2_u8; 32], vec![1_u8; 32], "finalized").await;
        // Stale fork row finalized on the old branch; its canonical sibling
        // (0x83) was never stored.
        seed_block(3, vec![0x43_u8; 32], vec![2_u8; 32], "finalized").await;
        // Newer live pending blocks above a sparse gap at height 4.
        seed_block(5, vec![0x85_u8; 32], vec![0x84_u8; 32], "pending").await;
        seed_block(6, vec![0x86_u8; 32], vec![0x85_u8; 32], "pending").await;

        let canonical_hash = |block_number: u64| {
            if block_number < 3 {
                BlockHash::from([block_number as u8; 32])
            } else {
                BlockHash::from([0x80 + block_number as u8; 32])
            }
        };

        update_finalized_blocks_aux(&mut db, 10, 1, 4, |block_number| {
            let hash = canonical_hash(block_number);
            async move { Ok(hash) }
        })
        .await;

        let status_of = |hash: Vec<u8>| {
            let pool = pool.clone();
            let chain_id = chain_id.as_i64();
            async move {
                sqlx::query_scalar::<_, String>(
                    "SELECT block_status FROM host_chain_blocks_valid \
                     WHERE chain_id = $1 AND block_hash = $2",
                )
                .bind(chain_id)
                .bind(hash)
                .fetch_one(&pool)
                .await
                .expect("block status")
            }
        };

        assert_eq!(
            status_of(vec![0x85_u8; 32]).await,
            "finalized",
            "pending blocks above the stale row must not be starved"
        );
        assert_eq!(status_of(vec![0x86_u8; 32]).await, "finalized");
        assert_eq!(
            status_of(vec![0x43_u8; 32]).await,
            "finalized",
            "the stale row stays untouched until its canonical sibling is re-ingested"
        );

        let mut tx = db
            .new_transaction()
            .await
            .expect("settlement tx")
            .expect("new_transaction() returns Some on a live stack");
        let settled_height = read_settled_height(&mut tx, chain_id.as_i64())
            .await
            .expect("read settlement");
        tx.rollback().await.expect("rollback settlement tx");
        assert_eq!(
            settled_height, 2,
            "settlement must hold below the contradicted height"
        );

        // Re-ingestion (poller replay from the settlement frontier) stores
        // the canonical sibling; the next pass resolves the fork.
        seed_block(3, vec![0x83_u8; 32], vec![2_u8; 32], "pending").await;
        update_finalized_blocks_aux(&mut db, 10, 1, 4, |block_number| {
            let hash = canonical_hash(block_number);
            async move { Ok(hash) }
        })
        .await;
        assert_eq!(status_of(vec![0x83_u8; 32]).await, "finalized");
        assert_eq!(
            status_of(vec![0x43_u8; 32]).await,
            "orphaned",
            "finalizing the canonical sibling orphans the stale fork row"
        );

        assert_eq!(
            db.process_orphaned_branch_cleanup_jobs()
                .await
                .expect("clean stale branch"),
            1
        );
        update_finalized_blocks_aux(&mut db, 10, 1, 4, |block_number| {
            let hash = canonical_hash(block_number);
            async move { Ok(hash) }
        })
        .await;
        let mut tx = db
            .new_transaction()
            .await
            .expect("settlement tx")
            .expect("new_transaction() returns Some on a live stack");
        let settled_height = read_settled_height(&mut tx, chain_id.as_i64())
            .await
            .expect("read recovered settlement");
        tx.rollback().await.expect("rollback settlement tx");
        assert_eq!(
            settled_height, 6,
            "settlement recovers to the candidate once the fork is resolved"
        );
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    async fn finalization_enqueues_orphan_cleanup_asynchronously() {
        let _cutover = EnvGuard::set("FHEVM_BRANCH_CUTOVER_BLOCK", "1");
        let db_instance =
            test_harness::instance::setup_test_db(ImportMode::None)
                .await
                .expect("valid db instance");
        let chain_id = ChainId::try_from(43_u64).unwrap();
        let mut db = Database::new(&db_instance.db_url, chain_id, 128)
            .await
            .expect("database");
        let pool = db.pool.read().await.clone();

        let canonical_hash = vec![0x02_u8; 32];
        let orphan_hash = vec![0x03_u8; 32];
        let orphan_handle = vec![0xA3_u8; 32];

        for (block_number, block_hash, parent_hash) in [
            (0_i64, vec![0x00_u8; 32], Vec::new()),
            (1_i64, vec![0x01_u8; 32], vec![0x00_u8; 32]),
            (2_i64, canonical_hash.clone(), vec![0x01_u8; 32]),
            (2_i64, orphan_hash.clone(), vec![0x01_u8; 32]),
        ] {
            sqlx::query(
                "INSERT INTO host_chain_blocks_valid \
                 (chain_id, block_hash, parent_hash, block_number, block_status) \
                 VALUES ($1, $2, $3, $4, 'pending')",
            )
            .bind(chain_id.as_i64())
            .bind(block_hash)
            .bind(parent_hash)
            .bind(block_number)
            .execute(&pool)
            .await
            .expect("insert pending block");
        }

        sqlx::query(
            "INSERT INTO computations_branch (
                output_handle,
                dependencies,
                fhe_operation,
                is_scalar,
                transaction_id,
                host_chain_id,
                block_number,
                producer_block_hash
             )
             VALUES ($1, $2, 1, FALSE, $3, $4, 2, $5)",
        )
        .bind(&orphan_handle)
        .bind(vec![vec![0x55_u8; 32]])
        .bind(vec![0x77_u8; 32])
        .bind(chain_id.as_i64())
        .bind(&orphan_hash)
        .execute(&pool)
        .await
        .expect("insert orphan branch computation");

        update_finalized_blocks_aux(
            &mut db,
            3,
            1,
            1,
            |block_number| async move {
                let byte = block_number as u8;
                Ok(BlockHash::from([byte; 32]))
            },
        )
        .await;

        let orphan_status: String = sqlx::query_scalar(
            "SELECT block_status::text
             FROM host_chain_blocks_valid
             WHERE chain_id = $1 AND block_hash = $2",
        )
        .bind(chain_id.as_i64())
        .bind(&orphan_hash)
        .fetch_one(&pool)
        .await
        .expect("orphan status");
        assert_eq!(orphan_status, "orphaned");

        let pending_cleanup: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM branch_cleanup_jobs
             WHERE chain_id = $1
               AND finalized_block_hash = $2
               AND status = 'pending'",
        )
        .bind(chain_id.as_i64())
        .bind(&canonical_hash)
        .fetch_one(&pool)
        .await
        .expect("pending cleanup job");
        assert_eq!(pending_cleanup, 1);

        let orphan_computations: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM computations_branch
             WHERE host_chain_id = $1
               AND producer_block_hash = $2",
        )
        .bind(chain_id.as_i64())
        .bind(&orphan_hash)
        .fetch_one(&pool)
        .await
        .expect("orphan branch computation still present");
        assert_eq!(
            orphan_computations, 1,
            "finalization should not run heavy cleanup inline"
        );

        let mut tx = db
            .new_transaction()
            .await
            .expect("settlement tx")
            .expect("new_transaction() returns Some on a live stack");
        let settled_height = read_settled_height(&mut tx, chain_id.as_i64())
            .await
            .expect("read settlement");
        tx.rollback().await.expect("rollback settlement tx");
        assert_eq!(
            settled_height, 1,
            "pending cleanup for block 2 should block settlement"
        );

        let processed = db
            .process_orphaned_branch_cleanup_jobs()
            .await
            .expect("process cleanup jobs");
        assert_eq!(processed, 1);

        let orphan_computations: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM computations_branch
             WHERE host_chain_id = $1
               AND producer_block_hash = $2",
        )
        .bind(chain_id.as_i64())
        .bind(&orphan_hash)
        .fetch_one(&pool)
        .await
        .expect("orphan branch computation cleaned");
        assert_eq!(orphan_computations, 0);

        let completed_cleanup: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM branch_cleanup_jobs
             WHERE chain_id = $1
               AND finalized_block_hash = $2
               AND status = 'completed'",
        )
        .bind(chain_id.as_i64())
        .bind(&canonical_hash)
        .fetch_one(&pool)
        .await
        .expect("completed cleanup job");
        assert_eq!(completed_cleanup, 1);

        update_finalized_blocks_aux(
            &mut db,
            3,
            1,
            1,
            |block_number| async move {
                let byte = block_number as u8;
                Ok(BlockHash::from([byte; 32]))
            },
        )
        .await;

        let mut tx = db
            .new_transaction()
            .await
            .expect("settlement tx")
            .expect("new_transaction() returns Some on a live stack");
        let settled_height = read_settled_height(&mut tx, chain_id.as_i64())
            .await
            .expect("read settlement");
        tx.rollback().await.expect("rollback settlement tx");
        assert_eq!(settled_height, 2);
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
}
