//! Baseline (pre-hotfix v0.13.4) submission driver against the real Gateway.
//!
//! This measures the shape the *unchanged* v0.13.4 sender actually runs, with
//! only the transport moved from WebSocket to HTTP:
//!
//! * a batch of up to `batch_limit` rows is selected,
//! * one task is spawned per row,
//! * each task estimates gas, takes the nonce mutex briefly to allocate a
//!   nonce, releases it, then calls `eth_sendRawTransactionSync`,
//! * the whole batch is joined before the next batch is selected.
//!
//! That join barrier is part of the design and is reproduced here: a single
//! slow submission holds up the next batch. Two operations (verify-proof and
//! add-ciphertexts) run as independent loops, so production concurrency is
//! `batch_limit x operations`, not an unbounded pool.
//!
//! Ignored by default; driven from an env file so no secret reaches `argv`,
//! shell history or this test's output. Only the derived signer address, error
//! classes and sanitized endpoint (`scheme://host/<redacted>`) are ever logged.
//!
//! ```sh
//! GW_ENV_FILE=~/.config/fhevm-gw-test.env \
//!   cargo test --release --locked -p transaction-sender \
//!   --test gw_baseline_capacity -- --ignored --nocapture
//! ```

#![cfg(test)]

use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use transaction_sender::{
    gateway_http_client, FillersWithoutNonceManagement, NonceManagedProvider,
};

/// Which transport the measured provider uses.
/// `https` builds exactly what the release binary builds: `gateway_http_client`
/// (4 s connect, 30 s request, no redirects, no transport retry) driven through
/// `connect_reqwest`. `wss` reconstructs the pre-migration provider for the
/// side-by-side comparison, since the saved WSS build is not available here.
#[derive(Clone, Copy, PartialEq)]
enum Transport {
    Https,
    Wss,
}

impl Transport {
    /// Strict: an unrecognized value panics rather than silently defaulting.
    /// A silent default here mislabelled a whole arm of an earlier comparison.
    fn from_env() -> Self {
        let raw = std::env::var("GW_TRANSPORT").unwrap_or_else(|_| "https".into());
        match raw.to_ascii_lowercase().as_str() {
            "wss" | "ws" => Transport::Wss,
            "https" | "http" => Transport::Https,
            other => panic!("GW_TRANSPORT={other:?} is not one of https|wss"),
        }
    }
    fn label(&self) -> &'static str {
        match self {
            Transport::Https => "https",
            Transport::Wss => "wss",
        }
    }
}

// ---------------------------------------------------------------- config ---

struct Secrets {
    wss: String,
    https: Option<String>,
    /// Every (address, key) pair in the file, in order. The first is the
    /// primary account used by single-account runs.
    accounts: Vec<(Address, String)>,
}

impl Secrets {
    /// URL for the transport under test.
    fn url_for(&self, t: Transport) -> anyhow::Result<String> {
        Ok(match t {
            Transport::Wss => self.wss.clone(),
            Transport::Https => self
                .https
                .clone()
                .ok_or_else(|| anyhow::anyhow!("no http(s) Gateway URL available"))?,
        })
    }
}

/// Parses the free-form `label: value` env file. Values are never printed.
fn load_secrets() -> anyhow::Result<Secrets> {
    let path = std::env::var("GW_ENV_FILE").unwrap_or_else(|_| {
        format!(
            "{}/.config/fhevm-gw-test.env",
            std::env::var("HOME").unwrap_or_default()
        )
    });
    let raw = std::fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("cannot read GW_ENV_FILE {path}: {e}"))?;

    let mut wss = None;
    let mut https = None;
    // Address/Pkey lines come in pairs, possibly repeated for extra accounts.
    let mut pending_address: Option<Address> = None;
    let mut accounts: Vec<(Address, String)> = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // A bare URL line, with no "Label:" prefix.
        if line.starts_with("wss://") || line.starts_with("ws://") {
            wss = Some(line.to_string());
            continue;
        }
        if line.starts_with("https://") || line.starts_with("http://") {
            https = Some(line.to_string());
            continue;
        }
        let Some(idx) = line.find([':', '=']) else {
            continue;
        };
        let (label, value) = line.split_at(idx);
        let value = value[1..].trim();
        if value.is_empty() {
            continue;
        }
        match label.trim().to_ascii_lowercase().replace(' ', "_").as_str() {
            "https" | "gw_https" => https = Some(value.to_string()),
            "wss" | "gw_wss" | "url" => {
                // Tolerate "WSS: wss://..." and a value that lost its scheme
                // because the split landed inside "wss://".
                wss = Some(if value.starts_with("//") {
                    format!("wss:{value}")
                } else {
                    value.to_string()
                });
            }
            "address" | "gw_address" => pending_address = Some(Address::from_str(value)?),
            "pkey" | "gw_pkey" | "private_key" => {
                let a = pending_address
                    .take()
                    .ok_or_else(|| anyhow::anyhow!("a Pkey line has no preceding Address line"))?;
                accounts.push((a, value.to_string()));
            }
            _ => {}
        }
    }
    anyhow::ensure!(
        !accounts.is_empty(),
        "env file defines no Address/Pkey pair"
    );
    Ok(Secrets {
        wss: wss
            .clone()
            .ok_or_else(|| anyhow::anyhow!("env file has no endpoint URL"))?,
        // If infra supplied no explicit HTTPS URL, fall back to substituting the
        // scheme. The strategy document warns not to assume this works, so the
        // driver probes it at startup and records that it was derived, not given.
        https: https.or_else(|| wss.map(|u| u.replacen("wss://", "https://", 1))),
        accounts,
    })
}

/// Endpoint identity safe to log: scheme and host only, never the path, which
/// is where Conduit carries its auth token.
fn redacted_endpoint(_url: &str) -> String {
    "<Gateway endpoint redacted>".to_string()
}

#[allow(dead_code)]
fn old_redacted_endpoint(url: &str) -> String {
    match url.split_once("://") {
        Some((scheme, rest)) => {
            let host = rest.split(['/', '?']).next().unwrap_or("");
            format!("{scheme}://{host}/<redacted>")
        }
        None => "<unparsable>".to_string(),
    }
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Hard stops. Any breach halts new work; it never silently continues.
struct Ceilings {
    max_attempts: u64,
    max_spend_wei: u128,
    max_unresolved: u64,
    max_secs: u64,
    tripped: AtomicBool,
    reason: std::sync::Mutex<Option<String>>,
}

impl Ceilings {
    fn from_env() -> Self {
        Self {
            max_attempts: env_u64("GW_MAX_ATTEMPTS", 200),
            // 0.02 ETH default for the pilot: a tenth of the funded budget.
            max_spend_wei: std::env::var("GW_MAX_SPEND_WEI")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(20_000_000_000_000_000u128),
            max_unresolved: env_u64("GW_MAX_UNRESOLVED", 32),
            max_secs: env_u64("GW_MAX_SECS", 600),
            tripped: AtomicBool::new(false),
            reason: std::sync::Mutex::new(None),
        }
    }

    fn trip(&self, why: impl Into<String>) {
        if !self.tripped.swap(true, Ordering::SeqCst) {
            *self.reason.lock().unwrap() = Some(why.into());
        }
    }

    fn is_tripped(&self) -> bool {
        self.tripped.load(Ordering::SeqCst)
    }
}

/// Includes confirmed fees plus maximum reservations for outstanding or
/// ambiguous sends. Only a mined receipt can refund a submission reservation.
struct SpendBudget {
    charged: std::sync::Mutex<u128>,
    limit: u128,
    per_send: u128,
}
impl SpendBudget {
    fn reserve(&self, count: u64) -> bool {
        let mut charged = self.charged.lock().unwrap();
        let Some(next) = self
            .per_send
            .checked_mul(count as u128)
            .and_then(|amount| charged.checked_add(amount))
        else {
            return false;
        };
        if next > self.limit {
            return false;
        }
        *charged = next;
        true
    }
    fn settle(&self, actual: u128) {
        assert!(
            actual <= self.per_send,
            "receipt exceeded maximum gas reservation"
        );
        *self.charged.lock().unwrap() -= self.per_send - actual;
    }
}

#[test]
fn reservation_refunds_only_confirmed_costs() {
    let b = SpendBudget {
        charged: std::sync::Mutex::new(0),
        limit: 100,
        per_send: 40,
    };
    assert!(b.reserve(2));
    assert!(!b.reserve(1));
    b.settle(10);
    assert!(b.reserve(1));
    // One ambiguous send stays reserved at 40; only confirmed work is refunded.
    assert_eq!(*b.charged.lock().unwrap(), 90);
    b.settle(20);
    assert_eq!(*b.charged.lock().unwrap(), 70);
    assert!(!b.reserve(1));
}

fn pct(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx]
}

fn summarize(label: &str, mut v: Vec<f64>) {
    if v.is_empty() {
        println!("  {label:<26}: no samples");
        return;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    println!(
        "  {label:<26}: n={:<5} mean={:>8.1} p50={:>8.1} p90={:>8.1} p99={:>8.1} max={:>8.1} ms",
        v.len(),
        mean,
        pct(&v, 0.50),
        pct(&v, 0.90),
        pct(&v, 0.99),
        v[v.len() - 1]
    );
}

/// Builds the provider under test.
///
/// `https` builds exactly what this branch's binary builds: `gateway_http_client`
/// (4 s connect, 30 s request ceiling, no redirects, no transport retry) driven
/// through `connect_reqwest`. `wss` reconstructs the pre-migration provider so
/// the transport change can be attributed.
async fn build_provider(
    t: Transport,
    url: &str,
    wallet: EthereumWallet,
    signer_addr: Address,
) -> anyhow::Result<NonceManagedProvider<alloy::providers::DynProvider>> {
    let base = ProviderBuilder::default()
        .filler(FillersWithoutNonceManagement::default())
        .wallet(wallet);
    let inner = match t {
        Transport::Https => {
            let parsed: alloy::transports::http::reqwest::Url = url.parse()?;
            base.connect_reqwest(gateway_http_client(&parsed)?, parsed)
                .erased()
        }
        Transport::Wss => base
            .connect_ws(WsConnect::new(url.to_string()))
            .await?
            .erased(),
    };
    Ok(NonceManagedProvider::new(inner, Some(signer_addr)))
}

/// Outcomes kept apart rather than lumped into an error count. A lost response
/// is an ambiguous acceptance, not a rejection, and the two must not be summed.
#[derive(Default)]
struct Outcomes {
    ok: AtomicU64,
    reverted: AtomicU64,
    send_timeout: AtomicU64,
    nonce_low: AtomicU64,
    nonce_high: AtomicU64,
    already_known: AtomicU64,
    null_response: AtomicU64,
    underpriced: AtomicU64,
    other: AtomicU64,
    estimate_failed: AtomicU64,
    samples: std::sync::Mutex<Vec<String>>,
}

impl Outcomes {
    fn classify(&self, e: &str) {
        let l = e.to_ascii_lowercase();
        if l.contains("eth_sendrawtransactionsync timeout") || l.contains("sync timeout") {
            self.send_timeout.fetch_add(1, Ordering::Relaxed);
        } else if l.contains("nonce too low") {
            self.nonce_low.fetch_add(1, Ordering::Relaxed);
        } else if l.contains("nonce too high") || l.contains("nonce gap") {
            self.nonce_high.fetch_add(1, Ordering::Relaxed);
        } else if l.contains("already known") || l.contains("already imported") {
            self.already_known.fetch_add(1, Ordering::Relaxed);
        } else if l.contains("null response") {
            self.null_response.fetch_add(1, Ordering::Relaxed);
        } else if l.contains("underpriced") {
            self.underpriced.fetch_add(1, Ordering::Relaxed);
        } else {
            self.other.fetch_add(1, Ordering::Relaxed);
        }
        let mut v = self.samples.lock().unwrap();
        if v.len() < 10 && !v.iter().any(|s| s == e) {
            v.push(e.to_string());
        }
    }

    fn total_errors(&self) -> u64 {
        self.send_timeout.load(Ordering::Relaxed)
            + self.nonce_low.load(Ordering::Relaxed)
            + self.nonce_high.load(Ordering::Relaxed)
            + self.already_known.load(Ordering::Relaxed)
            + self.null_response.load(Ordering::Relaxed)
            + self.underpriced.load(Ordering::Relaxed)
            + self.other.load(Ordering::Relaxed)
            + self.estimate_failed.load(Ordering::Relaxed)
    }

    fn report(&self) {
        println!(
            "  outcomes: ok={} reverted={} | send_timeout={} nonce_low={} nonce_high={} \
already_known={} null_response={} underpriced={} estimate_failed={} other={}",
            self.ok.load(Ordering::Relaxed),
            self.reverted.load(Ordering::Relaxed),
            self.send_timeout.load(Ordering::Relaxed),
            self.nonce_low.load(Ordering::Relaxed),
            self.nonce_high.load(Ordering::Relaxed),
            self.already_known.load(Ordering::Relaxed),
            self.null_response.load(Ordering::Relaxed),
            self.underpriced.load(Ordering::Relaxed),
            self.estimate_failed.load(Ordering::Relaxed),
            self.other.load(Ordering::Relaxed),
        );
        for m in self.samples.lock().unwrap().iter() {
            let _ = m; // Raw upstream error samples deliberately withheld.
        }
    }
}

struct Samples {
    tx_ms: std::sync::Mutex<Vec<f64>>,
    estimate_ms: std::sync::Mutex<Vec<f64>>,
    batch_ms: std::sync::Mutex<Vec<f64>>,
}

impl Samples {
    fn new() -> Self {
        Self {
            tx_ms: std::sync::Mutex::new(Vec::new()),
            estimate_ms: std::sync::Mutex::new(Vec::new()),
            batch_ms: std::sync::Mutex::new(Vec::new()),
        }
    }
    fn clear(&self) {
        self.tx_ms.lock().unwrap().clear();
        self.estimate_ms.lock().unwrap().clear();
        self.batch_ms.lock().unwrap().clear();
    }
}

/// One transaction the way the baseline operation code sends one: estimate the
/// gas limit, overprovision it, then a single bounded `send_transaction_sync`.
#[allow(clippy::too_many_arguments)]
async fn send_one(
    provider: &NonceManagedProvider<alloy::providers::DynProvider>,
    tx: TransactionRequest,
    estimate: bool,
    overprovision_pct: u32,
    send_timeout: Duration,
    outcomes: &Outcomes,
    samples: &Samples,
    measuring: &std::sync::atomic::AtomicBool,
    budget: &SpendBudget,
) {
    let t0 = Instant::now();
    let prepared = if estimate {
        match provider
            .overprovision_gas_limit(tx.clone(), overprovision_pct)
            .await
        {
            Ok(t) => t,
            Err(e) => {
                budget.settle(0); // No submission was attempted.
                outcomes.estimate_failed.fetch_add(1, Ordering::Relaxed);
                outcomes.classify(&e.to_string());
                return;
            }
        }
    } else {
        tx
    };
    // The budget assumes a bounded gas limit. Do not submit if estimation
    // violates that assumption (e.g. a recipient unexpectedly acquired code).
    if prepared.gas.unwrap_or(u64::MAX) > 25_200 {
        budget.settle(0);
        outcomes.estimate_failed.fetch_add(1, Ordering::Relaxed);
        return;
    }
    let t_est = Instant::now();
    let res = provider.send_transaction_sync(prepared, send_timeout).await;
    let done = Instant::now();
    if measuring.load(Ordering::Relaxed) {
        samples
            .estimate_ms
            .lock()
            .unwrap()
            .push((t_est - t0).as_secs_f64() * 1000.0);
        samples
            .tx_ms
            .lock()
            .unwrap()
            .push((done - t0).as_secs_f64() * 1000.0);
    }
    match res {
        Ok(receipt) => {
            budget.settle(receipt.effective_gas_price * u128::from(receipt.gas_used));
            if receipt.status() {
                outcomes.ok.fetch_add(1, Ordering::Relaxed);
            } else {
                outcomes.reverted.fetch_add(1, Ordering::Relaxed);
            }
        }
        Err(e) => outcomes.classify(&e.to_string()),
    }
}

/// Full evaluation of the baseline submission shape against the Gateway.
///
/// `GW_MODE=batch` (default) reproduces the operation loop with its join
/// barrier; `GW_MODE=pool` runs a continuous worker pool of the same width, so
/// the barrier's cost can be separated from the concurrency's benefit.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[ignore]
async fn gw_baseline_sweep() -> Result<(), String> {
    run_campaign()
        .await
        .map_err(|_| "Gateway campaign failed; raw error withheld".to_string())
}

async fn run_campaign() -> anyhow::Result<()> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let s = load_secrets()?;
    let transport = Transport::from_env();
    let url = s.url_for(transport)?;
    let ceilings = Arc::new(Ceilings::from_env());

    let batch_sweep: Vec<usize> = std::env::var("GW_SWEEP")
        .unwrap_or_else(|_| "10".into())
        .split(',')
        .filter_map(|v| v.trim().parse().ok())
        .collect();
    anyhow::ensure!(
        batch_sweep.len() == 1,
        "one batch configuration per process required"
    );
    let ops = env_u64("GW_OPS", 2) as usize;
    let warmup = env_u64("GW_WARMUP", 20);
    let measure = env_u64("GW_MEASURE", 60);
    let n_accounts = (env_u64("GW_ACCOUNTS", 1) as usize).min(s.accounts.len());
    let acct_offset = (env_u64("GW_ACCOUNT_OFFSET", 0) as usize).min(s.accounts.len() - 1);
    let send_timeout = Duration::from_millis(env_u64("GW_SEND_TIMEOUT_MS", 4000));
    let estimate = env_u64("GW_ESTIMATE", 1) == 1;
    let overprovision_pct = env_u64("GW_OVERPROVISION_PCT", 120) as u32;
    anyhow::ensure!(
        estimate && overprovision_pct == 120,
        "campaign requires estimation and 120 percent gas limit"
    );
    let pool_mode = std::env::var("GW_MODE").unwrap_or_else(|_| "batch".into()) == "pool";

    println!("======= v0.13.4 baseline submission, HTTP transport =======");
    println!("transport   : {}", transport.label());
    println!("endpoint    : {}", redacted_endpoint(&url));
    println!(
        "mode        : {}",
        if pool_mode {
            "pool (no join barrier)"
        } else {
            "batch (join barrier, as the operation loop runs)"
        }
    );
    println!("batch sweep : {batch_sweep:?} per operation x {ops} operation(s)");
    println!("send timeout: {send_timeout:?}   gas estimation: {estimate}");
    println!(
        "accounts    : {n_accounts} (offset {acct_offset}), warm {warmup}s, measure {measure}s"
    );

    let probe: alloy::providers::DynProvider = match transport {
        Transport::Https => {
            let parsed: alloy::transports::http::reqwest::Url = url.parse()?;
            ProviderBuilder::new()
                .connect_reqwest(gateway_http_client(&parsed)?, parsed)
                .erased()
        }
        Transport::Wss => ProviderBuilder::new()
            .connect_ws(WsConnect::new(url.clone()))
            .await?
            .erased(),
    };
    let t_cold = Instant::now();
    let chain_id = probe.get_chain_id().await?;
    println!(
        "cold setup  : {:.1} ms (first request incl. connect/TLS), chain {chain_id}",
        t_cold.elapsed().as_secs_f64() * 1000.0
    );

    let mut wallets = Vec::new();
    for (addr, pk) in s.accounts.iter().skip(acct_offset).take(n_accounts) {
        let mut sg = PrivateKeySigner::from_str(pk.trim())?;
        sg.set_chain_id(Some(chain_id));
        anyhow::ensure!(sg.address() == *addr, "key/address mismatch for {addr}");
        let latest = probe.get_transaction_count(*addr).await?;
        let pending = probe.get_transaction_count(*addr).pending().await?;
        anyhow::ensure!(
            latest == pending,
            "account {addr} has {} unmined transaction(s); reconcile first",
            pending - latest
        );
        println!("  account {addr}  nonce {latest}");
        wallets.push((*addr, EthereumWallet::from(sg)));
    }
    let recipient = Address::from_str(&std::env::var("GW_RECIPIENT")?)?;
    let mut start_nonces = Vec::new();
    let mut start_balances = Vec::new();
    for (addr, _) in &wallets {
        start_nonces.push(probe.get_transaction_count(*addr).await?);
        start_balances.push(probe.get_balance(*addr).await?);
    }

    let attempts = Arc::new(AtomicU64::new(0));
    let mut rows = Vec::new();
    for &batch in &batch_sweep {
        if ceilings.is_tripped() {
            break;
        }
        let outcomes = Arc::new(Outcomes::default());
        let samples = Arc::new(Samples::new());
        let measuring = Arc::new(AtomicBool::new(false));
        let stop = Arc::new(AtomicBool::new(false));
        let done_count = Arc::new(AtomicU64::new(0));

        let mut providers = Vec::new();
        for (addr, w) in &wallets {
            providers.push(build_provider(transport, &url, w.clone(), *addr).await?);
        }
        let fees = probe.estimate_eip1559_fees().await?;
        // Every transaction is a zero-value transfer to an EOA; reserve at the
        // maximum fee and overprovisioned gas limit before launching a batch.
        anyhow::ensure!(
            probe.get_code_at(recipient).await?.is_empty(),
            "recipient must be EOA"
        );
        let worst_cost = fees.max_fee_per_gas.saturating_mul(25_200);
        anyhow::ensure!(worst_cost > 0, "invalid fee estimate");
        let budget = Arc::new(SpendBudget {
            charged: std::sync::Mutex::new(0),
            limit: ceilings.max_spend_wei,
            per_send: worst_cost,
        });
        let allowed = ceilings.max_attempts;
        anyhow::ensure!(
            batch * ops * n_accounts <= ceilings.max_unresolved as usize,
            "concurrency ceiling"
        );

        let mut loops = tokio::task::JoinSet::new();
        for provider in providers.iter() {
            for _op in 0..ops {
                let provider = provider.clone();
                let outcomes = outcomes.clone();
                let samples = samples.clone();
                let measuring = measuring.clone();
                let stop = stop.clone();
                let ceilings = ceilings.clone();
                let done_count = done_count.clone();
                let attempts = attempts.clone();
                let budget = budget.clone();
                let tx = TransactionRequest::default()
                    .with_to(recipient)
                    .with_value(U256::ZERO)
                    .with_chain_id(chain_id)
                    .with_max_fee_per_gas(fees.max_fee_per_gas)
                    .with_max_priority_fee_per_gas(fees.max_priority_fee_per_gas);
                loops.spawn(async move {
                    while !stop.load(Ordering::Relaxed) && !ceilings.is_tripped() {
                        let width = if pool_mode { 1 } else { batch as u64 };
                        if attempts
                            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
                                n.checked_add(width).filter(|v| *v <= allowed)
                            })
                            .is_err()
                        {
                            ceilings.trip("attempt or reserved-spend ceiling");
                            break;
                        }
                        if !budget.reserve(width) {
                            ceilings.trip(
                                "spend ceiling (confirmed fees plus outstanding reservations)",
                            );
                            break;
                        }
                        let t_batch = Instant::now();
                        if pool_mode {
                            // No barrier: each worker loops independently.
                            send_one(
                                &provider,
                                tx.clone(),
                                estimate,
                                overprovision_pct,
                                send_timeout,
                                &outcomes,
                                &samples,
                                &measuring,
                                &budget,
                            )
                            .await;
                            done_count.fetch_add(1, Ordering::Relaxed);
                        } else {
                            let mut set = tokio::task::JoinSet::new();
                            for _ in 0..batch {
                                let provider = provider.clone();
                                let outcomes = outcomes.clone();
                                let samples = samples.clone();
                                let measuring = measuring.clone();
                                let tx = tx.clone();
                                let budget = budget.clone();
                                set.spawn(async move {
                                    send_one(
                                        &provider,
                                        tx,
                                        estimate,
                                        overprovision_pct,
                                        send_timeout,
                                        &outcomes,
                                        &samples,
                                        &measuring,
                                        &budget,
                                    )
                                    .await;
                                });
                            }
                            // The join barrier the operation loop imposes.
                            while set.join_next().await.is_some() {}
                            done_count.fetch_add(batch as u64, Ordering::Relaxed);
                            if measuring.load(Ordering::Relaxed) {
                                samples
                                    .batch_ms
                                    .lock()
                                    .unwrap()
                                    .push(t_batch.elapsed().as_secs_f64() * 1000.0);
                            }
                        }
                    }
                });
            }
        }

        tokio::time::sleep(Duration::from_secs(warmup)).await;
        samples.clear();
        let base_done = done_count.load(Ordering::Relaxed);
        let base_ok = outcomes.ok.load(Ordering::Relaxed);
        let m_start = Instant::now();
        measuring.store(true, Ordering::Relaxed);
        tokio::time::sleep(Duration::from_secs(measure)).await;
        measuring.store(false, Ordering::Relaxed);
        let measured_ok = outcomes.ok.load(Ordering::Relaxed) - base_ok;
        let elapsed = m_start.elapsed().as_secs_f64();
        let completed = done_count.load(Ordering::Relaxed) - base_done;
        stop.store(true, Ordering::Relaxed);
        while let Some(result) = loops.join_next().await {
            result?;
        }

        let tx_ms = samples.tx_ms.lock().unwrap().clone();
        let n_ok = outcomes.ok.load(Ordering::Relaxed);
        let rate = measured_ok as f64 / elapsed;
        println!("measured successful receipts: {measured_ok}; goodput {rate:.3} tx/s");
        println!(
            "\nbatch {batch} x {ops} op(s) x {n_accounts} account(s) = {} concurrent",
            batch * ops * n_accounts
        );
        println!(
            "  completed {completed} attempts in {elapsed:.1}s  ->  {rate:.2} attempts/s \
(sampled {} in-window)",
            tx_ms.len()
        );
        summarize("  per-tx total", tx_ms.clone());
        summarize(
            "  gas estimate",
            samples.estimate_ms.lock().unwrap().clone(),
        );
        if !pool_mode {
            summarize(
                "  batch wall time",
                samples.batch_ms.lock().unwrap().clone(),
            );
        }
        outcomes.report();

        let mut sorted = tx_ms.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        rows.push(format!(
            "{batch},{ops},{n_accounts},{},{:.3},{:.1},{:.1},{:.1},{},{},{}",
            tx_ms.len(),
            rate,
            pct(&sorted, 0.50),
            pct(&sorted, 0.90),
            pct(&sorted, 0.99),
            n_ok,
            outcomes.total_errors(),
            outcomes.send_timeout.load(Ordering::Relaxed),
        ));

        // Let everything in flight settle before reading the chain.
        tokio::time::sleep(Duration::from_secs(8)).await;
        let mut spent_total = U256::ZERO;
        for (i, (addr, _)) in wallets.iter().enumerate() {
            let latest = probe.get_transaction_count(*addr).await?;
            let pending = probe.get_transaction_count(*addr).pending().await?;
            anyhow::ensure!(latest == pending, "unmined work at rest; stop campaign");
            let bal = probe.get_balance(*addr).await?;
            spent_total += start_balances[i].saturating_sub(bal);
            println!(
                "  {addr}: nonce {} -> {latest} (pending {pending}, unmined {})",
                start_nonces[i],
                pending.saturating_sub(latest)
            );
        }
        if spent_total > U256::from(ceilings.max_spend_wei) {
            ceilings.trip(format!("spend ceiling: {spent_total} wei"));
        }
        if m_start.elapsed().as_secs() > ceilings.max_secs {
            ceilings.trip("time ceiling");
        }
    }

    println!(
        "\nbatch,ops,accounts,samples,rate_per_s,p50_ms,p90_ms,p99_ms,ok,errors,send_timeouts"
    );
    for r in &rows {
        println!("{r}");
    }
    if let Some(why) = ceilings.reason.lock().unwrap().as_ref() {
        println!("CEILING TRIPPED: {why}");
        anyhow::bail!("campaign ceiling tripped");
    }
    Ok(())
}
