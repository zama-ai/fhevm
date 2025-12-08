use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

use tracing::{error, warn};

use crate::database::tfhe_event_propagate::{
    tfhe_inputs_handle, tfhe_result_handle,
};
use crate::database::tfhe_event_propagate::{
    ChainCache, ChainDependency, Chains, Handle, LogTfhe, TransactionHash,
};

type HandleToTransaction = HashMap<Handle, (usize, TransactionHash)>;
type Transactions = VecDeque<TransactionHash>;
type TransactionToChain = HashMap<TransactionHash, ChainDependency>;

fn scan_transactions(logs: &[LogTfhe]) -> (HandleToTransaction, Transactions) {
    let nb_logs = logs.len();
    let mut prev_tx = None;
    let mut seen_txs = HashSet::with_capacity(nb_logs);
    let mut txs = VecDeque::with_capacity(nb_logs);
    let mut handle_to_tx = HashMap::with_capacity(nb_logs);
    for (age, log) in logs.iter().enumerate() {
        let tx = log.transaction_hash.unwrap_or_default();
        if prev_tx != Some(tx) && !seen_txs.contains(&tx) {
            prev_tx = Some(tx);
            seen_txs.insert(tx);
            txs.push_front(tx);
        }
        if let Some(handle) = tfhe_result_handle(&log.event) {
            // eprintln!("Mapping handle {:?} to tx {:?}", handle, tx);
            handle_to_tx.insert(handle, (age, tx));
        }
    }
    (handle_to_tx, txs)
}

async fn scan_transactions_dependencies(
    logs: &[LogTfhe],
    handle_to_tx: HandleToTransaction,
    nb_txs: usize,
    past_chains: &ChainCache,
) -> (
    HashMap<TransactionHash, (usize, TransactionHash)>,
    TransactionToChain,
) {
    // tx to its direct dependences or chain
    let mut tx_direct_tx_dependency: HashMap<
        TransactionHash,
        (usize, TransactionHash),
    > = HashMap::with_capacity(nb_txs);
    let mut tx_chain_dependency = HashMap::with_capacity(nb_txs);
    for log in logs {
        // eprintln!("Scanning dependencies for log {:?}", log.event.data);
        let tx = log.transaction_hash.unwrap_or_default();
        let log_inputs = tfhe_inputs_handle(&log.event);
        for input in log_inputs {
            // eprintln!("\tScanning input {:?}", input);
            let input_tx = handle_to_tx.get(&input);
            if let Some((_, dep_tx)) = input_tx {
                if &tx == dep_tx {
                    // eprintln!("Ignore local");
                    continue;
                }
            }
            let same_block_dep = tx_direct_tx_dependency.entry(tx);
            // eprintln!(
            //     "\tSame block dep for tx {:?}: {:?}",
            //     tx, same_block_dep
            // );
            let already_one = matches!(same_block_dep, Entry::Occupied(_));
            if let Some((age, dep_tx)) = input_tx {
                // block local, we keep the most recent tx only, assuming correct order
                let should_insert = match same_block_dep {
                    Entry::Vacant(_) => true,
                    Entry::Occupied(ref e) => *age > e.get().0,
                };
                if should_insert {
                    eprintln!(
                        "Tx {:?} depends on tx {:?} (age {})",
                        tx, dep_tx, age
                    );
                    if &tx != dep_tx {
                        same_block_dep.insert_entry((*age, *dep_tx));
                    }
                }
                continue;
            }
            if already_one {
                // this tx already has a local dependency, skip checking past ones
                continue;
            }
            let pre_block_dep = tx_chain_dependency
                .entry(log.transaction_hash.unwrap_or_default());
            if let Entry::Occupied(_) = pre_block_dep {
                // this tx already has a chain dependency, skip checking past ones
                continue;
            }
            if let Some(&dep_tx) = past_chains.write().await.get(&input) {
                // from a previous block
                pre_block_dep.insert_entry(dep_tx);
            }
            // no dependency or only unknown ones
        }
    }
    (tx_direct_tx_dependency, tx_chain_dependency)
}

fn assign_tx_a_chain_dependency(
    mut txs: Transactions,
    tx_direct_tx_dependency: &HashMap<
        TransactionHash,
        (usize, TransactionHash),
    >,
    tx_chain_dependency: &HashMap<TransactionHash, ChainDependency>,
) -> (TransactionToChain, Chains) {
    let nb_txs = txs.len();
    let mut txn_to_chain_dep = HashMap::with_capacity(nb_txs);
    let mut chains = HashSet::with_capacity(nb_txs);
    // tx to its chain dependency
    while let Some(tx) = txs.pop_back() {
        let chain_dep = txn_to_chain_dep.entry(tx);
        if let Entry::Occupied(_) = chain_dep {
            // already done
            continue;
        }
        // in block dependency we propagate the chain
        let chain = if let Some(direct_deps) = tx_direct_tx_dependency.get(&tx)
        {
            let (_age, tx_dep) = direct_deps;
            assert!(*tx_dep != tx);
            let Some(chain) = txn_to_chain_dep.get(tx_dep) else {
                warn!(?tx, "Out of order transactions");
                // only happens if logs are out of order
                txs.push_back(tx);
                // let's do its dependency first
                txs.push_back(*tx_dep);
                continue;
            };
            *chain
        } else if let Some(chain) = tx_chain_dependency.get(&tx) {
            *chain
        } else {
            // no dependency or unknown ones
            // createa new chain
            tx
        };
        // eprintln!("Assign tx {:?} to chain {:?}", tx, chain);
        txn_to_chain_dep.insert(tx, chain);
        chains.insert(chain);
    }
    (txn_to_chain_dep, chains)
}

pub async fn assign_logs_a_chain_dependency(
    logs: &mut [LogTfhe],
    txn_to_chain_dep: &TransactionToChain,
    past_chains: &ChainCache,
) {
    let mut past_chains_write = past_chains.write().await;
    for log in logs {
        let tx = log.transaction_hash.unwrap_or_default();
        let chain_dep = txn_to_chain_dep.get(&tx);
        if chain_dep.is_none() {
            error!(?tx, "No chain dependency found for transaction");
        }
        let chain_dep = *chain_dep.unwrap_or(&tx);
        log.dependence_chain = chain_dep;
        if log.is_allowed == false {
            // cannot be reused in future blocks
            continue;
        }
        // update past chains cache for next block
        if let Some(handle) = tfhe_result_handle(&log.event) {
            past_chains_write.put(handle, chain_dep);
        }
    }
}

pub async fn dependence_chains(
    logs: &mut [LogTfhe],
    past_chains: &ChainCache,
) -> Chains {
    // handle to transaction
    let (handle_to_tx, txs) = scan_transactions(logs);

    let nb_txs = txs.len();
    let (tx_direct_tx_dependency, tx_chain_dependency) =
        scan_transactions_dependencies(logs, handle_to_tx, nb_txs, past_chains)
            .await;

    let (txn_to_chain_dep, chains) = assign_tx_a_chain_dependency(
        txs,
        &tx_direct_tx_dependency,
        &tx_chain_dependency,
    );

    assign_logs_a_chain_dependency(logs, &txn_to_chain_dep, past_chains).await;
    chains
}

#[cfg(test)]
mod tests {
    use alloy::primitives::FixedBytes;
    use alloy_primitives::Address;

    use crate::contracts::TfheContract as C;
    use crate::contracts::TfheContract::TfheContractEvents as E;
    use crate::database::dependence_chains::dependence_chains;
    use crate::database::tfhe_event_propagate::{ChainCache, LogTfhe};
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
        let id =
            HANDLE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Handle::with_last_byte(id as u8)
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
        assert!(logs[5..].iter().all(|log| log.dependence_chain == tx2));
        assert_eq!(chains.len(), 2);
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain_mixed_bis() {
        // check that the last tx dependency is kept
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
        assert_eq!(chains.len(), 2);
        assert_eq!(logs[0].dependence_chain, tx1);
        assert_eq!(logs[1].dependence_chain, tx2);
        assert_eq!(logs[2].dependence_chain, tx2);
        assert_eq!(logs[3].dependence_chain, tx1);
        assert_eq!(logs[4].dependence_chain, tx1);
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    async fn test_dependence_chains_1_known_past_handle() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let mut logs = vec![];
        let past_handle = new_handle();
        let past_tx = TransactionHash::with_last_byte(0);
        cache.write().await.put(past_handle, past_tx);
        let tx1 = TransactionHash::with_last_byte(1);
        let _va_1 = op1(past_handle, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain == &past_tx));
        assert!(logs.iter().all(|log| log.dependence_chain == past_tx));
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
        assert!(chains.iter().all(|chain| chain == &tx1));
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        assert_eq!(cache.read().await.len(), 1);
    }

    #[tokio::test]
    async fn test_dependence_chains_1_local_and_known_past_handle() {
        let cache = ChainCache::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(100).unwrap(),
        ));
        let past_handle = new_handle();
        let past_tx = TransactionHash::with_last_byte(0);
        cache.write().await.put(past_handle, past_tx);
        let tx1 = TransactionHash::with_last_byte(1);
        let mut logs = vec![];
        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op2(past_handle, va_1, &mut logs, tx1);
        let chains = dependence_chains(&mut logs, &cache).await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain == &past_tx));
        assert!(logs.iter().all(|log| log.dependence_chain == past_tx));
        assert_eq!(cache.read().await.len(), 2);
    }
}
