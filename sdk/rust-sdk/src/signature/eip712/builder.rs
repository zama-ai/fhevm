//! Builder pattern implementation for EIP-712 signatures

use super::types::{Eip712Config, Eip712Result};
use super::verification;
use crate::signature::derive_address_from_private_key;
use crate::{FhevmError, Result};
use alloy::primitives::{Address, Bytes};
use alloy::sol_types::SolStruct;
use tracing::{debug, info};

/// Builder for EIP-712 signature generation with a fluent API
///
/// This builder provides a cleaner interface for generating EIP-712 signatures
/// for user decrypt operations, with better error messages and validation.
#[derive(Debug, Clone)]
pub struct Eip712SignatureBuilder {
    // Required fields
    public_key: Option<String>,
    contract_addresses: Vec<Address>,
    start_timestamp: Option<u64>,
    duration_days: Option<u64>,

    // Optional fields
    private_key: Option<String>,
    verify_signature: bool,
    delegated_account: Option<Address>,
    extra_data: Vec<u8>,

    // Configuration
    config: Eip712Config,
}

pub trait IntoEthereumAddress {
    fn into_address(self) -> Result<Address>;
}

impl IntoEthereumAddress for &str {
    fn into_address(self) -> Result<Address> {
        crate::utils::validate_address_from_str(self)
    }
}

impl IntoEthereumAddress for String {
    fn into_address(self) -> Result<Address> {
        crate::utils::validate_address_from_str(&self)
    }
}

impl IntoEthereumAddress for Address {
    fn into_address(self) -> Result<Address> {
        crate::utils::validate_address(&self)?;
        Ok(self)
    }
}

impl IntoEthereumAddress for &Address {
    fn into_address(self) -> Result<Address> {
        crate::utils::validate_address(self)?;
        Ok(*self)
    }
}

impl Eip712SignatureBuilder {
    /// Create a new builder with configuration
    pub fn new(config: Eip712Config) -> Self {
        Self {
            public_key: None,
            contract_addresses: Vec::new(),
            start_timestamp: None,
            duration_days: None,
            private_key: None,
            verify_signature: false,
            delegated_account: None,
            config,
            extra_data: vec![0],
        }
    }

    /// Set the user's public key for decryption
    pub fn with_public_key(mut self, key: &str) -> Self {
        self.public_key = Some(key.to_string());
        self
    }

    /// Add a single contract address (accepts &str, String, Address, or &Address)
    pub fn with_contract<T: IntoEthereumAddress>(mut self, address: T) -> Result<Self> {
        let addr = address.into_address()?;
        self.contract_addresses.push(addr);
        Ok(self)
    }

    /// Add multiple contract addresses from an iterator
    pub fn with_contracts<I, T>(mut self, addresses: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: IntoEthereumAddress,
    {
        for address in addresses {
            let addr = address.into_address()?;
            self.contract_addresses.push(addr);
        }
        Ok(self)
    }

    /// Set contract addresses from Address types
    pub fn with_contract_addresses_vec(mut self, addresses: Vec<Address>) -> Self {
        self.contract_addresses = addresses;
        self
    }

    /// Clear all contract addresses (useful for reusing builder)
    pub fn with_contracts_cleared(mut self) -> Self {
        self.contract_addresses.clear();
        self
    }

    /// Set the validity period
    pub fn with_validity_period(mut self, start_timestamp: u64, duration_days: u64) -> Self {
        self.start_timestamp = Some(start_timestamp);
        self.duration_days = Some(duration_days);
        self
    }

    /// Set just the start timestamp (duration defaults to 30 days)
    pub fn with_start_timestamp(mut self, timestamp: u64) -> Self {
        self.start_timestamp = Some(timestamp);
        if self.duration_days.is_none() {
            self.duration_days = Some(30); // Default 30 days
        }
        self
    }

    /// Set start timestamp to current time
    pub fn with_start_now(mut self) -> Self {
        self.start_timestamp = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        self
    }

    /// Set just the duration (start defaults to now)
    pub fn with_duration_days(mut self, days: u64) -> Self {
        self.duration_days = Some(days);
        if self.start_timestamp.is_none() {
            self.start_timestamp = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
        }
        self
    }

    /// Sign with a private key
    pub fn with_private_key(mut self, private_key: &str) -> Self {
        self.private_key = Some(private_key.to_string());
        self
    }

    /// Set the `extra_data` for decryption
    pub fn with_extra_data(mut self, extra_data: Vec<u8>) -> Self {
        self.extra_data = extra_data;
        self
    }

    /// Enable or disable signature verification
    pub fn with_verification(mut self, should_verify: bool) -> Self {
        self.verify_signature = should_verify;
        self
    }

    /// Set delegated account (for delegated decryption)
    pub fn with_delegated_account(mut self, account: &str) -> Result<Self> {
        let addr = crate::utils::validate_address_from_str(account)?;
        self.delegated_account = Some(addr);
        Ok(self)
    }

    /// Build and generate the EIP-712 signature
    pub fn build(self) -> Result<Eip712Result> {
        debug!("Building EIP-712 signature");

        self.validate()?;
        self.generate_signature()
    }

    fn generate_signature(self) -> Result<Eip712Result> {
        let public_key = self.public_key.as_ref().unwrap(); // Safe after validation
        let start_timestamp = self.start_timestamp.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        let duration_days = self.duration_days.unwrap_or(30);

        info!(
            "Generating EIP-712 hash with {} contracts, {} days validity",
            self.contract_addresses.len(),
            duration_days
        );

        // Generate hash
        let public_key_bytes = crate::utils::parse_hex_string(public_key, "public key")?;
        let hash =
            self.generate_hash_internal(&public_key_bytes, start_timestamp, duration_days)?;

        // Handle signing if private key provided
        if let Some(private_key) = self.private_key {
            crate::signature::validate_private_key_format(&private_key)?;
            let signature = crate::signature::sign_eip712_hash(hash, &private_key)?;
            let signer = verification::recover_signer(&signature, hash)?;

            let verified = if self.verify_signature {
                let expected_signer = derive_address_from_private_key(&private_key)?;
                Some(signer == expected_signer)
            } else {
                None
            };

            Ok(Eip712Result {
                hash,
                signature: Some(signature),
                signer: Some(signer),
                verified,
            })
        } else {
            Ok(Eip712Result {
                hash,
                signature: None,
                signer: None,
                verified: None,
            })
        }
    }

    fn validate(&self) -> Result<()> {
        if self.public_key.is_none() {
            return Err(FhevmError::InvalidParams(
                "Public key is required. Call .with_public_key() first.".to_string(),
            ));
        }

        if self.contract_addresses.is_empty() {
            return Err(FhevmError::InvalidParams(
                "At least one contract address is required. Call .with_contract() first."
                    .to_string(),
            ));
        }

        let duration_days = self.duration_days.unwrap_or(30);
        if duration_days == 0 || duration_days > 365 {
            return Err(FhevmError::InvalidParams(
                "Duration must be between 1 and 365 days".to_string(),
            ));
        }

        Ok(())
    }

    /// Generate the EIP-712 hash internally
    fn generate_hash_internal(
        &self,
        public_key_bytes: &[u8],
        start_timestamp: u64,
        duration_days: u64,
    ) -> Result<alloy::primitives::B256> {
        // Create domain
        let domain = alloy::sol_types::eip712_domain! {
            name: "Decryption",
            version: "1",
            chain_id: self.config.contracts_chain_id,
            verifying_contract: self.config.verifying_contract,
        };

        // Generate hash based on whether it's delegated or not
        let hash = if let Some(delegated_account) = self.delegated_account {
            let message = super::types::DelegatedUserDecryptRequestVerification {
                publicKey: Bytes::from(public_key_bytes.to_vec()),
                contractAddresses: self.contract_addresses.clone(),
                delegatorAddress: delegated_account,
                contractsChainId: alloy::primitives::U256::from(self.config.contracts_chain_id),
                startTimestamp: alloy::primitives::U256::from(start_timestamp),
                durationDays: alloy::primitives::U256::from(duration_days),
                extraData: self.extra_data.clone().into(),
            };
            message.eip712_signing_hash(&domain)
        } else {
            let message = super::types::UserDecryptRequestVerification {
                publicKey: Bytes::from(public_key_bytes.to_vec()),
                contractAddresses: self.contract_addresses.clone(),
                contractsChainId: alloy::primitives::U256::from(self.config.contracts_chain_id),
                startTimestamp: alloy::primitives::U256::from(start_timestamp),
                durationDays: alloy::primitives::U256::from(duration_days),
                extraData: self.extra_data.clone().into(),
            };
            message.eip712_signing_hash(&domain)
        };

        Ok(hash)
    }

    /// Convenience method to generate just the hash (no signing)
    pub fn generate_hash(self) -> Result<alloy::primitives::B256> {
        let result = self.build()?;
        Ok(result.hash)
    }

    /// Convenience method to generate and sign
    pub fn generate_and_sign(self) -> Result<Eip712Result> {
        self.build()
    }

    /// Convenience method to generate and sign WITHOUT verification (explicit)
    pub fn generate_and_sign_only(mut self) -> Result<Eip712Result> {
        self.verify_signature = false;
        self.build()
    }

    /// Get a summary of the current builder state
    pub fn summary(&self) -> String {
        format!(
            "EIP-712 Builder State:\n\
             - Public Key: {}\n\
             - Contracts: {}\n\
             - Start Time: {}\n\
             - Duration: {} days\n\
             - Will Sign: {}\n\
             - Will Verify: {}\n\
             - Delegated: {}",
            self.public_key.is_some(),
            self.contract_addresses.len(),
            self.start_timestamp
                .map(|t| t.to_string())
                .unwrap_or_else(|| "Now".to_string()),
            self.duration_days.unwrap_or(30),
            self.private_key.is_some(),
            self.verify_signature,
            self.delegated_account.is_some()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use std::str::FromStr;

    fn create_test_config() -> Eip712Config {
        Eip712Config {
            gateway_chain_id: 31337,
            verifying_contract: Address::from_str("0x1234567890123456789012345678901234567890")
                .unwrap(),
            contracts_chain_id: 31337,
        }
    }

    #[test]
    fn test_hash_only_generation() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config);

        let hash = builder
            .with_public_key(
                "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
            )
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .with_validity_period(1748252823, 10)
            .generate_hash()
            .unwrap();

        assert!(!hash.is_zero(), "Hash should not be zero");
    }

    #[test]
    fn test_signature_generation() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config);

        let result = builder
            .with_public_key(
                "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
            )
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .with_validity_period(1748252823, 10)
            .with_private_key("7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f")
            .build()
            .unwrap();

        assert!(result.is_signed(), "Result should be signed");
        assert!(result.signer.is_some(), "Signer should be present");
        assert!(
            !result.is_verified(),
            "Should not be verified (verification not requested)"
        );

        let signature = result.require_signature().unwrap();
        assert_eq!(signature.len(), 65, "Signature should be 65 bytes");
    }

    #[test]
    fn test_signature_verification() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config);

        let result = builder
            .with_public_key(
                "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
            )
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .with_validity_period(1748252823, 10)
            .with_private_key("7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f")
            .with_verification(true)
            .build() // ← Use build(), not generate_and_sign()
            .unwrap();

        assert!(result.is_signed(), "Result should be signed");
        assert!(result.is_verified(), "Result should be verified");
        assert!(result.verified == Some(true), "Verification should pass");
    }

    #[test]
    fn test_generate_and_sign_respects_verification() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config);

        // Test that generate_and_sign() now respects the verification setting
        let result = builder
            .with_public_key(
                "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
            )
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .with_validity_period(1748252823, 10)
            .with_private_key("7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f")
            .with_verification(true)
            .generate_and_sign() // ← Should now respect verification setting
            .unwrap();

        assert!(result.is_signed(), "Result should be signed");
        assert!(result.is_verified(), "Result should be verified after fix");
    }

    #[test]
    fn test_validation_errors() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config);

        // Test missing public key
        let result = builder.clone().build();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Public key is required")
        );

        // Test missing contracts
        let result = builder.clone().with_public_key("test_key").build();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("contract address is required")
        );
    }

    #[test]
    fn test_invalid_duration() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config);

        let result = builder
            .with_public_key(
                "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
            )
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .with_validity_period(1748252823, 0) // ← Invalid: 0 days
            .build();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Duration must be between 1 and 365 days")
        );
    }

    #[test]
    fn test_convenience_methods() {
        let config = create_test_config();

        // Test starts_now
        let builder = Eip712SignatureBuilder::new(config.clone())
            .with_public_key("test_key")
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .with_start_now()
            .with_duration_days(30);

        assert!(builder.start_timestamp.is_some());
        assert_eq!(builder.duration_days, Some(30));

        // Test with_contract_addresses
        let addresses =
            vec![Address::from_str("0x56a24bcaE11890353726596fD6f5cABb5a126Df9").unwrap()];
        let builder =
            Eip712SignatureBuilder::new(config).with_contract_addresses_vec(addresses.clone());

        assert_eq!(builder.contract_addresses, addresses);
    }
}
