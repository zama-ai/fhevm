//! Prometheus metrics: the catalogue of `OBSERVABILITY.md` (one registry per process, owned by [`Metrics`]) and
//! the `GET /metrics` listener. Label values come from enums and the configuration only, never from request data.

use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use kms_connector_api::ErrorCode;
use prometheus::{HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry, TextEncoder};
use tokio::net::TcpListener;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;

/// The flow a request or a KMS node call belongs to; `none` for what answers before a flow is known (router
/// fallbacks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowName {
    UserDecrypt,
    PublicDecrypt,
    PadDecrypt,
    None,
}

impl FlowName {
    /// Every value, in the order of the generated references.
    pub const ALL: [Self; 4] = [
        Self::UserDecrypt,
        Self::PublicDecrypt,
        Self::PadDecrypt,
        Self::None,
    ];

    /// The decryption flows: what a request or a node call can belong to.
    pub const DECRYPTS: [Self; 3] = [Self::UserDecrypt, Self::PublicDecrypt, Self::PadDecrypt];

    /// The label and log value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserDecrypt => "user_decrypt",
            Self::PublicDecrypt => "public_decrypt",
            Self::PadDecrypt => "pad_decrypt",
            Self::None => "none",
        }
    }
}

/// How a request ended, by status class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// 2xx.
    Succeeded,
    /// 4xx: the client's request is at fault.
    Rejected,
    /// 5xx: the relayer or the KMS nodes are at fault.
    Failed,
    /// No answer: the client closed the connection first.
    Cancelled,
}

impl Outcome {
    /// Every value, in the order of the generated references.
    pub const ALL: [Self; 4] = [
        Self::Succeeded,
        Self::Rejected,
        Self::Failed,
        Self::Cancelled,
    ];

    /// The class of an HTTP status.
    pub const fn of_status(status: u16) -> Self {
        match status {
            500.. => Self::Failed,
            400..=499 => Self::Rejected,
            _ => Self::Succeeded,
        }
    }

    /// The label and log value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Rejected => "rejected",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

/// The largest KMS node set a deployment is sized for (the thresholds are 9 of 13 and 5 of 13), enforced by the
/// configuration's validation. The bound of `kms_node` in the series budget.
pub const MAX_KMS_NODES: usize = 13;

/// The connector's error codes, in declaration order: the `code` of a KMS attempt error, and what an aggregation
/// error maps to. Built from [`next`], a match with no wildcard: a new `ErrorCode` variant does not compile until it
/// is placed in the chain.
pub fn connector_codes() -> impl Iterator<Item = ErrorCode> {
    std::iter::successors(Some(ErrorCode::Malformed), |code| next(*code))
}

/// The variant after `code` in declaration order.
fn next(code: ErrorCode) -> Option<ErrorCode> {
    match code {
        ErrorCode::Malformed => Some(ErrorCode::UnsupportedAttestationType),
        ErrorCode::UnsupportedAttestationType => Some(ErrorCode::SenderAuthenticationFailed),
        ErrorCode::SenderAuthenticationFailed => Some(ErrorCode::AuthExpired),
        ErrorCode::AuthExpired => Some(ErrorCode::RateLimited),
        ErrorCode::RateLimited => Some(ErrorCode::Overloaded),
        ErrorCode::Overloaded => Some(ErrorCode::AclDenied),
        ErrorCode::AclDenied => Some(ErrorCode::UserSignatureRejected),
        ErrorCode::UserSignatureRejected => Some(ErrorCode::UserDecryptionExpired),
        ErrorCode::UserDecryptionExpired => Some(ErrorCode::CiphertextNotFound),
        ErrorCode::CiphertextNotFound => Some(ErrorCode::CoproConsensusFailed),
        ErrorCode::CoproConsensusFailed => Some(ErrorCode::KmsContextInvalid),
        ErrorCode::KmsContextInvalid => Some(ErrorCode::KmsContextDestroyed),
        ErrorCode::KmsContextDestroyed => Some(ErrorCode::Unprocessable),
        ErrorCode::Unprocessable => Some(ErrorCode::UpstreamTransient),
        ErrorCode::UpstreamTransient => Some(ErrorCode::Timeout),
        ErrorCode::Timeout => Some(ErrorCode::Unknown),
        ErrorCode::Unknown => None,
    }
}

/// The `code` of a relayer defect: an internal error of the aggregation, or a panic of a request handler.
pub const INTERNAL_CODE: &str = "internal";

/// The `code` of a KMS attempt whose node task panicked.
pub const PANIC_CODE: &str = "panic";

/// Connector codes that blame the request, not the KMS node: the node answered, and answered correctly.
pub const CLIENT_CAUSED: [ErrorCode; 7] = [
    ErrorCode::Malformed,
    ErrorCode::UnsupportedAttestationType,
    ErrorCode::AclDenied,
    ErrorCode::UserSignatureRejected,
    ErrorCode::UserDecryptionExpired,
    ErrorCode::CiphertextNotFound,
    ErrorCode::Unprocessable,
];

/// The `code` values of a KMS attempt error that say nothing about the node's health: the client-caused connector
/// codes, `cancelled` (the relayer cut the attempt) and `panic` (the relayer's own task failed). The node health
/// alert excludes exactly these; a test compares.
pub fn not_node_health() -> Vec<String> {
    CLIENT_CAUSED
        .iter()
        .map(|code| code.as_str())
        .chain(["cancelled", PANIC_CODE])
        .map(str::to_owned)
        .collect()
}

/// Where a label's values come from.
pub enum Values {
    /// A closed set defined in code.
    Enumerated(fn() -> Vec<String>),
    /// Names from the configuration, at most `max` of them.
    Configured { source: &'static str, max: usize },
}

/// One label: its name and its bounded set of values.
pub struct Label {
    pub name: &'static str,
    pub meaning: &'static str,
    pub values: Values,
}

impl Label {
    /// The most values the label can take.
    pub fn bound(&self) -> usize {
        match &self.values {
            Values::Enumerated(values) => values().len(),
            Values::Configured { max, .. } => *max,
        }
    }
}

/// Every label of every metric. A metric names its labels from here, so a value set is defined once.
pub const LABELS: &[Label] = &[
    Label {
        name: "flow",
        meaning: "the decryption flow; `none` for requests answered before a flow is known",
        values: Values::Enumerated(|| FlowName::ALL.map(|f| f.as_str().to_owned()).to_vec()),
    },
    Label {
        name: "outcome",
        meaning: "how a request ended: by status class, or `cancelled` when the client closed the connection first",
        values: Values::Enumerated(|| Outcome::ALL.map(|o| o.as_str().to_owned()).to_vec()),
    },
    Label {
        name: "code",
        meaning: "`ok`, an API error code of a request, or the connector error code of a failed KMS node attempt (`cancelled` for an attempt cut in flight, `panic` for a node task that panicked)",
        values: Values::Enumerated(|| {
            let own = [
                "ok",
                "cancelled",
                "client_closed",
                "not_found",
                "method_not_allowed",
                INTERNAL_CODE,
                PANIC_CODE,
            ];
            own.iter()
                .map(|v| (*v).to_owned())
                .chain(connector_codes().map(|c| c.as_str().to_owned()))
                .collect()
        }),
    },
    Label {
        name: "kms_node",
        meaning: "a KMS node, by its name in the configuration",
        values: Values::Configured {
            source: "`kms_aggregator.endpoints[].name`",
            max: MAX_KMS_NODES,
        },
    },
];

/// What a metric counts.
pub enum Kind {
    Counter,
    /// Upper bounds of the buckets, `+Inf` excluded.
    Histogram(&'static [f64]),
}

/// One metric: the registry is built from these, and so are the generated references and the series budget.
pub struct MetricDef {
    pub name: &'static str,
    pub kind: Kind,
    pub labels: &'static [&'static str],
    /// What one observation is.
    pub help: &'static str,
}

/// Seconds, 5 ms to 30 s: a connector round trip, a request, a wait for a call slot. 4 s is the slow request alert's
/// threshold (0.8 x the default call timeout): it must be a bucket bound for the quantile to be exact there.
const DURATION_BUCKETS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 4.0, 5.0, 7.5, 10.0, 30.0,
];

/// Attempts one node call took: 1 is the first try, each step above is a retry.
const ATTEMPT_BUCKETS: &[f64] = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0];

/// Seconds, 5 ms to 5 s: the wait after the threshold is bounded by the deadline (5 s by default) and mostly under
/// 1 s, so the buckets are dense below 1 s.
const WAIT_BUCKETS: &[f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0];

const REQUESTS: MetricDef = MetricDef {
    name: "relayer_http_requests_total",
    kind: Kind::Counter,
    labels: &["flow", "outcome", "code"],
    help: "One request answered, by flow, outcome and API error code.",
};
const REQUEST_DURATION: MetricDef = MetricDef {
    name: "relayer_http_request_duration_seconds",
    kind: Kind::Histogram(DURATION_BUCKETS),
    labels: &["flow", "outcome"],
    help: "One request: seconds from receipt to the answer.",
};
const ATTEMPT_DURATION: MetricDef = MetricDef {
    name: "relayer_http_kms_attempt_duration_seconds",
    kind: Kind::Histogram(DURATION_BUCKETS),
    labels: &["kms_node", "flow"],
    help: "One attempt that got an answer from a KMS node, or was cut while waiting for it: seconds from sending to the answer or the cut. No observation when the connection failed.",
};
const ATTEMPT_ERRORS: MetricDef = MetricDef {
    name: "relayer_http_kms_attempt_errors_total",
    kind: Kind::Counter,
    labels: &["kms_node", "flow", "code"],
    help: "One failed attempt to a KMS node, by connector error code; `cancelled` for an attempt cut in flight, `panic` for a node task that panicked.",
};
const ATTEMPTS: MetricDef = MetricDef {
    name: "relayer_http_kms_attempts",
    kind: Kind::Histogram(ATTEMPT_BUCKETS),
    labels: &["kms_node", "flow"],
    help: "One call to a KMS node: how many attempts it made, the first try included; a failure to sign is not an attempt.",
};
const CALL_SLOT_WAIT: MetricDef = MetricDef {
    name: "relayer_http_kms_call_slot_wait_seconds",
    kind: Kind::Histogram(DURATION_BUCKETS),
    labels: &["flow"],
    help: "One attempt: seconds spent waiting for an outbound call slot (`kms_aggregator.max_concurrent_calls`).",
};

const SIGNER_ERRORS: MetricDef = MetricDef {
    name: "relayer_http_signer_errors_total",
    kind: Kind::Counter,
    labels: &["flow"],
    help: "One failed attempt to sign a connector call (the signing key unreachable or refusing): nothing was sent, the KMS node is not at fault.",
};
const WAIT_AFTER_THRESHOLD: MetricDef = MetricDef {
    name: "relayer_http_kms_wait_after_threshold_seconds",
    kind: Kind::Histogram(WAIT_BUCKETS),
    labels: &["flow"],
    help: "One aggregation that reached its threshold: seconds from reaching it to the answer, spent waiting for the remaining KMS nodes (until every node finished or the deadline). Buckets 5 ms to 5 s.",
};

/// Every metric of the process.
pub const METRICS: &[MetricDef] = &[
    REQUESTS,
    REQUEST_DURATION,
    ATTEMPT_DURATION,
    ATTEMPT_ERRORS,
    ATTEMPTS,
    CALL_SLOT_WAIT,
    SIGNER_ERRORS,
    WAIT_AFTER_THRESHOLD,
];

/// Every metric of the process, registered in its own registry. Built once in `App::new`; tests build their own.
pub struct Metrics {
    registry: Registry,
    requests: IntCounterVec,
    request_duration: HistogramVec,
    attempt_duration: HistogramVec,
    attempt_errors: IntCounterVec,
    attempts: HistogramVec,
    call_slot_wait: HistogramVec,
    signer_errors: IntCounterVec,
    wait_after_threshold: HistogramVec,
}

impl Metrics {
    /// Registers `METRICS`. Fails only on a duplicate or malformed definition, i.e. a bug.
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Registry::new();
        let counter = |def: &MetricDef| -> Result<IntCounterVec, prometheus::Error> {
            let counter = IntCounterVec::new(Opts::new(def.name, def.help), def.labels)?;
            registry.register(Box::new(counter.clone()))?;
            Ok(counter)
        };
        let histogram = |def: &MetricDef| -> Result<HistogramVec, prometheus::Error> {
            let Kind::Histogram(buckets) = def.kind else {
                return Err(prometheus::Error::Msg(format!(
                    "{} is not a histogram",
                    def.name
                )));
            };
            let opts = HistogramOpts::new(def.name, def.help).buckets(buckets.to_vec());
            let histogram = HistogramVec::new(opts, def.labels)?;
            registry.register(Box::new(histogram.clone()))?;
            Ok(histogram)
        };
        let metrics = Self {
            requests: counter(&REQUESTS)?,
            request_duration: histogram(&REQUEST_DURATION)?,
            attempt_duration: histogram(&ATTEMPT_DURATION)?,
            attempt_errors: counter(&ATTEMPT_ERRORS)?,
            attempts: histogram(&ATTEMPTS)?,
            call_slot_wait: histogram(&CALL_SLOT_WAIT)?,
            signer_errors: counter(&SIGNER_ERRORS)?,
            wait_after_threshold: histogram(&WAIT_AFTER_THRESHOLD)?,
            registry,
        };
        // A counter that only exists after its first increment hides that increment from `increase()`: the alerts
        // on failures that are never routine start from 0. `none` too: a panic outside a guarded handler.
        metrics.requests.with_label_values(&[
            FlowName::None.as_str(),
            Outcome::Failed.as_str(),
            INTERNAL_CODE,
        ]);
        for flow in FlowName::DECRYPTS {
            metrics.requests.with_label_values(&[
                flow.as_str(),
                Outcome::Failed.as_str(),
                INTERNAL_CODE,
            ]);
            metrics.signer_errors.with_label_values(&[flow.as_str()]);
            // Rare per flow: a first observation in a new series would be invisible to `rate()` too.
            metrics
                .wait_after_threshold
                .with_label_values(&[flow.as_str()]);
        }
        Ok(metrics)
    }

    /// Starts the `panic` error series of `kms_nodes` at 0 (see `new`); the nodes are known once the `Caller` is built.
    pub fn announce_nodes<'a>(&self, kms_nodes: impl IntoIterator<Item = &'a str>) {
        for kms_node in kms_nodes {
            for flow in FlowName::DECRYPTS {
                self.attempt_errors
                    .with_label_values(&[kms_node, flow.as_str(), PANIC_CODE]);
            }
        }
    }

    /// One answered request; `code` is `ok` or the API error code.
    pub fn request(&self, flow: FlowName, outcome: Outcome, code: &str, elapsed: Duration) {
        let (flow, outcome) = (flow.as_str(), outcome.as_str());
        self.requests
            .with_label_values(&[flow, outcome, code])
            .inc();
        self.request_duration
            .with_label_values(&[flow, outcome])
            .observe(elapsed.as_secs_f64());
    }

    /// One attempt that got an answer from `kms_node`, or was cut while waiting for it.
    pub fn attempt_answered(&self, kms_node: &str, flow: FlowName, elapsed: Duration) {
        self.attempt_duration
            .with_label_values(&[kms_node, flow.as_str()])
            .observe(elapsed.as_secs_f64());
    }

    /// One failed attempt; `code` is the connector error code, or `cancelled`.
    pub fn attempt_error(&self, kms_node: &str, flow: FlowName, code: &str) {
        self.attempt_errors
            .with_label_values(&[kms_node, flow.as_str(), code])
            .inc();
    }

    /// One call to `kms_node` ended after `attempts` attempts.
    pub fn call(&self, kms_node: &str, flow: FlowName, attempts: u32) {
        self.attempts
            .with_label_values(&[kms_node, flow.as_str()])
            .observe(f64::from(attempts));
    }

    /// One attempt waited `waited` for a call slot.
    pub fn call_slot_wait(&self, flow: FlowName, waited: Duration) {
        self.call_slot_wait
            .with_label_values(&[flow.as_str()])
            .observe(waited.as_secs_f64());
    }

    /// One attempt to sign a call for `flow` failed.
    pub fn signer_error(&self, flow: FlowName) {
        self.signer_errors.with_label_values(&[flow.as_str()]).inc();
    }

    /// One aggregation of `flow` answered `waited` after reaching its threshold.
    pub fn wait_after_threshold(&self, flow: FlowName, waited: Duration) {
        self.wait_after_threshold
            .with_label_values(&[flow.as_str()])
            .observe(waited.as_secs_f64());
    }

    /// The Prometheus text format of every metric.
    pub fn render(&self) -> Result<String, prometheus::Error> {
        TextEncoder::new().encode_to_string(&self.registry.gather())
    }

    /// Test only: every metric family, for assertions.
    #[cfg(test)]
    pub(crate) fn gather(&self) -> Vec<prometheus::proto::MetricFamily> {
        self.registry.gather()
    }
}

/// Serves `GET /metrics` on the bound `listener` until `shutdown` is cancelled.
pub async fn serve(
    listener: TcpListener,
    metrics: Arc<Metrics>,
    shutdown: CancellationToken,
) -> std::io::Result<()> {
    let router = Router::new()
        .route("/metrics", get(scrape))
        .with_state(metrics);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown.cancelled_owned())
        .await
}

async fn scrape(State(metrics): State<Arc<Metrics>>) -> Response {
    match metrics.render() {
        Ok(text) => ([(header::CONTENT_TYPE, prometheus::TEXT_FORMAT)], text).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::BTreeMap;

    use super::*;

    /// `(metric, labels)` → sample count (histograms) or value (counters), for assertions.
    pub(crate) fn samples(metrics: &Metrics) -> BTreeMap<(String, BTreeMap<String, String>), u64> {
        let mut out = BTreeMap::new();
        for family in metrics.gather() {
            for metric in family.get_metric() {
                let labels = metric
                    .get_label()
                    .iter()
                    .map(|l| (l.name().to_owned(), l.value().to_owned()))
                    .collect();
                let value = if family.get_field_type() == prometheus::proto::MetricType::HISTOGRAM {
                    metric.get_histogram().get_sample_count()
                } else {
                    metric.get_counter().get_value() as u64
                };
                out.insert((family.name().to_owned(), labels), value);
            }
        }
        out
    }

    /// The sum of the samples of `name`: counter values, histogram counts.
    pub(crate) fn total(metrics: &Metrics, name: &str) -> u64 {
        samples(metrics)
            .iter()
            .filter(|((metric, _), _)| metric == name)
            .map(|(_, v)| *v)
            .sum()
    }

    /// The sample of `name` whose labels include `labels`: a counter value or a histogram count.
    pub(crate) fn value(metrics: &Metrics, name: &str, labels: &[(&str, &str)]) -> Option<u64> {
        samples(metrics)
            .into_iter()
            .find_map(|((metric, have), v)| {
                let matches = labels
                    .iter()
                    .all(|(k, want)| have.get(*k).is_some_and(|got| got == want));
                (metric == name && matches).then_some(v)
            })
    }

    /// The sum of a histogram's observations, over every series of `name`.
    pub(crate) fn histogram_sum(metrics: &Metrics, name: &str) -> f64 {
        metrics
            .gather()
            .iter()
            .filter(|f| f.name() == name)
            .flat_map(|f| f.get_metric())
            .map(|m| m.get_histogram().get_sample_sum())
            .sum()
    }

    #[test]
    fn registry_is_built_from_the_definitions() {
        let m = Metrics::new().unwrap();
        m.request(FlowName::None, Outcome::Failed, "internal", Duration::ZERO);
        m.attempt_answered("kms_00", FlowName::UserDecrypt, Duration::ZERO);
        m.attempt_error("kms_00", FlowName::UserDecrypt, "cancelled");
        m.call("kms_00", FlowName::UserDecrypt, 1);
        m.call_slot_wait(FlowName::UserDecrypt, Duration::ZERO);
        m.signer_error(FlowName::UserDecrypt);
        m.wait_after_threshold(FlowName::UserDecrypt, Duration::ZERO);
        let mut registered: Vec<String> = m.gather().iter().map(|f| f.name().to_owned()).collect();
        let mut defined: Vec<String> = METRICS.iter().map(|d| d.name.to_owned()).collect();
        registered.sort_unstable();
        defined.sort_unstable();
        assert_eq!(registered, defined);
    }

    #[test]
    fn connector_codes_start_at_the_first_and_end_at_the_last_variant() {
        let codes: Vec<ErrorCode> = connector_codes().collect();
        assert_eq!(codes.first(), Some(&ErrorCode::Malformed));
        assert_eq!(codes.last(), Some(&ErrorCode::Unknown));
        assert_eq!(codes.len(), 17);
        let unique: std::collections::HashSet<&str> = codes.iter().map(|c| c.as_str()).collect();
        assert_eq!(unique.len(), codes.len());
    }

    #[test]
    fn the_defects_are_counted_from_zero_and_the_nodes_once_announced() {
        let m = Metrics::new().unwrap();
        for flow in ["user_decrypt", "public_decrypt", "none"] {
            let internal = [("flow", flow), ("outcome", "failed"), ("code", "internal")];
            assert_eq!(value(&m, "relayer_http_requests_total", &internal), Some(0));
        }
        for flow in ["user_decrypt", "public_decrypt"] {
            let wait = [("flow", flow)];
            assert_eq!(
                value(&m, "relayer_http_kms_wait_after_threshold_seconds", &wait),
                Some(0)
            );
            let signer = [("flow", flow)];
            assert_eq!(
                value(&m, "relayer_http_signer_errors_total", &signer),
                Some(0)
            );
        }
        assert_eq!(total(&m, "relayer_http_kms_attempt_errors_total"), 0);
        m.announce_nodes(["kms_00"]);
        let panic = [
            ("kms_node", "kms_00"),
            ("flow", "public_decrypt"),
            ("code", "panic"),
        ];
        assert_eq!(
            value(&m, "relayer_http_kms_attempt_errors_total", &panic),
            Some(0)
        );
    }

    #[test]
    fn the_node_health_exclusions_are_values_of_the_code_label() {
        let code = LABELS.iter().find(|l| l.name == "code").unwrap();
        let Values::Enumerated(values) = &code.values else {
            unreachable!("`code` is enumerated");
        };
        let values = values();
        for excluded in not_node_health() {
            assert!(values.contains(&excluded), "{excluded}");
        }
    }

    #[test]
    fn outcome_follows_the_status_class() {
        for (status, outcome) in [
            (200, Outcome::Succeeded),
            (400, Outcome::Rejected),
            (499, Outcome::Rejected),
            (500, Outcome::Failed),
            (504, Outcome::Failed),
        ] {
            assert_eq!(Outcome::of_status(status), outcome, "{status}");
        }
    }

    #[tokio::test]
    async fn serves_the_text_format() {
        let metrics = Arc::new(Metrics::new().unwrap());
        metrics.request(
            FlowName::UserDecrypt,
            Outcome::Succeeded,
            "ok",
            Duration::from_millis(40),
        );
        metrics.attempt_error("kms_00", FlowName::UserDecrypt, "rate_limited");
        metrics.call("kms_00", FlowName::UserDecrypt, 2);
        metrics.signer_error(FlowName::UserDecrypt);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let shutdown = CancellationToken::new();
        let server = tokio::spawn(serve(listener, metrics, shutdown.clone()));
        let response = reqwest::get(format!("http://{addr}/metrics"))
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        for line in [
            r#"relayer_http_requests_total{code="ok",flow="user_decrypt",outcome="succeeded"} 1"#,
            r#"relayer_http_kms_attempt_errors_total{code="rate_limited",flow="user_decrypt",kms_node="kms_00"} 1"#,
            r#"relayer_http_kms_attempts_sum{flow="user_decrypt",kms_node="kms_00"} 2"#,
            r#"relayer_http_signer_errors_total{flow="user_decrypt"} 1"#,
        ] {
            assert!(body.contains(line), "{line} not in {body}");
        }
        shutdown.cancel();
        server.await.unwrap().unwrap();
    }
}
