use super::*;

async fn green_pool(pool: &PgPool) -> PgPool {
    sqlx::query("CREATE SCHEMA \"gcs-containment-test\"")
        .execute(pool)
        .await
        .unwrap();
    for table in [
        "blue_green_consensus_epoch",
        "computations",
        "ciphertexts",
        "ciphertext_digest",
        "handle_producer_block",
    ] {
        sqlx::query(&format!(
            "CREATE TABLE \"gcs-containment-test\".{table} (LIKE public.{table} INCLUDING ALL)"
        ))
        .execute(pool)
        .await
        .unwrap();
    }
    sqlx::query("INSERT INTO \"gcs-containment-test\".blue_green_consensus_epoch (singleton, consensus_epoch) VALUES (true, 'green')")
        .execute(pool).await.unwrap();
    sqlx::postgres::PgPoolOptions::new()
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("SET search_path TO \"gcs-containment-test\", public")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect_with(pool.connect_options().as_ref().clone())
        .await
        .unwrap()
}

#[tokio::test]
#[serial(db)]
async fn both_schemas_converge_and_local_copy_wins() {
    let (_db, blue) = setup().await;
    let green = green_pool(&blue).await;
    root(&blue, 1).await;
    store_ciphertext(&blue, 1, 1).await;
    // The independent Green copy must hide Blue's drift for this input.
    store_ciphertext(&green, 1, 9).await;
    computation(&green, 2, 1, 2, true, true).await;
    computation(&blue, 4, 1, 4, true, true).await;
    // No own copy: carry contamination to Green, then follow its descendants.
    computation(&green, 5, 4, 5, true, true).await;
    computation(&green, 6, 5, 6, true, true).await;
    let result = enforce_guaranteed_containment(&blue).await.unwrap();
    assert_eq!(
        result,
        PropagationResult {
            inferred_handles: 3,
            contained_findings: 4
        }
    );
    assert_eq!(
        epoch_flags(&blue).await,
        vec![
            (bytes(1), TEST_EPOCH.into(), true),
            (bytes(4), TEST_EPOCH.into(), true),
            (bytes(5), "green".into(), true),
            (bytes(6), "green".into(), true),
        ]
    );
    assert_eq!(
        enforce_guaranteed_containment(&green).await.unwrap(),
        PropagationResult::default()
    );
    // Transaction-local search_path changes must not leak back into either pool.
    let epoch: String =
        sqlx::query_scalar("SELECT consensus_epoch FROM blue_green_consensus_epoch")
            .fetch_one(&green)
            .await
            .unwrap();
    assert_eq!(epoch, "green");
}

#[tokio::test]
#[serial(db)]
async fn same_handle_roots_keep_both_epochs_and_green_caller_covers_blue() {
    let (_db, blue) = setup().await;
    let green = green_pool(&blue).await;
    root(&blue, 1).await;
    root(&green, 1).await;
    store_ciphertext(&blue, 1, 1).await;
    store_ciphertext(&green, 1, 2).await;
    computation(&blue, 2, 1, 2, true, true).await;
    computation(&green, 2, 1, 2, true, true).await;
    let result = enforce_guaranteed_containment(&green).await.unwrap();
    assert_eq!(
        result,
        PropagationResult {
            inferred_handles: 2,
            contained_findings: 4
        }
    );
    assert_eq!(
        epoch_flags(&blue).await,
        vec![
            (bytes(1), "green".into(), true),
            (bytes(1), TEST_EPOCH.into(), true),
            (bytes(2), "green".into(), true),
            (bytes(2), TEST_EPOCH.into(), true),
        ]
    );
}
