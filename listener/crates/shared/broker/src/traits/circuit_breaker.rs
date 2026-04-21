use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Configuration for the consumer circuit breaker.
///
/// When enabled on a retry or prefetch consumer, the circuit breaker pauses
/// consumption when consecutive `Transient` handler errors exceed the threshold,
/// preventing DLQ pollution during downstream outages (DB down, API timeout, etc.).
///
/// Backend-agnostic: both RMQ and Redis consumers can use this.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of consecutive `Transient` failures required to trip the circuit (default: 5).
    pub failure_threshold: u32,
    /// How long to stay in the Open state before transitioning to Half-Open (default: 30s).
    pub cooldown_duration: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            cooldown_duration: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Prometheus labels attached to a `CircuitBreaker` for metric emission.
///
/// When absent, no `broker_circuit_breaker_*` metrics are emitted. When present,
/// every state transition and counter change produces a Prometheus update.
#[derive(Debug, Clone)]
struct MetricLabels {
    backend: &'static str,
    topic: String,
}

/// Lightweight circuit breaker state machine.
///
/// Tracks consecutive transient (infrastructure) failures and pauses
/// consumption when the threshold is exceeded. Resumes after a cooldown
/// period by allowing a single probe request.
///
/// This struct is internal to consumer implementations — the developer
/// configures it via [`CircuitBreakerConfig`] in the consumer builder.
///
/// Call [`Self::with_labels`] to enable Prometheus emission
/// (`broker_circuit_breaker_state` gauge, `broker_circuit_breaker_trips_total`
/// counter, and `broker_circuit_breaker_consecutive_failures` gauge).
pub struct CircuitBreaker {
    state: CircuitState,
    consecutive_transient_failures: u32,
    config: CircuitBreakerConfig,
    last_opened_at: Option<Instant>,
    labels: Option<MetricLabels>,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: CircuitState::Closed,
            consecutive_transient_failures: 0,
            config,
            last_opened_at: None,
            labels: None,
        }
    }

    /// Attach Prometheus labels and seed the circuit breaker metrics.
    ///
    /// After calling this, the breaker emits:
    /// - `broker_circuit_breaker_state{backend,topic}` (0=closed, 1=open, 2=half-open)
    /// - `broker_circuit_breaker_consecutive_failures{backend,topic}`
    /// - `broker_circuit_breaker_trips_total{backend,topic}` (on each trip)
    ///
    /// Called once at startup; the initial state (Closed, 0 failures) is emitted
    /// immediately so Grafana discovers the time series on the first scrape.
    pub fn with_labels(mut self, backend: &'static str, topic: impl Into<String>) -> Self {
        self.labels = Some(MetricLabels {
            backend,
            topic: topic.into(),
        });
        self.emit_state();
        self.emit_consecutive_failures();
        self
    }

    fn emit_state(&self) {
        if let Some(labels) = &self.labels {
            let code = match self.state {
                CircuitState::Closed => 0.0,
                CircuitState::Open => 1.0,
                CircuitState::HalfOpen => 2.0,
            };
            metrics::gauge!(
                "broker_circuit_breaker_state",
                "backend" => labels.backend,
                "topic" => labels.topic.clone(),
            )
            .set(code);
        }
    }

    fn emit_consecutive_failures(&self) {
        if let Some(labels) = &self.labels {
            metrics::gauge!(
                "broker_circuit_breaker_consecutive_failures",
                "backend" => labels.backend,
                "topic" => labels.topic.clone(),
            )
            .set(self.consecutive_transient_failures as f64);
        }
    }

    fn emit_trip(&self) {
        if let Some(labels) = &self.labels {
            metrics::counter!(
                "broker_circuit_breaker_trips_total",
                "backend" => labels.backend,
                "topic" => labels.topic.clone(),
            )
            .increment(1);
        }
    }

    /// Returns `true` when the consumer should proceed with reading messages.
    pub fn should_allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed | CircuitState::HalfOpen => true,
            CircuitState::Open => {
                if let Some(opened_at) = self.last_opened_at {
                    if opened_at.elapsed() >= self.config.cooldown_duration {
                        self.state = CircuitState::HalfOpen;
                        self.emit_state();
                        info!("Circuit breaker: transitioning to Half-Open (probing)");
                        true
                    } else {
                        false
                    }
                } else {
                    self.state = CircuitState::HalfOpen;
                    self.emit_state();
                    true
                }
            }
        }
    }

    /// Record a successful handler execution.
    ///
    /// - **Closed**: resets the consecutive-transient counter (normal operation).
    /// - **Half-Open**: closes the circuit (probe succeeded).
    /// - **Open**: no-op. An incidental success recorded inside the same XREADGROUP
    ///   batch that tripped the threshold must not bypass the cooldown. The only valid
    ///   `Open → Closed` transition is `cooldown expires → Half-Open probe → success`.
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.consecutive_transient_failures = 0;
                self.emit_consecutive_failures();
            }
            CircuitState::HalfOpen => {
                info!("Circuit breaker: probe succeeded, closing circuit");
                self.consecutive_transient_failures = 0;
                self.state = CircuitState::Closed;
                self.last_opened_at = None;
                self.emit_state();
                self.emit_consecutive_failures();
            }
            CircuitState::Open => {
                // No-op: do not bypass the cooldown.
            }
        }
    }

    /// Record a transient (infrastructure) failure. Trips the circuit at threshold.
    pub fn record_transient_failure(&mut self) {
        self.consecutive_transient_failures += 1;
        self.emit_consecutive_failures();

        match self.state {
            CircuitState::Closed => {
                if self.consecutive_transient_failures >= self.config.failure_threshold {
                    self.state = CircuitState::Open;
                    self.last_opened_at = Some(Instant::now());
                    self.emit_state();
                    self.emit_trip();
                    warn!(
                        consecutive_failures = self.consecutive_transient_failures,
                        cooldown = ?self.config.cooldown_duration,
                        "Circuit breaker: OPEN — pausing consumption"
                    );
                } else {
                    debug!(
                        consecutive_failures = self.consecutive_transient_failures,
                        threshold = self.config.failure_threshold,
                        "Circuit breaker: transient failure recorded"
                    );
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
                self.last_opened_at = Some(Instant::now());
                self.emit_state();
                self.emit_trip();
                warn!(
                    cooldown = ?self.config.cooldown_duration,
                    "Circuit breaker: probe failed, reopening circuit"
                );
            }
            CircuitState::Open => {}
        }
    }

    /// Record a permanent (execution/deserialization) failure.
    /// Does NOT trip the circuit — the message is the problem, not the infrastructure.
    pub fn record_permanent_failure(&mut self) {
        self.consecutive_transient_failures = 0;
        self.emit_consecutive_failures();
        debug!("Circuit breaker: permanent failure recorded, transient counter reset");
    }

    /// Remaining cooldown before the circuit transitions to Half-Open.
    pub fn remaining_cooldown(&self) -> Duration {
        match (self.state, self.last_opened_at) {
            (CircuitState::Open, Some(opened_at)) => self
                .config
                .cooldown_duration
                .checked_sub(opened_at.elapsed())
                .unwrap_or(Duration::ZERO),
            _ => Duration::ZERO,
        }
    }

    #[allow(dead_code)]
    pub fn is_open(&self) -> bool {
        self.state == CircuitState::Open
    }

    #[allow(dead_code)]
    pub fn is_half_open(&self) -> bool {
        self.state == CircuitState::HalfOpen
    }

    #[allow(dead_code)]
    pub fn is_closed(&self) -> bool {
        self.state == CircuitState::Closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> CircuitBreakerConfig {
        CircuitBreakerConfig {
            failure_threshold: 3,
            cooldown_duration: Duration::from_millis(100),
        }
    }

    #[test]
    fn starts_closed() {
        let cb = CircuitBreaker::new(default_config());
        assert!(cb.is_closed());
        assert!(!cb.is_open());
        assert!(!cb.is_half_open());
    }

    #[test]
    fn allows_requests_when_closed() {
        let mut cb = CircuitBreaker::new(default_config());
        assert!(cb.should_allow_request());
    }

    #[test]
    fn stays_closed_below_threshold() {
        let mut cb = CircuitBreaker::new(default_config());
        cb.record_transient_failure();
        assert!(cb.is_closed());
        cb.record_transient_failure();
        assert!(cb.is_closed());
        assert!(cb.should_allow_request());
    }

    #[test]
    fn opens_at_threshold() {
        let mut cb = CircuitBreaker::new(default_config());
        cb.record_transient_failure();
        cb.record_transient_failure();
        cb.record_transient_failure();
        assert!(cb.is_open());
        assert!(!cb.should_allow_request());
    }

    #[test]
    fn blocks_requests_when_open() {
        let mut cb = CircuitBreaker::new(default_config());
        for _ in 0..3 {
            cb.record_transient_failure();
        }
        assert!(cb.is_open());
        assert!(!cb.should_allow_request());
    }

    #[tokio::test]
    async fn transitions_to_half_open_after_cooldown() {
        let mut cb = CircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 1,
            cooldown_duration: Duration::from_millis(50),
        });
        cb.record_transient_failure();
        assert!(cb.is_open());
        tokio::time::sleep(Duration::from_millis(60)).await;
        assert!(cb.should_allow_request());
        assert!(cb.is_half_open());
    }

    #[tokio::test]
    async fn half_open_closes_on_success() {
        let mut cb = CircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 1,
            cooldown_duration: Duration::from_millis(50),
        });
        cb.record_transient_failure();
        tokio::time::sleep(Duration::from_millis(60)).await;
        cb.should_allow_request();
        cb.record_success();
        assert!(cb.is_closed());
        assert!(cb.should_allow_request());
    }

    #[tokio::test]
    async fn half_open_reopens_on_failure() {
        let mut cb = CircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 1,
            cooldown_duration: Duration::from_millis(50),
        });
        cb.record_transient_failure();
        tokio::time::sleep(Duration::from_millis(60)).await;
        cb.should_allow_request();
        cb.record_transient_failure();
        assert!(cb.is_open());
        assert!(!cb.should_allow_request());
    }

    #[test]
    fn success_resets_counter() {
        let mut cb = CircuitBreaker::new(default_config());
        cb.record_transient_failure();
        cb.record_transient_failure();
        cb.record_success();
        cb.record_transient_failure();
        assert!(cb.is_closed());
    }

    #[test]
    fn success_while_open_does_not_close_circuit() {
        // A success recorded in the same message batch that tripped the circuit
        // (e.g. an XREADGROUP batch where entry N trips and entry N+1 succeeds)
        // must not bypass the cooldown by closing the circuit directly from Open.
        let mut cb = CircuitBreaker::new(default_config());
        for _ in 0..3 {
            cb.record_transient_failure();
        }
        assert!(cb.is_open());

        cb.record_success(); // incidental success — must be a no-op
        assert!(
            cb.is_open(),
            "record_success from Open state must not close the circuit"
        );
        assert!(
            !cb.should_allow_request(),
            "circuit must still block requests"
        );
    }

    #[test]
    fn permanent_failure_resets_transient_counter() {
        let mut cb = CircuitBreaker::new(default_config());
        cb.record_transient_failure();
        cb.record_transient_failure();
        cb.record_permanent_failure();
        cb.record_transient_failure();
        assert!(cb.is_closed());
    }

    #[test]
    fn permanent_failure_does_not_trip() {
        let mut cb = CircuitBreaker::new(default_config());
        for _ in 0..100 {
            cb.record_permanent_failure();
        }
        assert!(cb.is_closed());
    }

    #[test]
    fn remaining_cooldown_when_closed() {
        let cb = CircuitBreaker::new(default_config());
        assert_eq!(cb.remaining_cooldown(), Duration::ZERO);
    }

    #[test]
    fn remaining_cooldown_when_open() {
        let mut cb = CircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 1,
            cooldown_duration: Duration::from_secs(30),
        });
        cb.record_transient_failure();
        assert!(cb.is_open());
        let remaining = cb.remaining_cooldown();
        assert!(remaining > Duration::ZERO);
        assert!(remaining <= Duration::from_secs(30));
    }

    #[test]
    fn default_config_values() {
        let config = CircuitBreakerConfig::default();
        assert_eq!(config.failure_threshold, 5);
        assert_eq!(config.cooldown_duration, Duration::from_secs(30));
    }
}
