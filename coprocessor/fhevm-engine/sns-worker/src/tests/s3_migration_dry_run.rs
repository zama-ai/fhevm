use std::{sync::Arc, time::Duration};

use alloy::signers::local::PrivateKeySigner;
use serial_test::serial;
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::time::timeout;

use crate::{
    s3_migration::S3MigrationConfig, s3_migration_dry_run::run_startup_migration_dry_run,
    S3MigrationMode,
};

use super::{build_test_config, init_tracing, setup_localstack};

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_dry_run_does_not_record_s3_migration_progress() {
    init_tracing();

    let db_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let mut conf = build_test_config(db_instance.db_url.clone(), true);
    conf.s3_migration = S3MigrationMode::DryRun;
    conf.health_checks.port = test_harness::localstack::pick_free_port();
    conf.s3.retry_policy.max_retries_per_upload = 1;
    conf.s3.retry_policy.max_backoff = Duration::from_millis(10);
    conf.s3.retry_policy.max_retries_timeout = Duration::from_secs(2);

    let (_s3_instance, s3_client) = setup_localstack(&conf).await.expect("valid localstack");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.db.max_connections)
        .acquire_timeout(conf.db.timeout)
        .connect(conf.db.url.as_str())
        .await
        .expect("connect test db");

    let signer = PrivateKeySigner::random();
    conf.private_key = Some(hex::encode(signer.to_bytes()));

    let handle = vec![0x24; 32];
    let ct64_digest = vec![0x42; 32];
    let key_id_gw = vec![0x07u8; 32];
    let host_chain_id = 1_i64;
    sqlx::query!(
        r#"
        INSERT INTO ciphertext_digest(
            host_chain_id,
            key_id_gw,
            handle,
            ciphertext,
            s3_format_version
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
        host_chain_id,
        &key_id_gw,
        &handle,
        &ct64_digest,
        Option::<i16>::None,
    )
    .execute(&pool)
    .await
    .expect("insert legacy ciphertext_digest row");

    let run_result = timeout(
        Duration::from_secs(15),
        crate::run_all(conf.clone(), db_instance.parent_token.child_token(), None),
    )
    .await
    .expect("dry-run should finish");

    run_result.expect("dry-run should not fail on missing ciphertext object");

    let migration_config = S3MigrationConfig {
        batch_size: conf.s3_migration_max_concurrent_handles.into(),
        signer: Arc::new(signer),
        s3: conf.s3.clone(),
        mode: conf.s3_migration,
        sleep_duration: conf.s3_migration_sleep_duration,
        max_retries: conf.s3_migration_max_retries,
    };
    let report = run_startup_migration_dry_run(&migration_config, &pool, &s3_client)
        .await
        .expect("inspect NULL-version candidate without backfilling");
    assert_eq!(report.handles_scanned, 1);

    let row = sqlx::query!(
        r#"
        SELECT s3_format_version,
               s3_migration_failure_count as "s3_migration_failure_count!",
               s3_migration_last_error
         FROM ciphertext_digest
         WHERE handle = $1
        "#,
        &handle,
    )
    .fetch_one(&pool)
    .await
    .expect("fetch dry-run row");

    assert!(row.s3_format_version.is_none());
    assert_eq!(row.s3_migration_failure_count, 0);
    assert!(row.s3_migration_last_error.is_none());
}
