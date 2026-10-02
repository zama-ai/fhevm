use super::*;
use crate::dfg::{build_component_nodes, DFGOp};
use std::collections::BTreeMap;
use std::sync::{Condvar, OnceLock};
use tfhe::prelude::FheEncrypt;

struct Fixture {
    client: tfhe::ClientKey,
    key: ServerKey,
    public: tfhe::CompactPublicKey,
}
fn fixture() -> &'static Fixture {
    static FIXTURE: OnceLock<Fixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        use fhevm_engine_common::keys::{
            TFHE_COMPACT_PK_PARAMS, TFHE_COMPRESSION_PARAMS, TFHE_PARAMS,
            TFHE_PKS_RERANDOMIZATION_PARAMS,
        };
        let config = tfhe::ConfigBuilder::with_custom_parameters(TFHE_PARAMS)
            .use_dedicated_compact_public_key_parameters((
                TFHE_COMPACT_PK_PARAMS.pke_params,
                TFHE_COMPACT_PK_PARAMS.ksk_params,
            ))
            .enable_compression(TFHE_COMPRESSION_PARAMS)
            .enable_ciphertext_re_randomization(TFHE_PKS_RERANDOMIZATION_PARAMS)
            .build();
        let client = tfhe::ClientKey::generate(config);
        let public = tfhe::CompactPublicKey::new(&client);
        #[cfg(not(feature = "gpu"))]
        let key = tfhe::ServerKey::new(&client);
        #[cfg(feature = "gpu")]
        let key = tfhe::CompressedServerKey::new(&client)
            .decompress_to_specific_gpu(tfhe::CudaGpuChoice::Single(tfhe::GpuIndex::new(0)));
        Fixture {
            client,
            key,
            public,
        }
    })
}
fn handle(value: u8) -> Handle {
    let mut h = vec![value; 32];
    h[30] = 2; // FheUint8
    h
}
fn op(
    value: u8,
    opcode: SupportedFheOperations,
    inputs: Vec<DFGTaskInput>,
    allowed: bool,
) -> DFGOp {
    DFGOp {
        outputs: vec![DFGOutput {
            handle: handle(value),
            is_allowed: allowed,
        }],
        fhe_op: opcode,
        inputs,
        is_owned: true,
    }
}
fn local(value: u8) -> DFGTaskInput {
    DFGTaskInput::LocalDependence(handle(value))
}
fn scalar(value: u8) -> DFGTaskInput {
    DFGTaskInput::Value(SupportedFheCiphertexts::Scalar(vec![value]))
}
fn component(
    ops: Vec<DFGOp>,
    tid: u8,
) -> (DFGraph, HashMap<Handle, Option<DFGTxInput>>, Handle, usize) {
    let mut components = build_component_nodes(ops, &vec![tid; 32]).unwrap().0;
    assert_eq!(
        components.len(),
        1,
        "transaction remains the materialisation unit"
    );
    let c = components.remove(0);
    (c.graph, c.inputs, c.transaction_id, c.component_id)
}
fn transactions(a: &SupportedFheCiphertexts, b: &SupportedFheCiphertexts, tid: u8) -> ComponentSet {
    vec![
        component(
            vec![
                op(
                    1,
                    SupportedFheOperations::FheAdd,
                    vec![DFGTaskInput::Value(a.clone()), scalar(3)],
                    false,
                ),
                op(
                    2,
                    SupportedFheOperations::FheSub,
                    vec![DFGTaskInput::Value(b.clone()), scalar(5)],
                    false,
                ),
                op(
                    3,
                    SupportedFheOperations::FheAdd,
                    vec![local(1), local(2)],
                    true,
                ),
                op(
                    4,
                    SupportedFheOperations::FheBitXor,
                    vec![local(1), local(1)],
                    true,
                ),
                op(
                    5,
                    SupportedFheOperations::FheAdd,
                    vec![local(1), scalar(1)],
                    true,
                ),
            ],
            tid,
        ),
        component(
            vec![op(
                6,
                SupportedFheOperations::FheAdd,
                vec![DFGTaskInput::BoundaryDependence(handle(3)), scalar(1)],
                true,
            )],
            tid + 1,
        ),
    ]
}
fn runtime(blocking_threads: usize) -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(blocking_threads)
        .enable_all()
        .build()
        .unwrap()
}
fn evaluate(
    transactions: ComponentSet,
    concurrency: usize,
    blocking_threads: usize,
) -> PartitionResult {
    let f = fixture();
    #[cfg(not(feature = "gpu"))]
    let _ = concurrency;
    runtime(blocking_threads).block_on(execute_partition(
        transactions,
        NodeIndex::new(0),
        std::time::Instant::now(),
        0,
        f.key.clone(),
        f.public.clone(),
        Duration::from_secs(30),
        HeartBeat::new(),
        #[cfg(feature = "gpu")]
        GpuExecutionLimiter::new(1, concurrency).unwrap(),
    ))
}
fn bytes(result: PartitionResult) -> BTreeMap<(Handle, Handle), Vec<u8>> {
    result
        .0
        .into_iter()
        .map(|(handles, tid, result)| {
            assert_eq!(handles.len(), 1);
            (
                (tid, handles[0].clone()),
                result.unwrap().compressed_ct.ct_bytes,
            )
        })
        .collect()
}

#[test]
fn parallel_fanout_join_and_repeated_operand_match_serial_bytes() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let b = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(29u8, &f.client));
    let reference = bytes(execute_partition_serial(
        transactions(&a, &b, 0xa1),
        NodeIndex::new(0),
        std::time::Instant::now(),
        0,
        f.key.clone(),
        f.public.clone(),
        Duration::from_secs(30),
        HeartBeat::new(),
    ));
    for capacity in [1, 2, 4] {
        let result = bytes(evaluate(transactions(&a, &b, 0xa1), capacity, capacity));
        assert_eq!(
            result, reference,
            "raw local edges and canonical cross-tx bytes at cap {capacity}"
        );
        for ((_, handle), bytes) in result {
            let value =
                SupportedFheCiphertexts::decompress(2, &bytes, 0, Duration::from_secs(30)).unwrap();
            let expected = match handle[0] {
                3 => "44",
                4 => "0",
                5 => "21",
                6 => "45",
                _ => unreachable!(),
            };
            assert_eq!(value.decrypt(&f.client), expected);
        }
    }
}

#[test]
fn shared_canonical_inputs_match_serial_with_concurrent_cache_access() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let compressed =
        compress_output(&a, &vec![0x99; 32], SupportedFheOperations::FheAdd as i32).unwrap();
    let txs = || {
        vec![component(
            (1..=6)
                .map(|id| {
                    op(
                        id,
                        SupportedFheOperations::FheAdd,
                        vec![
                            DFGTaskInput::Compressed(handle(9), compressed.clone()),
                            DFGTaskInput::Compressed(handle(9), compressed.clone()),
                        ],
                        true,
                    )
                })
                .collect(),
            0x99,
        )]
    };
    let reference = bytes(execute_partition_serial(
        txs(),
        NodeIndex::new(0),
        std::time::Instant::now(),
        0,
        f.key.clone(),
        f.public.clone(),
        Duration::from_secs(30),
        HeartBeat::new(),
    ));
    assert_eq!(reference.len(), 6);
    for capacity in [1, 2, 4] {
        assert_eq!(bytes(evaluate(txs(), capacity, capacity)), reference);
    }
}

#[test]
fn a_single_blocking_thread_finishes_without_nested_pool_deadlock() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let b = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(29u8, &f.client));
    assert_eq!(bytes(evaluate(transactions(&a, &b, 0xb1), 1, 1)).len(), 4);
}

#[test]
fn a_failed_internal_producer_does_not_discard_an_independent_branch() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let tx = component(
        vec![
            op(
                1,
                SupportedFheOperations::FheAdd,
                vec![
                    DFGTaskInput::Compressed(
                        handle(9),
                        CompressedCiphertext {
                            ct_type: 2,
                            ct_bytes: vec![0xff; 7],
                        },
                    ),
                    scalar(1),
                ],
                false,
            ),
            op(
                2,
                SupportedFheOperations::FheAdd,
                vec![local(1), scalar(1)],
                true,
            ),
            op(
                3,
                SupportedFheOperations::FheAdd,
                vec![DFGTaskInput::Value(a), scalar(1)],
                true,
            ),
        ],
        0xc1,
    );
    let results = evaluate(vec![tx], 2, 2).0;
    assert_eq!(results.len(), 3);
    for (handles, _, result) in results {
        assert_eq!(result.is_ok(), handles[0][0] == 3);
    }
}

#[derive(Default)]
struct ProbeState {
    entered: usize,
    active: usize,
    peak: usize,
    timed_out: bool,
}
struct Probe {
    tid: Handle,
    expected_active: usize,
    state: Mutex<ProbeState>,
    wake: Condvar,
}
static PROBE: Mutex<Option<Arc<Probe>>> = Mutex::new(None);
pub(super) struct ProbeGuard(Arc<Probe>);
impl Drop for ProbeGuard {
    fn drop(&mut self) {
        self.0.state.lock().unwrap().active -= 1;
    }
}
pub(super) fn operation_started(tid: &Handle, index: usize) -> Option<ProbeGuard> {
    let probe = PROBE.lock().unwrap().clone()?;
    if probe.tid != *tid || index > 1 {
        return None;
    }
    let mut state = probe.state.lock().unwrap();
    state.entered += 1;
    state.active += 1;
    state.peak = state.peak.max(state.active);
    probe.wake.notify_all();
    let (mut state, timeout) = probe
        .wake
        .wait_timeout_while(state, Duration::from_secs(5), |s| {
            s.entered < probe.expected_active
        })
        .unwrap();
    state.timed_out |= timeout.timed_out();
    drop(state);
    Some(ProbeGuard(probe))
}

#[test]
fn independent_operations_in_one_transaction_really_overlap() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let b = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(29u8, &f.client));
    // CPU admission respects the process cpuset; GPU admission uses the
    // explicitly configured per-device budget even on a single-CPU host.
    let expected_active = if cfg!(feature = "gpu") {
        2
    } else {
        std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1)
            .min(2)
    };
    let probe = Arc::new(Probe {
        tid: vec![0xd1; 32],
        expected_active,
        state: Mutex::new(ProbeState::default()),
        wake: Condvar::new(),
    });
    *PROBE.lock().unwrap() = Some(Arc::clone(&probe));
    let result = evaluate(transactions(&a, &b, 0xd1), 2, 2);
    *PROBE.lock().unwrap() = None;
    assert_eq!(bytes(result).len(), 4);
    let state = probe.state.lock().unwrap();
    assert_eq!(
        state.peak, expected_active,
        "ready siblings must use the available operation budget"
    );
    assert!(
        !state.timed_out,
        "ready work must not serialize when the operation budget allows overlap"
    );
}

// Frozen serial scheduling algorithm from the parent revision, retained only
// as a byte-level differential oracle. It shares crypto primitives, not the
// ready-queue or cross-thread forwarding implementation.
use tracing::warn;
/// Executes a partition of whole transactions in topological order.
///
/// The transaction is the materialization boundary. Every value produced in
/// this transaction is forwarded raw to its same-transaction consumers,
/// including values which are also compressed for persistence. Values which
/// enter from another transaction are reconstructed from their canonical
/// persisted representation. The consuming handle commits to that origin, so
/// the raw and canonical forms cannot alias even when they represent the same
/// plaintext operation.
#[allow(clippy::too_many_arguments)]
fn execute_partition_serial(
    transactions: ComponentSet,
    task_id: NodeIndex,
    dispatched_at: std::time::Instant,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
    gpu_reservation_timeout: Duration,
    activity_heartbeat: HeartBeat,
) -> PartitionResult {
    let spawned_at = std::time::Instant::now();
    tfhe::set_server_key(sks);
    let key_installed_at = std::time::Instant::now();
    let partition_tx_count = transactions.len();
    // Per-partition memo of the canonical decompressed form ct(h), permitted
    // by RFC-020 ("the worker may cache the canonical decompressed form ct(h)
    // for the duration of the transaction batch"). Without it a boundary
    // handle consumed by K ops is decompressed K times from identical bytes.
    //
    // Scoped to the PARTITION, not the batch, on purpose: a partition runs on
    // one thread with one server key installed, so entries are created on the
    // device that consumes them and need no synchronization. A batch-wide
    // cache would have to be shared across partition threads and would pin
    // every boundary value to whichever device populated it.
    //
    // Memoization is observationally transparent — Decompress(cmp(h)) is
    // deterministic, so a hit and a miss yield the same value.
    let boundary_cache = CanonicalInputs::default();
    // Two channels, because they answer different questions. `outputs` resolves a
    // later transaction's input from an earlier one's output in this partition,
    // where the handle IS the identity -- ct(h) is the same value whichever
    // transaction minted it, which is the same memo argument as
    // `boundary_cache` above. `reported` is what the graph is told, and there
    // the transaction matters: the same handle from two transactions is two
    // rows, two stamps and two verdicts.
    let mut outputs: HashMap<Handle, TaskResult> = HashMap::with_capacity(transactions.len());
    let mut reported: Vec<PartitionOutcome> = Vec::with_capacity(transactions.len());
    // Traverse transactions within the partition. The transactions
    // are topologically sorted so the order is executable
    'tx: for (ref mut dfg, ref mut tx_inputs, tid, _cid) in transactions {
        #[cfg(feature = "test-failpoints")]
        let _reservation_scope =
            fhevm_engine_common::reservation_test_control::TransactionScope::enter(&tid);
        let txn_id_short = telemetry::short_hex_id(&tid);

        // Update the transaction inputs based on allowed handles so
        // far. If any input is still missing, and we cannot fill it
        // (e.g., error in the producer transaction) we cannot execute
        // this transaction and possibly more downstream.
        for (h, i) in tx_inputs.iter_mut() {
            if i.is_none() {
                let Some(ct) = outputs.get(h) else {
                    warn!(target: "scheduler", {transaction_id = ?hex::encode(&tid) },
		       "Missing input to compute transaction - skipping");
                    for nidx in dfg.graph.node_identifiers() {
                        let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                            error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                            continue;
                        };
                        // `is_owned`, matching the bubbled-error arm below:
                        // an internal producer under our lease has a row of
                        // ours to stamp, and gating on allowance dropped its
                        // verdict on the floor.
                        if node.is_owned {
                            reported.push((
                                node.handles(),
                                tid.clone(),
                                Err(SchedulerError::MissingInputs.into()),
                            ));
                        }
                    }
                    continue 'tx;
                };
                *i = Some(DFGTxInput::Compressed((
                    ct.compressed_ct.clone(),
                    ct.is_allowed,
                )));
            }
        }

        // Prime the scheduler with ready ops from the transaction's subgraph
        let _exec_guard = tracing::info_span!(
            "execute_transaction",
            txn_id = %txn_id_short,
        )
        .entered();
        let started_at = std::time::Instant::now();

        let Ok(ts) = daggy::petgraph::algo::toposort(&dfg.graph, None) else {
            error!(target: "scheduler", {transaction_id = ?tid },
		       "Cyclical dependence error in transaction");
            for nidx in dfg.graph.node_identifiers() {
                let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                    error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                    continue;
                };
                if node.is_owned {
                    reported.push((
                        node.handles(),
                        tid.clone(),
                        Err(SchedulerError::CyclicDependence.into()),
                    ));
                }
            }
            continue 'tx;
        };
        let edges = dfg.graph.map(|_, _| (), |_, edge| *edge);
        for nidx in ts.iter() {
            let Some(node) = dfg.graph.node_weight_mut(*nidx) else {
                error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                continue;
            };
            let result = try_execute_node(
                node,
                nidx.index(),
                tx_inputs,
                gpu_idx,
                &tid,
                &cpk,
                gpu_reservation_timeout,
                &boundary_cache,
            );
            // Per-op progress tick: a partition can legitimately run longer
            // than both the heartbeat freshness window and the in-flight
            // batch TTL; liveness must track op completions, not partition
            // completions, so only a genuinely wedged op exhausts the TTL.
            activity_heartbeat.update();
            match result {
                Ok((node_index, op_result)) => {
                    let nidx = NodeIndex::new(node_index);
                    let Some(node) = dfg.graph.node_weight(nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    // Named apart from the partition's `outputs` results map.
                    let node_outputs = node.outputs.clone();
                    let producer_handles: Vec<Handle> =
                        node_outputs.iter().map(|o| o.handle.clone()).collect();
                    let opcode = node.opcode;
                    let working = match op_result {
                        Ok(working) => working,
                        Err(e) => {
                            reported.push((producer_handles.clone(), tid.clone(), Err(e)));
                            continue;
                        }
                    };
                    let produced: Vec<i16> = working.iter().map(|v| v.type_num()).collect();
                    // Nothing is forwarded or published: a wrong count or type
                    // means the results cannot be matched to the handles, so the
                    // whole group fails rather than binding to the wrong ones.
                    if let Err(error) = validate_results(&node_outputs, &produced) {
                        error!(target: "scheduler", { error = %error },
                        "Dispatch result does not match the operation's declared outputs");
                        reported.push((producer_handles.clone(), tid.clone(), Err(error.into())));
                        continue;
                    }
                    // Each consumer's representation of this output
                    // is pinned on chain: the executor folded a
                    // boundary bit per operand into the consuming
                    // handle, zero for operands minted in the
                    // consuming transaction. Every in-graph edge here
                    // is by definition that case, so all of them
                    // forward the raw working value — no
                    // compress/decompress round-trip for
                    // same-transaction consumers, and no byte-equality
                    // obligation against differently-sourced aliases,
                    // which now mint different handles.
                    //
                    // An output is compressed iff it is allowed:
                    // persistence needs the bytes, and any
                    // cross-transaction consumer must have been
                    // granted a persistent allowance first (transient
                    // allowances are transaction-scoped), so
                    // cross-transaction consumers need no separate
                    // tracking.
                    //
                    // A multi-output op materializes each output
                    // separately, so the rule above is applied per
                    // handle rather than once per operation.
                    //
                    // Compress all allowed outputs before recording any: a late
                    // failure must not leave earlier siblings completed.
                    let mut compressed: Vec<Option<CompressedCiphertext>> =
                        Vec::with_capacity(node_outputs.len());
                    let mut compression_failure: Option<anyhow::Error> = None;
                    for (output, value) in node_outputs.iter().zip(working.iter()) {
                        if !output.is_allowed {
                            compressed.push(None);
                            continue;
                        }
                        match compress_output(value, &tid, opcode) {
                            Ok(compressed_ct) => compressed.push(Some(compressed_ct)),
                            Err(e) => {
                                compression_failure = Some(e);
                                break;
                            }
                        }
                    }
                    if let Some(e) = compression_failure {
                        error!(target: "scheduler", { error = %e },
                        "Compression failed for an allowed output; failing the whole operation");
                        reported.push((producer_handles.clone(), tid.clone(), Err(e)));
                        continue;
                    }
                    let mut forwarded: Vec<SupportedFheCiphertexts> =
                        Vec::with_capacity(working.len());
                    for ((output, value), compressed_ct) in
                        node_outputs.iter().zip(working).zip(compressed)
                    {
                        if let Some(compressed_ct) = compressed_ct {
                            let task_result = TaskResult {
                                compressed_ct,
                                is_allowed: output.is_allowed,
                                transaction_id: tid.clone(),
                            };
                            outputs.insert(output.handle.clone(), task_result.clone());
                            reported.push((
                                vec![output.handle.clone()],
                                tid.clone(),
                                Ok(task_result),
                            ));
                        }
                        forwarded.push(value);
                    }
                    // Route each output to the consumers that name it:
                    // an edge carries the consuming input slot, and the
                    // handle in that slot selects which output feeds it.
                    for edge in edges.edges_directed(nidx, Direction::Outgoing) {
                        let child_index = edge.target();
                        let input_idx = *edge.weight() as usize;
                        let Some(child_node) = dfg.graph.node_weight_mut(child_index) else {
                            error!(target: "scheduler", {index = ?child_index.index() }, "Wrong dataflow graph index");
                            continue;
                        };
                        let dep_handle = match child_node.inputs.get(input_idx) {
                            Some(DFGTaskInput::LocalDependence(dh))
                            | Some(DFGTaskInput::BoundaryDependence(dh)) => dh.clone(),
                            // Already resolved; nothing to route.
                            Some(DFGTaskInput::Value(_)) | Some(DFGTaskInput::Compressed(..)) => {
                                continue
                            }
                            None => {
                                error!(target: "scheduler", { input_idx },
                                    "Edge names an input slot the consumer does not have - graph inconsistency");
                                continue;
                            }
                        };
                        let Some(out_idx) = producer_handles.iter().position(|h| h == &dep_handle)
                        else {
                            error!(target: "scheduler",
                            { handle = ?hex::encode(&dep_handle) },
                            "Consumer dependence handle not found in producer outputs - graph inconsistency");
                            continue;
                        };
                        child_node.inputs[input_idx] =
                            DFGTaskInput::Value(forwarded[out_idx].clone());
                    }
                }
                Err(e) => {
                    let Some(node) = dfg.graph.node_weight(*nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    // Report every producer's error, allowed or not: an unreported
                    // failure left its consumers waiting forever. The worker decides
                    // where a foreign row's error is recorded (`is_foreign_producer`).
                    reported.push((node.handles(), tid.clone(), Err(e)));
                }
            }
        }
        drop(_exec_guard);
        let elapsed = started_at.elapsed();
        FHE_BATCH_LATENCY_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    tracing::info!(
        target: "scheduler",
        dispatch_us = spawned_at.duration_since(dispatched_at).as_micros() as u64,
        key_install_us = key_installed_at.duration_since(spawned_at).as_micros() as u64,
        exec_us = key_installed_at.elapsed().as_micros() as u64,
        total_us = dispatched_at.elapsed().as_micros() as u64,
        tx_count = partition_tx_count,
        "partition_hop"
    );
    // No trailing device synchronization: it would be a DEVICE-WIDE barrier
    // that couples this partition to every other in-flight partition's
    // queued kernels — on a dependency-deep workload each link then waits
    // for all sibling chains before its successor can spawn. It is also not
    // needed for correctness: (1) every value that escapes the partition is
    // host bytes produced by compress, which synchronizes the partition's
    // own streams; (2) raw and canonical forwards are consumed inside the
    // partition on the same streams, in order; (3) buffer frees are
    // stream-ordered; (4) the GPU memory reservations already release at op
    // return, so the sync never extended that accounting.
    (reported, task_id, gpu_idx)
}

#[test]
fn malformed_multi_output_group_publishes_no_partial_success() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let mut invalid = op(
        1,
        SupportedFheOperations::FheAdd,
        vec![DFGTaskInput::Value(a), scalar(1)],
        true,
    );
    // This single-output operation cannot fulfill the declared output group.
    invalid.outputs.push(DFGOutput {
        handle: handle(2),
        is_allowed: true,
    });
    let results = evaluate(
        vec![component(
            vec![
                invalid,
                op(
                    3,
                    SupportedFheOperations::FheAdd,
                    vec![local(1), scalar(1)],
                    true,
                ),
                op(
                    4,
                    SupportedFheOperations::FheAdd,
                    vec![local(2), scalar(1)],
                    true,
                ),
            ],
            0xe1,
        )],
        2,
        2,
    )
    .0;
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|(_, _, result)| result.is_err()));
    assert_eq!(
        results
            .iter()
            .find(|(handles, _, _)| handles.len() == 2)
            .unwrap()
            .0,
        vec![handle(1), handle(2)]
    );
}

#[test]
fn equal_handles_in_different_transactions_keep_both_outcomes() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let b = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(29u8, &f.client));
    let mut txs = transactions(&a, &b, 0xf1);
    txs.extend(transactions(&a, &b, 0xf3));
    let results = bytes(evaluate(txs, 2, 2));
    assert_eq!(results.len(), 8);
    for output in [3, 4, 5] {
        assert_eq!(
            results[&(vec![0xf1; 32], handle(output))],
            results[&(vec![0xf3; 32], handle(output))]
        );
    }
}

#[test]
fn outer_scheduler_drains_multiple_partitions_with_one_blocking_thread() {
    let f = fixture();
    tfhe::set_server_key(f.key.clone());
    let a = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(17u8, &f.client));
    let b = SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(29u8, &f.client));
    let mut txs = transactions(&a, &b, 0xa3);
    txs.push(component(
        vec![op(
            8,
            SupportedFheOperations::FheAdd,
            vec![DFGTaskInput::Value(a), scalar(8)],
            true,
        )],
        0xa5,
    ));
    let mut components = txs
        .into_iter()
        .map(|(graph, inputs, transaction_id, component_id)| {
            let results = graph
                .graph
                .graph()
                .node_weights()
                .flat_map(OpNode::handles)
                .collect();
            crate::dfg::ComponentNode {
                graph,
                inputs,
                transaction_id,
                component_id,
                results,
                ..Default::default()
            }
        })
        .collect();
    let mut graph = DFComponentGraph::default();
    graph.build(&mut components).unwrap();
    graph.resolve_dependences(&Default::default()).unwrap();
    graph.snapshot_blocked_dependents();
    let mut partitions = Dag::default();
    partition_preserving_parallelism(&graph.graph, &mut partitions).unwrap();
    assert!(partitions.node_count() >= 2);
    runtime(1).block_on(async {
        let mut scheduler = Scheduler::new(
            &mut graph,
            #[cfg(not(feature = "gpu"))]
            f.key.clone(),
            f.public.clone(),
            #[cfg(feature = "gpu")]
            vec![f.key.clone()],
            #[cfg(feature = "gpu")]
            GpuExecutionLimiter::new(1, 1).unwrap(),
            HeartBeat::new(),
            Duration::from_secs(30),
        );
        scheduler.schedule().await.unwrap();
    });
    let results = graph.get_results();
    assert_eq!(results.len(), 5);
    assert!(results.iter().all(|r| r.compressed_ct.is_ok()));
}
