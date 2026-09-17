use super::containment_test_support::*;
use super::*;
use scheduler::dfg::types::DFGTxResult;

fn result(tx: u8, out: u8, success: bool) -> DFGTxResult {
    DFGTxResult {
        transaction_id: handle(tx),
        handles: vec![handle(out)],
        // These tests exercise persistence, not FHE execution or serialization.
        compressed_ct: if success {
            Ok(CompressedCiphertext {
                ct_type: 4,
                ct_bytes: vec![out],
            })
        } else {
            Err(fhevm_engine_common::types::FhevmError::BadInputs.into())
        },
    }
}

async fn check_late_drift(success: bool) {
    let (_db, pool) = setup().await;
    computation(&pool, 2, 1, 10, true, 90).await;
    computation(&pool, 3, 2, 10, false, 90).await;
    computation(&pool, 4, 3, 11, true, 90).await;
    computation(&pool, 5, 99, 10, true, 90).await;
    let original: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) FROM computations c WHERE output_handle <> $1 ORDER BY output_handle",
    )
    .bind(handle(5))
    .fetch_all(&pool)
    .await
    .unwrap();
    let mut trx = pool.begin().await.unwrap();
    fhevm_engine_common::drift_containment::acquire_ct_computation_permit(&mut trx)
        .await
        .unwrap();
    assert!(frozen_computations::drifted_ct64_handles(&pool)
        .await
        .unwrap()
        .is_empty());
    // Detection commits after scheduling, while the batch still holds its
    // shared barrier. The optimistic marking need not have run yet.
    drift(&pool, 1).await;
    let mut graph = DFComponentGraph::default();
    // Descendants precede their producers to catch order-dependent filtering.
    graph.results = vec![
        result(11, 4, success),
        result(10, 3, success),
        result(10, 2, success),
        result(10, 5, true),
    ];
    let mut locks = dependence_chain::LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        true,
        None,
        None,
        None,
        3,
    );
    let mut freeze = super::frozen_computations::Freeze {
        batch: vec![
            super::frozen_computations::BatchRow {
                output: handle(4),
                tx: handle(11),
                deps: vec![handle(3)],
            },
            super::frozen_computations::BatchRow {
                output: handle(3),
                tx: handle(10),
                deps: vec![handle(2)],
            },
            super::frozen_computations::BatchRow {
                output: handle(2),
                tx: handle(10),
                deps: vec![handle(1)],
            },
            super::frozen_computations::BatchRow {
                output: handle(5),
                tx: handle(10),
                deps: vec![handle(99)],
            },
        ],
        ..Default::default()
    };
    let (progress, _) =
        upload_transaction_graph_results(&mut graph, &mut trx, &mut locks, false, &mut freeze)
            .await
            .unwrap();
    assert!(progress, "independent output still commits");
    trx.commit().await.unwrap();
    let after: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) FROM computations c WHERE output_handle <> $1 ORDER BY output_handle",
    )
    .bind(handle(5))
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        after, original,
        "discarded successes/errors leave every computation field unchanged"
    );
    let stored: Vec<Vec<u8>> = sqlx::query_scalar("SELECT handle FROM ciphertexts")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(stored, vec![handle(5)]);
    let findings: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM drifted_handle")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(findings, 1, "discarding creates no inferred finding");
    let demand: f64 = sqlx::query_scalar(
        "SELECT tx_unlock_potential FROM drifted_handle_demand WHERE handle = $1",
    )
    .bind(handle(1))
    .fetch_one(&pool)
    .await
    .unwrap();
    // Discarded cone: tx 10 (outputs 2, 3) and tx 11 (output 4); k=1. (0+2)/2.
    assert_eq!(demand, 1.0);
    let queued: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM pbs_computations WHERE handle = ANY($1::bytea[])")
            .bind(vec![handle(2), handle(3), handle(4)])
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(queued, 0);
}

#[tokio::test]
async fn late_drift_discards_successes_and_transitive_batch_results() {
    check_late_drift(true).await;
}

#[tokio::test]
async fn late_drift_discards_errors_without_terminal_stamps() {
    check_late_drift(false).await;
}

#[tokio::test]
async fn healthy_success_and_error_results_keep_the_existing_persistence_behavior() {
    let (_db, pool) = setup().await;
    computation(&pool, 2, 99, 10, true, 90).await;
    computation(&pool, 3, 99, 11, true, 90).await;
    let mut graph = DFComponentGraph::default();
    graph.results = vec![result(10, 2, true), result(11, 3, false)];
    let mut locks = dependence_chain::LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        true,
        None,
        None,
        None,
        3,
    );
    let mut trx = pool.begin().await.unwrap();
    upload_transaction_graph_results(
        &mut graph,
        &mut trx,
        &mut locks,
        false,
        &mut super::frozen_computations::Freeze::default(),
    )
    .await
    .unwrap();
    trx.commit().await.unwrap();
    let rows: Vec<(Vec<u8>, bool, bool)> = sqlx::query_as(
        "SELECT output_handle, is_completed, is_error FROM computations ORDER BY output_handle",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        rows,
        vec![(handle(2), true, false), (handle(3), false, true)]
    );
}

#[tokio::test]
async fn internal_error_does_not_stamp_a_frozen_allowed_descendant() {
    let (_db, pool) = setup().await;
    computation(&pool, 2, 99, 10, true, 90).await;
    computation(&pool, 3, 2, 10, false, 90).await;
    sqlx::query(
        "UPDATE computations SET is_allowed = FALSE, is_completed = TRUE WHERE output_handle = $1",
    )
    .bind(handle(2))
    .execute(&pool)
    .await
    .unwrap();
    let mut mask = vec![0_u8; 32];
    mask[31] = 2;
    sqlx::query(
        "UPDATE computations SET dependencies = ARRAY[$1, $2]::bytea[],
        fhe_operation = $3, operand_boundary_mask = $4 WHERE output_handle = $5",
    )
    .bind(handle(2))
    .bind(handle(1))
    .bind(SupportedFheOperations::FheAdd as i16)
    .bind(mask)
    .bind(handle(3))
    .execute(&pool)
    .await
    .unwrap();
    let ops = vec![
        DFGOp {
            outputs: vec![DFGOutput {
                handle: handle(2),
                is_allowed: false,
            }],
            fhe_op: SupportedFheOperations::FheNot,
            inputs: vec![DFGTaskInput::BoundaryDependence(handle(99))],
            is_owned: true,
        },
        DFGOp {
            outputs: vec![DFGOutput {
                handle: handle(3),
                is_allowed: true,
            }],
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::LocalDependence(handle(2)),
                DFGTaskInput::BoundaryDependence(handle(1)),
            ],
            is_owned: true,
        },
    ];
    let (mut nodes, _) = build_component_nodes(ops, &handle(10)).unwrap();
    let mut graph = DFComponentGraph::default();
    graph.build(&mut nodes).unwrap();
    graph.snapshot_blocked_dependents();
    assert_eq!(
        graph.allowed_dependents(&handle(10), &handle(2)),
        vec![handle(3)]
    );
    graph.results = vec![result(10, 2, false)];
    drift(&pool, 1).await;
    let mut trx = pool.begin().await.unwrap();
    let mut locks = dependence_chain::LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        true,
        None,
        None,
        None,
        3,
    );
    let mut freeze = super::frozen_computations::Freeze {
        batch: vec![
            super::frozen_computations::BatchRow {
                output: handle(2),
                tx: handle(10),
                deps: vec![handle(99)],
            },
            super::frozen_computations::BatchRow {
                output: handle(3),
                tx: handle(10),
                deps: vec![handle(2), handle(1)],
            },
        ],
        ..Default::default()
    };
    upload_transaction_graph_results(&mut graph, &mut trx, &mut locks, false, &mut freeze)
        .await
        .unwrap();
    trx.commit().await.unwrap();
    let pending: bool = sqlx::query_scalar(
        "SELECT NOT is_completed AND NOT is_error FROM computations WHERE output_handle = $1",
    )
    .bind(handle(3))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        pending,
        "terminal error propagation must leave frozen work pending"
    );
}
