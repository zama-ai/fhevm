use serial_test::serial;
use sqlx::PgPool;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

const LEGACY_HANDLE: [u8; 32] = [0xa1; 32];
const UNSET_HANDLE: [u8; 32] = [0xa2; 32];

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

async fn setup_mirror_fixture() -> TestResult<(DBInstance, PgPool)> {
    let instance = setup_test_db(ImportMode::None).await?;
    let pool = PgPool::connect(instance.db_url.as_str()).await?;

    // Exercise both the multi-context and branchless INSERT mirrors.
    for block in [1_i64, 2] {
        sqlx::query(
            "INSERT INTO pbs_computations_branch
                (handle, host_chain_id, producer_block_hash, block_number, block_hash)
             VALUES ($1, 1, $2, $3, $2)",
        )
        .bind(LEGACY_HANDLE.as_slice())
        .bind(vec![block as u8])
        .bind(block)
        .execute(&pool)
        .await?;
    }
    for (handle, version) in [(LEGACY_HANDLE, Some(0_i16)), (UNSET_HANDLE, None)] {
        sqlx::query(
            "INSERT INTO ciphertext_digest
                (handle, host_chain_id, ciphertext, s3_format_version, s3_migration_failure_count)
             VALUES ($1, 1, $2, $3, 2)",
        )
        .bind(handle.as_slice())
        .bind(vec![0x11_u8])
        .bind(version)
        .execute(&pool)
        .await?;
    }
    Ok((instance, pool))
}

#[tokio::test]
#[serial(db)]
async fn s3_status_updates_do_not_wait_for_digest_mirror_stripes() -> TestResult {
    let (_instance, pool) = setup_mirror_fixture().await?;

    // A different upload can retain this stripe across S3 I/O. Hold the actual
    // trigger lock in another session to prove status-only writes bypass it.
    let mut blocker = pool.begin().await?;
    for handle in [LEGACY_HANDLE, UNSET_HANDLE] {
        sqlx::query(
            "SELECT pg_advisory_xact_lock(hashtextextended(
                '1:digest-mirror:' || abs(hashtextextended(encode($1::bytea, 'hex'), 0) % 256)::text,
                0))",
        )
        .bind(handle.as_slice())
        .execute(blocker.as_mut())
        .await?;
    }

    let updates = [
        (
            "UPDATE ciphertext_digest
             SET s3_format_version = 1, s3_migration_failure_count = 0,
                 s3_migration_last_error = NULL, s3_migration_last_error_at = NULL
             WHERE handle = $1 AND s3_format_version = 0",
            LEGACY_HANDLE,
            1_i16,
        ),
        (
            "UPDATE ciphertext_digest SET s3_format_version = 0
             WHERE handle = $1 AND s3_format_version IS NULL
               AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)",
            UNSET_HANDLE,
            0,
        ),
        (
            "UPDATE ciphertext_digest
             SET s3_migration_failure_count = s3_migration_failure_count + 1,
                 s3_migration_last_error = 'test failure', s3_migration_last_error_at = now()
             WHERE handle = $1",
            LEGACY_HANDLE,
            0,
        ),
    ];
    for (update, handle, expected_version) in updates {
        let mut transaction = pool.begin().await?;
        sqlx::query("SET LOCAL lock_timeout = '1s'")
            .execute(transaction.as_mut())
            .await?;
        // The migrator retains its row lock until after the S3-status UPDATE.
        sqlx::query("SELECT 1 FROM ciphertext_digest WHERE handle = $1 FOR UPDATE SKIP LOCKED")
            .bind(handle.as_slice())
            .fetch_one(transaction.as_mut())
            .await?;
        let result = sqlx::query(update)
            .bind(handle.as_slice())
            .execute(transaction.as_mut())
            .await?;
        assert_eq!(result.rows_affected(), 1);

        let version: Option<i16> =
            sqlx::query_scalar("SELECT s3_format_version FROM ciphertext_digest WHERE handle = $1")
                .bind(handle.as_slice())
                .fetch_one(transaction.as_mut())
                .await?;
        assert_eq!(version, Some(expected_version));
        let acquired_stripe: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM pg_locks
             WHERE pid = pg_backend_pid() AND locktype = 'advisory')",
        )
        .fetch_one(transaction.as_mut())
        .await?;
        assert!(
            !acquired_stripe,
            "S3 bookkeeping must not acquire a mirror stripe"
        );
        transaction.rollback().await?;
    }
    blocker.rollback().await?;
    pool.close().await;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn digest_mirrors_still_copy_uploads_and_transaction_status() -> TestResult {
    let (_instance, pool) = setup_mirror_fixture().await?;
    let contexts: i64 =
        sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest_branch WHERE handle = $1")
            .bind(LEGACY_HANDLE.as_slice())
            .fetch_one(&pool)
            .await?;
    assert_eq!(contexts, 2, "INSERT must populate both branch contexts");

    let mut transaction = pool.begin().await?;
    sqlx::query(
        "UPDATE ciphertext_digest SET ciphertext128 = $1, s3_format_version = 1 WHERE handle = $2",
    )
    .bind(vec![0x22_u8])
    .bind(LEGACY_HANDLE.as_slice())
    .execute(transaction.as_mut())
    .await?;
    let acquired_stripe: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM pg_locks
         WHERE pid = pg_backend_pid() AND locktype = 'advisory' AND granted)",
    )
    .fetch_one(transaction.as_mut())
    .await?;
    assert!(
        acquired_stripe,
        "upload digest changes must retain mirror synchronization"
    );
    sqlx::query(
        "UPDATE ciphertext_digest SET txn_is_sent = TRUE, txn_hash = $1, txn_block_number = 3
         WHERE handle = $2",
    )
    .bind(vec![0x33_u8])
    .bind(LEGACY_HANDLE.as_slice())
    .execute(transaction.as_mut())
    .await?;
    let mirrored: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM ciphertext_digest_branch
         WHERE handle = $1 AND ciphertext128 = $2
           AND txn_is_sent AND txn_hash = $3 AND txn_block_number = 3",
    )
    .bind(LEGACY_HANDLE.as_slice())
    .bind(vec![0x22_u8])
    .bind(vec![0x33_u8])
    .fetch_one(transaction.as_mut())
    .await?;
    assert_eq!(
        mirrored, 2,
        "uploads and transaction status must reach every context"
    );
    transaction.rollback().await?;

    let branchless_before: i64 =
        sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest_branch WHERE handle = $1")
            .bind(UNSET_HANDLE.as_slice())
            .fetch_one(&pool)
            .await?;
    assert_eq!(
        branchless_before, 1,
        "INSERT must populate the branchless mirror"
    );
    sqlx::query("DELETE FROM ciphertext_digest WHERE handle = $1")
        .bind(UNSET_HANDLE.as_slice())
        .execute(&pool)
        .await?;
    let branchless_remaining: i64 =
        sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest_branch WHERE handle = $1")
            .bind(UNSET_HANDLE.as_slice())
            .fetch_one(&pool)
            .await?;
    assert_eq!(
        branchless_remaining, 0,
        "DELETE must remove the branchless mirror"
    );
    pool.close().await;
    Ok(())
}
