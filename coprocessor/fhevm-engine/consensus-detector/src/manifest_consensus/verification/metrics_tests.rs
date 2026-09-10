use super::*;
use serial_test::serial;
use sqlx::PgPool;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

async fn setup_pool() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create verification metrics database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("connect verification metrics database");
    sqlx::query(
        r#"
        DELETE FROM block_manifest_verification_task
         WHERE local_manifest_id IN (
            SELECT id FROM block_manifest WHERE object_key LIKE 'metrics-test-%'
         )
        "#,
    )
    .execute(&pool)
    .await
    .expect("clear verification metric tasks");
    sqlx::query("DELETE FROM block_manifest WHERE object_key LIKE 'metrics-test-%'")
        .execute(&pool)
        .await
        .expect("clear verification metric manifests");
    (instance, pool)
}

async fn insert_local_manifest(pool: &PgPool, seed: u8) -> i64 {
    sqlx::query_scalar(
        r#"
        INSERT INTO block_manifest (
            publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision,
            manifest_digest,
            object_key,
            signed_manifest,
            manifest_source
        )
        VALUES ($1, 1, $2, 7, 42, $3, 0, $4, $5, $6, 'local')
        RETURNING id
        "#,
    )
    .bind(vec![seed; 20])
    .bind(vec![seed; 32])
    .bind(vec![seed.wrapping_add(1); 32])
    .bind(vec![seed.wrapping_add(2); 32])
    .bind(format!("metrics-test-{seed}"))
    .bind(vec![seed.wrapping_add(3)])
    .fetch_one(pool)
    .await
    .expect("insert local manifest for verification metric")
}

async fn overdue_age(pool: &PgPool) -> i64 {
    update_verification_gauges(pool)
        .await
        .expect("update verification gauges");
    OLDEST_DUE_VERIFICATION_AGE_SECONDS
        .with_label_values(&[block_manifest::LEGACY_CONSENSUS_EPOCH])
        .get()
}

#[tokio::test]
#[serial(db)]
async fn healthy_claim_does_not_count_as_overdue_verification_work() {
    let (_instance, pool) = setup_pool().await;
    let local_manifest_id = insert_local_manifest(&pool, 1).await;
    sqlx::query(
        r#"
        INSERT INTO block_manifest_verification_task (
            local_manifest_id,
            eligible_at,
            next_attempt_at,
            retry_delay_micros,
            max_attempts,
            state,
            claim_owner,
            claim_expires_at
        )
        VALUES (
            $1,
            NOW() - INTERVAL '10 minutes',
            NOW() - INTERVAL '10 minutes',
            0,
            1,
            'claimed',
            'healthy-worker',
            NOW() + INTERVAL '1 minute'
        )
        "#,
    )
    .bind(local_manifest_id)
    .execute(&pool)
    .await
    .expect("insert healthy claimed verification task");

    assert_eq!(overdue_age(&pool).await, 0);
}

#[tokio::test]
#[serial(db)]
async fn expired_claim_counts_from_its_expiry() {
    let (_instance, pool) = setup_pool().await;
    let local_manifest_id = insert_local_manifest(&pool, 2).await;
    sqlx::query(
        r#"
        INSERT INTO block_manifest_verification_task (
            local_manifest_id,
            eligible_at,
            next_attempt_at,
            retry_delay_micros,
            max_attempts,
            state,
            claim_owner,
            claim_expires_at
        )
        VALUES (
            $1,
            NOW() - INTERVAL '10 minutes',
            NOW() - INTERVAL '10 minutes',
            0,
            1,
            'claimed',
            'stalled-worker',
            NOW() - INTERVAL '10 seconds'
        )
        "#,
    )
    .bind(local_manifest_id)
    .execute(&pool)
    .await
    .expect("insert expired verification claim");

    assert!(overdue_age(&pool).await >= 10);
}

#[tokio::test]
#[serial(db)]
async fn overdue_pending_task_counts_from_its_due_time() {
    let (_instance, pool) = setup_pool().await;
    let local_manifest_id = insert_local_manifest(&pool, 3).await;
    sqlx::query(
        r#"
        INSERT INTO block_manifest_verification_task (
            local_manifest_id,
            eligible_at,
            next_attempt_at,
            retry_delay_micros,
            max_attempts
        )
        VALUES (
            $1,
            NOW() - INTERVAL '20 seconds',
            NOW() - INTERVAL '20 seconds',
            0,
            1
        )
        "#,
    )
    .bind(local_manifest_id)
    .execute(&pool)
    .await
    .expect("insert overdue pending verification task");

    assert!(overdue_age(&pool).await >= 20);
}
