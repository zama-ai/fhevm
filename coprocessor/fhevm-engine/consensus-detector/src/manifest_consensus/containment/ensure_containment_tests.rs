use super::*;

#[tokio::test]
#[serial(db)]
async fn explicit_calls_retry_failed_propagation_and_skip_completed_work() {
    let (_instance, pool) = setup().await;
    let id = insert_root(&pool, 1).await;
    // A failed lookup rolls back without acknowledging containment.
    assert!(enforce_guaranteed_containment(&pool).await.is_err());
    let contained: bool =
        sqlx::query_scalar("SELECT is_contained FROM drifted_handle WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(!contained);
    producer(&pool, 1, 1).await;
    assert_eq!(
        enforce_guaranteed_containment(&pool)
            .await
            .unwrap()
            .contained_findings,
        1
    );
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );

    // Already contained: skip the exclusive barrier even if a batch holds it.
    let mut batch = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .execute(batch.as_mut())
        .await
        .unwrap();
    assert_eq!(
        tokio::time::timeout(
            Duration::from_secs(1),
            enforce_guaranteed_containment(&pool)
        )
        .await
        .unwrap()
        .unwrap(),
        PropagationResult::default()
    );
    batch.rollback().await.unwrap();
}

#[tokio::test]
#[serial(db)]
async fn pending_check_ignores_ct128_and_unconsumed_other_epochs() {
    let (_instance, pool) = setup().await;
    direct_root(&pool, 1, "ct128_mismatch").await;
    let id = root(&pool, 2).await;
    sqlx::query("UPDATE drifted_handle SET consensus_epoch = 'other-epoch' WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    let result = enforce_guaranteed_containment(&pool).await.unwrap();
    assert_eq!(result.inferred_handles, 0);
}

#[tokio::test]
#[serial(db)]
async fn pending_check_runs_for_consumed_foreign_epoch_ct64() {
    let (_instance, pool) = setup().await;
    let id = root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    sqlx::query("UPDATE drifted_handle SET consensus_epoch = 'other-epoch' WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        enforce_guaranteed_containment(&pool)
            .await
            .unwrap()
            .inferred_handles,
        1
    );
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
}

#[tokio::test]
#[serial(db)]
async fn concurrent_containment_calls_are_idempotent() {
    let (_instance, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    let (first, second) = tokio::join!(
        enforce_guaranteed_containment(&pool),
        enforce_guaranteed_containment(&pool)
    );
    first.unwrap();
    second.unwrap();
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM drifted_handle")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 2);
}

#[tokio::test]
#[serial(db)]
async fn epoch_change_between_passes_leaves_old_findings_uncontained() {
    let (_instance, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    let other = root(&pool, 3).await;
    sqlx::query("UPDATE drifted_handle SET consensus_epoch = 'other-epoch' WHERE id = $1")
        .bind(other)
        .execute(&pool)
        .await
        .unwrap();
    let mut batch = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .execute(batch.as_mut())
        .await
        .unwrap();
    let task_pool = pool.clone();
    let containment = tokio::spawn(async move { enforce_guaranteed_containment(&task_pool).await });
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM pg_locks WHERE locktype = 'advisory' AND mode = 'ExclusiveLock' AND NOT granted AND objid = ($1::bigint & 4294967295)::oid)",
            )
            .bind(DRIFT_CONTAINMENT_BARRIER)
            .fetch_one(&pool)
            .await
            .unwrap();
            if waiting {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("guaranteed pass must wait on the barrier");
    sqlx::query(
        "UPDATE blue_green_consensus_epoch SET consensus_epoch = 'other-epoch' WHERE singleton",
    )
    .execute(&pool)
    .await
    .unwrap();
    batch.commit().await.unwrap();
    let result = tokio::time::timeout(Duration::from_secs(5), containment)
        .await
        .unwrap()
        .unwrap();
    assert!(result.is_err());
    assert_eq!(
        flags(&pool).await,
        vec![(bytes(1), false), (bytes(2), false), (bytes(3), false)]
    );
    let recovered = enforce_guaranteed_containment(&pool).await.unwrap();
    assert_eq!(recovered.contained_findings, 4);
    let stale: Vec<(Vec<u8>, String, bool)> = sqlx::query_as(
        "SELECT handle, consensus_epoch, is_contained FROM drifted_handle
         WHERE consensus_epoch = $1 ORDER BY handle",
    )
    .bind(TEST_EPOCH)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        stale,
        vec![
            (bytes(1), TEST_EPOCH.into(), true),
            (bytes(2), TEST_EPOCH.into(), true)
        ]
    );
    let live: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM drifted_handle
          WHERE consensus_epoch = 'other-epoch' AND is_contained",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(live, 2);
}
