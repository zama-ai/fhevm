use std::collections::{HashMap, HashSet};

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

fn classify_slow_lane_priorities(
    chains: &[Chain],
    dependent_ops_by_chain: &HashMap<ChainHash, u64>,
    dependent_ops_max_per_chain: u32,
) -> (HashMap<ChainHash, SchedulePriority>, u64, u64) {
    if dependent_ops_max_per_chain == 0 {
        return (HashMap::new(), 0, 0);
    }

    let mut schedule_priority_by_chain = HashMap::new();
    let mut allowed: u64 = 0;
    let mut throttled: u64 = 0;
    let max_per_chain = u64::from(dependent_ops_max_per_chain);

    for chain in chains {
        let Some(total) = dependent_ops_by_chain.get(&chain.hash) else {
            continue;
        };
        if *total > max_per_chain {
            throttled = throttled.saturating_add(*total);
            schedule_priority_by_chain
                .insert(chain.hash, SchedulePriority::SLOW);
        } else {
            allowed = allowed.saturating_add(*total);
        }
    }

    (schedule_priority_by_chain, allowed, throttled)
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
        if slow_lane_enabled
            && inserted
            && tfhe_log.is_allowed
            && !tfhe_inputs_handle(&tfhe_log.event).is_empty()
        {
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

    let mut schedule_priority_by_chain = HashMap::new();
    if slow_lane_enabled {
        let (schedule_priorities, allowed, throttled) =
            classify_slow_lane_priorities(
                &chains,
                &dependent_ops_by_chain,
                options.dependent_ops_max_per_chain,
            );
        schedule_priority_by_chain = schedule_priorities;
        db.record_dependent_ops_metrics(allowed, throttled);
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
            &schedule_priority_by_chain,
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
    use fhevm_engine_common::types::SchedulePriority;

    use super::classify_slow_lane_priorities;
    use crate::database::tfhe_event_propagate::{Chain, ChainHash};

    fn fixture_dep_chain(last_byte: u8) -> Chain {
        Chain {
            hash: ChainHash::with_last_byte(last_byte),
            dependencies: vec![],
            dependents: vec![],
            size: 0,
            before_size: 0,
            allowed_handle: vec![],
            new_chain: true,
        }
    }

    #[test]
    fn classify_slow_lane_priorities_cap_zero_disables() {
        let chains = vec![fixture_dep_chain(1)];
        let mut by_chain = std::collections::HashMap::new();
        by_chain.insert(chains[0].hash, 10_u64);

        let (priorities, allowed, throttled) =
            classify_slow_lane_priorities(&chains, &by_chain, 0);

        assert!(priorities.is_empty());
        assert_eq!(allowed, 0);
        assert_eq!(throttled, 0);
    }

    #[test]
    fn classify_slow_lane_priorities_marks_only_over_limit() {
        let chain_fast = fixture_dep_chain(1);
        let chain_slow = fixture_dep_chain(2);
        let chain_missing = fixture_dep_chain(3);
        let chains =
            vec![chain_fast.clone(), chain_slow.clone(), chain_missing];
        let mut by_chain = std::collections::HashMap::new();
        by_chain.insert(chain_fast.hash, 3_u64);
        by_chain.insert(chain_slow.hash, 5_u64);

        let (priorities, allowed, throttled) =
            classify_slow_lane_priorities(&chains, &by_chain, 4);

        assert_eq!(allowed, 3);
        assert_eq!(throttled, 5);
        assert_eq!(
            priorities.get(&chain_slow.hash),
            Some(&SchedulePriority::SLOW)
        );
        assert!(!priorities.contains_key(&chain_fast.hash));
        assert_eq!(priorities.len(), 1);
    }
}
