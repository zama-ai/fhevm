//! Resolve the executor rules in force at each log in a host block.
use std::{
    num::NonZeroUsize,
    sync::{LazyLock, Mutex},
    time::Duration,
};

use alloy::{
    eips::BlockId,
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::{Log, TransactionRequest},
    sol_types::SolCall,
};
use anyhow::{Context, Result};
use lru::LruCache;
use prometheus::IntCounter;
use tracing::warn;

use crate::contracts::TfheContract::getVersionCall;
use crate::database::event_operands::{
    executor_upgrade_transitions_for_proxy, requires_executor_version,
    ExecutorVersion, ExecutorVersionSchedule,
};
use crate::database::ingest::BlockLogs;

pub(crate) const RPC_TIMEOUT: Duration = Duration::from_secs(30);

static VERSION_LOOKUP_FAILURES: LazyLock<IntCounter> = LazyLock::new(|| {
    prometheus::register_int_counter!(
        "host_listener_executor_version_lookup_failures_total",
        "Failed host executor version lookups"
    )
    .expect("executor version lookup metric must register")
});

/// Owned version-resolution input so history recovery can resolve each
/// recovered block without borrowing its temporary log batch.
pub(crate) struct ExecutorVersionRequest {
    pub(crate) block_number: u64,
    pub(crate) block_hash: B256,
    pub(crate) parent_hash: B256,
    pub(crate) tfhe_proxy: Option<Address>,
    pub(crate) persisted_parent_version: Option<ExecutorVersion>,
    upgrades: Vec<(u64, Address)>,
    operation_indices: Vec<u64>,
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
        let mut operation_indices: Vec<_> = block
            .logs
            .iter()
            .filter(|log| {
                Some(log.inner.address) == tfhe_proxy
                    && requires_executor_version(&log.inner)
            })
            .filter_map(|log| log.log_index)
            .collect();
        operation_indices.sort_unstable();
        Self {
            block_number: block.summary.number,
            block_hash: block.summary.hash,
            parent_hash: block.summary.parent_hash,
            tfhe_proxy,
            persisted_parent_version: None,
            upgrades,
            operation_indices,
        }
    }
}

/// Each instance serves one chain. Cache only successful reads at an exact
/// address and block hash. A delivered block becomes a parent checkpoint only
/// when its ending version commits to the coprocessor database.
pub(crate) struct ExecutorVersionResolver {
    reads: Mutex<LruCache<(Address, B256), ExecutorVersion>>,
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
        let mut upgrades = request.upgrades.into_iter().peekable();
        let operation_in_segment = |start: Option<u64>, end: Option<u64>| {
            let first = start.map_or(0, |index| {
                request.operation_indices.partition_point(|op| *op <= index)
            });
            request
                .operation_indices
                .get(first)
                .is_some_and(|op| end.is_none_or(|index| *op <= index))
        };
        let mut initial = request.persisted_parent_version;
        if initial.is_none()
            && operation_in_segment(
                None,
                upgrades.peek().map(|(index, _)| *index),
            )
        {
            // Genesis has no parent state. Its header anchors any predeploys.
            let hash = if request.block_number == 0 {
                request.block_hash
            } else {
                request.parent_hash
            };
            initial = Some(self.lookup(provider, proxy, hash, timeout).await?);
        }
        let mut schedule = ExecutorVersionSchedule::new(initial);
        while let Some((index, implementation)) = upgrades.next() {
            let next = upgrades.peek().map(|(index, _)| *index);
            let required = operation_in_segment(Some(index), next);
            // Checkpoint the last upgrade when possible. Bootstrap proxies
            // and other unused implementations can lack getVersion entirely.
            let version = if required || next.is_none() {
                match self
                    .lookup(
                        provider,
                        implementation,
                        request.block_hash,
                        timeout,
                    )
                    .await
                {
                    Ok(version) => Some(version),
                    Err(error) if required => return Err(error),
                    Err(_) => None,
                }
            } else {
                None
            };
            schedule.add_transition(index, version);
        }
        Ok(schedule)
    }

    async fn lookup(
        &self,
        provider: Option<&dyn Provider>,
        address: Address,
        hash: B256,
        timeout: Duration,
    ) -> Result<ExecutorVersion> {
        let key = (address, hash);
        if let Some(version) = self.reads.lock().unwrap().get(&key).copied() {
            return Ok(version);
        }
        let result = async {
            let provider = provider.context("Host RPC unavailable")?;
            let request = TransactionRequest::default()
                .to(address)
                .input(getVersionCall {}.abi_encode().into());
            let value = tokio::time::timeout(
                timeout,
                provider.call(request).block(BlockId::hash_canonical(hash)),
            )
            .await
            .context("Executor version lookup timed out")??;
            let reported = getVersionCall::abi_decode_returns(&value)
                .context("Invalid getVersion response")?;
            ExecutorVersion::parse(&reported).with_context(|| {
                format!("Unsupported executor version {reported:?}")
            })
        }
        .await;
        match result {
            Ok(version) => {
                self.reads.lock().unwrap().put(key, version);
                Ok(version)
            }
            Err(error) => {
                VERSION_LOOKUP_FAILURES.inc();
                warn!(
                    address = %address,
                    block_hash = %hash,
                    error = %error,
                    "Could not resolve host executor version"
                );
                Err(error).with_context(|| {
                    format!(
                        "Cannot read executor version for {address} at {hash}"
                    )
                })
            }
        }
    }
}
