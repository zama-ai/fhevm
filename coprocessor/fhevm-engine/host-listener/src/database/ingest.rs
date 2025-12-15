use std::collections::HashSet;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy::sol_types::SolEventInterface;
use fhevm_engine_common::types::Handle;
use sqlx::types::time::OffsetDateTime;
use tracing::{error, info};

use crate::cmd::block_history::BlockSummary;
use crate::contracts::{AclContract, TfheContract};
use crate::database::tfhe_event_propagate::{
    acl_result_handles, tfhe_result_handle, Database, LogTfhe,
};

pub struct BlockLogs<T> {
    pub logs: Vec<T>,
    pub summary: BlockSummary,
    pub catchup: bool,
}

/// Converts a block timestamp to a UTC `OffsetDateTime`.
///
/// # Parameters
/// - `timestamp`: Seconds since Unix epoch.
///
/// # Returns
/// A UTC `OffsetDateTime` suitable for database writes.
fn block_date_time_utc(timestamp: u64) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(timestamp as i64).unwrap_or_else(|_| {
        error!(timestamp, "Invalid block timestamp, using now",);
        OffsetDateTime::now_utc()
    })
}

pub async fn ingest_block_logs(
    chain_id: u64,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
) -> Result<(), sqlx::Error> {
    let mut tx = db.new_transaction().await?;
    let mut is_allowed = HashSet::<Handle>::new();
    let mut tfhe_event_log = vec![];
    let block_hash = block_logs.summary.hash;
    let block_number = block_logs.summary.number;
    let mut catchup_insertion = 0;
    let block_timestamp = block_date_time_utc(block_logs.summary.timestamp);

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
                    is_allowed: false, // updated in the next loop
                    block_number,
                    block_timestamp,
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

    for tfhe_log in tfhe_event_log {
        let is_allowed =
            if let Some(result_handle) = tfhe_result_handle(&tfhe_log.event) {
                is_allowed.contains(&result_handle.to_vec())
            } else {
                false
            };
        let tfhe_log = LogTfhe {
            is_allowed,
            ..tfhe_log
        };
        let inserted = db.insert_tfhe_event(&mut tx, &tfhe_log).await?;
        if block_logs.catchup && inserted {
            info!(tfhe_log = ?tfhe_log, "TFHE event missed before");
            catchup_insertion += 1;
        } else {
            info!(tfhe_log = ?tfhe_log, "TFHE event");
        }
    }

    if catchup_insertion == block_logs.logs.len() {
        info!(block_number, catchup_insertion, "Catchup inserted a block");
    } else if catchup_insertion > 0 {
        info!(block_number, catchup_insertion, "Catchup inserted events");
    }

    db.mark_block_as_valid(&mut tx, &block_logs.summary).await?;
    tx.commit().await
}
