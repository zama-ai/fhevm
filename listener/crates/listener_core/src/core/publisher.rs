use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Duration;

use alloy::consensus::Transaction as _;
use alloy::network::AnyTransactionReceipt;
use alloy::primitives::Address;
use alloy::rpc::types::BlockTransactions;
use broker::{Broker, Publisher, Topic};
use thiserror::Error;
use tokio::time::sleep;
use tracing::{error, info, warn};

use primitives::event::{BlockFlow, BlockPayload, IndexedLog, TransactionPayload};
use primitives::routing::consumer_new_event_routing;

use crate::blockchain::evm::evm_block_fetcher::FetchedBlock;
use crate::config::PublishConfig;
use crate::store::models::Filter;
use crate::store::repositories::Repositories;

#[derive(Error, Debug)]
pub enum PublisherError {
    #[error("Failed to build payload for block {block_number}: {reason}")]
    PayloadBuildError { block_number: u64, reason: String },
}

/// An entry in the inverted index carrying optional log-level filtering.
#[derive(Debug, Clone)]
struct FilterEntry {
    consumer_id: String,
    log_address: Option<Address>,
}

/// Inverted index built from all filters for a chain_id.
/// Enables O(1) per-transaction matching instead of O(filters) scanning.
///
/// - `consumers`: every consumer that registered at least one filter.
///   Ensures all get a payload (even empty) so downstream can track block progress.
/// - `unfiltered`: subset of `consumers` with wildcard (None, None, None) filters.
///   These receive ALL transactions with ALL logs — matching is skipped for them.
pub struct FilterIndex {
    /// Every consumer that registered at least one filter.
    consumers: HashSet<String>,
    /// Consumers with (None, None, None) wildcard — receive ALL transactions, ALL logs.
    unfiltered: HashSet<String>,
    /// from_address → entries that filter on this `from` (with optional log_address).
    by_from: HashMap<Address, Vec<FilterEntry>>,
    /// to_address → entries that filter on this `to` (with optional log_address).
    by_to: HashMap<Address, Vec<FilterEntry>>,
    /// (from, to) → entries that filter on this exact pair (with optional log_address).
    by_pair: HashMap<(Address, Address), Vec<FilterEntry>>,
    /// log_address → consumer_ids for (None, None, Some(log)) filters.
    by_log: HashMap<Address, Vec<String>>,
}

impl FilterIndex {
    /// Build the inverted index from a list of filters. O(F) time and space.
    pub fn from_filters(filters: Vec<Filter>) -> Self {
        let mut consumers = HashSet::new();
        let mut unfiltered = HashSet::new();
        let mut by_from: HashMap<Address, Vec<FilterEntry>> = HashMap::new();
        let mut by_to: HashMap<Address, Vec<FilterEntry>> = HashMap::new();
        let mut by_pair: HashMap<(Address, Address), Vec<FilterEntry>> = HashMap::new();
        let mut by_log: HashMap<Address, Vec<String>> = HashMap::new();

        for filter in filters {
            consumers.insert(filter.consumer_id.clone());

            let from_addr = filter.from.as_deref().and_then(|s| {
                s.parse::<Address>()
                    .inspect_err(|e| {
                        warn!(
                            consumer_id = %filter.consumer_id,
                            raw_from = %s,
                            error = %e,
                            "Invalid 'from' address in filter, treating as None"
                        );
                    })
                    .ok()
            });
            let to_addr = filter.to.as_deref().and_then(|s| {
                s.parse::<Address>()
                    .inspect_err(|e| {
                        warn!(
                            consumer_id = %filter.consumer_id,
                            raw_to = %s,
                            error = %e,
                            "Invalid 'to' address in filter, treating as None"
                        );
                    })
                    .ok()
            });
            let log_addr = filter.log_address.as_deref().and_then(|s| {
                s.parse::<Address>()
                    .inspect_err(|e| {
                        warn!(
                            consumer_id = %filter.consumer_id,
                            raw_log_address = %s,
                            error = %e,
                            "Invalid 'log_address' in filter, treating as None"
                        );
                    })
                    .ok()
            });

            match (from_addr, to_addr, log_addr) {
                (None, None, None) => {
                    unfiltered.insert(filter.consumer_id);
                }
                (None, None, Some(log)) => {
                    by_log.entry(log).or_default().push(filter.consumer_id);
                }
                (Some(from), Some(to), _) => {
                    by_pair.entry((from, to)).or_default().push(FilterEntry {
                        consumer_id: filter.consumer_id,
                        log_address: log_addr,
                    });
                }
                (Some(from), None, _) => {
                    by_from.entry(from).or_default().push(FilterEntry {
                        consumer_id: filter.consumer_id,
                        log_address: log_addr,
                    });
                }
                (None, Some(to), _) => {
                    by_to.entry(to).or_default().push(FilterEntry {
                        consumer_id: filter.consumer_id,
                        log_address: log_addr,
                    });
                }
            }
        }

        Self {
            consumers,
            unfiltered,
            by_from,
            by_to,
            by_pair,
            by_log,
        }
    }

    /// Match transactions and filter their logs according to filter rules.
    ///
    /// Returns a Vec of (consumer_id, filtered_transactions) pairs.
    ///
    /// Log filtering uses `Option<HashSet<Address>>` per matched (consumer, tx) pair:
    /// - `None` = include all logs (no restriction).
    /// - `Some(set)` = include only logs from addresses in the set.
    pub fn match_and_filter_transactions(
        &self,
        all_tx_payloads: &[TransactionPayload],
    ) -> Vec<(String, Vec<TransactionPayload>)> {
        // consumer_id → BTreeMap<tx_index, log_filter> (BTreeMap preserves tx order).
        // log_filter: None = all logs, Some(addrs) = only these log addresses.
        let mut matched: HashMap<String, BTreeMap<usize, Option<HashSet<Address>>>> =
            HashMap::new();

        /// Record a match: merge log filters for the same (consumer, tx) pair.
        /// - None absorbs anything (all logs).
        /// - Some(a) ∪ Some(b) = Some(a ∪ b).
        fn record_match(
            matched: &mut HashMap<String, BTreeMap<usize, Option<HashSet<Address>>>>,
            consumer_id: &str,
            tx_idx: usize,
            log_addr: Option<Address>,
        ) {
            matched
                .entry(consumer_id.to_string())
                .or_default()
                .entry(tx_idx)
                .and_modify(|existing| {
                    if let Some(set) = existing {
                        match log_addr {
                            None => *existing = None,
                            Some(addr) => {
                                set.insert(addr);
                            }
                        }
                    }
                    // else: already None (all logs), no-op
                })
                .or_insert_with(|| log_addr.map(|addr| HashSet::from([addr])));
        }

        for (i, tx_payload) in all_tx_payloads.iter().enumerate() {
            let from = tx_payload.from;
            let to = tx_payload.to;

            // by_from lookup
            if let Some(entries) = self.by_from.get(&from) {
                for entry in entries {
                    record_match(&mut matched, &entry.consumer_id, i, entry.log_address);
                }
            }

            // by_to and by_pair lookup
            if let Some(to_addr) = to {
                if let Some(entries) = self.by_to.get(&to_addr) {
                    for entry in entries {
                        record_match(&mut matched, &entry.consumer_id, i, entry.log_address);
                    }
                }
                if let Some(entries) = self.by_pair.get(&(from, to_addr)) {
                    for entry in entries {
                        record_match(&mut matched, &entry.consumer_id, i, entry.log_address);
                    }
                }
            }

            // by_log: scan logs for matching addresses
            if !self.by_log.is_empty() {
                for log in &tx_payload.logs {
                    if let Some(consumer_ids) = self.by_log.get(&log.address) {
                        for consumer_id in consumer_ids {
                            record_match(&mut matched, consumer_id, i, Some(log.address));
                        }
                    }
                }
            }
        }

        // Assemble results — one entry per consumer.
        let mut results = Vec::with_capacity(self.consumers.len());

        for consumer_id in &self.consumers {
            let transactions = if self.unfiltered.contains(consumer_id) {
                // Unfiltered consumer: all transactions, all logs, no filtering.
                all_tx_payloads.to_vec()
            } else {
                matched
                    .get(consumer_id.as_str())
                    .map(|tx_map| {
                        tx_map
                            .iter()
                            .map(|(&idx, log_filter)| {
                                let mut tx = all_tx_payloads[idx].clone();
                                if let Some(addrs) = log_filter {
                                    tx.logs.retain(|log| addrs.contains(&log.address));
                                }
                                // None = all logs, no filtering needed.
                                tx
                            })
                            .collect()
                    })
                    .unwrap_or_default()
            };

            results.push((consumer_id.clone(), transactions));
        }

        results
    }

    /// Build one BlockPayload per consumer_id from a fetched block.
    ///
    /// Returns a Vec of (consumer_id, BlockPayload) pairs.
    /// Complexity: O(T × A + C) where T = transactions, A = avg fan-out, C = consumers.
    pub fn build_block_payloads(
        &self,
        fetched_block: &FetchedBlock,
        chain_id: u64,
        flow: BlockFlow,
    ) -> Result<Vec<(String, BlockPayload)>, PublisherError> {
        let block = &fetched_block.block;
        let block_number = block.header.number;
        let block_hash = block.header.hash;
        let parent_hash = block.header.parent_hash;
        let timestamp = block.header.timestamp;

        // Extract full transactions from the block.
        // Guaranteed by our RPC calls (full=true hardcoded in sem_evm_rpc_provider.rs),
        // but checked defensively because the alloy type allows Hashes and Uncle variants.
        let txs = match &block.transactions {
            BlockTransactions::Full(txs) => txs,
            _ => {
                return Err(PublisherError::PayloadBuildError {
                    block_number,
                    reason: "block does not contain full transactions".to_string(),
                });
            }
        };

        // Pre-build all TransactionPayloads once (shared across consumers).
        let all_tx_payloads: Vec<TransactionPayload> = txs
            .iter()
            .enumerate()
            .map(|(i, tx)| {
                let recovered = &tx.inner.inner;
                let from = recovered.signer();
                let tx_envelope = recovered.inner();

                let tx_hash = block.transactions.hashes().nth(i).ok_or_else(|| {
                    PublisherError::PayloadBuildError {
                        block_number,
                        reason: format!("missing hash for tx index {i}"),
                    }
                })?;
                let to = tx_envelope.to();
                let value = tx_envelope.value();
                let data = tx_envelope.input().clone();

                // Safety: build_fetched_block() validates receipt completeness for all strategies.
                // A missing receipt here indicates corrupted data — stale and retry to self-heal.
                let logs = fetched_block
                    .get_receipt(&tx_hash)
                    .map(build_indexed_logs)
                    .ok_or_else(|| PublisherError::PayloadBuildError {
                        block_number,
                        reason: format!("missing receipt for tx {tx_hash}"),
                    })?;

                Ok(TransactionPayload {
                    from,
                    to,
                    hash: tx_hash,
                    transaction_index: i as u64,
                    value,
                    data,
                    logs,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Match and filter transactions per consumer.
        let consumer_txs = self.match_and_filter_transactions(&all_tx_payloads);

        // Wrap in BlockPayload.
        Ok(consumer_txs
            .into_iter()
            .map(|(consumer_id, transactions)| {
                (
                    consumer_id,
                    BlockPayload {
                        flow,
                        chain_id,
                        block_number,
                        block_hash,
                        parent_hash,
                        timestamp,
                        transactions,
                    },
                )
            })
            .collect())
    }
}

/// Build IndexedLog entries from a transaction receipt.
fn build_indexed_logs(receipt: &AnyTransactionReceipt) -> Vec<IndexedLog> {
    receipt
        .inner
        .logs()
        .iter()
        .map(|log| IndexedLog {
            log_index: log.log_index.unwrap_or(0),
            address: log.address(),
            topics: log.topics().to_vec(),
            data: log.data().data.clone(),
        })
        .collect()
}

/// Orchestration function: fetch filters, build index, match, and publish.
///
/// Called from evm_listener.rs before each block is persisted to DB.
/// Uses infinite per-consumer retry to guarantee all consumers receive the event
/// before the function returns — prevents duplicate publishing on outer retry.
///
/// Returns `Err(PublisherError)` if payload construction fails (missing full txs,
/// missing receipt, etc.). Callers MUST treat this as a signal to NOT advance DB
/// state — the error is transient and a re-fetch from RPC can self-heal.
pub async fn publish_block_events(
    repositories: &Repositories,
    fetched_block: &FetchedBlock,
    chain_id: u64,
    flow: BlockFlow,
    broker: &Broker,
    event_publisher: &Publisher,
    publish_config: &PublishConfig,
) -> Result<(), PublisherError> {
    let publish_retry_delay = Duration::from_secs(publish_config.publish_retry_secs);

    // 1. Fetch filters — retry infinitely on DB error.
    let filters = loop {
        match repositories.filters.get_filters_by_chain_id().await {
            Ok(f) => break f,
            Err(e) => {
                error!(error = %e, "Failed to fetch filters, retrying");
                sleep(publish_retry_delay).await;
            }
        }
    };

    if filters.is_empty() {
        return Ok(());
    }

    // 2. Build inverted index from filters — O(F).
    let filter_index = FilterIndex::from_filters(filters);

    // 3. Match transactions and build per-consumer payloads.
    //    PayloadBuildError propagated to caller — data may be stale, re-fetch can self-heal.
    let payloads = filter_index.build_block_payloads(fetched_block, chain_id, flow)?;

    // 4. For each consumer: verify queue exists, then publish.
    //    When publish_stale=true: retry queue existence forever (stall until queue appears).
    //    When publish_stale=false: bounded retry via publish_no_stale_retries, then skip consumer.
    //    Broker/publish errors still retry infinitely (infrastructure-level, affects all consumers equally).
    for (consumer_id, payload) in &payloads {
        let routing_key = consumer_new_event_routing(consumer_id.clone());
        let topic = Topic::new(&routing_key);
        let mut queue_not_found_attempts: u32 = 0;

        loop {
            // Check queue existence (prevents AMQP silent drops).
            let queue_exists = match broker.exists(&topic).await {
                Ok(exists) => exists,
                Err(e) => {
                    error!(
                        consumer_id = %consumer_id,
                        block_number = payload.block_number,
                        error = %e,
                        "Failed to check queue existence, retrying"
                    );
                    sleep(publish_retry_delay).await;
                    continue;
                }
            };

            if !queue_exists {
                queue_not_found_attempts += 1;
                if !publish_config.publish_stale
                    && queue_not_found_attempts >= publish_config.publish_no_stale_retries
                {
                    error!(
                        consumer_id = %consumer_id,
                        block_number = payload.block_number,
                        routing_key = %routing_key,
                        attempts = queue_not_found_attempts,
                        "Consumer queue not found after max retries, skipping consumer"
                    );
                    break; // Skip this consumer — move to next.
                }
                if publish_config.publish_stale {
                    error!(
                        consumer_id = %consumer_id,
                        block_number = payload.block_number,
                        routing_key = %routing_key,
                        attempt = queue_not_found_attempts,
                        "Consumer queue not found, retrying indefinitely (publish_stale=true)"
                    );
                } else {
                    error!(
                        consumer_id = %consumer_id,
                        block_number = payload.block_number,
                        routing_key = %routing_key,
                        attempt = queue_not_found_attempts,
                        max_attempts = publish_config.publish_no_stale_retries,
                        "Consumer queue not found, retrying"
                    );
                }
                sleep(publish_retry_delay).await;
                continue;
            }

            // Publish.
            match event_publisher.publish(&routing_key, payload).await {
                Ok(()) => {
                    info!(
                        consumer_id = %consumer_id,
                        block_number = payload.block_number,
                        tx_count = payload.transactions.len(),
                        routing_key = %routing_key,
                        "Published block event to consumer"
                    );
                    break; // Success — move to next consumer.
                }
                Err(e) => {
                    error!(
                        consumer_id = %consumer_id,
                        block_number = payload.block_number,
                        error = %e,
                        "Publish failed, retrying"
                    );
                    sleep(publish_retry_delay).await;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{B256, Bytes, U256};
    use chrono::Utc;
    use uuid::Uuid;

    // Address constants for readability.
    const ADDR_1: &str = "0x0000000000000000000000000000000000000001";
    const ADDR_2: &str = "0x0000000000000000000000000000000000000002";
    const ADDR_3: &str = "0x0000000000000000000000000000000000000003";
    const ADDR_4: &str = "0x0000000000000000000000000000000000000004";
    const ADDR_5: &str = "0x0000000000000000000000000000000000000005";

    fn addr(s: &str) -> Address {
        s.parse().unwrap()
    }

    fn make_filter(
        consumer_id: &str,
        from: Option<&str>,
        to: Option<&str>,
        log_address: Option<&str>,
    ) -> Filter {
        Filter {
            id: Uuid::new_v4(),
            chain_id: 1,
            consumer_id: consumer_id.to_string(),
            from: from.map(|s| s.to_string()),
            to: to.map(|s| s.to_string()),
            log_address: log_address.map(|s| s.to_string()),
            created_at: Utc::now(),
        }
    }

    /// Build a TransactionPayload with specified from, to, and logs.
    /// `logs` is a slice of (address_str, log_index) tuples.
    fn make_tx(from: &str, to: &str, logs: &[(&str, u64)]) -> TransactionPayload {
        TransactionPayload {
            from: addr(from),
            to: Some(addr(to)),
            hash: B256::ZERO,
            transaction_index: 0,
            value: U256::ZERO,
            data: Bytes::new(),
            logs: logs
                .iter()
                .map(|(a, idx)| IndexedLog {
                    log_index: *idx,
                    address: addr(a),
                    topics: vec![],
                    data: Bytes::new(),
                })
                .collect(),
        }
    }

    /// Find the transaction list for a given consumer in match results.
    fn find_consumer_txs<'a>(
        results: &'a [(String, Vec<TransactionPayload>)],
        consumer_id: &str,
    ) -> &'a Vec<TransactionPayload> {
        &results
            .iter()
            .find(|(id, _)| id == consumer_id)
            .unwrap_or_else(|| panic!("consumer {consumer_id} not found in results"))
            .1
    }

    // ---- Index construction tests ----

    #[test]
    fn wildcard_filter_is_classified_correctly() {
        let filters = vec![make_filter("consumer_a", None, None, None)];
        let index = FilterIndex::from_filters(filters);

        assert!(index.unfiltered.contains("consumer_a"));
        assert_eq!(index.consumers.len(), 1);
        assert!(index.by_from.is_empty());
        assert!(index.by_to.is_empty());
        assert!(index.by_pair.is_empty());
        assert!(index.by_log.is_empty());
    }

    #[test]
    fn from_only_filter_is_indexed() {
        let filters = vec![make_filter("consumer_a", Some(ADDR_1), None, None)];
        let index = FilterIndex::from_filters(filters);

        let parsed = addr(ADDR_1);
        assert!(index.by_from.contains_key(&parsed));
        assert_eq!(index.by_from[&parsed].len(), 1);
        assert_eq!(index.by_from[&parsed][0].consumer_id, "consumer_a");
        assert!(index.by_from[&parsed][0].log_address.is_none());
        assert!(index.unfiltered.is_empty());
    }

    #[test]
    fn to_only_filter_is_indexed() {
        let filters = vec![make_filter("consumer_b", None, Some(ADDR_2), None)];
        let index = FilterIndex::from_filters(filters);

        let parsed = addr(ADDR_2);
        assert!(index.by_to.contains_key(&parsed));
        assert_eq!(index.by_to[&parsed].len(), 1);
        assert_eq!(index.by_to[&parsed][0].consumer_id, "consumer_b");
        assert!(index.by_to[&parsed][0].log_address.is_none());
    }

    #[test]
    fn pair_filter_is_indexed() {
        let filters = vec![make_filter("consumer_c", Some(ADDR_1), Some(ADDR_2), None)];
        let index = FilterIndex::from_filters(filters);

        let from_parsed = addr(ADDR_1);
        let to_parsed = addr(ADDR_2);
        assert!(index.by_pair.contains_key(&(from_parsed, to_parsed)));
        assert_eq!(index.by_pair[&(from_parsed, to_parsed)].len(), 1);
        assert_eq!(
            index.by_pair[&(from_parsed, to_parsed)][0].consumer_id,
            "consumer_c"
        );
        assert!(index.by_from.is_empty());
        assert!(index.by_to.is_empty());
    }

    #[test]
    fn multiple_consumers_multiple_filter_types() {
        let filters = vec![
            make_filter("wildcard_consumer", None, None, None),
            make_filter("from_consumer", Some(ADDR_1), None, None),
            make_filter("to_consumer", None, Some(ADDR_2), None),
            make_filter("pair_consumer", Some(ADDR_1), Some(ADDR_2), None),
            make_filter("log_consumer", None, None, Some(ADDR_3)),
        ];
        let index = FilterIndex::from_filters(filters);

        assert_eq!(index.consumers.len(), 5);
        assert!(index.unfiltered.contains("wildcard_consumer"));
        assert_eq!(index.by_from.len(), 1);
        assert_eq!(index.by_to.len(), 1);
        assert_eq!(index.by_pair.len(), 1);
        assert_eq!(index.by_log.len(), 1);
    }

    #[test]
    fn invalid_address_in_filter_treated_as_none() {
        let filters = vec![make_filter(
            "consumer_bad",
            Some("not_an_address"),
            None,
            None,
        )];
        let index = FilterIndex::from_filters(filters);

        assert!(index.unfiltered.contains("consumer_bad"));
        assert!(index.by_from.is_empty());
    }

    #[test]
    fn empty_filters_produces_empty_index() {
        let index = FilterIndex::from_filters(vec![]);

        assert!(index.consumers.is_empty());
        assert!(index.unfiltered.is_empty());
        assert!(index.by_from.is_empty());
        assert!(index.by_to.is_empty());
        assert!(index.by_pair.is_empty());
        assert!(index.by_log.is_empty());
    }

    #[test]
    fn duplicate_consumer_across_filter_types() {
        let filters = vec![
            make_filter("consumer_x", None, None, None),
            make_filter("consumer_x", Some(ADDR_1), Some(ADDR_2), None),
        ];
        let index = FilterIndex::from_filters(filters);

        assert_eq!(index.consumers.len(), 1);
        assert!(index.unfiltered.contains("consumer_x"));
        assert_eq!(index.by_pair.len(), 1);
    }

    #[test]
    fn log_only_filter_is_indexed() {
        let filters = vec![make_filter("log_consumer", None, None, Some(ADDR_3))];
        let index = FilterIndex::from_filters(filters);

        let parsed = addr(ADDR_3);
        assert!(index.by_log.contains_key(&parsed));
        assert_eq!(index.by_log[&parsed], vec!["log_consumer"]);
        assert!(index.unfiltered.is_empty());
        assert!(index.by_from.is_empty());
        assert!(index.by_to.is_empty());
        assert!(index.by_pair.is_empty());
    }

    #[test]
    fn from_with_log_address_is_indexed_in_from_index() {
        let filters = vec![make_filter("consumer_d", Some(ADDR_1), None, Some(ADDR_3))];
        let index = FilterIndex::from_filters(filters);

        let parsed = addr(ADDR_1);
        assert!(index.by_from.contains_key(&parsed));
        assert_eq!(index.by_from[&parsed].len(), 1);
        assert_eq!(index.by_from[&parsed][0].consumer_id, "consumer_d");
        assert_eq!(index.by_from[&parsed][0].log_address, Some(addr(ADDR_3)));
        assert!(index.by_log.is_empty());
    }

    #[test]
    fn invalid_log_address_treated_as_none() {
        let filters = vec![make_filter(
            "consumer_e",
            Some(ADDR_1),
            None,
            Some("not_valid"),
        )];
        let index = FilterIndex::from_filters(filters);

        let parsed = addr(ADDR_1);
        assert!(index.by_from.contains_key(&parsed));
        assert!(index.by_from[&parsed][0].log_address.is_none());
    }

    // ---- Matching behavior tests ----

    #[test]
    fn wildcard_receives_all_txs_all_logs() {
        let filters = vec![make_filter("wc", None, None, None)];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![
            make_tx(ADDR_1, ADDR_2, &[(ADDR_3, 0)]),
            make_tx(ADDR_4, ADDR_5, &[(ADDR_3, 1)]),
        ];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "wc");
        assert_eq!(consumer_txs.len(), 2);
        assert_eq!(consumer_txs[0].logs.len(), 1);
        assert_eq!(consumer_txs[1].logs.len(), 1);
    }

    #[test]
    fn from_filter_matches_and_includes_all_logs() {
        let filters = vec![make_filter("fc", Some(ADDR_1), None, None)];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![
            make_tx(ADDR_1, ADDR_2, &[(ADDR_3, 0), (ADDR_4, 1)]),
            make_tx(ADDR_5, ADDR_2, &[(ADDR_3, 2)]),
        ];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "fc");
        assert_eq!(consumer_txs.len(), 1);
        // All logs included (no log_address filter).
        assert_eq!(consumer_txs[0].logs.len(), 2);
    }

    #[test]
    fn log_only_filter_matches_tx_with_matching_log_and_filters_logs() {
        let filters = vec![make_filter("lc", None, None, Some(ADDR_3))];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![make_tx(ADDR_1, ADDR_2, &[(ADDR_3, 0), (ADDR_4, 1)])];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "lc");
        assert_eq!(consumer_txs.len(), 1);
        // Only the matching log is included.
        assert_eq!(consumer_txs[0].logs.len(), 1);
        assert_eq!(consumer_txs[0].logs[0].address, addr(ADDR_3));
    }

    #[test]
    fn log_only_does_not_match_tx_without_matching_log() {
        let filters = vec![make_filter("lc", None, None, Some(ADDR_3))];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![make_tx(ADDR_1, ADDR_2, &[(ADDR_4, 0), (ADDR_5, 1)])];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "lc");
        assert_eq!(consumer_txs.len(), 0);
    }

    #[test]
    fn from_with_log_address_matches_tx_and_filters_logs() {
        let filters = vec![make_filter("fc_log", Some(ADDR_1), None, Some(ADDR_3))];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![make_tx(
            ADDR_1,
            ADDR_2,
            &[(ADDR_3, 0), (ADDR_4, 1), (ADDR_5, 2)],
        )];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "fc_log");
        assert_eq!(consumer_txs.len(), 1);
        assert_eq!(consumer_txs[0].logs.len(), 1);
        assert_eq!(consumer_txs[0].logs[0].address, addr(ADDR_3));
    }

    #[test]
    fn broad_filter_supersedes_log_filter_for_same_consumer() {
        // Same consumer has a broad from filter AND a log-only filter.
        // The broad match (All) should supersede the narrow one (Specific).
        let filters = vec![
            make_filter("c", Some(ADDR_1), None, None), // broad: all logs
            make_filter("c", None, None, Some(ADDR_3)), // narrow: only ADDR_3 logs
        ];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![make_tx(ADDR_1, ADDR_2, &[(ADDR_3, 0), (ADDR_4, 1)])];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "c");
        assert_eq!(consumer_txs.len(), 1);
        // Broad subsumes narrow → all logs.
        assert_eq!(consumer_txs[0].logs.len(), 2);
    }

    #[test]
    fn multiple_log_only_filters_merge_addresses() {
        let filters = vec![
            make_filter("lc", None, None, Some(ADDR_3)),
            make_filter("lc", None, None, Some(ADDR_4)),
        ];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![make_tx(
            ADDR_1,
            ADDR_2,
            &[(ADDR_3, 0), (ADDR_4, 1), (ADDR_5, 2)],
        )];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "lc");
        assert_eq!(consumer_txs.len(), 1);
        // Both ADDR_3 and ADDR_4 logs included, but not ADDR_5.
        assert_eq!(consumer_txs[0].logs.len(), 2);
        let log_addrs: HashSet<Address> = consumer_txs[0].logs.iter().map(|l| l.address).collect();
        assert!(log_addrs.contains(&addr(ADDR_3)));
        assert!(log_addrs.contains(&addr(ADDR_4)));
    }

    #[test]
    fn from_filter_and_log_only_filter_merge_for_same_tx() {
        // from_filter with log_address + log_only filter for different address, same consumer.
        let filters = vec![
            make_filter("c", Some(ADDR_1), None, Some(ADDR_3)),
            make_filter("c", None, None, Some(ADDR_4)),
        ];
        let index = FilterIndex::from_filters(filters);

        let txs = vec![make_tx(
            ADDR_1,
            ADDR_2,
            &[(ADDR_3, 0), (ADDR_4, 1), (ADDR_5, 2)],
        )];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "c");
        assert_eq!(consumer_txs.len(), 1);
        // ADDR_3 (from from_filter) + ADDR_4 (from log_only) merged.
        assert_eq!(consumer_txs[0].logs.len(), 2);
        let log_addrs: HashSet<Address> = consumer_txs[0].logs.iter().map(|l| l.address).collect();
        assert!(log_addrs.contains(&addr(ADDR_3)));
        assert!(log_addrs.contains(&addr(ADDR_4)));
    }

    #[test]
    fn no_filter_match_produces_empty_tx_list() {
        let filters = vec![make_filter("fc", Some(ADDR_1), None, None)];
        let index = FilterIndex::from_filters(filters);

        // Transaction from ADDR_5, not ADDR_1.
        let txs = vec![make_tx(ADDR_5, ADDR_2, &[(ADDR_3, 0)])];
        let results = index.match_and_filter_transactions(&txs);

        let consumer_txs = find_consumer_txs(&results, "fc");
        assert_eq!(consumer_txs.len(), 0);
    }
}
