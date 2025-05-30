//! Decryption module for FHEVM SDK

use crate::{FhevmError, Result, types::DecryptedValue};
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::sol_types::SolCall;
use alloy::{primitives::U256, rpc::types::Log};

use crate::blockchain::bindings::Decryption::{
    self, PublicDecryptionRequest, PublicDecryptionResponse, UserDecryptionRequest,
    publicDecryptionResponseCall, userDecryptionRequestCall,
};

use crate::blockchain::bindings::Decryption::CtHandleContractPair;
use crate::blockchain::bindings::IDecryption::RequestValidity;
use crate::blockchain::bindings::InputVerification;

use log::info;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDecryptRequest {
    pub ct_handle_contract_pairs: Vec<CtHandleContractPair>,
    pub request_validity: RequestValidity,
    pub contracts_chain_id: u64,
    pub contract_addresses: Vec<Address>,
    pub user_address: Address,
    pub signature: Bytes,
    pub public_key: Bytes,
}

/// Builder pattern for creating UserDecryptRequest instances
///
/// This provides a convenient way to build UserDecryptRequest objects with validation
pub struct UserDecryptRequestBuilder {
    ct_handle_contract_pairs: Vec<CtHandleContractPair>,
    contract_addresses: Vec<Address>,
    user_address: Option<Address>,
    signature: Option<Bytes>,
    public_key: Option<Bytes>,
    start_timestamp: Option<u64>,
    duration_days: Option<u64>,
    contracts_chain_id: Option<u64>,
}

impl UserDecryptRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            ct_handle_contract_pairs: Vec::new(),
            contract_addresses: Vec::new(),
            user_address: None,
            signature: None,
            public_key: None,
            start_timestamp: None,
            duration_days: None,
            contracts_chain_id: None,
        }
    }

    /// Add a ciphertext handle with its contract address
    pub fn add_handle_contract_pair(mut self, ct_handle: U256, contract_address: Address) -> Self {
        self.ct_handle_contract_pairs.push(CtHandleContractPair {
            ctHandle: ct_handle.into(),
            contractAddress: contract_address,
        });
        self
    }

    /// Set the user address
    pub fn user_address(mut self, address: Address) -> Self {
        self.user_address = Some(address);
        self
    }

    /// Add a contract address
    pub fn add_contract_address(mut self, address: Address) -> Self {
        self.contract_addresses.push(address);
        self
    }

    /// Set the signature
    pub fn signature(mut self, signature: Bytes) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Set the public key
    pub fn public_key(mut self, public_key: Bytes) -> Self {
        self.public_key = Some(public_key);
        self
    }

    /// Set the start timestamp
    pub fn start_timestamp(mut self, timestamp: u64) -> Self {
        self.start_timestamp = Some(timestamp);
        self
    }

    /// Set the duration in days
    pub fn duration_days(mut self, days: u64) -> Self {
        self.duration_days = Some(days);
        self
    }

    /// Set the contracts chain ID
    pub fn contracts_chain_id(mut self, chain_id: u64) -> Self {
        self.contracts_chain_id = Some(chain_id);
        self
    }

    /// Build the UserDecryptRequest
    pub fn build(self) -> Result<UserDecryptRequest> {
        let user_address = self
            .user_address
            .ok_or_else(|| FhevmError::InvalidParams("User address is required".to_string()))?;

        let signature = self
            .signature
            .ok_or_else(|| FhevmError::InvalidParams("Signature is required".to_string()))?;

        let public_key = self
            .public_key
            .ok_or_else(|| FhevmError::InvalidParams("Public key is required".to_string()))?;

        let start_timestamp = self.start_timestamp.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let duration_days = self.duration_days.unwrap_or(7); // Default to 7 days
        let contracts_chain_id = self.contracts_chain_id.unwrap_or(1); // Default to mainnet

        let request_validity = RequestValidity {
            startTimestamp: U256::from(start_timestamp),
            durationDays: U256::from(duration_days),
        };

        Ok(UserDecryptRequest {
            ct_handle_contract_pairs: self.ct_handle_contract_pairs,
            request_validity,
            contracts_chain_id,
            contract_addresses: self.contract_addresses,
            user_address,
            signature,
            public_key,
        })
    }
}

impl Default for UserDecryptRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn user_decryption_req_calldata(user_decrypt_request: UserDecryptRequest) -> Result<Bytes> {
    info!("Generating user decryption request calldata");

    // Create the userDecryptionRequest call
    let call = userDecryptionRequestCall::new((
        user_decrypt_request.ct_handle_contract_pairs,
        user_decrypt_request.request_validity,
        U256::from(user_decrypt_request.contracts_chain_id),
        user_decrypt_request.contract_addresses,
        user_decrypt_request.user_address,
        user_decrypt_request.public_key,
        user_decrypt_request.signature,
    ));

    // Encode the call to get the calldata
    let calldata = userDecryptionRequestCall::abi_encode(&call);

    info!(
        "UserDecryptionRequest calldata: 0x{}",
        hex::encode(&calldata)
    );

    Ok(Bytes::from(calldata))
}

pub fn user_decrypt_request() -> Result<()> {
    // Placeholder implementation
    Ok(())
}

/// Reconstruct a plaintext from encrypted shares (for user decrypt)
pub fn user_decrypt_reconstruction(
    encrypted_shares: &[Vec<u8>],
    private_key: &[u8],
) -> Result<DecryptedValue> {
    // Placeholder implementation
    if encrypted_shares.is_empty() {
        return Err(FhevmError::DecryptionError(
            "No encrypted shares provided".to_string(),
        ));
    }

    if private_key.is_empty() {
        return Err(FhevmError::DecryptionError(
            "Invalid private key".to_string(),
        ));
    }

    // Return mock decrypted value
    Ok(DecryptedValue(vec![42]))
}

/// Public decrypt operation (used by the network)
pub fn public_decrypt(ciphertext: &[u8], _public_key: &[u8]) -> Result<DecryptedValue> {
    // Placeholder implementation
    if ciphertext.is_empty() {
        return Err(FhevmError::DecryptionError(
            "No ciphertext provided".to_string(),
        ));
    }

    // Return mock decrypted value
    Ok(DecryptedValue(vec![42]))
}
