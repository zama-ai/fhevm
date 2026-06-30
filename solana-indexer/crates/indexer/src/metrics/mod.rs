//! Prometheus registry + the indexer's counters/gauges.

pub mod server;

use prometheus::{IntCounter, IntGauge, Registry};
use std::sync::Arc;

/// All indexer metrics, registered against one registry shared with the server.
#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,
    /// EV-ACL instructions decoded by the pipeline.
    pub instructions_decoded: IntCounter,
    /// Lineage events appended to the DB.
    pub events_appended: IntCounter,
    /// `/build_proof` responses served (200).
    pub proofs_served: IntCounter,
    /// build_verified_proof attempts that could not reach/decode the chain.
    pub rpc_verify_failures: IntCounter,
    /// Last-processed slot (resume cursor).
    pub cursor_slot: IntGauge,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        let registry = Registry::new();
        let instructions_decoded = IntCounter::new(
            "indexer_instructions_decoded",
            "EV-ACL instructions decoded",
        )
        .unwrap();
        let events_appended =
            IntCounter::new("indexer_events_appended", "lineage events appended").unwrap();
        let proofs_served =
            IntCounter::new("indexer_proofs_served", "build_proof responses served").unwrap();
        let rpc_verify_failures = IntCounter::new(
            "indexer_rpc_verify_failures",
            "on-chain cross-check attempts that failed",
        )
        .unwrap();
        let cursor_slot =
            IntGauge::new("indexer_cursor_slot", "last-processed slot (resume cursor)").unwrap();

        registry
            .register(Box::new(instructions_decoded.clone()))
            .unwrap();
        registry
            .register(Box::new(events_appended.clone()))
            .unwrap();
        registry.register(Box::new(proofs_served.clone())).unwrap();
        registry
            .register(Box::new(rpc_verify_failures.clone()))
            .unwrap();
        registry.register(Box::new(cursor_slot.clone())).unwrap();

        Arc::new(Self {
            registry,
            instructions_decoded,
            events_appended,
            proofs_served,
            rpc_verify_failures,
            cursor_slot,
        })
    }
}
