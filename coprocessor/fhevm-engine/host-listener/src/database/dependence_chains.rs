use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use tracing::{debug, error, warn};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

use crate::database::tfhe_event_propagate::{
    tfhe_inputs_handle, tfhe_result_handle, ChainHash,
};
use crate::database::tfhe_event_propagate::{
    Chain, ChainCache, Handle, LogTfhe, OrderedChains, TransactionHash,
};

#[derive(Clone, Debug)]
struct Transaction {
    tx_hash: TransactionHash,
    input_handle: Vec<Handle>,
    output_handle: Vec<Handle>,
    allowed_handle: Vec<Handle>,
    input_tx: HashSet<TransactionHash>,
    linear_chain: TransactionHash,
    size: u64,
    depth_size: u64,
}

impl Transaction {
    fn new(tx_hash: TransactionHash) -> Self {
        Self {
            tx_hash,
            input_handle: Vec::with_capacity(5),
            output_handle: Vec::with_capacity(5),
            allowed_handle: Vec::with_capacity(5),
            input_tx: HashSet::with_capacity(3),
            linear_chain: tx_hash, //  before coalescing linear tx chains
            size: 0,
            depth_size: 0,
        }
    }
}

fn ensure_logs_order(logs: &mut [LogTfhe]) {
    if logs.iter().any(|log| log.log_index.is_none()) {
        warn!("Log without index, cannot ensure order, assuming it's ordered");
        return;
    }
    // Note: there is a fast path for already sorted logs
    logs.sort_by_key(|log| log.log_index.unwrap_or(0));
}

const AVG_LOGS_PER_TX: usize = 8;
fn scan_transactions(
    logs: &[LogTfhe],
) -> (Vec<TransactionHash>, HashMap<TransactionHash, Transaction>) {
    // TODO: OPT no need for hashmap if contiguous tx
    let mut txs = HashMap::new();
    let mut ordered_txs_hash = Vec::with_capacity(logs.len() / AVG_LOGS_PER_TX);
    for log in logs {
        let tx_hash = log.transaction_hash.unwrap_or_default();
        let tx_entry = txs.entry(tx_hash);
        let tx = match tx_entry {
            Entry::Vacant(e) => {
                ordered_txs_hash.push(tx_hash);
                e.insert(Transaction::new(tx_hash))
            }
            Entry::Occupied(e) => e.into_mut(),
        };
        tx.size += 1;
        let log_inputs = tfhe_inputs_handle(&log.event);
        for input in log_inputs {
            if tx.output_handle.contains(&input) {
                // self dependency, ignore, assuming logs are ordered in tx
                continue;
            }
            tx.input_handle.push(input);
        }
        if let Some(output) = tfhe_result_handle(&log.event) {
            tx.output_handle.push(output);
            if log.is_allowed {
                tx.allowed_handle.push(output);
            }
        }
    }
    (ordered_txs_hash, txs)
}

async fn fill_tx_dependence_maps(
    ordered_txs_hash: &[TransactionHash],
    txs: &mut HashMap<TransactionHash, Transaction>,
    past_chains: &ChainCache,
) {
    let mut allowed_handle_tx: HashMap<Handle, TransactionHash> =
        HashMap::new();
    for tx_hash in ordered_txs_hash {
        let Some(tx) = txs.get_mut(tx_hash) else {
            error!("Tx hash {:?} not found in txs map", tx_hash);
            continue;
        };
        // this tx depends on dep_tx
        let mut producer_tx = Vec::with_capacity(tx.input_handle.len());
        for input_handle in &tx.input_handle {
            if let Some(dep_tx) = allowed_handle_tx.get(input_handle) {
                // intra block
                tx.input_tx.insert(*dep_tx);
                producer_tx.push(*dep_tx);
            } else if let Some(dep_tx_hash) =
                past_chains.write().await.get(input_handle)
            {
                // extra block, this is directly a chain hash
                tx.input_tx.insert(*dep_tx_hash);
            }
        }
        // update allowed handle for next txs
        for allowed_handle in &tx.allowed_handle {
            allowed_handle_tx.entry(*allowed_handle).or_insert(*tx_hash);
        }
        // propagate memorized producers
        let mut depth_size = 0;
        for dep_tx in &producer_tx {
            if let Some(dep_tx) = txs.get(dep_tx) {
                depth_size = depth_size.max(dep_tx.depth_size + dep_tx.size);
            }
        }
        txs.entry(*tx_hash).and_modify(|dep_tx| {
            dep_tx.depth_size = depth_size;
        });
    }
}

fn grouping_to_component_chains(
    ordered_txs: &mut [Transaction],
    across_blocks: bool,
) -> OrderedChains {
    let mut uf = QuickUnionUf::<UnionBySize>::new(ordered_txs.len());
    let mut tx_index = HashMap::with_capacity(ordered_txs.len());
    let tx_hash = ordered_txs.iter().map(|tx| tx.tx_hash).collect::<Vec<_>>();
    for (index, tx_hash) in tx_hash.iter().enumerate() {
        tx_index.insert(tx_hash, index);
    }
    // create connected components of current block
    for (key, tx) in ordered_txs.iter().enumerate() {
        for dep_hash in &tx.input_tx {
            let Some(&dep_key) = tx_index.get(dep_hash) else {
                // from previous block
                continue;
            };
            uf.union(key, dep_key);
            debug!(
                "Union tx {:?} with dep tx {:?} to {:?} {:?}",
                tx.tx_hash,
                dep_hash,
                uf.find(key),
                uf.get(key)
            );
        }
    }
    let mut txs_component = Vec::with_capacity(ordered_txs.len());
    for key in 0..ordered_txs.len() {
        txs_component.push(uf.find(key));
    }

    let mut component_members: HashMap<usize, Vec<usize>> =
        HashMap::with_capacity(ordered_txs.len());
    let mut ordered_components = Vec::with_capacity(ordered_txs.len());
    for (index, component) in txs_component.iter().copied().enumerate() {
        if let Entry::Vacant(entry) = component_members.entry(component) {
            ordered_components.push(component);
            entry.insert(Vec::new());
        }
        component_members
            .get_mut(&component)
            .expect("component was just inserted")
            .push(index);
    }

    let mut ordered_chains_hash = Vec::with_capacity(ordered_components.len());
    let mut chains: HashMap<ChainHash, Chain> =
        HashMap::with_capacity(ordered_components.len());
    for component in ordered_components {
        let Some(members) = component_members.remove(&component) else {
            continue;
        };
        let Some(component_hash) =
            members.iter().map(|index| tx_hash[*index]).min()
        else {
            continue;
        };

        let mut size = 0_u64;
        let mut before_size = u64::MAX;
        let mut allowed_handle = Vec::new();
        let mut split_dependencies = HashSet::new();
        for index in members {
            let tx = &mut ordered_txs[index];
            tx.linear_chain = component_hash;
            size = size.saturating_add(tx.size);
            before_size = before_size.min(tx.depth_size);
            allowed_handle.extend(tx.allowed_handle.iter());
            if across_blocks {
                for dep_hash in &tx.input_tx {
                    if !tx_index.contains_key(dep_hash) {
                        split_dependencies.insert(*dep_hash);
                    }
                }
            }
        }

        let mut split_dependencies =
            split_dependencies.into_iter().collect::<Vec<_>>();
        split_dependencies.sort();
        ordered_chains_hash.push(component_hash);
        chains.insert(
            component_hash,
            Chain {
                hash: component_hash,
                size,
                before_size: if before_size == u64::MAX {
                    0
                } else {
                    before_size
                },
                // Same-block component DCIDs are the acquisition unit. Cross-block
                // links stay in split_dependencies for slow-lane propagation; they
                // are not dependency_count edges because existing release bookkeeping
                // only decrements same-block dependents.
                dependencies: vec![],
                split_dependencies,
                dependents: vec![],
                allowed_handle,
                new_chain: true,
            },
        );
    }

    ordered_chains_hash
        .iter()
        .filter_map(|hash| chains.remove(hash))
        .collect()
}

pub async fn dependence_chains(
    logs: &mut [LogTfhe],
    past_chains: &ChainCache,
    across_blocks: bool,
) -> OrderedChains {
    ensure_logs_order(logs);
    let (ordered_hash, mut txs) = scan_transactions(logs);
    fill_tx_dependence_maps(&ordered_hash, &mut txs, past_chains).await;
    debug!("Transactions: {:?}", txs.values());
    let mut ordered_txs: Vec<_> = ordered_hash
        .iter()
        .filter_map(|tx_hash| txs.remove(tx_hash))
        .collect();
    let chains = grouping_to_component_chains(&mut ordered_txs, across_blocks);
    // propagate to logs
    let txs = ordered_txs
        .iter()
        .map(|tx| (tx.tx_hash, tx))
        .collect::<HashMap<_, _>>();
    for log in logs.iter_mut() {
        let tx_hash = log.transaction_hash.unwrap_or_default();
        if let Some(tx) = txs.get(&tx_hash) {
            log.dependence_chain = tx.linear_chain;
            log.tx_depth_size = tx.depth_size;
        } else {
            // past chain
            log.dependence_chain = tx_hash;
        }
    }
    if across_blocks {
        // propagate to cache
        for chain in &chains {
            for handle in &chain.allowed_handle {
                past_chains.write().await.put(*handle, chain.hash);
            }
        }
    }
    chains
}

#[cfg(test)]
mod tests {
    use alloy::primitives::FixedBytes;
    use alloy_primitives::Address;

    use crate::contracts::TfheContract as C;
    use crate::contracts::TfheContract::TfheContractEvents as E;
    use crate::database::dependence_chains::dependence_chains;
    use crate::database::tfhe_event_propagate::{Chain, ChainCache, LogTfhe};
    use crate::database::tfhe_event_propagate::{
        ClearConst, Handle, TransactionHash,
    };

    fn caller() -> Address {
        Address::from_slice(&[0x11u8; 20])
    }

    fn tfhe_event(data: E) -> alloy::primitives::Log<E> {
        let address = "0x0000000000000000000000000000000000000000"
            .parse()
            .unwrap();
        alloy::primitives::Log::<E> { address, data }
    }

    fn push_event(
        e: E,
        logs: &mut Vec<LogTfhe>,
        is_allowed: bool,
        tx: TransactionHash,
    ) {
        static COUNTER: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(0);
        logs.push(LogTfhe {
            event: tfhe_event(e),
            is_allowed,
            block_number: 0,
            block_hash: TransactionHash::ZERO,
            block_timestamp: sqlx::types::time::PrimitiveDateTime::MIN,
            transaction_hash: Some(tx),
            dependence_chain: TransactionHash::ZERO,
            tx_depth_size: 0,
            log_index: Some(
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            ),
        })
    }

    fn new_handle() -> Handle {
        static HANDLE_COUNTER: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(1000);
        let id =
            HANDLE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Handle::from_slice(&[
            // 32 bytes
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            (id >> 56) as u8,
            (id >> 48) as u8,
            (id >> 40) as u8,
            (id >> 32) as u8,
            (id >> 24) as u8,
            (id >> 16) as u8,
            (id >> 8) as u8,
            id as u8,
        ])
    }

    fn input_handle(logs: &mut Vec<LogTfhe>, tx: TransactionHash) -> Handle {
        let result = new_handle();
        push_event(
            E::TrivialEncrypt(C::TrivialEncrypt {
                caller: caller(),
                pt: ClearConst::from_be_slice(&[0]),
                toType: 0,
                result,
            }),
            logs,
            false,
            tx,
        );
        result
    }

    fn input_shared_handle(
        logs: &mut Vec<LogTfhe>,
        handle: Handle,
        tx: TransactionHash,
    ) -> Handle {
        push_event(
            E::TrivialEncrypt(C::TrivialEncrypt {
                caller: caller(),
                pt: ClearConst::from_be_slice(&[0]),
                toType: 0,
                result: handle,
            }),
            logs,
            false,
            tx,
        );
        handle
    }

    fn op1(
        handle: Handle,
        logs: &mut Vec<LogTfhe>,
        tx: TransactionHash,
    ) -> Handle {
        let result = new_handle();
        push_event(
            E::FheAdd(C::FheAdd {
                lhs: handle,
                rhs: handle,
                scalarByte: FixedBytes::from_slice(&[0]),
                result,
                caller: caller(),
            }),
            logs,
            true,
            tx,
        );
        result
    }

    fn op2(
        handle1: Handle,
        handle2: Handle,
        logs: &mut Vec<LogTfhe>,
        tx: TransactionHash,
    ) -> Handle {
        let result = new_handle();
        push_event(
            E::FheAdd(C::FheAdd {
                lhs: handle1,
                rhs: handle2,
                scalarByte: FixedBytes::from_slice(&[0]),
                result,
                caller: caller(),
            }),
            logs,
            true,
            tx,
        );
        result
    }

    fn new_cache() -> ChainCache {
        ChainCache::new(tokio::sync::RwLock::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        )))
    }

    #[tokio::test]
    async fn test_dependence_chains_1_local_chain() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let v0 = input_handle(&mut logs, tx1);
        let _v1 = op1(v0, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);

        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let _vb_2 = op1(va_2, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 2);
        assert!(logs[0..2].iter().all(|log| log.dependence_chain == tx1));
        assert!(logs[2..4].iter().all(|log| log.dependence_chain == tx2));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain_mixed() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);
        let tx3 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let vb_2 = op1(va_2, &mut logs, tx2);
        let _vc_1 = op2(vb_1, vb_2, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert!(chains[0].dependencies.is_empty());
        assert!(chains[0].split_dependencies.is_empty());
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_dependence_chains_2_local_chain_mixed_bis() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);
        let tx3 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let vb_2 = op1(va_2, &mut logs, tx2);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let _vc_1 = op2(vb_1, vb_2, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert_eq!(logs[0].dependence_chain, tx1);
        assert_eq!(logs[1].dependence_chain, tx1);
        assert_eq!(logs[2].dependence_chain, tx1);
        assert_eq!(logs[3].dependence_chain, tx1);
        assert_eq!(logs[4].dependence_chain, tx1);
        assert_eq!(logs[0].tx_depth_size, 0);
        assert_eq!(logs[1].tx_depth_size, 0);
        assert_eq!(logs[2].tx_depth_size, 0);
        assert_eq!(logs[3].tx_depth_size, 0);
        assert_eq!(logs[4].tx_depth_size, 2);
        assert_eq!(cache.read().await.len(), 3);
        assert_eq!(chains[0].before_size, 0);
        assert_eq!(chains[0].dependencies.len(), 0);
        assert_eq!(chains[0].split_dependencies.len(), 0);
        assert!(chains[0].dependents.is_empty());
    }

    fn past_chain(last_byte: u8) -> Chain {
        Chain {
            hash: TransactionHash::with_last_byte(last_byte),
            dependencies: vec![],
            split_dependencies: vec![],
            dependents: vec![],
            size: 1,
            before_size: 0,
            allowed_handle: vec![],
            new_chain: false,
        }
    }

    #[tokio::test]
    async fn test_dependence_chains_1_known_past_handle() {
        let cache = new_cache();
        let mut logs = vec![];
        let past_handle = new_handle();
        let past_chain = past_chain(0);
        let past_chain_hash = past_chain.hash;
        cache.write().await.put(past_handle, past_chain_hash);
        let tx1 = TransactionHash::with_last_byte(1);
        let _va_1 = op1(past_handle, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert_eq!(chains[0].split_dependencies, vec![past_chain_hash]);
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_1_unknown_past_handle() {
        let cache = new_cache();
        let mut logs = vec![];
        let past_handle = new_handle();
        let tx1 = TransactionHash::with_last_byte(1);
        let _va_1 = op1(past_handle, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain.hash == tx1));
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_1_local_and_known_past_handle() {
        let cache = new_cache();
        let past_handle = new_handle();
        let past_chain = past_chain(0);
        let past_chain_hash = past_chain.hash;
        cache.write().await.put(past_handle, past_chain_hash);
        let tx1 = TransactionHash::with_last_byte(1);
        let mut logs = vec![];
        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op2(past_handle, va_1, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert_eq!(chains[0].split_dependencies, vec![past_chain_hash]);
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_duplicated_handle() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        let _va_2 = input_shared_handle(&mut logs, va_1, tx2);
        let _vb_2 = op1(va_1, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 2);
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_duplicated_trivial_encrypt() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_shared_handle(&mut logs, va_1, tx2);
        let _vb_2 = op2(vb_1, va_2, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_dep_with_bad_order() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let _va_1 = op1(vb_1, &mut logs, tx2);
        let last = logs.pop().unwrap();
        logs.insert(0, last);
        assert!(logs[0].transaction_hash == Some(tx2));
        let chains = dependence_chains(&mut logs, &cache, true).await;
        // answer is the same as with good order
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(chains.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_non_allowed_handle() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        logs[1].is_allowed = false;
        let va_2 = input_handle(&mut logs, tx2);
        let _vb_2 = op1(va_2, &mut logs, tx2);
        logs[3].is_allowed = false;
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 2);
        assert_eq!(cache.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_dependence_chains_auction() {
        let cache = new_cache();
        let mut logs = vec![];
        let mut past_handles = vec![];
        let shared_handle = new_handle();
        for tx_id in 0..1 {
            for chain in 1..=6 {
                let tx_hash =
                    TransactionHash::with_last_byte(chain * 10 + tx_id);
                if tx_id == 0 {
                    let past_chain = past_chain(chain);
                    let past_chain_hash = past_chain.hash;
                    cache.write().await.put(
                        Handle::with_last_byte(100 + chain),
                        past_chain_hash,
                    );
                    past_handles.push((
                        Handle::with_last_byte(100 + chain),
                        input_handle(&mut logs, tx_hash),
                    ));
                }
                let (v0_a, v0_b) = past_handles[chain as usize - 1];
                let v0 = input_handle(&mut logs, tx_hash);
                let v0_bis =
                    input_shared_handle(&mut logs, shared_handle, tx_hash);
                let v0 = op2(v0, v0_bis, &mut logs, tx_hash);
                let v1 = op2(v0_a, v0, &mut logs, tx_hash);
                let v2 = op2(v0_b, v0_a, &mut logs, tx_hash);
                let v3 = op2(v1, v2, &mut logs, tx_hash);
                // let v4 = op2(v3, shared_handle, &mut logs, tx_hash);
                past_handles[chain as usize - 1] = (v2, v3);
            }
        }
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 6);
        assert!(chains.iter().all(|c| c.before_size == 0));
        assert!(logs.iter().all(|log| log.tx_depth_size == 0));
    }

    #[tokio::test]
    async fn test_dependence_chains_2_independent_components() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);

        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let _vb_2 = op1(va_2, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 2);
        assert!(logs[0..2].iter().all(|log| log.dependence_chain == tx1));
        assert!(logs[2..4].iter().all(|log| log.dependence_chain == tx2));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_same_block_join_component() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);
        let tx3 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let vb_2 = op1(va_2, &mut logs, tx2);
        let _vc_1 = op2(vb_1, vb_2, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert!(logs[0..5].iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    async fn test_dependence_chains_same_block_join_component_one_past() {
        let cache = new_cache();
        let past_chain = past_chain(0);
        let past_chain_hash = past_chain.hash;
        cache
            .write()
            .await
            .put(Handle::with_last_byte(0), past_chain_hash);
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let tx3 = TransactionHash::with_last_byte(3);
        let vb_1 = op1(past_chain_hash, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let vb_2 = op1(va_2, &mut logs, tx2);
        let _vc_1 = op2(vb_1, vb_2, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert_eq!(chains[0].split_dependencies, vec![past_chain_hash]);
        assert!(logs[0..4].iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 4);
    }

    #[tokio::test]
    async fn test_dependence_chains_same_block_join_component_two_past() {
        let cache = new_cache();
        let past_chain1 = past_chain(100);
        let past_chain_hash1 = past_chain1.hash;
        let past_chain2 = past_chain(101);
        let past_chain_hash2 = past_chain2.hash;
        let past_handle1 = new_handle();
        let past_handle2 = new_handle();
        cache.write().await.put(past_handle1, past_chain_hash1);
        cache.write().await.put(past_handle2, past_chain_hash2);
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(2);
        let tx2 = TransactionHash::with_last_byte(3);
        let tx3 = TransactionHash::with_last_byte(4);
        let vb_1 = op1(past_handle1, &mut logs, tx1);
        let vb_2 = op1(past_handle2, &mut logs, tx2);
        let _vc_1 = op2(vb_1, vb_2, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert_eq!(
            chains[0].split_dependencies,
            vec![past_chain_hash1, past_chain_hash2]
        );
        assert!(logs[0..3].iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 5);
    }

    #[tokio::test]
    async fn test_past_chain_fork() {
        let cache = new_cache();
        let past_chain1 = past_chain(100);
        let past_chain_hash1 = past_chain1.hash;
        let past_handle1 = new_handle();
        cache.write().await.put(past_handle1, past_chain_hash1);
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(2);
        let tx2 = TransactionHash::with_last_byte(3);
        let _h1 = op1(past_handle1, &mut logs, tx1);
        let _h2 = op1(past_handle1, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 2);
        assert!(logs[0].dependence_chain == tx1);
        assert!(logs[1].dependence_chain == tx2);
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    async fn test_current_block_fork() {
        let cache = new_cache();
        let past_handle1 = new_handle();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(2);
        let tx2 = TransactionHash::with_last_byte(3);
        let tx3 = TransactionHash::with_last_byte(4);
        let h1 = op1(past_handle1, &mut logs, tx1);
        let _h2 = op1(h1, &mut logs, tx2);
        let _h3 = op1(h1, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache, true).await;
        assert_eq!(chains.len(), 1);
        assert!(logs[0].dependence_chain == tx1);
        assert!(logs[1].dependence_chain == tx1);
        assert!(logs[2].dependence_chain == tx1);
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    async fn test_dependence_chains_empty_logs() {
        let cache = new_cache();
        let mut logs: Vec<LogTfhe> = vec![];

        let chains = dependence_chains(&mut logs, &cache, true).await;

        assert!(chains.is_empty());
        assert_eq!(cache.read().await.len(), 0);
    }

    // Known past handle with across_blocks=false should not extend a past chain.
    // This verifies that cross-block dependency tracking is disabled when the flag is off.
    #[tokio::test]
    async fn test_dependence_chains_across_blocks_false() {
        let cache = new_cache();
        let past_handle = new_handle();
        let past_chain_hash = past_chain(0).hash;
        cache.write().await.put(past_handle, past_chain_hash);

        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let _v = op1(past_handle, &mut logs, tx1);

        let chains = dependence_chains(&mut logs, &cache, false).await;

        assert_eq!(chains.len(), 1);
        // Chain is local (tx1), not the past chain
        assert_eq!(chains[0].hash, tx1);
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        // Cache not updated when across_blocks is false
        assert_eq!(cache.read().await.len(), 1);
    }

    // Component mode: 2 past chains feed into 1 tx, producing one local component.
    #[tokio::test]
    async fn test_dependence_chains_component_two_past_chains() {
        let cache = new_cache();
        let past_handle1 = new_handle();
        let past_handle2 = new_handle();
        let past_chain_hash1 = past_chain(100).hash;
        let past_chain_hash2 = past_chain(101).hash;
        cache.write().await.put(past_handle1, past_chain_hash1);
        cache.write().await.put(past_handle2, past_chain_hash2);

        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(2);
        let _v = op2(past_handle1, past_handle2, &mut logs, tx1);

        let chains = dependence_chains(&mut logs, &cache, true).await;

        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert_eq!(cache.read().await.len(), 3);
    }
}
