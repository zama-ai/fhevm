//! Input processing module for FHEVM client core.
//!
//! Provides a factory for creating encrypted input builders. All key material
//! is accepted as pre-loaded `Arc` references — no filesystem access.

use crate::encryption::primitives::EncryptionType;
use crate::utils::chain_id_to_bytes;
use crate::{ClientCoreError, Result};
use alloy::primitives::{Address, keccak256};
use tracing::debug;

use crate::encryption::CIPHERTEXT_VERSION;
use crate::encryption::IntoU256;
use std::sync::Arc;
use tfhe::{safe_serialization::safe_serialize, zk::ZkComputeLoad};

const RAW_CT_HASH_DOMAIN_SEPARATOR: &str = "ZK-w_rct";
const HANDLE_HASH_DOMAIN_SEPARATOR: &str = "ZK-w_hdl";

/// Factory for creating encrypted input builders.
///
/// Holds shared key material and chain configuration. Create once,
/// then call [`create_builder`](InputBuilderFactory::create_builder) for each input.
pub struct InputBuilderFactory {
    acl_contract_address: Address,
    chain_id: u64,
    public_key: Arc<tfhe::CompactPublicKey>,
    crs: Arc<tfhe::zk::CompactPkeCrs>,
}

impl InputBuilderFactory {
    /// Create a new factory with pre-loaded key material.
    pub fn new(
        acl_contract_address: Address,
        chain_id: u64,
        public_key: Arc<tfhe::CompactPublicKey>,
        crs: Arc<tfhe::zk::CompactPkeCrs>,
    ) -> Self {
        Self {
            acl_contract_address,
            chain_id,
            public_key,
            crs,
        }
    }

    /// Create a new builder with this factory's configuration.
    pub fn create_builder(&self) -> EncryptedInputBuilder {
        EncryptedInputBuilder::new(
            self.acl_contract_address,
            self.public_key.clone(),
            self.crs.clone(),
            self.chain_id,
        )
    }

    /// Get the chain ID for this factory.
    pub fn get_chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Get the ACL contract address for this factory.
    pub fn get_acl_contract_address(&self) -> Address {
        self.acl_contract_address
    }
}

/// Builder for encrypted inputs with ZK proofs.
///
/// Add values of various types, then call
/// [`encrypt_and_prove_for`](EncryptedInputBuilder::encrypt_and_prove_for)
/// to produce a serialized ciphertext with proof and per-value handles.
///
/// ## Limits
/// - Maximum 2048 encrypted bits per input (booleans cost 2 encrypted bits each)
/// - Maximum 256 values per input
pub struct EncryptedInputBuilder {
    acl_contract_address: Address,
    chain_id: u64,
    public_key: Arc<tfhe::CompactPublicKey>,
    crs: Arc<tfhe::zk::CompactPkeCrs>,
    builder: tfhe::CompactCiphertextListBuilder,
    /// Logical type widths for handle computation (1 for bool, 8 for u8, etc.)
    bits: Vec<usize>,
    /// Running total of actual encrypted bits consumed (2 for bool, 8 for u8, etc.)
    encrypted_bits_total: usize,
}

impl EncryptedInputBuilder {
    /// Create a new builder with the given key material and chain configuration.
    pub fn new(
        acl_contract_address: Address,
        public_key: Arc<tfhe::CompactPublicKey>,
        crs: Arc<tfhe::zk::CompactPkeCrs>,
        chain_id: u64,
    ) -> Self {
        let builder = tfhe::ProvenCompactCiphertextList::builder(&public_key);
        Self {
            crs,
            builder,
            bits: Vec::new(),
            encrypted_bits_total: 0,
            acl_contract_address,
            chain_id,
            public_key,
        }
    }

    /// Get the ACL contract address.
    pub fn acl_contract_address(&self) -> &Address {
        &self.acl_contract_address
    }

    /// Get the chain ID.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn check_limit(&self, encrypted_bits: usize) -> Result<()> {
        let total_bits = self.encrypted_bits_total + encrypted_bits;
        if total_bits > 2048 {
            return Err(ClientCoreError::EncryptionError(
                "Exceeds maximum 2048 bits in a single input ciphertext".to_string(),
            ));
        }
        if self.bits.len() + 1 > 256 {
            return Err(ClientCoreError::EncryptionError(
                "Exceeds maximum 256 variables in a single input ciphertext".to_string(),
            ));
        }
        Ok(())
    }

    /// Add a boolean value. Costs 2 encrypted bits.
    pub fn add_bool(&mut self, value: bool) -> Result<&mut Self> {
        self.check_limit(2)?;
        self.builder.push(value);
        self.bits.push(1); // logical type width for handle computation
        self.encrypted_bits_total += 2; // actual encrypted bit cost
        Ok(self)
    }

    /// Add a u8 value. Costs 8 encrypted bits.
    pub fn add_u8(&mut self, value: u8) -> Result<&mut Self> {
        self.check_limit(8)?;
        self.builder.push(value);
        self.bits.push(8);
        self.encrypted_bits_total += 8;
        Ok(self)
    }

    /// Add a u16 value. Costs 16 encrypted bits.
    pub fn add_u16(&mut self, value: u16) -> Result<&mut Self> {
        self.check_limit(16)?;
        self.builder.push(value);
        self.bits.push(16);
        self.encrypted_bits_total += 16;
        Ok(self)
    }

    /// Add a u32 value. Costs 32 encrypted bits.
    pub fn add_u32(&mut self, value: u32) -> Result<&mut Self> {
        self.check_limit(32)?;
        self.builder.push(value);
        self.bits.push(32);
        self.encrypted_bits_total += 32;
        Ok(self)
    }

    /// Add a u64 value. Costs 64 encrypted bits.
    pub fn add_u64(&mut self, value: u64) -> Result<&mut Self> {
        self.check_limit(64)?;
        self.builder.push(value);
        self.bits.push(64);
        self.encrypted_bits_total += 64;
        Ok(self)
    }

    /// Add a u128 value. Costs 128 encrypted bits.
    pub fn add_u128(&mut self, value: u128) -> Result<&mut Self> {
        self.check_limit(128)?;
        self.builder.push(value);
        self.bits.push(128);
        self.encrypted_bits_total += 128;
        Ok(self)
    }

    /// Add an Ethereum address (160 bits).
    pub fn add_address(&mut self, address: &str) -> Result<&mut Self> {
        let address = if let Some(stripped) = address.strip_prefix("0x") {
            stripped
        } else {
            address
        };

        if address.len() != 40 {
            return Err(ClientCoreError::EncryptionError(
                "Invalid address length".to_string(),
            ));
        }

        let address_bytes = hex::decode(address)
            .map_err(|e| ClientCoreError::EncryptionError(format!("Invalid hex in address: {e}")))?;

        let mut padded_bytes = [0u8; 32];
        padded_bytes[12..32].copy_from_slice(&address_bytes);

        let mut address_u160 = tfhe::integer::bigint::U256::from(0u128);
        address_u160.copy_from_be_byte_slice(&padded_bytes);

        self.check_limit(160)?;
        self.builder
            .push_with_num_bits(address_u160, 160)
            .map_err(|e| ClientCoreError::EncryptionError(format!("Failed to push address: {e}")))?;

        self.bits.push(160);
        self.encrypted_bits_total += 160;
        Ok(self)
    }

    /// Add a u256 value. Costs 256 encrypted bits.
    pub fn add_u256<T: IntoU256>(&mut self, value: T) -> Result<&mut Self> {
        self.check_limit(256)?;

        let bytes = value.into_u256_bytes()?;

        let mut value_u256 = tfhe::integer::bigint::U256::from(0u128);
        value_u256.copy_from_be_byte_slice(&bytes);

        self.builder.push(value_u256);
        self.bits.push(256);
        self.encrypted_bits_total += 256;
        Ok(self)
    }

    /// Get the logical type widths for all added values.
    pub fn get_bits(&self) -> &[usize] {
        &self.bits
    }

    /// Create auxiliary data for the zero-knowledge proof.
    pub fn create_auxiliary_data(
        &self,
        contract_address: Address,
        user_address: Address,
    ) -> Result<Vec<u8>> {
        // contract_address(20) + user_address(20) + acl_address(20) + chain_id(32)
        let mut aux_data = Vec::with_capacity(92);
        aux_data.extend_from_slice(contract_address.as_slice());
        aux_data.extend_from_slice(user_address.as_slice());
        aux_data.extend_from_slice(self.acl_contract_address.as_slice());
        aux_data.extend_from_slice(&chain_id_to_bytes(self.chain_id));
        Ok(aux_data)
    }

    fn build_with_proof(&mut self, auxiliary_data: &[u8]) -> Result<Vec<u8>> {
        let proven_compact_list = self
            .builder
            .build_with_proof_packed(&self.crs, auxiliary_data, ZkComputeLoad::Verify)
            .map_err(|e| ClientCoreError::EncryptionError(format!("Failed to build proof: {e}")))?;

        let mut buffer = Vec::new();
        safe_serialize(&proven_compact_list, &mut buffer, 1 << 20)
            .map_err(|e| ClientCoreError::EncryptionError(format!("Failed to serialize: {e}")))?;

        Ok(buffer)
    }

    /// Encrypt all added values and produce a ZK proof.
    pub fn encrypt_and_prove_for(
        &mut self,
        contract_address: Address,
        user_address: Address,
    ) -> Result<EncryptedInput> {
        let aux_data = self.create_auxiliary_data(contract_address, user_address)?;
        let ciphertext = self.build_with_proof(&aux_data)?;
        debug!("Ciphertext built successfully: {} bytes", ciphertext.len());

        let bit_widths = self.get_bits();
        debug!("Generating handles for {} values", bit_widths.len());
        let handles = Self::compute_handles(
            &ciphertext,
            bit_widths,
            &self.acl_contract_address,
            self.chain_id,
            CIPHERTEXT_VERSION,
        )?;

        Ok(EncryptedInput {
            ciphertext,
            handles,
            contract_address,
            user_address,
            chain_id: self.chain_id,
        })
    }

    /// Reset the builder to create a new input with the same configuration.
    pub fn clear(&mut self) -> &mut Self {
        self.builder = tfhe::ProvenCompactCiphertextList::builder(&self.public_key);
        self.bits.clear();
        self.encrypted_bits_total = 0;
        self
    }

    /// Compute deterministic handles for each encrypted value.
    ///
    /// Uses [`EncryptionType`] for the bit-width-to-discriminant mapping.
    ///
    /// Handle layout (32 bytes):
    /// `[hash_prefix(0..21), index(21), chain_id(22..30), type(30), version(31)]`
    pub fn compute_handles(
        ciphertext: &[u8],
        bit_widths: &[usize],
        acl_contract_address: &Address,
        chain_id: u64,
        ciphertext_version: u8,
    ) -> Result<Vec<[u8; 32]>> {
        if bit_widths.len() > 256 {
            return Err(ClientCoreError::EncryptionError(
                "Maximum 256 values supported for handle computation".to_string(),
            ));
        }

        let ciphertext_preimage = [RAW_CT_HASH_DOMAIN_SEPARATOR.as_bytes(), ciphertext].concat();
        let ciphertext_hash = keccak256(ciphertext_preimage);
        let chain_id_bytes = chain_id_to_bytes(chain_id);

        let handles = bit_widths
            .iter()
            .enumerate()
            .map(|(index, &bit_width)| {
                let encryption_type = EncryptionType::from_bit_width(bit_width)?.discriminant();

                let index_byte = u8::try_from(index).map_err(|_| {
                    ClientCoreError::EncryptionError(format!(
                        "Handle index {index} exceeds maximum 255"
                    ))
                })?;

                // 8 (domain separator) + 32 (hash) + 1 (index) + 20 (address) + 32 (chain_id)
                let mut hash_input = Vec::with_capacity(93);
                hash_input.extend_from_slice(HANDLE_HASH_DOMAIN_SEPARATOR.as_bytes());
                hash_input.extend_from_slice(ciphertext_hash.as_slice());
                hash_input.push(index_byte);
                hash_input.extend_from_slice(acl_contract_address.as_slice());
                hash_input.extend_from_slice(&chain_id_bytes);

                let handle_hash = keccak256(&hash_input);

                let mut handle = [0u8; 32];
                handle.copy_from_slice(handle_hash.as_slice());
                handle[21] = index_byte;
                handle[22..30].copy_from_slice(&chain_id_bytes[24..32]);
                handle[30] = encryption_type;
                handle[31] = ciphertext_version;

                Ok(handle)
            })
            .collect::<Result<Vec<[u8; 32]>>>()?;

        Ok(handles)
    }
}

/// A fully encrypted input with its associated data.
///
/// Produced by [`EncryptedInputBuilder::encrypt_and_prove_for`]. Fields are
/// public for direct access but the struct is `#[non_exhaustive]` to prevent
/// external construction.
#[non_exhaustive]
pub struct EncryptedInput {
    /// The ciphertext with ZK proof.
    pub ciphertext: Vec<u8>,
    /// Handles for each encrypted value.
    pub handles: Vec<[u8; 32]>,
    /// Contract address this input is for.
    pub contract_address: Address,
    /// User address making the request.
    pub user_address: Address,
    /// Chain ID.
    pub chain_id: u64,
}

impl EncryptedInput {
    /// Format the handles as hex strings with 0x prefix.
    pub fn handles_as_hex(&self) -> Vec<String> {
        self.handles
            .iter()
            .map(|h| format!("0x{}", hex::encode(h)))
            .collect()
    }

    /// Get the ciphertext as a hex string with 0x prefix.
    pub fn ciphertext_as_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.ciphertext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use std::str::FromStr;

    #[test]
    fn test_handle_format() {
        let mock_ciphertext = vec![1, 2, 3, 4, 5];
        let bit_widths = vec![1, 8, 64];
        let acl_address = Address::from_str("0x1234567890123456789012345678901234567890").unwrap();
        let chain_id = 1;
        let version = 0;

        let handles = EncryptedInputBuilder::compute_handles(
            &mock_ciphertext,
            &bit_widths,
            &acl_address,
            chain_id,
            version,
        )
        .unwrap();

        assert_eq!(handles.len(), 3);

        for (i, handle) in handles.iter().enumerate() {
            assert_eq!(handle[21], i as u8);
            assert_eq!(handle[22..29], [0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(handle[29], 1);

            let expected_type = match bit_widths[i] {
                1 => 0,
                8 => 2,
                64 => 5,
                _ => panic!("Unexpected bit width"),
            };
            assert_eq!(handle[30], expected_type);
            assert_eq!(handle[31], version);
        }
    }

    #[test]
    fn test_compute_handles_rejects_too_many() {
        let mock_ciphertext = vec![1, 2, 3];
        let bit_widths = vec![8; 257]; // one too many
        let acl_address = Address::from_str("0x1234567890123456789012345678901234567890").unwrap();

        let result =
            EncryptedInputBuilder::compute_handles(&mock_ciphertext, &bit_widths, &acl_address, 1, 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Maximum 256"));
    }
}
