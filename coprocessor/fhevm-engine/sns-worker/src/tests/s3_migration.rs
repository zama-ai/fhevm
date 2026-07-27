use std::{collections::HashMap, sync::Arc, time::Duration};

use alloy::signers::local::PrivateKeySigner;
use aws_sdk_s3::primitives::ByteStream;
use fhevm_engine_common::types::CoproSigner;
use serial_test::serial;
use test_harness::{
    db_utils::insert_ciphertext64,
    instance::{setup_test_db, DBInstance, ImportMode},
    localstack::LocalstackContainer,
};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::{
    s3_migration::{run_startup_migrations, S3MigrationConfig},
    Ciphertext128Format, Config, S3MigrationMode, S3_FORMAT_VERSION_V0, S3_FORMAT_VERSION_V1,
};

use super::{
    build_test_config, fetch_host_chain_id, fetch_latest_key_id_gw, init_tracing, setup_localstack,
};

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_returns_s3_migration_error() {
    init_tracing();

    let db_instance = setup_test_db(ImportMode::WithAllKeys)
        .await
        .expect("valid db instance");
    let mut conf = build_test_config(db_instance.db_url.clone(), true);
    conf.s3_migration = S3MigrationMode::BeforeAndQuit;
    conf.health_checks.port = test_harness::localstack::pick_free_port();
    conf.s3_migration_max_retries = 2;
    conf.s3.retry_policy.max_retries_per_upload = 1;
    conf.s3.retry_policy.max_backoff = Duration::from_millis(10);
    conf.s3.retry_policy.max_retries_timeout = Duration::from_secs(2);

    let (_s3_instance, _s3_client) = setup_localstack(&conf).await.expect("valid localstack");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.db.max_connections)
        .acquire_timeout(conf.db.timeout)
        .connect(conf.db.url.as_str())
        .await
        .expect("connect test db");

    let signer = PrivateKeySigner::random();
    conf.private_key = Some(hex::encode(signer.to_bytes()));

    let handle = vec![0x42; 32];
    let ct64_digest = vec![0x24; 32];
    let key_id_gw = fetch_latest_key_id_gw(&pool).await;
    let host_chain_id = fetch_host_chain_id(&pool).await;
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
        S3_FORMAT_VERSION_V0,
    )
    .execute(&pool)
    .await
    .expect("insert legacy ciphertext_digest row");

    let run_result = timeout(
        Duration::from_secs(15),
        crate::run_all(conf, db_instance.parent_token.child_token(), None),
    )
    .await
    .expect("before-and-quit should finish");

    let err = run_result.expect_err("before-and-quit should return the S3 migration error");
    assert!(
        err.to_string().contains("after reaching max retry count 2"),
        "unexpected before-and-quit error: {err}"
    );

    let failure = sqlx::query!(
        r#"
        SELECT s3_migration_failure_count,
               s3_migration_last_error
         FROM ciphertext_digest
         WHERE handle = $1
        "#,
        &handle,
    )
    .fetch_one(&pool)
    .await
    .expect("fetch recorded migration failure");
    assert_eq!(failure.s3_migration_failure_count, 2);
    assert!(
        failure
            .s3_migration_last_error
            .as_deref()
            .is_some_and(|err| err.contains("missing ct64 object")),
        "unexpected recorded migration error: {:?}",
        failure.s3_migration_last_error,
    );
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_migrates_ct64_from_legacy_digest_key() {
    init_tracing();

    let db_instance = setup_test_db(ImportMode::WithAllKeys)
        .await
        .expect("valid db instance");
    let mut conf = build_test_config(db_instance.db_url.clone(), true);
    conf.s3_migration = S3MigrationMode::BeforeAndQuit;
    conf.health_checks.port = test_harness::localstack::pick_free_port();
    conf.s3_migration_max_retries = 1;
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

    let handle = vec![0x42; 32];
    let ct64_bytes = b"legacy ct64 object bytes".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    let key_id_gw = fetch_latest_key_id_gw(&pool).await;
    let host_chain_id = fetch_host_chain_id(&pool).await;
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
        S3_FORMAT_VERSION_V0,
    )
    .execute(&pool)
    .await
    .expect("insert legacy ciphertext_digest row");

    let legacy_digest_key = hex::encode(&ct64_digest);
    s3_client
        .put_object()
        .bucket(&conf.s3.bucket_ct64)
        .key(&legacy_digest_key)
        .body(ByteStream::from(ct64_bytes))
        .send()
        .await
        .expect("upload legacy ct64 digest-key object");

    let current_key =
        crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1);
    let bucket_ct64 = conf.s3.bucket_ct64.clone();

    let run_result = timeout(
        Duration::from_secs(15),
        crate::run_all(conf, db_instance.parent_token.child_token(), None),
    )
    .await
    .expect("before-and-quit should finish");

    run_result.expect("before-and-quit should migrate legacy digest-key ct64 object");

    s3_client
        .head_object()
        .bucket(bucket_ct64)
        .key(current_key)
        .send()
        .await
        .expect("current ct64 object should exist after migration");

    let row = sqlx::query!(
        r#"
        SELECT s3_format_version as "s3_format_version!",
               s3_migration_failure_count as "s3_migration_failure_count!",
               s3_migration_last_error
         FROM ciphertext_digest
         WHERE handle = $1
        "#,
        &handle,
    )
    .fetch_one(&pool)
    .await
    .expect("fetch migrated row");

    assert_eq!(row.s3_format_version, S3_FORMAT_VERSION_V1);
    assert_eq!(row.s3_migration_failure_count, 0);
    assert!(row.s3_migration_last_error.is_none());
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_migrates_ct64_and_ct128_from_legacy_digest_keys() {
    init_tracing();

    let db_instance = setup_test_db(ImportMode::WithAllKeys)
        .await
        .expect("valid db instance");
    let mut conf = build_test_config(db_instance.db_url.clone(), true);
    conf.s3_migration = S3MigrationMode::BeforeAndQuit;
    conf.health_checks.port = test_harness::localstack::pick_free_port();
    conf.s3_migration_max_retries = 1;
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

    let handle = vec![0x7a; 32];
    let ct64_bytes = b"legacy ct64 object bytes for full migration".to_vec();
    let ct128_bytes = b"legacy ct128 object bytes for full migration".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    let ct128_digest = crate::aws_upload::compute_digest(&ct128_bytes);
    let ct128_format: i16 = Ciphertext128Format::CompressedOnCpu.into();
    let key_id_gw = fetch_latest_key_id_gw(&pool).await;
    let host_chain_id = fetch_host_chain_id(&pool).await;
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
        S3_FORMAT_VERSION_V0,
    )
    .execute(&pool)
    .await
    .expect("insert legacy ciphertext_digest row");
    sqlx::query!(
        "UPDATE ciphertext_digest
            SET ciphertext128 = $1, ciphertext128_format = $2
            WHERE handle = $3",
        &ct128_digest,
        ct128_format,
        &handle,
    )
    .execute(&pool)
    .await
    .expect("set legacy ct128 digest and format");

    let ct64_legacy_digest_key = hex::encode(&ct64_digest);
    s3_client
        .put_object()
        .bucket(&conf.s3.bucket_ct64)
        .key(&ct64_legacy_digest_key)
        .body(ByteStream::from(ct64_bytes))
        .send()
        .await
        .expect("upload legacy ct64 digest-key object");

    let ct128_legacy_digest_key = hex::encode(&ct128_digest);
    s3_client
        .put_object()
        .bucket(&conf.s3.bucket_ct128)
        .key(&ct128_legacy_digest_key)
        .body(ByteStream::from(ct128_bytes))
        .send()
        .await
        .expect("upload legacy ct128 digest-key object");

    let current_key =
        crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1);
    let bucket_ct64 = conf.s3.bucket_ct64.clone();
    let bucket_ct128 = conf.s3.bucket_ct128.clone();

    let run_result = timeout(
        Duration::from_secs(15),
        crate::run_all(conf, db_instance.parent_token.child_token(), None),
    )
    .await
    .expect("before-and-quit should finish");

    run_result.expect("before-and-quit should migrate legacy ct64 and ct128 objects");

    s3_client
        .head_object()
        .bucket(bucket_ct64)
        .key(&current_key)
        .send()
        .await
        .expect("current ct64 object should exist after migration");

    let current_ct128 = s3_client
        .head_object()
        .bucket(&bucket_ct128)
        .key(&current_key)
        .send()
        .await
        .expect("current ct128 object should exist after migration");
    assert_metadata_eq(
        current_ct128.metadata(),
        "Ct-Format",
        &Ciphertext128Format::CompressedOnCpu.to_string(),
    );

    let digest_key_ct128 = s3_client
        .head_object()
        .bucket(&bucket_ct128)
        .key(&ct128_legacy_digest_key)
        .send()
        .await
        .expect("ct128 digest-key object should be rewritten with current metadata");
    assert_metadata_eq(
        digest_key_ct128.metadata(),
        "Ct-Format",
        &Ciphertext128Format::CompressedOnCpu.to_string(),
    );

    let upload_state = sqlx::query!(
        "SELECT ciphertext, ciphertext128, s3_format_version
             FROM ciphertext_digest
             WHERE handle = $1",
        &handle
    )
    .fetch_one(&pool)
    .await
    .expect("fetch migrated upload state");
    assert_eq!(
        upload_state.ciphertext.as_deref(),
        Some(ct64_digest.as_slice())
    );
    assert_eq!(
        upload_state.ciphertext128.as_deref(),
        Some(ct128_digest.as_slice())
    );
    assert_eq!(upload_state.s3_format_version, Some(S3_FORMAT_VERSION_V1));

    let migration_state = sqlx::query!(
        r#"
        SELECT s3_format_version as "s3_format_version!",
               s3_migration_failure_count as "s3_migration_failure_count!",
               s3_migration_last_error
         FROM ciphertext_digest
         WHERE handle = $1
        "#,
        &handle,
    )
    .fetch_one(&pool)
    .await
    .expect("fetch migrated retry state");
    assert_eq!(migration_state.s3_format_version, S3_FORMAT_VERSION_V1);
    assert_eq!(migration_state.s3_migration_failure_count, 0);
    assert!(migration_state.s3_migration_last_error.is_none());
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_records_corrupted_ct64_object_error() {
    init_tracing();

    let env = setup_direct_migration_env().await;
    let handle = vec![0x34; 32];
    let ct64_bytes = b"expected ct64 bytes".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    put_object(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &hex::encode(&ct64_digest),
        b"corrupted ct64 bytes".to_vec(),
    )
    .await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    let err = run_result.expect_err("corrupted ct64 object should fail migration");
    assert!(
        err.to_string().contains("after reaching max retry count 1"),
        "unexpected migration error: {err}"
    );
    assert_recorded_failure(&env.pool, &handle, 1, "missing ct64 object").await;
    assert_object_missing(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1),
    )
    .await;
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_records_corrupted_ct128_object_error() {
    init_tracing();

    let env = setup_direct_migration_env().await;
    let handle = vec![0x35; 32];
    let ct64_bytes = b"ct64 bytes for corrupted ct128 test".to_vec();
    let ct128_bytes = b"expected ct128 bytes".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    let ct128_digest = crate::aws_upload::compute_digest(&ct128_bytes);
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    set_ct128_digest_and_format(
        &env.pool,
        &handle,
        &ct128_digest,
        Ciphertext128Format::CompressedOnCpu.into(),
    )
    .await;
    put_object(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &hex::encode(&ct64_digest),
        ct64_bytes,
    )
    .await;
    put_object(
        &env.s3_client,
        &env.conf.s3.bucket_ct128,
        &hex::encode(&ct128_digest),
        b"corrupted ct128 bytes".to_vec(),
    )
    .await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    let err = run_result.expect_err("corrupted ct128 object should fail migration");
    assert!(
        err.to_string().contains("after reaching max retry count 1"),
        "unexpected migration error: {err}"
    );
    assert_recorded_failure(&env.pool, &handle, 1, "missing ct128 object").await;
    assert_object_missing(
        &env.s3_client,
        &env.conf.s3.bucket_ct128,
        &crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1),
    )
    .await;
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_rebuilds_ct64_from_db_when_legacy_object_is_missing() {
    init_tracing();

    let env = setup_direct_migration_env().await;
    let handle = vec![0x36; 32];
    let ct64_bytes = b"ct64 bytes recovered from db".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    insert_ct64_bytes(&env.pool, &handle, &ct64_bytes).await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    run_result.expect("missing legacy ct64 object should be rebuilt from DB bytes");

    let current_key =
        crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1);
    assert_object_body_eq(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &current_key,
        &ct64_bytes,
    )
    .await;
    assert_migrated_retry_state(&env.pool, &handle).await;
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_rebuilds_ct128_from_db_when_legacy_object_is_missing() {
    init_tracing();

    let env = setup_direct_migration_env().await;
    let handle = vec![0x37; 32];
    let ct64_bytes = b"ct64 bytes for ct128 db fallback".to_vec();
    let ct128_bytes = b"ct128 bytes recovered from db".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    let ct128_digest = crate::aws_upload::compute_digest(&ct128_bytes);
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    set_ct128_digest_and_format(
        &env.pool,
        &handle,
        &ct128_digest,
        Ciphertext128Format::CompressedOnCpu.into(),
    )
    .await;
    put_object(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &hex::encode(&ct64_digest),
        ct64_bytes,
    )
    .await;
    insert_ct128_bytes(&env.pool, &handle, &ct128_bytes).await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    run_result.expect("missing legacy ct128 object should be rebuilt from DB bytes");

    let current_key =
        crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1);
    assert_object_body_eq(
        &env.s3_client,
        &env.conf.s3.bucket_ct128,
        &current_key,
        &ct128_bytes,
    )
    .await;
    assert_object_body_eq(
        &env.s3_client,
        &env.conf.s3.bucket_ct128,
        &hex::encode(&ct128_digest),
        &ct128_bytes,
    )
    .await;
    let current_ct128 = env
        .s3_client
        .head_object()
        .bucket(&env.conf.s3.bucket_ct128)
        .key(&current_key)
        .send()
        .await
        .expect("current ct128 object should exist after migration");
    assert_metadata_eq(
        current_ct128.metadata(),
        "Ct-Format",
        &Ciphertext128Format::CompressedOnCpu.to_string(),
    );
    assert_migrated_retry_state(&env.pool, &handle).await;
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_rewrites_current_objects_with_stale_metadata() {
    init_tracing();

    let env = setup_direct_migration_env().await;
    let handle = vec![0x38; 32];
    let ct64_bytes = b"ct64 bytes for stale metadata rewrite".to_vec();
    let ct128_bytes = b"ct128 bytes for stale metadata rewrite".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    let ct128_digest = crate::aws_upload::compute_digest(&ct128_bytes);
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    set_ct128_digest_and_format(
        &env.pool,
        &handle,
        &ct128_digest,
        Ciphertext128Format::CompressedOnCpu.into(),
    )
    .await;

    let current_key =
        crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1);
    put_object_with_stale_metadata(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &current_key,
        ct64_bytes.clone(),
    )
    .await;
    put_object(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &hex::encode(&ct64_digest),
        ct64_bytes.clone(),
    )
    .await;
    put_object_with_stale_metadata(
        &env.s3_client,
        &env.conf.s3.bucket_ct128,
        &current_key,
        ct128_bytes.clone(),
    )
    .await;
    put_object_with_stale_metadata(
        &env.s3_client,
        &env.conf.s3.bucket_ct128,
        &hex::encode(&ct128_digest),
        ct128_bytes.clone(),
    )
    .await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    run_result.expect("stale current metadata should be rewritten");

    let current_ct64 = env
        .s3_client
        .head_object()
        .bucket(&env.conf.s3.bucket_ct64)
        .key(&current_key)
        .send()
        .await
        .expect("current ct64 object should exist after metadata rewrite");
    assert_metadata_eq(current_ct64.metadata(), "Uploaded-By", "sns-worker");

    let current_ct128 = env
        .s3_client
        .head_object()
        .bucket(&env.conf.s3.bucket_ct128)
        .key(&current_key)
        .send()
        .await
        .expect("current ct128 object should exist after metadata rewrite");
    assert_metadata_eq(
        current_ct128.metadata(),
        "Ct-Format",
        &Ciphertext128Format::CompressedOnCpu.to_string(),
    );

    let digest_ct128 = env
        .s3_client
        .head_object()
        .bucket(&env.conf.s3.bucket_ct128)
        .key(hex::encode(&ct128_digest))
        .send()
        .await
        .expect("ct128 digest-key object should exist after metadata rewrite");
    assert_metadata_eq(
        digest_ct128.metadata(),
        "Ct-Format",
        &Ciphertext128Format::CompressedOnCpu.to_string(),
    );

    assert_object_body_eq(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &current_key,
        &ct64_bytes,
    )
    .await;
    assert_object_body_eq(
        &env.s3_client,
        &env.conf.s3.bucket_ct128,
        &current_key,
        &ct128_bytes,
    )
    .await;
    assert_migrated_retry_state(&env.pool, &handle).await;
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_records_invalid_ct128_format_error() {
    init_tracing();

    let env = setup_direct_migration_env().await;
    let handle = vec![0x31; 32];
    let ct64_digest = vec![0x11; 32];
    let ct128_digest = vec![0x12; 32];
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    set_ct128_digest_and_format(
        &env.pool,
        &handle,
        &ct128_digest,
        Ciphertext128Format::Unknown.into(),
    )
    .await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    let err = run_result.expect_err("invalid ct128 format should fail migration");
    assert!(
        err.to_string().contains("after reaching max retry count 1"),
        "unexpected migration error: {err}"
    );
    assert_recorded_failure(&env.pool, &handle, 1, "invalid ciphertext128_format 0").await;
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_records_missing_ct128_object_error() {
    init_tracing();

    let env = setup_direct_migration_env().await;
    let handle = vec![0x32; 32];
    let ct64_bytes = b"ct64 bytes for missing ct128 test".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    let ct128_digest = vec![0x13; 32];
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    set_ct128_digest_and_format(
        &env.pool,
        &handle,
        &ct128_digest,
        Ciphertext128Format::CompressedOnCpu.into(),
    )
    .await;
    put_object(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &hex::encode(&ct64_digest),
        ct64_bytes,
    )
    .await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    let err = run_result.expect_err("missing ct128 object should fail migration");
    assert!(
        err.to_string().contains("after reaching max retry count 1"),
        "unexpected migration error: {err}"
    );
    assert_recorded_failure(&env.pool, &handle, 1, "missing ct128 object").await;
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_before_and_quit_retries_recorded_failure_and_clears_error() {
    init_tracing();

    let mut env = setup_direct_migration_env().await;
    env.conf.s3_migration_max_retries = 2;
    let handle = vec![0x33; 32];
    let ct64_bytes = b"ct64 bytes for retry success".to_vec();
    let ct64_digest = crate::aws_upload::compute_digest(&ct64_bytes);
    insert_legacy_ct64_digest_row(&env.pool, &handle, &ct64_digest).await;
    record_existing_failure(&env.pool, &handle, "previous missing ct64 object").await;
    put_object(
        &env.s3_client,
        &env.conf.s3.bucket_ct64,
        &hex::encode(&ct64_digest),
        ct64_bytes,
    )
    .await;

    let run_result = timeout(Duration::from_secs(15), run_direct_migration(&env))
        .await
        .expect("before-and-quit migration should finish");

    run_result.expect("recorded failure should be retried and migrated");

    let current_key =
        crate::aws_upload::s3_ciphertext_key(&handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1);
    env.s3_client
        .head_object()
        .bucket(&env.conf.s3.bucket_ct64)
        .key(current_key)
        .send()
        .await
        .expect("current ct64 object should exist after retry");

    let migration_state = sqlx::query!(
        r#"
        SELECT s3_format_version as "s3_format_version!",
               s3_migration_failure_count as "s3_migration_failure_count!",
               s3_migration_last_error
         FROM ciphertext_digest
         WHERE handle = $1
        "#,
        &handle,
    )
    .fetch_one(&env.pool)
    .await
    .expect("fetch migrated retry state");
    assert_eq!(migration_state.s3_format_version, S3_FORMAT_VERSION_V1);
    assert_eq!(migration_state.s3_migration_failure_count, 0);
    assert!(migration_state.s3_migration_last_error.is_none());
}

struct DirectMigrationEnv {
    _db_instance: DBInstance,
    _s3_instance: Option<Arc<LocalstackContainer>>,
    conf: Config,
    pool: sqlx::PgPool,
    s3_client: aws_sdk_s3::Client,
    signer: CoproSigner,
    token: CancellationToken,
}

async fn setup_direct_migration_env() -> DirectMigrationEnv {
    let db_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let mut conf = build_test_config(db_instance.db_url.clone(), true);
    conf.s3_migration = S3MigrationMode::BeforeAndQuit;
    conf.health_checks.port = test_harness::localstack::pick_free_port();
    conf.s3_migration_max_retries = 1;
    conf.s3.retry_policy.max_retries_per_upload = 1;
    conf.s3.retry_policy.max_backoff = Duration::from_millis(10);
    conf.s3.retry_policy.max_retries_timeout = Duration::from_secs(2);

    let (s3_instance, s3_client) = setup_localstack(&conf).await.expect("valid localstack");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.db.max_connections)
        .acquire_timeout(conf.db.timeout)
        .connect(conf.db.url.as_str())
        .await
        .expect("connect test db");

    DirectMigrationEnv {
        token: db_instance.parent_token.child_token(),
        _db_instance: db_instance,
        _s3_instance: s3_instance,
        conf,
        pool,
        s3_client,
        signer: Arc::new(PrivateKeySigner::random()),
    }
}

async fn run_direct_migration(env: &DirectMigrationEnv) -> Result<(), crate::ExecutionError> {
    let config = S3MigrationConfig {
        batch_size: env.conf.db.batch_limit.into(),
        signer: env.signer.clone(),
        s3: env.conf.s3.clone(),
        mode: env.conf.s3_migration,
        sleep_duration: env.conf.s3_migration_sleep_duration,
        max_retries: env.conf.s3_migration_max_retries,
    };

    run_startup_migrations(&config, &env.token, &env.pool, &env.s3_client).await
}

async fn insert_legacy_ct64_digest_row(pool: &sqlx::PgPool, handle: &[u8], ct64_digest: &[u8]) {
    let key_id_gw = vec![0x07; 32];
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
        1_i64,
        &key_id_gw,
        handle,
        ct64_digest,
        S3_FORMAT_VERSION_V0,
    )
    .execute(pool)
    .await
    .expect("insert legacy ciphertext_digest row");
}

async fn set_ct128_digest_and_format(
    pool: &sqlx::PgPool,
    handle: &[u8],
    ct128_digest: &[u8],
    ct128_format: i16,
) {
    sqlx::query!(
        "UPDATE ciphertext_digest
            SET ciphertext128 = $1, ciphertext128_format = $2
            WHERE handle = $3",
        ct128_digest,
        ct128_format,
        handle,
    )
    .execute(pool)
    .await
    .expect("set legacy ct128 digest and format");
}

async fn record_existing_failure(pool: &sqlx::PgPool, handle: &[u8], error: &str) {
    sqlx::query!(
        r#"
        UPDATE ciphertext_digest
         SET s3_migration_failure_count = s3_migration_failure_count + 1,
             s3_migration_last_error = $1,
             s3_migration_last_error_at = NOW()
         WHERE handle = $2
           AND s3_format_version = $3
        "#,
        error,
        handle,
        S3_FORMAT_VERSION_V0,
    )
    .execute(pool)
    .await
    .expect("record existing migration failure");
}

async fn put_object(s3_client: &aws_sdk_s3::Client, bucket: &str, key: &str, bytes: Vec<u8>) {
    s3_client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(bytes))
        .send()
        .await
        .expect("upload test object");
}

async fn put_object_with_stale_metadata(
    s3_client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    bytes: Vec<u8>,
) {
    s3_client
        .put_object()
        .bucket(bucket)
        .key(key)
        .metadata("Ct-Format", "stale")
        .metadata("Uploaded-By", "legacy-test")
        .body(ByteStream::from(bytes))
        .send()
        .await
        .expect("upload stale test object");
}

async fn insert_ct64_bytes(pool: &sqlx::PgPool, handle: &[u8], ciphertext: &[u8]) {
    insert_ciphertext64(pool, &handle.to_vec(), &ciphertext.to_vec())
        .await
        .expect("insert ct64 bytes");
}

async fn insert_ct128_bytes(pool: &sqlx::PgPool, handle: &[u8], ciphertext: &[u8]) {
    sqlx::query!(
        "INSERT INTO ciphertexts128(handle, ciphertext)
            VALUES ($1, $2)
        ON CONFLICT DO NOTHING;",
        handle,
        ciphertext,
    )
    .execute(pool)
    .await
    .expect("insert ct128 bytes");
}

async fn assert_object_missing(s3_client: &aws_sdk_s3::Client, bucket: &str, key: &str) {
    s3_client
        .head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .expect_err("object should be missing");
}

async fn assert_object_body_eq(
    s3_client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    expected: &[u8],
) {
    let body = s3_client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .expect("object should exist")
        .body
        .collect()
        .await
        .expect("read object body")
        .into_bytes();

    assert_eq!(body.as_ref(), expected, "unexpected object body for {key}");
}

async fn assert_migrated_retry_state(pool: &sqlx::PgPool, handle: &[u8]) {
    let migration_state = sqlx::query!(
        r#"
        SELECT s3_format_version as "s3_format_version!",
               s3_migration_failure_count as "s3_migration_failure_count!",
               s3_migration_last_error
         FROM ciphertext_digest
         WHERE handle = $1
        "#,
        handle,
    )
    .fetch_one(pool)
    .await
    .expect("fetch migrated retry state");
    assert_eq!(migration_state.s3_format_version, S3_FORMAT_VERSION_V1);
    assert_eq!(migration_state.s3_migration_failure_count, 0);
    assert!(migration_state.s3_migration_last_error.is_none());
}

async fn assert_recorded_failure(
    pool: &sqlx::PgPool,
    handle: &[u8],
    expected_count: i32,
    expected_error: &str,
) {
    let failure = sqlx::query!(
        r#"
        SELECT s3_migration_failure_count,
               s3_migration_last_error
         FROM ciphertext_digest
         WHERE handle = $1
        "#,
        handle,
    )
    .fetch_one(pool)
    .await
    .expect("fetch recorded migration failure");

    assert_eq!(failure.s3_migration_failure_count, expected_count);
    assert!(
        failure
            .s3_migration_last_error
            .as_deref()
            .is_some_and(|err| err.contains(expected_error)),
        "unexpected recorded migration error: {:?}",
        failure.s3_migration_last_error,
    );
}

fn assert_metadata_eq(metadata: Option<&HashMap<String, String>>, key: &str, expected: &str) {
    let actual = metadata
        .and_then(|metadata| {
            metadata
                .iter()
                .find(|(metadata_key, _)| metadata_key.eq_ignore_ascii_case(key))
        })
        .map(|(_, value)| value.as_str());

    assert_eq!(actual, Some(expected), "unexpected metadata for {key}");
}
