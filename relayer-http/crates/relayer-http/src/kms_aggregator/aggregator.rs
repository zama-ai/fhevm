//! Fan out one request to every node and decide under one deadline.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use kms_connector_api::ErrorCode;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep_until};
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, debug, error, instrument, warn};

use super::breaker::Verdict;
use super::call::{CallError, Caller};
use super::client::Endpoint;
use super::flows::Flow;
use crate::metrics::PANIC_CODE;

/// Where a node task panicked (`file:line:column`), filled by the panic hook and read from the task's `JoinError`.
type PanicSlot = Arc<Mutex<Option<String>>>;

tokio::task_local! {
    /// The panic slot of the node task being polled: one per task, set when the aggregation spawns it.
    static NODE_PANIC: PanicSlot;
}

/// The panic hook's part: keeps `location` for the `node task panicked` line, so a node task's panic is one ERROR
/// line. False outside a node task: the hook logs the panic itself.
pub fn note_node_panic(location: Option<String>) -> bool {
    NODE_PANIC
        .try_with(|slot| {
            if let Ok(mut noted) = slot.lock() {
                *noted = location;
            }
        })
        .is_ok()
}

/// Why the threshold was not met. `dominant` = most frequent connector error among the failed calls.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum AggregationError {
    /// The deadline (`call.timeout`) passed with fewer than `threshold` counted responses.
    #[error("deadline reached with {counted} of {threshold} responses")]
    Timeout {
        counted: usize,
        threshold: usize,
        rejected: usize,
        dominant: Option<ErrorCode>,
    },
    /// Every node finished, or too many failed for the threshold to be reachable.
    #[error("threshold not reached: {counted} of {threshold} responses")]
    ThresholdNotReached {
        counted: usize,
        threshold: usize,
        rejected: usize,
        dominant: Option<ErrorCode>,
    },
    /// A bug: serialisation or an impossible state. Never expected in production.
    #[error("internal: {0}")]
    Internal(String),
}

impl AggregationError {
    /// What a handler maps to a user-facing error (e.g. `acl_denied`, `ciphertext_not_found`).
    pub fn dominant(&self) -> Option<ErrorCode> {
        match self {
            Self::Timeout { dominant, .. } | Self::ThresholdNotReached { dominant, .. } => {
                *dominant
            }
            Self::Internal(_) => None,
        }
    }
}

/// What one aggregation counted: the request's summary line reports it, whatever the verdict.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tally {
    /// Responses that count toward the threshold.
    pub counted: usize,
    pub threshold: usize,
    /// Nodes whose call failed.
    pub failed: Vec<String>,
    /// Nodes whose response was refused by a check.
    pub rejected: Vec<String>,
    /// Nodes whose call was cut: at the deadline, or by fail fast.
    pub cancelled: Vec<String>,
    /// Nodes whose call has not ended; if the run is dropped they are cut with it.
    pub pending: Vec<String>,
    /// When the counted responses first reached the threshold; `None` if they never did.
    pub threshold_reached: Option<Instant>,
}

/// One per flow. Cheap to share behind an `Arc`.
pub struct Aggregator<F: Flow> {
    caller: Arc<Caller>,
    threshold: usize,
    checks: F::Checks,
    _flow: PhantomData<F>,
}

impl<F: Flow> Aggregator<F> {
    /// `checks`: the flow's optional checks, from the configuration.
    pub fn new(caller: Arc<Caller>, threshold: usize, checks: F::Checks) -> Self {
        Self {
            caller,
            threshold,
            checks,
            _flow: PhantomData,
        }
    }

    /// Posts `request` to every node and answers when all nodes have finished or at the deadline
    /// (`call.timeout`). `request_id` is the relayer's own correlation id, never generated here; `tally` is
    /// filled in whatever the verdict.
    #[instrument(name = "aggregation", skip_all, fields(request_id = %request_id))]
    pub async fn run(
        &self,
        request_id: &str,
        request: F::Request,
        tally: &mut Tally,
    ) -> Result<F::Output, AggregationError> {
        let threshold = self.threshold;
        let timeout = self.caller.timeout();
        let started = Instant::now();
        // `timeout` is validated to be at most 60 s (see `Caller::new`): this cannot overflow.
        let deadline = started + timeout;
        let cancel = CancellationToken::new();
        let body = serde_json::to_vec(&request)
            .map(Bytes::from)
            .map_err(|e| AggregationError::Internal(format!("serialise request: {e}")))?;
        // The circuit breakers decide which nodes this aggregation calls, and which open ones it probes beside
        // itself (`breaker.rs`); with the breakers off that is every node and no probe.
        let breakers = self.caller.circuit_breakers();
        let selection = breakers.select(started);
        let included: Vec<&Endpoint> = self
            .caller
            .endpoints()
            .iter()
            .enumerate()
            .filter(|(index, _)| selection.included.contains(index))
            .map(|(_, endpoint)| endpoint)
            .collect();
        let nodes = included.len();
        tally.threshold = threshold;
        tally.pending = included
            .iter()
            .map(|endpoint| endpoint.name.clone())
            .collect();
        // The fan-out, the threshold, the retry policy and the breakers' choice, so one request's log is
        // self-describing. The node lists are rendered as text, like the waits.
        let call = self.caller.call_config();
        let names = |indices: &[usize]| -> String {
            let names: Vec<&str> = self
                .caller
                .endpoints()
                .iter()
                .enumerate()
                .filter(|(index, _)| indices.contains(index))
                .map(|(_, endpoint)| endpoint.name.as_str())
                .collect();
            format!("{names:?}")
        };
        let excluded: Vec<usize> = (0..self.caller.endpoints().len())
            .filter(|index| !selection.included.contains(index))
            .collect();
        debug!(
            nodes,
            threshold,
            timeout_ms = timeout.as_millis() as u64,
            retries_waits_ms = self.caller.retries_waits_ms(),
            rate_limit_max = call.rate_limit.max_retries,
            rate_limit_initial_delay_ms = call.rate_limit.initial_delay.as_millis() as u64,
            circuit_breaker = if breakers.is_enabled() { "on" } else { "off" },
            excluded_nodes = names(&excluded),
            probed_nodes = names(&selection.probes),
            "aggregation started"
        );

        // A probe is the same request sent to a node whose exclusion window has elapsed; this aggregation never
        // waits for it, its outcome alone decides whether the next ones call that node again.
        for &index in &selection.probes {
            self.caller.probe::<F::Response>(
                F::NAME,
                index,
                F::ROUTE,
                body.clone(),
                deadline,
                selection.generation,
            );
        }

        // One task per node; all of them share the body bytes, the deadline and the cancellation token.
        let mut pending = JoinSet::new();
        // A task that panics returns no result: its id says which node it was.
        let mut task_nodes: HashMap<tokio::task::Id, (String, PanicSlot)> = HashMap::new();
        for endpoint in included {
            // The task must own its endpoint: it outlives this borrow.
            let endpoint = endpoint.clone();
            let name = endpoint.name.clone();
            let (caller, body, cancel) = (self.caller.clone(), body.clone(), cancel.clone());
            // The task carries the `aggregation` span: its lines have the request's id.
            let slot = PanicSlot::default();
            let task = pending.spawn(
                NODE_PANIC.scope(
                    slot.clone(),
                    async move {
                        caller
                            .call::<F::Response>(
                                F::NAME,
                                &endpoint,
                                F::ROUTE,
                                body,
                                deadline,
                                cancel,
                            )
                            .await
                    }
                    .in_current_span(),
                ),
            );
            task_nodes.insert(task.id(), (name, slot));
        }

        let mut accepted: Vec<F::Response> = Vec::with_capacity(nodes);
        let mut codes: Vec<ErrorCode> = Vec::new();
        let mut deadline_hit = false;

        // Three ways out of the loop: (a) every node finished, `join_next` returns None; (b) the deadline
        // fires: cancel the token, the pending calls come back `Cancelled` at once and the set drains;
        // (c) fail fast: same as (b). The verdict is taken once, after the loop.
        // `biased`: the deadline arm is checked first, so a burst of results cannot starve it.
        loop {
            let call = tokio::select! {
                biased;
                () = sleep_until(deadline), if !cancel.is_cancelled() => {
                    deadline_hit = true;
                    cancel.cancel();
                    debug!(pending = pending.len(), "deadline reached");
                    continue;
                }
                joined = pending.join_next() => match joined {
                    None => break,
                    Some(Ok(call)) => Some(call),
                    // A panic inside a node task: the node is counted as failed, never propagated. One ERROR line
                    // with the location the hook noted; the panic message is not logged: it is the relayer's own
                    // text, but nothing proves it quotes no input.
                    Some(Err(e)) => {
                        let (node, slot) = task_nodes.remove(&e.id()).unzip();
                        if let Some(node) = &node {
                            tally.pending.retain(|name| name != node);
                            tally.failed.push(node.clone());
                            self.caller.metrics().attempt_error(node, F::NAME, PANIC_CODE);
                        }
                        let location = slot.and_then(|slot| slot.lock().ok().and_then(|mut noted| noted.take()));
                        error!(kms_node = node.as_deref(), location = location.as_deref(), "node task panicked");
                        None
                    }
                },
            };
            // A panicked task has no result to account for, but it still counts against the threshold.
            if let Some(call) = call {
                tally.pending.retain(|name| *name != call.node);
                // What this outcome says about the node's health, for the next aggregations.
                breakers.on_call_outcome(
                    &call.node,
                    &Verdict::of(&call, deadline_hit),
                    Instant::now(),
                    selection.generation,
                );
                let elapsed_ms = call.elapsed.as_millis() as u64;
                match call.result {
                    Ok(response) => match F::check(&self.checks, &request, &accepted, &response) {
                        Ok(()) => {
                            accepted.push(response);
                            let counted = F::counted(&self.checks, &accepted);
                            if counted >= threshold && tally.threshold_reached.is_none() {
                                tally.threshold_reached = Some(Instant::now());
                            }
                            debug!(
                                kms_node = %call.node,
                                attempts = call.attempts,
                                elapsed_ms,
                                counted,
                                "response accepted"
                            );
                        }
                        Err(reason) => {
                            tally.rejected.push(call.node.clone());
                            warn!(kms_node = %call.node, elapsed_ms, error = %reason, "response rejected");
                        }
                    },
                    Err(CallError::Cancelled) => {
                        tally.cancelled.push(call.node.clone());
                        // Still running at the deadline: too slow, or hung. Otherwise fail fast cut it. The
                        // node's last answer (`none` when it never answered) says whether the node or the
                        // request was slow.
                        let cause = if deadline_hit {
                            "deadline"
                        } else {
                            "fail_fast"
                        };
                        warn!(
                            kms_node = %call.node,
                            attempts = call.attempts,
                            elapsed_ms,
                            cause,
                            last_code = call.last_code.map_or("none", ErrorCode::as_str),
                            "call cut"
                        );
                    }
                    Err(CallError::Failed(failure)) => {
                        tally.failed.push(call.node.clone());
                        codes.push(failure.code());
                        // The node's outcome after its last attempt, with how much of each retry budget it used.
                        warn!(
                            kms_node = %call.node,
                            attempts = call.attempts,
                            transient_retries = call.retries.transient,
                            rate_limit_retries = call.retries.rate_limit,
                            elapsed_ms,
                            error = %failure.digest(),
                            "call failed"
                        );
                    }
                }
            }
            // Fail fast: even if every pending node answered we could not reach the threshold.
            if !cancel.is_cancelled()
                && F::counted(&self.checks, &accepted) + pending.len() < threshold
            {
                cancel.cancel();
                debug!(pending = pending.len(), "threshold unreachable");
            }
        }

        // The answer waited for the remaining nodes after the threshold: the latency added for safety.
        if let Some(reached) = tally.threshold_reached {
            self.caller
                .metrics()
                .wait_after_threshold(F::NAME, reached.elapsed());
        }
        let counted = F::counted(&self.checks, &accepted);
        tally.counted = counted;
        if counted >= threshold {
            return F::output(&self.checks, accepted).ok_or_else(|| {
                AggregationError::Internal("no output from the accepted responses".to_owned())
            });
        }
        let dominant = dominant(&codes);
        let error = if deadline_hit {
            AggregationError::Timeout {
                counted,
                threshold,
                rejected: tally.rejected.len(),
                dominant,
            }
        } else {
            AggregationError::ThresholdNotReached {
                counted,
                threshold,
                rejected: tally.rejected.len(),
                dominant,
            }
        };
        Err(error)
    }
}

/// Most frequent code (ties: the one seen last). `ErrorCode` is not `Hash`, hence the linear count.
fn dominant(codes: &[ErrorCode]) -> Option<ErrorCode> {
    codes
        .iter()
        .copied()
        .max_by_key(|code| codes.iter().filter(|c| *c == code).count())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use kms_connector_api::{PublicDecryptionRequest, UserDecryptionRequest};

    use tokio::time::sleep;

    use super::*;
    use crate::kms_aggregator::config::{CallConfig, UserChecks};
    use crate::kms_aggregator::flows::public_decrypt::PublicDecrypt;
    use crate::kms_aggregator::flows::user_decrypt::UserDecrypt;
    use crate::kms_aggregator::mock::{
        Fixed, MockClient, PUBLIC_REQUEST_JSON, Reply, USER_REQUEST_JSON,
    };
    use crate::logging::capture::capture_runs;
    use crate::metrics::Metrics;
    use crate::metrics::tests::{histogram_sum, total, value};

    const OK: Reply = Reply::Fixed(Fixed::Ok);
    const HANG: Reply = Reply::Fixed(Fixed::Hang);
    const DIVERGENT: Reply = Reply::Fixed(Fixed::Divergent);
    const ACL: Reply = Reply::Error(ErrorCode::AclDenied);
    const PANIC: Reply = Reply::Fixed(Fixed::Panic);
    const BAD_SIGNATURE: Reply = Reply::Fixed(Fixed::BadSignature);
    const NOT_FOUND: Reply = Reply::Error(ErrorCode::CiphertextNotFound);
    const MALFORMED: Reply = Reply::Error(ErrorCode::Malformed);

    fn ms(v: u64) -> Duration {
        Duration::from_millis(v)
    }

    fn secs(v: u64) -> Duration {
        Duration::from_secs(v)
    }

    /// Node groups → one script per node, in order.
    fn nodes(groups: &[(usize, Reply)]) -> Vec<Vec<Reply>> {
        groups
            .iter()
            .flat_map(|(count, reply)| std::iter::repeat_n(vec![*reply], *count))
            .collect()
    }

    fn user_request() -> UserDecryptionRequest {
        serde_json::from_str(USER_REQUEST_JSON).unwrap()
    }

    fn public_request() -> PublicDecryptionRequest {
        serde_json::from_str(PUBLIC_REQUEST_JSON).unwrap()
    }

    /// 13 mock nodes, 10 ms per attempt, 5 s deadline, no retries, the process token given.
    fn aggregator<F: Flow>(
        groups: &[(usize, Reply)],
        threshold: usize,
        checks: F::Checks,
    ) -> (Arc<MockClient>, Aggregator<F>) {
        let mock = MockClient::new(nodes(groups), ms(10));
        let caller = mock.clone().caller(Duration::from_secs(5), 0);
        (mock, Aggregator::new(caller, threshold, checks))
    }

    fn user(groups: &[(usize, Reply)], threshold: usize) -> Aggregator<UserDecrypt> {
        aggregator(groups, threshold, UserChecks::default()).1
    }

    fn public(groups: &[(usize, Reply)], threshold: usize) -> Aggregator<PublicDecrypt> {
        aggregator(groups, threshold, ()).1
    }

    #[tokio::test(start_paused = true)]
    async fn all_nodes_ok_answers_when_the_last_one_finishes() {
        let started = Instant::now();
        let output = user(&[(13, OK)], 9)
            .run("req-1", user_request(), &mut Tally::default())
            .await
            .unwrap();
        assert_eq!(output.result.len(), 13);
        assert_eq!(started.elapsed(), ms(10));
    }

    /// The lines of one aggregation over `groups` (user decrypt, no retries), until all of `expected` were logged.
    async fn logged(
        groups: &[(usize, Reply)],
        threshold: usize,
        expected: &[&str],
    ) -> Vec<serde_json::Value> {
        capture_runs(expected, || async {
            let mut tally = Tally::default();
            let _ = user(groups, threshold)
                .run("req-lines", user_request(), &mut tally)
                .await;
        })
        .await
    }

    fn lines_of<'a>(lines: &'a [serde_json::Value], message: &str) -> Vec<&'a serde_json::Value> {
        lines
            .iter()
            .filter(|l| l["fields"]["message"] == message)
            .collect()
    }

    #[tokio::test(start_paused = true)]
    async fn lines_carry_the_request_id_and_only_that_from_the_request() {
        let lines = logged(&[(1, ACL), (12, OK)], 9, &["attempt failed"]).await;
        for line in lines_of(&lines, "attempt failed") {
            let span = &line["span"];
            assert_eq!(span["name"], "aggregation");
            assert_eq!(span["request_id"], "req-lines");
            for field in ["flow", "decryption_id", "handles"] {
                assert!(span[field].is_null(), "{field} is on the summary line only");
            }
            assert_eq!(line["fields"]["code"], "acl_denied");
            assert_eq!(line["fields"]["kms_node"], "node-0");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_failed_call_a_rejected_response_and_a_cut_call_are_each_a_warn() {
        // Nodes 0-7 answer, 8 sends a bad signature, 9 is denied, 10 and 11 hang until the deadline.
        let expected = ["call failed", "response rejected", "call cut"];
        let lines = logged(
            &[(8, OK), (1, BAD_SIGNATURE), (1, ACL), (2, HANG)],
            8,
            &expected,
        )
        .await;
        for message in expected {
            for line in lines_of(&lines, message) {
                assert_eq!(line["level"], "WARN", "{message}");
                assert_eq!(line["span"]["request_id"], "req-lines");
            }
        }
        let failed = lines_of(&lines, "call failed");
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0]["fields"]["kms_node"], "node-9");
        assert!(
            failed[0]["fields"]["error"]
                .as_str()
                .unwrap()
                .starts_with("acl_denied")
        );
        let rejected = lines_of(&lines, "response rejected");
        assert_eq!(rejected.len(), 1);
        assert_eq!(rejected[0]["fields"]["kms_node"], "node-8");
        assert_eq!(
            rejected[0]["fields"]["error"],
            "signature is 64 bytes, expected 65"
        );
        let cut = lines_of(&lines, "call cut");
        let nodes: Vec<&str> = cut
            .iter()
            .map(|l| l["fields"]["kms_node"].as_str().unwrap())
            .collect();
        assert_eq!(nodes.len(), 2);
        assert!(nodes.contains(&"node-10") && nodes.contains(&"node-11"));
        for line in cut {
            assert_eq!(line["fields"]["cause"], "deadline");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn started_and_call_failed_lines_carry_the_retry_policy_and_counters() {
        // One node keeps answering 429: one rate-limit retry (the fixture has no transient budget), then it fails.
        let lines = logged(
            &[(1, Reply::Error(ErrorCode::RateLimited)), (12, OK)],
            9,
            &["aggregation started", "call failed"],
        )
        .await;
        let started = lines_of(&lines, "aggregation started");
        assert_eq!(started.len(), 1, "{started:?}");
        assert_eq!(started[0]["level"], "DEBUG");
        let fields = &started[0]["fields"];
        assert_eq!(
            (fields["nodes"].as_u64(), fields["threshold"].as_u64()),
            (Some(13), Some(9))
        );
        assert_eq!(fields["timeout_ms"], 5000);
        // The fixture has no transient budget; the list is rendered as text by the JSON layer.
        assert_eq!(fields["retries_waits_ms"], "[]");
        assert_eq!(
            (
                fields["rate_limit_max"].as_u64(),
                fields["rate_limit_initial_delay_ms"].as_u64()
            ),
            (Some(1), Some(1000))
        );
        let failed = lines_of(&lines, "call failed");
        assert_eq!(failed.len(), 1, "{failed:?}");
        assert_eq!(failed[0]["level"], "WARN");
        let fields = &failed[0]["fields"];
        assert_eq!(fields["kms_node"], "node-0");
        assert_eq!(
            (
                fields["attempts"].as_u64(),
                fields["transient_retries"].as_u64()
            ),
            (Some(2), Some(0))
        );
        assert_eq!(fields["rate_limit_retries"], 1);
    }

    #[tokio::test(start_paused = true)]
    async fn a_call_cut_by_fail_fast_is_a_warn_too() {
        // Five denials make 9 of 13 unreachable: the 8 hung calls are cut at once.
        let lines = logged(&[(5, ACL), (8, HANG)], 9, &["call cut"]).await;
        let cut = lines_of(&lines, "call cut");
        assert_eq!(cut.len(), 8);
        for line in cut {
            assert_eq!(line["level"], "WARN");
            assert_eq!(line["fields"]["cause"], "fail_fast");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn the_tally_names_the_nodes_behind_every_outcome() {
        let mut tally = Tally::default();
        let result = user(&[(8, OK), (1, BAD_SIGNATURE), (1, ACL), (2, HANG)], 8)
            .run("req-tally", user_request(), &mut tally)
            .await;
        assert!(result.is_ok());
        assert_eq!(tally.counted, 8);
        assert_eq!(tally.threshold, 8);
        assert_eq!(tally.rejected, ["node-8"]);
        assert_eq!(tally.failed, ["node-9"]);
        let mut cut = tally.cancelled.clone();
        cut.sort();
        assert_eq!(cut, ["node-10", "node-11"]);
    }

    /// Runs `groups` (user decrypt, threshold `threshold`) once, capturing its lines until `expected` were logged.
    async fn run_logged(
        groups: &[(usize, Reply)],
        threshold: usize,
        expected: &[&str],
    ) -> (Tally, Vec<serde_json::Value>, Arc<Metrics>) {
        let result = std::cell::RefCell::new(None);
        let lines = capture_runs(expected, || async {
            let (_, aggregator) =
                aggregator::<UserDecrypt>(groups, threshold, UserChecks::default());
            let mut tally = Tally::default();
            let _ = aggregator
                .run("req-panic", user_request(), &mut tally)
                .await;
            *result.borrow_mut() = Some((tally, aggregator.caller.metrics().clone()));
        })
        .await;
        let (tally, metrics) = result.into_inner().unwrap();
        (tally, lines, metrics)
    }

    #[tokio::test(start_paused = true)]
    async fn a_panicked_node_task_is_an_error_line_a_failed_node_and_a_metric() {
        let (tally, lines, metrics) =
            run_logged(&[(12, OK), (1, PANIC)], 9, &["node task panicked"]).await;
        assert_eq!(tally.failed, ["node-12"]);
        assert_eq!(tally.counted, 12);
        assert!(tally.pending.is_empty(), "{:?}", tally.pending);
        let line = lines_of(&lines, "node task panicked")[0];
        assert_eq!(line["level"], "ERROR");
        assert_eq!(line["fields"]["kms_node"], "node-12");
        assert_eq!(line["span"]["request_id"], "req-panic");
        // The panic text is the task's own; the line has a fixed message and no error field.
        assert!(line["fields"]["error"].is_null(), "{line}");
        let panic = [
            ("kms_node", "node-12"),
            ("flow", "user_decrypt"),
            ("code", "panic"),
        ];
        assert!(value(&metrics, "relayer_http_kms_attempt_errors_total", &panic) >= Some(1));
    }

    #[tokio::test]
    async fn a_node_panic_is_noted_only_inside_a_node_task() {
        assert!(!note_node_panic(Some("a.rs:1:1".to_owned())));
        let slot = PanicSlot::default();
        let noted = NODE_PANIC
            .scope(slot.clone(), async {
                note_node_panic(Some("a.rs:1:1".to_owned()))
            })
            .await;
        assert!(noted);
        assert_eq!(slot.lock().unwrap().as_deref(), Some("a.rs:1:1"));
    }

    #[tokio::test(start_paused = true)]
    async fn a_panicked_node_task_with_the_hook_is_one_error_line_with_its_location() {
        crate::logging::init_panic_hook();
        let (_, lines, _) = run_logged(&[(12, OK), (1, PANIC)], 9, &["node task panicked"]).await;
        let _ = std::panic::take_hook();
        let errors: Vec<_> = lines.iter().filter(|l| l["level"] == "ERROR").collect();
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert_eq!(errors[0]["fields"]["message"], "node task panicked");
        assert!(
            errors[0]["fields"]["location"].as_str().is_some(),
            "{}",
            errors[0]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_panic_that_makes_the_threshold_unreachable_ends_the_aggregation_at_once() {
        // 4 denials and 1 panic leave 8 hung nodes: 8 < 9, so they are cut now, not at the 5 s deadline.
        let started = Instant::now();
        let (aggregator, mut tally) = (
            aggregator::<UserDecrypt>(&[(4, ACL), (1, PANIC), (8, HANG)], 9, UserChecks::default())
                .1,
            Tally::default(),
        );
        let result = aggregator
            .run("req-panic-fast", user_request(), &mut tally)
            .await;
        assert!(matches!(
            result,
            Err(AggregationError::ThresholdNotReached { .. })
        ));
        assert!(started.elapsed() < ms(100), "{:?}", started.elapsed());
        assert_eq!(tally.cancelled.len(), 8);
    }

    #[tokio::test(start_paused = true)]
    async fn threshold_reached_two_nodes_hang_until_the_deadline() {
        let started = Instant::now();
        let aggregator = user(&[(9, OK), (2, HANG), (2, ACL)], 9);
        let mut tally = Tally::default();
        let output = aggregator
            .run("req-2", user_request(), &mut tally)
            .await
            .unwrap();
        assert_eq!(output.result.len(), 9);
        assert_eq!(started.elapsed(), Duration::from_secs(5));
        // The threshold was reached with the 9 answers at 10 ms; the answer waited for the hung nodes until 5 s.
        assert_eq!(tally.threshold_reached, Some(started + ms(10)));
        let metrics = aggregator.caller.metrics();
        let name = "relayer_http_kms_wait_after_threshold_seconds";
        assert_eq!(value(metrics, name, &[("flow", "user_decrypt")]), Some(1));
        assert!((histogram_sum(metrics, name) - 4.99).abs() < 1e-9);
    }

    #[tokio::test(start_paused = true)]
    async fn threshold_unreachable_fails_fast_and_cancels_the_rest() {
        let started = Instant::now();
        let (mock, aggregator) =
            aggregator::<UserDecrypt>(&[(5, ACL), (8, HANG)], 9, UserChecks::default());
        let err = aggregator
            .run("req-3", user_request(), &mut Tally::default())
            .await
            .unwrap_err();
        assert_eq!(
            err,
            AggregationError::ThresholdNotReached {
                counted: 0,
                threshold: 9,
                rejected: 0,
                dominant: Some(ErrorCode::AclDenied),
            }
        );
        assert_eq!(started.elapsed(), ms(10));
        assert_eq!((0..13).map(|n| mock.attempts(n)).sum::<u32>(), 13);
    }

    #[tokio::test(start_paused = true)]
    async fn deadline_before_threshold_is_a_timeout() {
        let started = Instant::now();
        let aggregator = user(&[(8, OK), (5, HANG)], 9);
        let mut tally = Tally::default();
        let err = aggregator
            .run("req-4", user_request(), &mut tally)
            .await
            .unwrap_err();
        // Never reached: no wait after the threshold is observed.
        assert_eq!(tally.threshold_reached, None);
        let name = "relayer_http_kms_wait_after_threshold_seconds";
        assert_eq!(total(aggregator.caller.metrics(), name), 0);
        assert_eq!(
            err,
            AggregationError::Timeout {
                counted: 8,
                threshold: 9,
                rejected: 0,
                dominant: None,
            }
        );
        assert_eq!(err.dominant(), None);
        assert_eq!(started.elapsed(), Duration::from_secs(5));
    }

    #[tokio::test(start_paused = true)]
    async fn dominant_error_is_reported() {
        let err = user(&[(4, OK), (6, NOT_FOUND), (3, ACL)], 9)
            .run("req-5", user_request(), &mut Tally::default())
            .await
            .unwrap_err();
        assert_eq!(err.dominant(), Some(ErrorCode::CiphertextNotFound));
        assert!(matches!(
            err,
            AggregationError::ThresholdNotReached { counted: 4, .. }
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn bad_and_duplicate_signatures_are_rejected_not_counted() {
        // Node 12 copies node 0's signature: whichever of the two is joined first counts, the other is a
        // duplicate. Node 11 has a 64-byte signature. 11 shares out of 13 nodes.
        let output = user(
            &[
                (11, OK),
                (1, Reply::Fixed(Fixed::BadSignature)),
                (1, Reply::Fixed(Fixed::Duplicate)),
            ],
            9,
        )
        .run("req-6", user_request(), &mut Tally::default())
        .await
        .unwrap();
        assert_eq!(output.result.len(), 11);
    }

    #[tokio::test(start_paused = true)]
    async fn rejected_responses_are_counted_in_the_error() {
        let err = user(
            &[(4, OK), (1, Reply::Fixed(Fixed::BadSignature)), (8, ACL)],
            9,
        )
        .run("req-6b", user_request(), &mut Tally::default())
        .await
        .unwrap_err();
        assert!(
            matches!(
                err,
                AggregationError::ThresholdNotReached {
                    counted: 4,
                    rejected: 1,
                    ..
                }
            ),
            "{err:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn decryption_id_mismatch_is_rejected_only_when_checked() {
        let groups = [(2, Reply::Fixed(Fixed::WrongId)), (11, OK)];
        let unchecked = user(&groups, 9)
            .run("req-6c", user_request(), &mut Tally::default())
            .await
            .unwrap();
        assert_eq!(unchecked.result.len(), 13);

        let checks = UserChecks {
            decryption_id_match: true,
            decryption_id_majority: false,
        };
        let (_, checked) = aggregator::<UserDecrypt>(&groups, 9, checks);
        assert_eq!(
            checked
                .run("req-6d", user_request(), &mut Tally::default())
                .await
                .unwrap()
                .result
                .len(),
            11
        );

        let checks = UserChecks {
            decryption_id_match: false,
            decryption_id_majority: true,
        };
        let (_, majority) = aggregator::<UserDecrypt>(&groups, 9, checks);
        assert_eq!(
            majority
                .run("req-6e", user_request(), &mut Tally::default())
                .await
                .unwrap()
                .result
                .len(),
            11
        );
    }

    #[tokio::test(start_paused = true)]
    async fn public_majority_wins_and_only_its_signatures_are_returned() {
        let output = public(&[(8, OK), (3, DIVERGENT), (2, HANG)], 5)
            .run("req-7", public_request(), &mut Tally::default())
            .await
            .unwrap();
        assert_eq!(output.signatures.len(), 8);
        assert_eq!(
            output.decrypted_value,
            alloy::hex::encode(crate::kms_aggregator::mock::result(0))
        );
    }

    #[tokio::test(start_paused = true)]
    async fn public_split_vote_does_not_reach_the_threshold() {
        let err = public(&[(4, OK), (4, DIVERGENT), (5, ACL)], 5)
            .run("req-8", public_request(), &mut Tally::default())
            .await
            .unwrap_err();
        assert_eq!(
            err,
            AggregationError::ThresholdNotReached {
                counted: 4,
                threshold: 5,
                rejected: 0,
                dominant: Some(ErrorCode::AclDenied),
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn dropping_the_run_future_aborts_the_node_tasks() {
        let (mock, aggregator) = aggregator::<UserDecrypt>(&[(13, HANG)], 9, UserChecks::default());
        let mut tally = Tally::default();
        let run = aggregator.run("req-10", user_request(), &mut tally);
        tokio::select! {
            _ = run => panic!("hung nodes cannot finish"),
            () = tokio::time::sleep(ms(50)) => {}
        }
        // The node tasks held clones of the caller (and thus of the mock): once aborted they are gone.
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }
        assert_eq!((0..13).map(|n| mock.attempts(n)).sum::<u32>(), 13);
        assert_eq!(Arc::strong_count(&mock), 2, "mock + caller only");
    }

    #[test]
    fn dominant_picks_the_most_frequent_code() {
        use ErrorCode::*;
        assert_eq!(dominant(&[]), None);
        assert_eq!(dominant(&[AclDenied]), Some(AclDenied));
        assert_eq!(
            dominant(&[AclDenied, CiphertextNotFound, CiphertextNotFound]),
            Some(CiphertextNotFound)
        );
        assert_eq!(dominant(&[AclDenied, Timeout]), Some(Timeout));
    }

    #[test]
    fn aggregation_error_dominant_per_variant() {
        let some = Some(ErrorCode::Overloaded);
        assert_eq!(
            AggregationError::Timeout {
                counted: 1,
                threshold: 2,
                rejected: 0,
                dominant: some
            }
            .dominant(),
            some
        );
        assert_eq!(
            AggregationError::ThresholdNotReached {
                counted: 1,
                threshold: 2,
                rejected: 0,
                dominant: some
            }
            .dominant(),
            some
        );
        assert_eq!(AggregationError::Internal("x".into()).dominant(), None);
        assert_eq!(
            AggregationError::Timeout {
                counted: 7,
                threshold: 9,
                rejected: 0,
                dominant: None
            }
            .to_string(),
            "deadline reached with 7 of 9 responses"
        );
    }

    // Circuit breaker: sequential runs over one aggregator (`breaker.rs`; docs.md, Circuit breaker). 13 nodes
    // answering `groups`, 300 ms per attempt, the production retry policy under a 5 s deadline, user threshold
    // 9, `min_nodes 11`, `exclude_for 30s`: the issue's example.

    fn breaking(groups: &[(usize, Reply)]) -> (Arc<MockClient>, Aggregator<UserDecrypt>) {
        let mock = MockClient::new(nodes(groups), ms(300));
        let caller = mock
            .clone()
            .caller_with_breakers(CallConfig::default(), 9, 11, secs(30));
        (mock, Aggregator::new(caller, 9, UserChecks::default()))
    }

    /// One user-decrypt run: the shares returned (or the error) and how long it took.
    async fn run_once(
        aggregator: &Aggregator<UserDecrypt>,
    ) -> (Result<usize, AggregationError>, Duration) {
        let started = Instant::now();
        let result = aggregator
            .run("req-breaker", user_request(), &mut Tally::default())
            .await
            .map(|output| output.result.len());
        (result, started.elapsed())
    }

    fn excluded(aggregator: &Aggregator<UserDecrypt>) -> Vec<String> {
        aggregator.caller.circuit_breakers().excluded_names()
    }

    #[tokio::test(start_paused = true)]
    async fn the_example_table_reproduces() {
        let (mock, aggregator) = breaking(&[(12, OK), (1, HANG)]);
        // 1st request: node 12 is cut at the deadline with no answer. 5 s, 12 shares, then it is out.
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        assert_eq!(excluded(&aggregator), ["node-12"]);
        // Within the window: 12 nodes, healthy speed, node 12 not called.
        sleep(secs(1)).await;
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(300)));
        assert_eq!(mock.attempts(12), 1);
        // After the window: node 12 is probed beside the request, which does not wait for it.
        sleep(secs(30)).await;
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(300)));
        assert_eq!(mock.attempts(12), 2);
        // While that probe hangs: neither called nor probed again.
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(300)));
        assert_eq!(mock.attempts(12), 2);
        // The probe is cut at its deadline: out for another window, then probed again.
        sleep(secs(5)).await;
        assert_eq!(excluded(&aggregator), ["node-12"]);
        sleep(secs(30)).await;
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(300)));
        assert_eq!(mock.attempts(12), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn a_probe_that_answers_closes_the_node() {
        let (mock, aggregator) = breaking(&[(12, OK), (1, HANG)]);
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        mock.set_script(12, vec![OK]);
        sleep(secs(31)).await;
        // The probing request still returns 12 shares: the probe's share is observed, not counted.
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(300)));
        sleep(secs(1)).await;
        assert!(excluded(&aggregator).is_empty());
        assert_eq!(run_once(&aggregator).await, (Ok(13), ms(300)));
    }

    #[tokio::test(start_paused = true)]
    async fn three_hung_nodes_keep_the_floor() {
        let (_, aggregator) = breaking(&[(10, OK), (3, HANG)]);
        // Two are out, the third is kept: every run waits for the deadline with 10 shares, as without a breaker.
        for _ in 0..2 {
            assert_eq!(run_once(&aggregator).await, (Ok(10), secs(5)));
            assert_eq!(excluded(&aggregator).len(), 2);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn overloaded_after_the_retries_trips_and_rate_limited_does_not() {
        let (mock, aggregator) = breaking(&[(12, OK), (1, Reply::Error(ErrorCode::Overloaded))]);
        // Six attempts of 300 ms and 2.8 s of waits: the node spent the budget still overloaded, it is out.
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(6 * 300 + 2800)));
        assert_eq!(mock.attempts(12), 6);
        assert_eq!(excluded(&aggregator), ["node-12"]);

        let (mock, aggregator) = breaking(&[(12, OK), (1, Reply::Error(ErrorCode::RateLimited))]);
        // Our own send rate: one window, then the call fails; the node is up and stays in use.
        assert_eq!(run_once(&aggregator).await.0, Ok(12));
        assert!(excluded(&aggregator).is_empty());
        assert_eq!(run_once(&aggregator).await.0, Ok(12));
        assert_eq!(mock.attempts(12), 4);
    }

    #[tokio::test(start_paused = true)]
    async fn ciphertext_not_found_cut_at_the_deadline_does_not_trip() {
        let (mock, aggregator) = breaking(&[(12, OK), (1, NOT_FOUND)]);
        // Five legitimate answers, then the sixth attempt is still in flight at the deadline.
        mock.set_script(12, vec![NOT_FOUND; 5].into_iter().chain([HANG]).collect());
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        assert_eq!(mock.attempts(12), 6);
        assert!(excluded(&aggregator).is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn fail_fast_comes_earlier_with_a_reduced_fan_out() {
        // 13 nodes: four final failures leave 9, enough.
        let (_, aggregator) = breaking(&[(4, MALFORMED), (9, OK)]);
        assert_eq!(run_once(&aggregator).await, (Ok(9), ms(300)));
        // With node 12 out, the same four failures leave 8: fail fast at once, no deadline waited.
        let (mock, aggregator) = breaking(&[(12, OK), (1, HANG)]);
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        for node in 0..4 {
            mock.set_script(node, vec![MALFORMED]);
        }
        let (result, elapsed) = run_once(&aggregator).await;
        assert!(
            matches!(
                result,
                Err(AggregationError::ThresholdNotReached { counted: 8, .. })
            ),
            "{result:?}"
        );
        assert_eq!(elapsed, ms(300));
    }

    #[tokio::test(start_paused = true)]
    async fn reset_reincludes_for_the_next_run_only() {
        let (mock, aggregator) = breaking(&[(12, OK), (1, HANG)]);
        let aggregator = Arc::new(aggregator);
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        let in_flight = tokio::spawn({
            let aggregator = aggregator.clone();
            async move { run_once(&aggregator).await }
        });
        tokio::task::yield_now().await;
        assert_eq!(aggregator.caller.reset_circuit_breakers(), 1);
        // The run in flight keeps the 12 nodes it selected.
        assert_eq!(in_flight.await.unwrap(), (Ok(12), ms(300)));
        assert_eq!(mock.attempts(12), 1);
        // The next run calls node 12 again, and pays the deadline since it still hangs.
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        assert_eq!(mock.attempts(12), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn a_probe_survives_fail_fast_and_the_end_of_its_aggregation() {
        let (mock, aggregator) = breaking(&[(12, OK), (1, HANG)]);
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        sleep(secs(31)).await;
        // Five failures make the threshold unreachable: fail fast at 300 ms; the probe to node 12 goes on.
        for node in 0..5 {
            mock.set_script(node, vec![MALFORMED]);
        }
        let (result, elapsed) = run_once(&aggregator).await;
        assert!(matches!(
            result,
            Err(AggregationError::ThresholdNotReached { .. })
        ));
        assert_eq!(elapsed, ms(300));
        assert_eq!(mock.attempts(12), 2);
        // The probe is cut at the aggregation's deadline, not at its end: out for another window.
        sleep(secs(5)).await;
        assert_eq!(excluded(&aggregator), ["node-12"]);
        sleep(secs(24)).await;
        let _ = run_once(&aggregator).await;
        assert_eq!(mock.attempts(12), 2, "still inside the window");
        sleep(secs(6)).await;
        let _ = run_once(&aggregator).await;
        assert_eq!(mock.attempts(12), 3, "probed again after the window");
    }

    #[tokio::test(start_paused = true)]
    async fn breaker_lines_carry_the_request_id_and_their_fields() {
        let expected = [
            "node excluded",
            "node probed",
            "probe failed: node excluded",
        ];
        let lines = capture_runs(&expected, || async {
            let (_, aggregator) = breaking(&[(12, OK), (1, HANG)]);
            let _ = run_once(&aggregator).await;
            sleep(secs(31)).await;
            let _ = run_once(&aggregator).await;
            sleep(secs(6)).await;
        })
        .await;
        for message in expected {
            for line in lines_of(&lines, message) {
                assert_eq!(line["span"]["request_id"], "req-breaker", "{message}");
                assert_eq!(line["fields"]["kms_node"], "node-12", "{message}");
            }
        }
        let excluded = lines_of(&lines, "node excluded");
        assert_eq!(excluded.len(), 1);
        assert_eq!(excluded[0]["level"], "WARN");
        let fields = &excluded[0]["fields"];
        assert_eq!(fields["last_code"], "none");
        assert_eq!(fields["excluded_for_ms"], 30_000);
        assert_eq!(fields["closed_nodes"], 12);
        let probed = lines_of(&lines, "node probed");
        assert_eq!(probed.len(), 1);
        assert_eq!(probed[0]["level"], "DEBUG");
        assert!(probed[0]["fields"]["excluded_ms"].as_u64().unwrap() >= 30_000);
        let failed = lines_of(&lines, "probe failed: node excluded");
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0]["level"], "WARN");
        assert_eq!(failed[0]["fields"]["last_code"], "none");
        assert_eq!(failed[0]["fields"]["closed_nodes"], 12);
        // The second aggregation names what it left out and what it probed.
        let started = lines_of(&lines, "aggregation started");
        assert_eq!(started.len(), 2);
        assert_eq!(started[0]["fields"]["circuit_breaker"], "on");
        assert_eq!(started[0]["fields"]["excluded_nodes"], "[]");
        assert_eq!(started[1]["fields"]["nodes"], 12);
        assert_eq!(started[1]["fields"]["excluded_nodes"], "[\"node-12\"]");
        assert_eq!(started[1]["fields"]["probed_nodes"], "[\"node-12\"]");
    }

    #[tokio::test(start_paused = true)]
    async fn a_kept_node_logs_the_floor_and_an_answering_probe_logs_the_inclusion() {
        let lines = capture_runs(&["node kept: floor reached"], || async {
            let (_, aggregator) = breaking(&[(10, OK), (3, HANG)]);
            let _ = run_once(&aggregator).await;
        })
        .await;
        let kept = lines_of(&lines, "node kept: floor reached");
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0]["level"], "WARN");
        let fields = &kept[0]["fields"];
        assert_eq!(fields["min_nodes"], 11);
        assert_eq!(fields["closed_nodes"], 11);
        assert_eq!(fields["last_code"], "none");

        let lines = capture_runs(&["node included", "breakers reset"], || async {
            let (mock, aggregator) = breaking(&[(12, OK), (1, HANG)]);
            let _ = run_once(&aggregator).await;
            mock.set_script(12, vec![OK]);
            sleep(secs(31)).await;
            let _ = run_once(&aggregator).await;
            sleep(secs(1)).await;
            let _ = run_once(&aggregator).await;
            aggregator.caller.reset_circuit_breakers();
            aggregator.caller.reset_circuit_breakers();
        })
        .await;
        let included = lines_of(&lines, "node included");
        assert_eq!(included.len(), 1);
        assert_eq!(included[0]["level"], "INFO");
        assert_eq!(included[0]["span"]["request_id"], "req-breaker");
        let fields = &included[0]["fields"];
        assert_eq!(fields["kms_node"], "node-12");
        assert!(fields["excluded_ms"].as_u64().unwrap() >= 31_000);
        assert_eq!(fields["closed_nodes"], 13);
        // Reset is logged once per call, outside any span; nothing was out.
        let reset = lines_of(&lines, "breakers reset");
        assert_eq!(reset.len(), 2);
        assert_eq!(reset[0]["level"], "INFO");
        assert_eq!(reset[0]["fields"]["closed"], 0);
        assert!(reset[0]["span"].is_null());
    }

    #[tokio::test(start_paused = true)]
    async fn call_cut_carries_last_code() {
        let lines = capture_runs(&["call cut"], || async {
            let (mock, aggregator) = breaking(&[(11, OK), (1, HANG), (1, NOT_FOUND)]);
            mock.set_script(12, vec![NOT_FOUND; 5].into_iter().chain([HANG]).collect());
            let _ = run_once(&aggregator).await;
        })
        .await;
        let cut = lines_of(&lines, "call cut");
        assert_eq!(cut.len(), 2);
        let last_code = |node: &str| {
            cut.iter()
                .find(|l| l["fields"]["kms_node"] == node)
                .map(|l| l["fields"]["last_code"].clone())
                .unwrap()
        };
        assert_eq!(last_code("node-11"), "none");
        assert_eq!(last_code("node-12"), "ciphertext_not_found");
    }

    #[tokio::test(start_paused = true)]
    async fn a_panicking_probe_leaves_the_node_open_not_probing() {
        let (mock, aggregator) = breaking(&[(12, OK), (1, HANG)]);
        assert_eq!(run_once(&aggregator).await, (Ok(12), secs(5)));
        mock.set_script(12, vec![PANIC]);
        sleep(secs(31)).await;
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(300)));
        sleep(secs(1)).await;
        // The probe's task panicked: nothing is known, so the next request probes again.
        mock.set_script(12, vec![OK]);
        assert_eq!(run_once(&aggregator).await, (Ok(12), ms(300)));
        assert_eq!(mock.attempts(12), 3);
        sleep(secs(1)).await;
        assert_eq!(run_once(&aggregator).await, (Ok(13), ms(300)));
    }
}
