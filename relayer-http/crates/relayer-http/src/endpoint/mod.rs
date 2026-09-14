//! The relayer's HTTP layer: two sync decryption routes, the Kubernetes probes, one error model. Start with `docs.md`.

pub mod error;
pub mod flows;
pub mod validate;

pub use error::ApiError;
