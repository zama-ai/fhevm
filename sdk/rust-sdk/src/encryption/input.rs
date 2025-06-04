//! Input processing module for FHEVM SDK
//!
//! This module provides a factory for creating encrypted inputs builders.

use crate::utils::chain_id_to_bytes;
use crate::{FhevmError, Result};
use alloy::primitives::{Address, keccak256};

use crate::encryption::CIPHERTEXT_VERSION;
use crate::encryption::IntoU256;
use crate::encryption::primitives::create_encryption_parameters;
use std::sync::Arc;
use tfhe::{safe_serialization::safe_serialize, zk::ZkComputeLoad};
/// Struct for building encrypted inputs with verification data
/// Only constants are used for the builder factory
pub struct InputBuilderFactory {
    /// ACL contract address for permission management
    acl_contract_address: Address,
    /// Chain ID where the contract lives
    chain_id: u64,
    /// Public key
    public_key: Arc<tfhe::CompactPublicKey>,
    /// CRS for zero-knowledge proof
    crs: Arc<tfhe::zk::CompactPkeCrs>,
}

impl InputBuilderFactory {
    /// Creates a new InputBuilder
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

    /// Create a new EncryptedInputBuilder with the factory's configuration
    pub fn create_builder(&self) -> EncryptedInputBuilder {
        EncryptedInputBuilder::new(
            self.acl_contract_address,
            self.public_key.clone(),
            self.crs.clone(),
            self.chain_id,
        )
    }

    /// Get the chain id for this factory
    pub fn get_chain_id(&self) -> u64 {
        self.chain_id
    }

    pub fn get_acl_contract_address(&self) -> Address {
        self.acl_contract_address
    }
}

/// Represents an encrypted input builder for the fheVM.
///
/// This builder allows adding different types of encrypted values (bool, u8, u16, etc.)
/// and produces a serialized ciphertext with a zero-knowledge proof.
pub struct EncryptedInputBuilder {
    /// ACL contract address for permission management
    pub acl_contract_address: Address,
    /// Chain ID where the contract lives
    pub chain_id: u64,
    /// Public key
    public_key: Arc<tfhe::CompactPublicKey>,
    /// CRS for zero-knowledge proof
    crs: Arc<tfhe::zk::CompactPkeCrs>,
    /// TFHE builder
    builder: tfhe::CompactCiphertextListBuilder,
    /// Bit widths for each value
    bits: Vec<usize>,
}

impl EncryptedInputBuilder {
    /// Creates a new instance of EncryptedInputBuilder
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
            acl_contract_address,
            chain_id,
            public_key,
        }
    }

    /// Checks if adding more bits would exceed the 2048-bit limit
    fn check_limit(&self, added_bits: usize) -> Result<()> {
        let total_bits: usize = self.bits.iter().sum::<usize>() + added_bits;
        if total_bits > 2048 {
            return Err(FhevmError::EncryptionError(
                "Exceeds maximum 2048 bits in a single input ciphertext".to_string(),
            ));
        }
        if self.bits.len() + 1 > 256 {
            return Err(FhevmError::EncryptionError(
                "Exceeds maximum 256 variables in a single input ciphertext".to_string(),
            ));
        }
        Ok(())
    }

    /// Adds a boolean value
    pub fn add_bool(&mut self, value: bool) -> Result<&mut Self> {
        self.check_limit(2)?; // ebool takes 2 encrypted bits
        self.builder.push(value);
        self.bits.push(1);
        Ok(self)
    }

    /// Adds a u8 value
    pub fn add_u8(&mut self, value: u8) -> Result<&mut Self> {
        self.check_limit(8)?;
        self.builder.push(value);
        self.bits.push(8);
        Ok(self)
    }

    /// Adds a u16 value
    pub fn add_u16(&mut self, value: u16) -> Result<&mut Self> {
        self.check_limit(16)?;
        self.builder.push(value);
        self.bits.push(16);
        Ok(self)
    }

    /// Adds a u32 value
    pub fn add_u32(&mut self, value: u32) -> Result<&mut Self> {
        self.check_limit(32)?;
        self.builder.push(value);
        self.bits.push(32);
        Ok(self)
    }

    /// Adds a u64 value
    pub fn add_u64(&mut self, value: u64) -> Result<&mut Self> {
        self.check_limit(64)?;
        self.builder.push(value);
        self.bits.push(64);
        Ok(self)
    }

    /// Adds a u128 value
    pub fn add_u128(&mut self, value: u128) -> Result<&mut Self> {
        self.check_limit(128)?;
        self.builder.push(value);
        self.bits.push(128);
        Ok(self)
    }

    /// Adds an Ethereum address (160 bits)
    pub fn add_address(&mut self, address: &str) -> Result<&mut Self> {
        // First, validate and convert the address to u160
        let address = if let Some(stripped) = address.strip_prefix("0x") {
            stripped
        } else {
            address
        };

        if address.len() != 40 {
            return Err(FhevmError::EncryptionError(
                "Invalid address length".to_string(),
            ));
        }

        let address_bytes = hex::decode(address)
            .map_err(|e| FhevmError::EncryptionError(format!("Invalid hex in address: {}", e)))?;

        let mut padded_bytes = [0u8; 32];

        // Copy the 20 address bytes to the end of the padded array
        // This ensures the address is right-aligned with leading zeros
        // This circumvents the length error "got 20 expect 32 bytes"
        padded_bytes[12..32].copy_from_slice(&address_bytes);

        let mut address_u160 = tfhe::integer::bigint::U256::from(0u128);
        address_u160.copy_from_be_byte_slice(&padded_bytes);

        self.check_limit(160)?;
        self.builder
            .push_with_num_bits(address_u160, 160)
            .map_err(|e| FhevmError::EncryptionError(format!("Failed to push address: {}", e)))?;

        self.bits.push(160);
        Ok(self)
    }

    /// Adds a u256 value
    pub fn add_u256<T: IntoU256>(&mut self, value: T) -> Result<&mut Self> {
        self.check_limit(256)?;

        // This will validate the input size
        let bytes = value.into_u256_bytes()?;

        let mut value_u256 = tfhe::integer::bigint::U256::from(0u128);
        value_u256.copy_from_be_byte_slice(&bytes);

        self.builder.push(value_u256);
        self.bits.push(256);
        Ok(self)
    }

    /// Gets the bit widths for all added values
    pub fn get_bits(&self) -> &[usize] {
        &self.bits
    }

    /// Creates auxiliary data for the zero-knowledge proof
    pub fn create_auxiliary_data(
        &self,
        contract_address: Address,
        user_address: Address,
    ) -> Result<Vec<u8>> {
        let mut aux_data = Vec::with_capacity(92); // 20 + 20 + 20 + 32 bytes

        // Append contract address (20 bytes)
        aux_data.extend_from_slice(contract_address.as_slice());

        // Append user address (20 bytes)
        aux_data.extend_from_slice(user_address.as_slice());

        // Append ACL contract address (20 bytes)
        aux_data.extend_from_slice(self.acl_contract_address.as_slice());

        // Append chain ID (convert to bytes)
        aux_data.extend_from_slice(&chain_id_to_bytes(self.chain_id));

        Ok(aux_data)
    }

    /// Builds the final ciphertext with proof
    fn build_with_proof(&mut self, auxiliary_data: &[u8]) -> Result<Vec<u8>> {
        let metadata = auxiliary_data;

        let proven_compact_list = self
            .builder
            .build_with_proof_packed(&self.crs, metadata, ZkComputeLoad::Verify)
            .map_err(|e| FhevmError::EncryptionError(format!("Failed to build proof: {}", e)))?;

        let mut buffer = Vec::new();
        safe_serialize(&proven_compact_list, &mut buffer, 1 << 20)
            .map_err(|e| FhevmError::EncryptionError(format!("Failed to serialize: {}", e)))?;

        Ok(buffer)
    }

    /// Builds the final ciphertext with proof and generates handles
    pub fn encrypt_for(
        &mut self,
        contract_address: Address,
        user_address: Address,
    ) -> Result<EncryptedInput> {
        // Create auxiliary data by concatenating addresses and chain ID
        let aux_data = self.create_auxiliary_data(contract_address, user_address)?;

        // Build the ciphertext with ZK proof
        let ciphertext = self.build_with_proof(&aux_data)?;
        log::debug!("Ciphertext built successfully: {} bytes", ciphertext.len());

        // Generate handles for each value in the ciphertext
        let bit_widths = self.get_bits();
        log::debug!("Generating handles for {} values", bit_widths.len());
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

    /// Reset this builder to create a new input with the same configuration
    pub fn clear(&mut self) -> &mut Self {
        self.builder = tfhe::ProvenCompactCiphertextList::builder(&self.public_key);
        self.bits.clear();
        self
    }

    /// Compute handles for encrypted values
    ///
    /// This function computes a unique handle for each encrypted value based on:
    /// - The ciphertext hash
    /// - The value's index in the ciphertext
    /// - The ACL contract address
    /// - The chain ID
    /// - The encryption type
    /// - The ciphertext version
    pub fn compute_handles(
        ciphertext: &[u8],
        bit_widths: &[usize],
        acl_contract_address: &Address,
        chain_id: u64,
        ciphertext_version: u8,
    ) -> Result<Vec<[u8; 32]>> {
        // Calculate ciphertext hash using keccak256
        let ciphertext_hash = keccak256(ciphertext);

        // Convert chain_id to bytes (ensuring we only use the last 8 bytes)
        let chain_id_bytes = chain_id_to_bytes(chain_id);

        let handles = bit_widths
            .iter()
            .enumerate()
            .map(|(index, &bit_width)| {
                // Get the encryption type discriminant for this bit width
                let encryption_type = match bit_width {
                    1 => 0,     // ebool
                    8 => 2,     // euint8
                    16 => 3,    // euint16
                    32 => 4,    // euint32
                    64 => 5,    // euint64
                    128 => 6,   // euint128
                    160 => 7,   // eaddress
                    256 => 8,   // euint256
                    512 => 9,   // ebytes64
                    1024 => 10, // ebytes128
                    2048 => 11, // ebytes256
                    _ => {
                        return Err(FhevmError::InvalidParams(format!(
                            "Unsupported bit width: {}",
                            bit_width
                        )));
                    }
                };

                // Create a buffer for the handle using the same scheme as the JavaScript version
                let index_byte = index as u8;
                let mut hash_input = Vec::new();
                hash_input.extend_from_slice(ciphertext_hash.as_slice());
                hash_input.push(index_byte);
                hash_input.extend_from_slice(acl_contract_address.as_slice());
                hash_input.extend_from_slice(&chain_id_bytes);

                let handle_hash = keccak256(&hash_input);

                // Create the final handle by combining hash with metadata
                let mut handle = [0u8; 32];
                handle.copy_from_slice(handle_hash.as_slice());

                // Add the index in position 21
                handle[21] = index_byte;

                // Add the chain_id in positions 22-29 (8 bytes)
                handle[22..30].copy_from_slice(&chain_id_bytes[24..32]);

                // Add the encryption type and version in last two bytes
                handle[30] = encryption_type;
                handle[31] = ciphertext_version;

                Ok(handle)
            })
            .collect::<Result<Vec<[u8; 32]>>>()?;

        Ok(handles)
    }
}

/// Represents a fully encrypted input with its associated data
pub struct EncryptedInput {
    /// The ciphertext with ZK proof
    pub ciphertext: Vec<u8>,
    /// Handles for each encrypted value
    pub handles: Vec<[u8; 32]>,
    /// Contract address this input is for
    pub contract_address: Address,
    /// User address making the request
    pub user_address: Address,
    /// Chain ID
    pub chain_id: u64,
}

impl EncryptedInput {
    /// Format the handles as hex strings with 0x prefix
    pub fn handles_as_hex(&self) -> Vec<String> {
        self.handles
            .iter()
            .map(|h| format!("0x{}", hex::encode(h)))
            .collect()
    }

    /// Get the ciphertext as a hex string with 0x prefix
    pub fn ciphertext_as_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.ciphertext))
    }
}

/// Legacy function to maintain backward compatibility
/// This will create a default keys directory if it doesn't exist
pub fn get_default_encryption_parameters() -> Result<(
    tfhe::CompactPublicKey,
    tfhe::ClientKey,
    tfhe::ServerKey,
    tfhe::zk::CompactPkeCrs,
)> {
    // Use a default path for keys
    let default_path = std::path::PathBuf::from("./keys");
    create_encryption_parameters(&default_path)
}

#[cfg(test)]
mod tests {

    use super::*;
    use tfhe::{
        FheBool, FheUint8, FheUint64, FheUint160,
        integer::bigint::StaticUnsignedBigInt,
        prelude::{CiphertextList, FheDecrypt},
    };

    use alloy::primitives::{Address, U256};
    use std::str::FromStr;

    // Skipping actual tests because they require keys and network setup
    // This is a simple test that verifies the handle generation logic
    #[test]
    fn test_handle_format() {
        // Test values
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

        // Verify we got the right number of handles
        assert_eq!(handles.len(), 3);

        // Verify each handle has the right format
        for (i, handle) in handles.iter().enumerate() {
            // Check index byte
            assert_eq!(handle[21], i as u8);

            // Check chain ID (1 in big-endian should be all zeros followed by 1)
            assert_eq!(handle[22..29], [0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(handle[29], 1);

            // Check encryption type based on bit width
            let expected_type = match bit_widths[i] {
                1 => 0,
                8 => 2,
                64 => 5,
                _ => panic!("Unexpected bit width"),
            };
            assert_eq!(handle[30], expected_type);

            // Check version
            assert_eq!(handle[31], version);
        }
    }

    #[test]
    fn test_input_builder_factory() {
        log::info!("Testing InputBuilderFactory");

        // Load encryption parameters
        let (public_key, client_key, _server_key, crs) =
            get_default_encryption_parameters().unwrap();

        // Set up test addresses
        let contract_address1 =
            Address::from_str("0x1111111111111111111111111111111111111111").unwrap();
        let user_address1 =
            Address::from_str("0x2222222222222222222222222222222222222222").unwrap();
        let contract_address2 =
            Address::from_str("0x3333333333333333333333333333333333333333").unwrap();
        let user_address2 =
            Address::from_str("0x4444444444444444444444444444444444444444").unwrap();
        let acl_address = Address::from_str("0x9999999999999999999999999999999999999999").unwrap();
        let chain_id = 1;

        // Create the factory
        let factory = InputBuilderFactory::new(
            acl_address,
            chain_id,
            public_key.clone().into(),
            crs.clone().into(),
        );

        // Create first builder for first transaction
        let mut builder1 = factory.create_builder();
        builder1.add_bool(true).unwrap();
        builder1.add_u8(45).unwrap();
        let encrypted1 = builder1
            .encrypt_for(contract_address1, user_address1)
            .unwrap();
        let aux_data1 = builder1
            .create_auxiliary_data(contract_address1, user_address1)
            .unwrap();

        // Reuse builder with clear method for second transaction
        builder1.clear();
        builder1.add_bool(false).unwrap();
        builder1.add_u8(125).unwrap();
        let encrypted2 = builder1
            .encrypt_for(contract_address2, user_address2)
            .unwrap();

        let aux_data2 = builder1
            .create_auxiliary_data(contract_address2, user_address2)
            .unwrap();

        // Verify the results have different contract/user addresses
        assert_eq!(encrypted1.contract_address, contract_address1);
        assert_eq!(encrypted1.user_address, user_address1);
        assert_eq!(encrypted2.contract_address, contract_address2);
        assert_eq!(encrypted2.user_address, user_address2);

        // Verify the auxiliary data was correctly generated with different addresses

        // First 20 bytes should match contract address
        assert_eq!(&aux_data1[0..20], contract_address1.as_slice());
        assert_eq!(&aux_data2[0..20], contract_address2.as_slice());

        // Next 20 bytes should match user address
        assert_eq!(&aux_data1[20..40], user_address1.as_slice());
        assert_eq!(&aux_data2[20..40], user_address2.as_slice());

        let verified_value1 = crate::encryption::primitives::verify_expand(
            encrypted1.ciphertext.clone(),
            &public_key,
            _server_key,
            &crs,
            &aux_data1,
        )
        .unwrap();

        let enc_bool: FheBool = verified_value1.get(0).unwrap().unwrap();
        let dec_bool = enc_bool.decrypt(&client_key);
        assert!(dec_bool, "First value should decrypt to true");

        let enc_u8: FheUint8 = verified_value1.get(1).unwrap().unwrap();
        let dec_u8: u8 = enc_u8.decrypt(&client_key);
        assert_eq!(dec_u8, 45u8, "Second value should decrypt to 123");
    }

    #[test]
    fn test_input_builder_with_all_types() {
        log::info!("Testing InputBuilder with all supported types");

        // Load encryption parameters
        let (public_key, _client_key, _server_key, crs) =
            get_default_encryption_parameters().unwrap();

        let addresses = Address::from_str("0x1234567890123456789012345678901234567890").unwrap();
        let chain_id = 1;

        // Create the builder
        let mut input_builder = EncryptedInputBuilder::new(
            addresses,
            public_key.clone().into(),
            crs.clone().into(),
            chain_id,
        );

        // Add all types
        input_builder.add_bool(true).unwrap();
        input_builder.add_u8(123).unwrap();
        input_builder.add_u16(12345).unwrap();
        input_builder.add_u32(1234567).unwrap();
        input_builder.add_u64(1234567890).unwrap();
        input_builder
            .add_u128(123456789012345678901234567890u128)
            .unwrap();
        input_builder
            .add_address("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .unwrap();

        // Add a U256 value
        let u256_value = U256::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        )
        .unwrap(); // 2^256 - 1
        input_builder.add_u256(u256_value).unwrap();

        // Encrypt
        let encrypted_input = input_builder.encrypt_for(addresses, addresses).unwrap();

        // Verify we got the expected number of handles
        assert_eq!(
            encrypted_input.handles.len(),
            8,
            "Should have 8 handles for 8 values"
        );

        // Verify each handle has the correct encryption type
        let expected_types = [0, 2, 3, 4, 5, 6, 7, 8]; // Corresponding to the types we added

        for (i, handle) in encrypted_input.handles.iter().enumerate() {
            assert_eq!(
                handle[30], expected_types[i],
                "Encryption type for value {} is incorrect",
                i
            );
        }

        log::info!("All types encrypted successfully");
    }

    #[test]
    fn test_input_builder_with_real_data() {
        log::info!("Testing InputBuilder with real data");

        // Load encryption parameters
        let (public_key, client_key, server_key, crs) =
            get_default_encryption_parameters().unwrap();
        log::info!("Encryption keys loaded successfully");

        // Set up test addresses
        let contract_address =
            Address::from_str("0x7777777777777777777777777777777777777777").unwrap();
        let user_address = Address::from_str("0x8888888888888888888888888888888888888888").unwrap();
        let acl_address = Address::from_str("0x9999999999999999999999999999999999999999").unwrap();
        let chain_id = 1; // Ethereum mainnet

        // Create the InputBuilder
        let mut input_builder = EncryptedInputBuilder::new(
            acl_address,
            public_key.clone().into(),
            crs.clone().into(),
            chain_id,
        );

        // Add various data types
        input_builder.add_u8(123).unwrap();
        input_builder.add_u64(9999999).unwrap();
        input_builder
            .add_address("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .unwrap();

        // Encrypt the input
        let encrypted_input = input_builder
            .encrypt_for(contract_address, user_address)
            .unwrap();

        // Verify we got the ciphertext and handles
        assert!(
            !encrypted_input.ciphertext.is_empty(),
            "Ciphertext should not be empty"
        );
        assert_eq!(
            encrypted_input.handles.len(),
            3,
            "Should have 3 handles for 3 values"
        );

        log::info!(
            "Encrypted data size: {} bytes",
            encrypted_input.ciphertext.len()
        );

        // Verify the auxiliary data was created correctly
        let aux_data = input_builder
            .create_auxiliary_data(contract_address, user_address)
            .unwrap();
        assert_eq!(
            aux_data.len(),
            92,
            "Auxiliary data should be 92 bytes (20+20+20+32)"
        );

        // Verify the first 20 bytes match the contract address
        assert_eq!(&aux_data[0..20], contract_address.as_slice());

        // Verify the next 20 bytes match the user address
        assert_eq!(&aux_data[20..40], user_address.as_slice());

        // Verify the next 20 bytes match the ACL address
        assert_eq!(&aux_data[40..60], acl_address.as_slice());

        // Verify chain ID bytes - should be 32 bytes with 0x01 at the end
        let chain_id_bytes = chain_id_to_bytes(chain_id);
        assert_eq!(&aux_data[60..92], &chain_id_bytes);

        // Verify the handles are correctly formatted
        for (i, handle) in encrypted_input.handles.iter().enumerate() {
            // Index should be at position 21
            assert_eq!(
                handle[21], i as u8,
                "Handle index at position 21 is incorrect"
            );

            // Chain ID should be in positions 22-29
            let chain_id_slice = &handle[22..30];
            assert_eq!(
                chain_id_slice,
                &chain_id_bytes[24..32],
                "Chain ID in handle is incorrect"
            );

            // Encryption type and version should be in the last two bytes
            // Types should match our input: bool(0), u8(2), u64(5), address(7)
            let expected_type = match i {
                0 => 2, // u8
                1 => 5, // u64
                2 => 7, // address
                _ => panic!("Unexpected index"),
            };
            assert_eq!(handle[30], expected_type, "Encryption type is incorrect");
            assert_eq!(handle[31], CIPHERTEXT_VERSION, "Version is incorrect");
        }

        // Verify we can decrypt the data (using the encryption module's verification)
        let verified_value = crate::encryption::primitives::verify_expand(
            encrypted_input.ciphertext.clone(),
            &public_key,
            server_key,
            &crs,
            &aux_data,
        )
        .unwrap();

        // Verify each decrypted value matches what we encrypted

        let enc_u8: FheUint8 = verified_value.get(0).unwrap().unwrap();
        let dec_u8: u8 = enc_u8.decrypt(&client_key);
        assert_eq!(dec_u8, 123u8, "Second value should decrypt to 123");

        let enc_u64: FheUint64 = verified_value.get(1).unwrap().unwrap();
        let dec_u64: u64 = enc_u64.decrypt(&client_key);
        assert_eq!(dec_u64, 9999999u64, "Third value should decrypt to 9999999");

        let enc_address: FheUint160 = verified_value.get(2).unwrap().unwrap();
        let dec_address: StaticUnsignedBigInt<4> = enc_address.decrypt(&client_key);
        let mut bytes = [0u8; 32];
        dec_address.copy_to_be_byte_slice(&mut bytes);
        let address = hex::encode(&bytes[12..32]);
        assert_eq!(
            &address, "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            "Fourth value should decrypt to the correct address"
        );

        log::info!("All decrypted values match original inputs");

        // Test the utility methods on EncryptedInput
        let handles_hex = encrypted_input.handles_as_hex();
        assert_eq!(handles_hex.len(), 3, "Should have 4 hex handles");
        for handle_hex in &handles_hex {
            assert!(
                handle_hex.starts_with("0x"),
                "Handle hex should start with 0x"
            );
            assert_eq!(
                handle_hex.len(),
                66,
                "Handle hex should be 66 characters (0x + 64 hex chars)"
            );
        }

        let ciphertext_hex = encrypted_input.ciphertext_as_hex();
        assert!(
            ciphertext_hex.starts_with("0x"),
            "Ciphertext hex should start with 0x"
        );

        log::info!("Test passed successfully");
    }
}
