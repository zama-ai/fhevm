//! EIP-712 signature generation module.

mod builder;
mod types;
mod verification;

pub use self::builder::Eip712SignatureBuilder;
pub use self::types::{Eip712Config, Eip712Result};
pub use self::verification::{recover_signer, verify_signature};

pub use crate::signature::validate_private_key_format;
