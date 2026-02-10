use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::utils::{safe_deserialize, safe_deserialize_key, DatabaseURL};
use reqwest::Client;
use serde_json::json;
use serial_test::serial;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_listener::database::ingest::map_envelope_to_actions;
use solana_listener::database::solana_event_propagate::Database;
use solana_listener::poller::solana_rpc_source::SolanaRpcEventSource;
use solana_listener::poller::{Cursor, EventSource};
use solana_sdk::hash::hash;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::transaction::Transaction;
use sqlx::postgres::PgPoolOptions;
use testcontainers::{
    core::{IntoContainerPort, Mount, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use tfhe_worker::daemon_cli::Args as WorkerArgs;
use tfhe_worker::server::tfhe_worker::{
    fhevm_coprocessor_client::FhevmCoprocessorClient, TrivialEncryptBatch,
    TrivialEncryptRequestSingle,
};
use tokio::time::{sleep, Instant};
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tracing::info;

const POSTGRES_IMAGE: &str = "postgres";
const POSTGRES_TAG: &str = "15.7";
const SOLANA_RPC_PORT: u16 = 8899;
const SOLANA_PROGRAM_ID_STR: &str = "Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq";
const HOST_CHAIN_ID: i64 = 4242;
const TENANT_ID: i32 = 1;
const EQUIVALENCE_TENANT_ID: i32 = 2;
const OP_ADD: u8 = 0;
const OP_SUB: u8 = 1;

#[derive(Default, Debug)]
struct IngestionSummary {
    processed_events: usize,
    inserted_computations: usize,
    inserted_allowed_handles: usize,
    inserted_pbs_computations: usize,
}

#[derive(Debug)]
struct WorkerQueueSnapshot {
    runnable_rows: i64,
    runnable_transactions: i64,
    null_transaction_ids: i64,
}

struct PostgresHarness {
    _container: ContainerAsync<GenericImage>,
    db_url: String,
}

struct SolanaHarness {
    _container: ContainerAsync<GenericImage>,
    rpc_url: String,
}

struct ProgramMount {
    program_id: String,
    artifact_dir: PathBuf,
    program_file_name: String,
}

const DEFAULT_API_KEY: &str = "a1503fb6-d79b-4e9e-826d-44cf262f3e05";

struct WorkerHarness {
    close_tx: tokio::sync::watch::Sender<bool>,
    endpoint: String,
}

struct LocalnetClientHarness {
    solana: SolanaHarness,
    rpc_client: RpcClient,
    payer: Keypair,
    program_id: Pubkey,
}

struct ComputeHarness {
    postgres: PostgresHarness,
    pool: sqlx::PgPool,
    tenant_id: i32,
    _worker: WorkerHarness,
    worker_client: FhevmCoprocessorClient<Channel>,
    localnet: LocalnetClientHarness,
}

impl Drop for WorkerHarness {
    fn drop(&mut self) {
        let _ = self.close_tx.send_replace(true);
    }
}

fn pick_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("bind ephemeral port")
        .local_addr()
        .expect("resolve local address")
        .port()
}

async fn start_postgres_with_migrations() -> anyhow::Result<PostgresHarness> {
    let container = GenericImage::new(POSTGRES_IMAGE, POSTGRES_TAG)
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .context("start postgres container")?;

    let host = container.get_host().await.context("postgres host")?;
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .context("postgres mapped port")?;
    let admin_db_url = format!("postgresql://postgres:postgres@{host}:{port}/postgres");
    let db_url = format!("postgresql://postgres:postgres@{host}:{port}/coprocessor");

    let admin_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_db_url)
        .await
        .context("connect postgres admin DB")?;

    sqlx::query("DROP DATABASE IF EXISTS coprocessor;")
        .execute(&admin_pool)
        .await
        .context("drop coprocessor DB")?;
    sqlx::query("CREATE DATABASE coprocessor;")
        .execute(&admin_pool)
        .await
        .context("create coprocessor DB")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .context("connect coprocessor DB")?;

    sqlx::migrate!("../db-migration/migrations")
        .run(&pool)
        .await
        .context("run migrations")?;

    let has_computations: Option<String> =
        sqlx::query_scalar("SELECT to_regclass('public.computations')::text")
            .fetch_one(&pool)
            .await
            .context("check computations table exists")?;
    anyhow::ensure!(
        has_computations.is_some(),
        "migrations did not create computations table"
    );

    Ok(PostgresHarness {
        _container: container,
        db_url,
    })
}

async fn start_solana_validator_container(
    program_mount: Option<ProgramMount>,
) -> anyhow::Result<SolanaHarness> {
    let image =
        std::env::var("SOLANA_VALIDATOR_IMAGE").unwrap_or_else(|_| "solanalabs/solana".to_string());
    let tag = std::env::var("SOLANA_VALIDATOR_TAG").unwrap_or_else(|_| "v1.18.26".to_string());
    let host_rpc_port = pick_free_port();
    let mut cmd = vec![
        "--reset".to_string(),
        "--bind-address".to_string(),
        "0.0.0.0".to_string(),
        "--ledger".to_string(),
        "/tmp/test-ledger".to_string(),
    ];

    let mut request = GenericImage::new(image, tag)
        .with_exposed_port(SOLANA_RPC_PORT.tcp())
        .with_wait_for(WaitFor::seconds(2))
        .with_entrypoint("solana-test-validator")
        .with_mapped_port(host_rpc_port, SOLANA_RPC_PORT.tcp());

    if let Some(program_mount) = program_mount {
        request = request.with_mount(Mount::bind_mount(
            program_mount.artifact_dir.display().to_string(),
            "/program-artifacts",
        ));
        cmd.push("--bpf-program".to_string());
        cmd.push(program_mount.program_id);
        cmd.push(format!(
            "/program-artifacts/{}",
            program_mount.program_file_name
        ));
    }

    let container = request
        .with_cmd(cmd)
        .start()
        .await
        .context("start solana-test-validator container")?;

    let rpc_url = format!("http://127.0.0.1:{host_rpc_port}");
    wait_for_finalized_slot(&rpc_url, &container, Duration::from_secs(90)).await?;

    Ok(SolanaHarness {
        _container: container,
        rpc_url,
    })
}

async fn wait_for_finalized_slot(
    rpc_url: &str,
    container: &ContainerAsync<GenericImage>,
    timeout: Duration,
) -> anyhow::Result<()> {
    let client = Client::new();
    let deadline = Instant::now() + timeout;
    loop {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSlot",
            "params": [{"commitment": "finalized"}]
        });

        match client.post(rpc_url).json(&payload).send().await {
            Ok(resp) if resp.status().is_success() => {
                let body: serde_json::Value = resp.json().await.context("decode rpc response")?;
                if body.get("result").and_then(|v| v.as_u64()).is_some() {
                    return Ok(());
                }
            }
            Ok(_) | Err(_) => {}
        }

        if Instant::now() >= deadline {
            let stdout = container
                .stdout_to_vec()
                .await
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok())
                .unwrap_or_else(|| "<unavailable>".to_string());
            let stderr = container
                .stderr_to_vec()
                .await
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok())
                .unwrap_or_else(|| "<unavailable>".to_string());
            anyhow::bail!(
                "timed out waiting for finalized slot on {rpc_url}\nstdout:\n{}\nstderr:\n{}",
                stdout,
                stderr
            );
        }
        sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::test]
#[serial]
#[ignore = "requires Docker daemon and Solana validator image"]
async fn localnet_harness_starts_postgres_and_solana_validator() -> anyhow::Result<()> {
    let pg = start_postgres_with_migrations().await?;
    let solana = start_solana_validator_container(None).await?;

    anyhow::ensure!(
        pg.db_url.contains("coprocessor"),
        "unexpected postgres URL: {}",
        pg.db_url
    );
    anyhow::ensure!(
        solana.rpc_url.starts_with("http://127.0.0.1:"),
        "unexpected rpc URL: {}",
        solana.rpc_url
    );

    Ok(())
}

fn repo_root() -> anyhow::Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .join("../../..")
        .canonicalize()
        .context("resolve repository root")
}

async fn seed_tenant_with_keys(pool: &sqlx::PgPool) -> anyhow::Result<i32> {
    let root = repo_root()?;
    let key_dir = root.join("coprocessor/fhevm-engine/fhevm-keys");

    let sks = tokio::fs::read(key_dir.join("sks"))
        .await
        .context("read sks key")?;
    let pks = tokio::fs::read(key_dir.join("pks"))
        .await
        .context("read pks key")?;
    let cks = tokio::fs::read(key_dir.join("cks"))
        .await
        .context("read cks key")?;
    let public_params = tokio::fs::read(key_dir.join("pp"))
        .await
        .context("read public params")?;

    let tenant_id = sqlx::query_scalar::<_, i32>(
        "
        INSERT INTO tenants(
            tenant_api_key,
            chain_id,
            acl_contract_address,
            verifying_contract_address,
            pks_key,
            sks_key,
            public_params,
            cks_key
        )
        VALUES (
            $1::uuid,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8
        )
        RETURNING tenant_id
        ",
    )
    .bind(DEFAULT_API_KEY)
    .bind(12345_i64)
    .bind("0x339EcE85B9E11a3A3AA557582784a15d7F82AAf2")
    .bind("0x69dE3158643e738a0724418b21a35FAA20CBb1c5")
    .bind(pks)
    .bind(sks)
    .bind(public_params)
    .bind(cks)
    .fetch_one(pool)
    .await
    .context("insert test tenant with keys")?;

    Ok(tenant_id)
}

fn spawn_worker(db_url: &str) -> anyhow::Result<WorkerHarness> {
    let endpoint_port = pick_free_port();
    let health_port = pick_free_port();
    let endpoint = format!("http://127.0.0.1:{endpoint_port}");
    let server_addr = format!("127.0.0.1:{endpoint_port}");

    let key_path = repo_root()?
        .join("coprocessor/fhevm-engine/tfhe-worker/coprocessor.key")
        .display()
        .to_string();
    let db_url: DatabaseURL = db_url.into();

    let args = WorkerArgs {
        run_server: true,
        run_bg_worker: true,
        worker_polling_interval_ms: 200,
        generate_fhe_keys: false,
        server_maximum_ciphertexts_to_schedule: 5000,
        server_maximum_ciphertexts_to_get: 5000,
        work_items_batch_size: 40,
        dependence_chains_per_batch: 10,
        tenant_key_cache_size: 4,
        maximum_compact_inputs_upload: 10,
        maximum_handles_per_input: 255,
        coprocessor_fhe_threads: 4,
        tokio_threads: 2,
        pg_pool_max_connections: 4,
        server_addr,
        metrics_addr: None,
        database_url: Some(db_url),
        coprocessor_private_key: key_path,
        service_name: "solana-e2e-poc".to_string(),
        worker_id: None,
        dcid_ttl_sec: 30,
        disable_dcid_locking: true,
        dcid_timeslice_sec: 90,
        processed_dcid_ttl_sec: 48 * 60 * 60,
        dcid_cleanup_interval_sec: 3600,
        dcid_max_no_progress_cycles: 2,
        dcid_ignore_dependency_count_threshold: 100,
        log_level: tracing::Level::INFO,
        health_check_port: health_port,
        metric_rerand_batch_latency: Default::default(),
        metric_fhe_batch_latency: Default::default(),
    };

    let (close_tx, close_rx) = tokio::sync::watch::channel(false);
    std::thread::spawn(move || {
        tfhe_worker::start_runtime(args, Some(close_rx));
    });

    Ok(WorkerHarness { close_tx, endpoint })
}

async fn connect_worker(endpoint: &str) -> anyhow::Result<FhevmCoprocessorClient<Channel>> {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        match FhevmCoprocessorClient::connect(endpoint.to_string()).await {
            Ok(client) => return Ok(client),
            Err(err) => {
                if Instant::now() >= deadline {
                    return Err(err).context("connect worker gRPC");
                }
                sleep(Duration::from_millis(250)).await;
            }
        }
    }
}

fn airdrop_payer(rpc_client: &RpcClient, payer: &Keypair) -> anyhow::Result<()> {
    let airdrop_sig = rpc_client
        .request_airdrop(&payer.pubkey(), 2_000_000_000)
        .context("request airdrop")?;
    rpc_client
        .poll_for_signature_with_commitment(&airdrop_sig, CommitmentConfig::confirmed())
        .context("airdrop confirmation polling failed")?;
    Ok(())
}

fn finalized_cursor(rpc_client: &RpcClient) -> anyhow::Result<Cursor> {
    let slot = rpc_client
        .get_slot_with_commitment(CommitmentConfig::finalized())
        .context("get start finalized slot")?;
    Ok(Cursor {
        slot,
        tx_index: 0,
        op_index: 0,
    })
}

async fn start_localnet_client_with_program() -> anyhow::Result<LocalnetClientHarness> {
    let program_mount = build_anchor_program()?;
    let solana = start_solana_validator_container(Some(program_mount)).await?;
    let rpc_client =
        RpcClient::new_with_commitment(solana.rpc_url.clone(), CommitmentConfig::confirmed());
    let payer = Keypair::new();
    airdrop_payer(&rpc_client, &payer)?;
    let program_id = Pubkey::from_str(SOLANA_PROGRAM_ID_STR).context("parse program id")?;
    Ok(LocalnetClientHarness {
        solana,
        rpc_client,
        payer,
        program_id,
    })
}

async fn setup_compute_harness() -> anyhow::Result<ComputeHarness> {
    let postgres = start_postgres_with_migrations().await?;
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&postgres.db_url)
        .await
        .context("connect postgres pool for e2e")?;
    let tenant_id = seed_tenant_with_keys(&pool).await?;
    let worker = spawn_worker(&postgres.db_url)?;
    let worker_client = connect_worker(&worker.endpoint).await?;
    let localnet = start_localnet_client_with_program().await?;

    Ok(ComputeHarness {
        postgres,
        pool,
        tenant_id,
        _worker: worker,
        worker_client,
        localnet,
    })
}

async fn seed_trivial_inputs(
    worker_client: &mut FhevmCoprocessorClient<Channel>,
    lhs: [u8; 32],
    rhs: [u8; 32],
) -> anyhow::Result<()> {
    seed_trivial_inputs_with_values(worker_client, lhs, rhs, 123, 124).await
}

async fn seed_trivial_inputs_with_values(
    worker_client: &mut FhevmCoprocessorClient<Channel>,
    lhs: [u8; 32],
    rhs: [u8; 32],
    lhs_value: u8,
    rhs_value: u8,
) -> anyhow::Result<()> {
    let mut encrypt_request = tonic::Request::new(TrivialEncryptBatch {
        values: vec![
            TrivialEncryptRequestSingle {
                handle: lhs.to_vec(),
                be_value: vec![lhs_value],
                output_type: 4,
            },
            TrivialEncryptRequestSingle {
                handle: rhs.to_vec(),
                be_value: vec![rhs_value],
                output_type: 4,
            },
        ],
    });
    encrypt_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&format!("bearer {DEFAULT_API_KEY}"))
            .context("build auth header")?,
    );
    worker_client
        .trivial_encrypt_ciphertexts(encrypt_request)
        .await
        .context("seed lhs/rhs ciphertexts via worker")?;
    Ok(())
}

async fn decrypt_handle_value(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    handle: &[u8],
) -> anyhow::Result<(String, i16)> {
    let (cks_key, sks_key): (Vec<u8>, Vec<u8>) = sqlx::query_as(
        "
        SELECT cks_key, sks_key
        FROM tenants
        WHERE tenant_id = $1
        ",
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await
    .context("load tenant keys for decrypt")?;

    let (ciphertext, ciphertext_type): (Vec<u8>, i16) = sqlx::query_as(
        "
        SELECT ciphertext, ciphertext_type
        FROM ciphertexts
        WHERE tenant_id = $1
          AND handle = $2
          AND ciphertext_version = $3
        ",
    )
    .bind(tenant_id)
    .bind(handle)
    .bind(current_ciphertext_version())
    .fetch_one(pool)
    .await
    .context("load output ciphertext")?;

    let value = tokio::task::spawn_blocking(move || {
        let client_key: tfhe::ClientKey = safe_deserialize(&cks_key).expect("deserialize cks");
        let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key).expect("deserialize sks");
        tfhe::set_server_key(sks);

        let ct = SupportedFheCiphertexts::decompress_no_memcheck(ciphertext_type, &ciphertext)
            .expect("decompress ciphertext");
        ct.decrypt(&client_key)
    })
    .await
    .context("decrypt output ciphertext task")?;

    Ok((value, ciphertext_type))
}

async fn wait_for_output_completion(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    output_handle: &[u8],
    transaction_id: &[u8],
) -> anyhow::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(90);
    loop {
        let completed = sqlx::query_scalar::<_, i64>(
            "
            SELECT COUNT(*)
            FROM computations
            WHERE tenant_id = $1
              AND output_handle = $2
              AND transaction_id = $3
              AND is_completed = TRUE
              AND is_error = FALSE
            ",
        )
        .bind(tenant_id)
        .bind(output_handle)
        .bind(transaction_id)
        .fetch_one(pool)
        .await
        .context("check output completion")?;

        if completed > 0 {
            return Ok(());
        }

        if Instant::now() >= deadline {
            anyhow::bail!("timed out waiting for worker to complete output handle");
        }
        sleep(Duration::from_millis(250)).await;
    }
}

async fn output_ciphertext_count(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    output_handle: &[u8],
) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        "
        SELECT COUNT(*)
        FROM ciphertexts
        WHERE tenant_id = $1
          AND handle = $2
          AND ciphertext_version = $3
        ",
    )
    .bind(tenant_id)
    .bind(output_handle)
    .bind(current_ciphertext_version())
    .fetch_one(pool)
    .await
    .context("count output ciphertext rows")?;
    Ok(count)
}

async fn computation_is_allowed(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    output_handle: &[u8],
) -> anyhow::Result<bool> {
    sqlx::query_scalar::<_, bool>(
        "
        SELECT COALESCE(bool_or(is_allowed), FALSE)
        FROM computations
        WHERE tenant_id = $1
          AND output_handle = $2
        ",
    )
    .bind(tenant_id)
    .bind(output_handle)
    .fetch_one(pool)
    .await
    .context("read computation is_allowed state")
}

fn build_anchor_program() -> anyhow::Result<ProgramMount> {
    let workspace_dir = repo_root()?.join("solana/host-programs");
    let status = Command::new("anchor")
        .arg("build")
        .current_dir(&workspace_dir)
        .status()
        .context("run anchor build")?;
    anyhow::ensure!(status.success(), "anchor build failed");

    let artifact_dir = workspace_dir.join("target/deploy");
    let so_path = artifact_dir.join("zama_host.so");
    anyhow::ensure!(
        so_path.exists(),
        "missing built program artifact: {:?}",
        so_path
    );

    Ok(ProgramMount {
        program_id: SOLANA_PROGRAM_ID_STR.to_string(),
        artifact_dir,
        program_file_name: "zama_host.so".to_string(),
    })
}

fn build_request_add_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    lhs: [u8; 32],
    rhs: [u8; 32],
    is_scalar: bool,
) -> Instruction {
    let mut data = Vec::with_capacity(8 + 32 + 32 + 1);
    data.extend_from_slice(&anchor_global_discriminator("request_add"));
    data.extend_from_slice(&lhs);
    data.extend_from_slice(&rhs);
    data.push(u8::from(is_scalar));
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(signer, true)],
        data,
    }
}

fn build_request_sub_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    lhs: [u8; 32],
    rhs: [u8; 32],
    is_scalar: bool,
) -> Instruction {
    let mut data = Vec::with_capacity(8 + 32 + 32 + 1);
    data.extend_from_slice(&anchor_global_discriminator("request_sub"));
    data.extend_from_slice(&lhs);
    data.extend_from_slice(&rhs);
    data.push(u8::from(is_scalar));
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(signer, true)],
        data,
    }
}

fn anchor_global_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{name}");
    let digest = hash(preimage.as_bytes()).to_bytes();
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

fn build_allow_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    handle: [u8; 32],
    account: Pubkey,
) -> Instruction {
    let mut data = Vec::with_capacity(8 + 32 + 32);
    data.extend_from_slice(&anchor_global_discriminator("allow"));
    data.extend_from_slice(&handle);
    data.extend_from_slice(account.as_ref());
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(signer, true)],
        data,
    }
}

fn derive_result_handle_with_tag(
    lhs: [u8; 32],
    rhs: [u8; 32],
    is_scalar: bool,
    tag: u8,
) -> [u8; 32] {
    let mut output = [0u8; 32];
    for i in 0..32 {
        output[i] = lhs[i] ^ rhs[i];
    }
    output[29] ^= tag;
    if is_scalar {
        output[31] ^= 0x01;
    }
    output
}

fn derive_add_result_handle(lhs: [u8; 32], rhs: [u8; 32], is_scalar: bool) -> [u8; 32] {
    derive_result_handle_with_tag(lhs, rhs, is_scalar, OP_ADD)
}

fn derive_sub_result_handle(lhs: [u8; 32], rhs: [u8; 32], is_scalar: bool) -> [u8; 32] {
    derive_result_handle_with_tag(lhs, rhs, is_scalar, OP_SUB)
}

fn send_instruction(
    client: &RpcClient,
    signer: &Keypair,
    instruction: Instruction,
) -> anyhow::Result<Signature> {
    let blockhash = client
        .get_latest_blockhash()
        .context("get latest blockhash")?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        blockhash,
    );
    let signature = client.send_transaction(&tx).context("send transaction")?;
    let deadline = Instant::now() + Duration::from_secs(90);
    loop {
        let status = client
            .get_signature_status_with_commitment(&signature, CommitmentConfig::finalized())
            .context("query signature status")?;
        if let Some(status) = status {
            if let Err(err) = status {
                anyhow::bail!("transaction failed before finalization: {err}");
            }
            break;
        }

        if Instant::now() >= deadline {
            anyhow::bail!("wait for finalized transaction timed out: {signature}");
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    Ok(signature)
}

async fn ingest_from_cursor(
    source: &mut SolanaRpcEventSource,
    db: &mut Database,
    tenant_id: i32,
    start_cursor: Cursor,
    expected_events: usize,
) -> anyhow::Result<(IngestionSummary, Cursor)> {
    let mut summary = IngestionSummary::default();
    let mut cursor = start_cursor;

    for _ in 0..24 {
        let batch = source.next_batch(cursor, 128, true).await?;
        cursor = batch.next_cursor;
        if batch.events.is_empty() {
            sleep(Duration::from_millis(500)).await;
            continue;
        }

        for envelope in batch.events {
            summary.processed_events += 1;
            let actions = map_envelope_to_actions(&envelope, tenant_id)?;
            let stats = db.apply_actions(&actions).await?;
            summary.inserted_computations += stats.inserted_computations;
            summary.inserted_allowed_handles += stats.inserted_allowed_handles;
            summary.inserted_pbs_computations += stats.inserted_pbs_computations;
        }
        db.set_cursor(cursor.slot as i64).await?;
        if summary.processed_events >= expected_events {
            break;
        }
    }

    anyhow::ensure!(
        summary.processed_events >= expected_events,
        "expected at least {expected_events} decoded events, got {}",
        summary.processed_events
    );
    Ok((summary, cursor))
}

async fn worker_queue_snapshot(
    pool: &sqlx::PgPool,
    tenant_id: i32,
) -> anyhow::Result<WorkerQueueSnapshot> {
    let runnable_rows = sqlx::query_scalar::<_, i64>(
        "
        SELECT COUNT(*)
        FROM computations
        WHERE tenant_id = $1
          AND is_completed = FALSE
          AND is_error = FALSE
          AND is_allowed = TRUE
        ",
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await
    .context("count runnable rows")?;

    let runnable_transactions = sqlx::query_scalar::<_, i64>(
        "
        SELECT COUNT(DISTINCT transaction_id)
        FROM computations
        WHERE tenant_id = $1
          AND is_completed = FALSE
          AND is_error = FALSE
          AND is_allowed = TRUE
          AND transaction_id IS NOT NULL
        ",
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await
    .context("count runnable transactions")?;

    let null_transaction_ids = sqlx::query_scalar::<_, i64>(
        "
        SELECT COUNT(*)
        FROM computations
        WHERE tenant_id = $1
          AND transaction_id IS NULL
        ",
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await
    .context("count null transaction ids")?;

    Ok(WorkerQueueSnapshot {
        runnable_rows,
        runnable_transactions,
        null_transaction_ids,
    })
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
#[ignore = "requires Docker daemon, Solana image, and anchor binary"]
async fn localnet_source_maps_real_events_into_db() -> anyhow::Result<()> {
    let pg = start_postgres_with_migrations().await?;
    let localnet = start_localnet_client_with_program().await?;
    let start_cursor = finalized_cursor(&localnet.rpc_client)?;

    let lhs = [7u8; 32];
    let rhs = [11u8; 32];
    let is_scalar = true;
    let result_handle = derive_add_result_handle(lhs, rhs, is_scalar);
    let allowed_account = Pubkey::new_unique();

    let request_add_sig = send_instruction(
        &localnet.rpc_client,
        &localnet.payer,
        build_request_add_instruction(
            localnet.program_id,
            localnet.payer.pubkey(),
            lhs,
            rhs,
            is_scalar,
        ),
    )?;
    let allow_sig = send_instruction(
        &localnet.rpc_client,
        &localnet.payer,
        build_allow_instruction(
            localnet.program_id,
            localnet.payer.pubkey(),
            result_handle,
            allowed_account,
        ),
    )?;
    info!(
        request_add_signature = %request_add_sig,
        allow_signature = %allow_sig,
        "submitted finalized test transactions"
    );

    let mut source = SolanaRpcEventSource::new(
        localnet.solana.rpc_url.clone(),
        SOLANA_PROGRAM_ID_STR,
        HOST_CHAIN_ID,
    );
    let mut cursor = start_cursor;
    let mut db = Database::connect(&pg.db_url, HOST_CHAIN_ID, TENANT_ID).await?;

    let mut processed_events = 0usize;
    for _ in 0..20 {
        let batch = source.next_batch(cursor, 128, true).await?;
        cursor = batch.next_cursor;
        if batch.events.is_empty() {
            sleep(Duration::from_millis(500)).await;
            continue;
        }

        for envelope in batch.events {
            processed_events += 1;
            let actions = map_envelope_to_actions(&envelope, TENANT_ID)?;
            let _stats = db.apply_actions(&actions).await?;
        }
        db.set_cursor(cursor.slot as i64).await?;
        if processed_events >= 2 {
            break;
        }
    }
    anyhow::ensure!(processed_events >= 2, "expected at least 2 decoded events");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg.db_url)
        .await
        .context("connect for assertions")?;

    let computations_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM computations WHERE tenant_id = $1")
            .bind(TENANT_ID)
            .fetch_one(&pool)
            .await
            .context("count computations")?;
    let allowed_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM allowed_handles WHERE tenant_id = $1")
            .bind(TENANT_ID)
            .fetch_one(&pool)
            .await
            .context("count allowed_handles")?;
    let pbs_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM pbs_computations WHERE tenant_id = $1")
            .bind(TENANT_ID)
            .fetch_one(&pool)
            .await
            .context("count pbs_computations")?;

    let output_handle: Vec<u8> =
        sqlx::query_scalar("SELECT output_handle FROM computations WHERE tenant_id = $1 LIMIT 1")
            .bind(TENANT_ID)
            .fetch_one(&pool)
            .await
            .context("read output handle")?;
    let stored_allowed_account: String = sqlx::query_scalar(
        "SELECT account_address FROM allowed_handles WHERE tenant_id = $1 LIMIT 1",
    )
    .bind(TENANT_ID)
    .fetch_one(&pool)
    .await
    .context("read allowed account")?;

    assert_eq!(computations_count, 1);
    assert_eq!(allowed_count, 1);
    assert_eq!(pbs_count, 1);
    assert_eq!(output_handle, result_handle.to_vec());
    assert_eq!(
        stored_allowed_account,
        hex::encode(allowed_account.to_bytes())
    );

    let cursor_value = sqlx::query_scalar::<_, i64>(
        "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1",
    )
    .bind(HOST_CHAIN_ID)
    .fetch_one(&pool)
    .await
    .context("read poller cursor")?;
    anyhow::ensure!(
        cursor_value >= start_cursor.slot as i64,
        "cursor did not advance"
    );

    let queue = worker_queue_snapshot(&pool, TENANT_ID).await?;
    assert_eq!(queue.runnable_rows, 1);
    assert_eq!(queue.runnable_transactions, 1);
    assert_eq!(queue.null_transaction_ids, 0);

    info!(
        computations_count,
        allowed_count,
        pbs_count,
        cursor_value,
        queue_runnable_rows = queue.runnable_rows,
        queue_runnable_transactions = queue.runnable_transactions,
        "localnet db + worker queue assertions completed"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
#[ignore = "requires Docker daemon, Solana image, and anchor binary"]
async fn localnet_emit_replay_is_idempotent() -> anyhow::Result<()> {
    let pg = start_postgres_with_migrations().await?;
    let localnet = start_localnet_client_with_program().await?;
    let allowed_account = Pubkey::new_unique();
    let mut db = Database::connect(&pg.db_url, HOST_CHAIN_ID, EQUIVALENCE_TENANT_ID).await?;
    let lhs = [17u8; 32];
    let rhs = [31u8; 32];
    let is_scalar = true;
    let result_handle = derive_add_result_handle(lhs, rhs, is_scalar);
    let start_cursor = finalized_cursor(&localnet.rpc_client)?;

    let request_add_sig = send_instruction(
        &localnet.rpc_client,
        &localnet.payer,
        build_request_add_instruction(
            localnet.program_id,
            localnet.payer.pubkey(),
            lhs,
            rhs,
            is_scalar,
        ),
    )?;
    let allow_sig = send_instruction(
        &localnet.rpc_client,
        &localnet.payer,
        build_allow_instruction(
            localnet.program_id,
            localnet.payer.pubkey(),
            result_handle,
            allowed_account,
        ),
    )?;
    info!(
        request_add_signature = %request_add_sig,
        allow_signature = %allow_sig,
        "submitted transactions"
    );

    let mut source = SolanaRpcEventSource::new(
        localnet.solana.rpc_url.clone(),
        SOLANA_PROGRAM_ID_STR,
        HOST_CHAIN_ID,
    );
    let (first_pass, _) =
        ingest_from_cursor(&mut source, &mut db, EQUIVALENCE_TENANT_ID, start_cursor, 2).await?;

    assert_eq!(first_pass.inserted_computations, 1);
    assert_eq!(first_pass.inserted_allowed_handles, 1);
    assert_eq!(first_pass.inserted_pbs_computations, 1);

    let mut replay_source = SolanaRpcEventSource::new(
        localnet.solana.rpc_url.clone(),
        SOLANA_PROGRAM_ID_STR,
        HOST_CHAIN_ID,
    );
    let (replay_pass, _) = ingest_from_cursor(
        &mut replay_source,
        &mut db,
        EQUIVALENCE_TENANT_ID,
        start_cursor,
        2,
    )
    .await?;

    assert_eq!(replay_pass.inserted_computations, 0);
    assert_eq!(replay_pass.inserted_allowed_handles, 0);
    assert_eq!(replay_pass.inserted_pbs_computations, 0);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg.db_url)
        .await
        .context("connect for equivalence assertions")?;
    let computations_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM computations WHERE tenant_id = $1")
            .bind(EQUIVALENCE_TENANT_ID)
            .fetch_one(&pool)
            .await
            .context("count computations")?;
    let allowed_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM allowed_handles WHERE tenant_id = $1")
            .bind(EQUIVALENCE_TENANT_ID)
            .fetch_one(&pool)
            .await
            .context("count allowed_handles")?;
    let pbs_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM pbs_computations WHERE tenant_id = $1")
            .bind(EQUIVALENCE_TENANT_ID)
            .fetch_one(&pool)
            .await
            .context("count pbs_computations")?;

    assert_eq!(computations_count, 1);
    assert_eq!(allowed_count, 1);
    assert_eq!(pbs_count, 1);

    let queue = worker_queue_snapshot(&pool, EQUIVALENCE_TENANT_ID).await?;
    assert_eq!(queue.runnable_rows, 1);
    assert_eq!(queue.runnable_transactions, 1);
    assert_eq!(queue.null_transaction_ids, 0);

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
#[ignore = "requires Docker daemon, Solana image, anchor binary, and tfhe-worker runtime"]
async fn localnet_solana_request_add_computes_and_decrypts() -> anyhow::Result<()> {
    let mut harness = setup_compute_harness().await?;

    let lhs = [0x11_u8; 32];
    let rhs = [0x22_u8; 32];
    let is_scalar = false;
    let result_handle = derive_add_result_handle(lhs, rhs, is_scalar);
    let allowed_account = Pubkey::new_unique();

    seed_trivial_inputs(&mut harness.worker_client, lhs, rhs).await?;

    let start_cursor = finalized_cursor(&harness.localnet.rpc_client)?;
    let request_add_sig = send_instruction(
        &harness.localnet.rpc_client,
        &harness.localnet.payer,
        build_request_add_instruction(
            harness.localnet.program_id,
            harness.localnet.payer.pubkey(),
            lhs,
            rhs,
            is_scalar,
        ),
    )?;
    let _allow_sig = send_instruction(
        &harness.localnet.rpc_client,
        &harness.localnet.payer,
        build_allow_instruction(
            harness.localnet.program_id,
            harness.localnet.payer.pubkey(),
            result_handle,
            allowed_account,
        ),
    )?;

    let mut source = SolanaRpcEventSource::new(
        harness.localnet.solana.rpc_url.clone(),
        SOLANA_PROGRAM_ID_STR,
        HOST_CHAIN_ID,
    );
    let mut db =
        Database::connect(&harness.postgres.db_url, HOST_CHAIN_ID, harness.tenant_id).await?;
    let (ingest_summary, _) =
        ingest_from_cursor(&mut source, &mut db, harness.tenant_id, start_cursor, 2).await?;
    assert_eq!(ingest_summary.inserted_computations, 1);
    assert_eq!(ingest_summary.inserted_allowed_handles, 1);

    let tx_signature = request_add_sig.as_ref().to_vec();
    wait_for_output_completion(
        &harness.pool,
        harness.tenant_id,
        &result_handle,
        &tx_signature,
    )
    .await?;

    let output_exists =
        output_ciphertext_count(&harness.pool, harness.tenant_id, &result_handle).await?;
    assert_eq!(output_exists, 1);

    let (decrypted, output_type) =
        decrypt_handle_value(&harness.pool, harness.tenant_id, &result_handle).await?;
    assert_eq!(output_type, 4);
    assert_eq!(decrypted, "247");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
#[ignore = "requires Docker daemon, Solana image, anchor binary, and tfhe-worker runtime"]
async fn localnet_solana_request_sub_computes_and_decrypts() -> anyhow::Result<()> {
    let mut harness = setup_compute_harness().await?;

    let lhs = [0x91_u8; 32];
    let rhs = [0xA2_u8; 32];
    let is_scalar = false;
    let result_handle = derive_sub_result_handle(lhs, rhs, is_scalar);
    let allowed_account = Pubkey::new_unique();

    seed_trivial_inputs_with_values(&mut harness.worker_client, lhs, rhs, 124, 123).await?;

    let start_cursor = finalized_cursor(&harness.localnet.rpc_client)?;
    let request_sub_sig = send_instruction(
        &harness.localnet.rpc_client,
        &harness.localnet.payer,
        build_request_sub_instruction(
            harness.localnet.program_id,
            harness.localnet.payer.pubkey(),
            lhs,
            rhs,
            is_scalar,
        ),
    )?;
    let _allow_sig = send_instruction(
        &harness.localnet.rpc_client,
        &harness.localnet.payer,
        build_allow_instruction(
            harness.localnet.program_id,
            harness.localnet.payer.pubkey(),
            result_handle,
            allowed_account,
        ),
    )?;

    let mut source = SolanaRpcEventSource::new(
        harness.localnet.solana.rpc_url.clone(),
        SOLANA_PROGRAM_ID_STR,
        HOST_CHAIN_ID,
    );
    let mut db =
        Database::connect(&harness.postgres.db_url, HOST_CHAIN_ID, harness.tenant_id).await?;
    let (ingest_summary, _) =
        ingest_from_cursor(&mut source, &mut db, harness.tenant_id, start_cursor, 2).await?;
    assert_eq!(ingest_summary.inserted_computations, 1);
    assert_eq!(ingest_summary.inserted_allowed_handles, 1);

    let tx_signature = request_sub_sig.as_ref().to_vec();
    wait_for_output_completion(
        &harness.pool,
        harness.tenant_id,
        &result_handle,
        &tx_signature,
    )
    .await?;

    let output_exists =
        output_ciphertext_count(&harness.pool, harness.tenant_id, &result_handle).await?;
    assert_eq!(output_exists, 1);

    let (decrypted, output_type) =
        decrypt_handle_value(&harness.pool, harness.tenant_id, &result_handle).await?;
    assert_eq!(output_type, 4);
    assert_eq!(decrypted, "1");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
#[ignore = "requires Docker daemon, Solana image, anchor binary, and tfhe-worker runtime"]
async fn localnet_acl_gate_blocks_then_allows_compute() -> anyhow::Result<()> {
    let mut harness = setup_compute_harness().await?;

    let lhs = [0x55_u8; 32];
    let rhs = [0x66_u8; 32];
    let is_scalar = false;
    let result_handle = derive_add_result_handle(lhs, rhs, is_scalar);
    let allowed_account = Pubkey::new_unique();

    seed_trivial_inputs(&mut harness.worker_client, lhs, rhs).await?;

    let start_cursor = finalized_cursor(&harness.localnet.rpc_client)?;
    let request_add_sig = send_instruction(
        &harness.localnet.rpc_client,
        &harness.localnet.payer,
        build_request_add_instruction(
            harness.localnet.program_id,
            harness.localnet.payer.pubkey(),
            lhs,
            rhs,
            is_scalar,
        ),
    )?;

    let mut source = SolanaRpcEventSource::new(
        harness.localnet.solana.rpc_url.clone(),
        SOLANA_PROGRAM_ID_STR,
        HOST_CHAIN_ID,
    );
    let mut db =
        Database::connect(&harness.postgres.db_url, HOST_CHAIN_ID, harness.tenant_id).await?;
    let (_first_ingest, cursor_after_add) =
        ingest_from_cursor(&mut source, &mut db, harness.tenant_id, start_cursor, 1).await?;

    assert!(!computation_is_allowed(&harness.pool, harness.tenant_id, &result_handle).await?);
    assert_eq!(
        output_ciphertext_count(&harness.pool, harness.tenant_id, &result_handle).await?,
        0
    );

    sleep(Duration::from_secs(2)).await;
    assert_eq!(
        output_ciphertext_count(&harness.pool, harness.tenant_id, &result_handle).await?,
        0
    );

    let _allow_sig = send_instruction(
        &harness.localnet.rpc_client,
        &harness.localnet.payer,
        build_allow_instruction(
            harness.localnet.program_id,
            harness.localnet.payer.pubkey(),
            result_handle,
            allowed_account,
        ),
    )?;

    let mut replay_source = SolanaRpcEventSource::new(
        harness.localnet.solana.rpc_url.clone(),
        SOLANA_PROGRAM_ID_STR,
        HOST_CHAIN_ID,
    );
    let (_second_ingest, _) = ingest_from_cursor(
        &mut replay_source,
        &mut db,
        harness.tenant_id,
        cursor_after_add,
        1,
    )
    .await?;

    assert!(computation_is_allowed(&harness.pool, harness.tenant_id, &result_handle).await?);

    let tx_signature = request_add_sig.as_ref().to_vec();
    wait_for_output_completion(
        &harness.pool,
        harness.tenant_id,
        &result_handle,
        &tx_signature,
    )
    .await?;
    assert_eq!(
        output_ciphertext_count(&harness.pool, harness.tenant_id, &result_handle).await?,
        1
    );

    Ok(())
}
