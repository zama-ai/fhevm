mod consensus;
pub mod error;
pub mod fetcher;
pub mod registry;
pub mod verifier;

pub use error::AttestationError;
pub use registry::CoprocessorRegistry;
pub use verifier::AttestationVerifier;
