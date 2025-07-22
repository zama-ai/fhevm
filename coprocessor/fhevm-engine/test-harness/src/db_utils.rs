use alloy::primitives::U256;
use fhevm_engine_common::tenant_keys::write_large_object_in_chunks;
use rand::distr::Alphanumeric;
use rand::Rng;
use sqlx::postgres::types::Oid;
use sqlx::{query, PgPool};
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::time::sleep;
use tracing::info;

pub const ACL_CONTRACT_ADDR: &str = "0x339EcE85B9E11a3A3AA557582784a15d7F82AAf2";

/// Uploads a file to the database as a large object and returns its Oid
pub async fn import_file_into_db(pool: &PgPool, file_path: &str) -> Result<Oid, sqlx::Error> {
    let mut file = fs::File::open(file_path)
        .await
        .expect("Failed to open file");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .expect("Failed to read file");

    let oid = write_large_object_in_chunks(pool, &buffer, 16 * 1024)
        .await
        .expect("Writing a large object should succeed");

    info!("Uploaded large object with Oid: {:?}", oid);

    Ok(oid)
}

pub async fn insert_ciphertext64(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    handle: &Vec<u8>,
    ciphertext: &Vec<u8>,
    ciphertext128: &[u8],
) -> anyhow::Result<()> {
    let _ = query!(
        "INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext128, ciphertext_version, ciphertext_type) 
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT DO NOTHING;",
         tenant_id,
        handle,
        ciphertext,
        ciphertext128,
        0,
        0,
    )
    .execute(pool)
    .await
    .expect("insert into ciphertexts");

    Ok(())
}

pub async fn insert_into_pbs_computations(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    handle: &Vec<u8>,
) -> Result<(), anyhow::Error> {
    let _ = query!(
        "INSERT INTO pbs_computations(tenant_id, handle) VALUES($1, $2) 
             ON CONFLICT DO NOTHING;",
        tenant_id,
        handle,
    )
    .execute(pool)
    .await
    .expect("insert into pbs_computations");

    Ok(())
}

pub async fn insert_ciphertext_digest(
    pool: &PgPool,
    tenant_id: i32,
    handle: &[u8; 32],
    ciphertext: &[u8],
    ciphertext128: &[u8],
    txn_limited_retries_count: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO ciphertext_digest (tenant_id, handle, ciphertext, ciphertext128, txn_limited_retries_count)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        tenant_id,
        handle,
        ciphertext,
        ciphertext128,
        txn_limited_retries_count,
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Poll database until ciphertext128 of the specified handle is available
pub async fn wait_for_ciphertext(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    handle: &Vec<u8>,
    retries: u64,
) -> anyhow::Result<Vec<u8>> {
    for retry in 0..retries {
        let record = sqlx::query!(
            "SELECT ciphertext128 FROM ciphertexts WHERE tenant_id = $1 AND handle = $2",
            tenant_id,
            handle
        )
        .fetch_one(pool)
        .await;

        if let Result::Ok(record) = record {
            if let Some(ciphertext128) = record.ciphertext128 {
                return anyhow::Ok(ciphertext128);
            }
        }

        println!("wait for ciphertext, retry: {}", retry);

        // Wait before retrying
        sleep(Duration::from_millis(500)).await;
    }

    Err(sqlx::Error::RowNotFound.into())
}

pub async fn setup_test_user(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let (sks, cks, pks, pp, sns_pk) = if !cfg!(feature = "gpu") {
        (
            "../fhevm-keys/sks",
            "../fhevm-keys/cks",
            "../fhevm-keys/pks",
            "../fhevm-keys/pp",
            "../fhevm-keys/sns_pk",
        )
    } else {
        (
            "../fhevm-keys/gpu-csks",
            "../fhevm-keys/gpu-cks",
            "../fhevm-keys/gpu-pks",
            "../fhevm-keys/gpu-pp",
            "../fhevm-keys/sns_pk",
        )
    };
    let sks = tokio::fs::read(sks).await.expect("can't read sks key");
    let pks = tokio::fs::read(pks).await.expect("can't read pks key");
    let cks = tokio::fs::read(cks).await.expect("can't read cks key");
    let public_params = tokio::fs::read(pp).await.expect("can't read public params");

    let sns_pk_oid = import_file_into_db(pool, sns_pk).await?;
    info!("Uploaded sns_pk with Oid: {:?}", sns_pk_oid);

    sqlx::query!(
        "
            INSERT INTO tenants(tenant_api_key, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params, cks_key, sns_pk)
            VALUES (
                'a1503fb6-d79b-4e9e-826d-44cf262f3e05',
                12345,
                $1,
                '0x69dE3158643e738a0724418b21a35FAA20CBb1c5',
                $2,
                $3,
                $4,
                $5,
                $6
            )
        ",
        ACL_CONTRACT_ADDR.to_string(),
        &pks,
        &sks,
        &public_params,
        &cks,
        sns_pk_oid
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_random_tenant(pool: &PgPool) -> Result<i32, sqlx::Error> {
    let chain_id: i32 = rand::rng().random_range(1..10000);
    let key_id_i32: i32 = rand::rng().random_range(1..10000);

    let verifying_contract_address: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(42)
        .map(char::from)
        .collect();

    let acl_contract_address: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(42)
        .map(char::from)
        .collect();

    info!(
        "Dummy tenant info chain_id: {}, key_id: {}, acl_addr: {}, verify_addr: {}",
        chain_id, key_id_i32, acl_contract_address, verifying_contract_address
    );

    let pks_key: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
    let sks_key: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
    let public_params: Vec<u8> = (0..64).map(|_| rand::random::<u8>()).collect();
    let key_id = U256::from(key_id_i32).to_be_bytes::<32>();

    let row = sqlx::query!(
        r#"
        INSERT INTO tenants (chain_id, key_id, verifying_contract_address, acl_contract_address, 
                            pks_key, sks_key, public_params)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING tenant_id, tenant_api_key, chain_id, verifying_contract_address, 
                  acl_contract_address, pks_key, sks_key, public_params, key_id
        "#,
        chain_id,
        &key_id,
        verifying_contract_address,
        acl_contract_address,
        pks_key,
        sks_key,
        public_params
    )
    .fetch_one(pool)
    .await?;

    Ok(row.tenant_id)
}
