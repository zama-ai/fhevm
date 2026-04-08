//! User decryption module for FHEVM client core.
//!
//! Provides request building and type definitions. The KMS-dependent
//! response processing stays in gateway-sdk.

mod request;
mod types;

pub use self::request::UserDecryptRequestBuilder;
pub use self::types::{DecryptedValue, UserDecryptRequest};
