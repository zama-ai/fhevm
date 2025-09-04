use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use host_listener::database::tfhe_event_propagate::{Database as ListenerDatabase, Handle};

use sqlx::Postgres;
use std::{collections::HashMap, fmt, sync::atomic::AtomicU64};
use std::{
    ops::{Add, Sub},
    sync::Arc,
};
use std::{
    sync::atomic::Ordering,
    time::{Duration, SystemTime},
};
use stress_test_generator::utils::{
    default_dependence_cache_size, Dependence, GeneratorKind, Transaction,
};
use stress_test_generator::zk_gen::{generate_input_verification_transaction, get_inputs_vector};
use stress_test_generator::{args::parse_args, dex::dex_swap_claim_transaction};
use stress_test_generator::{
    args::Args,
    synthetics::{
        add_chain_transaction, generate_pub_decrypt_handles_types,
        generate_user_decrypt_handles_types, mul_chain_transaction,
    },
};
use stress_test_generator::{
    dex::dex_swap_request_transaction,
    erc20::erc20_transaction,
    utils::{EnvConfig, Job, Scenario},
};
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let args = parse_args();
    tracing_subscriber::fmt::init();

    if args.run_server {
        info!(target: "tool", args = ?args, "Initializing API server");
        match run_service(&args).await {
            Ok(_) => info!(target: "tool", "API server stopped"),
            Err(e) => error!("Error running API server: {}", e),
        }
    } else {
        info!(target: "tool", "Parsing and executing scenarios");
        parse_and_execute().await.unwrap();
        info!(target: "tool", "Done");
    }
}

pub static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
struct AppState {
    sender: mpsc::Sender<Job>,
    jobs_status: Arc<RwLock<HashMap<u64, JobStatus>>>,
}

impl AppState {
    fn new(sender: mpsc::Sender<Job>) -> Self {
        Self {
            sender,
            jobs_status: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn set_status(&self, job_id: u64, status: JobStatus) {
        let mut jobs_status = self.jobs_status.write().await;
        jobs_status.insert(job_id, status);
    }
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
enum JobStatus {
    Queued(DateTime<Utc>),
    Running(DateTime<Utc>),
    Completed(DateTime<Utc>),
}

impl fmt::Debug for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Queued(time) => write!(f, "Queued at {:?}", time),
            JobStatus::Running(time) => write!(f, "Running since {:?}", time),
            JobStatus::Completed(time) => write!(f, "Completed at {:?}", time),
        }
    }
}

async fn run_service(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let (sender, mut rx) = mpsc::channel::<Job>(100);
    let state = AppState::new(sender);
    let s = state.clone();
    tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            info!(target: "tool", job_id = job.id, "Processing job");
            let started_at = SystemTime::now();
            s.set_status(job.id, JobStatus::Running(Utc::now())).await;

            if let Err(e) = spawn_and_wait_all(job.scenarios).await {
                error!(target: "tool", job_id = job.id, "Error processing job: {}", e);
            }

            s.set_status(job.id, JobStatus::Completed(Utc::now())).await;

            info!(target: "tool", job_id = job.id, duration = ?started_at.elapsed(), "Job completed");
        }
    });

    let app = Router::new()
        .route("/job", post(enqueue_job))
        .route("/job/:id", get(get_job))
        .route("/status/running", get(get_running_job))
        .route("/status/queued", get(get_queued_job))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(args.listen_address.as_str())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_job(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<u64>,
) -> (StatusCode, Json<Option<JobStatus>>) {
    let status = state.jobs_status.read().await.get(&job_id).cloned();
    info!(target: "tool", status = ?status, "Job status");

    (StatusCode::OK, Json(status))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct EnqueuedJob {
    id: u64,
    scenarios_count: usize,
    queued_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RunningJob {
    id: u64,
    status: JobStatus,
}

async fn enqueue_job(
    State(state): State<Arc<AppState>>,
    Json(scenarios): Json<Vec<Scenario>>,
) -> (StatusCode, Json<EnqueuedJob>) {
    let job_id = GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let len = scenarios.len();

    state
        .sender
        .send(Job {
            id: job_id,
            scenarios,
        })
        .await
        .unwrap();

    info!(target: "tool", job_id, "Enqueued job");

    let now = Utc::now();
    state.set_status(job_id, JobStatus::Queued(now)).await;

    (
        StatusCode::CREATED,
        Json(EnqueuedJob {
            id: job_id,
            scenarios_count: len,
            queued_at: now,
        }),
    )
}

async fn get_running_job(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Option<RunningJob>>) {
    let running: Vec<(u64, JobStatus)> = state
        .jobs_status
        .read()
        .await
        .iter()
        .filter(|(_, v)| matches!(v, JobStatus::Running(_)))
        .map(|(k, v)| (*k, *v))
        .collect();

    if running.is_empty() {
        return (StatusCode::OK, Json(None));
    }

    info!(target: "tool", running_jobs = ?running, "Currently running job");

    (
        StatusCode::OK,
        Json(Some(RunningJob {
            id: running[0].0,
            status: running[0].1,
        })),
    )
}

async fn get_queued_job(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Option<Vec<(u64, JobStatus)>>>) {
    let queued: Vec<(u64, JobStatus)> = state
        .jobs_status
        .read()
        .await
        .iter()
        .filter(|(_, v)| matches!(v, JobStatus::Queued(_)))
        .map(|(k, v)| (*k, *v))
        .collect();

    if queued.is_empty() {
        return (StatusCode::OK, Json(None));
    }

    (StatusCode::OK, Json(Some(queued)))
}

/// Parse the input CSV file and create and spawn transaction scenarios
async fn parse_and_execute() -> Result<(), Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .trim(csv::Trim::All)
        .has_headers(false)
        .flexible(true)
        .from_path(ecfg.evgen_scenario)
        .unwrap();
    let iter = rdr.deserialize::<Scenario>();
    let generators: Vec<Scenario> = iter
        .map(|res| res.as_ref().expect("Incorrect scenario file").clone())
        .collect();

    spawn_and_wait_all(generators).await?;

    Ok(())
}

async fn spawn_and_wait_all(scenarios: Vec<Scenario>) -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];
    for scenario in scenarios {
        let handle = tokio::spawn(async move {
            info!(target: "tool", scenario = ?scenario, "Execute scenario");
            match scenario.kind {
                GeneratorKind::Count => {
                    if let Err(err) = generate_transactions_count(&scenario).await {
                        error!(scenario = ?scenario, err, "Generating transactions with count failed");
                    }
                }
                GeneratorKind::Rate => {
                    if let Err(err) = generate_transactions_at_rate(&scenario).await {
                        error!(scenario = ?scenario, err, "Generating transactions at rate failed");
                    }
                }
            }
        });
        handles.push(handle);
    }
    futures::future::join_all(handles).await;
    Ok(())
}

async fn generate_transactions_at_rate(
    scenario: &Scenario,
) -> Result<(), Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let coprocessor_api_key = sqlx::types::Uuid::parse_str(&ecfg.api_key).unwrap();
    let mut listener_event_to_db = ListenerDatabase::new(
        &ecfg.evgen_db_url,
        &coprocessor_api_key,
        default_dependence_cache_size(),
    )
    .await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&ecfg.evgen_db_url)
        .await
        .unwrap();
    let mut dependence_handle1: Option<Handle> = None;
    let mut dependence_handle2: Option<Handle> = None;
    for (target_throughput, duration_seconds) in scenario.scenario.iter() {
        // If target throughput is not meaningful, sleep for the interval
        if *target_throughput <= 0.0 {
            tokio::time::sleep(std::time::Duration::from_secs(*duration_seconds)).await;
            continue;
        }
        let start_time = SystemTime::now();
        let mut last_transaction_time = SystemTime::now();
        let end_target = start_time.add(std::time::Duration::from_secs(*duration_seconds));
        let time_between_transactions = std::time::Duration::from_secs_f64(1.0 / target_throughput);
        loop {
            let transaction_start = SystemTime::now();
            if transaction_start > end_target {
                break;
            }
            info!(target: "tool" , "Generating new transaction at rate");

            let (dep1, dep2) = generate_transaction(
                scenario,
                dependence_handle1,
                dependence_handle2,
                &mut listener_event_to_db,
                &pool,
            )
            .await?;
            if scenario.is_dependent == Dependence::Dependent {
                dependence_handle1 = Some(dep1);
                dependence_handle2 = Some(dep2);
            }
            let elapsed = SystemTime::now()
                .duration_since(last_transaction_time)
                .unwrap_or(Duration::new(0, 10));
            // Either we can keep up with target throughput and we
            // sleep the balance of time or we just do best effort and
            // continuously generate events (we may fall below the
            // target rate if it's set too high).
            if time_between_transactions > elapsed {
                tokio::time::sleep(time_between_transactions.sub(elapsed)).await;
                last_transaction_time = SystemTime::now();
            }
        }
    }
    Ok(())
}

async fn generate_transactions_count(
    scenario: &Scenario,
) -> Result<(), Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let coprocessor_api_key = sqlx::types::Uuid::parse_str(&ecfg.api_key).unwrap();
    let mut listener_event_to_db = ListenerDatabase::new(
        &ecfg.evgen_db_url,
        &coprocessor_api_key,
        default_dependence_cache_size(),
    )
    .await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&ecfg.evgen_db_url)
        .await
        .unwrap();

    let mut dependence_handle1: Option<Handle> = None;
    let mut dependence_handle2: Option<Handle> = None;
    for (num_transactions, iter_count) in scenario.scenario.iter() {
        let iters = (*num_transactions * *iter_count as f64) as u64;
        for iter in 0..iters {
            info!(target: "tool", iter , "Generating new transaction");

            let (dep1, dep2) = generate_transaction(
                scenario,
                dependence_handle1,
                dependence_handle2,
                &mut listener_event_to_db,
                &pool,
            )
            .await?;
            if scenario.is_dependent == Dependence::Dependent {
                dependence_handle1 = Some(dep1);
                dependence_handle2 = Some(dep2);
            }
        }
    }
    Ok(())
}

async fn generate_transaction(
    scenario: &Scenario,
    dependence1: Option<Handle>,
    dependence2: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let inputs = get_inputs_vector(
        scenario.inputs.to_owned(),
        &scenario.contract_address,
        &scenario.user_address,
    )
    .await?;
    match scenario.transaction {
        Transaction::ERC20Transfer => {
            let (_, output_dependence) = erc20_transaction(
                inputs[0],
                dependence1,
                inputs[1],
                None, // Transaction ID
                listener_event_to_db,
                pool,
                scenario.variant.to_owned(),
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((output_dependence, output_dependence))
        }
        Transaction::DEXSwapRequest => {
            let (new_current_balance_0, new_current_balance_1) = dex_swap_request_transaction(
                inputs[0],
                inputs[1],
                dependence1,
                dependence2,
                inputs[2],
                inputs[3],
                inputs[4],
                inputs[5],
                inputs[6],
                inputs[7],
                listener_event_to_db,
                pool,
                scenario.variant.to_owned(),
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((new_current_balance_0, new_current_balance_1))
        }
        Transaction::DEXSwapClaim => {
            let (new_current_balance_0, new_current_balance_1) = dex_swap_claim_transaction(
                inputs[0],
                inputs[1],
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
                inputs[2],
                inputs[3],
                dependence1,
                dependence2,
                listener_event_to_db,
                pool,
                scenario.variant.to_owned(),
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((new_current_balance_0, new_current_balance_1))
        }
        Transaction::ADDChain => {
            let (output_dependence1, output_dependence2) = add_chain_transaction(
                dependence1,
                inputs[1],
                ecfg.synthetic_chain_length,
                None, // Transaction ID
                listener_event_to_db,
                pool,
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((output_dependence1, output_dependence2))
        }
        Transaction::MULChain => {
            let (output_dependence1, output_dependence2) = mul_chain_transaction(
                dependence1,
                inputs[1],
                ecfg.synthetic_chain_length,
                None, // Transaction ID
                listener_event_to_db,
                pool,
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((output_dependence1, output_dependence2))
        }
        Transaction::InputVerif => {
            let (output_dependence1, output_dependence2) = generate_input_verification_transaction(
                ecfg.synthetic_chain_length,
                16u8,
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((output_dependence1, output_dependence2))
        }
        Transaction::GenPubDecHandles => {
            let (output_dependence1, output_dependence2) = generate_pub_decrypt_handles_types(
                ecfg.min_decryption_type,
                ecfg.max_decryption_type,
                None, // Transaction ID
                listener_event_to_db,
                pool,
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((output_dependence1, output_dependence2))
        }
        Transaction::GenUsrDecHandles => {
            let (output_dependence1, output_dependence2) = generate_user_decrypt_handles_types(
                ecfg.min_decryption_type,
                ecfg.max_decryption_type,
                None, // Transaction ID
                listener_event_to_db,
                pool,
                &scenario.contract_address,
                &scenario.user_address,
            )
            .await?;
            Ok((output_dependence1, output_dependence2))
        }
    }
}
