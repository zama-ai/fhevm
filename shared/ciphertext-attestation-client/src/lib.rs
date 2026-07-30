//! Networked half of off-chain ciphertext attestation consensus.
//!
//! Pairs with the pure `ciphertext-attestation` crate, which owns the wire types, signing, and
//! [`ciphertext_attestation::consensus::evaluate`]. This crate adds everything that touches the
//! network on the verifier side: a TTL'd mirror of the on-chain Coprocessor registry
//! ([`registry`]), the S3 `HEAD` attestation fetch ([`s3`]), and the fan-out that turns a handle
//! into a resolved [`fetch::ResolvedConsensus`] ([`fetch`]).
//!
//! Consumer flow:
//!
//! ```ignore
//! let registry =
//!     CoprocessorRegistry::connect(provider, gateway_config_address, refresh_interval, cancel_token)
//!         .await?;
//! let snapshot = registry.snapshot();
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
pub use registry::{CoprocessorRegistry, CoprocessorRegistrySnapshot, RegistryError};
