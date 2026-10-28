use super::*;
use clap::Parser;
use serial_test::serial;
use std::collections::HashSet;
use test_harness::instance::{setup_test_db, ImportMode};

fn handle(value: u8) -> Handle {
    let mut handle = vec![value; 32];
    handle[30] = 4; // euint32
    handle[31] = current_ciphertext_version() as u8;
    handle
}

/// Exercise the production work query, result upload and ciphertext fetch.
/// No FHE execution is needed to check that a false cycle leaves its rows
/// pending and can be resolved after the original producer reaches storage.
#[tokio::test]
#[serial(db)]
async fn cross_transaction_cycle_leaves_work_pending_for_retry(
) -> Result<(), Box<dyn std::error::Error>> {
    let instance = setup_test_db(ImportMode::None).await?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    let args = crate::daemon_cli::Args::parse_from(["tfhe_worker"]);
    let health = crate::health_check::HealthCheck::new(instance.db_url.clone());
    let mut locks = dependence_chain::LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        true,
        None,
        None,
        None,
    );

    // The original producer t0 is outside this batch. t1 consumes its h1;
    // t2 consumes t1's h3 and independently produces the same constant h1.
    for (transaction, output, op, dependencies, scalar) in [
        (
            0x11,
            0x03,
            SupportedFheOperations::FheAdd,
            vec![handle(0x01), handle(0x01)],
            false,
        ),
        (
            0x22,
            0x01,
            SupportedFheOperations::FheTrivialEncrypt,
            vec![vec![0; 32], vec![4]],
            true,
        ),
        (
            0x22,
            0x04,
            SupportedFheOperations::FheAdd,
            vec![handle(0x01), handle(0x03)],
            false,
        ),
    ] {
        sqlx::query(
            "INSERT INTO computations (
                output_handle, transaction_id, dependence_chain_id, dependencies,
                fhe_operation, is_scalar, is_allowed, is_completed, host_chain_id
             ) VALUES ($1, $2, $3, $4, $5, $6, TRUE, FALSE, 42)",
        )
        .bind(handle(output))
        .bind(handle(transaction))
        .bind(handle(0x11))
        .bind(dependencies)
        .bind(op as i16)
        .bind(scalar)
        .execute(&pool)
        .await?;
    }

    let mut tx = pool.begin().await?;
    let (mut components, _, has_work) = query_for_work(&args, &health, &mut tx, &mut locks).await?;
    assert!(has_work);
    let mut graph = DFComponentGraph::default();
    graph.build(&mut components)?;
    // The worker uploads recorded results even if resolve_dependences returns
    // an error. On the old scheduler this persisted CyclicDependence forever.
    let _ = graph.resolve_dependences(&HashSet::new());
    assert!(!upload_transaction_graph_results(&mut graph, &mut tx, &mut locks).await?);
    tx.commit().await?;

    let states: Vec<(bool, bool, Option<String>)> =
        sqlx::query_as("SELECT is_error, is_completed, error_message FROM computations")
            .fetch_all(&pool)
            .await?;
    assert_eq!(states.len(), 3);
    assert!(
        states
            .iter()
            .all(|(error, completed, message)| { !error && !completed && message.is_none() }),
        "deferred computations must have no persisted error or completion: {states:?}"
    );

    // The next work query must select both transactions again.
    let mut tx = pool.begin().await?;
    let (mut components, _, has_work) = query_for_work(&args, &health, &mut tx, &mut locks).await?;
    assert!(has_work);
    let selected: HashSet<_> = components
        .iter()
        .map(|c| c.transaction_id.clone())
        .collect();
    assert_eq!(selected, HashSet::from([handle(0x11), handle(0x22)]));

    // Stand in for t0's completed upload. The fetch and graph resolver treat
    // ciphertext bytes as opaque; this test deliberately does not decrypt them.
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, $3, 4)",
    )
    .bind(handle(0x01))
    .bind(Vec::<u8>::new())
    .bind(current_ciphertext_version())
    .execute(&mut *tx)
    .await?;
    let mut retry = DFComponentGraph::default();
    retry.build(&mut components)?;
    let needed: Vec<_> = retry.needed_map.keys().cloned().collect();
    let ciphertexts = query_ciphertexts(&needed, &mut tx, false).await?;
    let fetched: HashSet<_> = ciphertexts.keys().cloned().collect();
    assert!(fetched.contains(&handle(0x01)));
    for (handle, (ct_type, ct_bytes)) in ciphertexts {
        retry.add_input(
            &handle,
            &DFGTxInput::Compressed((CompressedCiphertext { ct_type, ct_bytes }, true)),
        )?;
    }
    retry.resolve_dependences(&fetched)?;
    assert!(retry.get_results().is_empty());
    assert!(retry
        .graph
        .graph()
        .node_weights()
        .all(|c| !c.is_uncomputable));
    tx.rollback().await?;
    pool.close().await;
    Ok(())
}
