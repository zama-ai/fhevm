use fhevm_engine_common::versioning::{bootstrap_versioning, resolve_gcs_mode};
use test_harness::instance::{setup_test_db, ImportMode};

async fn set_consensus_version(db_url: &str, version: i64) {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(db_url)
        .await
        .expect("connect");
    sqlx::query("UPDATE versioning SET consensus_version = $1 WHERE singleton = TRUE")
        .bind(version)
        .execute(&pool)
        .await
        .expect("set consensus version");
}

#[tokio::test]
async fn gcs_mode_tracks_consensus_version_not_release() {
    let db = setup_test_db(ImportMode::None)
        .await
        .expect("setup test db");

    assert!(
        !resolve_gcs_mode(db.db_url()).await.expect("resolve"),
        "matching versions must run blue"
    );

    // Rolling upgrade: a different release with the same consensus version stays live.
    sqlx::query("UPDATE versioning SET stack_version = 'v9.9.9' WHERE singleton = TRUE")
        .execute(
            &sqlx::postgres::PgPoolOptions::new()
                .max_connections(1)
                .connect(db.db_url())
                .await
                .expect("connect"),
        )
        .await
        .expect("set release");
    assert!(
        !resolve_gcs_mode(db.db_url()).await.expect("resolve"),
        "a release change without a consensus change must run blue"
    );

    if fhevm_engine_common::CONSENSUS_PROTOCOL_VERSION > 0 {
        set_consensus_version(
            db.db_url(),
            i64::from(fhevm_engine_common::CONSENSUS_PROTOCOL_VERSION - 1),
        )
        .await;
        assert!(
            resolve_gcs_mode(db.db_url()).await.expect("resolve"),
            "the next version must run green"
        );
    }

    set_consensus_version(db.db_url(), 99).await;
    assert!(
        resolve_gcs_mode(db.db_url()).await.is_err(),
        "an old version must not start"
    );
}

#[tokio::test]
async fn bootstrap_refuses_existing_database_and_consensus_downgrade() {
    let db = setup_test_db(ImportMode::None)
        .await
        .expect("setup test db");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(db.db_url())
        .await
        .expect("connect");

    let versions_before: (String, i64) = sqlx::query_as(
        "SELECT stack_version, consensus_version FROM versioning WHERE singleton = TRUE",
    )
    .fetch_one(&pool)
    .await
    .expect("read versions");

    let existing_error = bootstrap_versioning(&pool)
        .await
        .expect_err("existing database bootstrap must fail");
    assert!(
        existing_error.to_string().contains("not a new database"),
        "unexpected bootstrap error: {existing_error:#}"
    );

    let versions_after: (String, i64) = sqlx::query_as(
        "SELECT stack_version, consensus_version FROM versioning WHERE singleton = TRUE",
    )
    .fetch_one(&pool)
    .await
    .expect("read saved versions");
    assert_eq!(versions_after, versions_before);

    sqlx::query(
        "CREATE TABLE public._fhevm_versioning_bootstrap (
            singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )",
    )
    .execute(&pool)
    .await
    .expect("create setup marker table");
    sqlx::query("INSERT INTO public._fhevm_versioning_bootstrap (singleton) VALUES (TRUE)")
        .execute(&pool)
        .await
        .expect("create setup marker");

    let newer_consensus = i64::from(fhevm_engine_common::CONSENSUS_PROTOCOL_VERSION) + 1;
    set_consensus_version(db.db_url(), newer_consensus).await;
    let downgrade_error = bootstrap_versioning(&pool)
        .await
        .expect_err("consensus downgrade must fail");
    assert!(
        downgrade_error.to_string().contains("cannot lower"),
        "unexpected downgrade error: {downgrade_error:#}"
    );

    let preserved_consensus: i64 =
        sqlx::query_scalar("SELECT consensus_version FROM versioning WHERE singleton = TRUE")
            .fetch_one(&pool)
            .await
            .expect("read preserved consensus");
    assert_eq!(preserved_consensus, newer_consensus);

    let retry_marker_preserved: bool =
        sqlx::query_scalar("SELECT to_regclass('public._fhevm_versioning_bootstrap') IS NOT NULL")
            .fetch_one(&pool)
            .await
            .expect("read bootstrap marker");
    assert!(
        retry_marker_preserved,
        "a failed setup must keep its marker for a retry"
    );

    // The versions are not final while the marker exists, so no role may be picked.
    assert!(
        resolve_gcs_mode(db.db_url()).await.is_err(),
        "startup must fail while database setup is in progress"
    );
}
