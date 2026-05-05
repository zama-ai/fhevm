use fhevm_engine_common::db_keys::DbKeyCache;
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, Row};
use test_harness::instance::{setup_test_db, ImportMode};

fn db_url_for_role(base_url: &str, username: &str, password: &str) -> String {
    let (_, host_and_db) = base_url
        .split_once('@')
        .expect("database URL should include credentials");
    format!("postgresql://{username}:{password}@{host_and_db}")
}

fn random_key_id() -> Vec<u8> {
    (0..32).map(|_| rand::random::<u8>()).collect()
}

#[tokio::test]
#[serial(db)]
async fn test_fetch_latest_uses_cache_without_selecting_key_blobs(
) -> Result<(), Box<dyn std::error::Error>> {
    let db = setup_test_db(ImportMode::WithKeysNoSns).await?;
    let admin_pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(db.db_url())
        .await?;
    let cache = DbKeyCache::new(1)?;

    let expected = cache.fetch_latest_from_pool(&admin_pool).await?;

    let role = format!("key_meta_reader_{}", rand::random::<u32>());
    let password = "key_meta_reader_password";
    sqlx::query(&format!("CREATE ROLE {role} LOGIN PASSWORD '{password}'"))
        .execute(&admin_pool)
        .await?;
    sqlx::query(&format!("GRANT CONNECT ON DATABASE coprocessor TO {role}"))
        .execute(&admin_pool)
        .await?;
    sqlx::query(&format!("GRANT USAGE ON SCHEMA public TO {role}"))
        .execute(&admin_pool)
        .await?;
    sqlx::query(&format!(
        "GRANT SELECT (key_id, sequence_number) ON TABLE keys TO {role}"
    ))
    .execute(&admin_pool)
    .await?;

    let limited_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url_for_role(db.db_url(), &role, password))
        .await?;

    let cached = cache.fetch_latest_from_pool(&limited_pool).await?;
    assert_eq!(cached.key_id, expected.key_id);
    assert_eq!(cached.sequence_number, expected.sequence_number);

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_fetch_latest_refreshes_cache_after_key_rotation(
) -> Result<(), Box<dyn std::error::Error>> {
    let db = setup_test_db(ImportMode::WithKeysNoSns).await?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(db.db_url())
        .await?;
    let cache = DbKeyCache::new(1)?;

    let initial = cache.fetch_latest_from_pool(&pool).await?;

    let row = sqlx::query("SELECT pks_key, sks_key, cks_key FROM keys WHERE key_id = $1")
        .bind(&initial.key_id)
        .fetch_one(&pool)
        .await?;
    let pks_key: Vec<u8> = row.try_get("pks_key")?;
    let sks_key: Vec<u8> = row.try_get("sks_key")?;
    let cks_key: Option<Vec<u8>> = row.try_get("cks_key")?;

    let new_key_id = random_key_id();
    let new_key_id_gw = random_key_id();
    sqlx::query(
        "INSERT INTO keys (key_id, key_id_gw, pks_key, sks_key, cks_key) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&new_key_id)
    .bind(&new_key_id_gw)
    .bind(&pks_key)
    .bind(&sks_key)
    .bind(cks_key)
    .execute(&pool)
    .await?;

    let rotated = cache.fetch_latest_from_pool(&pool).await?;
    assert_eq!(rotated.key_id, new_key_id);
    assert!(rotated.sequence_number > initial.sequence_number);

    Ok(())
}
