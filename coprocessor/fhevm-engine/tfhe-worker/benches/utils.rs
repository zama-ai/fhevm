#![allow(dead_code)]

use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::{chain_id::ChainId, types::AllowEvents};
use rand::Rng;
use test_harness::db_utils::setup_test_key;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tfhe_worker::benchmark_exact_tuples::wait_for_exact_legacy_terminals;
use tfhe_worker::daemon_cli::Args;
use tokio::sync::watch::Receiver;
use tracing::Level;

use alloy::primitives::{FixedBytes, Log};
use bigdecimal::num_bigint::BigInt;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    operand_boundary_mask_from_minted, ClearConst, Database as ListenerDatabase, Handle, LogTfhe,
    ToType, Transaction,
};
use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;
use std::thread::JoinHandle;
use std::time::SystemTime;

pub struct TestInstance {
    // just to destroy container
    _container: Option<testcontainers::ContainerAsync<testcontainers::GenericImage>>,
    // send message to this on destruction to stop the app
    app_close_channel: Option<tokio::sync::watch::Sender<bool>>,
    worker_thread: Option<JoinHandle<()>>,
    db_url: String,
}

impl Drop for TestInstance {
    fn drop(&mut self) {
        if let Some(chan) = self.app_close_channel.take() {
            let _ = chan.send_replace(true);
        }
        if let Some(worker_thread) = self.worker_thread.take() {
            std::thread::spawn(move || {
                let _ = worker_thread.join();
            });
        }
    }
}

impl TestInstance {
    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(chan) = self.app_close_channel.take() {
            let _ = chan.send_replace(true);
        }
        let Some(worker_thread) = self.worker_thread.take() else {
            return Ok(());
        };
        let timeout = benchmark_shutdown_timeout()?;
        match tokio::time::timeout(
            timeout,
            tokio::task::spawn_blocking(move || worker_thread.join()),
        )
        .await
        {
            Ok(Ok(Ok(()))) => Ok(()),
            Ok(Ok(Err(_))) => Err("main_block_baseline worker panicked during shutdown".into()),
            Ok(Err(error)) => Err(format!("join main_block_baseline worker: {error}").into()),
            Err(_) => Err(format!(
                "timed out after {}s joining main_block_baseline worker",
                timeout.as_secs()
            )
            .into()),
        }
    }
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
    let db_url = local_benchmark_db_url()?;
    ensure_benchmark_keys(&db_url).await?;
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    let worker_thread = start_coprocessor(rx, &db_url).await?;
    Ok(TestInstance {
        _container: None,
        app_close_channel: Some(app_close_channel),
        worker_thread: Some(worker_thread),
        db_url,
    })
}

/// Seed the FHE keys the worker needs when the target database has none.
///
/// The docker path gets them from the test harness's import mode, but a local
/// database is prepared by `make init_db`, which runs migrations and seeds host
/// chains only. Without this a reportable run starts against a keyless database
/// and the worker cycles on "No keys found in database" until the run's wait
/// budget expires — a wedge whose cause is several layers away from its symptom.
async fn ensure_benchmark_keys(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(db_url)
        .await?;
    let key_count: i64 = sqlx::query_scalar("SELECT count(1) FROM keys")
        .fetch_one(&pool)
        .await?;
    if key_count > 0 {
        return Ok(());
    }
    println!("benchmark database has no keys; importing the test keys");
    // The import also inserts the test host chain, and inserts it last, so on a
    // database whose migration already seeded that same chain and ACL it fails
    // on the duplicate after the keys are already in. Clear the seeded row
    // first: the import writes it back with identical contents.
    sqlx::query("DELETE FROM host_chains")
        .execute(&pool)
        .await?;
    // Without the SnS key, matching the docker path: it belongs to sns-worker,
    // and importing it would add well over a gigabyte to every scenario that
    // starts from a freshly recreated database.
    setup_test_key(&pool, false).await?;
    Ok(())
}

fn local_benchmark_db_url() -> Result<String, Box<dyn std::error::Error>> {
    if std::env::var("FHEVM_BENCH_ISOLATED_DB").as_deref() != Ok("1") {
        return Err("COPROCESSOR_TEST_LOCAL_DB requires FHEVM_BENCH_ISOLATED_DB=1".into());
    }
    let url = std::env::var("FHEVM_BENCH_DATABASE_URL")?;
    if url.contains("application_name=") {
        return Err("FHEVM_BENCH_DATABASE_URL must not set application_name".into());
    }
    Ok(url)
}

async fn start_coprocessor(
    rx: Receiver<bool>,
    db_url: &str,
) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let application_name = format!("main-block-baseline-{:016x}", random_handle());
    let args: Args = Args {
        run_bg_worker: true,
        worker_polling_interval_ms: 1000,
        bridge_polling_interval_ms: 1000,
        bridge_associate_batch_size: 128,
        generate_fhe_keys: false,
        // Reportable one-shot targets can raise these independently.  Keep
        // the historical defaults for all other benchmark/test callers.
        work_items_batch_size: benchmark_work_items_batch_size(ecfg.batch_size)?,
        dependence_chains_per_batch: benchmark_dependence_chains_per_batch(2000)?,
        dcid_batch_execution: benchmark_dcid_batch_execution()?,
        dcid_adaptive_batch_execution: benchmark_dcid_adaptive_batch_execution()?,
        key_cache_size: 4,
        coprocessor_fhe_threads: 64,
        gpu_streams_per_device: benchmark_gpu_streams_per_device()?,
        tokio_threads: 32,
        pg_pool_max_connections: 8,
        metrics_addr: None,
        database_url: Some(fhevm_engine_common::utils::DatabaseURL::new_with_app_name(
            db_url,
            &application_name,
        )),
        service_name: std::env::var("OTEL_SERVICE_NAME").unwrap_or_default(),
        log_level: Level::INFO,
        health_check_port: test_harness::localstack::pick_free_port(),
        metric_rerand_batch_latency: MetricsConfig::default(),
        metric_fhe_batch_latency: MetricsConfig::default(),
        worker_id: None,
        dcid_ttl_sec: 30,
        disable_dcid_locking: false,
        dcid_timeslice_sec: 90,
        dcid_cleanup_interval_sec: 0,
        // Retain processed rows long enough for the direct smoke to prove the
        // native first->second DCID lifecycle. They are unique fixture IDs and
        // cannot be re-acquired once processed.
        processed_dcid_ttl_sec: 3600,
        dcid_max_no_progress_cycles: 2,
        dcid_ignore_dependency_count_threshold: 100,
        gpu_memory_reservation_timeout_ms: 300_000,
        drift_revert_watcher_timeouts: Default::default(),
        stack_version: false,
    };

    let (readiness_tx, readiness_rx) = tokio::sync::oneshot::channel();
    let worker_thread = std::thread::spawn(move || {
        tfhe_worker::start_runtime_with_readiness(args, Some(rx), Some(readiness_tx));
    });
    wait_for_worker_readiness(readiness_rx, &application_name, &worker_thread).await?;
    Ok(worker_thread)
}

fn benchmark_gpu_streams_per_device() -> Result<usize, Box<dyn std::error::Error>> {
    match std::env::var("FHEVM_GPU_STREAMS_PER_DEVICE") {
        Ok(value) => {
            let streams = value.parse::<usize>()?;
            if streams == 0 {
                return Err("FHEVM_GPU_STREAMS_PER_DEVICE must be at least 1".into());
            }
            Ok(streams)
        }
        Err(std::env::VarError::NotPresent) => Ok(16),
        Err(error) => Err(error.into()),
    }
}

pub fn benchmark_work_items_batch_size(default: i32) -> Result<i32, Box<dyn std::error::Error>> {
    benchmark_positive_i32("FHEVM_BENCH_WORK_ITEMS_BATCH_SIZE", default)
}

pub fn benchmark_dependence_chains_per_batch(
    default: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    benchmark_positive_i32("FHEVM_BENCH_DEPENDENCE_CHAINS_PER_BATCH", default)
}

pub fn benchmark_dcid_batch_execution() -> Result<bool, Box<dyn std::error::Error>> {
    // The production setting is the source of truth. Retain the former
    // benchmark-only variable as a fallback so older invocation scripts keep
    // producing their explicitly requested configuration.
    for variable in [
        "FHEVM_DCID_BATCH_EXECUTION",
        "FHEVM_BENCH_DCID_BATCH_EXECUTION",
    ] {
        match std::env::var(variable) {
            Ok(value) => {
                return value
                    .parse::<bool>()
                    .map_err(|error| format!("parse {variable}={value:?}: {error}").into());
            }
            Err(std::env::VarError::NotPresent) => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(true)
}

pub fn benchmark_dcid_adaptive_batch_execution() -> Result<bool, Box<dyn std::error::Error>> {
    // Same precedence as the batch-execution setting: the production variable
    // wins, with a benchmark-only fallback. Defaults to the production default
    // so a reportable run measures the shipped configuration unless a run
    // deliberately asks for the adaptive window.
    for variable in [
        "FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION",
        "FHEVM_BENCH_DCID_ADAPTIVE_BATCH_EXECUTION",
    ] {
        match std::env::var(variable) {
            Ok(value) => {
                return value
                    .parse::<bool>()
                    .map_err(|error| format!("parse {variable}={value:?}: {error}").into());
            }
            Err(std::env::VarError::NotPresent) => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(false)
}

fn benchmark_positive_i32(variable: &str, default: i32) -> Result<i32, Box<dyn std::error::Error>> {
    match std::env::var(variable) {
        Ok(value) => {
            let parsed = value
                .parse::<i32>()
                .map_err(|error| format!("parse {variable}={value:?}: {error}"))?;
            if parsed <= 0 {
                return Err(format!("{variable} must be at least 1").into());
            }
            Ok(parsed)
        }
        Err(std::env::VarError::NotPresent) => Ok(default),
        Err(error) => Err(error.into()),
    }
}

async fn wait_for_worker_readiness(
    readiness_rx: tokio::sync::oneshot::Receiver<Result<(), String>>,
    application_name: &str,
    worker_thread: &JoinHandle<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let timeout = benchmark_startup_timeout()?;
    match tokio::time::timeout(timeout, readiness_rx).await {
        Ok(Ok(Ok(()))) => Ok(()),
        Ok(Ok(Err(reason))) => Err(format!(
            "main_block_baseline worker reported failed LISTEN/acquisition/key-cache readiness (application_name={application_name:?}): {reason}"
        )
        .into()),
        Ok(Err(_)) => {
            let state = if worker_thread.is_finished() {
                "worker thread exited"
            } else {
                "readiness hook was dropped before acknowledgement"
            };
            Err(format!(
                "main_block_baseline worker did not acknowledge LISTEN/acquisition/key-cache readiness (application_name={application_name:?}): {state}"
            )
            .into())
        }
        Err(_) => Err(format!(
            "timed out after {}s waiting for main_block_baseline worker in-process readiness (application_name={application_name:?}, worker_thread_finished={})",
            timeout.as_secs(),
            worker_thread.is_finished(),
        )
        .into()),
    }
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
    let worker_thread = start_coprocessor(rx, &db_url).await?;
    Ok(TestInstance {
        _container: Some(container),
        app_close_channel: Some(app_close_channel),
        worker_thread: Some(worker_thread),
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
        let current_count: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM computations WHERE is_allowed = TRUE AND is_completed = FALSE",
        )
        .fetch_one(&pool)
        .await?;
        if current_count == 0 {
            break;
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub struct LegacyTerminal {
    pub handle: Handle,
    pub transaction_id: Handle,
}

/// Wait only for the legacy rows created by this sample.  The historical
/// all-allowed predicate is unsafe for a reused reportable database: unrelated
/// completed rows can make it appear that a sample has finished.
pub async fn wait_until_legacy_terminals_computed(
    db_url: String,
    terminals: &[LegacyTerminal],
) -> Result<std::time::Instant, Box<dyn std::error::Error>> {
    let expected = terminals
        .iter()
        .map(|terminal| (terminal.handle.to_vec(), terminal.transaction_id.to_vec()))
        .collect::<Vec<_>>();
    wait_for_exact_legacy_terminals(
        &db_url,
        &expected,
        benchmark_wait_timeout()?,
        std::env::var("FHEVM_BENCH_RUN_MODE").as_deref() == Ok(SMOKE_ONLY_RUN_MODE),
    )
    .await
}

/// Materialize native legacy DCID state without inventing any host block.  The
/// worker consumes this table directly when DCID locking is enabled.
pub async fn upsert_legacy_dependence_chain(
    tx: &mut Transaction<'_>,
    dependence_chain_id: &Handle,
    dependency_count: i32,
    dependents: &[Handle],
) -> Result<(), sqlx::Error> {
    let dependents = dependents
        .iter()
        .map(|handle| handle.to_vec())
        .collect::<Vec<_>>();
    sqlx::query(
        "INSERT INTO dependence_chain ( \
             dependence_chain_id, status, last_updated_at, dependency_count, dependents, \
             block_hash, block_height, schedule_priority \
         ) VALUES ($1, 'updated', NOW(), $2, $3, ''::bytea, 0, 0) \
         ON CONFLICT (dependence_chain_id) DO UPDATE \
         SET status = 'updated', dependency_count = EXCLUDED.dependency_count, \
             dependents = EXCLUDED.dependents, worker_id = NULL, \
             lock_acquired_at = NULL, lock_expires_at = NULL",
    )
    .bind(dependence_chain_id.to_vec())
    .bind(dependency_count)
    .bind(dependents)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

const MAIN_BLOCK_BASELINE_LABEL: &str = "main_block_baseline";
const REPORTABLE_RUN_MODE: &str = "reportable";
const SMOKE_ONLY_RUN_MODE: &str = "smoke_only";

pub fn validate_main_block_run_policy() -> Result<&'static str, String> {
    let smoke_triggered = std::env::var("FHEVM_BENCH_DIRECT_SMOKE").as_deref() == Ok("1")
        || std::env::var_os("FHEVM_TEST_NUM_SAMPLES").is_some();
    let requested = std::env::var("FHEVM_BENCH_RUN_MODE").ok();
    let mode = match requested.as_deref() {
        None if smoke_triggered => SMOKE_ONLY_RUN_MODE,
        None => REPORTABLE_RUN_MODE,
        Some(SMOKE_ONLY_RUN_MODE) => SMOKE_ONLY_RUN_MODE,
        Some(REPORTABLE_RUN_MODE) if !smoke_triggered => REPORTABLE_RUN_MODE,
        Some(REPORTABLE_RUN_MODE) => {
            return Err("FHEVM_BENCH_RUN_MODE=reportable conflicts with a smoke trigger".into())
        }
        Some(other) => {
            return Err(format!(
                "invalid FHEVM_BENCH_RUN_MODE={other:?}; expected {REPORTABLE_RUN_MODE:?} or {SMOKE_ONLY_RUN_MODE:?}"
            ))
        }
    };
    effective_main_block_bench_lto()?;
    if mode == REPORTABLE_RUN_MODE {
        for variable in [
            "FHEVM_BENCH_SMOKE_WARMUP_SECS",
            "FHEVM_BENCH_SMOKE_MEASUREMENT_SECS",
            "FHEVM_BENCH_SMOKE_SAMPLE_SIZE",
            "FHEVM_BENCH_SMOKE_NRESAMPLES",
            "FHEVM_BENCH_SMOKE_MAX_REQUESTED_ITERS",
            "FHEVM_BENCH_CRITERION_PROBE",
        ] {
            if std::env::var_os(variable).is_some() {
                return Err(format!(
                    "reportable {MAIN_BLOCK_BASELINE_LABEL} forbids smoke-only override {variable}"
                ));
            }
        }
        let source = runtime_source_identity().map_err(|error| error.to_string())?;
        if source.state != "clean" {
            return Err(
                "reportable main_block_baseline requires a clean committed source tree".into(),
            );
        }
        verify_reportable_source_claim("FHEVM_BENCH_BUILD_REVISION", &source.revision)?;
        verify_reportable_source_claim("FHEVM_BENCH_SOURCE_STATE", &source.state)?;
        verify_reportable_source_claim("FHEVM_BENCH_SOURCE_FINGERPRINT", &source.fingerprint)?;
    }
    Ok(mode)
}

pub fn effective_main_block_bench_lto() -> Result<&'static str, String> {
    let cargo_profile_lto = std::env::var("CARGO_PROFILE_BENCH_LTO").map_err(|_| {
        "main_block_baseline requires CARGO_PROFILE_BENCH_LTO=false from its isolated Make target"
            .to_owned()
    })?;
    let recorded_lto = std::env::var("FHEVM_BENCH_EFFECTIVE_LTO").map_err(|_| {
        "main_block_baseline requires FHEVM_BENCH_EFFECTIVE_LTO=false for artifact provenance"
            .to_owned()
    })?;
    if cargo_profile_lto != "false" || recorded_lto != "false" {
        return Err(format!(
            "main_block_baseline requires disabled bench LTO, got CARGO_PROFILE_BENCH_LTO={cargo_profile_lto:?}, FHEVM_BENCH_EFFECTIVE_LTO={recorded_lto:?}"
        ));
    }
    Ok("false")
}

#[derive(Debug, Clone, serde::Serialize)]
struct RuntimeSourceIdentity {
    revision: String,
    state: String,
    fingerprint: String,
}

fn runtime_source_identity() -> Result<RuntimeSourceIdentity, Box<dyn std::error::Error>> {
    let revision = command_stdout("git", &["rev-parse", "HEAD"])?;
    let status = command_output(
        "git",
        &["status", "--porcelain", "--untracked-files=normal"],
    )?;
    if status.is_empty() {
        return Ok(RuntimeSourceIdentity {
            fingerprint: revision.clone(),
            revision,
            state: "clean".to_owned(),
        });
    }
    let diff = command_output("git", &["diff", "--no-ext-diff", "--binary", "HEAD"])?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    revision.hash(&mut hasher);
    status.hash(&mut hasher);
    diff.hash(&mut hasher);
    Ok(RuntimeSourceIdentity {
        revision,
        state: "dirty".to_owned(),
        fingerprint: format!("dirty-{:016x}", hasher.finish()),
    })
}

fn command_output(program: &str, args: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let output = Command::new(program).args(args).output()?;
    if !output.status.success() {
        return Err(format!("{program} {args:?} exited with {}", output.status).into());
    }
    Ok(output.stdout)
}

fn command_stdout(program: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from_utf8(command_output(program, args)?)?
        .trim()
        .to_owned())
}

fn verify_reportable_source_claim(variable: &str, observed: &str) -> Result<(), String> {
    let claimed = std::env::var(variable).map_err(|_| {
        format!("reportable {MAIN_BLOCK_BASELINE_LABEL} requires {variable}={observed:?}")
    })?;
    if claimed != observed {
        return Err(format!(
            "reportable {MAIN_BLOCK_BASELINE_LABEL} source provenance mismatch: {variable}={claimed:?}, observed {observed:?}"
        ));
    }
    Ok(())
}

pub fn compiled_benchmark_backend() -> &'static str {
    if cfg!(feature = "gpu") {
        "gpu"
    } else {
        "cpu"
    }
}

pub fn persist_main_block_provenance() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if validate_main_block_run_policy()? != REPORTABLE_RUN_MODE {
        return Err("canonical main_block_baseline provenance is reportable-only".into());
    }
    let source = runtime_source_identity()?;
    let bench_lto = effective_main_block_bench_lto()?;
    let executable_path = std::env::current_exe()?;
    let mut host = BTreeMap::from([
        ("architecture", std::env::consts::ARCH.to_owned()),
        ("operating_system", std::env::consts::OS.to_owned()),
        ("kernel_release", command_stdout("uname", &["-r"])?),
        ("hostname", command_stdout("hostname", &[])?),
    ]);
    if compiled_benchmark_backend() == "gpu" {
        host.insert(
            "gpu_runtime",
            command_stdout(
                "nvidia-smi",
                &[
                    "--query-gpu=index,uuid,name,driver_version",
                    "--format=csv,noheader",
                ],
            )?,
        );
    } else {
        host.insert("cpu_model", cpu_model_name()?);
    }
    let manifest = serde_json::json!({
        "schema_version": 1,
        "baseline": MAIN_BLOCK_BASELINE_LABEL,
        "worker_semantics": "main_native_legacy_computations_ciphertexts_dependence_chain",
        "topology": "legacy_tx/dependence_chain_no_host_block_provenance",
        "run": { "mode": REPORTABLE_RUN_MODE },
        "build": {
            "revision": source.revision,
            "source_state": source.state,
            "source_fingerprint": source.fingerprint,
            "package_version": env!("CARGO_PKG_VERSION"),
            "backend": compiled_benchmark_backend(),
            "features": compiled_benchmark_features(),
            "bench_lto": bench_lto,
        },
        "executable": { "path": executable_path, "sha256": sha256_hex(&executable_path)? },
        "host": host,
    });
    let serialized = serde_json::to_string(&manifest)?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    serialized.hash(&mut hasher);
    let id = format!("{:016x}", hasher.finish());
    let path = criterion_output_directory()
        .join("benchmark-manifests")
        .join(format!("{MAIN_BLOCK_BASELINE_LABEL}-{id}.json"));
    std::fs::create_dir_all(path.parent().expect("manifest has a parent"))?;
    std::fs::write(
        &path,
        format!("{}\n", serde_json::to_string_pretty(&manifest)?),
    )?;
    Ok(path)
}

/// Persist the one and only reportable result for the canonical main legacy
/// baseline. The benchmark body supplies its operation/topology evidence;
/// this helper owns the comparable source, build, and runtime provenance.
pub fn persist_main_block_one_shot_artifact(
    result: serde_json::Value,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if validate_main_block_run_policy()? != REPORTABLE_RUN_MODE {
        return Err("main_block_baseline one-shot artifact is reportable-only".into());
    }
    let source = runtime_source_identity()?;
    let bench_lto = effective_main_block_bench_lto()?;
    let mut host = BTreeMap::from([
        ("architecture", std::env::consts::ARCH.to_owned()),
        ("operating_system", std::env::consts::OS.to_owned()),
        ("kernel_release", command_stdout("uname", &["-r"])?),
        ("hostname", command_stdout("hostname", &[])?),
    ]);
    if compiled_benchmark_backend() == "gpu" {
        host.insert(
            "gpu_runtime",
            command_stdout(
                "nvidia-smi",
                &[
                    "--query-gpu=index,uuid,name,driver_version",
                    "--format=csv,noheader",
                ],
            )?,
        );
    } else {
        host.insert("cpu_model", cpu_model_name()?);
    }
    let contents = serde_json::json!({
        "schema_version": 2,
        "run_mode": REPORTABLE_RUN_MODE,
        "baseline": MAIN_BLOCK_BASELINE_LABEL,
        "build": {
            "revision": source.revision,
            "source_state": source.state,
            "source_fingerprint": source.fingerprint,
            "package_version": env!("CARGO_PKG_VERSION"),
            "backend": compiled_benchmark_backend(),
            "features": compiled_benchmark_features(),
            "bench_lto": bench_lto,
        },
        "runtime_facts": {
            "xof_keyset_sha256": sha256_hex(std::path::Path::new("../fhevm-keys/xof-keyset"))?,
            "host": host,
        },
        "result": result,
    });
    let serialized = serde_json::to_string(&contents)?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    serialized.hash(&mut hasher);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();
    let path = criterion_output_directory()
        .join("benchmark-runs")
        .join(format!(
            "reportable-main-one-shot-{:016x}-{timestamp}.json",
            hasher.finish(),
        ));
    std::fs::create_dir_all(path.parent().expect("artifact has a parent"))?;
    std::fs::write(
        &path,
        format!("{}\n", serde_json::to_string_pretty(&contents)?),
    )?;
    Ok(path)
}

pub fn persist_main_block_smoke_artifact(
    smoke_kind: &str,
    details: serde_json::Value,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if validate_main_block_run_policy()? != SMOKE_ONLY_RUN_MODE {
        return Err("main_block_baseline smoke artifact requested for a reportable run".into());
    }
    let source = runtime_source_identity()?;
    let bench_lto = effective_main_block_bench_lto()?;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();
    let path = criterion_output_directory()
        .join("benchmark-smokes")
        .join(format!(
            "{MAIN_BLOCK_BASELINE_LABEL}-{smoke_kind}-{timestamp}-{:016x}.json",
            random_handle()
        ));
    std::fs::create_dir_all(path.parent().expect("smoke artifact has a parent"))?;
    std::fs::write(
        &path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema_version": 1,
                "baseline": MAIN_BLOCK_BASELINE_LABEL,
                "run_mode": SMOKE_ONLY_RUN_MODE,
                "smoke_kind": smoke_kind,
                "topology": "legacy_tx/dependence_chain_no_host_block_provenance",
                "source": source,
                "backend": compiled_benchmark_backend(),
                "bench_lto": bench_lto,
                "details": details,
            }))?
        ),
    )?;
    Ok(path)
}

fn criterion_output_directory() -> PathBuf {
    std::env::var_os("CRITERION_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("CARGO_TARGET_DIR").map(|path| PathBuf::from(path).join("criterion"))
        })
        .unwrap_or_else(|| PathBuf::from("target/criterion"))
}

fn compiled_benchmark_features() -> Vec<&'static str> {
    let mut features = vec!["bench"];
    if cfg!(feature = "gpu") {
        features.push("gpu");
    }
    if cfg!(feature = "latency") {
        features.push("latency");
    }
    if cfg!(feature = "throughput") {
        features.push("throughput");
    }
    features
}

fn cpu_model_name() -> Result<String, Box<dyn std::error::Error>> {
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo")?;
    cpuinfo
        .lines()
        .find_map(|line| line.strip_prefix("model name\t: ").map(str::to_owned))
        .ok_or_else(|| "CPU model name is missing from /proc/cpuinfo".into())
}

fn sha256_hex(path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let path = path.to_string_lossy();
    command_stdout("sha256sum", &[path.as_ref()]).map(|line| {
        line.split_whitespace()
            .next()
            .unwrap_or_default()
            .to_owned()
    })
}

fn benchmark_startup_timeout() -> Result<tokio::time::Duration, Box<dyn std::error::Error>> {
    benchmark_timeout(
        "FHEVM_BENCH_STARTUP_TIMEOUT_SECS",
        if std::env::var_os("FHEVM_BENCH_DIRECT_SMOKE").is_some() {
            60
        } else {
            300
        },
    )
}

pub fn benchmark_wait_timeout() -> Result<tokio::time::Duration, Box<dyn std::error::Error>> {
    benchmark_timeout(
        "FHEVM_BENCH_WAIT_TIMEOUT_SECS",
        if std::env::var_os("FHEVM_BENCH_DIRECT_SMOKE").is_some() {
            120
        } else {
            7200
        },
    )
}

fn benchmark_shutdown_timeout() -> Result<tokio::time::Duration, Box<dyn std::error::Error>> {
    benchmark_timeout("FHEVM_BENCH_SHUTDOWN_TIMEOUT_SECS", 30)
}

fn benchmark_timeout(
    variable: &str,
    default_seconds: u64,
) -> Result<tokio::time::Duration, Box<dyn std::error::Error>> {
    let seconds = std::env::var(variable)
        .ok()
        .map_or(Ok(default_seconds), |value| {
            value
                .parse::<u64>()
                .map_err(|_| format!("{variable} must be a positive integer, got {value:?}"))
        })?;
    if seconds == 0 {
        return Err(format!("{variable} must be greater than zero").into());
    }
    Ok(tokio::time::Duration::from_secs(seconds))
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
    insert_tfhe_event_with_dependence_chain(db, tx, log, tx_hash, tx_hash, is_allowed).await
}

/// Benchmark staging normally uses one transaction per native dependence
/// chain. The auction fixture instead preserves its 300 EVM transaction IDs
/// while intentionally executing their same-L1-block graph as one legacy
/// scheduling chain.
pub async fn insert_tfhe_event_with_dependence_chain(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    log: alloy::rpc::types::Log<TfheContractEvents>,
    transaction_hash: Handle,
    dependence_chain: Handle,
    is_allowed: bool,
) -> Result<bool, sqlx::Error> {
    // Bench staging bypasses ordered block ingestion, so derive the same
    // authoritative transaction-local origin bits from rows already staged
    // for this fixture transaction.
    let previously_minted = sqlx::query_scalar::<_, Vec<u8>>(
        "SELECT output_handle FROM computations WHERE transaction_id = $1",
    )
    .bind(transaction_hash.to_vec())
    .fetch_all(&mut **tx)
    .await?
    .into_iter()
    .collect::<std::collections::HashSet<_>>();
    let operand_boundary_mask = operand_boundary_mask_from_minted(&log.inner.data, |handle| {
        previously_minted.contains(handle.as_slice())
    })
    .map_err(sqlx::Error::Protocol)?;
    let event = LogTfhe {
        event: log.inner,
        transaction_hash: Some(transaction_hash),
        is_allowed,
        block_number: log.block_number.unwrap_or(0),
        block_hash: log.block_hash.unwrap_or_default(),
        block_timestamp: PrimitiveDateTime::MAX,
        dependence_chain,
        tx_depth_size: 0,
        log_index: log.log_index,
        operand_boundary_mask: Some(operand_boundary_mask),
        is_executor_minted: true,
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
        0,
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

pub async fn write_atomic_u64_bench_params(
    pool: &PgPool,
    bench_id: &str,
    display_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_key_cache = fhevm_engine_common::db_keys::DbKeyCache::new(100)?;
    let key = db_key_cache.fetch_latest_from_pool(pool).await?;
    let params = key
        .cks
        .ok_or_else(|| std::io::Error::other("latest key is missing cks"))?
        .computation_parameters();

    write_to_json::<u64, _>(
        bench_id,
        params,
        "",
        display_name,
        &OperatorType::Atomic,
        64,
        vec![],
    );
    Ok(())
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
