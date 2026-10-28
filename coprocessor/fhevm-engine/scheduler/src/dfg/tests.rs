use super::*;
use fhevm_engine_common::types::SupportedFheCiphertexts;

fn handle(value: u8) -> Handle {
    vec![value; 32]
}

fn trivial(output: u8) -> DFGOp {
    DFGOp {
        output_handle: handle(output),
        fhe_op: SupportedFheOperations::FheTrivialEncrypt,
        inputs: vec![
            DFGTaskInput::Value(SupportedFheCiphertexts::Scalar(vec![0; 32])),
            DFGTaskInput::Value(SupportedFheCiphertexts::Scalar(vec![4])),
        ],
        is_allowed: true,
    }
}

fn add(output: u8, lhs: u8, rhs: u8) -> DFGOp {
    DFGOp {
        output_handle: handle(output),
        fhe_op: SupportedFheOperations::FheAdd,
        inputs: vec![
            DFGTaskInput::Dependence(handle(lhs)),
            DFGTaskInput::Dependence(handle(rhs)),
        ],
        is_allowed: true,
    }
}

fn build(transactions: Vec<(u8, Vec<DFGOp>)>) -> DFComponentGraph {
    let mut components = vec![];
    for (tx, ops) in transactions {
        let (nodes, _) = build_component_nodes(ops, &handle(tx)).expect("valid transaction");
        components.extend(nodes);
    }
    let mut graph = DFComponentGraph::default();
    graph.build(&mut components).expect("component graph");
    graph
}

fn alias_transactions() -> Vec<(u8, Vec<DFGOp>)> {
    vec![
        // t0 originally produced h1, but belongs to another, unprocessed chain.
        (0x11, vec![add(0x03, 0x01, 0x01)]),
        // t2 repeats t0's constant, giving it the same handle within the block.
        (0x22, vec![trivial(0x01), add(0x04, 0x01, 0x03)]),
    ]
}

#[test]
fn cross_transaction_cycle_defers_its_members_instead_of_condemning_them() {
    let mut transactions = alias_transactions();
    transactions.extend([
        (0x99, vec![trivial(0x09)]),
        (0xaa, vec![add(0x0a, 0x09, 0x09)]),
    ]);
    let mut graph = build(transactions);
    graph
        .resolve_dependences(&HashSet::new())
        .expect("a batch-composition cycle must not abort unrelated work");

    let mut deferred = vec![];
    for result in graph.get_results() {
        let error = result.compressed_ct.err().expect("deferred, without bytes");
        assert!(
            matches!(
                error.downcast_ref::<SchedulerError>(),
                Some(SchedulerError::MissingInputs)
            ),
            "cycle members must remain retryable: {error}"
        );
        deferred.push((result.transaction_id, result.handle));
    }
    deferred.sort();
    assert_eq!(
        deferred,
        vec![
            (handle(0x11), handle(0x03)),
            (handle(0x22), handle(0x01)),
            (handle(0x22), handle(0x04)),
        ]
    );
    for (_, component) in graph.graph.node_references() {
        let in_cycle = [handle(0x11), handle(0x22)].contains(&component.transaction_id);
        assert_eq!(component.is_uncomputable, in_cycle);
    }
    // Edges touching the deferred members are omitted, but the unrelated
    // producer/consumer pair retains its scheduling order.
    assert_eq!(graph.graph.edge_count(), 1);
    let edge = graph.graph.edge_references().next().unwrap();
    assert_eq!(graph.graph[edge.source()].transaction_id, handle(0x99));
    assert_eq!(graph.graph[edge.target()].transaction_id, handle(0xaa));
}

#[test]
fn cross_transaction_cycle_resolves_after_original_producer_is_persisted() {
    let mut first_attempt = build(alias_transactions());
    first_attempt.resolve_dependences(&HashSet::new()).unwrap();
    assert_eq!(first_attempt.get_results().len(), 3);

    // A later worker cycle rebuilds the graph and fetches t0's persisted h1.
    // Graph construction does not inspect the compressed bytes.
    let mut retry = build(alias_transactions());
    retry
        .add_input(
            &handle(0x01),
            &DFGTxInput::Compressed((
                CompressedCiphertext {
                    ct_type: 4,
                    ct_bytes: vec![],
                },
                true,
            )),
        )
        .unwrap();
    retry
        .resolve_dependences(&HashSet::from([handle(0x01)]))
        .expect("persisted input removes the false back edge");
    assert!(retry.get_results().is_empty());
    assert!(retry
        .graph
        .graph()
        .node_weights()
        .all(|tx| !tx.is_uncomputable));
    assert_eq!(retry.graph.edge_count(), 1);
    let edge = retry.graph.edge_references().next().unwrap();
    assert_eq!(retry.graph[edge.source()].transaction_id, handle(0x11));
    assert_eq!(retry.graph[edge.target()].transaction_id, handle(0x22));
    assert!(retry.graph[edge.source()].inputs[&handle(0x01)].is_some());
}

#[test]
fn cross_transaction_alias_with_original_producer_in_batch_is_not_a_cycle() {
    let mut transactions = alias_transactions();
    transactions.push((0x00, vec![trivial(0x01)]));
    let mut graph = build(transactions);
    graph.resolve_dependences(&HashSet::new()).unwrap();
    assert!(graph.get_results().is_empty());
    assert!(graph
        .graph
        .graph()
        .node_weights()
        .all(|tx| !tx.is_uncomputable));
}

#[test]
fn transaction_local_cycles_remain_errors() {
    let result = build_component_nodes(
        vec![add(0x01, 0x02, 0x02), add(0x02, 0x01, 0x01)],
        &handle(0x11),
    );
    let error = result.expect_err("invalid transaction graph");
    assert!(matches!(
        error.downcast_ref::<SchedulerError>(),
        Some(SchedulerError::CyclicDependence)
    ));
}
