//! Load gate for the minimal v0.13 hotfix plan.
//!
//! Ignored by default; driven explicitly with env vars so one build can run the
//! whole comparison matrix:
//!
//! ```sh
//! LOAD_OUT=/tmp/run.csv \
//!   cargo test --locked -p transaction-sender --test hotfix_load_gate -- --ignored --nocapture
//! ```
//!
//! | var | default | meaning |
//! |---|---|---|
//! | `LOAD_BACKLOG` | 1024 | eligible items seeded per class (B) |
//! | `LOAD_ARRIVAL` | 2 | new rows per second per class (input) |
//! | `LOAD_DEADLINE` | 600 | recovery deadline D, seconds |
//! | `LOAD_MAX_INFLIGHT` | 8 | shared admission limit |
//! | `LOAD_PRERETRIED` | 0 | 1 = every seeded row already has retries |
//! | `LOAD_BLOCK_TIME` | 1 | anvil block seconds; 0 = automine |
//! | `LOAD_FAULT_SECS` | 90 | length of the fault window |
//! | `LOAD_MEASURE_SECS` | 600 | post-restore measurement window |
//! | `LOAD_RESTART` | 0 | 1 = restart the sender mid-backlog |
//! | `LOAD_SEED` | 1 | run label, for repeated seeded runs |
//!
//! Measurements go to `LOAD_OUT` as CSV, one row per second, plus a summary
//! line on stdout. Nothing here asserts a verdict: the gate is evaluated from
//! the recorded numbers, so a failing run still produces its evidence.

mod common;
mod support;

use std::time::Duration;

use alloy::providers::ext::AnvilApi;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use rand::random;
use serial_test::serial;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use transaction_sender::{
    ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

use support::{Fault, FaultProxy};

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

struct Cfg {
    backlog: u64,
    arrival: u64,
    deadline: u64,
    max_inflight: u64,
    preretried: bool,
    block_time: u64,
    fault_secs: u64,
    measure_secs: u64,
    restart: bool,
    seed: u64,
    /// "queue" (default) models one shared provider: estimate latency grows with
    /// concurrency. "flat" is the plan's literal "delay estimates to 5 s", which
    /// does not penalize unbounded concurrency and so cannot reproduce the
    /// incident's prep-latency inflation.
    fault_model: String,
    /// `--verify-proof-resp-batch-limit` / `--add-ciphertexts-batch-limit`.
    /// Exposed because the shared admission gate is FIFO, so the ratio of the
    /// two batch limits sets how throughput is split between the operations.
    vp_batch: u64,
    add_batch: u64,
    /// `--verify-proof-remove-after-max-retries`. Set to 0 to attribute an
    /// accounting residual: with deletion off, nothing can vanish at the retry
    /// cap, so the identity must balance exactly.
    remove_at_cap: u64,
    receipt_timeout: u64,
    out: String,
}

impl Cfg {
    fn from_env() -> Self {
        Self {
            backlog: env_u64("LOAD_BACKLOG", 1024),
            arrival: env_u64("LOAD_ARRIVAL", 2),
            deadline: env_u64("LOAD_DEADLINE", 600),
            max_inflight: env_u64("LOAD_MAX_INFLIGHT", 8),
            preretried: env_u64("LOAD_PRERETRIED", 0) == 1,
            block_time: env_u64("LOAD_BLOCK_TIME", 1),
            fault_secs: env_u64("LOAD_FAULT_SECS", 90),
            measure_secs: env_u64("LOAD_MEASURE_SECS", 600),
            restart: env_u64("LOAD_RESTART", 0) == 1,
            seed: env_u64("LOAD_SEED", 1),
            fault_model: std::env::var("LOAD_FAULT_MODEL").unwrap_or_else(|_| "queue".to_string()),
            receipt_timeout: env_u64("LOAD_RECEIPT_TIMEOUT", 30),
            remove_at_cap: env_u64("LOAD_REMOVE_AT_CAP", 1),
            vp_batch: env_u64("LOAD_VP_BATCH", 128),
            add_batch: env_u64("LOAD_ADD_BATCH", 10),
            out: std::env::var("LOAD_OUT").unwrap_or_else(|_| "/tmp/load_gate.csv".to_string()),
        }
    }
}

/// Reads a counter/gauge value out of the default prometheus registry by name.
/// The plan asks for existing counters rather than a new metric suite.
fn metric(name: &str) -> i64 {
    use prometheus::Encoder;
    let mut buf = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    if encoder.encode(&prometheus::gather(), &mut buf).is_err() {
        return 0;
    }
    let text = String::from_utf8_lossy(&buf);
    for line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.rsplit_once(' ') else {
            continue;
        };
        if key.trim() == name {
            return value.trim().parse::<f64>().unwrap_or(0.0) as i64;
        }
    }
    0
}

/// Service time of the modeled shared-provider channel. Calibrated so that the
/// baseline's 128 concurrent estimates queue to the p50 the incident recorded
/// (128 x 40 ms ~= 5.1 s, against 5.2-6.0 s observed in the field), while the
/// candidate's admission limit of 8 gives 8 x 40 ms ~= 0.32 s.
const QUEUE_SERVICE_MS: u64 = 40;

const USER: &str = "0x1234567890abcdef1234567890abcdef12345678";
const CONTRACT: &str = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd";

const NONCE_ERROR_PATTERNS: [&str; 4] = [
    "nonce too low",
    "nonce too high",
    "already known",
    "replacement transaction underpriced",
];

/// Independent accounting, kept outside the database so that cleanup deleting
/// rows cannot erase the evidence. Counts what the harness itself did.
#[derive(Default)]
struct Ledger {
    adds_inserted: AtomicU64,
    proofs_inserted: AtomicU64,
    insert_failures: AtomicU64,
    probe_failures: AtomicU64,
}

#[derive(Default, Clone)]
struct Sample {
    t: u64,
    phase: String,
    adds_done: i64,
    adds_left: i64,
    proofs_left: i64,
    add_success: i64,
    add_fail: i64,
    vp_success: i64,
    vp_fail: i64,
    nonce: u64,
    est_calls: usize,
    send_calls: usize,
    txcount_calls: usize,
    est_peak: usize,
    send_peak: usize,
    oldest_add_age: f64,
    oldest_proof_age: f64,
    nonce_errors: i64,
    proofs_exhausted: i64,
    est_p50_ms: u64,
    est_p90_ms: u64,
    /// Acknowledged by the node but not yet mined: pending nonce - latest nonce.
    outstanding: u64,
}

/// Everything a sample reads that does not change between samples, so the
/// per-sample call carries only the instant and the phase.
struct Sampler<'a, P: Provider<alloy::network::Ethereum>> {
    pool: &'a sqlx::PgPool,
    proxy: &'a FaultProxy,
    // Deliberately NOT the proxied provider: instrumentation must never travel
    // the faulted path, or a blackholed method stalls the sampler itself.
    probe: &'a P,
    ledger: &'a Ledger,
    signer: alloy::primitives::Address,
    max_retries: i32,
}

impl<P: Provider<alloy::network::Ethereum>> Sampler<'_, P> {
    async fn sample(&self, t: u64, phase: &str) -> anyhow::Result<Sample> {
        let adds_done: i64 =
            sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest WHERE txn_is_sent = true")
                .fetch_one(self.pool)
                .await?;
        let adds_left: i64 =
            sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest WHERE txn_is_sent = false")
                .fetch_one(self.pool)
                .await?;
        let proofs_left: i64 = sqlx::query_scalar("SELECT count(*) FROM verify_proofs")
            .fetch_one(self.pool)
            .await?;
        let oldest_add_age: Option<f64> = sqlx::query_scalar(
            "SELECT EXTRACT(EPOCH FROM (now() - min(created_at)))::float8
             FROM ciphertext_digest WHERE txn_is_sent = false",
        )
        .fetch_one(self.pool)
        .await?;
        let oldest_proof_age: Option<f64> = sqlx::query_scalar(
            "SELECT EXTRACT(EPOCH FROM (now() - min(created_at)))::float8 FROM verify_proofs",
        )
        .fetch_one(self.pool)
        .await?;
        // Counted from RPC responses at the self.proxy. The database `last_error` column
        // is overwritten by the next error and erased when a row is deleted, so it
        // cannot count occurrences.
        let nonce_errors: i64 = NONCE_ERROR_PATTERNS
            .iter()
            .map(|p| self.proxy.response_errors(p) as i64)
            .sum();
        let proofs_exhausted: i64 =
            sqlx::query_scalar("SELECT count(*) FROM verify_proofs WHERE retry_count >= $1")
                .bind(self.max_retries)
                .fetch_one(self.pool)
                .await?;

        Ok(Sample {
            t,
            phase: phase.to_string(),
            adds_done,
            adds_left,
            proofs_left,
            add_success: metric("coprocessor_txn_sender_add_ciphertext_material_success_counter"),
            add_fail: metric("coprocessor_txn_sender_add_ciphertext_material_fail_counter"),
            vp_success: metric("coprocessor_txn_sender_verify_proof_success_counter"),
            vp_fail: metric("coprocessor_txn_sender_verify_proof_fail_counter"),
            nonce: match tokio::time::timeout(
                Duration::from_secs(2),
                self.probe.get_transaction_count(self.signer),
            )
            .await
            {
                Ok(Ok(n)) => n,
                // A failed probe is unknown, not zero. Counted so the run can say so.
                _ => {
                    self.ledger.probe_failures.fetch_add(1, Ordering::Relaxed);
                    u64::MAX
                }
            },
            est_calls: self.proxy.calls("eth_estimateGas"),
            send_calls: self.proxy.calls("eth_sendRawTransaction"),
            txcount_calls: self.proxy.calls("eth_getTransactionCount"),
            est_peak: self.proxy.max_concurrency("eth_estimateGas"),
            send_peak: self.proxy.max_concurrency("eth_sendRawTransaction"),
            oldest_add_age: oldest_add_age.unwrap_or(0.0),
            oldest_proof_age: oldest_proof_age.unwrap_or(0.0),
            nonce_errors,
            proofs_exhausted,
            est_p50_ms: self.proxy.latency_pct("eth_estimateGas", 0.50),
            est_p90_ms: self.proxy.latency_pct("eth_estimateGas", 0.90),
            outstanding: {
                let pending = tokio::time::timeout(
                    Duration::from_secs(2),
                    self.probe.get_transaction_count(self.signer).pending(),
                )
                .await;
                let latest = tokio::time::timeout(
                    Duration::from_secs(2),
                    self.probe.get_transaction_count(self.signer),
                )
                .await;
                match (pending, latest) {
                    (Ok(Ok(p)), Ok(Ok(l))) => p.saturating_sub(l),
                    _ => {
                        self.ledger.probe_failures.fetch_add(1, Ordering::Relaxed);
                        u64::MAX
                    }
                }
            },
        })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[serial(db)]
#[ignore = "load gate; run explicitly with --ignored"]
async fn load_gate() -> anyhow::Result<()> {
    let cfg = Cfg::from_env();
    eprintln!(
        "load gate: backlog={} arrival={}/s deadline={}s max_inflight={} \
         preretried={} block_time={} fault={}s measure={}s restart={} seed={}",
        cfg.backlog,
        cfg.arrival,
        cfg.deadline,
        cfg.max_inflight,
        cfg.preretried,
        cfg.block_time,
        cfg.fault_secs,
        cfg.measure_secs,
        cfg.restart,
        cfg.seed
    );

    let conf = ConfigSettings {
        // Production arguments from the plan's baseline.
        verify_proof_resp_batch_limit: cfg.vp_batch as u32,
        add_ciphertexts_batch_limit: cfg.add_batch as u32,
        gas_limit_overprovision_percent: 300,
        send_txn_sync_timeout_secs: 4,
        txn_receipt_timeout_secs: cfg.receipt_timeout as u16,
        error_sleep_initial_secs: 1,
        error_sleep_max_secs: 4,
        verify_proof_resp_max_retries: 6,
        verify_proof_remove_after_max_retries: cfg.remove_at_cap == 1,
        gas_estimation_timeout_secs: 20,
        graceful_shutdown_timeout: Duration::from_secs(8),
        ..ConfigSettings::default()
    };
    let max_retries = conf.verify_proof_resp_max_retries as i32;

    // `block_time = 0` means anvil automine: one block per transaction, which
    // measures the sender's own serialized capacity rather than the chain's
    // block cadence.
    let env = if cfg.block_time == 0 {
        TestEnvironment::new_with_config_and_anvil_args(
            SignerType::PrivateKey,
            conf.clone(),
            false,
            &["--accounts", "10"],
        )
        .await?
    } else {
        TestEnvironment::new_with_config(SignerType::PrivateKey, conf.clone(), false).await?
    };

    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;

    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let ciphertext_commits = CiphertextCommits::deploy(&provider_deploy, false).await?;
    let input_verification =
        InputVerification::deploy(&provider_deploy, false, false, false, false).await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_http(proxy.url()),
        Some(env.signer_address()),
    )
    .with_max_inflight(cfg.max_inflight as usize);

    // Measurement path, straight to anvil, bypassing the fault proxy.
    let probe_provider = ProviderBuilder::new()
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;

    let (host_chain_id, key_id) =
        test_harness::db_utils::insert_random_keys_and_host_chain(&env.db_pool).await?;

    // ---- seed the backlog -------------------------------------------------
    let (limited, unlimited) = if cfg.preretried { (1, 1) } else { (0, 0) };
    for chunk_start in (0..cfg.backlog).step_by(256) {
        let chunk_end = (chunk_start + 256).min(cfg.backlog);
        let mut q = sqlx::QueryBuilder::new(
            "INSERT INTO ciphertext_digest
                (host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
                 txn_limited_retries_count, txn_unlimited_retries_count) ",
        );
        q.push_values(chunk_start..chunk_end, |mut b, _| {
            b.push_bind(host_chain_id)
                .push_bind(key_id.to_vec())
                .push_bind(random::<[u8; 32]>().to_vec())
                .push_bind(random::<[u8; 32]>().to_vec())
                .push_bind(random::<[u8; 32]>().to_vec())
                .push_bind(limited)
                .push_bind(unlimited);
        });
        q.build().execute(&env.db_pool).await?;
    }
    for chunk_start in (0..cfg.backlog).step_by(256) {
        let chunk_end = (chunk_start + 256).min(cfg.backlog);
        let mut q = sqlx::QueryBuilder::new(
            "INSERT INTO verify_proofs
                (zk_proof_id, chain_id, contract_address, user_address, handles,
                 verified, retry_count) ",
        );
        q.push_values(chunk_start..chunk_end, |mut b, i| {
            // Some rows deliberately near the retry limit, never reset.
            let retry = if !cfg.preretried {
                0
            } else if i % 32 == 0 {
                max_retries - 1
            } else {
                1
            };
            b.push_bind(i as i64)
                .push_bind(1_i32)
                .push_bind(CONTRACT)
                .push_bind(USER)
                .push_bind(random::<[u8; 32]>().to_vec())
                .push_bind(i % 8 != 0) // a mix of verify and reject
                .push_bind(retry);
        });
        q.build().execute(&env.db_pool).await?;
    }
    eprintln!("seeded {} rows per class", cfg.backlog);

    // ---- continuous arrivals ---------------------------------------------
    let arrivals_token = CancellationToken::new();
    let ledger = Arc::new(Ledger::default());
    ledger
        .adds_inserted
        .fetch_add(cfg.backlog, Ordering::Relaxed);
    ledger
        .proofs_inserted
        .fetch_add(cfg.backlog, Ordering::Relaxed);
    let arrivals = {
        let pool = env.db_pool.clone();
        let token = arrivals_token.clone();
        let ledger = ledger.clone();
        let rate = cfg.arrival;
        let add_channel = conf.add_ciphertexts_db_channel.clone();
        let vp_channel = conf.verify_proof_resp_db_channel.clone();
        let mut next_proof_id = cfg.backlog as i64;
        tokio::spawn(async move {
            if rate == 0 {
                return;
            }
            let mut ticker = tokio::time::interval(Duration::from_millis(1000 / rate.max(1)));
            loop {
                if token.is_cancelled() {
                    return;
                }
                ticker.tick().await;
                match sqlx::query(
                    "INSERT INTO ciphertext_digest
                        (host_chain_id, key_id_gw, handle, ciphertext, ciphertext128)
                     VALUES ($1, $2, $3, $4, $5)",
                )
                .bind(host_chain_id)
                .bind(key_id.to_vec())
                .bind(random::<[u8; 32]>().to_vec())
                .bind(random::<[u8; 32]>().to_vec())
                .bind(random::<[u8; 32]>().to_vec())
                .execute(&pool)
                .await
                {
                    Ok(_) => ledger.adds_inserted.fetch_add(1, Ordering::Relaxed),
                    Err(_) => ledger.insert_failures.fetch_add(1, Ordering::Relaxed),
                };
                match sqlx::query(
                    "INSERT INTO verify_proofs
                        (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
                     VALUES ($1, 1, $2, $3, $4, true)",
                )
                .bind(next_proof_id)
                .bind(CONTRACT)
                .bind(USER)
                .bind(random::<[u8; 32]>().to_vec())
                .execute(&pool)
                .await
                {
                    Ok(_) => ledger.proofs_inserted.fetch_add(1, Ordering::Relaxed),
                    Err(_) => ledger.insert_failures.fetch_add(1, Ordering::Relaxed),
                };
                next_proof_id += 1;
                let _ = sqlx::query("SELECT pg_notify($1, '')")
                    .bind(&add_channel)
                    .execute(&pool)
                    .await;
                let _ = sqlx::query("SELECT pg_notify($1, '')")
                    .bind(&vp_channel)
                    .execute(&pool)
                    .await;
            }
        })
    };

    // ---- sender ----------------------------------------------------------
    let mut sender_token = CancellationToken::new();
    // A restart must not inherit the previous process's nonce state. Cloning the
    // provider would carry its mutex, cached sequence and semaphore across the
    // restart, which is exactly what a real process restart loses.
    let make_sender = |token: CancellationToken, fresh: bool| {
        let db = env.db_pool.clone();
        let iv = *input_verification.address();
        let cc = *ciphertext_commits.address();
        let signer = env.signer.clone();
        let conf = conf.clone();
        let wallet = env.wallet.clone();
        let signer_address = env.signer_address();
        let proxy_url = proxy.url();
        let existing = provider.clone();
        let max_inflight = cfg.max_inflight as usize;
        async move {
            let p = if fresh {
                NonceManagedProvider::new(
                    ProviderBuilder::default()
                        .filler(FillersWithoutNonceManagement::default())
                        .wallet(wallet)
                        .connect_http(proxy_url),
                    Some(signer_address),
                )
                .with_max_inflight(max_inflight)
            } else {
                existing
            };
            TransactionSender::new(db, iv, cc, signer, p, token, conf, None).await
        }
    };
    let mut run_handle = {
        let s = make_sender(sender_token.clone(), false).await?;
        tokio::spawn(async move { s.run().await })
    };

    // ---- fault schedule + sampling ---------------------------------------
    let mut out = std::fs::File::create(&cfg.out)?;
    writeln!(
        out,
        "t,phase,adds_done,adds_left,proofs_left,add_success,add_fail,vp_success,vp_fail,\
         nonce,est_calls,send_calls,txcount_calls,est_peak,send_peak,oldest_add_age,\
         oldest_proof_age,nonce_errors,proofs_exhausted,est_p50_ms,est_p90_ms,outstanding"
    )?;

    let total_secs = cfg.fault_secs + cfg.measure_secs;
    let mut samples: Vec<Sample> = Vec::new();
    let signer_addr = env.signer_address();
    let started = tokio::time::Instant::now();
    let mut staged = [false; 6];
    let sampler = Sampler {
        pool: &env.db_pool,
        proxy: &proxy,
        probe: &probe_provider,
        ledger: &ledger,
        signer: signer_addr,
        max_retries,
    };

    loop {
        let t = started.elapsed().as_secs();
        if t >= total_secs {
            break;
        }
        let phase = if t < cfg.fault_secs {
            // Apply the plan's fault set, staged inside the fault window.
            if !staged[0] && {
                staged[0] = true;
                true
            } {
                if cfg.fault_model == "flat" {
                    // The plan's literal wording: a flat 5 s per estimate.
                    proxy.set_fault(
                        "eth_estimateGas",
                        Fault::Delay(Duration::from_secs(5)),
                        None,
                    );
                } else {
                    // Contention model: one service channel, so latency scales
                    // with how many estimates are in flight.
                    proxy.set_fault(
                        "eth_estimateGas",
                        Fault::Queue {
                            service: Duration::from_millis(QUEUE_SERVICE_MS),
                        },
                        None,
                    );
                }
                // A few accepted sends whose response never arrives.
                proxy.set_fault("eth_sendRawTransaction", Fault::SuppressResponse, Some(5));
            }
            if !staged[1] && t >= cfg.fault_secs / 3 {
                staged[1] = true;
                // An original submission delayed beyond its client timeout.
                proxy.set_fault(
                    "eth_sendRawTransaction",
                    Fault::DelayResponse(Duration::from_secs(10)),
                    Some(5),
                );
            }
            if !staged[2] && t >= cfg.fault_secs / 2 {
                staged[2] = true;
                // Blackhole the pending-count calls.
                proxy.set_fault("eth_getTransactionCount", Fault::Blackhole, None);
            }
            if !staged[3] && t >= cfg.fault_secs / 2 + 20 {
                staged[3] = true;
                proxy.clear_fault("eth_getTransactionCount");
                eprintln!("t={t}: pending-count blackhole lifted");
            }
            "fault"
        } else {
            if !staged[4] {
                staged[4] = true;
                proxy.clear_all_faults();
                // Percentiles from here on describe the recovery phase only;
                // mixing them with the fault window would understate both.
                let fault_p50 = proxy.latency_pct("eth_estimateGas", 0.50);
                let fault_p90 = proxy.latency_pct("eth_estimateGas", 0.90);
                eprintln!(
                    "t={t}: faults cleared. Fault-phase estimate latency p50/p90 = {fault_p50}/{fault_p90} ms"
                );
                proxy.reset_latencies();
            }
            if cfg.restart && !staged[5] && t >= cfg.fault_secs + 30 {
                staged[5] = true;
                eprintln!("t={t}: restarting the sender with the 8 s grace");
                // Hold inclusion first. Otherwise graceful shutdown can finish
                // the outstanding batch and the "restart with pending work"
                // scenario never actually occurs.
                probe_provider.anvil_set_interval_mining(0).await?;
                probe_provider.anvil_set_auto_mine(false).await?;
                sender_token.cancel();
                // The old task must actually terminate; a restart that overlaps
                // the previous sender would test nothing.
                match tokio::time::timeout(Duration::from_secs(20), &mut run_handle).await {
                    Ok(joined) => eprintln!(
                        "t={t}: previous sender terminated, outcome ok={}",
                        joined.is_ok()
                    ),
                    Err(_) => anyhow::bail!(
                        "the sender did not terminate within the graceful shutdown window"
                    ),
                }
                // The scenario under test is cold recovery *with work in
                // flight*, so assert that is what we have before restarting.
                let pending_at_restart = probe_provider
                    .get_transaction_count(signer_addr)
                    .pending()
                    .await?
                    .saturating_sub(probe_provider.get_transaction_count(signer_addr).await?);
                let unsent_at_restart: i64 = sqlx::query_scalar(
                    "SELECT count(*) FROM ciphertext_digest WHERE txn_is_sent = false",
                )
                .fetch_one(&env.db_pool)
                .await?;
                eprintln!(
                    "t={t}: at restart - {pending_at_restart} transactions unmined, \
                     {unsent_at_restart} rows unsent"
                );
                if pending_at_restart == 0 {
                    anyhow::bail!(
                        "no transaction was left unmined across the restart, so this run \
                         does not exercise cold recovery with pending work"
                    );
                }
                sender_token = CancellationToken::new();
                // Cold start: brand-new provider, nonce sequence and semaphore,
                // with transactions still outstanding on chain.
                let s = make_sender(sender_token.clone(), true).await?;
                run_handle = tokio::spawn(async move { s.run().await });
                // Let the chain move again; the fresh sender must recover the
                // sequence with those transactions still outstanding.
                probe_provider
                    .anvil_set_interval_mining(cfg.block_time.max(1))
                    .await?;
            }
            "recover"
        };

        let s = sampler.sample(t, phase).await?;
        writeln!(
            out,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.1},{:.1},{},{},{},{},{}",
            s.t,
            s.phase,
            s.adds_done,
            s.adds_left,
            s.proofs_left,
            s.add_success,
            s.add_fail,
            s.vp_success,
            s.vp_fail,
            s.nonce,
            s.est_calls,
            s.send_calls,
            s.txcount_calls,
            s.est_peak,
            s.send_peak,
            s.oldest_add_age,
            s.oldest_proof_age,
            s.nonce_errors,
            s.proofs_exhausted,
            s.est_p50_ms,
            s.est_p90_ms,
            s.outstanding
        )?;
        out.flush()?;
        if t.is_multiple_of(30) {
            eprintln!(
                "t={t} [{}] adds_left={} proofs_left={} done={} vp_ok={} est={} nonce={}",
                s.phase,
                s.adds_left,
                s.proofs_left,
                s.add_success,
                s.vp_success,
                s.est_calls,
                s.nonce
            );
        }
        samples.push(s);

        // Stop early once both classes are drained to just the arrival flow.
        let drained = samples
            .last()
            .map(|s| s.phase == "recover" && s.adds_left <= 2 && s.proofs_left <= 2)
            .unwrap_or(false);
        if drained && t > cfg.fault_secs + 5 {
            eprintln!("t={t}: both classes drained");
            break;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    // ---- quiesce before accounting --------------------------------------
    // The final identity must be evaluated when nothing is moving. Comparing
    // insertion counters against a sample taken while the sender was still
    // running mixes observations from different times, so a residual could not
    // be distinguished from a dropped item.
    arrivals_token.cancel();
    let _ = arrivals.await;
    sender_token.cancel();
    let shutdown_ok = tokio::time::timeout(Duration::from_secs(30), run_handle)
        .await
        .is_ok();
    if !shutdown_ok {
        eprintln!("WARNING: the sender did not terminate within 30 s; accounting may be racy");
    }
    // Give any in-flight transaction a chance to mine before the final read.
    tokio::time::sleep(Duration::from_secs(3)).await;

    let final_sample = sampler
        .sample(samples.last().map(|s| s.t).unwrap_or(0) + 3, "final")
        .await?;

    // ---- summary ---------------------------------------------------------
    let first_recover = samples
        .iter()
        .position(|s| s.phase == "recover")
        .unwrap_or(0);
    let start = &samples[first_recover];
    let end = samples.last().expect("at least one sample");
    // Rates come from the in-run samples; accounting comes from the quiesced read.
    let fin = &final_sample;
    let window = (end.t - start.t).max(1) as f64;
    let add_useful = end.add_success - start.add_success;
    let vp_useful = end.vp_success - start.vp_success;
    let useful = add_useful + vp_useful;
    let rate = useful as f64 / window;
    let add_rate = add_useful as f64 / window;
    let vp_rate = vp_useful as f64 / window;
    let required = cfg.arrival as f64 * 2.0 + (cfg.backlog as f64 * 2.0) / cfg.deadline as f64;

    eprintln!("================ load gate summary ================");
    // LOAD_SEED is a run label; it does not seed rand::random, so repeated runs
    // are repetitions under the same distribution, not reproducible replays.
    eprintln!("repetition label         : {}", cfg.seed);
    eprintln!("pre-retried backlog      : {}", cfg.preretried);
    eprintln!("block time (s, 0=auto)   : {}", cfg.block_time);
    eprintln!("recovery window measured : {window:.0}s");
    eprintln!("useful completions       : {useful}");
    eprintln!("sustained useful rate    : {rate:.2}/s");
    eprintln!(
        "required (input + 2B/D)  : {required:.2}/s  [input={}/s, B={} per class, D={}s]",
        cfg.arrival * 2,
        cfg.backlog,
        cfg.deadline
    );
    eprintln!("  add-ciphertext rate    : {add_rate:.2}/s ({add_useful} completions)");
    eprintln!("  verify-proof rate      : {vp_rate:.2}/s ({vp_useful} completions)");
    eprintln!(
        "starvation check         : {}",
        if add_rate > 0.05 && vp_rate > 0.05 {
            "both operations progressing"
        } else {
            "STARVED: one operation made essentially no progress"
        }
    );
    eprintln!(
        "verdict                  : {}",
        if rate >= required { "PASS" } else { "FAIL" }
    );
    // Independent accounting: what the harness inserted, against terminal
    // outcomes. Rows deleted by cleanup are invisible to a database count, so
    // this identity is what detects queue shrinkage that is not real progress.
    let proofs_inserted = ledger.proofs_inserted.load(Ordering::Relaxed) as i64;
    let adds_inserted = ledger.adds_inserted.load(Ordering::Relaxed) as i64;
    // Evaluated at quiescence: arrivals stopped, sender terminated, chain settled.
    // Any residual here is a real discrepancy, not a sampling race.
    let proofs_unaccounted = proofs_inserted - fin.vp_success - fin.proofs_left;
    let adds_unaccounted = adds_inserted - fin.adds_done - fin.adds_left;
    eprintln!("--- accounting at quiescence (arrivals stopped, sender terminated) ---");
    eprintln!("retry-cap deletion enabled: {}", cfg.remove_at_cap == 1);
    eprintln!("clean shutdown            : {shutdown_ok}");
    eprintln!(
        "proofs inserted/succeeded/remaining : {proofs_inserted} / {} / {}",
        fin.vp_success, fin.proofs_left
    );
    eprintln!(
        "proofs unaccounted        : {proofs_unaccounted}  {}",
        if proofs_unaccounted == 0 {
            "(balanced)"
        } else {
            "(UNEXPLAINED - every discrepancy must be attributed, e.g. retry-cap deletion)"
        }
    );
    eprintln!(
        "adds inserted/sent/remaining        : {adds_inserted} / {} / {}",
        fin.adds_done, fin.adds_left
    );
    eprintln!(
        "adds unaccounted          : {adds_unaccounted}  {}",
        if adds_unaccounted == 0 {
            "(balanced)"
        } else {
            "(UNEXPLAINED)"
        }
    );
    eprintln!(
        "dropped-proof verdict     : {}",
        if proofs_unaccounted == 0 && adds_unaccounted == 0 {
            "PROVEN ZERO at quiescence"
        } else {
            "NOT PROVEN - residual is unexplained"
        }
    );
    eprintln!(
        "insert failures           : {}",
        ledger.insert_failures.load(Ordering::Relaxed)
    );
    eprintln!(
        "probe failures            : {}  (samples recorded as unknown)",
        ledger.probe_failures.load(Ordering::Relaxed)
    );
    eprintln!("adds left at end         : {}", fin.adds_left);
    eprintln!("proofs left at end       : {}", fin.proofs_left);
    eprintln!("proofs at/over retry cap : {}", fin.proofs_exhausted);
    eprintln!(
        "nonce-error responses    : {}  (counted at the proxy, per occurrence)",
        end.nonce_errors
    );
    eprintln!("fault model              : {}", cfg.fault_model);
    eprintln!(
        "batch limits (vp/add)    : {}/{}",
        cfg.vp_batch, cfg.add_batch
    );
    eprintln!(
        "estimate latency p50/p90 : {} ms / {} ms  (RECOVERY PHASE ONLY; gate 1: p50 < 500 ms). \
         Measures eth_estimateGas service time at the proxy, not full preparation.",
        end.est_p50_ms, end.est_p90_ms
    );
    eprintln!("receipt timeout (s)      : {}", cfg.receipt_timeout);
    eprintln!("outstanding at end        : {}", end.outstanding);
    eprintln!(
        "peak outstanding          : {}",
        samples.iter().map(|s| s.outstanding).max().unwrap_or(0)
    );
    eprintln!("estimateGas calls         : {}", end.est_calls);
    eprintln!("estimateGas peak conc     : {}", end.est_peak);
    eprintln!("sendRawTransactionSync    : {}", end.send_calls);
    eprintln!("send peak concurrency     : {}", end.send_peak);
    eprintln!("signer nonce consumed     : {}", end.nonce);
    eprintln!("add fail counter          : {}", end.add_fail);
    eprintln!("verify fail counter       : {}", end.vp_fail);
    eprintln!("csv                       : {}", cfg.out);
    eprintln!("==================================================");

    Ok(())
}
