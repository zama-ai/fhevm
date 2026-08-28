//! Networked half of off-chain ciphertext attestation consensus.
//!
//! Pairs with the pure base half of this crate, which owns the wire types, signing, and
//! [`crate::consensus::evaluate`]. This module adds everything that touches the network on the
//! verifier side: a TTL'd mirror of the on-chain Coprocessor registry ([`registry`]), the S3
//! `HEAD` attestation fetch ([`s3`]), and the fan-out that turns a handle into a resolved
//! [`fetch::ResolvedConsensus`] ([`fetch`]). Gated behind the `client` feature so a consumer that
//! only needs the wire types (signing, verification, `consensus::evaluate`) is not made to pull
//! in `alloy`, `tokio`, and the rest of this module's networked dependencies.
//!
//! Consumer flow:
//!
//! ```ignore
//! let registry =
//!     CoprocessorRegistry::connect(provider, gateway_config_address, refresh_interval, cancel_token)
//!         .await?;
//! let snapshot = registry.snapshot();
//! let client = BoundedClient::new(Client::new(), max_concurrent_heads_per_bucket);
//! let resolved =
//!     fetch_attestations_and_check_consensus(&client, handle, &snapshot, head_timeout, context_id)
//!         .await?;
//! ```
//!
//! See RFC-023 (Off-chain ciphertext commits handling).

pub mod fetch;
pub mod registry;
pub mod s3;

pub use fetch::{ConsensusCheckError, ResolvedConsensus, fetch_attestations_and_check_consensus};
pub use registry::{
    CoprocessorEntry, CoprocessorRegistry, CoprocessorRegistrySnapshot, RegistryError,
};
pub use s3::BoundedClient;
