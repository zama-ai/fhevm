use std::time::Duration;

/// Per-pipeline tuning for the **live** consumer (`{consumer_id}.new-event`).
///
/// `Default` values match the literals previously baked into
/// `broker_consumer()` in `client.rs`. Callers that don't override anything
/// get bit-identical behavior to the pre-options codepath.
#[derive(Debug, Clone)]
pub struct LiveConsumerOptions {
    prefetch: usize,
    max_retries: u32,
    /// `None` disables the circuit breaker; `Some((threshold, cooldown))` enables it.
    circuit_breaker: Option<(u32, Duration)>,
    /// Redis-only. `None` falls back to the broker default (30s).
    redis_claim_min_idle: Option<Duration>,
    /// Redis-only. `None` falls back to the broker default (10s).
    redis_claim_interval: Option<Duration>,
}

impl Default for LiveConsumerOptions {
    fn default() -> Self {
        Self {
            prefetch: 1,
            max_retries: 3,
            circuit_breaker: Some((3, Duration::from_secs(30))),
            redis_claim_min_idle: Some(Duration::from_secs(60)),
            redis_claim_interval: Some(Duration::from_secs(1)),
        }
    }
}

impl LiveConsumerOptions {
    pub fn with_prefetch(mut self, n: usize) -> Self {
        self.prefetch = n;
        self
    }

    pub fn with_max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    pub fn with_circuit_breaker(mut self, threshold: u32, cooldown: Duration) -> Self {
        self.circuit_breaker = Some((threshold, cooldown));
        self
    }

    pub fn without_circuit_breaker(mut self) -> Self {
        self.circuit_breaker = None;
        self
    }

    pub fn with_redis_claim_min_idle_secs(mut self, secs: u64) -> Self {
        self.redis_claim_min_idle = Some(Duration::from_secs(secs));
        self
    }

    pub fn with_redis_claim_interval_secs(mut self, secs: u64) -> Self {
        self.redis_claim_interval = Some(Duration::from_secs(secs));
        self
    }

    pub(crate) fn prefetch(&self) -> usize {
        self.prefetch
    }

    pub(crate) fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub(crate) fn circuit_breaker(&self) -> Option<(u32, Duration)> {
        self.circuit_breaker
    }

    pub(crate) fn redis_claim_min_idle(&self) -> Option<Duration> {
        self.redis_claim_min_idle
    }

    pub(crate) fn redis_claim_interval(&self) -> Option<Duration> {
        self.redis_claim_interval
    }
}

/// Per-pipeline tuning for the **catchup** consumer (`{consumer_id}.catchup-event`).
///
/// `Default` values match the hardened catchup literals previously baked into
/// `broker_catchup_consumer()` in `client.rs`.
#[derive(Debug, Clone)]
pub struct CatchupConsumerOptions {
    prefetch: usize,
    max_retries: u32,
    circuit_breaker: Option<(u32, Duration)>,
    redis_claim_min_idle: Option<Duration>,
    redis_claim_interval: Option<Duration>,
}

impl Default for CatchupConsumerOptions {
    fn default() -> Self {
        Self {
            prefetch: 1,
            max_retries: 5,
            circuit_breaker: Some((5, Duration::from_secs(60))),
            redis_claim_min_idle: Some(Duration::from_secs(120)),
            redis_claim_interval: Some(Duration::from_secs(5)),
        }
    }
}

impl CatchupConsumerOptions {
    pub fn with_prefetch(mut self, n: usize) -> Self {
        self.prefetch = n;
        self
    }

    pub fn with_max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    pub fn with_circuit_breaker(mut self, threshold: u32, cooldown: Duration) -> Self {
        self.circuit_breaker = Some((threshold, cooldown));
        self
    }

    pub fn without_circuit_breaker(mut self) -> Self {
        self.circuit_breaker = None;
        self
    }

    pub fn with_redis_claim_min_idle_secs(mut self, secs: u64) -> Self {
        self.redis_claim_min_idle = Some(Duration::from_secs(secs));
        self
    }

    pub fn with_redis_claim_interval_secs(mut self, secs: u64) -> Self {
        self.redis_claim_interval = Some(Duration::from_secs(secs));
        self
    }

    pub(crate) fn prefetch(&self) -> usize {
        self.prefetch
    }

    pub(crate) fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub(crate) fn circuit_breaker(&self) -> Option<(u32, Duration)> {
        self.circuit_breaker
    }

    pub(crate) fn redis_claim_min_idle(&self) -> Option<Duration> {
        self.redis_claim_min_idle
    }

    pub(crate) fn redis_claim_interval(&self) -> Option<Duration> {
        self.redis_claim_interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_defaults_match_previous_hardcoded_values() {
        let o = LiveConsumerOptions::default();
        assert_eq!(o.prefetch(), 1);
        assert_eq!(o.max_retries(), 3);
        assert_eq!(o.circuit_breaker(), Some((3, Duration::from_secs(30))));
        assert_eq!(o.redis_claim_min_idle(), Some(Duration::from_secs(60)));
        assert_eq!(o.redis_claim_interval(), Some(Duration::from_secs(1)));
    }

    #[test]
    fn catchup_defaults_match_previous_hardcoded_values() {
        let o = CatchupConsumerOptions::default();
        assert_eq!(o.prefetch(), 1);
        assert_eq!(o.max_retries(), 5);
        assert_eq!(o.circuit_breaker(), Some((5, Duration::from_secs(60))));
        assert_eq!(o.redis_claim_min_idle(), Some(Duration::from_secs(120)));
        assert_eq!(o.redis_claim_interval(), Some(Duration::from_secs(5)));
    }

    #[test]
    fn live_builder_overrides_individual_fields() {
        let o = LiveConsumerOptions::default()
            .with_prefetch(8)
            .with_max_retries(7)
            .with_circuit_breaker(9, Duration::from_secs(45))
            .with_redis_claim_min_idle_secs(90)
            .with_redis_claim_interval_secs(2);

        assert_eq!(o.prefetch(), 8);
        assert_eq!(o.max_retries(), 7);
        assert_eq!(o.circuit_breaker(), Some((9, Duration::from_secs(45))));
        assert_eq!(o.redis_claim_min_idle(), Some(Duration::from_secs(90)));
        assert_eq!(o.redis_claim_interval(), Some(Duration::from_secs(2)));
    }

    #[test]
    fn catchup_builder_overrides_individual_fields() {
        let o = CatchupConsumerOptions::default()
            .with_prefetch(16)
            .with_max_retries(8)
            .with_circuit_breaker(10, Duration::from_secs(120))
            .with_redis_claim_min_idle_secs(240)
            .with_redis_claim_interval_secs(10);

        assert_eq!(o.prefetch(), 16);
        assert_eq!(o.max_retries(), 8);
        assert_eq!(o.circuit_breaker(), Some((10, Duration::from_secs(120))));
        assert_eq!(o.redis_claim_min_idle(), Some(Duration::from_secs(240)));
        assert_eq!(o.redis_claim_interval(), Some(Duration::from_secs(10)));
    }

    #[test]
    fn live_without_circuit_breaker_clears_the_breaker() {
        let o = LiveConsumerOptions::default()
            .with_circuit_breaker(9, Duration::from_secs(45))
            .without_circuit_breaker();
        assert_eq!(o.circuit_breaker(), None);
    }

    #[test]
    fn catchup_without_circuit_breaker_clears_the_breaker() {
        let o = CatchupConsumerOptions::default()
            .with_circuit_breaker(9, Duration::from_secs(45))
            .without_circuit_breaker();
        assert_eq!(o.circuit_breaker(), None);
    }
}
