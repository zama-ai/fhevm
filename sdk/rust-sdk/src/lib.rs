//! # FHEVM SDK
//!
//! A Rust SDK for interacting with FHEVM networks.

use alloy::primitives::{Address, B256};
use serde::{Deserialize, Serialize};
use signature::Eip712Builder;
// use signature::generate_eip712_user_decrypt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;
/// Configuration for the FHEVM SDK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhevmConfig {
    /// Path to the directory containing key files
    pub keys_directory: PathBuf,
    /// Gateway chain ID
    pub gateway_chain_id: u64,
    /// Host chain ID
    pub host_chain_id: u64,
    /// Contract addresses on Gateway chain
    pub gateway_contracts: HashMap<String, String>,
    /// Contract addresses on Host chain
    pub host_contracts: HashMap<String, String>,
}

/// Errors that can occur in the SDK
#[derive(Error, Debug)]
pub enum FhevmError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("File error: {0}")]
    FileError(String),
}

/// Result type for FHEVM operations
pub type Result<T> = std::result::Result<T, FhevmError>;

/// Main SDK struct
pub struct FhevmSdk {
    config: FhevmConfig,
    public_key: Option<Arc<tfhe::CompactPublicKey>>,
    crs: Option<Arc<tfhe::zk::CompactPkeCrs>>,
    input_factory: Option<InputBuilderFactory>,
}

impl FhevmSdk {
    /// Create a new SDK instance
    pub fn new(config: FhevmConfig) -> Self {
        log::info!("Creating new FHEVM SDK instance");
        Self {
            config,
            public_key: None,
            crs: None,
            input_factory: None,
        }
    }

    /// Ensure keys are loaded from the configured path
    fn ensure_keys_loaded(&mut self) -> Result<()> {
        if self.public_key.is_none() || self.crs.is_none() {
            log::debug!("Loading keys from {}", self.config.keys_directory.display());

            let (public_key, _client_key, _server_key, crs) =
                utils::load_fhe_keyset(&self.config.keys_directory)?;

            log::info!("Keys loaded successfully");
            self.public_key = Some(Arc::new(public_key));
            self.crs = Some(Arc::new(crs));
        }
        Ok(())
    }

    /// Create a new SDK instance by loading configuration from a YAML file
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = serde_yaml::from_str(&contents)?;
        Ok(Self::new(config))
    }

    /// Create a new SDK instance from a YAML string
    pub fn from_yaml_str(yaml: &str) -> Result<Self> {
        let config = serde_yaml::from_str(yaml)?;
        Ok(Self::new(config))
    }

    /// Generate calldata for UserDecrypt operation
    pub fn generate_user_decrypt_calldata(
        &self,
        _ct_handles: &[Vec<u8>],
        _user_address: &str,
    ) -> Result<Vec<u8>> {
        // Placeholder
        Ok(vec![])
    }

    /// Generate an EIP-712 signature for user decrypt
    pub fn generate_eip712_for_user_decrypt(
        &self,
        public_key: &[u8],
        contract_addresses: &[Address],
        start_timestamp: u64,
        duration_days: u64,
    ) -> Result<B256> {
        // Placeholder for EIP-712 signature generation
        // let _res = generate_eip712_user_decrypt(ct_handles, user_address, 1u64);

        let input_verifier_address_str = self
            .config
            .gateway_contracts
            .get("input-verifier")
            .ok_or_else(|| {
                FhevmError::InvalidParams("Input verifier contract address is not set".to_string())
            })?;

        let input_verifier_address =
            match alloy::primitives::Address::from_str(input_verifier_address_str) {
                Ok(addr) => addr,
                Err(_) => {
                    return Err(FhevmError::InvalidParams(
                        "Invalid ACL contract address".to_string(),
                    ));
                }
            };
        let builder = Eip712Builder::new(
            self.config.gateway_chain_id,
            input_verifier_address,
            self.config.host_chain_id,
        );
        builder.build_user_decrypt_hash(
            public_key,
            contract_addresses,
            start_timestamp,
            duration_days,
        )
    }

    /// Generate calldata for UserDelegatedDecrypt operation
    pub fn generate_user_delegated_decrypt_calldata(
        &self,
        _ct_handles: &[Vec<u8>],
        _user_address: &str,
        _delegate_address: &str,
    ) -> Result<Vec<u8>> {
        // Placeholder
        Ok(vec![])
    }

    /// Generate calldata for PublicDecrypt operation
    pub fn generate_public_decrypt_calldata(&self, _ct_handles: &[Vec<u8>]) -> Result<Vec<u8>> {
        // Placeholder
        Ok(vec![])
    }

    /// Generate calldata for Input operation
    pub fn generate_input_calldata(&self, _ciphertext: &[u8], _proof: &[u8]) -> Result<Vec<u8>> {
        // Placeholder
        Ok(vec![])
    }

    /// Get an input builder factory for creating encrypted inputs
    pub fn get_input_factory(&mut self) -> Result<&InputBuilderFactory> {
        if self.input_factory.is_none() {
            // Load public key and CRS from config

            self.ensure_keys_loaded()?;

            // Get ACL contract address from config
            let acl_address_str = self.config.host_contracts.get("acl").ok_or_else(|| {
                FhevmError::InvalidParams("ACL contract address is not set".to_string())
            })?;

            let acl_address = match alloy::primitives::Address::from_str(acl_address_str) {
                Ok(addr) => addr,
                Err(_) => {
                    return Err(FhevmError::InvalidParams(
                        "Invalid ACL contract address".to_string(),
                    ));
                }
            };

            let public_key = self
                .public_key
                .as_ref()
                .ok_or_else(|| FhevmError::InvalidParams("Public key not loaded".to_string()))?
                .clone();

            let crs = self
                .crs
                .as_ref()
                .ok_or_else(|| FhevmError::InvalidParams("CRS not loaded".to_string()))?
                .clone();

            // Create factory
            self.input_factory = Some(InputBuilderFactory::new(
                acl_address,
                self.config.host_chain_id,
                public_key,
                crs,
            ));
        }
        self.input_factory
            .as_ref()
            .ok_or_else(|| FhevmError::InvalidParams("Failed to create input factory".to_string()))
    }

    /// Create a new encrypted input builder
    pub fn create_input_builder(&mut self) -> Result<EncryptedInputBuilder> {
        log::debug!("Creating encrypted input builder");
        let factory = self.get_input_factory()?;
        Ok(factory.create_builder())
    }
}

// Define modules
pub mod blockchain;
pub mod decryption;
pub mod encryption;
pub mod logging;
pub mod signature;
pub mod utils;
pub mod verification;

pub use encryption::input::{EncryptedInput, EncryptedInputBuilder, InputBuilderFactory};

// Simple type definitions
pub mod types {
    use serde::{Deserialize, Serialize};

    /// Handle to a ciphertext stored on-chain
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CiphertextHandle(pub Vec<u8>);

    /// Decrypted value
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DecryptedValue(pub Vec<u8>);
}

// Create a builder for easier SDK configuration
pub struct FhevmSdkBuilder {
    /// Path to the directory containing key files
    keys_directory: Option<PathBuf>,
    gateway_chain_id: Option<u64>,
    host_chain_id: Option<u64>,
    gateway_contracts: HashMap<String, String>,
    host_contracts: HashMap<String, String>,
}

impl Default for FhevmSdkBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FhevmSdkBuilder {
    pub fn new() -> Self {
        Self {
            keys_directory: None,
            gateway_chain_id: None,
            host_chain_id: None,
            gateway_contracts: HashMap::new(),
            host_contracts: HashMap::new(),
        }
    }

    pub fn with_keys_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.keys_directory = Some(path.as_ref().to_path_buf());
        self
    }

    // Generate new keys if they don't exist
    pub fn with_keys_directory_or_generate<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();

        // Check if keys exist, generate if not
        if !path_buf.exists() || !path_buf.join("public_key.bin").exists() {
            log::info!(
                "Keys not found at {}, generating new keys...",
                path_buf.display()
            );
            utils::generate_fhe_keyset(&path_buf)?;
        }

        self.keys_directory = Some(path_buf);
        Ok(self)
    }

    pub fn with_gateway_chain_id(mut self, chain_id: u64) -> Self {
        self.gateway_chain_id = Some(chain_id);
        self
    }

    pub fn with_host_chain_id(mut self, chain_id: u64) -> Self {
        self.host_chain_id = Some(chain_id);
        self
    }

    pub fn with_gateway_contract(mut self, name: &str, address: &str) -> Self {
        self.gateway_contracts
            .insert(name.to_string(), address.to_string());
        self
    }

    pub fn with_host_contract(mut self, name: &str, address: &str) -> Self {
        self.host_contracts
            .insert(name.to_string(), address.to_string());
        self
    }

    /// Export the current builder state to YAML
    pub fn to_yaml(&self) -> Result<String> {
        // Convert builder to config
        let config = self.to_config()?;

        // Serialize to YAML
        let yaml = serde_yaml::to_string(&config).map_err(FhevmError::YamlError)?;

        Ok(yaml)
    }

    /// Save the current builder state to a YAML file
    pub fn save_to_yaml<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = self.to_yaml()?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    /// Convert the builder to a config
    fn to_config(&self) -> Result<FhevmConfig> {
        // Validate required fields
        let keys_directory = self
            .keys_directory
            .clone()
            .ok_or_else(|| FhevmError::InvalidParams("Keys directory is required".to_string()))?;

        let gateway_chain_id = self
            .gateway_chain_id
            .ok_or_else(|| FhevmError::InvalidParams("Gateway chain ID is required".to_string()))?;

        let host_chain_id = self
            .host_chain_id
            .ok_or_else(|| FhevmError::InvalidParams("Host chain ID is required".to_string()))?;

        // Ensure ACL contract is defined
        if !self.host_contracts.contains_key("acl") {
            return Err(FhevmError::InvalidParams(
                "ACL contract address is required in host_contracts".to_string(),
            ));
        }

        if !self.gateway_contracts.contains_key("input-verifier") {
            return Err(FhevmError::InvalidParams(
                "Input verifier contract address is required in gateway_contracts".to_string(),
            ));
        }

        // Create the config
        let config = FhevmConfig {
            keys_directory,
            gateway_chain_id,
            host_chain_id,
            gateway_contracts: self.gateway_contracts.clone(),
            host_contracts: self.host_contracts.clone(),
        };

        Ok(config)
    }

    pub fn build(self) -> Result<FhevmSdk> {
        // Convert to config and create the SDK
        log::debug!("Building FhevmSdk from builder");
        let config = self.to_config()?;

        log::info!("SDK configuration validated successfully");
        // Create and return the SDK
        Ok(FhevmSdk::new(config))
    }
}
