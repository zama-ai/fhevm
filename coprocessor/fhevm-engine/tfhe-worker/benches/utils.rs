use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::{chain_id::ChainId, types::AllowEvents};
use rand::Rng;
use test_harness::db_utils::setup_test_key;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tfhe_worker::daemon_cli::Args;
use tokio::sync::watch::Receiver;
use tracing::Level;

use alloy::primitives::{FixedBytes, Log};
use bigdecimal::num_bigint::BigInt;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    ClearConst, Database as ListenerDatabase, Handle, LogTfhe, ToType, Transaction,
};
use sqlx::types::time::PrimitiveDateTime;

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

pub fn random_handle() -> u64 {
    rand::rng().random()
}

pub async fn setup_test_app() -> Result<TestInstance, Box<dyn std::error::Error>> {
    if std::env::var("COPROCESSOR_TEST_LOCAL_DB").is_ok() {
        setup_test_app_existing_db().await
    } else {
        setup_test_app_custom_docker().await
    }
}

const LOCAL_DB_URL: &str = "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor";

async fn setup_test_app_existing_db() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, LOCAL_DB_URL).await;
    Ok(TestInstance {
        _container: None,
        app_close_channel: Some(app_close_channel),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn start_coprocessor(rx: Receiver<bool>, db_url: &str) {
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
        log_level: Level::INFO,
        health_check_port: 8080,
        metric_rerand_batch_latency: MetricsConfig::default(),
        metric_fhe_batch_latency: MetricsConfig::default(),
        worker_id: None,
        dcid_ttl_sec: 30,
        disable_dcid_locking: true,
        dcid_timeslice_sec: 90,
        dcid_cleanup_interval_sec: 0,
        processed_dcid_ttl_sec: 0,
        dcid_max_no_progress_cycles: 2,
        dcid_ignore_dependency_count_threshold: 100,
    };

    std::thread::spawn(move || {
        tfhe_worker::start_runtime(args, Some(rx));
    });

    // wait until app port is opened
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
}

async fn setup_test_app_custom_docker() -> Result<TestInstance, Box<dyn std::error::Error>> {
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
    setup_test_key(&pool, false).await?;

    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, &db_url).await;
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

pub fn to_ty(ty: i32) -> ToType {
    ToType::from(ty as u8)
}

pub fn as_scalar_uint(big_int: &BigInt) -> ClearConst {
    let (_, bytes) = big_int.to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

pub fn as_handle(v: u64) -> Handle {
    let mut out = [0_u8; 32];
    out[24..32].copy_from_slice(&v.to_be_bytes());
    Handle::from(out)
}

pub fn next_handle(counter: &mut u64) -> Handle {
    let out = as_handle(*counter);
    *counter += 1;
    out
}

pub fn tfhe_event(data: TfheContractEvents) -> Log<TfheContractEvents> {
    let address = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    Log::<TfheContractEvents> { address, data }
}

pub async fn listener_event_db(
    app: &TestInstance,
) -> Result<ListenerDatabase, Box<dyn std::error::Error>> {
    Ok(ListenerDatabase::new(
        &app.db_url().into(),
        ChainId::try_from(42_u64).unwrap(),
        default_dependence_cache_size(),
    )
    .await?)
}

pub fn default_dependence_cache_size() -> u16 {
    128
}

pub async fn insert_tfhe_event(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    log: alloy::rpc::types::Log<TfheContractEvents>,
    tx_hash: Handle,
    is_allowed: bool,
) -> Result<bool, sqlx::Error> {
    let event = LogTfhe {
        event: log.inner,
        transaction_hash: Some(tx_hash),
        is_allowed,
        block_number: log.block_number.unwrap_or(0),
        block_timestamp: PrimitiveDateTime::MAX,
        dependence_chain: tx_hash,
        tx_depth_size: 0,
    };
    db.insert_tfhe_event(tx, &event).await
}

pub async fn allow_handle(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    handle: &Handle,
) -> Result<bool, sqlx::Error> {
    db.insert_allowed_handle(
        tx,
        handle.to_vec(),
        String::new(),
        AllowEvents::AllowedForDecryption,
        None,
    )
    .await
}

pub fn zero_address() -> alloy::primitives::Address {
    "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap()
}

pub fn scalar_flag(is_scalar: bool) -> FixedBytes<1> {
    FixedBytes::from([if is_scalar { 1_u8 } else { 0_u8 }])
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

    // This array has been built according to performance benchmarks measuring latency over a
    // matrix of 4 parameters set, 3 grouping factor and a wide range of threads values.
    // The values available here as u64 are the optimal number of threads to use for a given triplet
    // representing one or more parameters set.
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

    /// Define the number of threads to use for  parameters doing multithreaded programmable
    /// bootstrapping.
    ///
    /// Parameters must have the same values between message and carry modulus.
    /// Grouping factor 2, 3 and 4 are the only ones that are supported.
    #[allow(dead_code)]
    pub fn multi_bit_num_threads(
        message_modulus: u64,
        carry_modulus: u64,
        grouping_factor: usize,
    ) -> Option<u64> {
        // TODO Implement an interpolation mechanism for X_Y parameters set
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
                    // If it's one of the pairs, we continue the process.
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
    // Ntt,
}

#[derive(Serialize)]
enum IntegerRepresentation {
    Radix,
    // Crt,
    // Hybrid,
}

#[derive(Serialize)]
enum ExecutionType {
    Sequential,
    Parallel,
}

#[derive(Serialize)]
enum KeySetType {
    Single,
    // Multi,
}

#[derive(Serialize)]
enum OperandType {
    CipherText,
    PlainText,
}

#[derive(Clone, Serialize)]
pub enum OperatorType {
    Atomic,
    // AtomicPattern,
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

/// Writes benchmarks parameters to disk in JSON format.
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
        pbs_algorithm: None, // To be added in future version
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

/// User configuration in which benchmarks must be run.
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

    /// Get precisions values to benchmark.
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
