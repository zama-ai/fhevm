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

use super::client::{AttemptError, ConnectorClient, Endpoint, decode};
use super::config::{CallConfig, ConfigError, KmsAggregatorConfig, RetryClass};
use super::signer::RequestSigner;

/// Terminal outcome of one node call.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CallError {
    /// The aggregation ended (deadline, fail fast or shutdown) while this call was still running.
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

    fn as_str(self) -> &'static str {
        match self {
            Self::Auth => "auth",
            Self::NonRetryable => "non_retryable",
            Self::BudgetExhausted => "budget_exhausted",
            Self::Deadline => "deadline",
        }
    }
}

/// What one node call produced, and what it cost.
pub struct CallResult<R> {
    pub node: String,
    pub attempts: u32,
    pub retries: Retries,
    pub elapsed: Duration,
    pub result: Result<R, CallError>,
}

/// Shared by every aggregation: the HTTP client, the signer, the endpoints, the call semaphore and the retry policy.
pub struct Caller {
    client: Arc<dyn ConnectorClient>,
    signer: Arc<RequestSigner>,
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
    ) -> Result<Self, ConfigError> {
        // The deadline arithmetic and the semaphore size rely on these bounds.
        cfg.validate()?;
        let endpoints = cfg.endpoints.iter().map(Endpoint::from).collect();
        let permits = Semaphore::new(cfg.max_concurrent_calls.min(Semaphore::MAX_PERMITS));
        Ok(Self {
            client,
            signer,
            endpoints,
            cfg: cfg.call.clone(),
            permits,
        })
    }

    pub fn endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    /// The deadline of one aggregation and of every call in it.
    pub fn timeout(&self) -> Duration {
        self.cfg.timeout
    }

    /// The deadline and the retry budgets every call follows.
    pub fn call_config(&self) -> &CallConfig {
        &self.cfg
    }

    /// Calls one node until success, a terminal error, retries exhausted, or cancellation.
    pub async fn call<R: DeserializeOwned>(
        &self,
        endpoint: &Endpoint,
        route: &str,
        body: Bytes,
        deadline: Instant,
        cancel: CancellationToken,
    ) -> CallResult<R> {
        let started = Instant::now();
        let (mut attempts, mut retries) = (0, Retries::default());
        let result = tokio::select! {
            biased;
            () = cancel.cancelled() => Err(CallError::Cancelled),
            result = self.attempt_loop(endpoint, route, body, deadline, &mut attempts, &mut retries) => result,
        };
        CallResult {
            node: endpoint.name.clone(),
            attempts,
            retries,
            elapsed: started.elapsed(),
            result,
        }
    }

    async fn attempt_loop<R: DeserializeOwned>(
        &self,
        endpoint: &Endpoint,
        route: &str,
        body: Bytes,
        deadline: Instant,
        attempts: &mut u32,
        retries: &mut Retries,
    ) -> Result<R, CallError> {
        // One signature per call, reused by every attempt.
        let mut authorization: Option<HeaderValue> = None;
        loop {
            let mut attempt_started = Instant::now();
            // The node's `Retry-After`, when it answered with one.
            let mut retry_after = None;
            let outcome = match self
                .authorization(&mut authorization, endpoint, route, &body)
                .await
            {
                Err(failure) => {
                    *attempts += 1;
                    Err(failure)
                }
                Ok(header) => {
                    // Sign before taking a slot.
                    let Ok(permit) = self.permits.acquire().await else {
                        return Err(CallError::Failed(AttemptError::Transport(
                            "call limiter closed".to_owned(),
                        )));
                    };
                    *attempts += 1;
                    attempt_started = Instant::now();
                    let outcome = self
                        .client
                        .post(endpoint, route, body.clone(), &header)
                        .await
                        .and_then(|reply| {
                            retry_after = reply.retry_after;
                            decode::<R>(reply)
                        });
                    // Never hold a slot while sleeping.
                    drop(permit);
                    outcome
                }
            };
            let failure = match outcome {
                Ok(value) => return Ok(value),
                Err(failure) => failure,
            };
            // Expired at the node: sign again on the next attempt.
            if failure.code() == ErrorCode::AuthExpired {
                authorization = None;
            }
            // The failure's class picks the budget and the curve; the deadline bounds both.
            let class = failure.retry_class();
            let delay = self
                .cfg
                .delay_for(class, retries.of(class).saturating_add(1), retry_after);
            let budget_left = retries.of(class) < self.cfg.max_retries(class);
            let fits = delay < deadline.saturating_duration_since(Instant::now());
            let give_up = GiveUp::decide(&failure, budget_left, fits);
            attempt_failed(
                &endpoint.name,
                *attempts,
                attempt_started.elapsed(),
                &failure,
                Decision {
                    class,
                    delay,
                    retry_after,
                    give_up,
                },
            );
            if give_up.is_some() {
                return Err(CallError::Failed(failure));
            }
            retries.bump(class);
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

/// What the policy decided after one failed attempt, for the log line.
#[derive(Debug, Clone, Copy)]
pub(super) struct Decision {
    class: RetryClass,
    /// The wait before the next attempt (meaningful when the call retries).
    delay: Duration,
    retry_after: Option<Duration>,
    give_up: Option<GiveUp>,
}

/// One failed attempt: `info` when a retry follows (expected behaviour), `warn` when the call gives up (the event
/// an operator looks for). The node task runs in the `aggregation` span, so the line also carries the request's
/// identifiers.
pub(super) fn attempt_failed(
    node: &str,
    attempt: u32,
    elapsed: Duration,
    failure: &AttemptError,
    decision: Decision,
) {
    let ms = |d: Duration| d.as_millis() as u64;
    macro_rules! line {
        ($level:ident) => {
            tracing::$level!(
                node,
                attempt,
                elapsed_ms = ms(elapsed),
                error = %failure,
                code = failure.code().as_str(),
                class = decision.class.as_str(),
                retry_in_ms = decision.give_up.is_none().then(|| ms(decision.delay)),
                retry_after_ms = decision.retry_after.map(ms),
                give_up_reason = decision.give_up.map(GiveUp::as_str),
                "attempt failed"
            )
        };
    }
    match decision.give_up {
        None => line!(info),
        Some(_) => line!(warn),
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
    use crate::logging::capture::{Sink, capture};

    const OK: Reply = Reply::Fixed(Fixed::Ok);
    const HANG: Reply = Reply::Fixed(Fixed::Hang);
    const REFUSED: Reply = Reply::Fixed(Fixed::Refused);
    const ACL: Reply = Reply::Error(ErrorCode::AclDenied);
    const RATE: Reply = Reply::Error(ErrorCode::RateLimited);
    const AUTH: Reply = Reply::Error(ErrorCode::SenderAuthenticationFailed);
    const MALFORMED: Reply = Reply::Error(ErrorCode::Malformed);

    fn ms(v: u64) -> Duration {
        Duration::from_millis(v)
    }

    /// One mock node with `script`, `max_retries` transient retries (one rate-limit retry), a 5 s deadline,
    /// 10 ms per attempt.
    async fn run(
        script: Vec<Reply>,
        max_retries: u32,
    ) -> (Arc<MockClient>, CallResult<UserDecryptionResponse>) {
        let call = MockClient::call_config(Duration::from_secs(5), max_retries);
        run_with(script, call).await
    }

    /// One mock node with `script` under `call`, the deadline being `call.timeout`, 10 ms per attempt.
    async fn run_with(
        script: Vec<Reply>,
        call: CallConfig,
    ) -> (Arc<MockClient>, CallResult<UserDecryptionResponse>) {
        let deadline = Instant::now() + call.timeout;
        let mock = MockClient::new(vec![script], ms(10));
        let caller = mock.clone().caller_with_call(call);
        let result = caller
            .call(
                &caller.endpoints()[0],
                USER_DECRYPTION_ROUTE,
                Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                deadline,
                CancellationToken::new(),
            )
            .await;
        (mock, result)
    }

    /// A 429 with `Retry-After: <seconds>`.
    fn rate_limited_after(seconds: u64) -> Reply {
        Reply::RateLimitedAfter {
            rate_limited_after: Duration::from_secs(seconds),
        }
    }

    /// `transient` retries of 200 ms and `rate_limit` retries from 1 s under a 5 s deadline.
    fn budgets(transient: u32, rate_limit: u32) -> CallConfig {
        let mut call = MockClient::call_config(Duration::from_secs(5), transient);
        call.rate_limit.max_retries = rate_limit;
        call
    }

    /// Captures the `attempt failed` lines of this thread (both levels: a retry and a give-up).
    pub(crate) async fn capture_attempts() -> (tracing::subscriber::DefaultGuard, Sink) {
        capture("attempt failed", 2, || {
            let decision = Decision {
                class: RetryClass::Transient,
                delay: Duration::ZERO,
                retry_after: None,
                give_up: None,
            };
            let failure = AttemptError::Status(500);
            attempt_failed("probe", 1, Duration::ZERO, &failure, decision);
            attempt_failed(
                "probe",
                2,
                Duration::ZERO,
                &failure,
                Decision {
                    give_up: Some(GiveUp::Deadline),
                    ..decision
                },
            );
        })
        .await
    }

    #[tokio::test(start_paused = true)]
    async fn attempt_lines_are_info_on_retry_and_warn_on_give_up() {
        let (_guard, sink) = capture_attempts().await;
        let (_, r) = run(vec![rate_limited_after(2), ACL, MALFORMED], 2).await;
        assert_eq!(r.attempts, 3);
        let lines = sink.lines("attempt failed");
        assert_eq!(lines.len(), 3, "{lines:?}");
        for (line, attempt) in lines.iter().zip(1..) {
            assert_eq!(line["fields"]["node"], "node-0");
            assert_eq!(line["fields"]["attempt"], attempt);
            assert_eq!(line["fields"]["elapsed_ms"], 10);
        }
        let fields = |i: usize, name: &str| lines[i]["fields"][name].clone();
        // A rate limit retried after its header: info, the class, the wait, the header.
        assert_eq!(lines[0]["level"], "INFO");
        assert_eq!(fields(0, "code"), "rate_limited");
        assert_eq!(fields(0, "class"), "rate_limit");
        assert_eq!(fields(0, "retry_in_ms"), 2000);
        assert_eq!(fields(0, "retry_after_ms"), 2000);
        assert!(fields(0, "give_up_reason").is_null(), "{}", lines[0]);
        // A transient error retried: info, no header.
        assert_eq!(lines[1]["level"], "INFO");
        assert_eq!(fields(1, "class"), "transient");
        assert_eq!(fields(1, "retry_in_ms"), 200);
        assert!(fields(1, "retry_after_ms").is_null(), "{}", lines[1]);
        // The give-up: warn, no wait, the reason.
        assert_eq!(lines[2]["level"], "WARN");
        assert_eq!(fields(2, "code"), "malformed");
        assert!(fields(2, "retry_in_ms").is_null(), "{}", lines[2]);
        assert_eq!(fields(2, "give_up_reason"), "non_retryable");
    }

    #[tokio::test(start_paused = true)]
    async fn give_up_reasons() {
        let (_guard, sink) = capture_attempts().await;
        let last_reason = || {
            let lines = sink.lines("attempt failed");
            let reason = lines.last().unwrap()["fields"]["give_up_reason"].clone();
            sink.clear();
            reason
        };
        run(vec![AUTH], 2).await;
        assert_eq!(last_reason(), "auth");
        run(vec![ACL], 1).await;
        assert_eq!(last_reason(), "budget_exhausted");
        run(vec![rate_limited_after(10)], 1).await;
        assert_eq!(last_reason(), "deadline");
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
        let caller = mock.clone().caller(Duration::from_secs(5), 3);
        // 140 ms left after the attempt: the 200 ms wait would end after the deadline.
        let r: CallResult<UserDecryptionResponse> = caller
            .call(
                &caller.endpoints()[0],
                USER_DECRYPTION_ROUTE,
                Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                Instant::now() + ms(150),
                CancellationToken::new(),
            )
            .await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::AclDenied));
        assert_eq!((r.attempts, r.elapsed), (1, ms(10)));
    }

    #[tokio::test(start_paused = true)]
    async fn cancel_while_hanging() {
        let mock = MockClient::new(vec![vec![HANG]], ms(10));
        let caller = mock.clone().caller(Duration::from_secs(5), 0);
        let cancel = CancellationToken::new();
        let call = caller.call::<UserDecryptionResponse>(
            &caller.endpoints()[0],
            USER_DECRYPTION_ROUTE,
            Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
            Instant::now() + Duration::from_secs(5),
            cancel.clone(),
        );
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
        let caller = mock.clone().caller(Duration::from_secs(5), 0);
        let cancel = CancellationToken::new();
        let body = Bytes::from_static(USER_REQUEST_JSON.as_bytes());
        let deadline = Instant::now() + Duration::from_secs(5);
        let first = caller.call::<UserDecryptionResponse>(
            &caller.endpoints()[0],
            USER_DECRYPTION_ROUTE,
            body.clone(),
            deadline,
            cancel.clone(),
        );
        let second = async {
            sleep(ms(50)).await;
            caller
                .call::<UserDecryptionResponse>(
                    &caller.endpoints()[0],
                    USER_DECRYPTION_ROUTE,
                    body.clone(),
                    deadline,
                    cancel.clone(),
                )
                .await
        };
        let (first, second, ()) = tokio::join!(first, second, async {
            sleep(ms(300)).await;
            cancel.cancel();
        });
        assert_eq!(first.attempts, 1);
        assert_eq!(second.result.unwrap_err(), CallError::Cancelled);
        assert_eq!(second.attempts, 0);
        assert_eq!(mock.attempts(0), 1);
    }

    #[test]
    fn new_rejects_an_invalid_config() {
        let mock = MockClient::new(vec![vec![OK]], ms(1));
        let mut cfg = valid(1);
        cfg.call.timeout = Duration::ZERO;
        let e = Caller::new(&cfg, mock, test_signer()).err().unwrap();
        assert!(e.0.contains("call.timeout"), "{e}");
    }

    /// A local signer counting its signatures.
    struct CountingSigner {
        inner: PrivateKeySigner,
        signatures: AtomicUsize,
    }

    #[async_trait::async_trait]
    impl Signer for CountingSigner {
        async fn sign_hash(&self, hash: &B256) -> alloy::signers::Result<Signature> {
            self.signatures.fetch_add(1, Ordering::SeqCst);
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

    /// One mock node with `script` and `max_retries`, signing with a `CountingSigner`.
    async fn run_counting(script: Vec<Reply>, max_retries: u32) -> (Arc<MockClient>, usize) {
        let counting = Arc::new(CountingSigner {
            inner: PrivateKeySigner::random(),
            signatures: AtomicUsize::new(0),
        });
        let signer = Arc::new(RequestSigner::new(
            counting.clone(),
            test_domain(),
            Duration::from_secs(60),
        ));
        let mock = MockClient::new(vec![script], ms(10));
        let call = MockClient::call_config(Duration::from_secs(5), max_retries);
        let caller = mock.clone().caller_with(call, signer);
        let result: CallResult<UserDecryptionResponse> = caller
            .call(
                &caller.endpoints()[0],
                USER_DECRYPTION_ROUTE,
                Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                Instant::now() + Duration::from_secs(5),
                CancellationToken::new(),
            )
            .await;
        assert!(result.result.is_ok(), "{:?}", result.result.err());
        (mock, counting.signatures.load(Ordering::SeqCst))
    }

    #[tokio::test(start_paused = true)]
    async fn each_node_gets_its_own_header_reused_across_retries() {
        let mock = MockClient::new(vec![vec![RATE, OK], vec![OK]], ms(10));
        let caller = mock.clone().caller(Duration::from_secs(5), 1);
        for endpoint in caller.endpoints() {
            let result: CallResult<UserDecryptionResponse> = caller
                .call(
                    endpoint,
                    USER_DECRYPTION_ROUTE,
                    Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                    Instant::now() + Duration::from_secs(5),
                    CancellationToken::new(),
                )
                .await;
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
        let (mock, signatures) =
            run_counting(vec![Reply::Error(ErrorCode::AuthExpired), OK], 1).await;
        assert_eq!(mock.attempts(0), 2);
        assert_eq!(signatures, 2);

        let (mock, signatures) = run_counting(vec![RATE, OK], 1).await;
        assert_eq!(mock.attempts(0), 2);
        assert_eq!(signatures, 1);
    }

    #[test]
    fn accessors_echo_the_config() {
        let mock = MockClient::new(vec![vec![OK], vec![OK]], ms(1));
        let caller = mock.caller(Duration::from_secs(3), 0);
        assert_eq!(caller.endpoints().len(), 2);
        assert_eq!(caller.endpoints()[1].name, "node-1");
        assert_eq!(caller.timeout(), Duration::from_secs(3));
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
