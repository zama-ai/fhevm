use alloy::hex::decode;
use alloy_primitives::{Address, ChainId, B256};
use alloy_signer::{Signer, SignerSync};
use alloy_signer_local::{coins_bip39::English, MnemonicBuilder, PrivateKeySigner};
use std::fs::File;
use std::path::{Path, PathBuf};
use tfhe::safe_serialization::safe_deserialize;
use thiserror::Error;
use tracing::{debug, info};

// Private signing key module to avoid Result type conflicts
mod private_sig_key {
    use alloy::signers::k256;
    use serde::{de::Visitor, Deserialize, Serialize};
    use tfhe::named::Named;
    use tfhe_versionable::{Versionize, VersionsDispatch};

    macro_rules! impl_generic_versionize {
        ($t:ty) => {
            impl tfhe_versionable::Versionize for $t {
                type Versioned<'vers> = &'vers $t;

                fn versionize(&self) -> Self::Versioned<'_> {
                    self
                }
            }

            impl tfhe_versionable::VersionizeOwned for $t {
                type VersionedOwned = $t;
                fn versionize_owned(self) -> Self::VersionedOwned {
                    self
                }
            }

            impl tfhe_versionable::Unversionize for $t {
                fn unversionize(
                    versioned: Self::VersionedOwned,
                ) -> Result<Self, tfhe_versionable::UnversionizeError> {
                    Ok(versioned)
                }
            }

            impl tfhe_versionable::NotVersioned for $t {}
        };
    }

    #[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, VersionsDispatch)]
    pub enum PrivateSigKeyVersioned {
        V0(PrivateSigKey),
    }

    #[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Versionize)]
    #[versionize(PrivateSigKeyVersioned)]
    pub struct PrivateSigKey {
        sk: WrappedSigningKey,
    }

    impl Named for PrivateSigKey {
        const NAME: &'static str = "PrivateSigKey";
    }

    impl PrivateSigKey {
        pub fn sk(&self) -> &k256::ecdsa::SigningKey {
            &self.sk.0
        }
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct WrappedSigningKey(pub(crate) k256::ecdsa::SigningKey);
    impl_generic_versionize!(WrappedSigningKey);

    impl Serialize for WrappedSigningKey {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bytes(&self.0.to_bytes())
        }
    }

    impl<'de> Deserialize<'de> for WrappedSigningKey {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_bytes(PrivateSigKeyVisitor)
        }
    }

    struct PrivateSigKeyVisitor;
    impl Visitor<'_> for PrivateSigKeyVisitor {
        type Value = WrappedSigningKey;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("A signing key for ECDSA signatures using secp256k1")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match k256::ecdsa::SigningKey::from_bytes(v.into()) {
                Ok(sk) => Ok(WrappedSigningKey(sk)),
                Err(e) => Err(E::custom(format!("Could not decode signing key: {:?}", e))),
            }
        }
    }
}

// Re-export the PrivateSigKey for use in this module
use private_sig_key::PrivateSigKey;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Signer error: {0}")]
    SignerError(#[from] alloy_signer::Error),
    #[error("Local signer error: {0}")]
    LocalSignerError(#[from] alloy_signer_local::LocalSignerError),
    #[error("Failed to load wallet: {0}")]
    LoadError(String),
    #[error("Failed to load signing key: {0}")]
    SigningKeyError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
}

pub type Result<T> = std::result::Result<T, WalletError>;

/// KMS wallet for signing decryption responses
///
/// This wallet implementation provides functionality for:
/// - Creating wallets from mnemonic phrases or files
/// - Creating wallets from signing key files
/// - Generating random wallets
/// - Signing messages, hashes, and decryption responses
///
/// It serves as a critical security component in the KMS Connector,
/// handling all cryptographic operations for blockchain interactions.
#[derive(Clone, Debug)]
pub struct KmsWallet {
    pub signer: PrivateKeySigner,
}

impl KmsWallet {
    /// Create a new wallet from a mnemonic phrase
    pub fn from_mnemonic(phrase: &str, chain_id: Option<ChainId>) -> Result<Self> {
        Self::from_mnemonic_with_index(phrase, 0, chain_id)
    }

    /// Create a new wallet from a mnemonic phrase with a specific account index
    pub fn from_mnemonic_with_index(
        phrase: &str,
        account_index: u32,
        chain_id: Option<ChainId>,
    ) -> Result<Self> {
        let derivation_path = format!("m/44'/60'/0'/0/{}", account_index);

        let signer = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(&derivation_path)?
            .build()?
            .with_chain_id(chain_id);

        info!("Created wallet from mnemonic phrase");
        Ok(Self { signer })
    }

    /// Create a new wallet from a mnemonic file
    pub fn from_mnemonic_file(path: PathBuf, chain_id: Option<ChainId>) -> Result<Self> {
        debug!("Loading mnemonic from file: {}", path.display());
        let phrase = std::fs::read_to_string(&path).map_err(|e| {
            WalletError::LoadError(format!(
                "Failed to read mnemonic file at {}: {}",
                path.display(),
                e
            ))
        })?;

        info!("Successfully read mnemonic file");
        Self::from_mnemonic(phrase.trim(), chain_id)
    }

    /// Create a new wallet from a signing key file
    pub fn from_signing_key_file<P: AsRef<Path>>(
        path: Option<P>,
        chain_id: Option<ChainId>,
    ) -> Result<Self> {
        // Default path relative to the project
        // In production, this should be configured via environment variables or config files
        let default_path = "../keys/CLIENT/SigningKey/e164d9de0bec6656928726433cc56bef6ee8417ad5a4f8c82fbcc2d3e5f220fd";

        // Use provided path or default
        let file_path = match path {
            Some(p) => p.as_ref().to_path_buf(),
            None => PathBuf::from(default_path),
        };

        // Open the file
        let f = File::open(&file_path).map_err(|e| {
            WalletError::LoadError(format!(
                "Failed to open signing key file at {}: {}",
                file_path.display(),
                e
            ))
        })?;

        // Deserialize the private signing key
        const SIZE_LIMIT: u64 = 1024;
        let sk: PrivateSigKey = safe_deserialize(f, SIZE_LIMIT).map_err(|e| {
            WalletError::SigningKeyError(format!("Failed to deserialize signing key: {:?}", e))
        })?;

        // Create signer from the signing key
        let signer = PrivateKeySigner::from_signing_key(sk.sk().clone()).with_chain_id(chain_id);

        info!("Created wallet from signing key file");
        Ok(Self { signer })
    }

    /// Create a new wallet from a private key string
    ///
    /// The private key string should be a hexadecimal string with or without '0x' prefix.
    /// This method is particularly useful for testing or when the private key is stored
    /// as a string in a secure environment variable or configuration.
    pub fn from_private_key_str(private_key: &str, chain_id: Option<ChainId>) -> Result<Self> {
        debug!("Creating wallet from private key string");

        // Remove 0x prefix if present
        let private_key = private_key.trim_start_matches("0x");

        // Convert hex string to bytes
        let bytes = decode(private_key)
            .map_err(|e| WalletError::InvalidPrivateKey(format!("Invalid hex encoding: {}", e)))?;

        // Ensure the key is the correct length
        if bytes.len() != 32 {
            return Err(WalletError::InvalidPrivateKey(format!(
                "Private key must be 32 bytes, got {} bytes",
                bytes.len()
            )));
        }

        // Create a signing key from the bytes
        let signing_key = alloy::signers::k256::ecdsa::SigningKey::from_bytes(
            bytes.as_slice().into(),
        )
        .map_err(|e| WalletError::InvalidPrivateKey(format!("Invalid private key: {}", e)))?;

        // Create signer from the signing key
        let signer = PrivateKeySigner::from_signing_key(signing_key).with_chain_id(chain_id);

        info!("Created wallet from private key string");
        Ok(Self { signer })
    }

    /// Create a new random wallet
    pub fn random(chain_id: Option<ChainId>) -> Result<Self> {
        let signer = PrivateKeySigner::random().with_chain_id(chain_id);
        info!("Created random wallet");
        Ok(Self { signer })
    }

    /// Get the wallet's address
    pub fn address(&self) -> Address {
        debug!("Getting wallet address");
        self.signer.address()
    }

    /// Sign a message
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        debug!("Signing message");
        Ok(self.signer.sign_message_sync(message)?.as_bytes().to_vec())
    }

    /// Sign a hash
    pub fn sign_hash(&self, hash: &B256) -> Result<Vec<u8>> {
        // Get signature
        debug!("Signing hash");
        let sig = self.signer.sign_hash_sync(hash)?;
        Ok(Vec::from(&sig))
    }

    /// Sign a decryption response
    pub fn sign_decryption_response(&self, id: &[u8], result: &[u8]) -> Result<Vec<u8>> {
        // Create message to sign: keccak256(abi.encodePacked(id, result))
        debug!("Signing decryption response");
        let mut message = Vec::with_capacity(id.len() + result.len());
        message.extend_from_slice(id);
        message.extend_from_slice(result);
        self.sign_message(&message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CHAIN_ID: u64 = 1337;

    #[test]
    fn test_wallet_from_mnemonic() {
        let wallet = KmsWallet::random(Some(TEST_CHAIN_ID)).unwrap();
        assert!(wallet.address() != Address::ZERO);
    }

    #[test]
    fn test_sign_decryption_response() {
        let wallet = KmsWallet::random(Some(TEST_CHAIN_ID)).unwrap();

        let id = b"test_id";
        let result = b"test_result";
        let signature = wallet.sign_decryption_response(id, result).unwrap();

        assert!(!signature.is_empty());
    }

    #[test]
    #[ignore] // Ignore by default as it requires the actual file to exist
    fn test_wallet_from_signing_key_file() {
        // This test assumes the signing key file exists at the default path
        // Run with: cargo test -- --ignored
        let wallet = KmsWallet::from_signing_key_file::<PathBuf>(None, Some(TEST_CHAIN_ID));
        assert!(wallet.is_ok(), "Failed to load wallet: {:?}", wallet.err());
        assert!(wallet.unwrap().address() != Address::ZERO);
    }

    #[test]
    fn test_wallet_from_private_key_str() {
        // Test private key (this is a test key, never use in production)
        let private_key = "8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f";

        // Create wallet from private key string
        let wallet = KmsWallet::from_private_key_str(private_key, Some(TEST_CHAIN_ID)).unwrap();

        // Expected address for this private key
        let expected_address =
            Address::parse_checksummed("0x63FaC9201494f0bd17B9892B9fae4d52fe3BD377", None).unwrap();

        // Verify the address matches
        assert_eq!(wallet.address(), expected_address);

        // Test with 0x prefix
        let wallet_with_prefix =
            KmsWallet::from_private_key_str(&format!("0x{}", private_key), Some(TEST_CHAIN_ID))
                .unwrap();
        assert_eq!(wallet_with_prefix.address(), expected_address);

        // Test signing
        let message = b"test message";
        let signature = wallet.sign_message(message).unwrap();
        assert!(!signature.is_empty());
    }

    #[test]
    fn test_wallet_from_private_key_str_invalid() {
        // Test with invalid hex string
        let result = KmsWallet::from_private_key_str("not a hex string", Some(TEST_CHAIN_ID));
        assert!(result.is_err());

        // Test with wrong length
        let result = KmsWallet::from_private_key_str("deadbeef", Some(TEST_CHAIN_ID));
        assert!(result.is_err());
    }
}
