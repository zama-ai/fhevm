//! # FHEVM Client Core
//!
//! Platform-agnostic FHE client operations extracted from the gateway-sdk.
//!
//! This crate provides the core cryptographic operations needed by all FHEVM clients:
//! encryption with ZK proofs, EIP-712 signatures, calldata generation, and decryption
//! request/response processing.
//!
//! ## Design Principles
//!
//! - **No platform-specific APIs**: no filesystem, no tokio, no gRPC, no system clock
//! - **Keys as `Arc` references**: all key material accepted via pre-loaded `Arc` references
//!   to constructors like [`InputBuilderFactory::new`](encryption::input::InputBuilderFactory::new)
//! - **Single source of truth**: all FHE operations implemented once in Rust
//! - **Same API surface everywhere**: identical semantics on WASM, iOS, Android, and native
//!
//! ## Build Requirements
//!
//! This crate depends on the `tfhe` crate for FHE cryptographic operations,
//! which requires a C toolchain at build time.

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Contract addresses on the Gateway chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayContracts {
    pub input_verification: Option<Address>,
    pub decryption: Option<Address>,
}

/// Contract addresses on the Host chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostContracts {
    pub acl: Option<Address>,
}

/// Configuration for the FHEVM client core.
///
/// Does not reference the filesystem — key material is provided directly
/// to constructors like [`InputBuilderFactory::new`](encryption::input::InputBuilderFactory::new).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCoreConfig {
    /// Gateway chain ID
    pub gateway_chain_id: u64,
    /// Host chain ID
    pub host_chain_id: u64,
    /// Contract addresses on Gateway chain
    pub gateway_contracts: GatewayContracts,
    /// Contract addresses on Host chain
    pub host_contracts: HostContracts,
}

/// Platform-agnostic error type — no I/O or file-related variants.
///
/// Each variant carries a string error code accessible via [`error_code`](ClientCoreError::error_code)
/// for FFI consumers who cannot pattern-match on Rust enum variants.
#[derive(Error, Debug)]
pub enum ClientCoreError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("Key error: {0}")]
    KeyError(String),

    #[error("Hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("Alloy parse error: {0}")]
    AlloyParseError(#[from] alloy::primitives::ruint::ParseError),
}

impl ClientCoreError {
    /// Returns a stable error code string suitable for FFI consumers
    /// (WASM, Swift, Kotlin) who cannot pattern-match on Rust enums.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::EncryptionError(_) => "ENCRYPTION_ERROR",
            Self::DecryptionError(_) => "DECRYPTION_ERROR",
            Self::InvalidParams(_) => "INVALID_PARAMS",
            Self::SignatureError(_) => "SIGNATURE_ERROR",
            Self::KeyError(_) => "KEY_ERROR",
            Self::HexError(_) => "HEX_ERROR",
            Self::AlloyParseError(_) => "ALLOY_PARSE_ERROR",
        }
    }
}

/// Result type for client core operations.
pub type Result<T> = std::result::Result<T, ClientCoreError>;

// Modules
pub mod blockchain;
pub mod decryption;
pub mod encryption;
pub mod signature;
pub mod utils;

pub use encryption::input::{EncryptedInput, EncryptedInputBuilder, InputBuilderFactory};

