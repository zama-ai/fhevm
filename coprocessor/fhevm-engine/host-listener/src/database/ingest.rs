use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::RwLock;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy::sol_types::SolEventInterface;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::types::Handle;
use sqlx::types::time::{OffsetDateTime, PrimitiveDateTime};
use tracing::{error, info, warn};

use crate::cmd::block_history::{BlockHash, BlockSummary};
use crate::cmd::InfiniteLogIter;
use crate::contracts::{AclContract, KMSGeneration, TfheContract};
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    acl_result_handles, tfhe_result_handle, Chain, ChainHash, Database, LogTfhe,
};
use crate::kms_generation::insert_kms_generation_events_tx;
use crate::kms_generation::metrics::KMS_EVENT_DECODE_FAIL_COUNTER;

pub struct BlockLogs<T> {
    pub logs: Vec<T>,
    pub summary: BlockSummary,
    pub catchup: bool,
    pub finalized: bool,
}

/// Refuse to commit logs under a header from another fork, or logs whose
/// identity/order cannot be established. The caller must refetch the batch.
pub fn validate_block_logs(block: &BlockLogs<Log>) -> Result<(), sqlx::Error> {
    for log in &block.logs {
        if log.block_hash != Some(block.summary.hash)
            || log.block_number != Some(block.summary.number)
            || log.removed
            || log.transaction_hash.is_none()
            || log.log_index.is_none()
        {
            return Err(sqlx::Error::Protocol(format!(
                "Invalid log identity in block {} ({:?}); refetch the block",
                block.summary.number, block.summary.hash
            )));
        }
    }
    Ok(())
}

#[derive(Copy, Clone, Debug)]
pub struct IngestOptions {
    pub dependence_by_connexity: bool,
    pub dependence_cross_block: bool,
    pub dependent_ops_max_per_chain: u32,
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

pub async fn ingest_block_logs(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
    kms_generation_contract_address: &Option<Address>,
    options: IngestOptions,
) -> Result<(), sqlx::Error> {
    validate_block_logs(block_logs)?;
    let mut tx = db.new_transaction().await?;
    let mut acl_event_logs = vec![];
    let mut is_allowed = HashSet::<Handle>::new();
    let mut tfhe_event_log = vec![];
    let mut kms_gen_events = vec![];
    let block_hash = block_logs.summary.hash;
    let block_number = block_logs.summary.number;
    let mut catchup_insertion = 0;
    let block_timestamp = block_date_time_utc(block_logs.summary.timestamp);
    let mut at_least_one_insertion = false;

    for log in &block_logs.logs {
        let current_address = Some(log.inner.address);
        let is_acl_address = &current_address == acl_contract_address;
        if acl_contract_address.is_none() || is_acl_address {
            if let Ok(event) =
                AclContract::AclContractEvents::decode_log(&log.inner)
            {
                let handles = acl_result_handles(&event);
                for handle in handles {
                    is_allowed.insert(handle.to_vec());
                }
                acl_event_logs.push((event, log.transaction_hash));
                continue;
            }
        }

        let is_tfhe_address = &current_address == tfhe_contract_address;
        if tfhe_contract_address.is_none() || is_tfhe_address {
            if let Ok(event) =
                TfheContract::TfheContractEvents::decode_log(&log.inner)
            {
                let log = LogTfhe {
                    event,
                    transaction_hash: log.transaction_hash,
                    block_number,
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

        if is_acl_address || is_tfhe_address || is_kms_gen_address {
            error!(
                event_address = ?log.inner.address,
                acl_contract_address = ?acl_contract_address,
                tfhe_contract_address = ?tfhe_contract_address,
                kms_generation_contract_address = ?kms_generation_contract_address,
                log = ?log,
                "Cannot decode event",
            );
        }
    }
    // Read and publish ACL evidence under the same per-handle transaction locks
    // as computation writes. This also serializes split deliveries across blocks.
    let mut touched_handles = is_allowed.clone();
    for log in &tfhe_event_log {
        if let Some(handle) = tfhe_result_handle(&log.event) {
            touched_handles.insert(handle.to_vec());
        }
    }
    let touched_handles = touched_handles.into_iter().collect::<Vec<_>>();
    db.lock_ingestion_handles(&mut tx, &touched_handles).await?;
    for (event, transaction_hash) in acl_event_logs {
        let inserted = db
            .handle_acl_event(
                &mut tx,
                &event,
                &transaction_hash,
                chain_id,
                block_hash.as_ref(),
                block_number,
            )
            .await?;
        at_least_one_insertion |= inserted;
        if block_logs.catchup && inserted {
            info!(acl_event = ?event, ?transaction_hash, block_number, "ACL event missed before");
            catchup_insertion += 1;
        } else {
            info!(acl_event = ?event, ?transaction_hash, block_number, "ACL event");
        }
    }
    // An ACL may already be stored from an earlier delivery or another block,
    // even if this batch has no ACL event for the computation. Resolve that
    // evidence before grouping chains, which also uses is_allowed; checking
    // allowed_handles only in the computation INSERT would be too late.
    is_allowed.extend(
        db.allowed_computation_handles(&mut tx, &touched_handles)
            .await?,
    );

    for tfhe_log in tfhe_event_log.iter_mut() {
        tfhe_log.is_allowed =
            if let Some(result_handle) = tfhe_result_handle(&tfhe_log.event) {
                is_allowed.contains(&result_handle.to_vec())
            } else {
                false
            };
    }

    // Group on a private snapshot. Publishing before commit leaks mappings from
    // rolled-back batches into later or concurrently ingested blocks.
    let private_cache =
        Arc::new(RwLock::new(db.dependence_chain.read().await.clone()));
    let chains = dependence_chains(
        &mut tfhe_event_log,
        &private_cache,
        options.dependence_by_connexity,
        options.dependence_cross_block,
    )
    .await;

    let inferred_chains = tfhe_event_log
        .iter()
        .filter_map(|log| {
            tfhe_result_handle(&log.event)
                .map(|handle| (handle.to_vec(), log.dependence_chain))
        })
        .collect::<HashMap<_, _>>();
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

    let upgraded = db
        .upgrade_allowed_computations(&mut tx, &touched_handles)
        .await?;
    let upgraded_chains = upgraded
        .iter()
        .filter_map(|(_, chain)| chain.clone())
        .collect::<Vec<_>>();
    at_least_one_insertion |= !upgraded.is_empty();
    if slow_lane_enabled {
        for (handle, stored_chain) in &upgraded {
            let chain = inferred_chains.get(handle).copied().or_else(|| {
                stored_chain
                    .as_ref()
                    .and_then(|id| ChainHash::try_from(id.as_slice()).ok())
            });
            if let Some(chain) = chain {
                *dependent_ops_by_chain.entry(chain).or_default() += 1;
            }
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

        // ACL-only replay may have no newly inferred chain at all.
        slow_dep_chain_ids.extend(
            dependent_ops_by_chain
                .iter()
                .filter(|(_, count)| **count > max_per_chain)
                .map(|(chain, _)| *chain),
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

        // Preserve classification on the stored identity when replay grouped
        // an upgraded computation differently from its first ingestion.
        for (handle, stored_chain) in &upgraded {
            if inferred_chains
                .get(handle)
                .is_some_and(|chain| slow_dep_chain_ids.contains(chain))
            {
                if let Some(chain) = stored_chain
                    .as_ref()
                    .and_then(|id| ChainHash::try_from(id.as_slice()).ok())
                {
                    slow_dep_chain_ids.insert(chain);
                }
            }
        }
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
    db.mark_block_as_valid(&mut tx, &block_logs.summary, block_logs.finalized)
        .await?;
    // A replay's inferred grouping can differ from the persisted computation's
    // chain. Order the union before taking any chain row locks: two separately
    // sorted passes can still deadlock (inferred B -> stored A vs A -> B).
    // Write missing rows in this order too; SELECT FOR UPDATE alone misses them.
    let mut chain_updates = BTreeMap::new();
    if at_least_one_insertion {
        for chain in chains {
            chain_updates.insert(chain.hash.to_vec(), (Some(chain), None));
        }
    }
    for (id, schedule_order) in db
        .pending_computation_chains(&mut tx, &touched_handles)
        .await?
    {
        chain_updates.entry(id).or_default().1 = Some(schedule_order);
    }
    let upgraded_chains = upgraded_chains.into_iter().collect::<HashSet<_>>();
    let slow_chains = slow_dep_chain_ids
        .iter()
        .map(|id| id.to_vec())
        .collect::<HashSet<_>>();
    let mut repaired = 0;
    for (id, (inferred, pending)) in chain_updates {
        if let Some(chain) = inferred {
            db.update_dependence_chain(
                &mut tx,
                &chain,
                block_timestamp,
                &block_logs.summary,
                &slow_dep_chain_ids,
            )
            .await?;
            // This upsert already wakes/recreates the chain and preserves its
            // slow priority, so it also covers any pending repair for this ID.
        } else if let Some(schedule_order) = pending {
            repaired += usize::from(
                db.repair_pending_computation_chain(
                    &mut tx,
                    &id,
                    schedule_order,
                    upgraded_chains.contains(&id),
                    slow_chains.contains(&id),
                )
                .await?,
            );
        }
    }
    if repaired > 0 {
        warn!(
            count = repaired,
            "Pending computation dependence chains re-armed"
        );
    }
    let cache_updates = if options.dependence_cross_block {
        db.computation_chain_cache_entries(&mut tx, &touched_handles)
            .await?
    } else {
        vec![]
    };
    tx.commit().await?;
    if options.dependence_cross_block {
        // Merge only this batch's entries: replacing the entire snapshot could
        // erase another listener clone's successful publication.
        let mut shared_cache = db.dependence_chain.write().await;
        for (handle, chain) in cache_updates {
            if let (Ok(handle), Ok(chain)) = (
                alloy::primitives::FixedBytes::<32>::try_from(
                    handle.as_slice(),
                ),
                ChainHash::try_from(chain.as_slice()),
            ) {
                shared_cache.put(handle, chain);
            }
        }
    }
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
    let mut tx = match db.new_transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                ?err,
                "Failed to create transaction for finalized blocks update"
            );
            return;
        }
    };
    let last_finalized_block = last_block_number.saturating_sub(finality_lag);
    let blocks_number = match Database::get_finalized_blocks_number(
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
                last_finalized_block, "Failed to fetch finalized blocks number"
            );
            return;
        }
    };
    info!(?blocks_number, "Finalizing blocks");
    for block_number in blocks_number {
        let block_hash =
            match get_block_hash_by_number(block_number as u64).await {
                Ok(block_hash) => block_hash,
                Err(err) => {
                    error!(
                        block_number,
                        ?err,
                        "Failed to fetch block for finalization"
                    );
                    continue;
                }
            };
        if let Err(err) = db
            .update_block_as_finalized(&mut tx, block_number, &block_hash)
            .await
        {
            error!(block_number, ?err, "Failed to update block as finalized");
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
}
