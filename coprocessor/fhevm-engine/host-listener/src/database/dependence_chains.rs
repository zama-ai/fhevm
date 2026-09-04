use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use tracing::{debug, error, info, warn};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

use crate::database::tfhe_event_propagate::{
    tfhe_inputs_handle, tfhe_result_handle, ChainHash,
};
use crate::database::tfhe_event_propagate::{
    Chain, ChainCache, ConsumedBoundaryGuard, Handle, LogTfhe, OrderedChains,
    SealedChainGuard, TransactionHash,
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
    size: u64,
    depth_size: u64,
    /// A cross-block input of this tx was already consumed by an earlier
    /// batch: extending the producer chain would serialize an independent
    /// branch onto it, so this tx must form its own gated chain.
    forked_cross_block: bool,
    /// (producer chain, boundary handle) for every CROSS-BLOCK input, kept so
    /// the gate can be armed on the handle this tx actually waits for rather
    /// than on retirement of the producer's whole chain.
    outer_boundary: Vec<(ChainHash, Handle)>,
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
            linear_chain: tx_hash, //  before coalescing linear tx chains
            size: 0,
            depth_size: 0,
            forked_cross_block: false,
            outer_boundary: Vec::new(),
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
    used_txs_chains: &mut HashMap<TransactionHash, HashSet<TransactionHash>>,
    past_chains: &ChainCache,
    consumed_boundaries: &ConsumedBoundaryGuard,
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
                // mark as consumer; record the boundary consumption so a
                // later-batch consumer of the same handle forks instead of
                // extending (in-batch double consumption is already handled
                // by the sibling logic below).
                consumed_boundaries
                    .write()
                    .await
                    .put(*input_handle, *tx_hash);
                tx.input_tx.insert(*dep_tx);
                used_txs_chains
                    .entry(*dep_tx)
                    .and_modify(|v| {
                        v.insert(*tx_hash);
                    })
                    .or_insert({
                        let mut h = HashSet::new();
                        h.insert(*tx_hash);
                        h
                    });
                // memorize as producer
                producer_tx.push(*dep_tx);
            } else if let Some(dep_tx_hash) =
                past_chains.write().await.get(input_handle)
            {
                // extra block, this is directly a chain hash
                // A boundary handle feeds exactly one linear continuation:
                // its first cross-batch consumer may extend the producer
                // chain, every DIFFERENT later consumer is a fork and forms
                // its own gated chain.
                if let Some(previous_consumer) = consumed_boundaries
                    .write()
                    .await
                    .put(*input_handle, *tx_hash)
                {
                    if previous_consumer != *tx_hash {
                        tx.forked_cross_block = true;
                    }
                }
                tx.input_tx.insert(*dep_tx_hash);
                tx.outer_boundary.push((*dep_tx_hash, *input_handle));
                used_txs_chains
                    .entry(*dep_tx_hash)
                    .and_modify(|v| {
                        v.insert(tx.tx_hash);
                    })
                    .or_insert({
                        let mut h = HashSet::new();
                        h.insert(tx.tx_hash);
                        h
                    });
            }
        }
        // update allowed handle for next txs
        for allowed_handle in &tx.allowed_handle {
            allowed_handle_tx.entry(*allowed_handle).or_insert(*tx_hash);
        }
        // propagate memorized producers
        let mut depth_size = 0;
        for dep_tx in &producer_tx {
            txs.entry(*dep_tx).and_modify(|dep_tx| {
                dep_tx.output_tx.insert(*tx_hash);
                depth_size = depth_size.max(dep_tx.depth_size + dep_tx.size);
            });
        }
        txs.entry(*tx_hash).and_modify(|dep_tx| {
            dep_tx.depth_size = depth_size;
        });
    }
}

async fn grouping_to_chains_connex(
    ordered_txs: &mut [Transaction],
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
            info!(
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
    // list components past chains dependencies
    let mut past_chains_deps: HashMap<usize, HashSet<TransactionHash>> =
        HashMap::new();
    for (key, tx) in ordered_txs.iter_mut().enumerate() {
        for dep_hash in &tx.input_tx {
            if !tx_index.contains_key(dep_hash) {
                // from previous block
                let component = txs_component[key];
                match past_chains_deps.entry(component) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().insert(*dep_hash);
                    }
                    Entry::Vacant(e) => {
                        let set = HashSet::from([*dep_hash]);
                        e.insert(set);
                    }
                }
            }
        }
    }
    let mut ordered_chains_hash = Vec::with_capacity(ordered_txs.len());
    let mut chains: HashMap<ChainHash, Chain> =
        HashMap::with_capacity(ordered_txs.len());
    // create chain from component or merge to 1 past chain
    for (index, tx) in ordered_txs.iter_mut().enumerate() {
        let component = txs_component[index];
        let mut component_hash = tx_hash[component];
        let mut new_chain = true;
        if let Some(chains) = past_chains_deps.get(&component) {
            if chains.len() == 1 {
                info!(
                    " Merging component {:?} into past chains {:?} ",
                    component, chains
                );
                component_hash =
                    chains.iter().next().cloned().unwrap_or(component_hash);
                new_chain = false;
            };
        };
        tx.linear_chain = component_hash;
        match chains.entry(component_hash) {
            Entry::Occupied(mut e) => {
                let c = e.get_mut();
                c.size += tx.size;
                c.allowed_handle.extend(tx.allowed_handle.iter());
            }
            Entry::Vacant(e) => {
                ordered_chains_hash.push(tx.linear_chain);
                let new_chain = Chain {
                    hash: tx.linear_chain,
                    size: tx.size,
                    before_size: 0,
                    dependencies: vec![],
                    split_dependencies: vec![],
                    outer_boundary_handles: vec![],
                    dependents: vec![],
                    allowed_handle: tx.allowed_handle.clone(),
                    new_chain,
                };
                e.insert(new_chain);
            }
        }
    }
    ordered_chains_hash
        .iter()
        .filter_map(|hash| chains.remove(hash))
        .collect()
}

async fn grouping_to_chains_no_fork(
    ordered_txs: &mut [Transaction],
    used_txs_chains: &mut HashMap<TransactionHash, HashSet<TransactionHash>>,
    across_blocks: bool,
    sealed_chains: &SealedChainGuard,
) -> OrderedChains {
    let mut used_tx: HashMap<TransactionHash, &Transaction> =
        HashMap::with_capacity(ordered_txs.len());
    let mut chains: HashMap<ChainHash, Chain> =
        HashMap::with_capacity(ordered_txs.len());
    let mut ordered_chains_hash = Vec::with_capacity(ordered_txs.len());
    let block_tx_hashes = ordered_txs
        .iter()
        .map(|tx| tx.tx_hash)
        .collect::<HashSet<_>>();
    for tx in ordered_txs.iter_mut() {
        let mut dependencies_block = Vec::with_capacity(tx.input_tx.len());
        let mut dependencies_outer = Vec::with_capacity(tx.input_tx.len());
        let mut dependencies_seen = HashSet::with_capacity(tx.input_tx.len());
        for dep_hash in &tx.input_tx {
            // Only record dependences within the block as we don't
            // have a clean way of handling cross-block dependences
            if let Some(linear_chain) =
                used_tx.get(dep_hash).map(|tx| tx.linear_chain)
            {
                if !dependencies_seen.contains(&linear_chain) {
                    if block_tx_hashes.contains(&linear_chain) {
                        dependencies_block.push(linear_chain);
                    } else if across_blocks {
                        dependencies_outer.push(linear_chain);
                    }
                    dependencies_seen.insert(linear_chain);
                }
            } else if across_blocks {
                // if not in used_tx, it is a past chain
                if !dependencies_seen.contains(dep_hash) {
                    dependencies_outer.push(*dep_hash);
                    dependencies_seen.insert(*dep_hash);
                }
            }
        }
        // A chain is linear if there's no joins on the current
        // transaction and if the current transaction is not a
        // descendant of a fork
        // 1. Test for joins
        let mut is_linear =
            (dependencies_block.len() + dependencies_outer.len()) == 1
                && !tx.forked_cross_block;
        // 2. Test for forks
        if is_linear {
            let unique_parent = if dependencies_block.is_empty() {
                dependencies_outer[0]
            } else {
                dependencies_block[0]
            };
            if let Some(siblings) = used_txs_chains.get_mut(&unique_parent) {
                for s in siblings.clone().iter() {
                    // If one sibling is already within a chain, this
                    // chain could be the same as another in the
                    // siblings set, so both dependences are then
                    // covered by the same chain.
                    if let Some(linear_chain) =
                        used_tx.get(s).map(|tx| tx.linear_chain)
                    {
                        siblings.remove(s);
                        siblings.insert(linear_chain);
                    }
                }
                // If there is only one descendant for the unique
                // ancestor or all descendents are in a single
                // dependence chain as a totally ordered set, then the
                // linear chain continues
                is_linear = siblings.len() == 1;
            }
            // A SEALED producer is not extended. It was sealed because a fork
            // gates on a handle it has not materialized, and every further
            // continuation is unrelated work that fork would wait behind.
            //
            // LAST, and never folded into the sibling test above: that test
            // ASSIGNS `is_linear` rather than narrowing it, so a seal checked
            // before it is silently overwritten. `peek` so consulting a seal
            // does not reorder the LRU — eviction should follow when a chain
            // was last sealed, not when it was last read.
            if is_linear
                && sealed_chains.read().await.peek(&unique_parent).is_some()
            {
                is_linear = false;
            }
        }
        if is_linear {
            tx.linear_chain = if dependencies_block.is_empty() {
                dependencies_outer[0]
            } else {
                dependencies_block[0]
            };
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
                        split_dependencies: vec![],
                        outer_boundary_handles: vec![],
                        dependents: vec![],
                        allowed_handle: tx.allowed_handle.clone(), // needed to publish in cache
                        new_chain: false,
                    };
                    ordered_chains_hash.push(new_chain.hash);
                    e.insert(new_chain);
                }
            }
        } else {
            let mut before_size = 0;
            for dep in &dependencies_block {
                before_size = before_size.max(
                    chains
                        .get(dep)
                        .map(|c| c.size + c.before_size)
                        .unwrap_or(0),
                );
            }
            debug!("Creating new chain for tx {:?} with block dependencies {:?}, outer dependencies {:?}, before_size {}",
		   tx, dependencies_block, dependencies_outer, before_size);
            let split_dependencies =
                [dependencies_block.clone(), dependencies_outer.clone()]
                    .concat();
            // Only the pairs whose producer is actually an outer dependency of
            // THIS chain: a tx can reference a cross-block handle that the
            // in-block grouping already covers.
            let outer_boundary_handles = tx
                .outer_boundary
                .iter()
                .filter(|(chain, _)| dependencies_outer.contains(chain))
                .copied()
                .collect::<Vec<_>>();
            let new_chain = Chain {
                hash: tx.tx_hash,
                size: tx.size,
                before_size,
                dependencies: dependencies_block,
                split_dependencies,
                outer_boundary_handles,
                dependents: vec![],
                allowed_handle: tx.allowed_handle.clone(),
                new_chain: true,
            };
            ordered_chains_hash.push(new_chain.hash);
            chains.insert(new_chain.hash, new_chain);
        }
        if !tx.output_tx.is_empty() {
            used_tx.insert(tx.tx_hash, tx);
        }
    }
    // compute dependents field - only limited to within a block for now
    for chain_hash in ordered_chains_hash.iter() {
        let Some(chain) = chains.get(chain_hash) else {
            continue;
        };
        if !chain.new_chain {
            continue;
        }
        for dep in chain.dependencies.clone() {
            if let Some(dep_chain) = chains.get_mut(&dep) {
                if !dep_chain.new_chain {
                    continue;
                }
                dep_chain.dependents.push(*chain_hash);
            }
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
    consumed_boundaries: &ConsumedBoundaryGuard,
    sealed_chains: &SealedChainGuard,
    connex: bool,
    across_blocks: bool,
) -> OrderedChains {
    ensure_logs_order(logs);
    let (ordered_hash, mut txs) = scan_transactions(logs);
    let mut used_txs_chains: HashMap<
        TransactionHash,
        HashSet<TransactionHash>,
    > = HashMap::with_capacity(txs.len());
    fill_tx_dependence_maps(
        &ordered_hash,
        &mut txs,
        &mut used_txs_chains,
        past_chains,
        consumed_boundaries,
    )
    .await;
    debug!("Transactions: {:?}", txs.values());
    let mut ordered_txs: Vec<_> = ordered_hash
        .iter()
        .filter_map(|tx_hash| txs.remove(tx_hash))
        .collect();
    let chains = if connex {
        grouping_to_chains_connex(&mut ordered_txs).await
    } else {
        grouping_to_chains_no_fork(
            &mut ordered_txs,
            &mut used_txs_chains,
            across_blocks,
            sealed_chains,
        )
        .await
    };
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
            operand_boundary_mask: None,
            is_executor_minted: true,
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

    fn new_guard(
    ) -> crate::database::tfhe_event_propagate::ConsumedBoundaryGuard {
        std::sync::Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(4096).unwrap(),
        )))
    }

    /// Drive one block through the REAL ingest sequence for one listener:
    /// chain grouping against that process's own caches, then the row inserts
    /// and the chain upsert, in the order `ingest.rs` uses. `dependence_chains`
    /// writes the assignment back into the logs, so the inserts must follow it.
    pub(super) async fn ingest_block(
        db: &crate::database::tfhe_event_propagate::Database,
        logs: &mut [LogTfhe],
        block_number: u64,
    ) {
        use crate::cmd::block_history::BlockSummary;
        let block_hash_early =
            TransactionHash::with_last_byte(block_number as u8);
        let now = time::OffsetDateTime::now_utc();
        let block_timestamp =
            sqlx::types::time::PrimitiveDateTime::new(now.date(), now.time());
        for log in logs.iter_mut() {
            log.block_number = block_number;
            log.block_hash = block_hash_early;
            // `push_event` leaves this at PrimitiveDateTime::MIN, which
            // Postgres rejects as out of range.
            log.block_timestamp = block_timestamp;
        }
        // Production order: the boundary mask is consensus-critical execution
        // metadata and must be derived BEFORE grouping and insertion, so the
        // test drives it the same way rather than hand-filling masks.
        crate::database::ingest::populate_operand_boundary_masks(logs)
            .expect("derive operand boundary masks");
        let chains = dependence_chains(
            logs,
            &db.dependence_chain,
            &db.consumed_boundaries,
            &db.sealed_chains,
            false,
            true,
        )
        .await;
        let block_hash = block_hash_early;
        let summary = BlockSummary {
            number: block_number,
            hash: block_hash,
            parent_hash: TransactionHash::with_last_byte(
                block_number.saturating_sub(1) as u8,
            ),
            timestamp: 1_700_000_000 + block_number,
        };
        let mut tx = db
            .new_transaction()
            .await
            .expect("begin")
            .expect("live transaction");
        for log in logs.iter() {
            db.insert_tfhe_event(&mut tx, log)
                .await
                .expect("insert event");
        }
        db.update_dependence_chain(
            &mut tx,
            chains,
            block_timestamp,
            &summary,
            &std::collections::HashSet::new(),
        )
        .await
        .expect("update chains");
        tx.commit().await.expect("commit");
    }

    /// What each listener ingests, plus the transaction they disagree about:
    /// (primary blocks, catchup blocks, split transaction).
    pub(super) type ListenerFixture = (
        Vec<(u64, Vec<LogTfhe>)>,
        Vec<(u64, Vec<LogTfhe>)>,
        TransactionHash,
    );

    /// A test-local copy of a log. `LogTfhe` is deliberately not `Clone` in
    /// production, and two listeners observing the same chain event is a test
    /// concern, so the duplication lives here rather than on the type.
    fn copy_log(log: &LogTfhe) -> LogTfhe {
        LogTfhe {
            event: log.event.clone(),
            transaction_hash: log.transaction_hash,
            is_allowed: log.is_allowed,
            block_number: log.block_number,
            block_hash: log.block_hash,
            block_timestamp: log.block_timestamp,
            tx_depth_size: log.tx_depth_size,
            dependence_chain: log.dependence_chain,
            log_index: log.log_index,
            operand_boundary_mask: log.operand_boundary_mask,
            is_executor_minted: log.is_executor_minted,
        }
    }

    fn copy_logs(logs: &[LogTfhe]) -> Vec<LogTfhe> {
        logs.iter().map(copy_log).collect()
    }

    /// The blocks each listener sees, and the transaction they disagree about.
    ///
    /// Block 1 establishes a producer, and only the PRIMARY sees it -- an
    /// hourly catchup process starts cold and rescans a recent window, so a
    /// block outside that window is simply not in its cache. Block 2 holds a
    /// transaction of two operations; the primary sees only the first of them
    /// (the missed-event case) and, knowing block 1, files it as a
    /// continuation of the producer's chain. The catchup later sees both
    /// operations but cannot resolve their cross-block producer, so it opens a
    /// chain of its own -- and `ON CONFLICT DO NOTHING` leaves the first
    /// operation where the primary put it. One transaction, two chains.
    pub(super) fn primary_and_catchup_logs() -> ListenerFixture {
        let tx1 = TransactionHash::with_last_byte(0x21);
        let split_tx = TransactionHash::with_last_byte(0x22);

        let mut block1 = vec![];
        let root = input_handle(&mut block1, tx1);
        let produced = op1(root, &mut block1, tx1);

        let mut full_block2 = vec![];
        let first = op1(produced, &mut full_block2, split_tx);
        let _second = op1(first, &mut full_block2, split_tx);

        let mut partial_block2 = copy_logs(&full_block2);
        partial_block2.pop();

        (
            vec![(1, block1), (2, partial_block2)],
            vec![(2, full_block2)],
            split_tx,
        )
    }

    fn new_sealed() -> crate::database::tfhe_event_propagate::SealedChainGuard {
        std::sync::Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(4096).unwrap(),
        )))
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
        assert!(logs[0..2].iter().all(|log| log.dependence_chain == tx1));
        assert!(logs[2..4].iter().all(|log| log.dependence_chain == tx2));
        assert!(logs[4..].iter().all(|log| log.dependence_chain == tx3));
        assert_eq!(chains.len(), 3);
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
        assert_eq!(chains.len(), 3);
        assert_eq!(logs[0].dependence_chain, tx1);
        assert_eq!(logs[1].dependence_chain, tx2);
        assert_eq!(logs[2].dependence_chain, tx2);
        assert_eq!(logs[3].dependence_chain, tx1);
        assert_eq!(logs[4].dependence_chain, tx3);
        assert_eq!(logs[0].tx_depth_size, 0);
        assert_eq!(logs[1].tx_depth_size, 0);
        assert_eq!(logs[2].tx_depth_size, 0);
        assert_eq!(logs[3].tx_depth_size, 0);
        assert_eq!(logs[4].tx_depth_size, 2);
        assert_eq!(cache.read().await.len(), 3);
        assert_eq!(chains[0].before_size, 0);
        assert_eq!(chains[1].before_size, 0);
        assert_eq!(chains[2].before_size, 2);
        assert_eq!(chains[0].dependencies.len(), 0);
        assert_eq!(chains[1].dependencies.len(), 0);
        assert_eq!(chains[2].dependencies.len(), 2);
        assert_eq!(chains[0].dependents, vec![tx3]);
        assert_eq!(chains[1].dependents, vec![tx3]);
        assert!(chains[2].dependents.is_empty());
    }

    fn past_chain(last_byte: u8) -> Chain {
        Chain {
            hash: TransactionHash::with_last_byte(last_byte),
            dependencies: vec![],
            split_dependencies: vec![],
            outer_boundary_handles: vec![],
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain.hash == past_chain_hash));
        assert!(logs
            .iter()
            .all(|log| log.dependence_chain == past_chain_hash));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_1_unknown_past_handle() {
        let cache = new_cache();
        let mut logs = vec![];
        let past_handle = new_handle();
        let tx1 = TransactionHash::with_last_byte(1);
        let _va_1 = op1(past_handle, &mut logs, tx1);
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
        assert_eq!(chains.len(), 1);
        assert!(chains.iter().all(|chain| chain.hash == past_chain_hash));
        assert!(logs
            .iter()
            .all(|log| log.dependence_chain == past_chain_hash));
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
        assert_eq!(chains.len(), 6);
        assert!(chains.iter().all(|c| c.before_size == 0));
        assert!(logs.iter().all(|log| log.tx_depth_size == 0));
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain_connex() {
        let cache = new_cache();
        let mut logs = vec![];
        let tx1 = TransactionHash::with_last_byte(0);
        let tx2 = TransactionHash::with_last_byte(1);

        let va_1 = input_handle(&mut logs, tx1);
        let _vb_1 = op1(va_1, &mut logs, tx1);
        let va_2 = input_handle(&mut logs, tx2);
        let _vb_2 = op1(va_2, &mut logs, tx2);
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            true,
            true,
        )
        .await;
        assert_eq!(chains.len(), 2);
        assert!(logs[0..2].iter().all(|log| log.dependence_chain == tx1));
        assert!(logs[2..4].iter().all(|log| log.dependence_chain == tx2));
        assert_eq!(cache.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain_mixed_connex() {
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            true,
            true,
        )
        .await;
        assert_eq!(chains.len(), 1);
        assert!(logs[0..5].iter().all(|log| log.dependence_chain == tx3));
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain_mixed_1_past_connex() {
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            true,
            true,
        )
        .await;
        assert_eq!(chains.len(), 1);
        assert!(logs[0..4]
            .iter()
            .all(|log| log.dependence_chain == past_chain_hash));
        assert_eq!(cache.read().await.len(), 4);
    }

    #[tokio::test]
    async fn test_dependence_chains_2_local_chain_mixed_2_past_connex() {
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            true,
            true,
        )
        .await;
        assert_eq!(chains.len(), 1);
        assert!(logs[0..3].iter().all(|log| log.dependence_chain == tx3));
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
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
        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;
        assert_eq!(chains.len(), 3);
        assert!(logs[0].dependence_chain == tx1);
        assert!(logs[1].dependence_chain == tx2);
        assert!(logs[2].dependence_chain == tx3);
        assert_eq!(cache.read().await.len(), 3);
    }

    #[tokio::test]
    async fn test_dependence_chains_empty_logs() {
        let cache = new_cache();
        let mut logs: Vec<LogTfhe> = vec![];

        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            true,
        )
        .await;

        assert!(chains.is_empty());
        assert_eq!(cache.read().await.len(), 0);
    }

    // Known past handle with across_blocks=false should not extent a past chain.
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

        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            false,
            false,
        )
        .await;

        assert_eq!(chains.len(), 1);
        // Chain is local (tx1), not the past chain
        assert_eq!(chains[0].hash, tx1);
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));
        // Cache not updated when across_blocks is false
        assert_eq!(cache.read().await.len(), 1);
    }

    // Connex mode: 2 past chains feed into 1 tx, producing a single component.
    #[tokio::test]
    async fn test_dependence_chains_connex_two_past_chains_merge() {
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

        let chains = dependence_chains(
            &mut logs,
            &cache,
            &new_guard(),
            &new_sealed(),
            true,
            true,
        )
        .await;

        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, tx1);
        assert_eq!(cache.read().await.len(), 3);
    }

    /// A SEALED producer stops absorbing continuations.
    ///
    /// This is the companion to `test_cross_batch_linear_growth_extends_one_chain`:
    /// the same sequential traffic that extends a chain forever must instead
    /// start a new one once the producer has been sealed, because a fork is
    /// waiting on a handle that producer has not materialized and every extra
    /// continuation is unrelated work the fork would wait behind.
    #[tokio::test]
    async fn test_sealed_chain_is_not_extended() {
        let cache = new_cache();
        let guard = new_guard();
        let sealed = new_sealed();

        let tx1 = TransactionHash::with_last_byte(1);
        let mut logs = vec![];
        let v0 = input_handle(&mut logs, tx1);
        let v1 = op1(v0, &mut logs, tx1);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);
        let producer = chains[0].hash;

        // Unsealed, the next batch extends the producer — the baseline the
        // seal has to change.
        let tx2 = TransactionHash::with_last_byte(2);
        let mut logs = vec![];
        let v2 = op1(v1, &mut logs, tx2);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);
        assert!(!chains[0].new_chain, "unsealed producer is extended");

        // Seal it, as arming a gate on an un-materialized handle would.
        sealed.write().await.put(producer, ());

        let tx3 = TransactionHash::with_last_byte(3);
        let mut logs = vec![];
        let _v3 = op1(v2, &mut logs, tx3);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);
        assert!(
            chains[0].new_chain,
            "a sealed producer must not be extended; the continuation forms \
             its own chain"
        );
        assert_eq!(
            chains[0].split_dependencies,
            vec![producer],
            "the new chain is gated on the sealed producer"
        );
    }

    /// Sequential cross-batch growth keeps extending one chain: the guard
    /// records consumed boundaries but a chain's own tail (fresh handles
    /// every batch) never trips it.
    #[tokio::test]
    async fn test_cross_batch_linear_growth_extends_one_chain() {
        let cache = new_cache();
        let guard = new_guard();
        let sealed = new_sealed();
        let tx1 = TransactionHash::with_last_byte(1);
        let mut logs = vec![];
        let v0 = input_handle(&mut logs, tx1);
        let v1 = op1(v0, &mut logs, tx1);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);

        let mut tail = v1;
        for batch in 2..5u8 {
            let txn = TransactionHash::with_last_byte(batch);
            let mut logs = vec![];
            tail = op1(tail, &mut logs, txn);
            let chains = dependence_chains(
                &mut logs, &cache, &guard, &sealed, false, true,
            )
            .await;
            assert_eq!(chains.len(), 1, "batch {batch} extends");
            assert!(
                !chains[0].new_chain,
                "batch {batch} must extend the past chain, not fork"
            );
            assert!(
                logs.iter().all(|log| log.dependence_chain == tx1),
                "batch {batch} stays in the original chain"
            );
        }
    }

    /// A second cross-batch consumer of an already-consumed boundary handle
    /// is a fork: it must form its own chain, gated on the producer chain
    /// through split_dependencies, instead of serializing onto it.
    #[tokio::test]
    async fn test_cross_batch_fork_forms_gated_chain() {
        let cache = new_cache();
        let guard = new_guard();
        let sealed = new_sealed();
        let tx1 = TransactionHash::with_last_byte(1);
        let mut logs = vec![];
        let v0 = input_handle(&mut logs, tx1);
        let v1 = op1(v0, &mut logs, tx1);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);

        // First cross-batch consumer of v1: linear continuation.
        let tx2 = TransactionHash::with_last_byte(2);
        let mut logs = vec![];
        let _v2 = op1(v1, &mut logs, tx2);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);
        assert!(!chains[0].new_chain, "first consumer extends");
        assert!(logs.iter().all(|log| log.dependence_chain == tx1));

        // Second cross-batch consumer of the SAME boundary handle: fork.
        let tx3 = TransactionHash::with_last_byte(3);
        let mut logs = vec![];
        let _v3 = op1(v1, &mut logs, tx3);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);
        assert!(
            chains[0].new_chain,
            "second consumer must fork into its own chain"
        );
        assert_eq!(chains[0].hash, tx3);
        assert!(
            chains[0].split_dependencies.contains(&tx1),
            "forked chain is gated on the producer chain"
        );
        assert!(logs.iter().all(|log| log.dependence_chain == tx3));
    }

    /// Reproduction of the ingested linear traffic shape: one chain advanced
    /// across three lag-2 batches. The chain must keep the head
    /// transaction's hash as its DCID through every extension.
    #[tokio::test]
    async fn test_cross_batch_traffic_chain_keeps_head_identity() {
        let cache = new_cache();
        let guard = new_guard();
        let sealed = new_sealed();
        let head_tx = TransactionHash::with_last_byte(1);
        let mut logs = vec![];
        let from0 = input_handle(&mut logs, head_tx);
        let new_from = op1(from0, &mut logs, head_tx);
        let chains =
            dependence_chains(&mut logs, &cache, &guard, &sealed, false, true)
                .await;
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].hash, head_tx, "head chain is rooted at its tx");
        assert!(chains[0].new_chain);
        assert!(logs.iter().all(|log| log.dependence_chain == head_tx));

        let mut tail = new_from;
        for batch in 2..4u8 {
            let txn = TransactionHash::with_last_byte(batch);
            let mut logs = vec![];
            // Same 2-op shape as a transfer: consume tail + fresh input.
            let amount = input_handle(&mut logs, txn);
            let ge = op2(tail, amount, &mut logs, txn);
            tail = op1(ge, &mut logs, txn);
            let chains = dependence_chains(
                &mut logs, &cache, &guard, &sealed, false, true,
            )
            .await;
            assert_eq!(chains.len(), 1, "batch {batch}");
            assert_eq!(
                chains[0].hash, head_tx,
                "batch {batch}: extension keeps the head DCID"
            );
            assert!(!chains[0].new_chain, "batch {batch}");
            assert!(
                logs.iter().all(|log| log.dependence_chain == head_tx),
                "batch {batch}: computations carry the head DCID"
            );
        }
    }
}

/// Two listeners, independent caches, one database.
///
/// Production runs three host-listener processes over one database -- a
/// realtime WS listener, a poller some blocks behind, and an hourly catchup
/// pass. Chain identity is computed from PER-PROCESS state (`past_chains`,
/// `consumed_boundaries`, `sealed_chains`), so those processes do not agree,
/// and the row-level `ON CONFLICT (output_handle, transaction_id) DO NOTHING`
/// means whichever one inserts a row first fixes its chain. When one listener
/// misses an event and another catches it, a single TRANSACTION can therefore
/// end up split across two dependence chains.
///
/// That shape is assumed all over the worker -- the work window loads every
/// row of a selected transaction regardless of chain and demotes rows of
/// other chains to recompute-only producers -- but nothing demonstrated that
/// the listener side actually produces it, or that the result is workable.
/// Every fixture for it was hand-seeded. This drives the real ingest path
/// twice with divergent caches and asserts the merged database is coherent.
#[cfg(test)]
mod multi_listener_tests {
    use super::tests::{ingest_block, primary_and_catchup_logs};
    use crate::database::tfhe_event_propagate::Database;
    use fhevm_engine_common::chain_id::ChainId;
    use serial_test::serial;
    use test_harness::instance::ImportMode;

    #[tokio::test]
    #[serial(db)]
    async fn divergent_listener_caches_split_a_transaction_but_stay_coherent() {
        let instance = test_harness::instance::setup_test_db(ImportMode::None)
            .await
            .expect("test database");
        let chain_id = ChainId::try_from(42_u64).expect("chain id");
        // Two processes over ONE database. The caches are what diverge, and
        // they live on the struct, so two instances is the whole simulation.
        let primary = Database::new(&instance.db_url, chain_id, 10_000)
            .await
            .expect("primary listener");
        let catchup = Database::new(&instance.db_url, chain_id, 10_000)
            .await
            .expect("catchup listener");

        let (mut primary_blocks, mut catchup_blocks, split_tx) =
            primary_and_catchup_logs();

        // The primary ingests the block but misses one event of the second
        // transaction; the catchup pass later ingests the whole thing.
        for (number, logs) in primary_blocks.iter_mut() {
            ingest_block(&primary, logs, *number).await;
        }
        for (number, logs) in catchup_blocks.iter_mut() {
            ingest_block(&catchup, logs, *number).await;
        }

        let pool = primary.pool.read().await.clone();

        // 1. Every event is present exactly once. The catchup re-ingest must
        //    not duplicate what the primary already wrote.
        let (rows, handles): (i64, i64) = sqlx::query_as(
            "SELECT COUNT(*), COUNT(DISTINCT output_handle) FROM computations",
        )
        .fetch_one(&pool)
        .await
        .expect("count computations");
        assert_eq!(
            rows, handles,
            "a handle ingested by both listeners must exist once, not twice"
        );

        // 2. Every computation is filed under a chain that exists. A row
        //    pointing at a missing chain is unreachable work.
        let orphans: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM computations c
             WHERE c.dependence_chain_id IS NOT NULL
               AND NOT EXISTS (
                 SELECT 1 FROM dependence_chain dc
                 WHERE dc.dependence_chain_id = c.dependence_chain_id
               )",
        )
        .fetch_one(&pool)
        .await
        .expect("orphan scan");
        assert_eq!(
            orphans, 0,
            "every row must be filed under a chain that exists, or it is \
             unreachable work"
        );

        // 3. The transaction the two listeners disagreed about really is
        //    split across chains. If this ever stops holding, the fixture has
        //    stopped reproducing the production shape and the assertions
        //    below are vacuous.
        let chains_for_split: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT dependence_chain_id) FROM computations
             WHERE transaction_id = $1",
        )
        .bind(split_tx.as_slice())
        .fetch_one(&pool)
        .await
        .expect("split scan");
        assert!(
            chains_for_split >= 2,
            "divergent caches must actually split the transaction across \
             chains; got {chains_for_split}"
        );

        // 4. No chain is gated on a producer that does not exist. An
        //    over-armed gate that nothing can discharge is the one way this
        //    topology could strand work permanently -- the decrement clamps
        //    at zero, so the danger is a count that is never reached, not one
        //    that goes negative.
        let ungrounded: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM dependence_chain c
             WHERE c.dependency_count > 0
               AND NOT EXISTS (
                 SELECT 1 FROM dependence_chain p
                 WHERE c.dependence_chain_id = ANY(p.dependents)
               )",
        )
        .fetch_one(&pool)
        .await
        .expect("gate scan");
        assert_eq!(
            ungrounded, 0,
            "a chain gated with no producer listing it as a dependent can \
             never be discharged by a release"
        );

        // 5. Counts stay in range. `GREATEST(count - n, 0)` is what keeps a
        //    double discharge from driving a chain below zero, which would
        //    make it permanently unacquirable: acquisition requires
        //    `dependency_count = 0` exactly.
        let negative: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM dependence_chain WHERE dependency_count < 0",
        )
        .fetch_one(&pool)
        .await
        .expect("negative scan");
        assert_eq!(negative, 0, "a negative gate is unacquirable forever");

        // 6. A third process derives the same seal set from durable state.
        //    Seals are per-process, so the listeners disagree in memory; the
        //    periodic rebuild is what makes them converge, and it must not
        //    depend on which listener happened to ingest what.
        let fresh = Database::new(&instance.db_url, chain_id, 10_000)
            .await
            .expect("third listener");
        let derived: Vec<Vec<u8>> = sqlx::query_scalar(
            "SELECT DISTINCT p.dependence_chain_id
             FROM dependence_chain p
             JOIN dependence_chain c
               ON c.dependence_chain_id = ANY(p.dependents)
             WHERE c.dependency_count > 0 AND p.status <> 'processed'",
        )
        .fetch_all(&pool)
        .await
        .expect("derive seals");
        let sealed = fresh.sealed_chains.read().await;
        assert_eq!(
            sealed.len(),
            derived.len(),
            "a freshly started listener must derive exactly the seals the \
             durable state implies, whichever process wrote it"
        );
    }
}
