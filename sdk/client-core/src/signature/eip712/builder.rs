//! Builder pattern implementation for EIP-712 signatures.
//!
//! All timestamps must be provided explicitly — the builder never reads the
//! system clock, ensuring platform-agnostic behavior on WASM, iOS, and Android.

use super::types::{Eip712Config, Eip712Result};
use super::verification;
use crate::signature::derive_address_from_private_key;
use crate::{ClientCoreError, Result};
use alloy::primitives::{Address, Bytes};
use alloy::sol_types::SolStruct;
use tracing::{debug, info};

/// Builder for EIP-712 signature generation with a fluent API.
///
/// # Required fields
/// - **public_key**: set via [`with_public_key`](Self::with_public_key)
/// - **at least one contract**: set via [`with_contract`](Self::with_contract)
/// - **validity period**: set via [`with_validity_period`](Self::with_validity_period),
///   or separately via [`with_start_timestamp`](Self::with_start_timestamp) and
///   [`with_duration_days`](Self::with_duration_days)
///
/// # Defaults
/// - `extra_data`: `vec![0]` (single zero byte, matching the on-chain protocol)
/// - `verify_signature`: `false`
///
/// # Example
/// ```ignore
/// let result = Eip712SignatureBuilder::new(config)
///     .with_public_key("0x2000...")
///     .with_contract("0x742d...")?
///     .with_validity_period(1748252823, 10)
///     .with_private_key("0x7136...")
///     .build()?;
/// ```
#[derive(Clone)]
pub struct Eip712SignatureBuilder {
    public_key: Option<String>,
    contract_addresses: Vec<Address>,
    start_timestamp: Option<u64>,
    duration_days: Option<u64>,
    private_key: Option<String>,
    verify_signature: bool,
    delegated_account: Option<Address>,
    /// Extra data for the EIP-712 message. Defaults to `vec![0]` (single zero byte)
    /// matching the on-chain protocol. Use `with_extra_data(vec![])` for empty.
    extra_data: Vec<u8>,
    config: Eip712Config,
}

impl std::fmt::Debug for Eip712SignatureBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Eip712SignatureBuilder")
            .field("public_key", &self.public_key.is_some())
            .field("contract_addresses", &self.contract_addresses)
            .field("start_timestamp", &self.start_timestamp)
            .field("duration_days", &self.duration_days)
            .field("private_key", &self.private_key.as_ref().map(|_| "[REDACTED]"))
            .field("verify_signature", &self.verify_signature)
            .field("delegated_account", &self.delegated_account)
            .field("extra_data", &self.extra_data)
            .field("config", &self.config)
            .finish()
    }
}

/// Trait for ergonomic address input — accepts `&str`, `String`, `Address`, or `&Address`.
pub trait IntoEthereumAddress {
    /// Convert to a validated Ethereum address.
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
    /// Create a new builder with EIP-712 domain configuration.
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

    /// Set the user's public key for decryption (hex-encoded).
    pub fn with_public_key(mut self, key: &str) -> Self {
        self.public_key = Some(key.to_string());
        self
    }

    /// Add a single contract address.
    pub fn with_contract<T: IntoEthereumAddress>(mut self, address: T) -> Result<Self> {
        let addr = address.into_address()?;
        self.contract_addresses.push(addr);
        Ok(self)
    }

    /// Add multiple contract addresses.
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

    /// Set contract addresses from a vector. Validates all addresses are non-zero.
    pub fn with_contract_addresses_vec(mut self, addresses: Vec<Address>) -> Result<Self> {
        if addresses.is_empty() {
            return Err(ClientCoreError::InvalidParams(
                "At least one contract address is required".to_string(),
            ));
        }
        for addr in &addresses {
            crate::utils::validate_address(addr)?;
        }
        self.contract_addresses = addresses;
        Ok(self)
    }

    /// Clear all contract addresses.
    pub fn with_contracts_cleared(mut self) -> Self {
        self.contract_addresses.clear();
        self
    }

    /// Set the validity period (start timestamp and duration in days).
    pub fn with_validity_period(mut self, start_timestamp: u64, duration_days: u64) -> Self {
        self.start_timestamp = Some(start_timestamp);
        self.duration_days = Some(duration_days);
        self
    }

    /// Set the start timestamp. Must also call `with_duration_days` or `with_validity_period`.
    pub fn with_start_timestamp(mut self, timestamp: u64) -> Self {
        self.start_timestamp = Some(timestamp);
        self
    }

    /// Set the duration in days. Must also call `with_start_timestamp` or `with_validity_period`.
    pub fn with_duration_days(mut self, days: u64) -> Self {
        self.duration_days = Some(days);
        self
    }

    /// Sign with a private key (hex-encoded).
    pub fn with_private_key(mut self, private_key: &str) -> Self {
        self.private_key = Some(private_key.to_string());
        self
    }

    /// Set extra data for the EIP-712 message.
    ///
    /// Default is `vec![0]` (single zero byte matching the on-chain protocol).
    /// Pass `vec![]` for empty extra data.
    pub fn with_extra_data(mut self, extra_data: Vec<u8>) -> Self {
        self.extra_data = extra_data;
        self
    }

    /// Enable or disable signature verification.
    pub fn with_verification(mut self, should_verify: bool) -> Self {
        self.verify_signature = should_verify;
        self
    }

    /// Set delegated account for delegated decryption.
    pub fn with_delegated_account(mut self, account: &str) -> Result<Self> {
        let addr = crate::utils::validate_address_from_str(account)?;
        self.delegated_account = Some(addr);
        Ok(self)
    }

    /// Build and generate the EIP-712 result.
    pub fn build(self) -> Result<Eip712Result> {
        debug!("Building EIP-712 signature");
        self.validate()?;
        self.generate_signature()
    }

    fn generate_signature(self) -> Result<Eip712Result> {
        let public_key = self.public_key.as_ref().ok_or_else(|| {
            ClientCoreError::InvalidParams("Public key is required".to_string())
        })?;

        let start_timestamp = self.start_timestamp.ok_or_else(|| {
            ClientCoreError::InvalidParams(
                "Start timestamp is required. Call .with_validity_period() first.".to_string(),
            )
        })?;

        let duration_days = self.duration_days.ok_or_else(|| {
            ClientCoreError::InvalidParams(
                "Duration is required. Call .with_validity_period() first.".to_string(),
            )
        })?;

        info!(
            "Generating EIP-712 hash with {} contracts, {} days validity",
            self.contract_addresses.len(),
            duration_days
        );

        let public_key_bytes = crate::utils::parse_hex_string(public_key, "public key")?;
        let hash =
            self.generate_hash_internal(&public_key_bytes, start_timestamp, duration_days)?;

        if let Some(private_key) = self.private_key {
            crate::signature::validate_private_key_format(&private_key)?;
            let signature = crate::signature::sign_eip712_hash(hash, &private_key)?;
            let signer = verification::recover_signer(&signature, hash)?;

            if self.verify_signature {
                let expected_signer = derive_address_from_private_key(&private_key)?;
                if signer != expected_signer {
                    return Err(ClientCoreError::SignatureError(format!(
                        "Signature verification failed: recovered signer {} does not match expected {}",
                        signer, expected_signer
                    )));
                }
                Ok(Eip712Result::Verified {
                    hash,
                    signature,
                    signer,
                })
            } else {
                Ok(Eip712Result::Signed {
                    hash,
                    signature,
                    signer,
                })
            }
        } else {
            Ok(Eip712Result::HashOnly { hash })
        }
    }

    fn validate(&self) -> Result<()> {
        if self.public_key.is_none() {
            return Err(ClientCoreError::InvalidParams(
                "Public key is required. Call .with_public_key() first.".to_string(),
            ));
        }

        if self.contract_addresses.is_empty() {
            return Err(ClientCoreError::InvalidParams(
                "At least one contract address is required. Call .with_contract() first."
                    .to_string(),
            ));
        }

        if self.start_timestamp.is_none() {
            return Err(ClientCoreError::InvalidParams(
                "Start timestamp is required. Call .with_validity_period() first.".to_string(),
            ));
        }

        match self.duration_days {
            None => {
                return Err(ClientCoreError::InvalidParams(
                    "Duration is required. Call .with_validity_period() first.".to_string(),
                ));
            }
            Some(d) if d == 0 || d > 365 => {
                return Err(ClientCoreError::InvalidParams(
                    "Duration must be between 1 and 365 days".to_string(),
                ));
            }
            _ => {}
        }

        Ok(())
    }

    fn generate_hash_internal(
        &self,
        public_key_bytes: &[u8],
        start_timestamp: u64,
        duration_days: u64,
    ) -> Result<alloy::primitives::B256> {
        let domain = alloy::sol_types::eip712_domain! {
            name: "Decryption",
            version: "1",
            chain_id: self.config.contracts_chain_id,
            verifying_contract: self.config.verifying_contract,
        };

        let hash = if let Some(delegated_account) = self.delegated_account {
            let message = super::types::DelegatedUserDecryptRequestVerification {
                publicKey: Bytes::from(public_key_bytes.to_vec()),
                contractAddresses: self.contract_addresses.clone(),
                delegatorAddress: delegated_account,
                startTimestamp: alloy::primitives::U256::from(start_timestamp),
                durationDays: alloy::primitives::U256::from(duration_days),
                extraData: self.extra_data.clone().into(),
            };
            message.eip712_signing_hash(&domain)
        } else {
            let message = super::types::UserDecryptRequestVerification {
                publicKey: Bytes::from(public_key_bytes.to_vec()),
                contractAddresses: self.contract_addresses.clone(),
                startTimestamp: alloy::primitives::U256::from(start_timestamp),
                durationDays: alloy::primitives::U256::from(duration_days),
                extraData: self.extra_data.clone().into(),
            };
            message.eip712_signing_hash(&domain)
        };

        Ok(hash)
    }

    /// Convenience: generate just the hash (no signing).
    pub fn generate_hash(self) -> Result<alloy::primitives::B256> {
        let result = self.build()?;
        Ok(*result.hash())
    }

    /// Get a summary of the current builder state.
    pub fn summary(&self) -> String {
        format!(
            "EIP-712 Builder State:\n\
             - Public Key: {}\n\
             - Contracts: {}\n\
             - Start Time: {}\n\
             - Duration: {}\n\
             - Will Sign: {}\n\
             - Will Verify: {}\n\
             - Delegated: {}",
            self.public_key.is_some(),
            self.contract_addresses.len(),
            self.start_timestamp
                .map(|t| t.to_string())
                .unwrap_or_else(|| "Not set".to_string()),
            self.duration_days
                .map(|d| format!("{d} days"))
                .unwrap_or_else(|| "Not set".to_string()),
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

        assert!(result.is_signed());
        assert!(result.signer().is_some());
        assert!(!result.is_verified()); // verification not requested

        let signature = result.require_signature().unwrap();
        assert_eq!(signature.len(), 65);
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
            .build()
            .unwrap();

        assert!(result.is_signed());
        assert!(result.is_verified());
    }

    #[test]
    fn test_validation_errors() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config);

        // Missing public key
        let result = builder
            .clone()
            .with_validity_period(1748252823, 10)
            .build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Public key is required"));

        // Missing contracts
        let result = builder
            .clone()
            .with_public_key("test_key")
            .with_validity_period(1748252823, 10)
            .build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("contract address is required"));

        // Missing validity period
        let result = builder
            .clone()
            .with_public_key(
                "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
            )
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Start timestamp is required"));
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
            .with_validity_period(1748252823, 0)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duration must be between 1 and 365 days"));
    }

    #[test]
    fn test_separate_timestamp_and_duration() {
        let config = create_test_config();
        let builder = Eip712SignatureBuilder::new(config)
            .with_public_key(
                "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
            )
            .with_contract("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")
            .unwrap()
            .with_start_timestamp(1748252823)
            .with_duration_days(30);

        let result = builder.build();
        assert!(result.is_ok());
    }
}
