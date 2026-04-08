//! UniFFI bindings for FHEVM client operations.
//!
//! Exposes a flat, FFI-friendly API for Swift, Kotlin, and React Native
//! by wrapping [`fhevm_client_core`].

uniffi::setup_scaffolding!();

mod error;

use error::FhevmError;
use fhevm_client_core::encryption::primitives::EncryptionType;
use fhevm_client_core::utils::validate_address_from_str;
use std::str::FromStr;
use std::sync::Arc;

// ── Types ──────────────────────────────────────────────────────────

/// ML-KEM keypair for user decryption operations.
#[derive(uniffi::Record)]
pub struct Keypair {
    /// Hex-encoded public key (with 0x prefix).
    pub public_key: String,
    /// Hex-encoded private key (with 0x prefix).
    pub private_key: String,
}

/// Result of encrypting values with a ZK proof.
#[derive(uniffi::Record)]
pub struct EncryptedInput {
    /// List of 32-byte handles, one per encrypted value.
    pub handles: Vec<Vec<u8>>,
    /// Serialized ciphertext with ZK proof.
    pub input_proof: Vec<u8>,
}

/// A value to encrypt along with its FHE type.
#[derive(uniffi::Record)]
pub struct TypedValue {
    /// Decimal or hex string representation of the value.
    pub value: String,
    /// FHE type: "ebool", "euint8", "euint16", "euint32", "euint64",
    /// "euint128", "eaddress", or "euint256".
    pub fhe_type: String,
}

/// Configuration for EIP-712 signature generation.
#[derive(uniffi::Record)]
pub struct Eip712Config {
    /// Gateway chain ID used in the EIP-712 domain.
    pub gateway_chain_id: u64,
    /// Hex-encoded address of the decryption verifying contract.
    pub verifying_contract: String,
}

/// EIP-712 structured data ready for signing by a wallet.
#[derive(uniffi::Record)]
pub struct Eip712Result {
    /// JSON-encoded EIP-712 domain.
    pub domain_json: String,
    /// JSON-encoded EIP-712 types.
    pub types_json: String,
    /// JSON-encoded EIP-712 message.
    pub message_json: String,
}

/// A decrypted value returned from the gateway.
#[derive(uniffi::Record)]
pub struct DecryptedValue {
    /// 32-byte ciphertext handle.
    pub handle: Vec<u8>,
    /// Decrypted value as a hex string.
    pub value: String,
    /// FHE type: "ebool", "euint8", etc.
    pub fhe_type: String,
}

/// A handle paired with the contract address that owns it.
#[derive(uniffi::Record)]
pub struct HandleContractPair {
    /// 32-byte ciphertext handle.
    pub handle: Vec<u8>,
    /// Hex-encoded contract address (with 0x prefix).
    pub contract_address: String,
}

// ── Functions ──────────────────────────────────────────────────────

/// Generate an ML-KEM keypair for user decryption operations.
///
/// Returns hex-encoded public and private keys with 0x prefix.
#[uniffi::export]
pub fn generate_keypair() -> Result<Keypair, FhevmError> {
    let kp = fhevm_client_core::signature::generate_keypair()?;
    Ok(Keypair {
        public_key: kp.public_key().to_string(),
        private_key: kp.private_key().to_string(),
    })
}

/// Build EIP-712 structured data for a user decrypt permission.
///
/// Returns JSON-encoded domain, types, and message suitable for signing
/// by a mobile wallet (e.g. via WalletConnect `eth_signTypedData_v4`).
#[uniffi::export]
pub fn create_eip712(
    config: Eip712Config,
    public_key: String,
    contract_addresses: Vec<String>,
    start_timestamp: u64,
    duration_days: u64,
) -> Result<Eip712Result, FhevmError> {
    let verifying_contract = validate_address_from_str(&config.verifying_contract)?;

    let core_config = fhevm_client_core::signature::eip712::Eip712Config {
        gateway_chain_id: config.gateway_chain_id,
        verifying_contract,
        // Use gateway_chain_id as contracts_chain_id — the caller controls
        // this via the config, and for mobile the distinction is handled
        // at the JS/Swift/Kotlin layer.
        contracts_chain_id: config.gateway_chain_id,
    };

    let mut builder =
        fhevm_client_core::signature::eip712::Eip712SignatureBuilder::new(core_config)
            .with_public_key(&public_key)
            .with_validity_period(start_timestamp, duration_days);

    for addr in &contract_addresses {
        builder = builder.with_contract(addr.as_str())?;
    }

    // Domain name must match client-core's eip712 domain ("Decryption")
    let domain = serde_json::json!({
        "name": "Decryption",
        "version": "1",
        "chainId": config.gateway_chain_id,
        "verifyingContract": verifying_contract.to_string()
    });

    let types = serde_json::json!({
        "EIP712Domain": [
            {"name": "name", "type": "string"},
            {"name": "version", "type": "string"},
            {"name": "chainId", "type": "uint256"},
            {"name": "verifyingContract", "type": "address"}
        ],
        "UserDecryptRequestVerification": [
            {"name": "publicKey", "type": "bytes"},
            {"name": "contractAddresses", "type": "address[]"},
            {"name": "startTimestamp", "type": "uint256"},
            {"name": "durationDays", "type": "uint256"},
            {"name": "extraData", "type": "bytes"}
        ]
    });

    let message = serde_json::json!({
        "publicKey": public_key,
        "contractAddresses": contract_addresses,
        "startTimestamp": start_timestamp.to_string(),
        "durationDays": duration_days.to_string(),
        "extraData": "0x"
    });

    Ok(Eip712Result {
        domain_json: serde_json::to_string(&domain).map_err(|e| FhevmError::InternalError {
            reason: format!("Failed to serialize domain: {e}"),
        })?,
        types_json: serde_json::to_string(&types).map_err(|e| FhevmError::InternalError {
            reason: format!("Failed to serialize types: {e}"),
        })?,
        message_json: serde_json::to_string(&message).map_err(|e| FhevmError::InternalError {
            reason: format!("Failed to serialize message: {e}"),
        })?,
    })
}

/// Encrypt values with a ZK proof.
///
/// `public_key` and `crs` are serialized `tfhe` key material (binary).
/// Values are added in order; each gets a 32-byte handle in the result.
#[uniffi::export]
pub fn encrypt(
    public_key: Vec<u8>,
    crs: Vec<u8>,
    values: Vec<TypedValue>,
    contract_address: String,
    user_address: String,
    acl_address: String,
    chain_id: u64,
) -> Result<EncryptedInput, FhevmError> {
    let pk: tfhe::CompactPublicKey = tfhe::safe_serialization::safe_deserialize_conformant(
        public_key.as_slice(),
        1 << 30,
        &tfhe::shortint::parameters::PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128,
    )
    .map_err(|e| FhevmError::EncryptionError {
        reason: format!("Failed to deserialize public key: {e}"),
    })?;

    let crs_obj: tfhe::zk::CompactPkeCrs =
        tfhe::safe_serialization::safe_deserialize(crs.as_slice(), 1 << 30).map_err(|e| {
            FhevmError::EncryptionError {
                reason: format!("Failed to deserialize CRS: {e}"),
            }
        })?;

    let acl_addr = validate_address_from_str(&acl_address)?;
    let contract_addr = validate_address_from_str(&contract_address)?;
    let user_addr = validate_address_from_str(&user_address)?;

    let mut builder = fhevm_client_core::EncryptedInputBuilder::new(
        acl_addr,
        Arc::new(pk),
        Arc::new(crs_obj),
        chain_id,
    );

    for tv in &values {
        add_typed_value(&mut builder, tv)?;
    }

    let result = builder.encrypt_and_prove_for(contract_addr, user_addr)?;

    Ok(EncryptedInput {
        handles: result.handles.iter().map(|h| h.to_vec()).collect(),
        input_proof: result.ciphertext,
    })
}

/// Build ABI-encoded calldata for a user decrypt request.
///
/// The returned bytes are ready to be submitted as a transaction to the
/// gateway decryption contract.
#[uniffi::export]
pub fn build_user_decrypt_calldata(
    handles: Vec<Vec<u8>>,
    contract_addresses: Vec<String>,
    user_address: String,
    signature: Vec<u8>,
    public_key: Vec<u8>,
    start_timestamp: u64,
    duration_days: u64,
    chain_id: u64,
) -> Result<Vec<u8>, FhevmError> {
    let addresses: Vec<alloy::primitives::Address> = contract_addresses
        .iter()
        .map(|a| Ok(validate_address_from_str(a)?))
        .collect::<Result<Vec<_>, FhevmError>>()?;

    let calldata = fhevm_client_core::decryption::user::UserDecryptRequestBuilder::new()
        .with_handles_from_bytes(&handles, &addresses)?
        .with_user_address_from_str(&user_address)?
        .with_signature_from_hex(&format!("0x{}", hex::encode(&signature)))?
        .with_public_key_from_hex(&format!("0x{}", hex::encode(&public_key)))?
        .with_validity(start_timestamp, duration_days)?
        .with_contracts_chain_id(chain_id)
        .build_and_generate_calldata()?;

    Ok(calldata)
}

/// Process a user decrypt response from the gateway.
///
/// Decrypts the response using the ML-KEM private key and verifies
/// KMS signatures. Returns the decrypted plaintext values.
#[uniffi::export]
pub fn process_user_decrypt_response(
    response_json: String,
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    kms_signer_addresses: Vec<String>,
    user_address: String,
    gateway_chain_id: u64,
    verifying_contract: String,
    handle_contract_pairs: Vec<HandleContractPair>,
    signature: String,
) -> Result<Vec<DecryptedValue>, FhevmError> {
    let pairs: Vec<fhevm_gateway_bindings::decryption::Decryption::CtHandleContractPair> =
        handle_contract_pairs
            .iter()
            .map(|p| {
                let handle = alloy::primitives::U256::from_be_slice(&p.handle);
                let contract = validate_address_from_str(&p.contract_address)?;
                Ok(
                    fhevm_gateway_bindings::decryption::Decryption::CtHandleContractPair {
                        ctHandle: handle.into(),
                        contractAddress: contract,
                    },
                )
            })
            .collect::<Result<Vec<_>, FhevmError>>()?;

    let results = fhevm_client_core::decryption::user::process_user_decryption_response()
        .with_kms_signers(kms_signer_addresses)
        .with_user_address(&user_address)
        .with_gateway_chain_id(gateway_chain_id)
        .with_verifying_contract_address(&verifying_contract)
        .with_signature(&signature)
        .with_public_key(&format!("0x{}", hex::encode(&public_key)))
        .with_private_key(&format!("0x{}", hex::encode(&private_key)))
        .with_handle_contract_pairs(pairs)
        .with_json_response(&response_json)
        .process()?;

    let decrypted: Vec<DecryptedValue> = results
        .iter()
        .enumerate()
        .map(|(i, pt)| {
            let fhe_type = EncryptionType::from_discriminant(pt.fhe_type as u8)
                .map(|t| t.as_str())
                .unwrap_or("unknown");
            DecryptedValue {
                handle: handle_contract_pairs
                    .get(i)
                    .map(|p| p.handle.clone())
                    .unwrap_or_default(),
                value: hex::encode(&pt.bytes),
                fhe_type: fhe_type.to_string(),
            }
        })
        .collect();

    Ok(decrypted)
}

// ── Helpers ────────────────────────────────────────────────────────

/// Parse an FHE type string and add the corresponding value to the builder.
fn add_typed_value(
    builder: &mut fhevm_client_core::EncryptedInputBuilder,
    tv: &TypedValue,
) -> Result<(), FhevmError> {
    // Validate fhe_type string via EncryptionType (centralized mapping)
    let enc_type = EncryptionType::from_str(&tv.fhe_type)?;

    match enc_type {
        EncryptionType::Bit1 => {
            let v: bool = match tv.value.as_str() {
                "true" | "1" => true,
                "false" | "0" => false,
                other => {
                    return Err(FhevmError::InvalidInput {
                        reason: format!(
                            "Invalid boolean value: '{other}'. Use 'true'/'false' or '1'/'0'."
                        ),
                    });
                }
            };
            builder.add_bool(v)?;
        }
        EncryptionType::Bit8 => {
            let v: u8 = tv.value.parse().map_err(|e| FhevmError::InvalidInput {
                reason: format!("Invalid u8 value '{}': {e}", tv.value),
            })?;
            builder.add_u8(v)?;
        }
        EncryptionType::Bit16 => {
            let v: u16 = tv.value.parse().map_err(|e| FhevmError::InvalidInput {
                reason: format!("Invalid u16 value '{}': {e}", tv.value),
            })?;
            builder.add_u16(v)?;
        }
        EncryptionType::Bit32 => {
            let v: u32 = tv.value.parse().map_err(|e| FhevmError::InvalidInput {
                reason: format!("Invalid u32 value '{}': {e}", tv.value),
            })?;
            builder.add_u32(v)?;
        }
        EncryptionType::Bit64 => {
            let v: u64 = tv.value.parse().map_err(|e| FhevmError::InvalidInput {
                reason: format!("Invalid u64 value '{}': {e}", tv.value),
            })?;
            builder.add_u64(v)?;
        }
        EncryptionType::Bit128 => {
            let v: u128 = tv.value.parse().map_err(|e| FhevmError::InvalidInput {
                reason: format!("Invalid u128 value '{}': {e}", tv.value),
            })?;
            builder.add_u128(v)?;
        }
        EncryptionType::Bit160 => {
            builder.add_address(&tv.value)?;
        }
        EncryptionType::Bit256 => {
            let u256 = alloy::primitives::U256::from_str(&tv.value).map_err(|e| {
                FhevmError::InvalidInput {
                    reason: format!("Invalid u256 value '{}': {e}", tv.value),
                }
            })?;
            builder.add_u256(u256)?;
        }
    }
    Ok(())
}
