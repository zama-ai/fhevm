//! Type definitions for EIP-712 operations.

use alloy::primitives::{Address, B256, Bytes};
use serde::{Deserialize, Serialize};

/// Result of EIP-712 generation and optional signing.
///
/// Modeled as an enum to make illegal states unrepresentable:
/// - `HashOnly`: only the hash was computed (no private key provided)
/// - `Signed`: hash was computed and signed (no verification requested)
/// - `Verified`: hash was computed, signed, and signature was verified to match the signing key
#[derive(Debug, Clone)]
pub enum Eip712Result {
    /// Only the EIP-712 hash was generated (no signing).
    HashOnly { hash: B256 },
    /// Hash generated and signed, but verification was not requested.
    Signed {
        hash: B256,
        signature: Bytes,
        signer: Address,
    },
    /// Hash generated, signed, and signature was verified successfully.
    Verified {
        hash: B256,
        signature: Bytes,
        signer: Address,
    },
}

impl Eip712Result {
    /// Get the EIP-712 hash.
    pub fn hash(&self) -> &B256 {
        match self {
            Self::HashOnly { hash }
            | Self::Signed { hash, .. }
            | Self::Verified { hash, .. } => hash,
        }
    }

    /// Check if a signature was generated.
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Signed { .. } | Self::Verified { .. })
    }

    /// Check if the signature was verified successfully.
    pub fn is_verified(&self) -> bool {
        matches!(self, Self::Verified { .. })
    }

    /// Get verification status as a descriptive string.
    pub fn verification_status(&self) -> &'static str {
        match self {
            Self::HashOnly { .. } | Self::Signed { .. } => "not attempted",
            Self::Verified { .. } => "verified",
        }
    }

    /// Get the signature, if present.
    pub fn signature(&self) -> Option<&Bytes> {
        match self {
            Self::HashOnly { .. } => None,
            Self::Signed { signature, .. } | Self::Verified { signature, .. } => Some(signature),
        }
    }

    /// Get the signer address, if present.
    pub fn signer(&self) -> Option<&Address> {
        match self {
            Self::HashOnly { .. } => None,
            Self::Signed { signer, .. } | Self::Verified { signer, .. } => Some(signer),
        }
    }

    /// Get the signature or return an error if not signed.
    pub fn require_signature(&self) -> crate::Result<&Bytes> {
        self.signature().ok_or_else(|| {
            crate::ClientCoreError::SignatureError(
                "No signature available - wallet private key was not provided".to_string(),
            )
        })
    }

    /// Ensure the signature was verified, return error if not.
    pub fn ensure_verified(&self) -> crate::Result<()> {
        match self {
            Self::Verified { .. } => Ok(()),
            _ => Err(crate::ClientCoreError::SignatureError(
                "Signature was not verified".to_string(),
            )),
        }
    }
}

/// Configuration for EIP-712 domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eip712Config {
    /// Gateway chain ID (stored for reference; used by gateway-sdk for response processing).
    pub gateway_chain_id: u64,
    /// Verifying contract address.
    pub verifying_contract: Address,
    /// Chain ID where contracts are deployed.
    pub contracts_chain_id: u64,
}

impl TryFrom<&crate::ClientCoreConfig> for Eip712Config {
    type Error = crate::ClientCoreError;

    /// Derive EIP-712 config from the core config.
    ///
    /// Maps `host_chain_id` to `contracts_chain_id` and uses the
    /// decryption contract as the verifying contract.
    ///
    /// Returns an error if the decryption contract address is not configured.
    fn try_from(config: &crate::ClientCoreConfig) -> crate::Result<Self> {
        let verifying_contract = config.gateway_contracts.decryption.ok_or_else(|| {
            crate::ClientCoreError::InvalidParams(
                "Decryption contract address is required for EIP-712 config".to_string(),
            )
        })?;

        Ok(Self {
            gateway_chain_id: config.gateway_chain_id,
            contracts_chain_id: config.host_chain_id,
            verifying_contract,
        })
    }
}

// EIP-712 typed data structures
alloy::sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct UserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct DelegatedUserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        address delegatorAddress;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }
}
