#![allow(dead_code)]

use ethereum_rpc_mock::SubscriptionTarget;

/// A table entry describing how events should be routed across listeners.
#[derive(Clone)]
pub struct RedundancyCase {
    pub name: &'static str,
    pub listener_count: usize,
    pub targets_per_event: Vec<SubscriptionTarget>,
    pub requests: usize,
}

/// Number of events emitted by the user-decrypt success path (3+3+3+1).
pub const USER_DECRYPT_EVENT_COUNT: usize = 10;

/// Cases that apply to all flow types (user-decrypt, public-decypt and input-proof).
///
/// Note: "All listeners miss events" (e.g., Only(vec![])) is not included here
/// as it results in timeout rather than testing deduplication logic. That
/// scenario is covered by dedicated timeout tests.
pub fn common_redundancy_cases() -> Vec<RedundancyCase> {
    vec![
        RedundancyCase {
            name: "all_listeners",
            listener_count: 3,
            targets_per_event: vec![SubscriptionTarget::All],
            requests: 4,
        },
        RedundancyCase {
            name: "first_only",
            listener_count: 3,
            targets_per_event: vec![SubscriptionTarget::only(vec![0])],
            requests: 4,
        },
        RedundancyCase {
            name: "last_only",
            listener_count: 3,
            targets_per_event: vec![SubscriptionTarget::only(vec![2])],
            requests: 4,
        },
        RedundancyCase {
            name: "two_listeners_alternate",
            listener_count: 2,
            targets_per_event: vec![
                SubscriptionTarget::only(vec![0]),
                SubscriptionTarget::only(vec![1]),
            ],
            requests: 4,
        },
        RedundancyCase {
            name: "three_listeners_round_robin",
            listener_count: 3,
            targets_per_event: vec![
                SubscriptionTarget::only(vec![0]),
                SubscriptionTarget::only(vec![1]),
                SubscriptionTarget::only(vec![2]),
            ],
            requests: 4,
        },
        RedundancyCase {
            name: "duplicate_adjacent",
            listener_count: 3,
            targets_per_event: vec![SubscriptionTarget::only(vec![0, 1])],
            requests: 4,
        },
        RedundancyCase {
            name: "duplicate_non_adjacent",
            listener_count: 3,
            targets_per_event: vec![SubscriptionTarget::only(vec![0, 2])],
            requests: 4,
        },
        RedundancyCase {
            name: "broadcast_then_single",
            listener_count: 3,
            targets_per_event: vec![SubscriptionTarget::All, SubscriptionTarget::only(vec![1])],
            requests: 4,
        },
        RedundancyCase {
            name: "broadcast_then_duplicate",
            listener_count: 3,
            targets_per_event: vec![
                SubscriptionTarget::All,
                SubscriptionTarget::only(vec![0, 2]),
            ],
            requests: 4,
        },
        RedundancyCase {
            name: "duplicate_then_alternate",
            listener_count: 3,
            targets_per_event: vec![
                SubscriptionTarget::only(vec![0, 1]),
                SubscriptionTarget::only(vec![2]),
            ],
            requests: 4,
        },
    ]
}

/// Cases that only apply to multi-event flows (e.g., user-decrypt share streams).
pub fn user_only_redundancy_cases() -> Vec<RedundancyCase> {
    vec![RedundancyCase {
        name: "request_level_alternate",
        listener_count: 3,
        targets_per_event: vec![
            SubscriptionTarget::only(vec![0]),
            SubscriptionTarget::only(vec![1]),
        ],
        requests: 4,
    }]
}

/// Expand a (possibly short) target list to the required event or request count by cycling; default to All.
pub fn expand_targets(
    event_count: usize,
    targets: &[SubscriptionTarget],
) -> Vec<SubscriptionTarget> {
    if targets.is_empty() {
        return vec![SubscriptionTarget::All; event_count];
    }

    targets.iter().cloned().cycle().take(event_count).collect()
}
