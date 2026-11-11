use alloy::primitives::{Address, B256};
use ciphertext_attestation::{prf::Version, PrfOutputAttestation, PrfOutputRef};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::time::Duration;

pub const CONNECTOR: Address = Address::repeat_byte(0x11);
pub const DEADLINE: Duration = Duration::from_secs(5);

pub const GRANTS: &str = include_str!("../../../db-migration/db-scripts/grant_pad_roles.sql");

/// A pool that logs in as `prf_service`, with the rights that the migration job gives it.
pub async fn prf_service_pool(admin: &PgPool, db_url: &str, max_connections: u32) -> PgPool {
    sqlx::raw_sql(
        "DO $$ BEGIN CREATE ROLE prf_service LOGIN PASSWORD 'prf_service';
         EXCEPTION WHEN duplicate_object THEN NULL; END $$",
    )
    .execute(admin)
    .await
    .unwrap();
    sqlx::raw_sql(GRANTS).execute(admin).await.unwrap();
    // A live lane for instance 0. The heartbeat is in the future, so it stays live during a test.
    sqlx::raw_sql(
        "INSERT INTO prf_lanes (prf_instance, ready_at, heartbeat_at)
         VALUES (0, NOW(), NOW() + INTERVAL '1 day') ON CONFLICT DO NOTHING",
    )
    .execute(admin)
    .await
    .unwrap();
    let options: PgConnectOptions = db_url.parse().unwrap();
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options.username("prf_service").password("prf_service"))
        .await
        .unwrap()
}

/// A different byte in each field, so a test finds two swapped columns.
pub fn attestation(byte: u8) -> PrfOutputAttestation {
    PrfOutputAttestation {
        version: Version::V1,
        ciphertext_digest: B256::repeat_byte(byte + 1),
        sns_ciphertext_digest: B256::repeat_byte(byte + 2),
        signer: Address::repeat_byte(byte + 3),
        signature: vec![byte + 4; 65],
    }
}

/// Completes the row with `attestation(byte)` and a body of 32 `byte`s, as the pad-worker does.
pub async fn set_output(pool: &PgPool, path: &PrfOutputRef, byte: u8) {
    set_output_of_length(pool, path, byte, 32).await;
}

/// Completes the row with `attestation(byte)` and a body of `length` `byte`s.
pub async fn set_output_of_length(pool: &PgPool, path: &PrfOutputRef, byte: u8, length: usize) {
    let attestation = attestation(byte);
    sqlx::query(
        "UPDATE pads SET ciphertext128 = $4, ciphertext_digest = $5, ciphertext128_digest = $6,
                signer = $7, signature = $8, attestation_version = $9
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
    )
    .bind(i32::from(path.prf_id))
    .bind(path.label.as_slice())
    .bind(i32::from(path.pad_length))
    .bind(vec![byte; length])
    .bind(attestation.ciphertext_digest.as_slice())
    .bind(attestation.sns_ciphertext_digest.as_slice())
    .bind(attestation.signer.as_slice())
    .bind(&attestation.signature)
    .bind(i16::from(u8::from(attestation.version)))
    .execute(pool)
    .await
    .unwrap();
}

pub async fn set_error(pool: &PgPool, path: &PrfOutputRef) {
    sqlx::query(
        "UPDATE pads SET is_error = true
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
    )
    .bind(i32::from(path.prf_id))
    .bind(path.label.as_slice())
    .bind(i32::from(path.pad_length))
    .execute(pool)
    .await
    .unwrap();
}
