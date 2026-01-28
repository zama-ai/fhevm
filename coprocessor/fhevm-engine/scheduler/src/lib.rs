//! # Scheduler Module
//!
//! This module provides the core scheduling infrastructure for executing FHE
//! operations in the coprocessor. It orchestrates parallel execution of FHE
//! computations using dataflow graphs to manage dependences between operations.
//!
//! ## Architecture
//!
//! The scheduler uses a dataflow graph (DFG) based approach where:
//! - Operations are represented as nodes in a directed acyclic graph (DAG)
//! - Dependences between operations are represented as edges
//! - The scheduler partitions the graph and executes independent partitions in parallel
//!
//! ## Modules
//!
//! - [`dfg`]: Contains the dataflow graph implementation, partitioning algorithms, and the
//!   main scheduler execution engine.
//!
//! ## Metrics
//!
//! The module exports Prometheus metrics for monitoring:
//! - `coprocessor_rerand_batch_latency_seconds`: Latency of re-randomisation operations
//! - `coprocessor_fhe_batch_latency_seconds`: Latency of FHE batch operations

use fhevm_engine_common::telemetry::{register_histogram, MetricsConfig};
use prometheus::Histogram;
use std::sync::{LazyLock, OnceLock};

pub mod dfg;

/// Configuration for the re-randomisation latency histogram metric.
pub static RERAND_LATENCY_BATCH_HISTOGRAM_CONF: OnceLock<MetricsConfig> = OnceLock::new();

/// Prometheus histogram tracking re-randomisation latencies per transaction.
pub static RERAND_LATENCY_BATCH_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram(
        RERAND_LATENCY_BATCH_HISTOGRAM_CONF.get(),
        "coprocessor_rerand_batch_latency_seconds",
        "Re-randomisation latencies per transaction in seconds",
    )
});

/// Configuration for the FHE batch latency histogram metric.
pub static FHE_BATCH_LATENCY_HISTOGRAM_CONF: OnceLock<MetricsConfig> = OnceLock::new();

/// Prometheus histogram tracking FHE operation latencies per transaction.
pub static FHE_BATCH_LATENCY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram(
        FHE_BATCH_LATENCY_HISTOGRAM_CONF.get(),
        "coprocessor_fhe_batch_latency_seconds",
        "The latency of FHE operations within a single transaction, in seconds",
    )
});
