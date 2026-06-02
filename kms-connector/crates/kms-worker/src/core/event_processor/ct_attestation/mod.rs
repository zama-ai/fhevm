mod consensus;
pub mod error;
pub mod fetch;
pub mod registry;
pub mod verifier;

use alloy::primitives::U256;
pub use error::AttestationError;
pub use registry::CoprocessorRegistry;
pub use verifier::AttestationVerifier;

/// Hardcoded to `0` per RFC 023; bound by the attestation signature.
const COPROCESSOR_CONTEXT_ID: U256 = U256::ZERO;
