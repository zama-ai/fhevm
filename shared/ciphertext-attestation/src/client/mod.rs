//! Networked half of off-chain ciphertext attestation consensus: a TTL'd mirror of the on-chain
//! Coprocessor registry ([`registry`]), the S3 `HEAD` attestation fetch ([`s3`]), and the fan-out
//! that turns a handle into a resolved [`fetch::ResolvedConsensus`] ([`fetch`]). Gated behind the
//! `client` feature so a consumer that only needs the wire types is not made to pull in `alloy`,
//! `tokio`, and the rest of this module's networked dependencies.
//!
//! See RFC-023 (Off-chain ciphertext commits handling).

pub mod fetch;
pub mod registry;
pub mod s3;

pub use fetch::{ConsensusCheckError, ResolvedConsensus, fetch_attestations_and_check_consensus};
pub use registry::{
    CoprocessorEntry, CoprocessorRegistry, CoprocessorRegistrySnapshot, RegistryError,
};
pub use s3::{BoundedClient, FetchAttestationError, FetchCiphertextError};
