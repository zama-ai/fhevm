//! Resolve the executor rules in force at each log in a host block.
use std::{num::NonZeroUsize, sync::Mutex, time::Duration};

use alloy::{
    eips::BlockId,
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::{Log, TransactionRequest},
    sol_types::SolCall,
};
use anyhow::{Context, Result};
use lru::LruCache;

use crate::contracts::TfheContract::getVersionCall;
use crate::database::event_operands::{
    executor_upgrade_transitions_for_proxy, ExecutorVersion,
    ExecutorVersionSchedule,
};
use crate::database::ingest::BlockLogs;

pub(crate) const RPC_TIMEOUT: Duration = Duration::from_secs(30);

/// Owned version-resolution input so history recovery can resolve each
/// recovered block without borrowing its temporary log batch.
pub(crate) struct ExecutorVersionRequest {
    pub(crate) block_number: u64,
    pub(crate) block_hash: B256,
    pub(crate) parent_hash: B256,
    pub(crate) tfhe_proxy: Option<Address>,
    pub(crate) persisted_parent_version: Option<ExecutorVersion>,
    pub(crate) upgrades: Vec<(u64, Address)>,
}

impl ExecutorVersionRequest {
    pub(crate) fn from_block(
        block: &BlockLogs<Log>,
        tfhe_proxy: Option<Address>,
    ) -> Self {
        let upgrades = tfhe_proxy.map_or_else(Vec::new, |proxy| {
            executor_upgrade_transitions_for_proxy(
                block.logs.iter().map(|log| (log.log_index, &log.inner)),
                proxy,
            )
        });
        Self {
            block_number: block.summary.number,
            block_hash: block.summary.hash,
            parent_hash: block.summary.parent_hash,
            tfhe_proxy,
            persisted_parent_version: None,
            upgrades,
        }
    }
}

/// Each instance serves one chain. Cache only successful reads at an exact
/// address and block hash. A delivered block becomes a parent checkpoint only
/// when its ending version commits to the coprocessor database.
pub(crate) struct ExecutorVersionResolver {
    reads: Mutex<LruCache<(Address, B256), Option<ExecutorVersion>>>,
}

impl Default for ExecutorVersionResolver {
    fn default() -> Self {
        Self {
            reads: Mutex::new(LruCache::new(NonZeroUsize::new(128).unwrap())),
        }
    }
}

impl ExecutorVersionResolver {
    pub(crate) async fn resolve(
        &self,
        request: ExecutorVersionRequest,
        provider: Option<&dyn Provider>,
        timeout: Duration,
    ) -> Result<ExecutorVersionSchedule> {
        let Some(proxy) = request.tfhe_proxy else {
            return Ok(ExecutorVersionSchedule::default());
        };
        let initial = match request.persisted_parent_version {
            Some(version) => Some(version),
            None => {
                // Genesis has no parent state. Its header anchors any predeploys.
                let hash = if request.block_number == 0 {
                    request.block_hash
                } else {
                    request.parent_hash
                };
                self.lookup(provider, proxy, hash, timeout).await?
            }
        };
        let mut schedule = ExecutorVersionSchedule::new(initial);
        for (index, implementation) in request.upgrades {
            let version = self
                .lookup(provider, implementation, request.block_hash, timeout)
                .await?;
            schedule.add_transition(index, version);
        }
        Ok(schedule)
    }

    pub(crate) async fn lookup(
        &self,
        provider: Option<&dyn Provider>,
        address: Address,
        hash: B256,
        timeout: Duration,
    ) -> Result<Option<ExecutorVersion>> {
        let key = (address, hash);
        if let Some(version) = self.reads.lock().unwrap().get(&key).copied() {
            return Ok(version);
        }
        let provider = provider
            .context("Host RPC unavailable for executor version lookup")?;
        let request = TransactionRequest::default()
            .to(address)
            .input(getVersionCall {}.abi_encode().into());
        let value = tokio::time::timeout(
            timeout,
            provider.call(request).block(BlockId::hash_canonical(hash)),
        )
        .await
        .context("Executor version lookup timed out")?
        .with_context(|| {
            format!("Cannot read executor version for {address} at {hash}")
        })?;
        // An empty response before deployment and an unrecognized version
        // string follow the historical compatibility policy. RPC and ABI
        // decoding failures must leave the delivery available for retry.
        let version = if value.is_empty() {
            None
        } else {
            ExecutorVersion::parse(&getVersionCall::abi_decode_returns(&value)?)
        };
        self.reads.lock().unwrap().put(key, version);
        Ok(version)
    }
}
