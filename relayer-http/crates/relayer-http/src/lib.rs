//! Zama relayer over HTTP. Reading order: `kms_aggregator/docs.md`, then `kms_aggregator/aggregator.rs`.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic
    )
)]

pub mod kms_aggregator;
pub mod logging;
pub mod settings;
