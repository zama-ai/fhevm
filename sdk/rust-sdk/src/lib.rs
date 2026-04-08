//! # FHEVM SDK
//!
//! A Rust SDK for interacting with FHEVM networks.
//!
//! Built on top of [`fhevm_client_core`] which provides the platform-agnostic
//! cryptographic operations. This crate adds filesystem-based key management,
//! ML-KEM keypair generation, YAML configuration, and KMS response processing.

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, info, warn};

// Re-export core types that are identical
pub use fhevm_client_core::{GatewayContracts, HostContracts};

use alloy::primitives::Address;
use decryption::user::{UserDecryptRequestBuilder, UserDecryptionResponseBuilder};

/// Configuration for the FHEVM SDK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhevmConfig {
    /// Path to the directory containing key files. Required to create inputs with the SDK.
    pub keys_directory: Option<PathBuf>,
    /// Gateway chain ID
    pub gateway_chain_id: u64,
    /// Host chain ID
    pub host_chain_id: u64,
    /// Contract addresses on Gateway chain
    pub gateway_contracts: GatewayContracts,
    /// Contract addresses on Host chain
    pub host_contracts: HostContracts,
}

impl FhevmConfig {
    /// Convert to a [`fhevm_client_core::ClientCoreConfig`] (drops `keys_directory`).
    pub fn to_core_config(&self) -> fhevm_client_core::ClientCoreConfig {
        fhevm_client_core::ClientCoreConfig {
            gateway_chain_id: self.gateway_chain_id,
            host_chain_id: self.host_chain_id,
            gateway_contracts: self.gateway_contracts.clone(),
            host_contracts: self.host_contracts.clone(),
        }
    }
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

    #[error("Key generation error: {0}")]
    KeyGenerationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("File error: {0}")]
    FileError(String),

    #[error("Address parsing error: {0}")]
    AddressError(String),

    #[error("Hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("Alloy parse error: {0}")]
    AlloyParseError(#[from] alloy::primitives::ruint::ParseError),

    #[error("Client core error: {0}")]
    CoreError(#[from] fhevm_client_core::ClientCoreError),
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
        info!("Creating new FHEVM SDK instance");
        Self {
            config,
            public_key: None,
            crs: None,
            input_factory: None,
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &FhevmConfig {
        &self.config
    }

    /// Get the gateway chain ID
    pub fn gateway_chain_id(&self) -> u64 {
        self.config.gateway_chain_id
    }

    /// Get the host chain ID
    pub fn host_chain_id(&self) -> u64 {
        self.config.host_chain_id
    }

    /// Check if all required contracts are configured
    pub fn is_fully_configured(&self) -> bool {
        self.config.gateway_contracts.input_verification.is_some()
            && self.config.gateway_contracts.decryption.is_some()
            && self.config.host_contracts.acl.is_some()
    }

    /// Get a summary of the configuration status
    pub fn configuration_status(&self) -> String {
        let mut status = String::new();
        status.push_str("FHEVM SDK Configuration Status:\n");
        status.push_str(&format!(
            "  Gateway Chain ID: {}\n",
            self.config.gateway_chain_id
        ));
        status.push_str(&format!("  Host Chain ID: {}\n", self.config.host_chain_id));
        status.push_str(&format!(
            "  Keys Directory: {:?}\n",
            self.config.keys_directory
        ));

        status.push_str("\nGateway Contracts:\n");
        status.push_str(&format!(
            "  Input Verification: {}\n",
            self.config
                .gateway_contracts
                .input_verification
                .map(|a| a.to_string())
                .unwrap_or_else(|| "Not Set".to_string())
        ));
        status.push_str(&format!(
            "  Decryption: {}\n",
            self.config
                .gateway_contracts
                .decryption
                .map(|a| a.to_string())
                .unwrap_or_else(|| "Not Set".to_string())
        ));

        status.push_str("\nHost Contracts:\n");
        status.push_str(&format!(
            "  ACL: {}\n",
            self.config
                .host_contracts
                .acl
                .map(|a| a.to_string())
                .unwrap_or_else(|| "Not Set".to_string())
        ));

        status.push_str(&format!(
            "\nKeys Loaded: {}",
            self.public_key.is_some() && self.crs.is_some()
        ));

        status
    }

    /// Ensure keys are loaded from the configured path
    fn ensure_keys_loaded(&mut self) -> Result<()> {
        if self.public_key.is_none() || self.crs.is_none() {
            let keys_directory = self
                .config
                .keys_directory
                .as_ref()
                .expect("Keys directory must be configured");

            debug!("Loading keys from {}", keys_directory.display());
            let (public_key, _client_key, _server_key, crs) =
                utils::load_fhe_keyset(keys_directory)?;

            info!("Keys loaded successfully");
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

    /// Create an EIP-712 signature builder for user decrypt operations
    pub fn create_eip712_signature_builder(&self) -> signature::eip712::Eip712SignatureBuilder {
        let verifying_contract = self.config.gateway_contracts.decryption.unwrap_or_else(|| {
            warn!("Decryption contract not set, using zero address");
            Address::ZERO
        });

        let config = signature::eip712::Eip712Config {
            gateway_chain_id: self.config.gateway_chain_id,
            verifying_contract,
            contracts_chain_id: self.config.host_chain_id,
        };

        signature::eip712::Eip712SignatureBuilder::new(config)
    }

    /// Alternative shorter name for discoverability
    pub fn eip712_builder(&self) -> signature::eip712::Eip712SignatureBuilder {
        self.create_eip712_signature_builder()
    }

    /// Generate a new cryptobox keypair
    pub fn generate_keypair(&self) -> Result<signature::Keypair> {
        signature::generate_keypair()
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
    pub fn generate_public_decrypt_calldata(&self, ct_handles: &[Vec<u8>]) -> Result<Vec<u8>> {
        info!(
            "🔓 Generating public decrypt calldata for {} handles",
            ct_handles.len()
        );

        let calldata = self
            .create_public_decrypt_request_builder()
            .with_handles_from_bytes(ct_handles)?
            .build_and_generate_calldata()?;

        info!(
            "✅ Generated public decrypt calldata: {} bytes",
            calldata.len()
        );
        Ok(calldata)
    }

    /// Generate calldata for Input verification operation
    pub fn generate_verify_proof_calldata(
        &self,
        encrypted_input: &EncryptedInput,
    ) -> Result<Vec<u8>> {
        info!("🔍 Generating verify proof calldata");
        debug!("   Contract: {}", encrypted_input.contract_address);
        debug!("   User: {}", encrypted_input.user_address);
        debug!("   Handles: {}", encrypted_input.handles.len());

        let calldata = crate::blockchain::calldata::verify_proof_req(
            encrypted_input.chain_id,
            encrypted_input.contract_address,
            encrypted_input.user_address,
            encrypted_input.ciphertext.clone().into(),
        )?;

        info!(
            "✅ Generated verify proof calldata: {} bytes",
            calldata.len()
        );
        Ok(calldata.to_vec())
    }

    /// Create an input builder factory for creating encrypted inputs
    pub fn create_input_factory(&mut self) -> Result<()> {
        if self.input_factory.is_none() {
            self.ensure_keys_loaded()?;

            let acl_address = self.config.host_contracts.acl.ok_or_else(|| {
                FhevmError::InvalidParams("ACL contract address is not set".to_string())
            })?;

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

            self.input_factory = Some(InputBuilderFactory::new(
                acl_address,
                self.config.host_chain_id,
                public_key,
                crs,
            ));
        }

        Ok(())
    }

    /// Get an input builder factory for creating encrypted inputs
    pub fn get_input_factory(&self) -> Result<&InputBuilderFactory> {
        self.input_factory
            .as_ref()
            .ok_or_else(|| FhevmError::InvalidParams("Failed to create input factory".to_string()))
    }

    /// Create a new encrypted input builder
    pub fn create_input_builder(&self) -> Result<EncryptedInputBuilder> {
        debug!("Creating encrypted input builder");
        let factory = self.get_input_factory()?;
        Ok(factory.create_builder())
    }

    /// Create a user decrypt request builder
    pub fn create_user_decrypt_request_builder(&self) -> UserDecryptRequestBuilder {
        UserDecryptRequestBuilder::new().with_contracts_chain_id(self.config.host_chain_id)
    }

    /// Alternative shorter name for discoverability
    pub fn user_decrypt_request_builder(&self) -> UserDecryptRequestBuilder {
        self.create_user_decrypt_request_builder()
    }

    pub fn create_user_decrypt_response_builder(&self) -> UserDecryptionResponseBuilder {
        UserDecryptionResponseBuilder::new().with_gateway_chain_id(self.config.gateway_chain_id)
    }

    /// Alternative shorter name for discoverability
    pub fn user_decrypt_response_builder(&self) -> UserDecryptionResponseBuilder {
        self.create_user_decrypt_response_builder()
    }

    /// Builder pattern for creating PublicDecryptRequest instances
    pub fn create_public_decrypt_request_builder(
        &self,
    ) -> decryption::public::PublicDecryptRequestBuilder {
        decryption::public::PublicDecryptRequestBuilder::new()
    }

    /// Alternative shorter name for discoverability
    pub fn public_decrypt_request_builder(
        &self,
    ) -> decryption::public::PublicDecryptRequestBuilder {
        self.create_public_decrypt_request_builder()
    }

    /// Create a public decrypt response builder
    pub fn create_public_decrypt_response_builder(
        &self,
    ) -> decryption::public::PublicDecryptionResponseBuilder {
        decryption::public::PublicDecryptionResponseBuilder::new()
            .with_gateway_chain_id(self.config.gateway_chain_id)
    }

    /// Alternative shorter name for discoverability
    pub fn public_decrypt_response_builder(
        &self,
    ) -> decryption::public::PublicDecryptionResponseBuilder {
        self.create_public_decrypt_response_builder()
    }
}

// Define modules
pub mod blockchain;
pub mod decryption;
pub mod encryption;
pub mod logging;
pub mod signature;
pub mod utils;

// Re-export core encryption types (these come from fhevm_client_core)
pub use fhevm_client_core::encryption::input::{
    EncryptedInput, EncryptedInputBuilder, InputBuilderFactory,
};

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
    gateway_contracts: GatewayContracts,
    host_contracts: HostContracts,
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
            gateway_contracts: GatewayContracts {
                input_verification: None,
                decryption: None,
            },
            host_contracts: HostContracts { acl: None },
        }
    }

    pub fn with_keys_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.keys_directory = Some(path.as_ref().to_path_buf());
        self
    }

    // Generate new keys if they don't exist
    pub fn with_keys_directory_or_generate<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();

        if !path_buf.exists() || !path_buf.join("public_key.bin").exists() {
            info!(
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
        let addr = Address::from_str(address).unwrap_or_else(|_| {
            panic!("Invalid address provided for gateway contract '{name}': {address}")
        });

        match name.to_lowercase().as_str() {
            "input_verification" | "input-verifier" | "input-verification" => {
                self.gateway_contracts.input_verification = Some(addr);
            }
            "decryption" => {
                self.gateway_contracts.decryption = Some(addr);
            }
            _ => {
                warn!(
                    "Unknown gateway contract name: '{}'. Valid names are: 'input_verification', 'decryption'",
                    name
                );
            }
        }
        self
    }

    pub fn with_input_verification_contract(mut self, address: &str) -> Self {
        self.gateway_contracts.input_verification =
            Some(Address::from_str(address).unwrap_or_else(|_| {
                panic!("Invalid address provided for input verification contract: {address}")
            }));
        self
    }

    pub fn with_decryption_contract(mut self, address: &str) -> Self {
        self.gateway_contracts.decryption = Some(Address::from_str(address).unwrap_or_else(|_| {
            panic!("Invalid address provided for decryption contract: {address}")
        }));
        self
    }

    pub fn with_acl_contract(mut self, address: &str) -> Self {
        self.host_contracts.acl =
            Some(Address::from_str(address).unwrap_or_else(|_| {
                panic!("Invalid address provided for ACL contract: {address}")
            }));
        self
    }

    pub fn with_host_contract(mut self, name: &str, address: &str) -> Self {
        let addr = Address::from_str(address).unwrap_or_else(|_| {
            panic!("Invalid address provided for host contract '{name}': {address}")
        });

        match name.to_lowercase().as_str() {
            "acl" => {
                self.host_contracts.acl = Some(addr);
            }
            _ => {
                warn!(
                    "Unknown host contract name: '{}'. Valid names are: 'acl'",
                    name
                );
            }
        }
        self
    }

    /// Export the current builder state to YAML
    pub fn to_yaml(&self) -> Result<String> {
        let config = self.to_config()?;
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
        let keys_directory = self.keys_directory.clone();

        let gateway_chain_id = self
            .gateway_chain_id
            .ok_or_else(|| FhevmError::InvalidParams("Gateway chain ID is required".to_string()))?;

        let host_chain_id = self
            .host_chain_id
            .ok_or_else(|| FhevmError::InvalidParams("Host chain ID is required".to_string()))?;

        if self.host_contracts.acl.is_none() {
            return Err(FhevmError::InvalidParams(
                "ACL contract address is required in host_contracts".to_string(),
            ));
        }

        if self.gateway_contracts.input_verification.is_none() {
            return Err(FhevmError::InvalidParams(
                "Input verification contract address is required in gateway_contracts".to_string(),
            ));
        }

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
        debug!("Building FhevmSdk from builder");
        let config = self.to_config()?;
        let is_keys_directory_set = config.keys_directory.is_some();

        info!("SDK configuration validated successfully");

        let mut fhevm = FhevmSdk::new(config);

        if is_keys_directory_set {
            fhevm.ensure_keys_loaded()?;
            fhevm.create_input_factory()?;
        }

        Ok(fhevm)
    }
}
