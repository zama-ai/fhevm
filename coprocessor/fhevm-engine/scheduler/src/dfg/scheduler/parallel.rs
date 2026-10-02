//! Operation readiness within a transaction, without changing materialisation.
//!
//! Coordinators are async and never occupy the blocking pool while waiting for
//! child work. Operations use that existing bounded pool and, on CUDA, the same
//! process-wide device permits as other admitted batches. Raw values are shared
//! only within a transaction and always consumed on its assigned device.
use super::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[cfg(feature = "gpu")]
type ServerKey = tfhe::CudaServerKey;
#[cfg(not(feature = "gpu"))]
type ServerKey = tfhe::ServerKey;

/// Per-partition canonical decompression memo (RFC-020). All operations in
/// the partition use the same device and keys. Arc pointers are copied under
/// the mutex, but ciphertext cloning and decompression run outside it. Insertion
/// receives a completed clone, so cache hits can safely use another stream.
#[derive(Default)]
pub(super) struct CanonicalInputs(Mutex<HashMap<Handle, Arc<SupportedFheCiphertexts>>>);
impl CanonicalInputs {
    pub(super) fn get(&self, handle: &Handle) -> Option<Arc<SupportedFheCiphertexts>> {
        self.0
            .lock()
            .expect("canonical input cache poisoned")
            .get(handle)
            .cloned()
    }
    pub(super) fn insert(&self, handle: Handle, value: SupportedFheCiphertexts) {
        self.0
            .lock()
            .expect("canonical input cache poisoned")
            .insert(handle, Arc::new(value));
    }
}

struct OperationOutputs {
    raw: Vec<Arc<SupportedFheCiphertexts>>,
    compressed: Vec<Option<CompressedCiphertext>>,
}

struct Completed {
    index: NodeIndex,
    outputs: Vec<DFGOutput>,
    result: Result<OperationOutputs>,
}

/// All ciphertext operations, including clones, happen with the correct key
/// installed on a blocking thread. The coordinator only clones Arc pointers.
#[allow(clippy::too_many_arguments)]
fn execute_ready(
    mut node: OpNode,
    index: NodeIndex,
    inputs: Arc<HashMap<Handle, Option<DFGTxInput>>>,
    local: HashMap<Handle, Arc<SupportedFheCiphertexts>>,
    cache: Arc<CanonicalInputs>,
    sks: ServerKey,
    cpk: tfhe::CompactPublicKey,
    device: usize,
    tid: Handle,
    timeout: Duration,
    heartbeat: HeartBeat,
) -> Completed {
    let outputs = node.outputs.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tfhe::set_server_key(sks);
        #[cfg(test)]
        let _probe = tests::operation_started(&tid, index.index());
        #[cfg(feature = "test-failpoints")]
        let _reservation_scope =
            fhevm_engine_common::reservation_test_control::TransactionScope::enter(&tid);
        for input in &mut node.inputs {
            if let DFGTaskInput::LocalDependence(handle) = input {
                if let Some(value) = local.get(handle) {
                    *input = DFGTaskInput::Value(value.as_ref().clone());
                }
            }
        }
        let (_, result) = try_execute_node(
            &mut node,
            index.index(),
            &inputs,
            device,
            &tid,
            &cpk,
            timeout,
            &cache,
        )?;
        let working = result?;
        validate_results(
            &outputs,
            &working.iter().map(|v| v.type_num()).collect::<Vec<_>>(),
        )?;
        // Multi-output publication stays atomic: no raw or persisted sibling
        // escapes if a later output fails validation or compression.
        let compressed = outputs
            .iter()
            .zip(&working)
            .map(|(output, value)| {
                output
                    .is_allowed
                    .then(|| compress_output(value, &tid, node.opcode))
                    .transpose()
            })
            .collect::<Result<Vec<_>>>()?;
        let mut working = working.into_iter();
        let mut ready = Vec::with_capacity(outputs.len());
        if let Some(first) = working.next() {
            // TFHE 1.8.1's high-level CUDA clone duplicates on the installed
            // key's streams and synchronizes them. Fence the producer stream
            // before handing raw ciphertexts to other threads/streams, including
            // operations with NO compressed outputs. This never serializes to
            // the host or inserts a canonical compression boundary.
            #[cfg(feature = "gpu")]
            let first = first.clone();
            ready.push(Arc::new(first));
        }
        ready.extend(working.map(Arc::new));
        Ok(OperationOutputs {
            raw: ready,
            compressed,
        })
    }))
    .unwrap_or_else(|panic| Err(SchedulerError::ExecutionPanic(panic_message(panic)).into()));
    heartbeat.update();
    Completed {
        index,
        outputs,
        result,
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn execute_partition(
    transactions: ComponentSet,
    task_id: NodeIndex,
    dispatched_at: std::time::Instant,
    device: usize,
    sks: ServerKey,
    cpk: tfhe::CompactPublicKey,
    timeout: Duration,
    heartbeat: HeartBeat,
    #[cfg(feature = "gpu")] limiter: GpuExecutionLimiter,
) -> PartitionResult {
    let mut outputs: HashMap<Handle, TaskResult> = HashMap::new();
    let mut reported = Vec::new();
    let cache = Arc::new(CanonicalInputs::default());
    let partition_tx_count = transactions.len();
    'transaction: for (mut dfg, mut inputs, tid, _) in transactions {
        for (handle, input) in &mut inputs {
            if input.is_none() {
                if let Some(value) = outputs.get(handle) {
                    *input = Some(DFGTxInput::Compressed((
                        value.compressed_ct.clone(),
                        value.is_allowed,
                    )));
                } else {
                    for node in dfg.graph.graph().node_weights() {
                        if node.is_owned {
                            reported.push((
                                node.handles(),
                                tid.clone(),
                                Err(SchedulerError::MissingInputs.into()),
                            ));
                        }
                    }
                    continue 'transaction;
                }
            }
        }
        if daggy::petgraph::algo::toposort(&dfg.graph, None).is_err() {
            for node in dfg.graph.graph().node_weights() {
                if node.is_owned {
                    reported.push((
                        node.handles(),
                        tid.clone(),
                        Err(SchedulerError::CyclicDependence.into()),
                    ));
                }
            }
            continue;
        }
        let started = std::time::Instant::now();
        let inputs = Arc::new(inputs);
        let mut pending: Vec<_> = dfg
            .graph
            .node_identifiers()
            .map(|i| dfg.graph.edges_directed(i, Direction::Incoming).count())
            .collect();
        let mut ready: VecDeque<_> = pending
            .iter()
            .enumerate()
            .filter_map(|(i, &count)| (count == 0).then_some(NodeIndex::new(i)))
            .collect();
        let mut raw: HashMap<Handle, Arc<SupportedFheCiphertexts>> = HashMap::new();
        // Keep raw results only until their last consumer has been dispatched.
        // Tasks own Arc references, so the coordinator need not pin every
        // intermediate for the lifetime of a wide or long transaction.
        let mut uses: HashMap<Handle, usize> = HashMap::new();
        for node in dfg.graph.graph().node_weights() {
            for input in &node.inputs {
                if let DFGTaskInput::LocalDependence(handle) = input {
                    *uses.entry(handle.clone()).or_default() += 1;
                }
            }
        }
        let mut tasks = JoinSet::new();
        let mut task_nodes = HashMap::new();
        #[cfg(feature = "gpu")]
        let capacity = limiter.device_capacity();
        #[cfg(not(feature = "gpu"))]
        let capacity = std::thread::available_parallelism().map_or(1, usize::from);
        while !ready.is_empty() || !tasks.is_empty() {
            while !ready.is_empty() && tasks.len() < capacity {
                #[cfg(feature = "gpu")]
                let permit = if tasks.is_empty() {
                    limiter.acquire(device).await.map(Some)
                } else {
                    limiter.try_acquire(device)
                };
                #[cfg(feature = "gpu")]
                let permit = match permit {
                    Ok(Some(permit)) => permit,
                    Ok(None) => break,
                    Err(error) => {
                        let index = ready.pop_front().expect("nonempty ready queue");
                        let outputs = dfg.graph[index].outputs.clone();
                        let task = tasks.spawn(async move {
                            Completed {
                                index,
                                outputs,
                                result: Err(error),
                            }
                        });
                        task_nodes.insert(task.id(), index);
                        continue;
                    }
                };
                let index = ready.pop_front().expect("nonempty ready queue");
                let node = &mut dfg.graph[index];
                let local = node
                    .inputs
                    .iter()
                    .filter_map(|input| match input {
                        DFGTaskInput::LocalDependence(handle) => raw
                            .get(handle)
                            .map(|value| (handle.clone(), Arc::clone(value))),
                        _ => None,
                    })
                    .collect();
                for input in &node.inputs {
                    if let DFGTaskInput::LocalDependence(handle) = input {
                        let remaining = uses.get_mut(handle).expect("local use counted");
                        *remaining -= 1;
                        if *remaining == 0 {
                            raw.remove(handle);
                        }
                    }
                }
                let work = OpNode {
                    opcode: node.opcode,
                    outputs: node.outputs.clone(),
                    inputs: std::mem::take(&mut node.inputs),
                    is_owned: node.is_owned,
                };
                let (inputs, cache, sks, cpk, tid, heartbeat) = (
                    Arc::clone(&inputs),
                    Arc::clone(&cache),
                    sks.clone(),
                    cpk.clone(),
                    tid.clone(),
                    heartbeat.clone(),
                );
                let span = tracing::Span::current();
                let task = tasks.spawn_blocking(move || {
                    #[cfg(feature = "gpu")]
                    let _permit = permit;
                    let _span = span.enter();
                    execute_ready(
                        work, index, inputs, local, cache, sks, cpk, device, tid, timeout,
                        heartbeat,
                    )
                });
                task_nodes.insert(task.id(), index);
            }
            let Some(completed) = tasks.join_next_with_id().await else {
                break;
            };
            // execute_ready contains all operation panics. An executor-level
            // panic is unexpected, but must not silently drop remaining work.
            let completed = match completed {
                Ok((task, completed)) => {
                    task_nodes.remove(&task);
                    completed
                }
                Err(error) => {
                    let index = task_nodes
                        .remove(&error.id())
                        .expect("operation task tracked");
                    Completed {
                        index,
                        outputs: dfg.graph[index].outputs.clone(),
                        result: Err(SchedulerError::ExecutionPanic(format!(
                            "operation task join: {error}"
                        ))
                        .into()),
                    }
                }
            };
            let index = completed.index;
            match completed.result {
                Ok(OperationOutputs {
                    raw: working,
                    compressed,
                }) => {
                    for ((output, value), compressed) in
                        completed.outputs.into_iter().zip(working).zip(compressed)
                    {
                        if uses.get(&output.handle).copied().unwrap_or(0) > 0 {
                            raw.insert(output.handle.clone(), value);
                        }
                        if let Some(compressed_ct) = compressed {
                            let result = TaskResult {
                                compressed_ct,
                                is_allowed: output.is_allowed,
                                transaction_id: tid.clone(),
                            };
                            outputs.insert(output.handle.clone(), result.clone());
                            reported.push((vec![output.handle], tid.clone(), Ok(result)));
                        }
                    }
                }
                Err(error) => reported.push((
                    completed.outputs.iter().map(|o| o.handle.clone()).collect(),
                    tid.clone(),
                    Err(error),
                )),
            }
            // Failed producers also release dependants so they get explicit
            // missing-input outcomes; independent branches continue normally.
            for edge in dfg.graph.edges_directed(index, Direction::Outgoing) {
                let child = edge.target();
                pending[child.index()] -= 1;
                if pending[child.index()] == 0 {
                    ready.push_back(child);
                }
            }
        }
        FHE_BATCH_LATENCY_HISTOGRAM.observe(started.elapsed().as_secs_f64());
    }
    tracing::info!(target: "scheduler", total_us = dispatched_at.elapsed().as_micros() as u64,
        tx_count = partition_tx_count, "parallel_partition");
    (reported, task_id, device)
}

#[cfg(test)]
mod tests;
