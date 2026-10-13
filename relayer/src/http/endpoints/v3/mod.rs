//! v3 endpoints: unified EIP-712 user-decryption.
//!
//! The v3 surface is intentionally minimal — only `/v3/user-decrypt` POST +
//! GET. The HTTP request body is a typed-attestation envelope so future
//! signature schemes (e.g. Solana ed25519) can be added under a new
//! `attestationType` without bumping to v4. v2 endpoints remain untouched
//! throughout the deprecation window.

pub mod handlers;
pub mod types;

pub use handlers::{UserDecryptHandler, UserDecryptResponse};
pub use types::{AttestedUserDecryptRequestJson, Eip712UnifiedUserDecryptPayloadJson};
