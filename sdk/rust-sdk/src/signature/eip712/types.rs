//! Type definitions for EIP-712 operations

use alloy::primitives::{Address, B256, Bytes};
use serde::{Deserialize, Serialize};

/// Result of EIP-712 generation and optional signing
#[derive(Debug, Clone)]
pub struct Eip712Result {
    /// The EIP-712 hash
    pub hash: B256,
    /// Optional signature (if wallet private key was provided)
    pub signature: Option<Bytes>,
    /// Optional signer address (if signature was created)
    pub signer: Option<Address>,
    /// Whether signature was verified (if verification was requested)
    pub verified: Option<bool>,
}

impl Eip712Result {
    /// Check if a signature was generated
    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }

    /// Check if the signature was verified successfully
    pub fn is_verified(&self) -> bool {
        self.verified == Some(true)
    }

    /// Check if verification was attempted
    pub fn was_verification_attempted(&self) -> bool {
        self.verified.is_some()
    }

    /// Get verification status as a descriptive string
    pub fn verification_status(&self) -> &'static str {
        match self.verified {
            None => "not attempted",
            Some(true) => "verified",
            Some(false) => "failed",
        }
    }

    /// Get the signature or return an error if not signed
    pub fn require_signature(&self) -> crate::Result<&Bytes> {
        self.signature.as_ref().ok_or_else(|| {
            crate::FhevmError::SignatureError(
                "No signature available - wallet private key was not provided".to_string(),
            )
        })
    }

    /// Ensure the signature was verified, return error if not
    pub fn ensure_verified(&self) -> crate::Result<()> {
        match self.verified {
            Some(true) => Ok(()),
            Some(false) => Err(crate::FhevmError::SignatureError(
                "Signature verification failed".to_string(),
            )),
            None => Err(crate::FhevmError::SignatureError(
                "Signature was not verified".to_string(),
            )),
        }
    }
}

/// Configuration for EIP-712 domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eip712Config {
    /// Gateway chain ID for the domain
    pub gateway_chain_id: u64,
    /// Verifying contract address
    pub verifying_contract: Address,
    /// Chain ID where contracts are deployed
    pub contracts_chain_id: u64,
}

// Define the EIP-712 typed data structures
alloy::sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct UserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        uint256 contractsChainId;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct DelegatedUserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        address delegatorAddress;
        uint256 contractsChainId;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }
}
