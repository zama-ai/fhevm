//! v3 endpoints: unified user-decryption (EIP-712 and Solana sRFC-38).
//!
//! The v3 surface is intentionally minimal — only `/v3/user-decrypt` POST +
//! GET. The HTTP request body is a typed-attestation envelope discriminated
//! by `attestationType`, so a new signature scheme can be added without
//! bumping to v4. v2 endpoints remain untouched throughout the deprecation
//! window.

pub mod handlers;
pub mod types;

pub use handlers::{UserDecryptHandler, UserDecryptResponse};
pub use types::{
    AttestedUserDecryptRequestJson, Eip712UnifiedUserDecryptPayloadJson, UserDecryptV3RequestJson,
};
