use super::*;
use crate::dfg::tests::{add, alias_transactions, build, handle, trivial};
use crate::dfg::DFGOp;
use daggy::petgraph::visit::IntoNodeReferences;
use fhevm_engine_common::keys::{
    TFHE_COMPACT_PK_PARAMS, TFHE_COMPRESSION_PARAMS, TFHE_PARAMS, TFHE_PKS_RERANDOMIZATION_PARAMS,
};
use std::collections::HashSet;
use tfhe::prelude::FheDecrypt;

fn pending_transactions() -> Vec<(u8, Vec<DFGOp>)> {
    let mut transactions = alias_transactions();
    // The repeated constant is an internal operand of the allowed result.
    transactions[1].1[0].is_allowed = false;
    transactions.push((0x33, vec![add(0x05, 0x04, 0x04)]));
    transactions
}

async fn schedule(
    graph: &mut DFComponentGraph,
    sks: &tfhe::ServerKey,
    cpk: &tfhe::CompactPublicKey,
    max_locality: bool,
) {
    let strategy = if max_locality {
        PartitionStrategy::MaxLocality
    } else {
        PartitionStrategy::MaxParallelism
    };
    Scheduler::new(graph, sks.clone(), cpk.clone(), HeartBeat::new())
        .schedule_coarse_grain(strategy)
        .await
        .unwrap();
}

fn decrypt(compressed: &CompressedCiphertext, cks: &tfhe::ClientKey) -> u32 {
    let ct = SupportedFheCiphertexts::decompress(compressed.ct_type, &compressed.ct_bytes, 0)
        .expect("valid compressed result");
    let SupportedFheCiphertexts::FheUint32(value) = ct else {
        panic!("expected euint32 result");
    };
    value.decrypt(cks)
}

#[tokio::test]
async fn deferred_cycles_skip_dispatch_preserve_other_work_and_retry() {
    // Fresh CPU keys using the worker's compute, compression and rerandomisation
    // parameters. No noise-squashing key or global environment changes needed.
    let config = tfhe::ConfigBuilder::with_custom_parameters(TFHE_PARAMS)
        .enable_compression(TFHE_COMPRESSION_PARAMS)
        .use_dedicated_compact_public_key_parameters((
            TFHE_COMPACT_PK_PARAMS.pke_params,
            TFHE_COMPACT_PK_PARAMS.ksk_params,
        ))
        .enable_ciphertext_re_randomization(TFHE_PKS_RERANDOMIZATION_PARAMS)
        .build();
    let keygen_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(32)
        .build()
        .unwrap();
    let (cks, sks, cpk) = keygen_pool.install(|| {
        let cks = tfhe::ClientKey::generate(config);
        let sks = tfhe::ServerKey::new(&cks);
        let cpk = tfhe::CompactPublicKey::new(&cks);
        (cks, sks, cpk)
    });

    for max_locality in [false, true] {
        let mut transactions = pending_transactions();
        let mut independent = trivial(0x09);
        let mut plaintext = vec![0; 32];
        plaintext[31] = 7;
        independent.inputs[0] = DFGTaskInput::Value(SupportedFheCiphertexts::Scalar(plaintext));
        transactions.extend([
            (0x99, vec![independent]),
            (0xaa, vec![add(0x0a, 0x09, 0x09)]),
        ]);
        let mut graph = build(transactions);
        graph.resolve_dependences(&HashSet::new()).unwrap();
        let deferred: Vec<_> = graph
            .graph
            .node_references()
            .filter(|(_, component)| component.is_uncomputable)
            .map(|(index, component)| (index, component.graph.graph.node_count()))
            .collect();
        assert_eq!(deferred.len(), 2);

        schedule(&mut graph, &sks, &cpk, max_locality).await;

        // The initial-ready branch must honor the same deferral flag as the
        // dependent-ready branch. Dispatching a component takes its inner graph.
        for (index, node_count) in deferred {
            assert_eq!(graph.graph[index].graph.graph.node_count(), node_count);
        }
        let mut successes = vec![];
        let mut pending = vec![];
        for result in graph.get_results() {
            match result.compressed_ct {
                Ok(compressed) => successes.push((result.handle, decrypt(&compressed, &cks))),
                Err(error) => {
                    assert!(matches!(
                        error.downcast_ref::<SchedulerError>(),
                        Some(SchedulerError::MissingInputs)
                    ));
                    pending.push((result.transaction_id, result.handle));
                }
            }
        }
        successes.sort();
        assert_eq!(successes, vec![(handle(0x09), 7), (handle(0x0a), 14)]);
        pending.sort();
        assert_eq!(
            pending,
            vec![
                (handle(0x11), handle(0x03)),
                (handle(0x22), handle(0x01)),
                (handle(0x22), handle(0x04)),
                (handle(0x33), handle(0x05)),
            ]
        );
        assert!(!graph
            .get_intermediate_handles()
            .contains(&(handle(0x01), handle(0x22))));

        // Materialize the real producer, then use its actual bytes on retry.
        let mut original = build(vec![(0x00, vec![trivial(0x01)])]);
        original.resolve_dependences(&HashSet::new()).unwrap();
        schedule(&mut original, &sks, &cpk, max_locality).await;
        let input = original.get_results().pop().unwrap().compressed_ct.unwrap();
        let mut retry = build(pending_transactions());
        retry
            .add_input(&handle(0x01), &DFGTxInput::Compressed((input, true)))
            .unwrap();
        retry
            .resolve_dependences(&HashSet::from([handle(0x01)]))
            .unwrap();
        schedule(&mut retry, &sks, &cpk, max_locality).await;
        let mut successes = vec![];
        for result in retry.get_results() {
            let compressed = result
                .compressed_ct
                .expect("retry computes allowed outputs");
            assert_eq!(decrypt(&compressed, &cks), 0);
            successes.push((result.transaction_id, result.handle));
        }
        successes.sort();
        assert_eq!(
            successes,
            vec![
                (handle(0x11), handle(0x03)),
                (handle(0x22), handle(0x04)),
                (handle(0x33), handle(0x05)),
            ]
        );
        assert!(retry
            .get_intermediate_handles()
            .contains(&(handle(0x01), handle(0x22))));
    }
}
