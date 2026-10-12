//! Networked half of off-chain ciphertext attestation consensus: a TTL'd mirror of the on-chain
//! Coprocessor registry ([`registry`]), the S3 `HEAD` attestation fetch ([`s3`]), and the fan-out
//! that fetches attestations and evaluates consensus ([`fetch`]).
//!
//! Gated behind the `client` feature so consumers can avoid this module's network dependencies.

pub mod fetch;
pub mod registry;
pub mod s3;

pub use fetch::fetch_attestations_and_check_consensus;
pub use registry::{
    CoprocessorRegistry, CoprocessorRegistrySnapshot, CriticalFailurePolicy, RegistryError,
};
pub use s3::{BoundedClient, FetchAttestationError, FetchCiphertextError};
