use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::crs::CrsCache;
use fhevm_engine_common::db_keys::DbKeyCache;
use fhevm_engine_common::pg_pool::PostgresPoolManager;
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, extract_ct_list};
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::utils::{safe_deserialize_conformant, safe_serialize};
use sqlx::Row;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use test_harness::instance::{DBInstance, ImportMode};
use tfhe::integer::ciphertext::IntegerProvenCompactCiphertextListConformanceParams;
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::auxiliary::ZkData;
use crate::verifier::MAX_CACHED_KEYS;

pub async fn setup() -> anyhow::Result<(PostgresPoolManager, DBInstance)> {
    let _ = tracing_subscriber::fmt().json().with_level(true).try_init();
    let test_instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
        .await
        .expect("valid db instance");

    let conf = crate::Config {
        database_url: test_instance.db_url.clone(),
        listen_database_channel: "fhevm".to_string(),
        notify_database_channel: "notify".to_string(),
        pg_pool_connections: 10,
        pg_polling_interval: 60,
        worker_thread_count: 1,
        pg_timeout: Duration::from_secs(15),
        pg_auto_explain_with_min_duration: None,
    };

    let pool_mngr = PostgresPoolManager::connect_pool(
        test_instance.parent_token.child_token(),
        conf.database_url.as_str(),
        conf.pg_timeout,
        conf.pg_pool_connections,
        Duration::from_secs(2),
        conf.pg_auto_explain_with_min_duration,
    )
    .await
    .unwrap();

    let pmngr = pool_mngr.clone();

    sqlx::query("TRUNCATE TABLE verify_proofs")
        .execute(&pmngr.pool())
        .await
        .unwrap();

    let last_active_at = Arc::new(RwLock::new(SystemTime::now()));

    tokio::spawn(async move {
        crate::verifier::execute_verify_proofs_loop(pmngr, conf.clone(), last_active_at.clone())
            .await
            .unwrap();
    });

    sleep(Duration::from_secs(2)).await;

    Ok((pool_mngr, test_instance))
}

/// Checks if the proof is valid by querying the database continuously.
pub(crate) async fn is_valid(
    pool: &sqlx::PgPool,
    zk_proof_id: i64,
    max_retries: usize,
) -> Result<bool, sqlx::Error> {
    for _ in 0..max_retries {
        sleep(Duration::from_millis(100)).await;
        let result = sqlx::query!(
            "SELECT verified FROM verify_proofs WHERE zk_proof_id = $1",
            zk_proof_id
        )
        .fetch_one(pool)
        .await?;

        match result.verified {
            Some(verified) => return Ok(verified),
            None => continue,
        }
    }

    Ok(false)
}

#[derive(Debug)]
pub(crate) struct StoredCiphertext {
    pub(crate) handle: Vec<u8>,
    pub(crate) ciphertext: Vec<u8>,
    pub(crate) ciphertext_type: i16,
    pub(crate) input_blob_index: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DecryptionResult {
    pub(crate) output_type: i16,
    pub(crate) value: String,
}

pub(crate) async fn wait_for_handles(
    pool: &sqlx::PgPool,
    zk_proof_id: i64,
    max_retries: usize,
) -> Result<Vec<Vec<u8>>, sqlx::Error> {
    for _ in 0..max_retries {
        sleep(Duration::from_millis(100)).await;
        let row = sqlx::query("SELECT verified, handles FROM verify_proofs WHERE zk_proof_id = $1")
            .bind(zk_proof_id)
            .fetch_one(pool)
            .await?;

        let verified: Option<bool> = row.try_get("verified")?;
        if !matches!(verified, Some(true)) {
            continue;
        }

        let handles: Option<Vec<u8>> = row.try_get("handles")?;
        let handles = handles.unwrap_or_default();
        assert_eq!(handles.len() % 32, 0);

        return Ok(handles.chunks(32).map(|chunk| chunk.to_vec()).collect());
    }

    Ok(vec![])
}

pub(crate) async fn fetch_stored_ciphertexts(
    pool: &sqlx::PgPool,
    handles: &[Vec<u8>],
) -> Result<Vec<StoredCiphertext>, sqlx::Error> {
    if handles.is_empty() {
        return Ok(vec![]);
    }

    let rows = sqlx::query(
        "
            SELECT handle, ciphertext, ciphertext_type, input_blob_index
            FROM ciphertexts
            WHERE handle = ANY($1::BYTEA[])
            AND ciphertext_version = $2
            ORDER BY input_blob_index ASC
        ",
    )
    .bind(handles)
    .bind(current_ciphertext_version())
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(StoredCiphertext {
                handle: row.try_get("handle")?,
                ciphertext: row.try_get("ciphertext")?,
                ciphertext_type: row.try_get("ciphertext_type")?,
                input_blob_index: row.try_get("input_blob_index")?,
            })
        })
        .collect()
}

pub(crate) async fn decrypt_ciphertexts(
    pool: &sqlx::PgPool,
    handles: &[Vec<u8>],
) -> anyhow::Result<Vec<DecryptionResult>> {
    let stored = fetch_stored_ciphertexts(pool, handles).await?;
    let db_key_cache = DbKeyCache::new(MAX_CACHED_KEYS).expect("create db key cache");
    let key = db_key_cache.fetch_latest_from_pool(pool).await?;

    tokio::task::spawn_blocking(move || {
        let client_key = key.cks.expect("client key available in tests");
        tfhe::set_server_key(key.sks);

        stored
            .into_iter()
            .map(|ct| {
                let deserialized = SupportedFheCiphertexts::decompress_no_memcheck(
                    ct.ciphertext_type,
                    &ct.ciphertext,
                )
                .expect("valid compressed ciphertext");
                DecryptionResult {
                    output_type: ct.ciphertext_type,
                    value: deserialized.decrypt(&client_key),
                }
            })
            .collect::<Vec<_>>()
    })
    .await
    .map_err(anyhow::Error::from)
}

pub(crate) async fn compress_inputs_without_rerandomization(
    pool: &sqlx::PgPool,
    raw_ct: &[u8],
) -> anyhow::Result<Vec<Vec<u8>>> {
    let db_key_cache = DbKeyCache::new(MAX_CACHED_KEYS).expect("create db key cache");
    let latest_key = db_key_cache.fetch_latest_from_pool(pool).await?;
    let latest_crs = CrsCache::load(pool)
        .await?
        .get_latest()
        .cloned()
        .expect("latest CRS");

    let verified_list: tfhe::ProvenCompactCiphertextList = safe_deserialize_conformant(
        raw_ct,
        &IntegerProvenCompactCiphertextListConformanceParams::from_public_key_encryption_parameters_and_crs_parameters(
            latest_key.pks.parameters(),
            &latest_crs.crs,
        ),
    )?;

    if verified_list.is_empty() {
        return Ok(vec![]);
    }

    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(latest_key.sks);
        let expanded = verified_list.expand_without_verification()?;
        let cts = extract_ct_list(&expanded)?;
        cts.into_iter()
            .map(|ct| ct.compress().map_err(anyhow::Error::from))
            .collect()
    })
    .await?
}

#[derive(Debug, Clone)]
pub(crate) enum ZkInput {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl ZkInput {
    pub(crate) fn cleartext(&self) -> String {
        match self {
            Self::Bool(value) => value.to_string(),
            Self::U8(value) => value.to_string(),
            Self::U16(value) => value.to_string(),
            Self::U32(value) => value.to_string(),
            Self::U64(value) => value.to_string(),
        }
    }
}

pub(crate) async fn generate_zk_pok_with_inputs(
    pool: &sqlx::PgPool,
    aux_data: &[u8],
    inputs: &[ZkInput],
) -> Vec<u8> {
    let db_key_cache = DbKeyCache::new(MAX_CACHED_KEYS).expect("create db key cache");

    let latest_key = db_key_cache.fetch_latest_from_pool(pool).await.unwrap();

    let latest_crs = CrsCache::load(pool)
        .await
        .unwrap()
        .get_latest()
        .cloned()
        .unwrap();

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&latest_key.pks);
    for v in inputs {
        match *v {
            ZkInput::Bool(b) => builder.push(b),
            ZkInput::U8(x) => builder.push(x),
            ZkInput::U16(x) => builder.push(x),
            ZkInput::U32(x) => builder.push(x),
            ZkInput::U64(x) => builder.push(x),
        };
    }

    let the_list = builder
        .build_with_proof_packed(&latest_crs.crs, aux_data, tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();

    safe_serialize(&the_list)
}

pub(crate) async fn generate_sample_zk_pok(pool: &sqlx::PgPool, aux_data: &[u8]) -> Vec<u8> {
    let inputs = vec![
        ZkInput::Bool(true),
        ZkInput::U8(42),
        ZkInput::U16(12345),
        ZkInput::U32(67890),
        ZkInput::U64(1234567890),
    ];
    generate_zk_pok_with_inputs(pool, aux_data, &inputs).await
}

pub(crate) async fn generate_empty_input_list(pool: &sqlx::PgPool, aux_data: &[u8]) -> Vec<u8> {
    let inputs = Vec::new();
    generate_zk_pok_with_inputs(pool, aux_data, &inputs).await
}

pub(crate) async fn insert_proof(
    pool: &sqlx::PgPool,
    request_id: i64,
    zk_pok: &[u8],
    aux: &ZkData,
) -> Result<i64, sqlx::Error> {
    //  Insert ZkPok into database
    sqlx::query(
            "INSERT INTO verify_proofs (zk_proof_id, input, chain_id, contract_address, user_address, verified)
            VALUES ($1, $2, $3, $4, $5, NULL )" 
        ).bind(request_id)
        .bind(zk_pok)
        .bind(aux.chain_id.as_i64())
        .bind(aux.contract_address.clone())
        .bind(aux.user_address.clone())
        .execute(pool).await?;

    // pg_notify to trigger the worker

    sqlx::query("SELECT pg_notify($1, '')")
        .bind("fhevm")
        .execute(pool)
        .await
        .unwrap();

    Ok(request_id)
}

pub(crate) fn aux_fixture(acl_contract_address: String) -> (ZkData, [u8; 92]) {
    // Define  20-byte addresses
    let contract_address = "0x1111111111111111111111111111111111111111".to_string();
    let user_address = "0x2222222222222222222222222222222222222222".to_string();
    let zk_data = ZkData {
        contract_address,
        user_address,
        acl_contract_address,
        chain_id: ChainId::try_from(12345_u64).unwrap(),
    };

    (
        zk_data.clone(),
        zk_data.assemble().expect("Failed to assemble ZkData"),
    )
}
