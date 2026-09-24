//! Fan out one request to every node and decide under one deadline.

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use kms_connector_api::ErrorCode;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep_until};
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, debug, error, info, instrument, warn};

use super::call::{CallError, Caller};
use super::flows::Flow;

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
    /// The process is shutting down.
    #[error("cancelled by shutdown")]
    Cancelled,
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
            Self::Cancelled | Self::Internal(_) => None,
        }
    }
}

/// One per flow. Cheap to share behind an `Arc`.
pub struct Aggregator<F: Flow> {
    caller: Arc<Caller>,
    threshold: usize,
    checks: F::Checks,
    shutdown: CancellationToken,
    _flow: PhantomData<F>,
}

impl<F: Flow> Aggregator<F> {
    /// `checks`: the flow's optional checks, from the configuration.
    pub fn new(
        caller: Arc<Caller>,
        threshold: usize,
        checks: F::Checks,
        shutdown: CancellationToken,
    ) -> Self {
        Self {
            caller,
            threshold,
            checks,
            shutdown,
            _flow: PhantomData,
        }
    }

    /// Posts `request` to every node and answers when all nodes have finished or at the deadline
    /// (`call.timeout`). `request_id` is the relayer's own correlation id, never generated here.
    #[instrument(
        name = "aggregation",
        skip_all,
        fields(
            flow = F::NAME,
            request_id = %request_id,
            decryption_id = %F::decryption_id(&request),
            handles = ?F::handles(&request),
        )
    )]
    pub async fn run(
        &self,
        request_id: &str,
        request: F::Request,
    ) -> Result<F::Output, AggregationError> {
        let nodes = self.caller.endpoints().len();
        let threshold = self.threshold;
        let timeout = self.caller.timeout();
        let started = Instant::now();
        // `timeout` is validated to be at most 60 s (see `Caller::new`): this cannot overflow.
        let deadline = started + timeout;
        let cancel = self.shutdown.child_token();
        let body = serde_json::to_vec(&request)
            .map(Bytes::from)
            .map_err(|e| AggregationError::Internal(format!("serialise request: {e}")))?;
        info!(
            nodes,
            threshold,
            timeout_ms = timeout.as_millis() as u64,
            "aggregation started"
        );

        // One task per node; all of them share the body bytes, the deadline and the cancellation token.
        let mut pending = JoinSet::new();
        for endpoint in self.caller.endpoints() {
            // The task must own its endpoint: it outlives this borrow.
            let endpoint = endpoint.clone();
            let (caller, body, cancel) = (self.caller.clone(), body.clone(), cancel.clone());
            // The task carries the `aggregation` span: its attempt lines have the request's identifiers.
            pending.spawn(
                async move {
                    caller
                        .call::<F::Response>(&endpoint, F::ROUTE, body, deadline, cancel)
                        .await
                }
                .in_current_span(),
            );
        }

        // Tallies for the final log line: the node names behind every outcome, so one line names the culprits.
        let mut accepted: Vec<F::Response> = Vec::with_capacity(nodes);
        let mut codes: Vec<ErrorCode> = Vec::new();
        let (mut rejected, mut failed, mut cancelled) = (
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
        );
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
                    Some(Ok(call)) => call,
                    // A panic inside a node task: counted as failed, never propagated.
                    Some(Err(e)) => {
                        failed.push("?".to_owned());
                        error!(error = %e, "node task failed");
                        continue;
                    }
                },
            };
            let elapsed_ms = call.elapsed.as_millis() as u64;
            match call.result {
                Ok(response) => match F::check(&self.checks, &request, &accepted, &response) {
                    Ok(()) => {
                        accepted.push(response);
                        info!(
                            node = %call.node,
                            attempts = call.attempts,
                            elapsed_ms,
                            counted = F::counted(&self.checks, &accepted),
                            "response accepted"
                        );
                    }
                    Err(reason) => {
                        rejected.push(call.node.clone());
                        warn!(node = %call.node, elapsed_ms, %reason, "response rejected");
                    }
                },
                Err(CallError::Cancelled) => {
                    cancelled.push(call.node.clone());
                    if deadline_hit {
                        // Still running at the deadline: too slow, or hung. Visible at the default level.
                        warn!(node = %call.node, attempts = call.attempts, elapsed_ms, "call cancelled at the deadline");
                    } else {
                        // Fail fast or shutdown: not the node's fault.
                        debug!(node = %call.node, attempts = call.attempts, elapsed_ms, "call cancelled");
                    }
                }
                Err(e) => {
                    failed.push(call.node.clone());
                    codes.extend(e.code());
                    warn!(node = %call.node, attempts = call.attempts, elapsed_ms, error = %e, "call failed");
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

        let counted = F::counted(&self.checks, &accepted);
        let dominant = dominant(&codes);
        let elapsed_ms = started.elapsed().as_millis() as u64;
        if counted >= threshold {
            info!(
                counted,
                accepted = accepted.len(),
                rejected = rejected.len(),
                rejected_nodes = ?rejected,
                failed = failed.len(),
                failed_nodes = ?failed,
                cancelled = cancelled.len(),
                cancelled_nodes = ?cancelled,
                deadline_hit,
                elapsed_ms,
                "aggregation succeeded"
            );
            return F::output(&self.checks, accepted).ok_or_else(|| {
                AggregationError::Internal("no output from the accepted responses".to_owned())
            });
        }
        warn!(
            counted,
            threshold,
            accepted = accepted.len(),
            rejected = rejected.len(),
            rejected_nodes = ?rejected,
            failed = failed.len(),
            failed_nodes = ?failed,
            cancelled = cancelled.len(),
            cancelled_nodes = ?cancelled,
            deadline_hit,
            dominant = dominant.map(ErrorCode::as_str),
            elapsed_ms,
            "aggregation failed"
        );
        let error = if self.shutdown.is_cancelled() {
            AggregationError::Cancelled
        } else if deadline_hit {
            AggregationError::Timeout {
                counted,
                threshold,
                rejected: rejected.len(),
                dominant,
            }
        } else {
            AggregationError::ThresholdNotReached {
                counted,
                threshold,
                rejected: rejected.len(),
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

    use super::*;
    use crate::kms_aggregator::config::UserChecks;
    use crate::kms_aggregator::flows::public_decrypt::PublicDecrypt;
    use crate::kms_aggregator::flows::user_decrypt::UserDecrypt;
    use crate::kms_aggregator::mock::{
        Fixed, MockClient, PUBLIC_REQUEST_JSON, Reply, USER_REQUEST_JSON,
    };

    const OK: Reply = Reply::Fixed(Fixed::Ok);
    const HANG: Reply = Reply::Fixed(Fixed::Hang);
    const DIVERGENT: Reply = Reply::Fixed(Fixed::Divergent);
    const ACL: Reply = Reply::Error(ErrorCode::AclDenied);
    const NOT_FOUND: Reply = Reply::Error(ErrorCode::CiphertextNotFound);

    fn ms(v: u64) -> Duration {
        Duration::from_millis(v)
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
        shutdown: CancellationToken,
    ) -> (Arc<MockClient>, Aggregator<F>) {
        let mock = MockClient::new(nodes(groups), ms(10));
        let caller = mock.clone().caller(Duration::from_secs(5), 0);
        (mock, Aggregator::new(caller, threshold, checks, shutdown))
    }

    fn user(groups: &[(usize, Reply)], threshold: usize) -> Aggregator<UserDecrypt> {
        aggregator(
            groups,
            threshold,
            UserChecks::default(),
            CancellationToken::new(),
        )
        .1
    }

    fn public(groups: &[(usize, Reply)], threshold: usize) -> Aggregator<PublicDecrypt> {
        aggregator(groups, threshold, (), CancellationToken::new()).1
    }

    #[tokio::test(start_paused = true)]
    async fn all_nodes_ok_answers_when_the_last_one_finishes() {
        let started = Instant::now();
        let output = user(&[(13, OK)], 9)
            .run("req-1", user_request())
            .await
            .unwrap();
        assert_eq!(output.result.len(), 13);
        assert_eq!(started.elapsed(), ms(10));
    }

    #[tokio::test(start_paused = true)]
    async fn attempt_lines_carry_the_request_identifiers() {
        let (_guard, sink) = crate::kms_aggregator::call::tests::capture_attempts().await;
        let request = user_request();
        let decryption_id = request.id().to_string();
        user(&[(1, ACL), (12, OK)], 9)
            .run("req-attempt", request)
            .await
            .unwrap();
        let lines = sink.lines("attempt failed");
        assert_eq!(lines.len(), 1, "{lines:?}");
        let span = &lines[0]["span"];
        assert_eq!(span["name"], "aggregation");
        assert_eq!(span["request_id"], "req-attempt");
        assert_eq!(span["flow"], "user_decrypt");
        assert_eq!(span["decryption_id"], decryption_id);
        assert_eq!(lines[0]["fields"]["code"], "acl_denied");
    }

    #[tokio::test(start_paused = true)]
    async fn threshold_reached_two_nodes_hang_until_the_deadline() {
        let started = Instant::now();
        let output = user(&[(9, OK), (2, HANG), (2, ACL)], 9)
            .run("req-2", user_request())
            .await
            .unwrap();
        assert_eq!(output.result.len(), 9);
        assert_eq!(started.elapsed(), Duration::from_secs(5));
    }

    #[tokio::test(start_paused = true)]
    async fn threshold_unreachable_fails_fast_and_cancels_the_rest() {
        let started = Instant::now();
        let (mock, aggregator) = aggregator::<UserDecrypt>(
            &[(5, ACL), (8, HANG)],
            9,
            UserChecks::default(),
            CancellationToken::new(),
        );
        let err = aggregator.run("req-3", user_request()).await.unwrap_err();
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
        let err = user(&[(8, OK), (5, HANG)], 9)
            .run("req-4", user_request())
            .await
            .unwrap_err();
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
            .run("req-5", user_request())
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
        .run("req-6", user_request())
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
        .run("req-6b", user_request())
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
            .run("req-6c", user_request())
            .await
            .unwrap();
        assert_eq!(unchecked.result.len(), 13);

        let checks = UserChecks {
            decryption_id_match: true,
            decryption_id_majority: false,
        };
        let (_, checked) = aggregator::<UserDecrypt>(&groups, 9, checks, CancellationToken::new());
        assert_eq!(
            checked
                .run("req-6d", user_request())
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
        let (_, majority) = aggregator::<UserDecrypt>(&groups, 9, checks, CancellationToken::new());
        assert_eq!(
            majority
                .run("req-6e", user_request())
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
            .run("req-7", public_request())
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
            .run("req-8", public_request())
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
    async fn shutdown_cancels_a_running_aggregation() {
        let shutdown = CancellationToken::new();
        let (_, aggregator) =
            aggregator::<UserDecrypt>(&[(13, HANG)], 9, UserChecks::default(), shutdown.clone());
        let started = Instant::now();
        let (result, ()) = tokio::join!(aggregator.run("req-9", user_request()), async {
            tokio::time::sleep(ms(100)).await;
            shutdown.cancel();
        });
        assert_eq!(result.unwrap_err(), AggregationError::Cancelled);
        assert_eq!(started.elapsed(), ms(100));
    }

    #[tokio::test(start_paused = true)]
    async fn dropping_the_run_future_aborts_the_node_tasks() {
        let (mock, aggregator) = aggregator::<UserDecrypt>(
            &[(13, HANG)],
            9,
            UserChecks::default(),
            CancellationToken::new(),
        );
        let run = aggregator.run("req-10", user_request());
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
        assert_eq!(AggregationError::Cancelled.dominant(), None);
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
}
