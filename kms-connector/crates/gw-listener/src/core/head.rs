use crate::{core::publish::ChainName, monitoring::metrics::RPC_HEAD_BLOCK};
use alloy::transports::http::reqwest::Url;
use prometheus::IntGauge;
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Tracks the head reported by a chain's RPC node, and warns when it stops advancing.
///
/// From the listener's side, an idle chain and a lagging RPC node look the same: polling finds no
/// new block. This makes the second case visible in the logs.
pub struct HeadTracker {
    chain: ChainName,
    /// The RPC node host, logged instead of the full URL, which may contain an API key.
    rpc_host: String,
    /// How long the head may stay the same before a warning, and the delay between two warnings.
    stale_threshold: Duration,
    /// The highest head reported so far, and when it was first reported.
    highest_head: Option<(u64, Instant)>,
    /// When the last stale head warning was logged, while the head has not advanced since.
    warned_at: Option<Instant>,
    /// The `RPC_HEAD_BLOCK` gauge of the chain.
    head_gauge: IntGauge,
}

impl HeadTracker {
    pub fn new(chain: ChainName, rpc_url: &Url, stale_threshold: Duration) -> Self {
        Self {
            chain,
            rpc_host: rpc_url.host_str().unwrap_or("unknown host").to_string(),
            stale_threshold,
            highest_head: None,
            warned_at: None,
            head_gauge: RPC_HEAD_BLOCK.with_label_values(&[chain.as_str()]),
        }
    }

    /// Records the head reported by the RPC node at `now`, and sets the `RPC_HEAD_BLOCK` gauge.
    ///
    /// Warns, at most once per `stale_threshold`, once the head has not advanced for
    /// `stale_threshold`. Logs once when the head advances again after a warning.
    pub fn observe(&mut self, head: u64, next_block_to_poll: u64, now: Instant) {
        let chain = self.chain.as_str();
        let (head_name, idle_cause) = self.chain_wording();
        self.head_gauge.set(i64::try_from(head).unwrap_or(i64::MAX));

        let (highest_head, advanced_at) = match self.highest_head {
            Some((highest_head, advanced_at)) if head <= highest_head => {
                (highest_head, advanced_at)
            }
            previous => {
                if let (Some((_, advanced_at)), Some(_)) = (previous, self.warned_at.take()) {
                    info!(
                        "The {chain} {head_name} reported by RPC node {} advanced to block {head} \
                        after {}s",
                        self.rpc_host,
                        now.saturating_duration_since(advanced_at).as_secs(),
                    );
                }
                self.highest_head = Some((head, now));
                return;
            }
        };

        let stale_for = now.saturating_duration_since(advanced_at);
        let warned_recently = self.warned_at.is_some_and(|warned_at| {
            now.saturating_duration_since(warned_at) < self.stale_threshold
        });
        if stale_for < self.stale_threshold || warned_recently {
            return;
        }

        self.warned_at = Some(now);
        warn!(
            "The {chain} {head_name} reported by RPC node {} has stayed at block {highest_head} \
            for {}s (latest reported: {head}, next block to poll: {next_block_to_poll}): either \
            {idle_cause} or the RPC node is lagging",
            self.rpc_host,
            stale_for.as_secs(),
        );
    }

    /// The name of the tracked head, and the cause other than a lagging RPC node for it to stall.
    fn chain_wording(&self) -> (&'static str, &'static str) {
        match self.chain {
            ChainName::Gateway => ("head", "the chain is idle"),
            ChainName::Ethereum => ("finalized block", "finality is delayed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use tracing_test::traced_test;

    const THRESHOLD: Duration = Duration::from_secs(60);
    const STALE: &str = "has stayed at block";
    const ADVANCED: &str = "advanced to block";

    fn tracker() -> HeadTracker {
        let url = Url::from_str("https://user:api-key@rpc.example.com/v1/secret-key").unwrap();
        HeadTracker::new(ChainName::Gateway, &url, THRESHOLD)
    }

    #[test]
    #[traced_test]
    fn no_warning_while_the_head_advances() {
        let mut tracker = tracker();
        let start = Instant::now();
        for i in 0..10 {
            tracker.observe(
                100 + i,
                100 + i,
                start + THRESHOLD * u32::try_from(i).unwrap(),
            );
        }
        assert!(!logs_contain(STALE));
        assert!(!logs_contain(ADVANCED));
    }

    #[test]
    #[traced_test]
    fn no_warning_before_the_threshold() {
        let mut tracker = tracker();
        let start = Instant::now();
        tracker.observe(100, 101, start);
        tracker.observe(100, 101, start + THRESHOLD - Duration::from_secs(1));
        assert!(!logs_contain(STALE));
    }

    #[test]
    #[traced_test]
    fn warns_once_per_threshold_while_stale() {
        let mut tracker = tracker();
        let start = Instant::now();
        tracker.observe(100, 101, start);
        tracker.observe(100, 101, start + THRESHOLD);
        tracker.observe(100, 101, start + THRESHOLD + Duration::from_secs(1));
        tracker.observe(100, 101, start + 2 * THRESHOLD - Duration::from_secs(1));
        logs_assert(
            |lines| match lines.iter().filter(|l| l.contains(STALE)).count() {
                1 => Ok(()),
                n => Err(format!("expected 1 stale head warning, got {n}")),
            },
        );

        tracker.observe(100, 101, start + 2 * THRESHOLD);
        logs_assert(
            |lines| match lines.iter().filter(|l| l.contains(STALE)).count() {
                2 => Ok(()),
                n => Err(format!("expected 2 stale head warnings, got {n}")),
            },
        );
        assert!(logs_contain("for 120s"));
        assert!(logs_contain("rpc.example.com"));
        assert!(!logs_contain("api-key"));
        assert!(!logs_contain("secret-key"));
    }

    #[test]
    #[traced_test]
    fn a_lower_head_is_not_progress() {
        let mut tracker = tracker();
        let start = Instant::now();
        tracker.observe(100, 101, start);
        tracker.observe(99, 101, start + THRESHOLD / 2);
        tracker.observe(99, 101, start + THRESHOLD);
        assert!(logs_contain(
            "has stayed at block 100 for 60s (latest reported: 99"
        ));
    }

    #[test]
    #[traced_test]
    fn logs_when_the_head_advances_after_a_warning() {
        let mut tracker = tracker();
        let start = Instant::now();
        tracker.observe(100, 101, start);
        tracker.observe(100, 101, start + THRESHOLD);
        tracker.observe(101, 101, start + 3 * THRESHOLD);
        assert!(logs_contain("advanced to block 101 after 180s"));

        // Later advances are not logged, as no warning preceded them
        tracker.observe(102, 102, start + 3 * THRESHOLD + Duration::from_secs(1));
        logs_assert(
            |lines| match lines.iter().filter(|l| l.contains(ADVANCED)).count() {
                1 => Ok(()),
                n => Err(format!("expected 1 head advance log, got {n}")),
            },
        );

        // The warning delay starts over from the new head
        tracker.observe(101, 102, start + 4 * THRESHOLD - Duration::from_secs(1));
        logs_assert(
            |lines| match lines.iter().filter(|l| l.contains(STALE)).count() {
                1 => Ok(()),
                n => Err(format!("expected 1 stale head warning, got {n}")),
            },
        );
    }

    #[test]
    fn sets_the_gauge_to_the_reported_head() {
        let mut tracker = tracker();
        // A gauge of its own, as other tests set the registered one concurrently.
        tracker.head_gauge = IntGauge::new("test_rpc_head_block", "test").unwrap();
        let start = Instant::now();

        tracker.observe(100, 101, start);
        assert_eq!(tracker.head_gauge.get(), 100);
        // The gauge shows what the RPC node reports, even when it goes backwards.
        tracker.observe(99, 101, start);
        assert_eq!(tracker.head_gauge.get(), 99);
    }

    #[test]
    #[traced_test]
    fn no_recovery_log_without_a_warning() {
        let mut tracker = tracker();
        let start = Instant::now();
        tracker.observe(100, 101, start);
        tracker.observe(101, 102, start + THRESHOLD / 2);
        assert!(!logs_contain(ADVANCED));
    }
}
