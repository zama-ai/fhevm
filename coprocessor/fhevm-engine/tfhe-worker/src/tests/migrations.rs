use sqlx::{PgPool, Row};
use test_harness::instance::{setup_test_db, ImportMode};

/// The version number of the remove_tenants migration under test.
const TARGET_MIGRATION_VERSION: i64 = 20260128095635;

/// Runs all migrations before the target version and returns the target migration's SQL.
async fn run_migrations_before_target(pool: &PgPool) -> String {
    let migrator = sqlx::migrate!("./migrations");
    let mut target_sql = None;

    for migration in migrator.migrations.iter() {
        if migration.migration_type.is_down_migration() {
            continue;
        }

        if migration.version < TARGET_MIGRATION_VERSION {
            sqlx::raw_sql(&migration.sql)
                .execute(pool)
                .await
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to run migration {} ({}): {}",
                        migration.version, migration.description, e
                    )
                });
        } else if migration.version == TARGET_MIGRATION_VERSION {
            target_sql = Some(migration.sql.to_string());
        }
    }

    target_sql.expect("Target migration not found in compiled migrations")
}

/// Inserts test data using the OLD schema (with tenant_id columns).
/// Returns the auto-generated tenant_id.
async fn seed_old_schema_data(pool: &PgPool) -> i32 {
    // 1. Insert a single tenant.
    sqlx::query(
        "INSERT INTO tenants (
            chain_id, verifying_contract_address, acl_contract_address,
            pks_key, sks_key, public_params, cks_key, key_id
        ) VALUES (
            12345, '0xVerifyingAddr', '0xACLContractAddr',
            '\\xaa'::bytea, '\\xbb'::bytea, '\\xcc'::bytea, '\\xdd'::bytea, '\\xee'::bytea
        )",
    )
    .execute(pool)
    .await
    .expect("Insert tenant");

    let tenant_id: i32 = sqlx::query_scalar("SELECT tenant_id FROM tenants LIMIT 1")
        .fetch_one(pool)
        .await
        .expect("Fetch tenant_id");

    // 2. Insert into computations.
    sqlx::query(
        "INSERT INTO computations (
            tenant_id, output_handle, dependencies, fhe_operation, is_scalar,
            transaction_id
        ) VALUES (
            $1, '\\x0001'::bytea, ARRAY['\\x0002'::bytea], 1, false,
            '\\x0003'::bytea
        )",
    )
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("Insert computation");

    // 3. Insert into ciphertext_digest.
    sqlx::query(
        "INSERT INTO ciphertext_digest (
            tenant_id, handle, txn_is_sent, txn_limited_retries_count
        ) VALUES (
            $1, '\\x0010'::bytea, false, 0
        )",
    )
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("Insert ciphertext_digest");

    // 4. Insert into pbs_computations.
    sqlx::query(
        "INSERT INTO pbs_computations (tenant_id, handle)
         VALUES ($1, '\\x0020'::bytea)",
    )
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("Insert pbs_computation");

    // 5. Insert into ciphertexts.
    sqlx::query(
        "INSERT INTO ciphertexts (
            tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type
        ) VALUES (
            $1, '\\x0030'::bytea, '\\xab'::bytea, 0, 4
        )",
    )
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("Insert ciphertext");

    // 6. Insert into ciphertexts128.
    sqlx::query(
        "INSERT INTO ciphertexts128 (tenant_id, handle, ciphertext)
         VALUES ($1, '\\x0040'::bytea, '\\xcd'::bytea)",
    )
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("Insert ciphertext128");

    // 7. Insert into input_blobs.
    sqlx::query(
        "INSERT INTO input_blobs (tenant_id, blob_hash, blob_data, blob_ciphertext_count)
         VALUES ($1, '\\x0050'::bytea, '\\xef'::bytea, 2)",
    )
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("Insert input_blob");

    // 8. Insert into allowed_handles.
    sqlx::query(
        "INSERT INTO allowed_handles (
            tenant_id, handle, account_address, event_type
        ) VALUES (
            $1, '\\x0060'::bytea, '0xAccount1', 0
        )",
    )
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("Insert allowed_handle");

    // 9. Insert into verify_proofs (to test chain_id -> host_chain_id rename).
    sqlx::query(
        "INSERT INTO verify_proofs (
            zk_proof_id, chain_id, contract_address, user_address
        ) VALUES (
            1, 12345, '0xContract', '0xUser'
        )",
    )
    .execute(pool)
    .await
    .expect("Insert verify_proof");

    tenant_id
}

/// Helper to check if a column exists in a table.
async fn column_exists(pool: &PgPool, table: &str, column: &str) -> bool {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_name = $1 AND column_name = $2
        )",
    )
    .bind(table)
    .bind(column)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// Helper to check if a table exists.
async fn table_exists(pool: &PgPool, table: &str) -> bool {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_name = $1
        )",
    )
    .bind(table)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_remove_tenants_migration_with_data() {
    let db = setup_test_db(ImportMode::SkipMigrations)
        .await
        .expect("setup test db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    // Phase 1: Run all migrations before the target.
    let target_sql = run_migrations_before_target(&pool).await;

    // Phase 2: Insert data using the old schema.
    seed_old_schema_data(&pool).await;

    // Phase 3: Run the target migration.
    sqlx::raw_sql(&target_sql)
        .execute(&pool)
        .await
        .expect("remove_tenants migration should succeed");

    // Phase 4: Assert the new schema and data correctness.

    // 4a. `tenants` table renamed to `keys`.
    assert!(
        !table_exists(&pool, "tenants").await,
        "tenants table should no longer exist"
    );
    assert!(table_exists(&pool, "keys").await, "keys table should exist");

    // 4b. Dropped columns are gone from keys.
    for col in &[
        "tenant_id",
        "tenant_api_key",
        "is_admin",
        "sns_sk",
        "verifying_contract_address",
        "chain_id",
        "public_params",
    ] {
        assert!(
            !column_exists(&pool, "keys", col).await,
            "Column '{col}' should be dropped from keys"
        );
    }

    // 4c. keys table has new columns and correct data.
    let key_row = sqlx::query("SELECT key_id, key_id_gw, pks_key, sks_key, cks_key FROM keys")
        .fetch_one(&pool)
        .await
        .expect("keys should have exactly one row");

    let key_id: &[u8] = key_row.get("key_id");
    assert_eq!(key_id, b"", "key_id should be set to empty bytes");

    let key_id_gw: &[u8] = key_row.get("key_id_gw");
    assert_eq!(
        key_id_gw, b"\xee",
        "key_id_gw should preserve old key_id value"
    );

    // 4d. CRS moved from keys to new crs table.
    assert!(table_exists(&pool, "crs").await, "crs table should exist");
    let crs_row = sqlx::query("SELECT crs_id, crs FROM crs")
        .fetch_one(&pool)
        .await
        .expect("crs should have one row");
    let crs_id: &[u8] = crs_row.get("crs_id");
    let crs: &[u8] = crs_row.get("crs");
    assert_eq!(crs_id, b"", "crs_id should be empty bytes");
    assert_eq!(crs, b"\xcc", "crs should contain old public_params value");

    // 4e. host_chains populated from old tenant data.
    assert!(
        table_exists(&pool, "host_chains").await,
        "host_chains table should exist"
    );
    let hc_row = sqlx::query("SELECT chain_id, name, acl_contract_address FROM host_chains")
        .fetch_one(&pool)
        .await
        .expect("host_chains should have one row");
    let chain_id: i64 = hc_row.get("chain_id");
    let name: &str = hc_row.get("name");
    let acl: &str = hc_row.get("acl_contract_address");
    assert_eq!(chain_id, 12345);
    assert_eq!(name, "ethereum");
    assert_eq!(acl, "0xACLContractAddr");

    // 4f. computations: tenant_id dropped, host_chain_id populated from old chain_id.
    assert!(!column_exists(&pool, "computations", "tenant_id").await);
    let comp_row = sqlx::query("SELECT output_handle, host_chain_id FROM computations")
        .fetch_one(&pool)
        .await
        .expect("computation should exist");
    let host_chain_id: i64 = comp_row.get("host_chain_id");
    assert_eq!(
        host_chain_id, 12345,
        "host_chain_id should be populated from tenant's chain_id"
    );

    // 4g. ciphertext_digest: tenant_id dropped, host_chain_id + key_id_gw added.
    assert!(!column_exists(&pool, "ciphertext_digest", "tenant_id").await);
    let cd_row = sqlx::query("SELECT handle, host_chain_id, key_id_gw FROM ciphertext_digest")
        .fetch_one(&pool)
        .await
        .expect("ciphertext_digest should exist");
    let cd_chain: i64 = cd_row.get("host_chain_id");
    let cd_key_id_gw: &[u8] = cd_row.get("key_id_gw");
    assert_eq!(cd_chain, 12345);
    assert_eq!(
        cd_key_id_gw, b"\xee",
        "key_id_gw should be populated from keys.key_id_gw"
    );

    // 4h. pbs_computations: tenant_id dropped, host_chain_id populated.
    assert!(!column_exists(&pool, "pbs_computations", "tenant_id").await);
    let pbs_row = sqlx::query("SELECT handle, host_chain_id FROM pbs_computations")
        .fetch_one(&pool)
        .await
        .expect("pbs_computation should exist");
    let pbs_chain: i64 = pbs_row.get("host_chain_id");
    assert_eq!(pbs_chain, 12345);

    // 4i. ciphertexts: tenant_id dropped, data preserved.
    assert!(!column_exists(&pool, "ciphertexts", "tenant_id").await);
    let ct_row = sqlx::query("SELECT handle, ciphertext FROM ciphertexts")
        .fetch_one(&pool)
        .await
        .expect("ciphertext should exist");
    let ct_handle: &[u8] = ct_row.get("handle");
    assert_eq!(ct_handle, b"\x00\x30");

    // 4j. ciphertexts128: tenant_id dropped, data preserved.
    assert!(!column_exists(&pool, "ciphertexts128", "tenant_id").await);
    let ct128 = sqlx::query("SELECT handle FROM ciphertexts128")
        .fetch_one(&pool)
        .await
        .expect("ciphertext128 should exist");
    let ct128_handle: &[u8] = ct128.get("handle");
    assert_eq!(ct128_handle, b"\x00\x40");

    // 4k. input_blobs: tenant_id dropped, data preserved.
    assert!(!column_exists(&pool, "input_blobs", "tenant_id").await);
    let ib = sqlx::query("SELECT blob_hash FROM input_blobs")
        .fetch_one(&pool)
        .await
        .expect("input_blob should exist");
    let blob_hash: &[u8] = ib.get("blob_hash");
    assert_eq!(blob_hash, b"\x00\x50");

    // 4l. allowed_handles: tenant_id dropped, data preserved.
    assert!(!column_exists(&pool, "allowed_handles", "tenant_id").await);
    let ah = sqlx::query("SELECT handle, account_address FROM allowed_handles")
        .fetch_one(&pool)
        .await
        .expect("allowed_handle should exist");
    let ah_handle: &[u8] = ah.get("handle");
    let ah_account: &str = ah.get("account_address");
    assert_eq!(ah_handle, b"\x00\x60");
    assert_eq!(ah_account, "0xAccount1");

    // 4m. verify_proofs: chain_id renamed to host_chain_id.
    assert!(
        column_exists(&pool, "verify_proofs", "host_chain_id").await,
        "verify_proofs should have host_chain_id"
    );
    assert!(
        !column_exists(&pool, "verify_proofs", "chain_id").await,
        "verify_proofs chain_id should be renamed"
    );
}

#[tokio::test]
async fn test_remove_tenants_migration_rejects_multiple_tenants() {
    let db = setup_test_db(ImportMode::SkipMigrations)
        .await
        .expect("setup test db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    let target_sql = run_migrations_before_target(&pool).await;

    // Insert TWO tenants.
    sqlx::query(
        "INSERT INTO tenants (
            chain_id, verifying_contract_address, acl_contract_address,
            pks_key, sks_key, public_params
        ) VALUES
        (111, '0xV1', '0xA1', '\\xaa'::bytea, '\\xbb'::bytea, '\\xcc'::bytea),
        (222, '0xV2', '0xA2', '\\xdd'::bytea, '\\xee'::bytea, '\\xff'::bytea)",
    )
    .execute(&pool)
    .await
    .expect("Insert two tenants");

    // Running the target migration should fail due to the >1 row check.
    let result = sqlx::raw_sql(&target_sql).execute(&pool).await;

    assert!(
        result.is_err(),
        "Migration should fail with more than one tenant"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Expected zero or one row"),
        "Error should mention row count check, got: {err_msg}"
    );
}

#[tokio::test]
async fn test_remove_tenants_migration_empty_db() {
    let db = setup_test_db(ImportMode::SkipMigrations)
        .await
        .expect("setup test db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    let target_sql = run_migrations_before_target(&pool).await;

    // No data inserted. Migration should succeed on empty tables.
    sqlx::raw_sql(&target_sql)
        .execute(&pool)
        .await
        .expect("remove_tenants migration should succeed on empty DB");

    // Verify the new tables exist and are empty.
    assert!(table_exists(&pool, "keys").await);
    assert!(table_exists(&pool, "crs").await);
    assert!(table_exists(&pool, "host_chains").await);

    let key_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM keys")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(key_count, 0);

    let crs_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(crs_count, 0);

    let hc_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chains")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(hc_count, 0);
}
