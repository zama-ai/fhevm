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
use tracing::{error, info, warn};

use crate::cmd::block_history::{BlockHash, BlockSummary};
use crate::cmd::InfiniteLogIter;
use crate::contracts::{
    AclContract, BridgeContract, KMSGeneration, ProtocolConfig, TfheContract,
};
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    acl_result_handles, tfhe_result_handle, Chain, ChainHash, Database, LogTfhe,
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
                if let Err(err) = db
                    .cleanup_orphaned_branch_state(&mut tx, &orphaned_hashes)
                    .await
                {
                    error!(
                        block_number,
                        ?err,
                        "Failed to clean orphaned branch state during finalization"
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
    use alloy::primitives::FixedBytes;

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
