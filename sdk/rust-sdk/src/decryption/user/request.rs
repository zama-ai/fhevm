use super::types::UserDecryptRequest;
use crate::blockchain::calldata::user_decryption_req;
use crate::utils::{parse_hex_string, validate_address_from_str};
use crate::{FhevmError, Result};
use alloy::primitives::{Address, Bytes, U256};
use fhevm_gateway_rust_bindings::decryption::Decryption::CtHandleContractPair;
use fhevm_gateway_rust_bindings::decryption::IDecryption::RequestValidity;
use tracing::debug;

/// Builder pattern for creating UserDecryptRequest instances
///
/// This builder provides a fluent API for constructing user decrypt requests
/// with comprehensive validation and clear error messages.
///
/// # Example
///
/// ```no_run
/// # use gateway_sdk::decryption::user::UserDecryptRequestBuilder;
/// # use gateway_sdk::FhevmError;
/// # use alloy::primitives::Address;
/// # use std::str::FromStr;
/// #
/// # fn example() -> Result<(), FhevmError> {
/// # let handles = vec![vec![1u8; 32]];
/// # let contracts = vec![Address::from_str("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1").unwrap()];
/// # let timestamp = 1640995200u64;
/// #
/// let calldata = UserDecryptRequestBuilder::new()
///     .with_handles_from_bytes(&handles, &contracts)?
///     .with_user_address_from_str("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B3")?
///     .with_signature_from_hex("0x1234567890abc5678...")?
///     .with_public_key_from_hex("0x200000000000...bc6f331")?
///     .with_validity(timestamp, 30)?
///     .build_and_generate_calldata()?;
///
/// println!("Generated calldata: {} bytes", calldata.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
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

    /// Add handles and their associated contract addresses
    ///
    /// # Arguments
    /// * `handles` - Array of 32-byte ciphertext handles
    /// * `contract_addresses` - Contracts that can access the decryption
    ///
    /// # Errors
    /// * If no handles provided
    /// * If no contract addresses provided
    /// * If more than 10 contracts (protocol limit)
    /// * If handle is not 32 bytes
    pub fn with_handles_from_bytes(
        mut self,
        handles: &[Vec<u8>],
        contract_addresses: &[Address],
    ) -> Result<Self> {
        // Validate inputs
        validate_handles_input(handles, contract_addresses)?;

        // Process handles with validation
        for (i, handle) in handles.iter().enumerate() {
            validate_handle_size(handle, i)?;

            let handle_u256 = U256::from_be_slice(handle);
            let contract_addr = contract_addresses[i % contract_addresses.len()];

            self.ct_handle_contract_pairs.push(CtHandleContractPair {
                ctHandle: handle_u256.into(),
                contractAddress: contract_addr,
            });
        }

        // Store unique contract addresses
        for &addr in contract_addresses {
            if !self.contract_addresses.contains(&addr) {
                self.contract_addresses.push(addr);
            }
        }

        Ok(self)
    }

    /// Set the user address who can decrypt
    pub fn with_user_address_from_str(mut self, address: &str) -> Result<Self> {
        if address.trim().is_empty() {
            return Err(FhevmError::InvalidParams(
                "User address cannot be empty".to_string(),
            ));
        }

        let addr = validate_address_from_str(address)?;

        if addr.is_zero() {
            return Err(FhevmError::InvalidParams(
                "Zero address is not allowed for user".to_string(),
            ));
        }

        self.user_address = Some(addr);
        Ok(self)
    }

    /// Set the EIP-712 signature
    pub fn with_signature_from_hex(mut self, signature: &str) -> Result<Self> {
        let signature_bytes = parse_hex_string(signature, "signature")?;

        // Validate signature length (should be 65 bytes for ECDSA)
        if signature_bytes.len() != 65 {
            return Err(FhevmError::InvalidParams(format!(
                "Invalid signature length: expected 65 bytes, got {}",
                signature_bytes.len()
            )));
        }

        self.signature = Some(signature_bytes);
        Ok(self)
    }

    /// Set the user's public key for decryption
    pub fn with_public_key_from_hex(mut self, public_key: &str) -> Result<Self> {
        let public_key_bytes = parse_hex_string(public_key, "public key")?;

        if public_key_bytes.is_empty() {
            return Err(FhevmError::InvalidParams(
                "Public key cannot be empty".to_string(),
            ));
        }

        self.public_key = Some(public_key_bytes);
        Ok(self)
    }

    /// Set the validity period for the decryption permission
    pub fn with_validity(mut self, start_timestamp: u64, duration_days: u64) -> Result<Self> {
        validate_validity_params(start_timestamp, duration_days)?;

        self.start_timestamp = Some(start_timestamp);
        self.duration_days = Some(duration_days);
        Ok(self)
    }

    /// Set the chain ID where contracts are deployed
    pub fn with_contracts_chain_id(mut self, chain_id: u64) -> Self {
        self.contracts_chain_id = Some(chain_id);
        self
    }

    /// Build the request and generate calldata
    ///
    /// This is the final step that creates transaction calldata ready to send.
    pub fn build_and_generate_calldata(self) -> Result<Vec<u8>> {
        let request = self.build()?;
        let calldata = user_decryption_req(request)?;
        Ok(calldata.to_vec())
    }

    /// Build just the request object
    pub fn build(self) -> Result<UserDecryptRequest> {
        self.validate()?;
        self.construct()
    }

    fn validate(&self) -> Result<()> {
        if self.ct_handle_contract_pairs.is_empty() {
            return Err(FhevmError::InvalidParams(
                "Missing handles: Call `with_handles_from_bytes()` first.".to_string(),
            ));
        }

        if self.user_address.is_none() {
            return Err(FhevmError::InvalidParams(
                "Missing user address: Call `with_user_address_from_str()` first.".to_string(),
            ));
        }

        if self.signature.is_none() {
            return Err(FhevmError::InvalidParams(
                "Missing signature: Call `with_signature_from_hex()` first.".to_string(),
            ));
        }

        if self.public_key.is_none() {
            return Err(FhevmError::InvalidParams(
                "Missing public key: Call `with_public_key_from_hex()` first.".to_string(),
            ));
        }

        if self.start_timestamp.is_none() || self.duration_days.is_none() {
            return Err(FhevmError::InvalidParams(
                "Missing validity: Call `with_validity()` first.".to_string(),
            ));
        }

        // Validate handle sizes
        for (i, pair) in self.ct_handle_contract_pairs.iter().enumerate() {
            let handle_bytes = pair.ctHandle.as_slice();
            if handle_bytes.len() != 32 {
                return Err(FhevmError::InvalidParams(format!(
                    "Handle {} must be exactly 32 bytes, got {}",
                    i,
                    handle_bytes.len()
                )));
            }
        }

        // Validate duration
        let duration_days = self.duration_days.unwrap();
        if duration_days == 0 || duration_days > 365 {
            return Err(FhevmError::InvalidParams(
                "Duration must be between 1 and 365 days".to_string(),
            ));
        }

        // Validate signature length
        if let Some(ref sig) = self.signature
            && sig.len() != 65
        {
            return Err(FhevmError::InvalidParams(format!(
                "Invalid signature length: expected 65 bytes, got {}",
                sig.len()
            )));
        }

        Ok(())
    }

    fn construct(self) -> Result<UserDecryptRequest> {
        let user_address = self.user_address.unwrap();
        let signature = self.signature.unwrap();
        let public_key = self.public_key.unwrap();
        let start_timestamp = self.start_timestamp.unwrap();
        let duration_days = self.duration_days.unwrap();
        let contracts_chain_id = self.contracts_chain_id.unwrap_or(1);

        debug!("✅ UserDecryptRequest built successfully");
        debug!("   📊 Handles: {}", self.ct_handle_contract_pairs.len());
        debug!("   👤 User: {}", user_address);
        debug!("   🏢 Contracts: {}", self.contract_addresses.len());
        debug!("   ⏰ Duration: {} days", duration_days);

        Ok(UserDecryptRequest {
            ct_handle_contract_pairs: self.ct_handle_contract_pairs,
            request_validity: RequestValidity {
                startTimestamp: U256::from(start_timestamp),
                durationDays: U256::from(duration_days),
            },
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

// Validation helpers
const MAX_CONTRACT_ADDRESSES: usize = 10;
const HANDLE_SIZE: usize = 32;
const MAX_DURATION_DAYS: u64 = 365;

fn validate_handles_input(handles: &[Vec<u8>], contract_addresses: &[Address]) -> Result<()> {
    if handles.is_empty() {
        return Err(FhevmError::InvalidParams(
            "At least one ciphertext handle is required".to_string(),
        ));
    }

    if contract_addresses.is_empty() {
        return Err(FhevmError::InvalidParams(
            "At least one contract address is required".to_string(),
        ));
    }

    if contract_addresses.len() > MAX_CONTRACT_ADDRESSES {
        return Err(FhevmError::InvalidParams(format!(
            "Maximum {MAX_CONTRACT_ADDRESSES} contract addresses allowed"
        )));
    }

    Ok(())
}

fn validate_handle_size(handle: &[u8], index: usize) -> Result<()> {
    if handle.len() != HANDLE_SIZE {
        return Err(FhevmError::InvalidParams(format!(
            "Handle {} must be exactly {} bytes, got {}",
            index,
            HANDLE_SIZE,
            handle.len()
        )));
    }
    Ok(())
}

fn validate_validity_params(_start_timestamp: u64, duration_days: u64) -> Result<()> {
    if duration_days == 0 {
        return Err(FhevmError::InvalidParams(
            "Duration days cannot be zero".to_string(),
        ));
    }

    if duration_days > MAX_DURATION_DAYS {
        return Err(FhevmError::InvalidParams(format!(
            "Duration days cannot exceed {MAX_DURATION_DAYS}"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_builder_with_valid_data() {
        let handles = vec![vec![1u8; 32], vec![2u8; 32]];
        let contracts = vec![
            Address::from_str("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1").unwrap(),
            Address::from_str("0x853d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B2").unwrap(),
        ];

        let result = UserDecryptRequestBuilder::new()
            .with_handles_from_bytes(&handles, &contracts)
            .unwrap()
            .with_user_address_from_str("0x963d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B3")
            .unwrap()
            .with_signature_from_hex(&("0x".to_owned() + &"12".repeat(65)))
            .unwrap()
            .with_public_key_from_hex("0x1234567890abcdef")
            .unwrap()
            .with_validity(1640995200, 30)
            .unwrap()
            .build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.ct_handle_contract_pairs.len(), 2);
        assert_eq!(request.contract_addresses.len(), 2);
    }

    #[test]
    fn test_builder_validates_empty_handles() {
        let result = UserDecryptRequestBuilder::new().build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing handles"));
    }

    #[test]
    fn test_validates_handle_size() {
        let handles = vec![vec![1u8; 31]]; // Wrong size
        let contracts =
            vec![Address::from_str("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1").unwrap()];

        let result = UserDecryptRequestBuilder::new().with_handles_from_bytes(&handles, &contracts);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("32 bytes"));
    }
}
