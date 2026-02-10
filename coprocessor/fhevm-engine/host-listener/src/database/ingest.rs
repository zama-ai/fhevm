use std::collections::{HashMap, HashSet, VecDeque};

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy::sol_types::SolEventInterface;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::types::{Handle, SchedulePriority};
use sqlx::types::time::{OffsetDateTime, PrimitiveDateTime};
use tracing::{error, info};

use crate::cmd::block_history::BlockSummary;
use crate::contracts::{AclContract, TfheContract};
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    acl_result_handles, tfhe_inputs_handle, tfhe_result_handle, Chain,
    ChainHash, Database, LogTfhe,
};

pub struct BlockLogs<T> {
    pub logs: Vec<T>,
    pub summary: BlockSummary,
    pub catchup: bool,
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
        for dependency in &chain.dependencies {
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

pub async fn ingest_block_logs(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
    options: IngestOptions,
) -> Result<(), sqlx::Error> {
    let mut tx = db.new_transaction().await?;
    let mut is_allowed = HashSet::<Handle>::new();
    let mut tfhe_event_log = vec![];
    let block_hash = block_logs.summary.hash;
    let block_number = block_logs.summary.number;
    let mut catchup_insertion = 0;
    let block_timestamp = block_date_time_utc(block_logs.summary.timestamp);
    let mut at_least_one_insertion = false;

    for log in &block_logs.logs {
        let current_address = Some(log.inner.address);
        let is_acl_address = &current_address == acl_contract_address;
        let transaction_hash = log.transaction_hash;
        if acl_contract_address.is_none() || is_acl_address {
            if let Ok(event) =
                AclContract::AclContractEvents::decode_log(&log.inner)
            {
                let handles = acl_result_handles(&event);
                for handle in handles {
                    is_allowed.insert(handle.to_vec());
                }
                let inserted = db
                    .handle_acl_event(
                        &mut tx,
                        &event,
                        &log.transaction_hash,
                        chain_id,
                        block_hash.as_ref(),
                        block_number,
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
                };
                tfhe_event_log.push(log);
                continue;
            }
        }

        if is_acl_address || is_tfhe_address {
            error!(
                event_address = ?log.inner.address,
                acl_contract_address = ?acl_contract_address,
                tfhe_contract_address = ?tfhe_contract_address,
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
    let seen_dep_chain_ids = if slow_lane_enabled {
        Vec::new()
    } else {
        chains.iter().map(|chain| chain.hash.to_vec()).collect()
    };
    let mut dependent_ops_by_chain: HashMap<ChainHash, u64> = HashMap::new();
    for tfhe_log in tfhe_event_log {
        let inserted = db.insert_tfhe_event(&mut tx, &tfhe_log).await?;
        at_least_one_insertion |= inserted;
        // Count only newly inserted, currently allowed TFHE ops that actually
        // consume input handles. This approximates dependent work added to a
        // chain by this ingest pass.
        let has_dependencies = !tfhe_inputs_handle(&tfhe_log.event).is_empty();
        let is_new_allowed_event = inserted && tfhe_log.is_allowed;
        if slow_lane_enabled && is_new_allowed_event && has_dependencies {
            let total = dependent_ops_by_chain
                .entry(tfhe_log.dependence_chain)
                .or_default();
            *total = total.saturating_add(1);
        }
        if block_logs.catchup && inserted {
            info!(tfhe_log = ?tfhe_log, "TFHE event missed before");
            catchup_insertion += 1;
        } else {
            info!(tfhe_log = ?tfhe_log, "TFHE event");
        }
    }

    let mut slow_dep_chain_ids: HashSet<ChainHash> = HashSet::new();
    if slow_lane_enabled {
        let max_per_chain = u64::from(options.dependent_ops_max_per_chain);
        for chain in &chains {
            if let Some(chain_dep_ops) = dependent_ops_by_chain.get(&chain.hash)
            {
                if *chain_dep_ops > max_per_chain {
                    slow_dep_chain_ids.insert(chain.hash);
                }
            }
        }

        let parent_dep_chain_ids = chains
            .iter()
            .flat_map(|chain| {
                chain
                    .dependencies
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

    db.mark_block_as_valid(&mut tx, &block_logs.summary).await?;
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
    if !slow_lane_enabled {
        let promoted = db
            .promote_seen_dep_chains_to_fast_priority(
                &mut tx,
                &seen_dep_chain_ids,
            )
            .await?;
        if promoted > 0 {
            info!(
                count = promoted,
                "Slow-lane disabled: promoted seen chains to fast"
            );
        }
    }
    tx.commit().await
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
}
