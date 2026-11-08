//! Runs every `scenarios/*.yaml` file against the aggregator with the mock connector, under paused tokio time.
//! Adding a scenario = adding a file; `cargo test yaml_scenarios` runs them all and lists every failure.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use kms_connector_api::{ErrorCode, PublicDecryptionRequest, UserDecryptionRequest};
use serde::Deserialize;
use tokio::time::Instant;

use super::aggregator::{AggregationError, Aggregator, Tally};
use super::config::{CallConfig, RateLimitConfig, RetryConfig, UserChecks};
use super::flows::Flow;
use super::flows::public_decrypt::PublicDecrypt;
use super::flows::user_decrypt::UserDecrypt;
use super::mock::{MockClient, PUBLIC_REQUEST_JSON, Reply, USER_REQUEST_JSON};
use crate::metrics::Metrics;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Scenario {
    flow: ScenarioFlow,
    threshold: usize,
    #[serde(with = "humantime_serde")]
    timeout: Duration,
    /// The transient retry budget (`kms_aggregator.call.retries`). Absent: a single attempt; present: the config's
    /// defaults for the keys left out (`retries: {}` is the default list of waits).
    #[serde(default = "no_transient_retries")]
    retries: RetryConfig,
    /// The rate-limit retry budget (`kms_aggregator.call.rate_limit`). Absent: a single attempt; present: the
    /// config's defaults for the keys left out (`rate_limit: {}` is one retry).
    #[serde(default = "no_rate_limit_retries")]
    rate_limit: RateLimitConfig,
    /// How long the scripted node takes to answer each attempt (test only).
    #[serde(with = "humantime_serde", default = "default_attempt_latency")]
    attempt_latency: Duration,
    /// Node groups in node order; the total is the number of nodes.
    nodes: Vec<Group>,
    /// Optional user-decrypt checks (`decryption_id_match`, `decryption_id_majority`).
    checks: Option<UserChecks>,
    expect: Expect,
}

fn no_transient_retries() -> RetryConfig {
    RetryConfig { waits: Vec::new() }
}

fn no_rate_limit_retries() -> RateLimitConfig {
    RateLimitConfig {
        max_retries: 0,
        ..RateLimitConfig::default()
    }
}

fn default_attempt_latency() -> Duration {
    Duration::from_millis(10)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ScenarioFlow {
    UserDecrypt,
    PublicDecrypt,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Group {
    count: usize,
    /// One reply per attempt; the last one repeats.
    replies: Vec<Reply>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expect {
    outcome: Outcome,
    /// Responses counting toward the threshold at the end.
    counted: Option<usize>,
    /// Exact virtual duration of the aggregation.
    #[serde(with = "humantime_serde", default)]
    elapsed: Option<Duration>,
    /// Most frequent connector error among the failed calls (failures only).
    dominant: Option<ErrorCode>,
    /// HTTP attempts across all nodes.
    attempts: Option<u32>,
    /// Shares (user decrypt) or signatures (public decrypt) in the answer.
    responses: Option<usize>,
    /// What the per-node metrics recorded, summed over the nodes.
    metrics: Option<ExpectMetrics>,
}

/// `answered`: observations of `relayer_http_kms_attempt_duration_seconds` (attempts that got an answer or were
/// cut). `errors`: `relayer_http_kms_attempt_errors_total` per `code`; a code left out must be zero.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectMetrics {
    answered: u64,
    #[serde(default)]
    errors: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Outcome {
    Ok,
    Timeout,
    ThresholdNotReached,
}

/// What one run produced, flattened for comparison with `Expect`.
struct Observed {
    outcome: Outcome,
    counted: usize,
    elapsed: Duration,
    dominant: Option<ErrorCode>,
    attempts: u32,
    responses: Option<usize>,
    /// Attempts that got an answer or were cut, from the metrics.
    answered: u64,
    /// Failed attempts per code, from the metrics.
    errors: BTreeMap<String, u64>,
    /// Label values outside their enumerated set, and broken metric invariants.
    metric_errors: Vec<String>,
}

fn load(path: &Path) -> Result<Scenario, String> {
    config::Config::builder()
        .add_source(config::File::from(path))
        .build()
        .and_then(config::Config::try_deserialize)
        .map_err(|e| format!("invalid scenario: {e}"))
}

async fn run<F: Flow>(
    scenario: &Scenario,
    request: F::Request,
    checks: F::Checks,
    responses: fn(&F::Output) -> usize,
) -> Observed {
    let scripts: Vec<Vec<Reply>> = scenario
        .nodes
        .iter()
        .flat_map(|g| std::iter::repeat_n(g.replies.clone(), g.count))
        .collect();
    let mock = MockClient::new(scripts.clone(), scenario.attempt_latency);
    let caller = mock.clone().caller_with_call(CallConfig {
        timeout: scenario.timeout,
        retries: scenario.retries.clone(),
        rate_limit: scenario.rate_limit.clone(),
    });
    let metrics = caller.metrics().clone();
    let aggregator = Aggregator::<F>::new(caller, scenario.threshold, checks);
    let started = Instant::now();
    let result = aggregator
        .run("scenario", request, &mut Tally::default())
        .await;
    let elapsed = started.elapsed();
    let attempts = (0..scripts.len()).map(|n| mock.attempts(n)).sum();
    let (answered, errors, metric_errors) =
        node_metrics(&metrics, F::NAME.as_str(), scripts.len(), attempts);
    let observed = |outcome, counted, dominant, responses| Observed {
        outcome,
        counted,
        elapsed,
        dominant,
        attempts,
        responses,
        answered,
        errors: errors.clone(),
        metric_errors: metric_errors.clone(),
    };
    match result {
        Ok(output) => observed(
            Outcome::Ok,
            responses(&output),
            None,
            Some(responses(&output)),
        ),
        Err(AggregationError::Timeout {
            counted, dominant, ..
        }) => observed(Outcome::Timeout, counted, dominant, None),
        Err(AggregationError::ThresholdNotReached {
            counted, dominant, ..
        }) => observed(Outcome::ThresholdNotReached, counted, dominant, None),
        Err(other) => panic!("unexpected error: {other}"),
    }
}

/// Sums the per-node metrics of one run and checks them on every scenario, expectation or not: each label value
/// is in its set (`OBSERVABILITY.md`, Metrics), every node call is recorded once, every attempt the mock received
/// is counted once in the attempts per call.
fn node_metrics(
    metrics: &Metrics,
    flow: &str,
    nodes: usize,
    received: u32,
) -> (u64, BTreeMap<String, u64>, Vec<String>) {
    let (mut answered, mut calls, mut errors_by_code, mut errors) =
        (0, 0, BTreeMap::new(), Vec::new());
    let label = |labels: &BTreeMap<String, String>, name: &str| {
        labels.get(name).cloned().unwrap_or_default()
    };
    for ((name, labels), count) in crate::metrics::tests::samples(metrics) {
        // The alerting counters start at 0 for every flow and node (`Metrics::new`): not part of the run.
        if count == 0 {
            continue;
        }
        if label(&labels, "flow") != flow {
            errors.push(format!("{name}: flow {:?}", label(&labels, "flow")));
        }
        let node = label(&labels, "kms_node");
        let known_node = node
            .strip_prefix("node-")
            .and_then(|i| i.parse::<usize>().ok())
            .is_some_and(|i| i < nodes);
        // The call slot is the pod's and the wait after the threshold the aggregation's, not a node's.
        let per_node = !matches!(
            name.as_str(),
            "relayer_http_kms_call_slot_wait_seconds"
                | "relayer_http_kms_wait_after_threshold_seconds"
        );
        if per_node && !known_node {
            errors.push(format!("{name}: kms_node {node:?}"));
        }
        match name.as_str() {
            "relayer_http_kms_attempts" => calls += count,
            "relayer_http_kms_attempt_duration_seconds" => answered += count,
            "relayer_http_kms_call_slot_wait_seconds" => {}
            // One run, one wait at most.
            "relayer_http_kms_wait_after_threshold_seconds" if count == 1 => {}
            "relayer_http_kms_attempt_errors_total" => {
                let code = label(&labels, "code");
                if code != "cancelled" && ErrorCode::from_str(&code).is_err() {
                    errors.push(format!("{name}: code {code:?}"));
                }
                *errors_by_code.entry(code).or_default() += count;
            }
            other => errors.push(format!("unexpected metric {other}")),
        }
    }
    if calls != nodes as u64 {
        errors.push(format!("{calls} node calls recorded for {nodes} nodes"));
    }
    let attempts = crate::metrics::tests::histogram_sum(metrics, "relayer_http_kms_attempts");
    if attempts != f64::from(received) {
        errors.push(format!(
            "{attempts} attempts recorded per call, {received} received by the nodes"
        ));
    }
    (answered, errors_by_code, errors)
}

/// Every mismatch between `expect` and `observed`, one per line.
fn compare(expect: &Expect, observed: &Observed) -> String {
    let mut report = String::new();
    let mut check = |field: &str, expected: Option<String>, got: String| {
        if let Some(expected) = expected
            && expected != got
        {
            let _ = writeln!(report, "  {field}: expected {expected}, got {got}");
        }
    };
    check(
        "outcome",
        Some(format!("{:?}", expect.outcome)),
        format!("{:?}", observed.outcome),
    );
    check(
        "counted",
        expect.counted.map(|v| v.to_string()),
        observed.counted.to_string(),
    );
    check(
        "elapsed",
        expect.elapsed.map(|v| format!("{v:?}")),
        format!("{:?}", observed.elapsed),
    );
    check(
        "dominant",
        expect.dominant.map(|v| format!("{v:?}")),
        observed
            .dominant
            .map_or("none".to_owned(), |v| format!("{v:?}")),
    );
    check(
        "attempts",
        expect.attempts.map(|v| v.to_string()),
        observed.attempts.to_string(),
    );
    check(
        "responses",
        expect.responses.map(|v| v.to_string()),
        observed
            .responses
            .map_or("none".to_owned(), |v| v.to_string()),
    );
    if let Some(metrics) = &expect.metrics {
        check(
            "metrics.answered",
            Some(metrics.answered.to_string()),
            observed.answered.to_string(),
        );
        check(
            "metrics.errors",
            Some(format!("{:?}", metrics.errors)),
            format!("{:?}", observed.errors),
        );
    }
    for error in &observed.metric_errors {
        let _ = writeln!(report, "  metrics: {error}");
    }
    report
}

async fn run_file(path: &Path) -> Result<(), String> {
    let scenario = load(path)?;
    let unknown = scenario
        .nodes
        .iter()
        .flat_map(|g| &g.replies)
        .any(|r| *r == Reply::Error(ErrorCode::Unknown));
    if unknown {
        return Err(
            "a reply is not a known fixed reply or connector error code (typo?)".to_owned(),
        );
    }
    let observed = match scenario.flow {
        ScenarioFlow::UserDecrypt => {
            let request: UserDecryptionRequest = serde_json::from_str(USER_REQUEST_JSON).unwrap();
            let checks = scenario.checks.unwrap_or_default();
            run::<UserDecrypt>(&scenario, request, checks, |o| o.result.len()).await
        }
        ScenarioFlow::PublicDecrypt => {
            if scenario.checks.is_some() {
                return Err("`checks` applies to user_decrypt only".to_owned());
            }
            let request: PublicDecryptionRequest =
                serde_json::from_str(PUBLIC_REQUEST_JSON).unwrap();
            run::<PublicDecrypt>(&scenario, request, (), |o| o.signatures.len()).await
        }
    };
    let report = compare(&scenario.expect, &observed);
    if report.is_empty() {
        Ok(())
    } else {
        Err(format!("\n{report}"))
    }
}

fn scenario_files() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/kms_aggregator/scenarios");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    files.sort();
    files
}

/// A fresh current-thread runtime with paused time: exact virtual durations, one scenario at a time.
fn paused_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .start_paused(true)
        .build()
        .unwrap()
}

#[test]
fn yaml_scenarios() {
    let files = scenario_files();
    assert!(!files.is_empty(), "no scenario files");
    let mut failures = Vec::new();
    for file in &files {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        if let Err(e) = paused_runtime().block_on(run_file(file)) {
            failures.push(format!("{name}: {e}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} scenarios failed:\n{}",
        failures.len(),
        files.len(),
        failures.join("\n")
    );
}

#[test]
fn every_scenario_file_parses() {
    for file in scenario_files() {
        load(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    }
}

#[test]
fn compare_reports_every_mismatch() {
    let expect = Expect {
        outcome: Outcome::Ok,
        counted: Some(9),
        elapsed: Some(Duration::from_secs(5)),
        dominant: None,
        attempts: Some(13),
        responses: None,
        metrics: None,
    };
    let observed = Observed {
        outcome: Outcome::Timeout,
        counted: 8,
        elapsed: Duration::from_secs(5),
        dominant: Some(ErrorCode::AclDenied),
        attempts: 13,
        responses: None,
        answered: 0,
        errors: BTreeMap::new(),
        metric_errors: Vec::new(),
    };
    let report = compare(&expect, &observed);
    assert!(
        report.contains("outcome: expected Ok, got Timeout"),
        "{report}"
    );
    assert!(report.contains("counted: expected 9, got 8"), "{report}");
    assert!(!report.contains("elapsed"), "{report}");
    assert!(!report.contains("dominant"), "{report}");
    assert!(
        compare(
            &expect,
            &Observed {
                outcome: Outcome::Ok,
                counted: 9,
                ..observed
            }
        )
        .is_empty()
    );
}

#[test]
fn typo_in_a_reply_is_caught() {
    let dir = std::env::temp_dir().join(format!("relayer-http-scenario-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("typo.yaml");
    std::fs::write(
        &file,
        "flow: user_decrypt\nthreshold: 1\ntimeout: 1s\nnodes:\n  - { count: 1, replies: [acl_denied_typo] }\nexpect: { outcome: ok }\n",
    )
    .unwrap();
    let e = paused_runtime().block_on(run_file(&file)).unwrap_err();
    assert!(e.contains("typo"), "{e}");
    let _ = std::fs::remove_dir_all(dir);
}
