//! Method-aware JSON-RPC fault proxy for the transaction-sender hotfix
//! validation (see `docs/minimal-v013-hotfix-plan.md`).
//!
//! Sits between a provider and anvil, counts calls per method, and can delay,
//! blackhole, or suppress the response of individual methods. This is what the
//! plan's "method-aware RPC fault proxy" refers to: the regression cases need to
//! stall `eth_getTransactionCount(pending)` while every other RPC stays healthy,
//! and to count `eth_estimateGas` per attempt.
//!
//! Deliberately dependency-free: built on `axum` and the `reqwest` re-exported
//! by alloy, both already in the crate's dependency graph, so `--locked` runs
//! and the release lockfile are unaffected.
//!
//! Transport note: production now uses HTTP/HTTPS. This proxy exercises the
//! same RPC transport class, but does not establish TLS, load-balancer or Nitro
//! behavior. Explicit WS tests remain library-only historical compatibility tests.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use alloy::transports::http::reqwest;
use axum::{body::Bytes, extract::State, routing::post, Router};
use serde_json::Value;

/// What the proxy does with a matching request.
#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// Close the HTTP response body with an I/O error without forwarding.
    DisconnectBeforeForward,
    /// Arbitrary body for testing echoed credentials and invalid JSON.
    RawResponse { status: u16, body: String },
    /// Fail before forwarding, leaving upstream chain state untouched.
    HttpError(u16),
    /// Return an RPC error before forwarding, with the caller's request ID.
    RpcError { code: i64, message: String },
    /// Let the caller's deadline expire, then discard this request permanently.
    /// Unlike Blackhole, clearing the fault never forwards an old request.
    DiscardAfter(Duration),
    /// Park the request and never answer it, until the fault is cleared (then
    /// the parked request is forwarded normally). Models a blackholed RPC.
    Blackhole,
    /// Forward, but only after this delay.
    Delay(Duration),
    /// Forward upstream so the call really executes, then discard the response.
    /// Models an accepted transaction whose success response is lost.
    SuppressResponse,
    /// Serve matching requests through a single service channel, each taking
    /// `service` time. Latency therefore grows with concurrency, which is how a
    /// single shared WS provider actually behaves: the incident's 0.05 s -> 5-6 s
    /// p50 "prep" inflation came from 128 concurrent `eth_estimateGas` calls
    /// queuing on one connection, not from a flat per-call delay. A flat delay
    /// rewards unbounded concurrency and so cannot reproduce the failure.
    Queue { service: Duration },
    /// Answer with this JSON-RPC `result` without forwarding upstream. Used to
    /// inject a stale `eth_getTransactionCount`, which the node would otherwise
    /// never return.
    RespondWith(Value),
    /// Forward upstream and wait for the real answer, then hold it for this long
    /// before replying. For `eth_sendRawTransactionSync` the upstream answer
    /// only arrives once the transaction is mined, so this guarantees the
    /// transaction is on-chain while the caller is still waiting - which is what
    /// makes the "cancelled sibling leaves on-chain work with no DB record"
    /// scenario deterministic rather than a race.
    DelayResponse(Duration),
}

#[derive(Clone, Debug)]
struct FaultSpec {
    fault: Fault,
    /// Remaining applications; `None` means unlimited.
    remaining: Option<usize>,
    proof_id_max: Option<u64>,
}

#[derive(Default, Debug)]
struct Stats {
    calls: HashMap<String, usize>,
    /// `eth_getTransactionCount` split by its block-tag parameter.
    tx_count_by_tag: HashMap<String, usize>,
    /// Observed handling latency per method, in milliseconds.
    latencies: HashMap<String, Vec<u64>>,
    /// Error strings seen in upstream *responses*, counted as events.
    /// A database `last_error` column is overwritten by the next error and
    /// erased by cleanup, so it cannot be used to count occurrences; this can.
    response_errors: HashMap<String, usize>,
    in_flight: HashMap<String, usize>,
    max_in_flight: HashMap<String, usize>,
}

#[derive(Default, Debug)]
struct Inner {
    faults: HashMap<String, FaultSpec>,
    stats: Stats,
}

/// Per-method single service channel, for `Fault::Queue`.
#[derive(Default)]
struct Queues {
    channels: HashMap<String, Arc<tokio::sync::Mutex<()>>>,
}

struct Shared {
    inner: Mutex<Inner>,
    upstream: String,
    client: reqwest::Client,
    queues: Mutex<Queues>,
}

pub struct FaultProxy {
    shared: Arc<Shared>,
    url: reqwest::Url,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}

/// Safety valve: a blackholed request is dropped rather than parked forever, so
/// a mistake in a test surfaces as a failure instead of a hang.
const MAX_PARK: Duration = Duration::from_secs(180);

impl FaultProxy {
    /// Starts a proxy in front of `upstream_http` (anvil's HTTP endpoint).
    pub async fn start(upstream_http: reqwest::Url) -> anyhow::Result<Self> {
        let shared = Arc::new(Shared {
            inner: Mutex::new(Inner::default()),
            queues: Mutex::new(Queues::default()),
            upstream: upstream_http.to_string(),
            client: reqwest::Client::builder()
                // The proxy must not impose its own deadline: the point is to
                // let the *client's* timeout fire.
                .timeout(Duration::from_secs(600))
                .build()?,
        });

        let app = Router::new()
            .route("/", post(handle))
            .fallback(handle)
            .with_state(shared.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let (tx, rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let _ = axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = rx.await;
                })
                .await;
        });

        Ok(Self {
            shared,
            url: reqwest::Url::parse(&format!("http://127.0.0.1:{port}/"))?,
            shutdown: Some(tx),
        })
    }

    pub fn url(&self) -> reqwest::Url {
        self.url.clone()
    }

    /// Applies `fault` to `method`. `remaining = None` means until cleared.
    pub fn set_fault(&self, method: &str, fault: Fault, remaining: Option<usize>) {
        self.shared.inner.lock().unwrap().faults.insert(
            method.to_string(),
            FaultSpec {
                fault,
                remaining,
                proof_id_max: None,
            },
        );
    }

    /// Restrict estimation faults to proof IDs in the earliest batch.
    pub fn set_early_proof_fault(&self, max_id: u64) {
        self.shared.inner.lock().unwrap().faults.insert(
            "eth_estimateGas".into(),
            FaultSpec {
                fault: Fault::HttpError(503),
                remaining: None,
                proof_id_max: Some(max_id),
            },
        );
    }

    pub fn clear_fault(&self, method: &str) {
        self.shared.inner.lock().unwrap().faults.remove(method);
    }

    pub fn clear_all_faults(&self) {
        self.shared.inner.lock().unwrap().faults.clear();
    }

    pub fn calls(&self, method: &str) -> usize {
        *self
            .shared
            .inner
            .lock()
            .unwrap()
            .stats
            .calls
            .get(method)
            .unwrap_or(&0)
    }

    /// `eth_getTransactionCount` calls made with the given block tag.
    pub fn tx_count_calls_with_tag(&self, tag: &str) -> usize {
        *self
            .shared
            .inner
            .lock()
            .unwrap()
            .stats
            .tx_count_by_tag
            .get(tag)
            .unwrap_or(&0)
    }

    /// Peak observed concurrency for a method, for the admission-limit check.
    pub fn max_concurrency(&self, method: &str) -> usize {
        *self
            .shared
            .inner
            .lock()
            .unwrap()
            .stats
            .max_in_flight
            .get(method)
            .unwrap_or(&0)
    }

    /// Observed latency percentile for a method, in milliseconds. This is the
    /// plan's "prep latency" proxy measurement: the time the sender waits for
    /// `eth_estimateGas`, measured at the proxy rather than from log brackets.
    pub fn latency_pct(&self, method: &str, pct: f64) -> u64 {
        let inner = self.shared.inner.lock().unwrap();
        let Some(v) = inner.stats.latencies.get(method) else {
            return 0;
        };
        if v.is_empty() {
            return 0;
        }
        let mut v = v.clone();
        v.sort_unstable();
        let idx = ((v.len() as f64 - 1.0) * pct).round() as usize;
        v[idx]
    }

    pub fn latency_count(&self, method: &str) -> usize {
        self.shared
            .inner
            .lock()
            .unwrap()
            .stats
            .latencies
            .get(method)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Number of upstream responses containing `needle` (case-insensitive).
    pub fn response_errors(&self, needle: &str) -> usize {
        *self
            .shared
            .inner
            .lock()
            .unwrap()
            .stats
            .response_errors
            .get(&needle.to_lowercase())
            .unwrap_or(&0)
    }

    /// Clears only the latency samples, so percentiles can be reported per
    /// phase instead of mixing a fault window with the recovery that follows.
    pub fn reset_latencies(&self) {
        self.shared.inner.lock().unwrap().stats.latencies.clear();
    }

    pub fn reset_stats(&self) {
        self.shared.inner.lock().unwrap().stats = Stats::default();
    }

    pub fn snapshot(&self) -> HashMap<String, usize> {
        self.shared.inner.lock().unwrap().stats.calls.clone()
    }
}

impl Drop for FaultProxy {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
    }
}

/// Pulls the method name out of a single or batched JSON-RPC body.
fn methods_of(body: &Value) -> Vec<String> {
    match body {
        Value::Array(items) => items
            .iter()
            .filter_map(|i| i.get("method").and_then(|m| m.as_str()).map(String::from))
            .collect(),
        other => other
            .get("method")
            .and_then(|m| m.as_str())
            .map(|m| vec![m.to_string()])
            .unwrap_or_default(),
    }
}

/// Block tag of an `eth_getTransactionCount` call, e.g. `pending` / `latest`.
fn tx_count_tag(body: &Value) -> Option<String> {
    let params = body.get("params")?.as_array()?;
    match params.get(1) {
        Some(Value::String(s)) => Some(s.clone()),
        Some(other) => Some(other.to_string()),
        // Omitted tag defaults to `latest` per the JSON-RPC spec.
        None => Some("latest".to_string()),
    }
}

fn record_start(shared: &Shared, methods: &[String], body: &Value) {
    let mut inner = shared.inner.lock().unwrap();
    for m in methods {
        *inner.stats.calls.entry(m.clone()).or_insert(0) += 1;
        let n = inner.stats.in_flight.entry(m.clone()).or_insert(0);
        *n += 1;
        let n = *n;
        let peak = inner.stats.max_in_flight.entry(m.clone()).or_insert(0);
        if n > *peak {
            *peak = n;
        }
        if m == "eth_getTransactionCount" {
            if let Some(tag) = tx_count_tag(body) {
                *inner.stats.tx_count_by_tag.entry(tag).or_insert(0) += 1;
            }
        }
    }
}

fn record_end(shared: &Shared, methods: &[String], elapsed_ms: u64) {
    let mut inner = shared.inner.lock().unwrap();
    for m in methods {
        if let Some(n) = inner.stats.in_flight.get_mut(m) {
            *n = n.saturating_sub(1);
        }
        inner
            .stats
            .latencies
            .entry(m.clone())
            .or_default()
            .push(elapsed_ms);
    }
}

/// Takes the fault to apply, consuming one unit of its budget.
fn take_fault(shared: &Shared, methods: &[String], body: &Value) -> Option<Fault> {
    let mut inner = shared.inner.lock().unwrap();
    for m in methods {
        let Some(spec) = inner.faults.get_mut(m) else {
            continue;
        };
        if let Some(max_id) = spec.proof_id_max {
            let tx = &body["params"][0];
            let data = tx
                .get("input")
                .or_else(|| tx.get("data"))
                .and_then(Value::as_str)
                .unwrap_or("");
            // ABI first argument follows the four-byte function selector.
            let id = data
                .get(10..74)
                .and_then(|word| u64::from_str_radix(word.trim_start_matches('0'), 16).ok());
            if !matches!(id, Some(id) if id > 0 && id <= max_id) {
                continue;
            }
        }
        match &mut spec.remaining {
            Some(0) => continue,
            Some(n) => *n -= 1,
            None => {}
        }
        return Some(spec.fault.clone());
    }
    None
}

/// True while `method` still has an active blackhole.
fn blackhole_active(shared: &Shared, methods: &[String]) -> bool {
    let inner = shared.inner.lock().unwrap();
    methods.iter().any(|m| {
        matches!(
            inner.faults.get(m),
            Some(FaultSpec {
                fault: Fault::Blackhole,
                ..
            })
        )
    })
}

async fn handle(State(shared): State<Arc<Shared>>, body: Bytes) -> axum::response::Response {
    let began = std::time::Instant::now();
    let parsed: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    let methods = methods_of(&parsed);
    record_start(&shared, &methods, &parsed);

    let fault = take_fault(&shared, &methods, &parsed);
    let mut suppress = false;
    let mut hold_response = None;

    match fault {
        Some(Fault::DisconnectBeforeForward) => {
            record_end(&shared, &methods, began.elapsed().as_millis() as u64);
            let stream = futures_util::stream::once(async {
                Err::<Bytes, std::io::Error>(std::io::Error::new(
                    std::io::ErrorKind::ConnectionReset,
                    "injected disconnect",
                ))
            });
            return axum::response::Response::new(axum::body::Body::from_stream(stream));
        }
        Some(Fault::RawResponse { status, mut body }) => {
            use axum::response::IntoResponse;
            if let Ok(mut response) = serde_json::from_str::<Value>(&body) {
                if response.get("jsonrpc").is_some() {
                    response["id"] = parsed.get("id").cloned().unwrap_or(Value::Null);
                    body = response.to_string();
                }
            }
            record_end(&shared, &methods, began.elapsed().as_millis() as u64);
            return (axum::http::StatusCode::from_u16(status).unwrap(), body).into_response();
        }
        Some(Fault::HttpError(status)) => {
            use axum::response::IntoResponse;
            record_end(&shared, &methods, began.elapsed().as_millis() as u64);
            return (
                axum::http::StatusCode::from_u16(status).unwrap(),
                "injected failure",
            )
                .into_response();
        }
        Some(Fault::RpcError { code, message }) => {
            use axum::response::IntoResponse;
            let body = serde_json::json!({
                "jsonrpc": "2.0", "id": parsed.get("id"),
                "error": { "code": code, "message": message },
            });
            record_end(&shared, &methods, began.elapsed().as_millis() as u64);
            return axum::Json(body).into_response();
        }
        Some(Fault::DiscardAfter(delay)) => {
            use axum::response::IntoResponse;
            tokio::time::sleep(delay).await;
            record_end(&shared, &methods, began.elapsed().as_millis() as u64);
            return axum::http::StatusCode::GATEWAY_TIMEOUT.into_response();
        }
        Some(Fault::Delay(d)) => tokio::time::sleep(d).await,
        Some(Fault::SuppressResponse) => suppress = true,
        Some(Fault::DelayResponse(d)) => hold_response = Some(d),
        Some(Fault::RespondWith(value)) => {
            use axum::response::IntoResponse;
            let id = parsed.get("id").cloned().unwrap_or(Value::Null);
            let body = serde_json::json!({"jsonrpc": "2.0", "id": id, "result": value});
            record_end(&shared, &methods, began.elapsed().as_millis() as u64);
            return (
                axum::http::StatusCode::OK,
                [("content-type", "application/json")],
                serde_json::to_vec(&body).unwrap_or_default(),
            )
                .into_response();
        }
        Some(Fault::Queue { service }) => {
            // One service channel per method: concurrent callers queue, so the
            // observed latency is proportional to how many are in flight.
            let channel = {
                let mut q = shared.queues.lock().unwrap();
                q.channels
                    .entry(methods.first().cloned().unwrap_or_default())
                    .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
                    .clone()
            };
            let _slot = channel.lock().await;
            tokio::time::sleep(service).await;
        }
        Some(Fault::Blackhole) => {
            // Park until the fault is lifted, then fall through and forward.
            let deadline = tokio::time::Instant::now() + MAX_PARK;
            while blackhole_active(&shared, &methods) {
                if tokio::time::Instant::now() >= deadline {
                    record_end(&shared, &methods, began.elapsed().as_millis() as u64);
                    // Give up rather than hang the test forever.
                    return (axum::http::StatusCode::GATEWAY_TIMEOUT, "proxy park cap")
                        .into_response();
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        }
        None => {}
    }

    let upstream = shared
        .client
        .post(&shared.upstream)
        .header("content-type", "application/json")
        .body(body.clone())
        .send()
        .await;

    record_end(&shared, &methods, began.elapsed().as_millis() as u64);

    use axum::response::IntoResponse;
    match upstream {
        Ok(resp) => {
            let status = resp.status();
            let bytes = resp.bytes().await.unwrap_or_default();
            record_response_errors(&shared, &bytes);
            if suppress {
                // The call executed upstream; the caller never learns the result.
                return futures_pending().await;
            }
            if let Some(d) = hold_response {
                // Upstream already applied the call; only the answer is late.
                tokio::time::sleep(d).await;
            }
            (
                axum::http::StatusCode::from_u16(status.as_u16())
                    .unwrap_or(axum::http::StatusCode::OK),
                [("content-type", "application/json")],
                bytes,
            )
                .into_response()
        }
        Err(e) => (
            axum::http::StatusCode::BAD_GATEWAY,
            format!("proxy upstream error: {e}"),
        )
            .into_response(),
    }
}

/// Error strings worth counting as events when they appear in a response.
const COUNTED_RESPONSE_ERRORS: [&str; 5] = [
    "nonce too low",
    "nonce too high",
    "already known",
    "replacement transaction underpriced",
    "insufficient funds",
];

fn record_response_errors(shared: &Shared, body: &[u8]) {
    let text = String::from_utf8_lossy(body).to_lowercase();
    if !text.contains("error") {
        return;
    }
    let mut inner = shared.inner.lock().unwrap();
    for needle in COUNTED_RESPONSE_ERRORS {
        if text.contains(needle) {
            *inner
                .stats
                .response_errors
                .entry(needle.to_string())
                .or_insert(0) += 1;
        }
    }
}

/// Never resolves: the client must hit its own timeout.
async fn futures_pending() -> axum::response::Response {
    tokio::time::sleep(MAX_PARK).await;
    use axum::response::IntoResponse;
    (
        axum::http::StatusCode::GATEWAY_TIMEOUT,
        "proxy suppressed response",
    )
        .into_response()
}
