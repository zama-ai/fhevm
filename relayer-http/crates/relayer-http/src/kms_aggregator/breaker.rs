//! Per-node circuit breaker: which KMS nodes an aggregation calls, judged from the outcome of their last call.
//!
//! Every KMS node has a switch. Closed means the node is used, open means it is skipped.
//!
//! 1. A request is sent to the closed nodes only.
//! 2. When a node's call ends, it is judged once. Unhealthy means the node could not be reached, answered 502,
//!    503 or 504, or gave no answer at all before the deadline. Anything else means healthy, including a request
//!    error such as `ciphertext_not_found` or `acl_denied`, even when the deadline cut the last retry.
//! 3. An unhealthy node is opened for `exclude_for`, unless that would leave fewer than `min_nodes` closed nodes.
//!    Then it stays in use and the requests stay slow.
//! 4. After the window, the next request is also sent to the opened node, as a probe. The request does not wait
//!    for the probe. If the probe answers, the node is closed again. If not, it is opened for another window.
//! 5. `reset()` closes every node.
//!
//! Only the request that detects an outage is slow, once per pod per outage. No request ever waits for an open
//! node or for a probe.
//!
//! Why the deadline alone is not a signal: a healthy node answering `ciphertext_not_found` is retried until its
//! next wait does not fit before the deadline; if its last attempt is still in flight when the deadline fires, the
//! call ends cancelled exactly like a hung node's. The two are told apart by the node's last answer during the
//! call (`CallResult::last_code`): no answer at all, or a last answer saying the node cannot serve, is unhealthy;
//! any other last answer means the node is up and the retries were legitimate.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use kms_connector_api::ErrorCode;
use tokio::time::Instant;
use tracing::{debug, info, warn};

use super::call::{CallError, CallResult};
use super::client::AttemptError;
use crate::metrics::{Metrics, Outcome};

/// The node's last answer during a call, as the `last_code` log field and the `code` label of an exclusion
/// render it: the code, or `cancelled` when the node never answered before the cut (a value the label already has
/// for an attempt cut in flight).
pub(super) fn last_code_label(last_code: Option<ErrorCode>) -> &'static str {
    last_code.map_or("cancelled", ErrorCode::as_str)
}

/// A duration as the `*_ms` log fields carry it.
fn ms(duration: Duration) -> u64 {
    duration.as_millis() as u64
}

/// What one finished call says about its node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The node could not serve: unreachable, hung, at its in-flight cap, or failing behind its proxy.
    /// `last_code` is the node's last answer, `None` when it never answered before the deadline.
    Unhealthy { last_code: Option<ErrorCode> },
    /// The node is up: it answered, whatever it answered.
    Answered,
    /// Nothing is known about the node: fail fast cut the call, signing failed, or the task panicked.
    Ignored,
}

impl Verdict {
    /// Judges one finished call; `cut_at_deadline` is true when the aggregation's deadline cancelled it.
    pub fn of<R>(call: &CallResult<R>, cut_at_deadline: bool) -> Self {
        match &call.result {
            Ok(_) => Self::Answered,
            // Nothing was sent: our signer failed, not the node.
            Err(CallError::Failed(AttemptError::Signer(_))) => Self::Ignored,
            Err(CallError::Failed(failure)) => Self::of_last_code(Some(failure.code())),
            // Cut at the deadline: the node's last answer during the call tells a hung node from a legitimate
            // retry that did not finish in time.
            Err(CallError::Cancelled) if cut_at_deadline => Self::of_last_code(call.last_code),
            // Cut by fail fast: the aggregation gave up on every node, this one may be fine.
            Err(CallError::Cancelled) => Self::Ignored,
        }
    }

    /// No answer, or a last answer saying the node cannot serve, is unhealthy; any other answer means it is up.
    fn of_last_code(last_code: Option<ErrorCode>) -> Self {
        match last_code {
            Some(code) if !blames_the_node(code) => Self::Answered,
            last_code => Self::Unhealthy { last_code },
        }
    }
}

/// Is `code` one a node answers when it cannot serve? `upstream_transient` (the proxy without a healthy
/// backend, the endpoint's storage or stream, the worker's transient failures; a transport error or a bare
/// status maps to it too), `overloaded` (the endpoint's in-flight cap) and `timeout` (a wait that ran out
/// behind the node). Every other code is about the request or our own side: the node answered correctly.
fn blames_the_node(code: ErrorCode) -> bool {
    matches!(
        code,
        ErrorCode::UpstreamTransient | ErrorCode::Overloaded | ErrorCode::Timeout
    )
}

/// One KMS node's switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeState {
    /// In use.
    Closed,
    /// Skipped until `until`; then the next `select` sends it a probe. `since`: when it was first opened.
    Open { since: Instant, until: Instant },
    /// A probe is in flight; skipped until it reports.
    Probing { since: Instant },
}

/// Which nodes one aggregation calls and which ones it probes, as indices into the endpoint list, with the names
/// of the nodes left out and probed for the `aggregation started` line. `generation` is echoed back by the
/// outcomes of that aggregation, so that a `reset()` in between drops them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub included: Vec<usize>,
    pub probes: Vec<usize>,
    pub excluded: Vec<String>,
    pub probed: Vec<String>,
    pub generation: u64,
}

/// Every switch and the rules they follow, behind one lock.
struct State {
    /// One switch per endpoint, in endpoint order, with the node's name.
    nodes: Vec<(String, NodeState)>,
    /// Never open a node when fewer than this many would stay closed.
    min_nodes: usize,
    /// How long a node stays open before the next request probes it.
    exclude_for: Duration,
    /// Bumped by `reset()`: outcomes carrying an older value are dropped.
    generation: u64,
}

impl State {
    fn closed_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|(_, state)| *state == NodeState::Closed)
            .count()
    }

    fn node_mut(&mut self, name: &str) -> Option<&mut NodeState> {
        self.nodes
            .iter_mut()
            .find(|(node, _)| node == name)
            .map(|(_, state)| state)
    }
}

/// The switches of every KMS node of the pod, shared by both flows. Disabled: every node is always in use and
/// every method is a no-op. The lock is held for a few comparisons at a time, never across an await.
pub struct CircuitBreakers {
    /// How many endpoints there are, for the disabled selection.
    nodes: usize,
    /// `None` when disabled.
    state: Option<Mutex<State>>,
    metrics: Arc<Metrics>,
}

impl CircuitBreakers {
    /// `names` in endpoint order; `enabled: false` builds the no-op set, which records no metric.
    pub fn new(
        enabled: bool,
        names: Vec<String>,
        min_nodes: usize,
        exclude_for: Duration,
        metrics: Arc<Metrics>,
    ) -> Self {
        let nodes = names.len();
        let state = enabled.then(|| {
            // Every node starts in use (the floor counter starts at 0 in `Metrics::announce_nodes`).
            for name in &names {
                metrics.node_excluded(name, false);
            }
            Mutex::new(State {
                nodes: names
                    .into_iter()
                    .map(|name| (name, NodeState::Closed))
                    .collect(),
                min_nodes,
                exclude_for,
                generation: 0,
            })
        });
        Self {
            nodes,
            state,
            metrics,
        }
    }

    /// Whether `circuit_breaker.enabled` was set: off, nothing is tracked and every node is always called.
    pub fn is_enabled(&self) -> bool {
        self.state.is_some()
    }

    /// The state, when enabled. A poisoned lock is still consistent: every update is a few assignments.
    fn lock(&self) -> Option<MutexGuard<'_, State>> {
        self.state
            .as_ref()
            .map(|state| state.lock().unwrap_or_else(PoisonError::into_inner))
    }

    /// Before a fan-out: the closed nodes to call, and the open nodes whose window has elapsed to probe. Each
    /// probed node is marked `Probing`, so concurrent aggregations send it one probe at a time.
    pub fn select(&self, now: Instant) -> Selection {
        let Some(mut state) = self.lock() else {
            return Selection {
                included: (0..self.nodes).collect(),
                probes: Vec::new(),
                excluded: Vec::new(),
                probed: Vec::new(),
                generation: 0,
            };
        };
        let mut selection = Selection {
            included: Vec::with_capacity(self.nodes),
            probes: Vec::new(),
            excluded: Vec::new(),
            probed: Vec::new(),
            generation: state.generation,
        };
        for (index, (name, node)) in state.nodes.iter_mut().enumerate() {
            match *node {
                NodeState::Closed => selection.included.push(index),
                NodeState::Open { since, until } if until <= now => {
                    *node = NodeState::Probing { since };
                    selection.probes.push(index);
                    selection.probed.push(name.clone());
                    selection.excluded.push(name.clone());
                    debug!(kms_node = %name, excluded_ms = ms(now - since), "node probed");
                }
                // Still inside its window, or a probe already in flight: left out.
                NodeState::Open { .. } | NodeState::Probing { .. } => {
                    selection.excluded.push(name.clone());
                }
            }
        }
        selection
    }

    /// Opens `switch` for a window: the state, the gauge, the exclusion counter and one WARN line, `message`.
    /// `closed_after` is how many nodes stay in use.
    #[allow(clippy::too_many_arguments)]
    fn open_for_window(
        &self,
        switch: &mut NodeState,
        node: &str,
        since: Instant,
        now: Instant,
        exclude_for: Duration,
        last_code: Option<ErrorCode>,
        closed_after: usize,
        message: &'static str,
    ) {
        // `exclude_for` is validated to be at most an hour: this cannot overflow.
        *switch = NodeState::Open {
            since,
            until: now + exclude_for,
        };
        self.metrics.node_excluded(node, true);
        self.metrics
            .node_exclusion(node, last_code_label(last_code));
        warn!(
            kms_node = node,
            last_code = last_code_label(last_code),
            excluded_for_ms = ms(exclude_for),
            closed_nodes = closed_after,
            "{message}"
        );
    }

    /// After a called node's outcome: `Unhealthy` opens it for `exclude_for`, unless that would leave fewer than
    /// `min_nodes` closed nodes (then it is kept). An outcome of a call started before a `reset()`, or for a
    /// node another aggregation opened meanwhile, changes nothing.
    pub fn on_call_outcome(&self, node: &str, verdict: &Verdict, now: Instant, generation: u64) {
        let Some(mut state) = self.lock() else { return };
        let Verdict::Unhealthy { last_code } = verdict else {
            return;
        };
        if state.generation != generation {
            return;
        }
        let (closed, min_nodes, exclude_for) =
            (state.closed_count(), state.min_nodes, state.exclude_for);
        let Some(switch) = state.node_mut(node) else {
            return;
        };
        if *switch != NodeState::Closed {
            return;
        }
        // The floor: opening this node must leave at least `min_nodes` in use.
        if closed > min_nodes {
            self.open_for_window(
                switch,
                node,
                now,
                now,
                exclude_for,
                *last_code,
                closed - 1,
                "node excluded",
            );
        } else {
            self.metrics.floor_hit(node);
            warn!(
                kms_node = node,
                last_code = last_code_label(*last_code),
                min_nodes,
                closed_nodes = closed,
                "node kept: floor reached"
            );
        }
    }

    /// After a probe's outcome: `Answered` closes the node, `Unhealthy` opens it for another window, `Ignored`
    /// lets the next aggregation probe it again. Dropped after a `reset()`, or when the node is not probing.
    pub fn on_probe_outcome(&self, node: &str, verdict: &Verdict, now: Instant, generation: u64) {
        let Some(mut state) = self.lock() else { return };
        if state.generation != generation {
            return;
        }
        let (closed, exclude_for) = (state.closed_count(), state.exclude_for);
        let Some(switch) = state.node_mut(node) else {
            return;
        };
        let NodeState::Probing { since } = *switch else {
            return;
        };
        match verdict {
            Verdict::Answered => {
                self.metrics.node_excluded(node, false);
                self.metrics.node_probe(node, Outcome::Succeeded);
                info!(
                    kms_node = node,
                    excluded_ms = ms(now - since),
                    closed_nodes = closed + 1,
                    "node included"
                );
                *switch = NodeState::Closed;
            }
            Verdict::Unhealthy { last_code } => {
                self.metrics.node_probe(node, Outcome::Failed);
                self.open_for_window(
                    switch,
                    node,
                    since,
                    now,
                    exclude_for,
                    *last_code,
                    closed,
                    "probe failed: node excluded",
                );
            }
            // Nothing is known: the next aggregation probes again.
            Verdict::Ignored => {
                self.metrics.node_probe(node, Outcome::Cancelled);
                *switch = NodeState::Open { since, until: now };
            }
        }
    }

    /// Closes every node and starts a new generation: the outcomes of calls and probes started before are
    /// dropped. Returns how many nodes were out. For the dynamic configuration: a refreshed endpoint list.
    pub fn reset(&self) -> usize {
        let Some(mut state) = self.lock() else {
            return 0;
        };
        let out = state.nodes.len() - state.closed_count();
        for (name, switch) in &mut state.nodes {
            *switch = NodeState::Closed;
            self.metrics.node_excluded(name, false);
        }
        state.generation += 1;
        info!(closed = out, "breakers reset");
        out
    }

    /// Test only: the names of the nodes currently out (open or probing).
    #[cfg(test)]
    pub fn excluded_names(&self) -> Vec<String> {
        self.lock().map_or_else(Vec::new, |state| {
            state
                .nodes
                .iter()
                .filter(|(_, switch)| *switch != NodeState::Closed)
                .map(|(name, _)| name.clone())
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use kms_connector_api::{ErrorCode, ErrorResponse};
    use tokio::time::{Instant, advance};

    use super::*;
    use crate::kms_aggregator::call::{CallError, CallResult, Retries};
    use crate::kms_aggregator::client::AttemptError;
    use crate::metrics::tests::value;
    use crate::metrics::{Metrics, connector_codes};

    const N: usize = 13;
    const MIN_NODES: usize = 11;
    const WINDOW: Duration = Duration::from_secs(30);

    fn names() -> Vec<String> {
        (0..N).map(|i| format!("node-{i}")).collect()
    }

    fn breakers() -> CircuitBreakers {
        CircuitBreakers::new(true, names(), MIN_NODES, WINDOW, metrics())
    }

    fn metrics() -> Arc<Metrics> {
        Arc::new(Metrics::new().unwrap())
    }

    #[tokio::test(start_paused = true)]
    async fn breaker_metrics_move_with_the_state() {
        let metrics = metrics();
        metrics.announce_nodes(names().iter().map(String::as_str));
        let breakers = CircuitBreakers::new(true, names(), MIN_NODES, WINDOW, metrics.clone());
        let gauge = |node: &str| {
            value(
                &metrics,
                "relayer_http_kms_node_excluded",
                &[("kms_node", node)],
            )
        };
        let exclusions = |node: &str, code: &str| {
            value(
                &metrics,
                "relayer_http_kms_exclusions_total",
                &[("kms_node", node), ("code", code)],
            )
        };
        let probes = |node: &str, outcome: &str| {
            value(
                &metrics,
                "relayer_http_kms_probes_total",
                &[("kms_node", node), ("outcome", outcome)],
            )
        };
        let floor_hits = |node: &str| {
            value(
                &metrics,
                "relayer_http_kms_floor_hits_total",
                &[("kms_node", node)],
            )
        };
        // Every node starts in use, with its alerting counter at 0.
        assert_eq!(
            (gauge("node-12"), floor_hits("node-12")),
            (Some(0), Some(0))
        );
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        assert_eq!(gauge("node-12"), Some(1));
        assert_eq!(exclusions("node-12", "cancelled"), Some(1));
        let overloaded = Verdict::Unhealthy {
            last_code: Some(ErrorCode::Overloaded),
        };
        breakers.on_call_outcome("node-11", &overloaded, Instant::now(), generation);
        assert_eq!(exclusions("node-11", "overloaded"), Some(1));
        // The floor: counted, the node stays in use.
        breakers.on_call_outcome("node-10", &unhealthy(), Instant::now(), generation);
        assert_eq!(
            (gauge("node-10"), floor_hits("node-10")),
            (Some(0), Some(1))
        );
        advance(WINDOW).await;
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_probe_outcome("node-12", &Verdict::Answered, Instant::now(), generation);
        assert_eq!(
            (gauge("node-12"), probes("node-12", "succeeded")),
            (Some(0), Some(1))
        );
        breakers.on_probe_outcome("node-11", &unhealthy(), Instant::now(), generation);
        assert_eq!(
            (gauge("node-11"), probes("node-11", "failed")),
            (Some(1), Some(1))
        );
        assert_eq!(exclusions("node-11", "cancelled"), Some(1));
        advance(WINDOW).await;
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_probe_outcome("node-11", &Verdict::Ignored, Instant::now(), generation);
        assert_eq!(
            (gauge("node-11"), probes("node-11", "cancelled")),
            (Some(1), Some(1))
        );
        breakers.reset();
        assert_eq!(gauge("node-11"), Some(0));
    }

    fn unhealthy() -> Verdict {
        Verdict::Unhealthy { last_code: None }
    }

    /// A finished call with the given result and last answer.
    fn finished(result: Result<(), CallError>, last_code: Option<ErrorCode>) -> CallResult<()> {
        CallResult {
            node: "node-0".to_owned(),
            attempts: 1,
            retries: Retries::default(),
            elapsed: Duration::ZERO,
            last_code,
            result,
        }
    }

    fn api(code: ErrorCode) -> AttemptError {
        AttemptError::Api {
            status: code.http_status(),
            error: ErrorResponse::new(code, "test", None),
        }
    }

    fn included(breakers: &CircuitBreakers) -> Vec<usize> {
        breakers.select(Instant::now()).included
    }

    #[tokio::test(start_paused = true)]
    async fn select_with_nothing_open_includes_every_node_and_probes_none() {
        let selection = breakers().select(Instant::now());
        assert_eq!(selection.included, (0..N).collect::<Vec<_>>());
        assert!(selection.probes.is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn an_unhealthy_outcome_opens_the_node_for_exactly_the_window() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        assert_eq!(included(&breakers).len(), 12);
        assert_eq!(breakers.excluded_names(), vec!["node-12".to_owned()]);
        advance(WINDOW - Duration::from_millis(1)).await;
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.included.len(), 12);
        assert!(selection.probes.is_empty());
        advance(Duration::from_millis(1)).await;
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.included.len(), 12);
        assert_eq!(selection.probes, vec![12]);
        // Probing: neither included nor probed again until the probe reports.
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.included.len(), 12);
        assert!(selection.probes.is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn the_floor_keeps_the_third_unhealthy_node() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        for node in ["node-10", "node-11"] {
            breakers.on_call_outcome(node, &unhealthy(), Instant::now(), generation);
        }
        assert_eq!(included(&breakers).len(), 11);
        breakers.on_call_outcome("node-9", &unhealthy(), Instant::now(), generation);
        assert!(included(&breakers).contains(&9), "kept by the floor");
        // Probing nodes are out too: the floor still holds while the two probes run.
        advance(WINDOW).await;
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.probes, vec![10, 11]);
        breakers.on_call_outcome("node-9", &unhealthy(), Instant::now(), selection.generation);
        assert!(included(&breakers).contains(&9));
    }

    #[tokio::test(start_paused = true)]
    async fn a_probe_that_answers_closes_the_node_and_one_that_fails_reopens_it() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        advance(WINDOW).await;
        assert_eq!(breakers.select(Instant::now()).probes, vec![12]);
        breakers.on_probe_outcome("node-12", &Verdict::Answered, Instant::now(), generation);
        assert_eq!(included(&breakers).len(), 13);

        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        advance(WINDOW).await;
        assert_eq!(breakers.select(Instant::now()).probes, vec![12]);
        breakers.on_probe_outcome("node-12", &unhealthy(), Instant::now(), generation);
        assert_eq!(included(&breakers).len(), 12);
        advance(WINDOW - Duration::from_millis(1)).await;
        assert!(breakers.select(Instant::now()).probes.is_empty());
        advance(Duration::from_millis(1)).await;
        assert_eq!(breakers.select(Instant::now()).probes, vec![12]);
    }

    #[tokio::test(start_paused = true)]
    async fn an_ignored_probe_is_probed_again_by_the_next_select() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        advance(WINDOW).await;
        assert_eq!(breakers.select(Instant::now()).probes, vec![12]);
        breakers.on_probe_outcome("node-12", &Verdict::Ignored, Instant::now(), generation);
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.included.len(), 12);
        assert_eq!(selection.probes, vec![12]);
    }

    #[tokio::test(start_paused = true)]
    async fn reset_closes_everything_and_drops_stale_outcomes() {
        let breakers = breakers();
        let stale = breakers.select(Instant::now()).generation;
        for node in ["node-11", "node-12"] {
            breakers.on_call_outcome(node, &unhealthy(), Instant::now(), stale);
        }
        assert_eq!(breakers.reset(), 2);
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.included.len(), 13);
        assert!(selection.probes.is_empty());
        assert_ne!(selection.generation, stale);
        breakers.on_call_outcome("node-10", &unhealthy(), Instant::now(), stale);
        breakers.on_probe_outcome("node-12", &unhealthy(), Instant::now(), stale);
        assert_eq!(included(&breakers).len(), 13, "stale outcomes are dropped");
        breakers.on_call_outcome(
            "node-10",
            &unhealthy(),
            Instant::now(),
            selection.generation,
        );
        assert_eq!(included(&breakers).len(), 12, "current outcomes apply");
    }

    #[tokio::test(start_paused = true)]
    async fn min_nodes_equal_to_n_never_excludes() {
        let breakers = CircuitBreakers::new(true, names(), N, WINDOW, metrics());
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-0", &unhealthy(), Instant::now(), generation);
        assert_eq!(included(&breakers).len(), 13);
    }

    #[tokio::test(start_paused = true)]
    async fn an_outcome_for_a_node_already_open_is_ignored() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        advance(Duration::from_secs(10)).await;
        // An aggregation that selected node 12 before it opened reports late: the window is not extended.
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        breakers.on_call_outcome("node-12", &Verdict::Answered, Instant::now(), generation);
        advance(Duration::from_secs(20)).await;
        assert_eq!(breakers.select(Instant::now()).probes, vec![12]);
    }

    #[tokio::test(start_paused = true)]
    async fn disabled_breakers_change_nothing() {
        let breakers = CircuitBreakers::new(false, names(), MIN_NODES, WINDOW, metrics());
        assert!(!breakers.is_enabled());
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        breakers.on_probe_outcome("node-12", &unhealthy(), Instant::now(), generation);
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.included.len(), 13);
        assert!(selection.probes.is_empty());
        assert!(breakers.excluded_names().is_empty());
        assert_eq!(breakers.reset(), 0);
    }

    #[test]
    fn verdict_covers_every_code_and_variant() {
        let blaming = [
            ErrorCode::UpstreamTransient,
            ErrorCode::Overloaded,
            ErrorCode::Timeout,
        ];
        for code in connector_codes() {
            let verdict = Verdict::of(
                &finished(Err(CallError::Failed(api(code))), Some(code)),
                false,
            );
            if blaming.contains(&code) {
                assert_eq!(
                    verdict,
                    Verdict::Unhealthy {
                        last_code: Some(code)
                    },
                    "{code:?}"
                );
            } else {
                assert_eq!(verdict, Verdict::Answered, "{code:?}");
            }
        }
        let failed = |failure: AttemptError| {
            let last_code = Some(failure.code());
            Verdict::of(&finished(Err(CallError::Failed(failure)), last_code), false)
        };
        let transient = Verdict::Unhealthy {
            last_code: Some(ErrorCode::UpstreamTransient),
        };
        assert_eq!(failed(AttemptError::Status(502)), transient);
        assert_eq!(failed(AttemptError::Status(404)), transient);
        assert_eq!(failed(AttemptError::Status(429)), Verdict::Answered);
        assert_eq!(failed(AttemptError::Status(401)), Verdict::Answered);
        assert_eq!(
            failed(AttemptError::Transport("refused".to_owned())),
            transient
        );
        assert_eq!(
            failed(AttemptError::Body {
                kind: "syntax",
                len: 3
            }),
            Verdict::Answered
        );
        assert_eq!(failed(AttemptError::TooLarge(9)), Verdict::Answered);
        assert_eq!(
            Verdict::of(
                &finished(
                    Err(CallError::Failed(AttemptError::Signer("kms".to_owned()))),
                    None
                ),
                false
            ),
            Verdict::Ignored
        );
        assert_eq!(
            Verdict::of(&finished(Ok(()), None), false),
            Verdict::Answered
        );
        assert_eq!(
            Verdict::of(&finished(Err(CallError::Cancelled), None), false),
            Verdict::Ignored
        );
        let cut = |last_code: Option<ErrorCode>| {
            Verdict::of(&finished(Err(CallError::Cancelled), last_code), true)
        };
        assert_eq!(cut(None), Verdict::Unhealthy { last_code: None });
        assert_eq!(
            cut(Some(ErrorCode::Overloaded)),
            Verdict::Unhealthy {
                last_code: Some(ErrorCode::Overloaded)
            }
        );
        assert_eq!(cut(Some(ErrorCode::CiphertextNotFound)), Verdict::Answered);
        assert_eq!(cut(Some(ErrorCode::RateLimited)), Verdict::Answered);
    }

    #[tokio::test(start_paused = true)]
    async fn two_outcomes_at_the_same_instant_with_room_for_one_open_exactly_one() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-11", &unhealthy(), Instant::now(), generation);
        let now = Instant::now();
        breakers.on_call_outcome("node-10", &unhealthy(), now, generation);
        breakers.on_call_outcome("node-12", &unhealthy(), now, generation);
        assert_eq!(included(&breakers).len(), 11);
    }

    #[test]
    fn concurrent_outcomes_from_threads_never_cross_the_floor() {
        // Thirteen threads report their node unhealthy at once, twice over; exactly two nodes open, every time.
        let breakers = breakers();
        for _ in 0..2 {
            let generation = breakers.select(Instant::now()).generation;
            std::thread::scope(|scope| {
                for name in names() {
                    let breakers = &breakers;
                    scope.spawn(move || {
                        breakers.on_call_outcome(&name, &unhealthy(), Instant::now(), generation);
                    });
                }
            });
            assert_eq!(breakers.excluded_names().len(), N - MIN_NODES);
            assert_eq!(breakers.select(Instant::now()).included.len(), MIN_NODES);
        }
        // Thirteen threads select at once with every open node expired: each node is probed exactly once.
        std::thread::sleep(Duration::from_millis(1));
        let later = Instant::now() + WINDOW;
        let probes: Vec<usize> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..N)
                .map(|_| scope.spawn(|| breakers.select(later).probes))
                .collect();
            handles
                .into_iter()
                .flat_map(|h| h.join().unwrap())
                .collect()
        });
        let mut probes = probes;
        probes.sort_unstable();
        assert_eq!(probes.len(), N - MIN_NODES);
        probes.dedup();
        assert_eq!(probes.len(), N - MIN_NODES, "one probe per node");
    }

    #[tokio::test(start_paused = true)]
    async fn two_probes_answering_at_the_same_instant_close_both_nodes() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        for node in ["node-11", "node-12"] {
            breakers.on_call_outcome(node, &unhealthy(), Instant::now(), generation);
        }
        advance(WINDOW).await;
        let selection = breakers.select(Instant::now());
        assert_eq!(selection.probes, vec![11, 12]);
        let now = Instant::now();
        for node in ["node-11", "node-12"] {
            breakers.on_probe_outcome(node, &Verdict::Answered, now, selection.generation);
        }
        assert_eq!(included(&breakers).len(), 13);
    }

    #[tokio::test(start_paused = true)]
    async fn two_selects_at_the_same_instant_send_one_probe() {
        let breakers = breakers();
        let generation = breakers.select(Instant::now()).generation;
        breakers.on_call_outcome("node-12", &unhealthy(), Instant::now(), generation);
        advance(WINDOW).await;
        let now = Instant::now();
        assert_eq!(breakers.select(now).probes, vec![12]);
        assert!(breakers.select(now).probes.is_empty());
    }
}
