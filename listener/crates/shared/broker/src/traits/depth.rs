//! Queue depth introspection API.
//!
//! Provides a backend-agnostic way to query message counts across
//! the principal, retry, and dead-letter queues for a given topic.
//!
//! When a consumer `group` is specified, backends that support it (Redis)
//! also return **pending** (PEL) and **lag** (undelivered) counts, enabling
//! callers to determine whether a consumer will receive messages without
//! relying solely on the raw stream length.

use async_trait::async_trait;

/// Message counts for all queues in a topic's queue group.
///
/// Covers the three queue tiers used by both Redis Streams and AMQP backends:
/// - **principal**: the main processing queue/stream (XLEN for Redis, ready count for AMQP)
/// - **retry**: messages awaiting retry (AMQP retry queue; `None` for Redis
///   since retry is PEL-based, not a separate stream)
/// - **dead_letter**: messages that exhausted their retry budget
///
/// When queried with a consumer group, two additional fields are populated:
/// - **pending**: messages delivered but not yet ACKed (PEL count)
/// - **lag**: messages not yet delivered to the group (Redis 7.0+ `XINFO GROUPS` lag)
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct QueueDepths {
    /// Messages in the principal queue/stream.
    pub principal: u64,
    /// Messages in the retry queue (AMQP only; `None` for backends
    /// where retry is not a separate queue, e.g. Redis PEL-based retry).
    pub retry: Option<u64>,
    /// Messages in the dead-letter queue/stream.
    pub dead_letter: u64,
    /// Pending entries: delivered to a consumer but not yet ACKed (PEL count).
    /// `None` when no group was specified or the backend doesn't track this.
    pub pending: Option<u64>,
    /// Consumer group lag: entries in the stream not yet delivered to the group.
    /// Populated from Redis 7.0+ `XINFO GROUPS` `lag` field.
    /// `None` when no group was specified, the group doesn't exist, or Redis < 7.0.
    pub lag: Option<u64>,
}

impl QueueDepths {
    /// Total messages across all queues (principal + retry + dead-letter).
    ///
    /// Does **not** include `pending`/`lag` — those are different views of
    /// entries already counted in `principal`.
    #[must_use]
    pub fn total(&self) -> u64 {
        self.principal + self.retry.unwrap_or(0) + self.dead_letter
    }

    /// Returns `true` if all queues are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    /// Returns `true` if a consumer in the queried group will receive at
    /// least one message — either from PEL drain or new delivery.
    ///
    /// Decision logic (group-level metrics available):
    /// 1. If `pending > 0` → consumer's drain phase will deliver PEL entries.
    /// 2. If `lag` is `Some(n)` where `n > 0` → stream has undelivered entries.
    /// 3. If `pending == Some(0)` and `lag == None` (Redis < 7.0) → unknown lag,
    ///    conservatively returns `false` so the caller publishes a seed. An
    ///    unnecessary seed is harmless (duplicates self-correct on `UpToDate`).
    ///
    /// Fallback (no group queried, or AMQP where `pending`/`lag` are `None`):
    /// Returns `principal > 0`.
    #[must_use]
    pub fn has_pending_work(&self) -> bool {
        match (self.pending, self.lag) {
            // Group was queried and both metrics are known.
            (Some(pending), Some(lag)) => pending > 0 || lag > 0,
            // PEL is non-empty — drain will deliver regardless of lag.
            (Some(pending), None) if pending > 0 => true,
            // PEL is empty, lag unknown (Redis < 7.0) — can't be sure,
            // return false so the caller seeds conservatively.
            (Some(0), None) => false,
            // No group-level metrics at all (no group queried, or AMQP).
            // Fall back to stream-level heuristic.
            (None, _) => self.principal > 0,
            // Shouldn't happen (lag without pending), but handle gracefully.
            (Some(_), None) => true,
        }
    }
}

/// Trait for inspecting queue/stream depth across backends.
///
/// Implementations derive the full set of queue names (principal, retry,
/// dead-letter) from the given `name` using backend-specific conventions:
///
/// - **Redis**: principal = `{name}`, dead = `{name}:dead`
/// - **AMQP**: principal = `{name}`, retry = `{name}.retry`, dead = `{name}.error`
///
/// When `group` is `Some`, Redis backends also query `XINFO GROUPS` to
/// populate `pending` and `lag` fields on the returned [`QueueDepths`].
#[async_trait]
pub trait QueueInspector: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Returns the current message counts for the queue group identified by `name`.
    ///
    /// - `name`: logical queue/stream name (e.g., `"ethereum.blocks"`).
    /// - `group`: optional consumer group name. When provided, Redis populates
    ///   `pending` (PEL count) and `lag` (undelivered count) on the result.
    ///   AMQP ignores this parameter.
    async fn queue_depths(
        &self,
        name: &str,
        group: Option<&str>,
    ) -> Result<QueueDepths, Self::Error>;

    /// Returns `true` if the consumer group has **no** messages to receive —
    /// neither pending (PEL) nor undelivered (lag).
    ///
    /// This is a fast, single-round-trip check designed for the seed-message
    /// decision at startup. It answers: "will the consumer block forever if
    /// I don't publish a seed?"
    ///
    /// - **Redis**: single `XINFO GROUPS` call → checks `pending` and `lag`.
    ///   Returns `true` if stream/group doesn't exist.
    /// - **AMQP**: single passive `queue_declare` → checks `message_count`.
    ///   Returns `true` if queue doesn't exist.
    async fn is_empty(&self, name: &str, group: &str) -> Result<bool, Self::Error>;

    /// Returns `true` if the consumer group is caught up — either fully idle
    /// or has at most one message currently being consumed (pending <= 1, lag == 0).
    ///
    /// Designed for deduplication guards: when the prefetch count is 1, a single
    /// pending entry means a consumer is already processing the message. Callers
    /// can use this to skip duplicate work rather than enqueueing an overlapping task.
    ///
    /// - **Redis**: single `XINFO GROUPS` call — returns `true` when `pending` is
    ///   0 or 1 **and** `lag` is 0. Returns `true` if stream/group doesn't exist.
    /// - **AMQP**: equivalent to [`is_empty`](Self::is_empty) — AMQP has no
    ///   pending/lag distinction, so returns `true` when `message_count == 0`.
    async fn is_empty_or_pending(&self, name: &str, _group: &str) -> Result<bool, Self::Error>;

    /// Returns `true` if the queue or stream identified by `name` exists.
    ///
    /// - **Redis**: checks key type via `TYPE` command — returns `true` only
    ///   for keys of type `stream`.
    /// - **AMQP**: passive `queue_declare` — returns `true` if the broker
    ///   acknowledges the queue. Returns `false` on `NOT_FOUND` (404).
    ///
    /// Does **not** check consumer group existence — only the underlying
    /// queue/stream.
    async fn exists(&self, name: &str) -> Result<bool, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_depths_total() {
        let depths = QueueDepths {
            principal: 10,
            retry: Some(5),
            dead_letter: 2,
            pending: None,
            lag: None,
        };
        assert_eq!(depths.total(), 17);

        let no_retry = QueueDepths {
            principal: 10,
            retry: None,
            dead_letter: 2,
            pending: None,
            lag: None,
        };
        assert_eq!(no_retry.total(), 12);
    }

    #[test]
    fn queue_depths_is_empty() {
        let empty = QueueDepths {
            principal: 0,
            retry: None,
            dead_letter: 0,
            pending: None,
            lag: None,
        };
        assert!(empty.is_empty());

        let non_empty = QueueDepths {
            principal: 1,
            retry: None,
            dead_letter: 0,
            pending: None,
            lag: None,
        };
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn has_pending_work_with_pending() {
        let depths = QueueDepths {
            principal: 5,
            retry: None,
            dead_letter: 0,
            pending: Some(2),
            lag: Some(0),
        };
        assert!(depths.has_pending_work());
    }

    #[test]
    fn has_pending_work_with_lag() {
        let depths = QueueDepths {
            principal: 5,
            retry: None,
            dead_letter: 0,
            pending: Some(0),
            lag: Some(3),
        };
        assert!(depths.has_pending_work());
    }

    #[test]
    fn has_pending_work_none_falls_back_to_principal() {
        let depths = QueueDepths {
            principal: 5,
            retry: None,
            dead_letter: 0,
            pending: None,
            lag: None,
        };
        assert!(depths.has_pending_work());

        let empty = QueueDepths {
            principal: 0,
            retry: None,
            dead_letter: 0,
            pending: None,
            lag: None,
        };
        assert!(!empty.has_pending_work());
    }

    #[test]
    fn has_pending_work_no_work() {
        let depths = QueueDepths {
            principal: 10,
            retry: None,
            dead_letter: 0,
            pending: Some(0),
            lag: Some(0),
        };
        // principal=10 but all delivered and ACKed (just not trimmed yet)
        assert!(!depths.has_pending_work());
    }

    #[test]
    fn has_pending_work_redis_6_unknown_lag_is_conservative() {
        // Redis < 7.0: pending is known (0), but lag is not available.
        // Must return false so the caller publishes a seed as precaution.
        let depths = QueueDepths {
            principal: 50, // stream has entries, but all may be consumed
            retry: None,
            dead_letter: 0,
            pending: Some(0),
            lag: None, // Redis < 7.0: no lag field
        };
        assert!(
            !depths.has_pending_work(),
            "unknown lag with empty PEL must return false (conservative seed)"
        );
    }

    #[test]
    fn has_pending_work_redis_6_pending_nonzero_still_true() {
        // Redis < 7.0: lag unknown, but PEL has entries → drain will deliver.
        let depths = QueueDepths {
            principal: 50,
            retry: None,
            dead_letter: 0,
            pending: Some(3),
            lag: None,
        };
        assert!(
            depths.has_pending_work(),
            "non-zero pending means drain phase will deliver"
        );
    }
}
