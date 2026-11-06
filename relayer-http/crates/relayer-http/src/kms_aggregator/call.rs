//! One node call: sign → semaphore permit → POST → decode → retry per the class of the failure, cancellable and
//! bounded by the deadline.

use std::sync::Arc;

use bytes::Bytes;
use kms_connector_api::ErrorCode;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant, sleep};
use tokio_util::sync::CancellationToken;
use tracing::warn;

use super::client::{AttemptError, ConnectorClient, Endpoint, decode};
use super::config::{CallConfig, ConfigError, KmsAggregatorConfig, RetryClass};
use super::signer::RequestSigner;
use crate::metrics::{FlowName, Metrics};

/// Terminal outcome of one node call.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CallError {
    /// The aggregation ended (deadline or fail fast) while this call was still running.
    #[error("cancelled")]
    Cancelled,
    /// Non-retryable error (including an authentication rejection), or retries exhausted.
    #[error("{0}")]
    Failed(AttemptError),
}

impl CallError {
    /// Connector code for the failure summary; `None` for cancelled calls.
    pub fn code(&self) -> Option<ErrorCode> {
        match self {
            Self::Cancelled => None,
            Self::Failed(e) => Some(e.code()),
        }
    }
}

/// Retries of one call, per class. Each budget is consumed once and never restarts within the call.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Retries {
    pub transient: u32,
    pub rate_limit: u32,
}

impl Retries {
    fn of(self, class: RetryClass) -> u32 {
        match class {
            RetryClass::Transient => self.transient,
            RetryClass::RateLimit => self.rate_limit,
        }
    }

    fn bump(&mut self, class: RetryClass) {
        match class {
            RetryClass::Transient => self.transient += 1,
            RetryClass::RateLimit => self.rate_limit += 1,
        }
    }
}

/// Why a call stops retrying, in decision order: a rejected credential and a non-retryable error are final
/// whatever the budget; then the class's budget; then a wait that would end after the deadline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GiveUp {
    Auth,
    NonRetryable,
    BudgetExhausted,
    Deadline,
}

impl GiveUp {
    fn decide(failure: &AttemptError, budget_left: bool, fits: bool) -> Option<Self> {
        if failure.is_auth() {
            Some(Self::Auth)
        } else if !failure.is_retryable() {
            Some(Self::NonRetryable)
        } else if !budget_left {
            Some(Self::BudgetExhausted)
        } else if !fits {
            Some(Self::Deadline)
        } else {
            None
        }
    }
}

/// The `code` of an attempt cut in flight.
const CANCELLED: &str = "cancelled";

/// Where the current attempt of a call stands, for a cancellation to record it.
#[derive(Clone, Copy)]
enum Stage {
    /// Between attempts: signing or sleeping.
    Idle,
    /// Waiting for a call slot since.
    WaitingForSlot(Instant),
    /// Sent to the node, unanswered, since.
    Sent(Instant),
}

/// How far a call got: the attempts that reached the node pool (signing failures excluded), the retries of each
/// class it used (signing failures included: the policy counts them), and where the current attempt stands.
struct Progress {
    attempts: u32,
    retries: Retries,
    stage: Stage,
}

/// Records what a call did when it ends, however it ends: it returns, it is cancelled, or its future is dropped (the
/// client closed the connection and the task was aborted). A cancellation ends an attempt where it stands: it is
/// recorded then, as a hung or slow node left out would make the latency look healthy and a saturated pod look idle.
/// Exactly once, from `Drop`.
struct CallRecord<'a> {
    metrics: &'a Metrics,
    kms_node: &'a str,
    flow: FlowName,
    progress: Progress,
}

impl Drop for CallRecord<'_> {
    fn drop(&mut self) {
        // A panic unwinds through here: the aggregator counts it as `panic`, and the attempt in flight was not cut.
        if !std::thread::panicking() {
            match self.progress.stage {
                Stage::Idle => {}
                Stage::WaitingForSlot(since) => {
                    self.metrics.call_slot_wait(self.flow, since.elapsed());
                }
                Stage::Sent(since) => {
                    self.metrics
                        .attempt_answered(self.kms_node, self.flow, since.elapsed());
                    self.metrics
                        .attempt_error(self.kms_node, self.flow, CANCELLED);
                }
            }
        }
        // A call that made no attempt (signing failed, or cut while waiting for a slot) has no attempts to count.
        if self.progress.attempts > 0 {
            self.metrics
                .call(self.kms_node, self.flow, self.progress.attempts);
        }
    }
}

/// What one node call produced, and what it cost. `retries` counts every retried try, a signing failure included,
/// so `retries.transient` can exceed `attempts - 1` when signing failed before anything reached the node.
pub struct CallResult<R> {
    pub node: String,
    pub attempts: u32,
    pub retries: Retries,
    pub elapsed: Duration,
    pub result: Result<R, CallError>,
}

/// Shared by every aggregation: the HTTP client, the signer, the endpoints, the call semaphore, the retry policy
/// and the metrics every attempt and call records.
pub struct Caller {
    client: Arc<dyn ConnectorClient>,
    signer: Arc<RequestSigner>,
    metrics: Arc<Metrics>,
    endpoints: Vec<Endpoint>,
    cfg: CallConfig,
    permits: Semaphore,
}

impl Caller {
    /// Validates the config, resolves every endpoint and sizes the semaphore.
    pub fn new(
        cfg: &KmsAggregatorConfig,
        client: Arc<dyn ConnectorClient>,
        signer: Arc<RequestSigner>,
        metrics: Arc<Metrics>,
    ) -> Result<Self, ConfigError> {
        // The deadline arithmetic and the semaphore size rely on these bounds.
        cfg.validate()?;
        let endpoints: Vec<Endpoint> = cfg.endpoints.iter().map(Endpoint::from).collect();
        let permits = Semaphore::new(cfg.max_concurrent_calls.min(Semaphore::MAX_PERMITS));
        metrics.announce_nodes(
            endpoints
                .iter()
                .map(|endpoint: &Endpoint| endpoint.name.as_str()),
        );
        Ok(Self {
            client,
            signer,
            metrics,
            endpoints,
            cfg: cfg.call.clone(),
            permits,
        })
    }

    pub fn metrics(&self) -> &Arc<Metrics> {
        &self.metrics
    }

    pub fn endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    /// The deadline of one aggregation and of every call in it.
    pub fn timeout(&self) -> Duration {
        self.cfg.timeout
    }

    /// Calls one node until success, a terminal error, retries exhausted, or cancellation. `flow` labels the
    /// metrics.
    pub async fn call<R: DeserializeOwned>(
        &self,
        flow: FlowName,
        endpoint: &Endpoint,
        route: &str,
        body: Bytes,
        deadline: Instant,
        cancel: CancellationToken,
    ) -> CallResult<R> {
        let started = Instant::now();
        let mut record = CallRecord {
            metrics: &self.metrics,
            kms_node: &endpoint.name,
            flow,
            progress: Progress {
                attempts: 0,
                retries: Retries::default(),
                stage: Stage::Idle,
            },
        };
        let result = tokio::select! {
            biased;
            () = cancel.cancelled() => Err(CallError::Cancelled),
            result = self.attempt_loop(flow, endpoint, route, body, deadline, &mut record.progress) => result,
        };
        CallResult {
            node: endpoint.name.clone(),
            attempts: record.progress.attempts,
            retries: record.progress.retries,
            elapsed: started.elapsed(),
            result,
        }
    }

    async fn attempt_loop<R: DeserializeOwned>(
        &self,
        flow: FlowName,
        endpoint: &Endpoint,
        route: &str,
        body: Bytes,
        deadline: Instant,
        progress: &mut Progress,
    ) -> Result<R, CallError> {
        // One signature per call, reused by every attempt.
        let mut authorization: Option<HeaderValue> = None;
        // Every try, signing failures included: the retry policy counts them, the node metrics do not.
        let mut tries = 0;
        loop {
            tries += 1;
            let mut attempt_started = Instant::now();
            // The node's `Retry-After`, when it answered with one.
            let mut retry_after = None;
            let outcome = match self
                .authorization(&mut authorization, endpoint, route, &body)
                .await
            {
                // Nothing was sent: not an attempt to the node.
                Err(failure) => Err(failure),
                Ok(header) => {
                    // Sign before taking a slot: the wait starts once the signature is ready.
                    let waiting = Instant::now();
                    progress.stage = Stage::WaitingForSlot(waiting);
                    let Ok(permit) = self.permits.acquire().await else {
                        return Err(CallError::Failed(AttemptError::Transport(
                            "call limiter closed".to_owned(),
                        )));
                    };
                    self.metrics.call_slot_wait(flow, waiting.elapsed());
                    progress.attempts += 1;
                    attempt_started = Instant::now();
                    progress.stage = Stage::Sent(attempt_started);
                    let outcome = self
                        .client
                        .post(endpoint, route, body.clone(), &header)
                        .await
                        .and_then(|reply| {
                            retry_after = reply.retry_after;
                            decode::<R>(reply)
                        });
                    progress.stage = Stage::Idle;
                    // Never hold a slot while sleeping.
                    drop(permit);
                    // The node's own latency: the round trip only, no slot wait, no backoff. A connection
                    // that failed has no answer to time.
                    if !matches!(outcome, Err(AttemptError::Transport(_))) {
                        self.metrics.attempt_answered(
                            &endpoint.name,
                            flow,
                            attempt_started.elapsed(),
                        );
                    }
                    outcome
                }
            };
            let failure = match outcome {
                Ok(value) => return Ok(value),
                Err(failure) => failure,
            };
            match failure {
                AttemptError::Signer(_) => self.metrics.signer_error(flow),
                _ => self
                    .metrics
                    .attempt_error(&endpoint.name, flow, failure.code().as_str()),
            }
            // Expired at the node: sign again on the next attempt.
            if failure.code() == ErrorCode::AuthExpired {
                authorization = None;
            }
            // The failure's class picks the budget and the curve; the deadline bounds both.
            let class = failure.retry_class();
            let retries = progress.retries;
            let delay = self
                .cfg
                .delay_for(class, retries.of(class).saturating_add(1), retry_after);
            let budget_left = retries.of(class) < self.cfg.max_retries(class);
            let fits = delay < deadline.saturating_duration_since(Instant::now());
            let give_up = GiveUp::decide(&failure, budget_left, fits);
            attempt_failed(
                &endpoint.name,
                tries,
                attempt_started.elapsed(),
                &failure,
                give_up.is_none().then_some(delay),
            );
            if give_up.is_some() {
                return Err(CallError::Failed(failure));
            }
            progress.retries.bump(class);
            sleep(delay).await;
        }
    }

    /// The cached header, or a fresh signature for the node's audience.
    async fn authorization(
        &self,
        cached: &mut Option<HeaderValue>,
        endpoint: &Endpoint,
        route: &str,
        body: &[u8],
    ) -> Result<HeaderValue, AttemptError> {
        if let Some(header) = cached {
            return Ok(header.clone());
        }
        let header = self
            .signer
            .sign(endpoint.audience, route, body)
            .await
            .map_err(AttemptError::Signer)?;
        *cached = Some(header.clone());
        Ok(header)
    }
}

/// One failed attempt, visible at the default level; the node task runs in the `aggregation` span, so the line
/// also carries the request's id. `delay` is `None` when the call gives up. A signing failure is the relayer's own
/// and is reported as such: nothing reached the node.
fn attempt_failed(
    kms_node: &str,
    attempt: u32,
    elapsed: Duration,
    failure: &AttemptError,
    delay: Option<Duration>,
) {
    let elapsed_ms = elapsed.as_millis() as u64;
    let retry_in_ms = delay.map(|d| d.as_millis() as u64);
    if let AttemptError::Signer(_) = failure {
        warn!(
            kms_node,
            attempt,
            elapsed_ms,
            error = %failure.digest(),
            retry_in_ms,
            "signing failed"
        );
    } else {
        warn!(
            kms_node,
            attempt,
            elapsed_ms,
            error = %failure.digest(),
            code = failure.code().as_str(),
            retry_in_ms,
            "attempt failed"
        );
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use kms_connector_api::{ErrorCode, USER_DECRYPTION_ROUTE, UserDecryptionResponse};

    use super::*;
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use alloy::primitives::{Address, B256, ChainId, Signature, keccak256};
    use alloy::signers::Signer;
    use alloy::signers::local::PrivateKeySigner;

    use crate::kms_aggregator::config::tests::{audience, valid};
    use crate::kms_aggregator::mock::{Fixed, MockClient, Reply, USER_REQUEST_JSON};
    use crate::kms_aggregator::signer::tests::{test_domain, test_signer};
    use crate::logging::capture::capture_runs;
    use crate::metrics::tests::{histogram_sum, total, value};

    const OK: Reply = Reply::Fixed(Fixed::Ok);
    const HANG: Reply = Reply::Fixed(Fixed::Hang);
    const REFUSED: Reply = Reply::Fixed(Fixed::Refused);
    const PANIC: Reply = Reply::Fixed(Fixed::Panic);
    const ACL: Reply = Reply::Error(ErrorCode::AclDenied);
    const RATE: Reply = Reply::Error(ErrorCode::RateLimited);
    const AUTH: Reply = Reply::Error(ErrorCode::SenderAuthenticationFailed);
    const MALFORMED: Reply = Reply::Error(ErrorCode::Malformed);

    fn ms(v: u64) -> Duration {
        Duration::from_millis(v)
    }

    fn secs(v: u64) -> Duration {
        Duration::from_secs(v)
    }

    /// One user decrypt call to node `node` of `caller`, `deadline` from the first poll, cut by `cancel`.
    async fn call_node(
        caller: &Caller,
        node: usize,
        deadline: Duration,
        cancel: &CancellationToken,
    ) -> CallResult<UserDecryptionResponse> {
        caller
            .call(
                FlowName::UserDecrypt,
                &caller.endpoints()[node],
                USER_DECRYPTION_ROUTE,
                Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                Instant::now() + deadline,
                cancel.clone(),
            )
            .await
    }

    /// A local signer that takes `delay` to sign, or refuses to, and counts its signatures.
    struct TestSigner {
        inner: PrivateKeySigner,
        delay: Duration,
        refuses: bool,
        signatures: AtomicUsize,
    }

    impl TestSigner {
        fn new(delay: Duration, refuses: bool) -> Arc<Self> {
            Arc::new(Self {
                inner: PrivateKeySigner::random(),
                delay,
                refuses,
                signatures: AtomicUsize::new(0),
            })
        }

        fn signatures(&self) -> usize {
            self.signatures.load(Ordering::SeqCst)
        }
    }

    #[async_trait::async_trait]
    impl Signer for TestSigner {
        async fn sign_hash(&self, hash: &B256) -> alloy::signers::Result<Signature> {
            self.signatures.fetch_add(1, Ordering::SeqCst);
            sleep(self.delay).await;
            if self.refuses {
                return Err(alloy::signers::Error::other("AccessDeniedException"));
            }
            self.inner.sign_hash(hash).await
        }

        fn address(&self) -> Address {
            self.inner.address()
        }

        fn chain_id(&self) -> Option<ChainId> {
            None
        }

        fn set_chain_id(&mut self, _chain_id: Option<ChainId>) {}
    }

    /// One call to a mock node (`script`, 5 s deadline, 10 ms per attempt) signed by `signer`.
    async fn run_signed_by(
        signer: &Arc<TestSigner>,
        script: Vec<Reply>,
        max_retries: u32,
    ) -> (
        Arc<MockClient>,
        Arc<Caller>,
        CallResult<UserDecryptionResponse>,
    ) {
        let signer = Arc::new(RequestSigner::new(signer.clone(), test_domain(), secs(60)));
        let mock = MockClient::new(vec![script], ms(10));
        let caller = mock
            .clone()
            .caller_with(MockClient::call_config(secs(5), max_retries), signer);
        let result = call_node(&caller, 0, secs(5), &CancellationToken::new()).await;
        (mock, caller, result)
    }

    /// One mock node with `script`, `max_retries` transient retries (one rate-limit retry), a 5 s deadline,
    /// 10 ms per attempt.
    async fn run(
        script: Vec<Reply>,
        max_retries: u32,
    ) -> (Arc<MockClient>, CallResult<UserDecryptionResponse>) {
        run_with(script, MockClient::call_config(secs(5), max_retries)).await
    }

    /// One mock node with `script` under `call`, the deadline being `call.timeout`, 10 ms per attempt.
    async fn run_with(
        script: Vec<Reply>,
        call: CallConfig,
    ) -> (Arc<MockClient>, CallResult<UserDecryptionResponse>) {
        let mock = MockClient::new(vec![script], ms(10));
        let caller = mock.clone().caller_with_call(call.clone());
        let result = call_node(&caller, 0, call.timeout, &CancellationToken::new()).await;
        (mock, result)
    }

    /// A 429 with `Retry-After: <seconds>`.
    fn rate_limited_after(seconds: u64) -> Reply {
        Reply::RateLimitedAfter {
            rate_limited_after: secs(seconds),
        }
    }

    /// `transient` retries of 200 ms and `rate_limit` retries from 1 s under a 5 s deadline.
    fn budgets(transient: u32, rate_limit: u32) -> CallConfig {
        let mut call = MockClient::call_config(secs(5), transient);
        call.rate_limit.max_retries = rate_limit;
        call
    }

    #[tokio::test(start_paused = true)]
    async fn every_failed_attempt_is_a_warn_with_its_decision() {
        let lines = capture_runs(&["attempt failed"], || async {
            run(vec![RATE, MALFORMED], 2).await;
        })
        .await;
        let lines: Vec<_> = lines
            .iter()
            .filter(|l| l["fields"]["message"] == "attempt failed")
            .collect();
        assert_eq!(lines.len(), 2, "{lines:?}");
        for (line, attempt) in lines.iter().zip(1..) {
            assert_eq!(line["level"], "WARN");
            assert_eq!(line["fields"]["kms_node"], "node-0");
            assert_eq!(line["fields"]["attempt"], attempt);
            assert_eq!(line["fields"]["elapsed_ms"], 10);
        }
        assert_eq!(lines[0]["fields"]["code"], "rate_limited");
        assert!(lines[0]["fields"]["retry_in_ms"].as_u64().is_some());
        assert_eq!(lines[1]["fields"]["code"], "malformed");
        assert!(lines[1]["fields"]["retry_in_ms"].is_null(), "{}", lines[1]);
    }

    #[tokio::test(start_paused = true)]
    async fn success_on_first_attempt() {
        let (_, r) = run(vec![OK], 2).await;
        assert!(r.result.is_ok());
        assert_eq!(
            (r.attempts, r.elapsed, r.node.as_str()),
            (1, ms(10), "node-0")
        );
    }

    #[tokio::test(start_paused = true)]
    async fn transient_error_is_retried_linearly_then_gives_up() {
        let (mock, r) = run(vec![ACL], 2).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::AclDenied));
        assert_eq!(mock.attempts(0), 3);
        // 3 attempts of 10 ms, waits 200 ms then 400 ms.
        assert_eq!((r.attempts, r.elapsed), (3, ms(630)));
        assert_eq!(
            r.retries,
            Retries {
                transient: 2,
                rate_limit: 0
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn transient_retries_are_linear() {
        let (_, r) = run(vec![ACL, ACL, ACL, OK], 3).await;
        assert!(r.result.is_ok());
        // 4 attempts of 10 ms, waits 200 + 400 + 600 ms.
        assert_eq!((r.attempts, r.elapsed), (4, ms(1240)));
    }

    #[tokio::test(start_paused = true)]
    async fn rate_limit_waits_one_window() {
        let (_, r) = run(vec![RATE, OK], 0).await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (2, ms(1020)));
        assert_eq!(
            r.retries,
            Retries {
                transient: 0,
                rate_limit: 1
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn rate_limit_backoff_doubles_from_the_floor() {
        let (_, r) = run_with(vec![RATE, RATE, OK], budgets(0, 2)).await;
        assert!(r.result.is_ok());
        // Waits 1 s then 2 s.
        assert_eq!((r.attempts, r.elapsed), (3, ms(3030)));
        // A third 429 would wait 4 s: past the 5 s deadline, so the budget is also bounded there.
        let (_, r) = run_with(vec![RATE], budgets(0, 2)).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::RateLimited));
        assert_eq!((r.attempts, r.elapsed), (3, ms(3030)));
    }

    #[tokio::test(start_paused = true)]
    async fn retry_after_above_the_floor_is_the_base_of_the_curve() {
        let (_, r) = run_with(vec![rate_limited_after(2), OK], budgets(0, 2)).await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (2, ms(2020)));
        // Then 4 s: does not fit before the 5 s deadline, the call gives up without sleeping.
        let (_, r) = run_with(vec![rate_limited_after(2)], budgets(0, 2)).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::RateLimited));
        assert_eq!((r.attempts, r.elapsed), (2, ms(2020)));
    }

    #[tokio::test(start_paused = true)]
    async fn retry_after_beyond_the_deadline_gives_up_at_once() {
        let (_, r) = run(vec![rate_limited_after(10), OK], 0).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::RateLimited));
        assert_eq!((r.attempts, r.elapsed), (1, ms(10)));
    }

    #[tokio::test(start_paused = true)]
    async fn retry_after_below_the_floor_keeps_the_floor() {
        let (_, r) = run(vec![rate_limited_after(0), OK], 0).await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (2, ms(1020)));
    }

    #[tokio::test(start_paused = true)]
    async fn classes_have_independent_budgets() {
        // One 429 then one acl_denied: one window, then one linear wait; each budget used once.
        let (_, r) = run(vec![RATE, ACL, OK], 3).await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (3, ms(1230)));
        assert_eq!(
            r.retries,
            Retries {
                transient: 1,
                rate_limit: 1
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn budgets_never_restart() {
        // No transient budget: the 429 is retried, the acl_denied is not.
        let (_, r) = run_with(vec![RATE, OK], budgets(0, 1)).await;
        assert!(r.result.is_ok());
        let (_, r) = run_with(vec![ACL, OK], budgets(0, 1)).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::AclDenied));
        assert_eq!(r.attempts, 1);
        // A transient retry in between does not refill the rate-limit budget.
        let (_, r) = run_with(vec![RATE, ACL, RATE, OK], budgets(1, 1)).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::RateLimited));
        assert_eq!((r.attempts, r.elapsed), (3, ms(1230)));
    }

    #[tokio::test(start_paused = true)]
    async fn transport_error_is_retried() {
        let (_, r) = run(vec![REFUSED, OK], 1).await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (2, ms(220)));
    }

    #[tokio::test(start_paused = true)]
    async fn non_retryable_and_auth_errors_are_never_retried() {
        for (reply, code) in [
            (MALFORMED, ErrorCode::Malformed),
            (AUTH, ErrorCode::SenderAuthenticationFailed),
        ] {
            let (_, r) = run(vec![reply, OK], 3).await;
            assert_eq!(r.result.unwrap_err().code(), Some(code));
            assert_eq!(r.attempts, 1);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn max_retries_zero_is_a_single_attempt() {
        let (_, r) = run(vec![ACL, OK], 0).await;
        assert!(r.result.is_err());
        assert_eq!(r.attempts, 1);
    }

    #[tokio::test(start_paused = true)]
    async fn retry_that_does_not_fit_before_the_deadline_is_skipped() {
        let mock = MockClient::new(vec![vec![ACL, OK]], ms(10));
        let caller = mock.clone().caller(secs(5), 3);
        // 140 ms left after the attempt: the 200 ms wait would end after the deadline.
        let r = call_node(&caller, 0, ms(150), &CancellationToken::new()).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::AclDenied));
        assert_eq!((r.attempts, r.elapsed), (1, ms(10)));
    }

    #[tokio::test(start_paused = true)]
    async fn cancel_while_hanging() {
        let mock = MockClient::new(vec![vec![HANG]], ms(10));
        let caller = mock.clone().caller(secs(5), 0);
        let cancel = CancellationToken::new();
        let call = call_node(&caller, 0, secs(5), &cancel);
        let r = tokio::join!(call, async {
            sleep(ms(700)).await;
            cancel.cancel();
        })
        .0;
        assert_eq!(r.result.unwrap_err(), CallError::Cancelled);
        assert_eq!((r.attempts, r.elapsed), (1, ms(700)));
    }

    #[tokio::test(start_paused = true)]
    async fn cancel_while_waiting_for_a_permit_reports_zero_attempts() {
        // One permit for one node: the second call to it waits on the semaphore.
        let mock = MockClient::new(vec![vec![HANG]], ms(10));
        let caller = mock.clone().caller(secs(5), 0);
        let cancel = CancellationToken::new();
        let first = call_node(&caller, 0, secs(5), &cancel);
        let second = async {
            sleep(ms(50)).await;
            call_node(&caller, 0, secs(5), &cancel).await
        };
        let (first, second, ()) = tokio::join!(first, second, async {
            sleep(ms(300)).await;
            cancel.cancel();
        });
        assert_eq!(first.attempts, 1);
        assert_eq!(second.result.unwrap_err(), CallError::Cancelled);
        assert_eq!(second.attempts, 0);
        assert_eq!(mock.attempts(0), 1);
        // The wait that ended in a cancellation is in the slot wait: a saturated pod must not look idle.
        let waited = histogram_sum(caller.metrics(), "relayer_http_kms_call_slot_wait_seconds");
        assert!((waited - 0.25).abs() < 1e-9, "{waited}");
        // Only the call that made an attempt is counted in the attempts per call.
        assert_eq!(total(caller.metrics(), "relayer_http_kms_attempts"), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn an_attempt_cut_in_flight_is_recorded_when_it_is_cut() {
        let mock = MockClient::new(vec![vec![HANG]], ms(10));
        let caller = mock.clone().caller(secs(5), 0);
        let cancel = CancellationToken::new();
        let call = call_node(&caller, 0, secs(5), &cancel);
        tokio::join!(call, async {
            sleep(ms(700)).await;
            cancel.cancel();
        });
        let metrics = caller.metrics();
        // The 700 ms the node held the attempt are in the latency: a hung node shows as slow.
        let slow = histogram_sum(metrics, "relayer_http_kms_attempt_duration_seconds");
        assert!((slow - 0.7).abs() < 1e-9, "{slow}");
        let cancelled = [("kms_node", "node-0"), ("code", "cancelled")];
        assert_eq!(
            value(metrics, "relayer_http_kms_attempt_errors_total", &cancelled),
            Some(1)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_panicked_attempt_is_not_recorded_as_cut() {
        let mock = MockClient::new(vec![vec![PANIC]], ms(10));
        let caller = mock.clone().caller(secs(5), 0);
        let task = tokio::spawn({
            let caller = caller.clone();
            async move {
                call_node(&caller, 0, secs(5), &CancellationToken::new())
                    .await
                    .attempts
            }
        });
        assert!(task.await.unwrap_err().is_panic());
        // The aggregator counts the `panic`; the unwinding records no cut attempt and no latency.
        let metrics = caller.metrics();
        assert_eq!(total(metrics, "relayer_http_kms_attempt_errors_total"), 0);
        assert_eq!(
            total(metrics, "relayer_http_kms_attempt_duration_seconds"),
            0
        );
    }

    #[tokio::test(start_paused = true)]
    async fn the_attempts_of_a_call_and_its_answers_are_recorded() {
        let mock = MockClient::new(vec![vec![RATE, REFUSED, OK]], ms(10));
        let caller = mock.clone().caller(secs(5), 2);
        let r = call_node(&caller, 0, secs(5), &CancellationToken::new()).await;
        assert_eq!(r.attempts, 3);
        let metrics = caller.metrics();
        // Answered: the 429 and the success; the refused connection has no answer to time.
        assert_eq!(
            total(metrics, "relayer_http_kms_attempt_duration_seconds"),
            2
        );
        assert_eq!(total(metrics, "relayer_http_kms_attempt_errors_total"), 2);
        assert_eq!(total(metrics, "relayer_http_kms_attempts"), 1);
        assert_eq!(total(metrics, "relayer_http_kms_call_slot_wait_seconds"), 3);
        assert_eq!(histogram_sum(metrics, "relayer_http_kms_attempts"), 3.0);
    }

    #[test]
    fn new_rejects_an_invalid_config() {
        let mock = MockClient::new(vec![vec![OK]], ms(1));
        let mut cfg = valid(1);
        cfg.call.timeout = Duration::ZERO;
        let e = Caller::new(&cfg, mock, test_signer(), Arc::new(Metrics::new().unwrap()))
            .err()
            .unwrap();
        assert!(e.0.contains("call.timeout"), "{e}");
    }

    #[test]
    fn new_announces_the_nodes_panic_series_at_zero() {
        let mock = MockClient::new(vec![vec![OK], vec![OK]], ms(1));
        let caller = mock.caller(secs(5), 0);
        for node in ["node-0", "node-1"] {
            for flow in ["user_decrypt", "public_decrypt"] {
                let labels = [("kms_node", node), ("flow", flow), ("code", "panic")];
                assert_eq!(
                    value(
                        caller.metrics(),
                        "relayer_http_kms_attempt_errors_total",
                        &labels
                    ),
                    Some(0),
                    "{node} {flow}"
                );
            }
        }
    }

    #[tokio::test(start_paused = true)]
    async fn each_node_gets_its_own_header_reused_across_retries() {
        let mock = MockClient::new(vec![vec![RATE, OK], vec![OK]], ms(10));
        let caller = mock.clone().caller(secs(5), 1);
        for node in 0..2 {
            let result = call_node(&caller, node, secs(5), &CancellationToken::new()).await;
            assert!(result.result.is_ok());
        }
        let node0 = mock.authorizations(0);
        assert_eq!(node0.len(), 2);
        assert_eq!(node0[0], node0[1], "a retry reuses the header");

        let now = fhevm_sig_auth::unix_now();
        for node in 0..2 {
            let header = &mock.authorizations(node)[0];
            let signer = fhevm_sig_auth::FhevmSigVerifier::new(
                test_domain(),
                audience(node),
                HashSet::new(),
            )
            .recover(
                Some(header.as_bytes()),
                &fhevm_sig_auth::PathAndQuery::from_static(USER_DECRYPTION_ROUTE),
                now,
            );
            let digest = keccak256(USER_REQUEST_JSON.as_bytes());
            assert_eq!(signer, Ok((test_signer().address(), digest)), "node {node}");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn auth_expired_is_retried_with_a_fresh_signature() {
        let signer = TestSigner::new(Duration::ZERO, false);
        let (mock, _, r) =
            run_signed_by(&signer, vec![Reply::Error(ErrorCode::AuthExpired), OK], 1).await;
        assert!(r.result.is_ok());
        assert_eq!((mock.attempts(0), signer.signatures()), (2, 2));

        let signer = TestSigner::new(Duration::ZERO, false);
        let (mock, _, r) = run_signed_by(&signer, vec![RATE, OK], 1).await;
        assert!(r.result.is_ok());
        assert_eq!((mock.attempts(0), signer.signatures()), (2, 1));
    }

    #[tokio::test(start_paused = true)]
    async fn the_slot_wait_does_not_include_the_signing_time() {
        let signer = TestSigner::new(ms(300), false);
        let (_, caller, result) = run_signed_by(&signer, vec![OK], 2).await;
        assert!(result.result.is_ok());
        // Signing took 300 ms, a free slot took none: the pod is not saturated.
        let metrics = caller.metrics();
        assert_eq!(
            histogram_sum(metrics, "relayer_http_kms_call_slot_wait_seconds"),
            0.0
        );
        // Nor is it the node's latency.
        let node = histogram_sum(metrics, "relayer_http_kms_attempt_duration_seconds");
        assert!((node - 0.01).abs() < 1e-9, "{node}");
    }

    #[tokio::test(start_paused = true)]
    async fn a_signing_failure_is_the_relayers_own_signal_not_a_node_attempt() {
        let lines = capture_runs(&["signing failed"], || async {
            let signer = TestSigner::new(ms(5), true);
            let (mock, caller, result) = run_signed_by(&signer, vec![OK], 2).await;
            assert!(matches!(
                result.result,
                Err(CallError::Failed(AttemptError::Signer(_)))
            ));
            // Three tries (two retries), none of them reached the node.
            assert_eq!((result.attempts, mock.attempts(0)), (0, 0));
            let metrics = caller.metrics();
            assert_eq!(total(metrics, "relayer_http_signer_errors_total"), 3);
            for node_metric in [
                "relayer_http_kms_attempt_errors_total",
                "relayer_http_kms_attempt_duration_seconds",
                "relayer_http_kms_call_slot_wait_seconds",
                "relayer_http_kms_attempts",
            ] {
                assert_eq!(total(metrics, node_metric), 0, "{node_metric}");
            }
        })
        .await;
        let signing: Vec<_> = lines
            .iter()
            .filter(|l| l["fields"]["message"] == "signing failed")
            .collect();
        assert_eq!(signing.len(), 3);
        for (line, attempt) in signing.iter().zip(1..) {
            assert_eq!(line["level"], "WARN");
            assert_eq!(line["fields"]["kms_node"], "node-0");
            assert_eq!(line["fields"]["attempt"], attempt);
            assert!(line["fields"]["code"].is_null(), "no node code: {line}");
            assert!(
                line["fields"]["error"]
                    .as_str()
                    .unwrap()
                    .starts_with("signer: ")
            );
        }
        assert!(signing[2]["fields"]["retry_in_ms"].is_null());
    }

    #[test]
    fn accessors_echo_the_config() {
        let mock = MockClient::new(vec![vec![OK], vec![OK]], ms(1));
        let caller = mock.caller(secs(3), 0);
        assert_eq!(caller.endpoints().len(), 2);
        assert_eq!(caller.endpoints()[1].name, "node-1");
        assert_eq!(caller.timeout(), secs(3));
    }

    #[test]
    fn call_error_code() {
        assert_eq!(CallError::Cancelled.code(), None);
        assert_eq!(
            CallError::Failed(AttemptError::Status(502)).code(),
            Some(ErrorCode::UpstreamTransient)
        );
    }
}
