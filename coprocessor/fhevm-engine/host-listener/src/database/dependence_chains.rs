use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use tracing::{debug, error};

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
    output_tx: HashSet<TransactionHash>,
    linear_chain: TransactionHash,
    size: usize,
}

impl Transaction {
    fn new(tx_hash: TransactionHash) -> Self {
        Self {
            tx_hash,
            input_handle: Vec::with_capacity(5),
            output_handle: Vec::with_capacity(5),
            allowed_handle: Vec::with_capacity(5),
            input_tx: HashSet::with_capacity(3),
            output_tx: HashSet::with_capacity(3),
            linear_chain: tx_hash, //  before coallescing linear tx chains
            size: 1,
        }
    }
}

const AVG_LOGS_PER_TX: usize = 8;
fn scan_transactions(logs: &[LogTfhe]) -> Vec<Transaction> {
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
    ordered_txs_hash
        .iter()
        .filter_map(|tx_hash| txs.remove(tx_hash))
        .collect()
}

fn tx_of_handle(
    ordered_txs: &[Transaction],
) -> (
    HashMap<Handle, TransactionHash>,
    HashMap<Handle, HashSet<TransactionHash>>,
) {
    // handle to tx maps
    let mut handle_creator = HashMap::new(); // no intermediate value
    let mut handle_consumer = HashMap::new();
    for tx in ordered_txs {
        for handle in &tx.allowed_handle {
            handle_creator.insert(*handle, tx.tx_hash);
        }
    }
    for tx in ordered_txs {
        for handle in &tx.input_handle {
            if tx.output_handle.contains(handle) {
                // self dependency, ignore
                continue;
            }
            if !handle_creator.contains_key(handle) {
                // non allowed handle, could be from past chain
                continue;
            }
            match handle_consumer.entry(*handle) {
                Entry::Vacant(e) => {
                    let mut set = HashSet::new();
                    set.insert(tx.tx_hash);
                    e.insert(set);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(tx.tx_hash);
                }
            }
        }
    }
    (handle_creator, handle_consumer)
}

async fn fill_tx_dependence_maps(
    txs: &mut [Transaction],
    past_chains: &ChainCache,
) {
    // handle to tx maps
    let (handle_creator, handle_consumer) = tx_of_handle(txs);
    // txs relations
    for tx in txs {
        // this tx depends on dep_tx
        for input_handle in &tx.input_handle {
            if tx.output_handle.contains(input_handle) {
                // self dependency, ignore
                continue;
            }
            if let Some(dep_tx) = handle_creator.get(input_handle) {
                // intra block
                tx.input_tx.insert(*dep_tx);
            } else if let Some(dep_tx_hash) =
                past_chains.write().await.get(input_handle)
            {
                // extra block, this is directly a chain hash
                tx.input_tx.insert(*dep_tx_hash);
            }
        }
        // this tx is used by consumer_tx
        for output_handle in &tx.output_handle {
            let Some(consumer_txs) = handle_consumer.get(output_handle) else {
                continue;
            };
            for dep_tx in consumer_txs {
                if *dep_tx == tx.tx_hash {
                    // self dependency, ignore
                    continue;
                }
                tx.output_tx.insert(*dep_tx);
            }
        }
    }
}

fn topological_order(ordered_txs: &mut Vec<Transaction>) {
    let mut seen_tx: HashSet<TransactionHash> =
        HashSet::with_capacity(ordered_txs.len());
    let mut is_already_sorted = true;
    for tx in ordered_txs.iter() {
        for input_tx in &tx.input_tx {
            if !seen_tx.contains(input_tx) {
                is_already_sorted = false;
                error!("Out of order transaction detected: tx {:?} depends on tx {:?} which is later in the block", tx.tx_hash, input_tx);
                break;
            }
        }
        seen_tx.insert(tx.tx_hash);
    }
    if is_already_sorted {
        return;
    }
    let mut txs = ordered_txs
        .clone()
        .iter()
        .map(|tx| (tx.tx_hash, tx.clone()))
        .collect::<HashMap<_, _>>();
    let mut done_tx = HashSet::with_capacity(ordered_txs.len());
    let mut stack = Vec::new();
    let mut reordered = Vec::with_capacity(ordered_txs.len());
    for tx in ordered_txs.iter() {
        stack.push(tx.tx_hash);
        while let Some(tx_hash) = stack.pop() {
            if done_tx.contains(&tx_hash) {
                continue;
            }
            let Some(tx) = txs.get(&tx_hash) else {
                // previous block tx, already seen
                reordered.push(tx_hash);
                done_tx.insert(tx_hash);
                continue;
            };
            let mut unseen = vec![];
            for input_tx in &tx.input_tx {
                let is_other_block = !txs.contains_key(input_tx);
                if is_other_block {
                    continue;
                }
                if !done_tx.contains(input_tx) {
                    unseen.push(*input_tx);
                }
            }
            if unseen.is_empty() {
                reordered.push(tx_hash);
                done_tx.insert(tx_hash);
            } else {
                stack.push(tx_hash);
                stack.extend(unseen);
            }
        }
    }
    ordered_txs.clear();
    debug!("Reordered txs: {:?}", reordered);
    for tx_hash in reordered.iter() {
        let Some(tx) = txs.remove(tx_hash) else {
            continue;
        };
        ordered_txs.push(tx);
    }
}

fn grouping_to_chains(ordered_txs: &mut [Transaction]) -> OrderedChains {
    let mut used_tx: HashMap<TransactionHash, &Transaction> =
        HashMap::with_capacity(ordered_txs.len());
    let mut chains: HashMap<ChainHash, Chain> =
        HashMap::with_capacity(ordered_txs.len());
    let mut ordered_chains_hash = Vec::with_capacity(ordered_txs.len());
    for tx in ordered_txs.iter_mut() {
        let mut dependencies = Vec::with_capacity(tx.input_tx.len());
        for dep_hash in &tx.input_tx {
            let linear_chain = used_tx
                .get(dep_hash)
                .map(|tx| tx.linear_chain)
                .unwrap_or(*dep_hash); // if not in used_tx, it is a past chain
            dependencies.push(linear_chain);
        }
        let is_linear = tx.input_tx.len() == 1 && tx.output_tx.len() <= 1;
        if is_linear {
            tx.linear_chain = dependencies[0];
            match chains.entry(tx.linear_chain) {
                // extend the existing chain from same block
                Entry::Occupied(mut e) => {
                    let c = e.get_mut();
                    c.size += tx.size;
                    c.allowed_handle.extend(tx.allowed_handle.iter());
                }
                // extend the existing chain from past block, dummy values, just for a timestamp update
                Entry::Vacant(e) => {
                    let new_chain = Chain {
                        hash: tx.linear_chain,
                        size: 0,
                        before_size: 0,
                        dependencies: vec![],
                        allowed_handle: tx.allowed_handle.clone(), // needed to publish in cache
                    };
                    ordered_chains_hash.push(new_chain.hash);
                    e.insert(new_chain);
                }
            }
        } else {
            let mut before_size = 0;
            for dep in &dependencies {
                before_size = before_size.max(
                    chains
                        .get(dep)
                        .map(|c| c.size + c.before_size)
                        .unwrap_or(0),
                );
            }
            debug!("Creating new chain for tx {:?} with dependencies {:?}, before_size {}", tx, dependencies, before_size);
            let new_chain = Chain {
                hash: tx.tx_hash,
                size: tx.size,
                before_size,
                dependencies,
                allowed_handle: tx.allowed_handle.clone(),
            };
            ordered_chains_hash.push(new_chain.hash);
            chains.insert(new_chain.hash, new_chain);
        }
        if !tx.output_tx.is_empty() {
            used_tx.insert(tx.tx_hash, tx);
        }
    }
    ordered_chains_hash
        .iter()
        .filter_map(|hash| chains.remove(hash))
        .collect()
}

pub async fn dependence_chains(
    logs: &mut [LogTfhe],
    past_chains: &ChainCache,
) -> OrderedChains {
    let mut ordered_txs = scan_transactions(logs);
    fill_tx_dependence_maps(ordered_txs.as_mut_slice(), past_chains).await;
    debug!("Transactions: {:?}", ordered_txs);
    topological_order(&mut ordered_txs);
    let chains = grouping_to_chains(&mut ordered_txs);
    // propagate to logs
    let txs = ordered_txs
        .iter()
        .map(|tx| (tx.tx_hash, tx))
        .collect::<HashMap<_, _>>();
    for log in logs.iter_mut() {
        let tx_hash = log.transaction_hash.unwrap_or_default();
        if let Some(tx) = txs.get(&tx_hash) {
            log.dependence_chain = tx.linear_chain;
        } else {
            // past chain
            log.dependence_chain = tx_hash;
        }
    }
    // propagate to cache
    for chain in &chains {
        for handle in &chain.allowed_handle {
            past_chains.write().await.put(*handle, chain.hash);
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
        logs.push(LogTfhe {
            event: tfhe_event(e),
            is_allowed,
            block_number: 0,
            block_timestamp: sqlx::types::time::PrimitiveDateTime::MIN,
            transaction_hash: Some(tx),
            dependence_chain: TransactionHash::ZERO,
        })
    }

    fn new_handle() -> Handle {
        static HANDLE_COUNTER: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(1);
        let id = HANDLE_COUNTER
            .fetch_add(10000, std::sync::atomic::Ordering::SeqCst);
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

    fn allowed_input_handle(
        logs: &mut Vec<LogTfhe>,
        tx: TransactionHash,
    ) -> Handle {
        let result = new_handle();
        push_event(
            E::TrivialEncrypt(C::TrivialEncrypt {
                caller: caller(),
                pt: ClearConst::from_be_slice(&[0]),
                toType: 0,
                result,
            }),
            logs,
            true,
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

    #[tokio::test]
    async fn test_dependence_chains_1_local_chain() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let v0 = input_handle(&mut logs, tx1);
        let _v1 = op1(v0, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);

        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let _vb_2 = op1(va_2, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 2);
        assert!(logs[0..2].iter().all(|log| log.dependence_chain == tx1));
        assert!(logs[2..4].iter().all(|log| log.dependence_chain == tx2));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_dependence_chains_2_local_chain_bad_tx_order() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);

        let va_1 = allowed_input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        let _vb_2 = op1(va_1, &mut logs, tx2);

        let line = logs.pop().unwrap();
        logs.insert(0, line);

        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 3);
        assert!(logs_contain("Out of order"));
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain_mixed() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);
        let tx3 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let vb_2 = op1(va_2, &mut logs, tx2);
        let _vc_1 = op2(vb_1, vb_2, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert!(logs[0..2].iter().all(|log| log.dependence_chain == tx1));
        assert!(logs[2..4].iter().all(|log| log.dependence_chain == tx2));
        assert!(logs[5..].iter().all(|log| log.dependence_chain == tx3));
        assert_eq!(chains.len(), 3);
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_dependence_chains_2_local_chain_mixed_bis() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);
        let tx3 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let vb_2 = op1(va_2, &mut logs, tx2);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let _vc_1 = op2(vb_1, vb_2, &mut logs, tx3);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 3);
        assert_eq!(logs[0].dependence_chain, tx1);
        assert_eq!(logs[1].dependence_chain, tx2);
        assert_eq!(logs[2].dependence_chain, tx2);
        assert_eq!(logs[3].dependence_chain, tx1);
        assert_eq!(logs[4].dependence_chain, tx3);
        assert_eq!(cache.read().await.len(), 3);
    }

    fn past_chain(last_byte: u8) -> Chain {
        Chain {
            hash: TransactionHash::with_last_byte(last_byte),
            dependencies: vec![],
            size: 1,
            before_size: 0,
            allowed_handle: vec![],
        }
    }

    #[tokio::test]
    async fn test_dependence_chains_1_known_past_handle() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let past_handle = new_handle();
        let past_chain = past_chain(0);
        let past_chain_hash = past_chain.hash;
        cache.write().await.put(past_handle, past_chain_hash);
        let tx1 = TransactionHash::with_last_byte(1);
        let _va_1 = op1(past_handle, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain.hash == past_chain_hash));
        assert!(logs
            .iter()
            .all(|log| log.dependence_chain == past_chain_hash));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_1_unknown_past_handle() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let past_handle = new_handle();
        let tx1 = TransactionHash::with_last_byte(1);
        let _va_1 = op1(past_handle, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain.hash == tx1));
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_1_local_and_known_past_handle() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let past_handle = new_handle();
        let past_chain = past_chain(0);
        let past_chain_hash = past_chain.hash;
        cache.write().await.put(past_handle, past_chain_hash);
        let tx1 = TransactionHash::with_last_byte(1);
        let mut logs = vec![];
        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op2(past_handle, va_1, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain.hash == past_chain_hash));
        assert!(logs
            .iter()
            .all(|log| log.dependence_chain == past_chain_hash));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_duplicated_handle() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        let _va_2 = input_shared_handle(&mut logs, va_1, tx2);
        let _vb_2 = op1(va_1, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 2);
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_duplicated_trivial_encrypt() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_shared_handle(&mut logs, va_1, tx2);
        let vb_2 = op2(vb_1, va_2, &mut logs, tx2);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_non_allowed_handle() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(1);
        let tx2 = TransactionHash::with_last_byte(2);
        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        logs[1].is_allowed = false;
        let va_2 = input_handle(&mut logs, tx2);
        let _vb_2 = op1(va_2, &mut logs, tx2);
        logs[3].is_allowed = false;
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 2);
        assert_eq!(cache.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_dependence_chains_auction() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
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
                        Handle::with_last_byte(100 + chain as u8),
                        past_chain_hash,
                    );
                    past_handles.push((
                        Handle::with_last_byte(100 + chain as u8),
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
        eprintln!("Logs: {:?}", logs);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 6);
        // assert_eq!(cache.read().await.len(), 66);
    }
}
