use sqlx::{postgres::PgPoolOptions, PgPool};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

const GRANTS: &str = include_str!("../../db-migration/db-scripts/grant_pad_roles.sql");

const INSERT_PAD: &str = "INSERT INTO pads (prf_instance, label, pad_length, triggered_by)
    VALUES (1, decode(repeat('ab', 32), 'hex'), 64, decode(repeat('11', 20), 'hex'))
    ON CONFLICT DO NOTHING RETURNING label";

async fn test_db() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None).await.expect("test db");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("pool");
    (instance, pool)
}

/// Runs `sql` as `prf_service` in a transaction, then rolls the transaction back.
async fn as_prf_service(pool: &PgPool, sql: &str) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::raw_sql("SET LOCAL ROLE prf_service")
        .execute(&mut *tx)
        .await?;
    let result = sqlx::raw_sql(sql).execute(&mut *tx).await.map(|_| ());
    tx.rollback().await?;
    result
}

fn assert_denied(result: Result<(), sqlx::Error>, sql: &str) {
    let err = result.expect_err(sql);
    let code = err.as_database_error().and_then(|e| e.code());
    assert_eq!(code.as_deref(), Some("42501"), "{sql}: {err}");
}

#[tokio::test]
async fn grants_skip_a_missing_role() {
    let (_instance, pool) = test_db().await;
    sqlx::raw_sql("DROP ROLE IF EXISTS prf_service")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();
}

#[tokio::test]
async fn prf_service_role_writes_only_the_queue() {
    let (_instance, pool) = test_db().await;
    sqlx::raw_sql(
        "DO $$ BEGIN CREATE ROLE prf_service NOLOGIN;
         EXCEPTION WHEN duplicate_object THEN NULL; END $$",
    )
    .execute(&pool)
    .await
    .unwrap();
    // The grants run on each deploy, so a second run must give the same result.
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();

    for allowed in [INSERT_PAD, "SELECT prf_instance, signature FROM pads"] {
        as_prf_service(&pool, allowed).await.expect(allowed);
    }

    for denied in [
        "SELECT 1 FROM computations",
        "SELECT 1 FROM ciphertexts",
        "SELECT 1 FROM keys",
        "UPDATE pads SET is_error = false",
        "UPDATE pads SET signature = NULL",
        "UPDATE pads SET claimed_at = NULL",
        "DELETE FROM pads",
    ] {
        assert_denied(as_prf_service(&pool, denied).await, denied);
    }
}
