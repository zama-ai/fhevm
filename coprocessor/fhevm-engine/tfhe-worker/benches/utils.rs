use alloy::primitives::{Address, Log as PrimitivesLog};
use alloy::rpc::types::Log as RpcLog;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, try_expand_ciphertext_list};
use fhevm_engine_common::types::SupportedFheOperations;
use fhevm_engine_common::utils::safe_deserialize_key;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, LogTfhe, ScalarByte, Transaction,
};
use sha3::{Digest, Keccak256};
use sqlx::PgPool;
use sqlx::Row;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tfhe_worker::daemon_cli::Args;
use tokio::sync::watch::Receiver;
use tracing::Level;

pub struct TestInstance {
    // just to destroy container
    _container: Option<testcontainers::ContainerAsync<testcontainers::GenericImage>>,
    // send message to this on destruction to stop the app
    app_close_channel: Option<tokio::sync::watch::Sender<bool>>,
    db_url: String,
}

impl Drop for TestInstance {
    fn drop(&mut self) {
        if let Some(chan) = &self.app_close_channel {
            let _ = chan.send_replace(true);
        }
    }
}

impl TestInstance {
    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }
}

pub fn default_api_key() -> &'static str {
    "a1503fb6-d79b-4e9e-826d-44cf262f3e05"
}

pub fn default_tenant_id() -> i32 {
    1
}

pub fn default_dependence_cache_size() -> u16 {
    128
}

pub fn next_handle() -> Handle {
    #[expect(non_upper_case_globals)]
    static count: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let v = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Handle::right_padding_from(&v.to_be_bytes())
}

pub async fn setup_test_app() -> Result<TestInstance, Box<dyn std::error::Error>> {
    if std::env::var("COPROCESSOR_TEST_LOCALHOST").is_ok() {
        setup_test_app_existing_localhost().await
    } else if std::env::var("COPROCESSOR_TEST_LOCAL_DB").is_ok() {
        setup_test_app_existing_db().await
    } else {
        setup_test_app_custom_docker().await
    }
}

const LOCAL_DB_URL: &str = "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor";

pub async fn setup_test_app_existing_localhost() -> Result<TestInstance, Box<dyn std::error::Error>>
{
    Ok(TestInstance {
        _container: None,
        app_close_channel: None,
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn setup_test_app_existing_db() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let health_port = get_app_port();
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, health_port, LOCAL_DB_URL).await;
    Ok(TestInstance {
        _container: None,
        app_close_channel: Some(app_close_channel),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn start_coprocessor(rx: Receiver<bool>, health_port: u16, db_url: &str) {
    let ecfg = EnvConfig::new();
    let args: Args = Args {
        run_bg_worker: true,
        worker_polling_interval_ms: 1000,
        generate_fhe_keys: false,
        work_items_batch_size: ecfg.batch_size,
        dependence_chains_per_batch: 2000,
        key_cache_size: 4,
        coprocessor_fhe_threads: 64,
        tokio_threads: 32,
        pg_pool_max_connections: 2,
        metrics_addr: None,
        database_url: Some(db_url.into()),
        service_name: "coprocessor".to_string(),
        worker_id: None,
        dcid_ttl_sec: 30,
        disable_dcid_locking: true,
        dcid_timeslice_sec: 90,
        processed_dcid_ttl_sec: 0,
        dcid_cleanup_interval_sec: 0,
        dcid_max_no_progress_cycles: 2,
        dcid_ignore_dependency_count_threshold: 100,
        log_level: Level::INFO,
        health_check_port: health_port,
        metric_rerand_batch_latency: MetricsConfig::default(),
        metric_fhe_batch_latency: MetricsConfig::default(),
    };

    std::thread::spawn(move || {
        tfhe_worker::start_runtime(args, Some(rx));
    });

    // wait until app is ready
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
}

fn get_app_port() -> u16 {
    static PORT_COUNTER: AtomicU16 = AtomicU16::new(10000);

    let app_port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
    // wrap around, if we ever have that many tests?
    if app_port >= 50000 {
        PORT_COUNTER.store(10000, Ordering::SeqCst);
    }
    app_port
}

async fn setup_test_app_custom_docker() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let health_port = get_app_port();
    let container = GenericImage::new("postgres", "15.7")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .expect("postgres started");
    let cont_host = container.get_host().await?;
    let cont_port = container.get_host_port_ipv4(5432).await?;
    let admin_db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/postgres");
    let db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/coprocessor");
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_db_url)
        .await?;
    sqlx::query!("CREATE DATABASE coprocessor;")
        .execute(&admin_pool)
        .await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    setup_test_user(&pool).await?;

    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, health_port, &db_url).await;
    Ok(TestInstance {
        _container: Some(container),
        app_close_channel: Some(app_close_channel),
        db_url,
    })
}

#[allow(dead_code)]
pub async fn wait_until_all_allowed_handles_computed(
    db_url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let count = sqlx::query!(
            "SELECT count(1) FROM computations WHERE is_allowed = TRUE AND is_completed = FALSE"
        )
        .fetch_one(&pool)
        .await?;
        let current_count = count.count.unwrap();
        if current_count == 0 {
            break;
        }
    }

    Ok(())
}

pub async fn setup_test_user(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let (sks, cks, pks, pp) = if !cfg!(feature = "gpu") {
        (
            "../fhevm-keys/sks",
            "../fhevm-keys/cks",
            "../fhevm-keys/pks",
            "../fhevm-keys/pp",
        )
    } else {
        (
            "../fhevm-keys/gpu-csks",
            "../fhevm-keys/gpu-cks",
            "../fhevm-keys/gpu-pks",
            "../fhevm-keys/gpu-pp",
        )
    };
    let sks = tokio::fs::read(sks).await.expect("can't read sks key");
    let pks = tokio::fs::read(pks).await.expect("can't read pks key");
    let cks = tokio::fs::read(cks).await.expect("can't read cks key");
    let public_params = tokio::fs::read(pp).await.expect("can't read public params");
    sqlx::query(
        "
            INSERT INTO tenants(tenant_api_key, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params, cks_key)
            VALUES (
                'a1503fb6-d79b-4e9e-826d-44cf262f3e05',
                12345,
                '0x339EcE85B9E11a3A3AA557582784a15d7F82AAf2',
                '0x69dE3158643e738a0724418b21a35FAA20CBb1c5',
                $1,
                $2,
                $3,
                $4
            )
        ",
    )
    .bind(&pks)
    .bind(&sks)
    .bind(&public_params)
    .bind(&cks)
    .execute(pool)
    .await?;

    Ok(())
}

pub struct BenchKeys {
    pub tenant_id: i32,
    pub chain_id: i64,
    pub acl_contract_address: String,
    pub pks: tfhe::CompactPublicKey,
    pub public_params: Arc<tfhe::zk::CompactPkeCrs>,
    pub cks: tfhe::ClientKey,
    pub sks: tfhe::ServerKey,
}

pub async fn query_tenant_keys<'a, T>(
    tenants_to_query: Vec<i32>,
    conn: T,
) -> Result<Vec<BenchKeys>, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    let mut res = Vec::with_capacity(tenants_to_query.len());
    let rows = sqlx::query(
        "
            SELECT tenant_id, chain_id, acl_contract_address, pks_key, sks_key, public_params, cks_key
            FROM tenants
            WHERE tenant_id = ANY($1::INT[])
        ",
    )
    .bind(&tenants_to_query)
    .fetch_all(conn)
    .await?;

    for row in rows {
        let tenant_id: i32 = row.try_get("tenant_id")?;
        let chain_id: i64 = row.try_get("chain_id")?;
        let acl_contract_address: String = row.try_get("acl_contract_address")?;
        let pks_key: Vec<u8> = row.try_get("pks_key")?;
        let sks_key: Vec<u8> = row.try_get("sks_key")?;
        let public_params: Vec<u8> = row.try_get("public_params")?;
        let cks_key: Option<Vec<u8>> = row.try_get("cks_key")?;
        #[cfg(not(feature = "gpu"))]
        {
            let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)
                .expect("We can't deserialize our own validated pks key");
            let public_params: tfhe::zk::CompactPkeCrs = safe_deserialize_key(&public_params)
                .expect("We can't deserialize our own validated public params");
            let cks: tfhe::ClientKey =
                safe_deserialize_key(&cks_key.expect("client key should be present in benches"))
                    .expect("We can't deserialize client key");
            let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key)
                .expect("We can't deserialize our own validated sks key");
            res.push(BenchKeys {
                tenant_id,
                chain_id,
                acl_contract_address,
                pks,
                public_params: Arc::new(public_params),
                cks,
                sks,
            });
        }
        #[cfg(feature = "gpu")]
        {
            let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)
                .expect("We can't deserialize our own validated pks key");
            let public_params: tfhe::zk::CompactPkeCrs = safe_deserialize_key(&public_params)
                .expect("We can't deserialize our own validated public params");
            let cks: tfhe::ClientKey =
                safe_deserialize_key(&cks_key.expect("client key should be present in benches"))
                    .expect("We can't deserialize client key");
            let csks: tfhe::CompressedServerKey = safe_deserialize_key(&sks_key)
                .expect("We can't deserialize the gpu compressed sks key");
            res.push(BenchKeys {
                tenant_id,
                chain_id,
                acl_contract_address,
                pks,
                public_params: Arc::new(public_params),
                cks,
                sks: csks.decompress(),
            });
        }
    }

    Ok(res)
}

fn derive_handle(
    blob_hash: &[u8],
    ct_idx: usize,
    acl_address: &Address,
    chain_id: u64,
    ct_type: i16,
    ciphertext_version: i16,
) -> Handle {
    let mut handle_hash = Keccak256::new();
    handle_hash.update(blob_hash);
    handle_hash.update([ct_idx as u8]);
    handle_hash.update(acl_address.as_slice());
    handle_hash.update(chain_id.to_be_bytes());
    let mut handle = handle_hash.finalize().to_vec();
    handle[29] = ct_idx as u8;
    handle[30] = ct_type as u8;
    handle[31] = ciphertext_version as u8;
    Handle::from_slice(&handle)
}

pub async fn insert_compact_list(
    pool: &PgPool,
    keys: &BenchKeys,
    serialized: &[u8],
) -> Result<Vec<Handle>, Box<dyn std::error::Error>> {
    let blob_hash = Keccak256::digest(serialized).to_vec();
    tfhe::set_server_key(keys.sks.clone());
    let expanded = try_expand_ciphertext_list(serialized, keys.public_params.as_ref())?;
    let cipher_version = current_ciphertext_version();
    let acl_address = Address::parse_checksummed(&keys.acl_contract_address, None)?;
    let chain_id = keys.chain_id as u64;
    let tenant_id = keys.tenant_id;

    let mut tx = pool.begin().await?;
    sqlx::query(
        "
            INSERT INTO input_blobs(tenant_id, blob_hash, blob_data, blob_ciphertext_count)
            VALUES($1, $2, $3, $4)
            ON CONFLICT (tenant_id, blob_hash) DO NOTHING
        ",
    )
    .bind(tenant_id)
    .bind(&blob_hash)
    .bind(serialized)
    .bind(expanded.len() as i32)
    .execute(tx.as_mut())
    .await?;

    let mut handles = Vec::with_capacity(expanded.len());
    for (ct_idx, the_ct) in expanded.into_iter().enumerate() {
        let ct_type = the_ct.type_num();
        let ct_bytes = the_ct.compress()?;
        let handle = derive_handle(
            &blob_hash,
            ct_idx,
            &acl_address,
            chain_id,
            ct_type,
            cipher_version,
        );
        sqlx::query(
            "
                INSERT INTO ciphertexts(
                    tenant_id,
                    handle,
                    ciphertext,
                    ciphertext_version,
                    ciphertext_type,
                    input_blob_hash,
                    input_blob_index
                )
                VALUES($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
            ",
        )
        .bind(tenant_id)
        .bind(handle.as_slice())
        .bind(&ct_bytes)
        .bind(cipher_version)
        .bind(ct_type)
        .bind(&blob_hash)
        .bind(ct_idx as i32)
        .execute(tx.as_mut())
        .await?;
        handles.push(handle);
    }
    tx.commit().await?;

    Ok(handles)
}

#[derive(Clone, Copy)]
#[allow(clippy::enum_variant_names)]
pub enum FheOperation {
    FheAdd,
    FheSub,
    FheMul,
    FheDiv,
    FheGe,
    FheIfThenElse,
    FheCast,
}

impl From<FheOperation> for SupportedFheOperations {
    fn from(value: FheOperation) -> Self {
        match value {
            FheOperation::FheAdd => SupportedFheOperations::FheAdd,
            FheOperation::FheSub => SupportedFheOperations::FheSub,
            FheOperation::FheMul => SupportedFheOperations::FheMul,
            FheOperation::FheDiv => SupportedFheOperations::FheDiv,
            FheOperation::FheGe => SupportedFheOperations::FheGe,
            FheOperation::FheIfThenElse => SupportedFheOperations::FheIfThenElse,
            FheOperation::FheCast => SupportedFheOperations::FheCast,
        }
    }
}

#[derive(Clone)]
pub enum Input {
    InputHandle(Handle),
    Scalar(Vec<u8>),
}

#[derive(Clone)]
pub struct AsyncComputationInput {
    pub input: Option<Input>,
}

pub struct AsyncComputation {
    pub operation: SupportedFheOperations,
    pub transaction_id: Handle,
    pub output_handle: Handle,
    pub inputs: Vec<AsyncComputationInput>,
    pub is_allowed: bool,
}

pub async fn insert_computations(
    listener: &ListenerDatabase,
    computations: &[AsyncComputation],
) -> Result<(), sqlx::Error> {
    let mut tx = listener.new_transaction().await?;
    for computation in computations {
        let event = computation_to_event(computation);
        insert_tfhe_event(
            listener,
            &mut tx,
            tfhe_log(event, computation.transaction_id),
            computation.is_allowed,
        )
        .await?;
        if computation.is_allowed {
            allow_handle(listener, &mut tx, computation.output_handle.as_ref()).await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

fn computation_to_event(computation: &AsyncComputation) -> TfheContractEvents {
    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    match computation.operation {
        SupportedFheOperations::FheIfThenElse => {
            let control = handle_from_input(&computation.inputs[0]);
            let if_true = handle_from_input(&computation.inputs[1]);
            let if_false = handle_from_input(&computation.inputs[2]);
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                caller,
                control,
                ifTrue: if_true,
                ifFalse: if_false,
                result: computation.output_handle,
            })
        }
        SupportedFheOperations::FheCast => {
            let ct = handle_from_input(&computation.inputs[0]);
            let to_type = scalar_to_u8(&computation.inputs[1]);
            TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct,
                toType: to_type,
                result: computation.output_handle,
            })
        }
        SupportedFheOperations::FheAdd
        | SupportedFheOperations::FheSub
        | SupportedFheOperations::FheMul
        | SupportedFheOperations::FheDiv
        | SupportedFheOperations::FheGe => {
            let lhs = handle_from_input(&computation.inputs[0]);
            let (rhs, scalar_byte) = rhs_with_scalar_flag(&computation.inputs[1]);
            match computation.operation {
                SupportedFheOperations::FheAdd => {
                    TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs,
                        rhs,
                        scalarByte: scalar_byte,
                        result: computation.output_handle,
                    })
                }
                SupportedFheOperations::FheSub => {
                    TfheContractEvents::FheSub(TfheContract::FheSub {
                        caller,
                        lhs,
                        rhs,
                        scalarByte: scalar_byte,
                        result: computation.output_handle,
                    })
                }
                SupportedFheOperations::FheMul => {
                    TfheContractEvents::FheMul(TfheContract::FheMul {
                        caller,
                        lhs,
                        rhs,
                        scalarByte: scalar_byte,
                        result: computation.output_handle,
                    })
                }
                SupportedFheOperations::FheDiv => {
                    TfheContractEvents::FheDiv(TfheContract::FheDiv {
                        caller,
                        lhs,
                        rhs,
                        scalarByte: scalar_byte,
                        result: computation.output_handle,
                    })
                }
                SupportedFheOperations::FheGe => TfheContractEvents::FheGe(TfheContract::FheGe {
                    caller,
                    lhs,
                    rhs,
                    scalarByte: scalar_byte,
                    result: computation.output_handle,
                }),
                _ => unreachable!("unsupported binary op"),
            }
        }
        _ => panic!("unsupported operation {:?}", computation.operation),
    }
}

fn handle_from_input(input: &AsyncComputationInput) -> Handle {
    match input.input.as_ref().expect("input missing") {
        Input::InputHandle(handle) => *handle,
        Input::Scalar(bytes) => scalar_handle(bytes),
    }
}

fn scalar_to_u8(input: &AsyncComputationInput) -> u8 {
    match input.input.as_ref().expect("input missing") {
        Input::Scalar(bytes) => *bytes.last().unwrap_or(&0),
        _ => panic!("expected scalar input"),
    }
}

fn rhs_with_scalar_flag(input: &AsyncComputationInput) -> (Handle, ScalarByte) {
    match input.input.as_ref().expect("input missing") {
        Input::InputHandle(handle) => (*handle, ScalarByte::from(0u8)),
        Input::Scalar(bytes) => (scalar_handle(bytes), ScalarByte::from(1u8)),
    }
}

fn scalar_handle(bytes: &[u8]) -> Handle {
    let mut buf = [0u8; 32];
    let len = bytes.len().min(32);
    buf[32 - len..].copy_from_slice(&bytes[bytes.len() - len..]);
    Handle::from_slice(&buf)
}

fn tfhe_event(data: TfheContractEvents) -> PrimitivesLog<TfheContractEvents> {
    let address = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    PrimitivesLog::<TfheContractEvents> { address, data }
}

fn tfhe_log(event: TfheContractEvents, transaction_hash: Handle) -> RpcLog<TfheContractEvents> {
    RpcLog {
        inner: tfhe_event(event),
        block_hash: None,
        block_number: None,
        block_timestamp: None,
        transaction_hash: Some(transaction_hash),
        transaction_index: Some(0),
        log_index: None,
        removed: false,
    }
}

pub async fn listener_db(app: &TestInstance) -> ListenerDatabase {
    ListenerDatabase::new(
        &app.db_url().into(),
        ChainId::try_from(12345_u64).unwrap(),
        default_dependence_cache_size(),
    )
    .await
    .unwrap()
}

async fn insert_tfhe_event(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    log: RpcLog<TfheContractEvents>,
    is_allowed: bool,
) -> Result<bool, sqlx::Error> {
    let event = LogTfhe {
        event: log.inner,
        transaction_hash: log.transaction_hash,
        is_allowed,
        block_number: log.block_number.unwrap_or(0),
        block_timestamp: time::PrimitiveDateTime::MAX,
        dependence_chain: log.transaction_hash.unwrap_or_default(),
        tx_depth_size: 0,
    };
    db.insert_tfhe_event(tx, &event).await
}

async fn allow_handle(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    handle: &[u8],
) -> Result<bool, sqlx::Error> {
    let account_address = String::new();
    let event_type = fhevm_engine_common::types::AllowEvents::AllowedForDecryption;
    db.insert_allowed_handle(tx, handle.to_owned(), account_address, event_type, None)
        .await
}

use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::{env, fs};
use tfhe::core_crypto::prelude::*;

pub mod shortint_utils {
    use super::*;
    use tfhe::shortint::parameters::compact_public_key_only::CompactPublicKeyEncryptionParameters;
    use tfhe::shortint::parameters::list_compression::CompressionParameters;
    use tfhe::shortint::parameters::ShortintKeySwitchingParameters;
    use tfhe::shortint::{
        AtomicPatternParameters, CarryModulus, ClassicPBSParameters, MessageModulus,
        MultiBitPBSParameters, PBSParameters, ShortintParameterSet,
    };

    impl From<PBSParameters> for CryptoParametersRecord<u64> {
        fn from(params: PBSParameters) -> Self {
            CryptoParametersRecord {
                lwe_dimension: Some(params.lwe_dimension()),
                glwe_dimension: Some(params.glwe_dimension()),
                polynomial_size: Some(params.polynomial_size()),
                lwe_noise_distribution: Some(params.lwe_noise_distribution()),
                glwe_noise_distribution: Some(params.glwe_noise_distribution()),
                pbs_base_log: Some(params.pbs_base_log()),
                pbs_level: Some(params.pbs_level()),
                ks_base_log: Some(params.ks_base_log()),
                ks_level: Some(params.ks_level()),
                message_modulus: Some(params.message_modulus().0),
                carry_modulus: Some(params.carry_modulus().0),
                ciphertext_modulus: Some(
                    params
                        .ciphertext_modulus()
                        .try_to()
                        .expect("failed to convert ciphertext modulus"),
                ),
                ..Default::default()
            }
        }
    }

    impl From<ShortintKeySwitchingParameters> for CryptoParametersRecord<u64> {
        fn from(params: ShortintKeySwitchingParameters) -> Self {
            CryptoParametersRecord {
                ks_base_log: Some(params.ks_base_log),
                ks_level: Some(params.ks_level),
                ..Default::default()
            }
        }
    }

    impl From<CompactPublicKeyEncryptionParameters> for CryptoParametersRecord<u64> {
        fn from(params: CompactPublicKeyEncryptionParameters) -> Self {
            CryptoParametersRecord {
                message_modulus: Some(params.message_modulus.0),
                carry_modulus: Some(params.carry_modulus.0),
                ciphertext_modulus: Some(params.ciphertext_modulus),
                ..Default::default()
            }
        }
    }

    impl From<(CompressionParameters, ClassicPBSParameters)> for CryptoParametersRecord<u64> {
        fn from((comp_params, pbs_params): (CompressionParameters, ClassicPBSParameters)) -> Self {
            (comp_params, PBSParameters::PBS(pbs_params)).into()
        }
    }

    impl From<(CompressionParameters, MultiBitPBSParameters)> for CryptoParametersRecord<u64> {
        fn from(
            (comp_params, multi_bit_pbs_params): (CompressionParameters, MultiBitPBSParameters),
        ) -> Self {
            (
                comp_params,
                PBSParameters::MultiBitPBS(multi_bit_pbs_params),
            )
                .into()
        }
    }

    impl From<(CompressionParameters, PBSParameters)> for CryptoParametersRecord<u64> {
        fn from((comp_params, pbs_params): (CompressionParameters, PBSParameters)) -> Self {
            let pbs_params = ShortintParameterSet::new_pbs_param_set(pbs_params);
            let lwe_dimension = pbs_params.encryption_lwe_dimension();
            CryptoParametersRecord {
                lwe_dimension: Some(lwe_dimension),
                br_level: Some(comp_params.br_level()),
                br_base_log: Some(comp_params.br_base_log()),
                packing_ks_level: Some(comp_params.packing_ks_level()),
                packing_ks_base_log: Some(comp_params.packing_ks_base_log()),
                packing_ks_polynomial_size: Some(comp_params.packing_ks_polynomial_size()),
                packing_ks_glwe_dimension: Some(comp_params.packing_ks_glwe_dimension()),
                lwe_per_glwe: Some(comp_params.lwe_per_glwe()),
                storage_log_modulus: Some(comp_params.storage_log_modulus()),
                lwe_noise_distribution: Some(pbs_params.encryption_noise_distribution()),
                packing_ks_key_noise_distribution: Some(
                    comp_params.packing_ks_key_noise_distribution(),
                ),
                ciphertext_modulus: Some(pbs_params.ciphertext_modulus()),
                ..Default::default()
            }
        }
    }

    impl From<AtomicPatternParameters> for CryptoParametersRecord<u64> {
        fn from(params: AtomicPatternParameters) -> Self {
            CryptoParametersRecord {
                lwe_dimension: Some(params.lwe_dimension()),
                glwe_dimension: Some(params.glwe_dimension()),
                polynomial_size: Some(params.polynomial_size()),
                lwe_noise_distribution: Some(params.lwe_noise_distribution()),
                glwe_noise_distribution: Some(params.glwe_noise_distribution()),
                pbs_base_log: Some(params.pbs_base_log()),
                pbs_level: Some(params.pbs_level()),
                ks_base_log: Some(params.ks_base_log()),
                ks_level: Some(params.ks_level()),
                message_modulus: Some(params.message_modulus().0),
                carry_modulus: Some(params.carry_modulus().0),
                ciphertext_modulus: Some(
                    params
                        .ciphertext_modulus()
                        .try_to()
                        .expect("failed to convert ciphertext modulus"),
                ),
                ..Default::default()
            }
        }
    }

    const MULTI_BIT_THREADS_ARRAY: [((MessageModulus, CarryModulus, LweBskGroupingFactor), u64);
        12] = [
        (
            (MessageModulus(2), CarryModulus(2), LweBskGroupingFactor(2)),
            5,
        ),
        (
            (MessageModulus(4), CarryModulus(4), LweBskGroupingFactor(2)),
            5,
        ),
        (
            (MessageModulus(8), CarryModulus(8), LweBskGroupingFactor(2)),
            5,
        ),
        (
            (
                MessageModulus(16),
                CarryModulus(16),
                LweBskGroupingFactor(2),
            ),
            5,
        ),
        (
            (MessageModulus(2), CarryModulus(2), LweBskGroupingFactor(3)),
            7,
        ),
        (
            (MessageModulus(4), CarryModulus(4), LweBskGroupingFactor(3)),
            9,
        ),
        (
            (MessageModulus(8), CarryModulus(8), LweBskGroupingFactor(3)),
            10,
        ),
        (
            (
                MessageModulus(16),
                CarryModulus(16),
                LweBskGroupingFactor(3),
            ),
            10,
        ),
        (
            (MessageModulus(2), CarryModulus(2), LweBskGroupingFactor(4)),
            11,
        ),
        (
            (MessageModulus(4), CarryModulus(4), LweBskGroupingFactor(4)),
            13,
        ),
        (
            (MessageModulus(8), CarryModulus(8), LweBskGroupingFactor(4)),
            11,
        ),
        (
            (
                MessageModulus(16),
                CarryModulus(16),
                LweBskGroupingFactor(4),
            ),
            11,
        ),
    ];

    #[allow(dead_code)]
    pub fn multi_bit_num_threads(
        message_modulus: u64,
        carry_modulus: u64,
        grouping_factor: usize,
    ) -> Option<u64> {
        if message_modulus != carry_modulus || [2, 3, 4].contains(&(grouping_factor as i32)) {
            return None;
        }
        let thread_map: HashMap<(MessageModulus, CarryModulus, LweBskGroupingFactor), u64> =
            HashMap::from_iter(MULTI_BIT_THREADS_ARRAY);
        thread_map
            .get(&(
                MessageModulus(message_modulus),
                CarryModulus(carry_modulus),
                LweBskGroupingFactor(grouping_factor),
            ))
            .copied()
    }

    #[allow(dead_code)]
    pub static PARAMETERS_SET: OnceLock<ParametersSet> = OnceLock::new();

    pub enum ParametersSet {
        Default,
        All,
    }

    #[allow(dead_code)]
    impl ParametersSet {
        pub fn from_env() -> Result<Self, String> {
            let raw_value = env::var("__TFHE_RS_PARAMS_SET").unwrap_or("default".to_string());
            match raw_value.to_lowercase().as_str() {
                "default" => Ok(ParametersSet::Default),
                "all" => Ok(ParametersSet::All),
                _ => Err(format!("parameters set '{raw_value}' is not supported")),
            }
        }
    }

    #[allow(dead_code)]
    pub fn init_parameters_set() {
        PARAMETERS_SET.get_or_init(|| ParametersSet::from_env().unwrap());
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug)]
    pub enum DesiredNoiseDistribution {
        Gaussian,
        TUniform,
        Both,
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug)]
    pub enum DesiredBackend {
        Cpu,
        Gpu,
    }

    #[allow(dead_code)]
    impl DesiredBackend {
        fn matches_parameter_name_backend(&self, param_name: &str) -> bool {
            matches!(
                (self, param_name.to_lowercase().contains("gpu")),
                (DesiredBackend::Cpu, false) | (DesiredBackend::Gpu, true)
            )
        }
    }

    #[allow(dead_code)]
    pub fn filter_parameters<'a, P: Copy + Into<PBSParameters>>(
        params: &[(&'a P, &'a str)],
        desired_noise_distribution: DesiredNoiseDistribution,
        desired_backend: DesiredBackend,
    ) -> Vec<(&'a P, &'a str)> {
        params
            .iter()
            .filter_map(|(p, name)| {
                let temp_param: PBSParameters = (**p).into();

                match (
                    temp_param.lwe_noise_distribution(),
                    desired_noise_distribution,
                ) {
                    (DynamicDistribution::Gaussian(_), DesiredNoiseDistribution::Gaussian)
                    | (DynamicDistribution::TUniform(_), DesiredNoiseDistribution::TUniform)
                    | (_, DesiredNoiseDistribution::Both) => (),
                    _ => return None,
                }

                if !desired_backend.matches_parameter_name_backend(name) {
                    return None;
                };

                Some((*p, *name))
            })
            .collect()
    }
}

#[derive(Clone, Copy, Default, Serialize)]
pub struct CryptoParametersRecord<Scalar: UnsignedInteger> {
    pub lwe_dimension: Option<LweDimension>,
    pub glwe_dimension: Option<GlweDimension>,
    pub packing_ks_glwe_dimension: Option<GlweDimension>,
    pub polynomial_size: Option<PolynomialSize>,
    pub packing_ks_polynomial_size: Option<PolynomialSize>,
    #[serde(serialize_with = "CryptoParametersRecord::serialize_distribution")]
    pub lwe_noise_distribution: Option<DynamicDistribution<Scalar>>,
    #[serde(serialize_with = "CryptoParametersRecord::serialize_distribution")]
    pub glwe_noise_distribution: Option<DynamicDistribution<Scalar>>,
    #[serde(serialize_with = "CryptoParametersRecord::serialize_distribution")]
    pub packing_ks_key_noise_distribution: Option<DynamicDistribution<Scalar>>,
    pub pbs_base_log: Option<DecompositionBaseLog>,
    pub pbs_level: Option<DecompositionLevelCount>,
    pub ks_base_log: Option<DecompositionBaseLog>,
    pub ks_level: Option<DecompositionLevelCount>,
    pub pfks_level: Option<DecompositionLevelCount>,
    pub pfks_base_log: Option<DecompositionBaseLog>,
    pub pfks_std_dev: Option<StandardDev>,
    pub cbs_level: Option<DecompositionLevelCount>,
    pub cbs_base_log: Option<DecompositionBaseLog>,
    pub br_level: Option<DecompositionLevelCount>,
    pub br_base_log: Option<DecompositionBaseLog>,
    pub packing_ks_level: Option<DecompositionLevelCount>,
    pub packing_ks_base_log: Option<DecompositionBaseLog>,
    pub message_modulus: Option<u64>,
    pub carry_modulus: Option<u64>,
    pub ciphertext_modulus: Option<CiphertextModulus<Scalar>>,
    pub lwe_per_glwe: Option<LweCiphertextCount>,
    pub storage_log_modulus: Option<CiphertextModulusLog>,
}

impl<Scalar: UnsignedInteger> CryptoParametersRecord<Scalar> {
    pub fn noise_distribution_as_string(noise_distribution: DynamicDistribution<Scalar>) -> String {
        match noise_distribution {
            DynamicDistribution::Gaussian(g) => format!("Gaussian({}, {})", g.std, g.mean),
            DynamicDistribution::TUniform(t) => format!("TUniform({})", t.bound_log2()),
        }
    }

    pub fn serialize_distribution<S>(
        noise_distribution: &Option<DynamicDistribution<Scalar>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match noise_distribution {
            Some(d) => serializer.serialize_some(&Self::noise_distribution_as_string(*d)),
            None => serializer.serialize_none(),
        }
    }
}

#[derive(Serialize)]
enum PolynomialMultiplication {
    Fft,
}

#[derive(Serialize)]
enum IntegerRepresentation {
    Radix,
}

#[derive(Serialize)]
enum ExecutionType {
    Sequential,
    Parallel,
}

#[derive(Serialize)]
enum KeySetType {
    Single,
}

#[derive(Serialize)]
enum OperandType {
    CipherText,
    PlainText,
}

#[derive(Clone, Serialize)]
pub enum OperatorType {
    Atomic,
}

#[derive(Serialize)]
struct BenchmarkParametersRecord<Scalar: UnsignedInteger> {
    display_name: String,
    crypto_parameters_alias: String,
    crypto_parameters: CryptoParametersRecord<Scalar>,
    message_modulus: Option<u64>,
    carry_modulus: Option<u64>,
    ciphertext_modulus: usize,
    bit_size: u32,
    polynomial_multiplication: PolynomialMultiplication,
    precision: u32,
    error_probability: f64,
    integer_representation: IntegerRepresentation,
    decomposition_basis: Vec<u32>,
    pbs_algorithm: Option<String>,
    execution_type: ExecutionType,
    key_set_type: KeySetType,
    operand_type: OperandType,
    operator_type: OperatorType,
}

pub fn write_to_json<
    Scalar: UnsignedInteger + Serialize,
    T: Into<CryptoParametersRecord<Scalar>>,
>(
    bench_id: &str,
    params: T,
    params_alias: impl Into<String>,
    display_name: impl Into<String>,
    operator_type: &OperatorType,
    bit_size: u32,
    decomposition_basis: Vec<u32>,
) {
    let params = params.into();

    let execution_type = match bench_id.contains("parallelized") {
        true => ExecutionType::Parallel,
        false => ExecutionType::Sequential,
    };
    let operand_type = match bench_id.contains("scalar") {
        true => OperandType::PlainText,
        false => OperandType::CipherText,
    };

    let record = BenchmarkParametersRecord {
        display_name: display_name.into(),
        crypto_parameters_alias: params_alias.into(),
        crypto_parameters: params.to_owned(),
        message_modulus: params.message_modulus,
        carry_modulus: params.carry_modulus,
        ciphertext_modulus: 64,
        bit_size,
        polynomial_multiplication: PolynomialMultiplication::Fft,
        precision: (params.message_modulus.unwrap_or(2) as u32).ilog2(),
        error_probability: 2f64.powf(-41.0),
        integer_representation: IntegerRepresentation::Radix,
        decomposition_basis,
        pbs_algorithm: None,
        execution_type,
        key_set_type: KeySetType::Single,
        operand_type,
        operator_type: operator_type.to_owned(),
    };

    let mut params_directory = ["benchmarks_parameters", bench_id]
        .iter()
        .collect::<PathBuf>();
    fs::create_dir_all(&params_directory).unwrap();
    params_directory.push("parameters.json");

    fs::write(params_directory, serde_json::to_string(&record).unwrap()).unwrap();
}

#[allow(dead_code)]
#[cfg(feature = "gpu")]
pub const GPU_MAX_SUPPORTED_POLYNOMIAL_SIZE: usize = 16384;

const FAST_BENCH_BIT_SIZES: [usize; 1] = [64];
const BENCH_BIT_SIZES: [usize; 8] = [4, 8, 16, 32, 40, 64, 128, 256];
const MULTI_BIT_CPU_SIZES: [usize; 6] = [4, 8, 16, 32, 40, 64];

#[derive(Default)]
pub struct EnvConfig {
    pub is_multi_bit: bool,
    pub is_fast_bench: bool,
    pub batch_size: i32,
    #[allow(dead_code)]
    pub scheduling_policy: String,
    pub benchmark_type: String,
    #[allow(dead_code)]
    pub optimization_target: String,
}

impl EnvConfig {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let is_multi_bit = match env::var("__TFHE_RS_PARAM_TYPE") {
            Ok(val) => val.to_lowercase() == "multi_bit",
            Err(_) => false,
        };
        let is_fast_bench = match env::var("__TFHE_RS_FAST_BENCH") {
            Ok(val) => val.to_lowercase() == "true",
            Err(_) => false,
        };
        let batch_size: i32 = match env::var("BENCHMARK_BATCH_SIZE") {
            Ok(val) => val.parse::<i32>().unwrap(),
            Err(_) => 4000,
        };
        let scheduling_policy: String = match env::var("FHEVM_DF_SCHEDULE") {
            Ok(val) => val,
            Err(_) => "MAX_PARALLELISM".to_string(),
        };
        let benchmark_type: String = match env::var("BENCHMARK_TYPE") {
            Ok(val) => val,
            Err(_) => "ALL".to_string(),
        };
        let optimization_target: String = match env::var("OPTIMIZATION_TARGET") {
            Ok(val) => val,
            Err(_) => "throughput".to_string(),
        };

        EnvConfig {
            is_multi_bit,
            is_fast_bench,
            batch_size,
            scheduling_policy,
            benchmark_type,
            optimization_target,
        }
    }

    #[allow(dead_code)]
    pub fn bit_sizes(&self) -> Vec<usize> {
        if self.is_fast_bench {
            FAST_BENCH_BIT_SIZES.to_vec()
        } else if self.is_multi_bit {
            if cfg!(feature = "gpu") {
                BENCH_BIT_SIZES.to_vec()
            } else {
                MULTI_BIT_CPU_SIZES.to_vec()
            }
        } else {
            BENCH_BIT_SIZES.to_vec()
        }
    }
}
