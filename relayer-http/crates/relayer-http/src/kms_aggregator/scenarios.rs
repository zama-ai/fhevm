//! Runs every `scenarios/*.yaml` file against the aggregator with the mock connector, under paused tokio time.
//! Adding a scenario = adding a file; `cargo test yaml_scenarios` runs them all and lists every failure.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use kms_connector_api::{ErrorCode, PublicDecryptionRequest, UserDecryptionRequest};
use serde::Deserialize;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use super::aggregator::{AggregationError, Aggregator};
use super::flows::Flow;
use super::flows::public_decrypt::PublicDecrypt;
use super::flows::user_decrypt::UserDecrypt;
use super::mock::{MockClient, PUBLIC_REQUEST_JSON, Reply, USER_REQUEST_JSON};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Scenario {
    flow: FlowName,
    threshold: usize,
    #[serde(with = "humantime_serde")]
    timeout: Duration,
    #[serde(default)]
    max_retries: u32,
    /// Time each attempt takes on every node.
    #[serde(with = "humantime_serde", default = "default_delay")]
    delay: Duration,
    /// Node groups in node order; the total is the number of nodes.
    nodes: Vec<Group>,
    expect: Expect,
}

fn default_delay() -> Duration {
    Duration::from_millis(10)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FlowName {
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
    responses: fn(&F::Output) -> usize,
) -> Observed {
    let scripts: Vec<Vec<Reply>> = scenario
        .nodes
        .iter()
        .flat_map(|g| std::iter::repeat_n(g.replies.clone(), g.count))
        .collect();
    let mock = MockClient::new(scripts.clone(), scenario.delay);
    let caller = mock.clone().caller(scenario.timeout, scenario.max_retries);
    let aggregator = Aggregator::<F>::new(caller, scenario.threshold, CancellationToken::new());
    let started = Instant::now();
    let result = aggregator.run("scenario", request).await;
    let elapsed = started.elapsed();
    let attempts = (0..scripts.len()).map(|n| mock.attempts(n)).sum();
    match result {
        Ok(output) => Observed {
            outcome: Outcome::Ok,
            counted: responses(&output),
            elapsed,
            dominant: None,
            attempts,
            responses: Some(responses(&output)),
        },
        Err(AggregationError::Timeout {
            counted, dominant, ..
        }) => Observed {
            outcome: Outcome::Timeout,
            counted,
            elapsed,
            dominant,
            attempts,
            responses: None,
        },
        Err(AggregationError::ThresholdNotReached {
            counted, dominant, ..
        }) => Observed {
            outcome: Outcome::ThresholdNotReached,
            counted,
            elapsed,
            dominant,
            attempts,
            responses: None,
        },
        Err(other) => panic!("unexpected error: {other}"),
    }
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
        FlowName::UserDecrypt => {
            let request: UserDecryptionRequest = serde_json::from_str(USER_REQUEST_JSON).unwrap();
            run::<UserDecrypt>(&scenario, request, |o| o.result.len()).await
        }
        FlowName::PublicDecrypt => {
            let request: PublicDecryptionRequest =
                serde_json::from_str(PUBLIC_REQUEST_JSON).unwrap();
            run::<PublicDecrypt>(&scenario, request, |o| o.signatures.len()).await
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
    };
    let observed = Observed {
        outcome: Outcome::Timeout,
        counted: 8,
        elapsed: Duration::from_secs(5),
        dominant: Some(ErrorCode::AclDenied),
        attempts: 13,
        responses: None,
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
