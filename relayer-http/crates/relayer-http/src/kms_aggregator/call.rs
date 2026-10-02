//! One node call: semaphore permit → POST → decode → retry with backoff, cancellable and bounded by the deadline.

use std::sync::Arc;

use bytes::Bytes;
use kms_connector_api::ErrorCode;
use serde::de::DeserializeOwned;
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant, sleep};
use tokio_util::sync::CancellationToken;
use tracing::warn;

use super::client::{AttemptError, ConnectorClient, Endpoint, decode};
use super::config::{CallConfig, ConfigError, KmsAggregatorConfig};

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

/// What one node call produced, and what it cost.
pub struct CallResult<R> {
    pub node: String,
    pub attempts: u32,
    pub elapsed: Duration,
    pub result: Result<R, CallError>,
}

/// Shared by every aggregation: the HTTP client, the endpoints, the call semaphore and the retry policy.
pub struct Caller {
    client: Arc<dyn ConnectorClient>,
    endpoints: Vec<Endpoint>,
    cfg: CallConfig,
    permits: Semaphore,
}

impl Caller {
    /// Validates the config, resolves every endpoint (API keys from the environment) and sizes the semaphore.
    pub fn new(
        cfg: &KmsAggregatorConfig,
        client: Arc<dyn ConnectorClient>,
    ) -> Result<Self, ConfigError> {
        // The deadline arithmetic and the semaphore size rely on these bounds.
        cfg.validate()?;
        let endpoints = cfg
            .endpoints
            .iter()
            .map(Endpoint::from_config)
            .collect::<Result<_, _>>()?;
        let permits = Semaphore::new(cfg.max_concurrent_calls.min(Semaphore::MAX_PERMITS));
        Ok(Self {
            client,
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
        let mut attempts = 0;
        let result = tokio::select! {
            biased;
            () = cancel.cancelled() => Err(CallError::Cancelled),
            result = self.attempt_loop(endpoint, route, body, deadline, &mut attempts) => result,
        };
        CallResult {
            node: endpoint.name.clone(),
            attempts,
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
    ) -> Result<R, CallError> {
        loop {
            let Ok(permit) = self.permits.acquire().await else {
                return Err(CallError::Failed(AttemptError::Transport(
                    "call limiter closed".to_owned(),
                )));
            };
            *attempts += 1;
            let attempt_started = Instant::now();
            let outcome = self
                .client
                .post(endpoint, route, body.clone())
                .await
                .and_then(decode::<R>);
            // Never hold a slot while sleeping.
            drop(permit);
            let failure = match outcome {
                Ok(value) => return Ok(value),
                Err(failure) => failure,
            };
            let delay = self.cfg.retries.delay_for(*attempts);
            let fits = delay < deadline.saturating_duration_since(Instant::now());
            // Never retry an authentication rejection (our key is wrong for that node), a non-retryable
            // error, past `max_retries`, or when the delay would end after the deadline.
            let gives_up = failure.is_auth()
                || !failure.is_retryable()
                || *attempts > self.cfg.retries.max_retries
                || !fits;
            attempt_failed(
                &endpoint.name,
                *attempts,
                attempt_started.elapsed(),
                &failure,
                (!gives_up).then_some(delay),
            );
            if gives_up {
                return Err(CallError::Failed(failure));
            }
            sleep(delay).await;
        }
    }
}

/// One failed attempt, visible at the default level; the node task runs in the `aggregation` span, so the line
/// also carries the request's identifiers. `delay` is `None` when the call gives up.
pub(super) fn attempt_failed(
    node: &str,
    attempt: u32,
    elapsed: Duration,
    failure: &AttemptError,
    delay: Option<Duration>,
) {
    warn!(
        node,
        attempt,
        elapsed_ms = elapsed.as_millis() as u64,
        error = %failure,
        code = failure.code().as_str(),
        retry_in_ms = delay.map(|d| d.as_millis() as u64),
        "attempt failed"
    );
}

#[cfg(test)]
pub(crate) mod tests {
    use kms_connector_api::{ErrorCode, USER_DECRYPTION_ROUTE, UserDecryptionResponse};

    use super::*;
    use crate::kms_aggregator::config::tests::valid;
    use crate::kms_aggregator::mock::{Fixed, MockClient, Reply, USER_REQUEST_JSON};
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

    /// One mock node with `script`, `max_retries`, a 5 s deadline, 10 ms per attempt.
    async fn run(
        script: Vec<Reply>,
        max_retries: u32,
    ) -> (Arc<MockClient>, CallResult<UserDecryptionResponse>) {
        let mock = MockClient::new(vec![script], ms(10));
        let caller = mock.clone().caller(Duration::from_secs(5), max_retries);
        let result = caller
            .call(
                &caller.endpoints()[0],
                USER_DECRYPTION_ROUTE,
                Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                Instant::now() + Duration::from_secs(5),
                CancellationToken::new(),
            )
            .await;
        (mock, result)
    }

    /// Captures the `attempt failed` lines of this thread.
    pub(crate) async fn capture_attempts() -> (tracing::subscriber::DefaultGuard, Sink) {
        capture("attempt failed", 1, || {
            attempt_failed("probe", 1, Duration::ZERO, &AttemptError::Status(500), None);
        })
        .await
    }

    #[tokio::test(start_paused = true)]
    async fn every_failed_attempt_is_a_warn_with_its_decision() {
        let (_guard, sink) = capture_attempts().await;
        let (_, r) = run(vec![RATE, MALFORMED], 2).await;
        assert_eq!(r.attempts, 2);
        let lines = sink.lines("attempt failed");
        assert_eq!(lines.len(), 2, "{lines:?}");
        for (line, attempt) in lines.iter().zip(1..) {
            assert_eq!(line["level"], "WARN");
            assert_eq!(line["fields"]["node"], "node-0");
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
    async fn retryable_error_is_retried_with_backoff_then_gives_up() {
        let (mock, r) = run(vec![ACL], 2).await;
        assert_eq!(r.result.unwrap_err().code(), Some(ErrorCode::AclDenied));
        assert_eq!(mock.attempts(0), 3);
        // 3 attempts of 10 ms, delays 500 ms then 1 s.
        assert_eq!((r.attempts, r.elapsed), (3, ms(1530)));
    }

    #[tokio::test(start_paused = true)]
    async fn rate_limited_twice_then_success() {
        let (_, r) = run(vec![RATE, RATE, OK], 2).await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (3, ms(1530)));
    }

    #[tokio::test(start_paused = true)]
    async fn transport_error_is_retried() {
        let (_, r) = run(vec![REFUSED, OK], 1).await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (2, ms(520)));
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
    async fn backoff_is_capped_by_backoff_max() {
        // Delays: 500ms, 1s, 2s, 4s, 4s (cap) = 11.5 s + 6 attempts of 10 ms; deadline 60 s.
        let mock = MockClient::new(vec![vec![RATE, RATE, RATE, RATE, RATE, OK]], ms(10));
        let caller = mock.clone().caller(Duration::from_secs(60), 5);
        let r: CallResult<UserDecryptionResponse> = caller
            .call(
                &caller.endpoints()[0],
                USER_DECRYPTION_ROUTE,
                Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                Instant::now() + Duration::from_secs(60),
                CancellationToken::new(),
            )
            .await;
        assert!(r.result.is_ok());
        assert_eq!((r.attempts, r.elapsed), (6, ms(11_560)));
    }

    #[tokio::test(start_paused = true)]
    async fn retry_that_does_not_fit_before_the_deadline_is_skipped() {
        let mock = MockClient::new(vec![vec![ACL, OK]], ms(10));
        let caller = mock.clone().caller(Duration::from_secs(5), 3);
        let r: CallResult<UserDecryptionResponse> = caller
            .call(
                &caller.endpoints()[0],
                USER_DECRYPTION_ROUTE,
                Bytes::from_static(USER_REQUEST_JSON.as_bytes()),
                Instant::now() + ms(400),
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
    fn new_rejects_an_invalid_config_and_a_missing_key() {
        let mock = MockClient::new(vec![vec![OK]], ms(1));
        let mut cfg = valid(1);
        cfg.call.timeout = Duration::ZERO;
        let e = Caller::new(&cfg, mock.clone()).err().unwrap();
        assert!(e.0.contains("call.timeout"), "{e}");
        let e = Caller::new(&valid(1), mock).err().unwrap();
        assert!(e.0.contains("KMS_00_API_KEY is not set"), "{e}");
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
