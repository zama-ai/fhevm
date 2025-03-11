use crate::blockchain::ethereum::bindings::DecyptionManager::{
    self, publicDecryptionResponseCall, PublicDecryptionRequest, PublicDecryptionResponse,
    UserDecryptionRequest,
};
use crate::blockchain::ethereum::bindings::IDecryptionManager::{
    CtHandleContractPair, RequestValidity,
};
use crate::blockchain::ethereum_host_l1_handlers::DecryptionRequestData;
use crate::core::errors::EventProcessingError;
use crate::core::event::UserDecryptRequest;
use alloy::primitives::{keccak256, Address, Bytes, Uint, U256};
use alloy::signers::SignerSync;
use rusqlite::{Connection, Result};
use serde::Serialize;
use tracing::{debug, error, info};

use alloy::{
    sol,
    sol_types::SolCall,
    sol_types::{eip712_domain, SolStruct},
};
use std::str::FromStr;

use alloy::{dyn_abi::DynSolValue, hex, signers::local::PrivateKeySigner};

sol! {
    #[allow(missing_docs)]
    #[derive(Serialize)]
    #[derive(Debug)]
    struct PublicDecryptionResult {
        uint256[] handlesList;
        bytes decryptedResult;
    }
}

pub struct ComputeCalldata;

impl ComputeCalldata {
    pub fn callback_req(
        req: &DecryptionRequestData,
        public_decryption_response: PublicDecryptionResponse,
        _signature_number: u8,
    ) -> Result<Bytes, EventProcessingError> {
        let mut calldata = Vec::new();

        calldata.extend_from_slice(&req.callback_selector.0);

        let request_id_bytes = req.host_l1_request_id.to_be_bytes::<32>();
        calldata.extend_from_slice(&request_id_bytes);

        calldata.extend_from_slice(&public_decryption_response.decryptedResult);

        // Add signatures array length (32 bytes)
        let sig_count = public_decryption_response.signatures.len();
        let sig_length_bytes = U256::from(sig_count).to_be_bytes::<32>();
        calldata.extend_from_slice(&sig_length_bytes);

        // Add offset to signatures data (32 bytes)
        let mut offset_bytes = [0u8; 32];
        offset_bytes[31] = 32u8; // offset is always 32 for the first element
        calldata.extend_from_slice(&offset_bytes);

        println!(
            "public_decryption_response {:?}",
            &public_decryption_response.signatures
        );
        // For each signature:
        for signature in &public_decryption_response.signatures {
            // Add length of signature (32 bytes)
            let sig_size = signature.len(); // typically 65 (0x41)
            let sig_size_bytes = U256::from(sig_size).to_be_bytes::<32>();
            calldata.extend_from_slice(&sig_size_bytes);
            calldata.extend_from_slice(signature);
            let padding_length = (32 - (signature.len() % 32)) % 32;
            if padding_length > 0 {
                calldata.extend_from_slice(&vec![0u8; padding_length]);
            }
        }

        Ok(Bytes::from(calldata))
    }

    pub fn decryption_req(handles: Vec<Uint<256, 4>>) -> Result<Bytes, EventProcessingError> {
        let selector = &keccak256("publicDecryptionRequest(uint256[])")[..4];
        // Encode the parameters properly following ABI encoding rules
        let mut calldata = Vec::new();

        // 1. Add function selector
        calldata.extend_from_slice(selector);

        // 2. Add offset to start of array (32 bytes from start of parameters)
        calldata.extend_from_slice(&U256::from(32).to_be_bytes::<32>());

        // 3. Add array length
        calldata.extend_from_slice(&U256::from(handles.len()).to_be_bytes::<32>());

        // 4. Add array elements
        for handle in handles {
            calldata.extend_from_slice(&handle.to_be_bytes::<32>());
        }

        Ok(Bytes::from(calldata))
    }

    pub fn user_decryption_req(
        ct_handles: Vec<Bytes>,
        contract_chain_id: U256,
        contract_address: Address,
        user_address: Address,
        public_key: Bytes,
        signature: Bytes,
    ) -> Result<Bytes, EventProcessingError> {
        // Convert each handle to Uint<256, 4> and create a pair with the same contract address
        let ct_handle_contract_pairs: Vec<CtHandleContractPair> = ct_handles
            .into_iter()
            .map(|handle_bytes| {
                // Convert the bytes to a proper U256 handle
                // We assume the handle bytes are already in proper format
                // Typically handles are 32 bytes long representing a uint256
                let mut handle_array = [0u8; 32];
                let copy_len = std::cmp::min(handle_bytes.len(), 32);
                handle_array[32 - copy_len..].copy_from_slice(&handle_bytes[..copy_len]);

                let handle = Uint::<256, 4>::from_be_bytes(handle_array);

                // TODO: we receive only one contract address, so for now
                //  we use the same contract address for all in the contract_addresses array

                // Create a pair with this handle and the provided contract address
                CtHandleContractPair {
                    ctHandle: handle,
                    contractAddress: contract_address,
                }
            })
            .collect();

        // Create validity struct with current timestamp and a default duration (e.g., 1 day)
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let validity = RequestValidity {
            startTimestamp: U256::from(current_timestamp),
            durationDays: U256::from(1), // Default to 1 day validity
        };

        // TODO: we receive only one contract address, so for now
        //  we use the same contract address for all in the contract_addresses array
        let contract_addresses = vec![contract_address];

        // Create the userDecryptionRequest call
        let call = DecyptionManager::userDecryptionRequestCall::new((
            ct_handle_contract_pairs,
            validity,
            contract_chain_id,
            contract_addresses,
            user_address,
            public_key,
            signature,
        ));

        // Encode the call to get the calldata
        let calldata = DecyptionManager::userDecryptionRequestCall::abi_encode(&call);

        info!(
            "UserDecryptionRequest calldata: 0x{}",
            hex::encode(&calldata)
        );

        Ok(Bytes::from(calldata))
    }

    /// Computes calldata for verifyProofRequest function
    ///
    /// # Arguments
    /// * `contract_chain_id` - Chain ID where the contract is deployed
    /// * `contract_address` - Address of the contract
    /// * `user_address` - Address of the user
    /// * `ciphertext_with_zkproof` - Combined ciphertext and ZK proof data
    pub fn verify_proof_req(
        contract_chain_id: U256,
        contract_address: Address,
        user_address: Address,
        ciphertext_with_zkproof: Bytes,
    ) -> Result<Bytes, EventProcessingError> {
        let mut calldata = Vec::new();

        // Function selector for verifyProofRequest(uint256,address,address,bytes)
        let selector = &keccak256("verifyProofRequest(uint256,address,address,bytes)")[..4];
        info!(
            "Selector for VerifyProofRequest: 0x{}",
            hex::encode(selector)
        );
        calldata.extend_from_slice(selector);

        // Encode contract_chain_id (uint256)
        calldata.extend_from_slice(&contract_chain_id.to_be_bytes::<32>());

        // Encode contract_address (address) - pad to 32 bytes
        let mut padded_contract = [0u8; 32];
        padded_contract[12..].copy_from_slice(contract_address.as_ref());
        calldata.extend_from_slice(&padded_contract);

        // Encode user_address (address) - pad to 32 bytes
        let mut padded_user = [0u8; 32];
        padded_user[12..].copy_from_slice(user_address.as_ref());
        calldata.extend_from_slice(&padded_user);

        // Encode dynamic bytes offset (pointing to ciphertext_with_zkproof)
        // Should be 128 (4 + 3*32) for selector + 3 fixed params
        calldata.extend_from_slice(&U256::from(128).to_be_bytes::<32>());

        // Encode ciphertext_with_zkproof length
        calldata.extend_from_slice(&U256::from(ciphertext_with_zkproof.len()).to_be_bytes::<32>());

        // Encode ciphertext_with_zkproof data
        calldata.extend_from_slice(&ciphertext_with_zkproof);

        // Pad to 32 byte boundary if needed
        if ciphertext_with_zkproof.len() % 32 != 0 {
            let padding = vec![0u8; 32 - (ciphertext_with_zkproof.len() % 32)];
            calldata.extend_from_slice(&padding);
        }

        // Debug log the calldata
        debug!(
            "Calldata: \n\
             Selector: 0x{}\n\
             ChainID: 0x{}\n\
             Contract: 0x{}\n\
             User: 0x{}\n\
             Offset: 0x{}\n\
             Length: 0x{}\n\
             Data: 0x{}",
            hex::encode(&calldata[..4]),
            hex::encode(&calldata[4..36]),
            hex::encode(&calldata[36..68]),
            hex::encode(&calldata[68..100]),
            hex::encode(&calldata[100..132]),
            hex::encode(&calldata[132..164]),
            hex::encode(&calldata[164..])
        );

        Ok(Bytes::from(calldata))
    }

    /// Computes calldata for verifyProofResponse function with multiple signatures
    ///
    /// # Arguments
    /// * `zkpok_id` - ID of the proof being verified
    /// * `handles` - Array of 32-byte handles
    /// * `signatures` - Vector of signatures to be concatenated
    ///
    /// # Returns
    /// * `Ok(Bytes)` - The encoded calldata
    /// * `Err(EventProcessingError)` - If encoding fails
    pub fn verify_proof_response(
        zkpok_id: U256,
        handles: Vec<[u8; 32]>,
        signature: Vec<u8>,
    ) -> Result<Bytes, EventProcessingError> {
        let mut calldata = Vec::new();

        // 1. Function selector for verifyProofResponse(uint256,bytes32[],bytes)
        let selector = &keccak256("verifyProofResponse(uint256,bytes32[],bytes)")[..4];
        calldata.extend_from_slice(selector);

        // 2. zkpok_id (uint256)
        calldata.extend_from_slice(&zkpok_id.to_be_bytes::<32>());

        // 3. Offset to handles array (0x60)
        calldata.extend_from_slice(&U256::from(0x60).to_be_bytes::<32>());

        // 4. Offset to signature (0xc0 = 192 for 2 handles)
        let sig_offset = 0xc0u32;
        calldata.extend_from_slice(&U256::from(sig_offset).to_be_bytes::<32>());

        // 5. Length of handles array
        calldata.extend_from_slice(&U256::from(handles.len()).to_be_bytes::<32>());

        // 6. Handles data
        for handle in &handles {
            calldata.extend_from_slice(handle);
        }

        // 7. Single signature encoding
        // Length prefix for signature (65 bytes)
        let mut sig_length = [0u8; 32];
        sig_length[31] = 0x41; // 65 in hex
        calldata.extend_from_slice(&sig_length);

        calldata.extend_from_slice(&signature);

        // No padding needed for single signature as it's not an array

        // Debug logging
        debug!("Detailed calldata breakdown:");
        debug!("Selector (4 bytes): 0x{}", hex::encode(&calldata[..4]));
        debug!("zkpok_id (32 bytes): 0x{}", hex::encode(&calldata[4..36]));
        debug!(
            "handles_offset (32 bytes): 0x{}",
            hex::encode(&calldata[36..68])
        );
        debug!(
            "signature_offset (32 bytes): 0x{}",
            hex::encode(&calldata[68..100])
        );
        debug!(
            "handles_length (32 bytes): 0x{}",
            hex::encode(&calldata[100..132])
        );

        let handles_end = 132 + handles.len() * 32;
        debug!(
            "handles_data ({} bytes): 0x{}",
            handles.len() * 32,
            hex::encode(&calldata[132..handles_end])
        );

        debug!(
            "signature_length (32 bytes): 0x{}",
            hex::encode(&calldata[handles_end..handles_end + 32])
        );

        let sig_start = handles_end + 32;
        debug!(
            "signature_data (65 bytes): 0x{}",
            hex::encode(&calldata[sig_start..])
        );

        debug!("Total size: {} bytes", calldata.len());
        debug!("Raw calldata: 0x{}", hex::encode(&calldata));

        Ok(Bytes::from(calldata))
    }

    pub fn user_decryption_response(
        req: UserDecryptionRequest,
    ) -> Result<Bytes, EventProcessingError> {
        // Extract user_decryption_id directly from the request
        let user_decryption_id = req.userDecryptionId;

        // Create dummy values for the other parameters
        // In a real implementation, these would be generated from the actual decryption process
        let dummy_reencrypted_share = Bytes::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        // Create a dummy signature (65 bytes is typical for an Ethereum signature)
        let dummy_signature = Bytes::from(vec![42u8; 65]);

        // Create the userDecryptionResponse call using Alloy's type-safe interface
        let call = DecyptionManager::userDecryptionResponseCall::new((
            user_decryption_id,
            dummy_reencrypted_share,
            dummy_signature,
        ));

        // Encode the call to get the calldata
        let calldata = DecyptionManager::userDecryptionResponseCall::abi_encode(&call);

        info!(
            "UserDecryptionResponse calldata for user_decryption_id {}: 0x{}",
            user_decryption_id,
            hex::encode(&calldata)
        );

        Ok(Bytes::from(calldata))
    }

    pub fn decryption_response(
        req: PublicDecryptionRequest,
        decryption_manager_address: Address,
    ) -> Result<Bytes, EventProcessingError> {
        // 1. Compute decryptedResult bytes array
        let mut results: Vec<DynSolValue> = Vec::new();
        results.push(DynSolValue::Uint(U256::from(42), 256)); // requestID placeholder

        for sns_ct_material in req.snsCtMaterials.clone() {
            let handle: [u8; 32] = sns_ct_material.ctHandle.to_be_bytes();

            // Using a hardcoded value for now

            let clear_text = match get_clear_text("hardhat/contracts/sql.db", &handle) {
                Ok(Some(text)) => text,
                Ok(None) => {
                    error!("No value found for this handle");
                    "65".to_string()
                }
                Err(_) => {
                    error!("Error accessing database");
                    "65".to_string()
                }
            };

            match handle[30] {
                9 => {
                    // Parse the string to Uint, handle potential parsing errors
                    let num: Uint<512, 8> = clear_text.parse().map_err(|e| {
                        EventProcessingError::ParseError(format!(
                            "Failed to parse to Uint<512,8>: {}",
                            e
                        ))
                    })?;

                    let bytes: [u8; 64] = num.to_be_bytes();
                    let bytes_vec = bytes.to_vec();
                    results.push(DynSolValue::Bytes(bytes_vec));
                }
                10 => {
                    let num: Uint<1024, 16> = clear_text.parse().map_err(|e| {
                        EventProcessingError::ParseError(format!(
                            "Failed to parse to Uint<1024,16>: {}",
                            e
                        ))
                    })?;

                    let bytes: [u8; 128] = num.to_be_bytes();
                    let bytes_vec = bytes.to_vec();
                    results.push(DynSolValue::Bytes(bytes_vec));
                }
                11 => {
                    let num: Uint<2048, 32> = clear_text.parse().map_err(|e| {
                        EventProcessingError::ParseError(format!(
                            "Failed to parse to Uint<2048,32>: {}",
                            e
                        ))
                    })?;

                    let bytes: [u8; 256] = num.to_be_bytes();
                    let bytes_vec = bytes.to_vec();
                    results.push(DynSolValue::Bytes(bytes_vec));
                }
                _ => {
                    // Parse the string to U256, handle potential parsing errors
                    let value = U256::from_str(&clear_text).map_err(|e| {
                        EventProcessingError::ParseError(format!("Failed to parse to U256: {}", e))
                    })?;

                    results.push(DynSolValue::Uint(value, 256));
                }
            }
        }

        results.push(DynSolValue::Array(vec![])); // signatures placeholder

        let data = DynSolValue::Tuple(results).abi_encode_params();
        let decrypted_result = data[32..data.len() - 32].to_vec(); // remove placeholder corresponding to requestID and signatures

        println!(
            "decryptedResult : 0x{}",
            hex::encode(decrypted_result.clone())
        );

        // 2. EIP712 signature of KMS signer
        let signer = PrivateKeySigner::from_str(
            "30d45b1c5a771e20d0ec15097c3b6ac7153bc1992bc78c42af37725dd93f096a",
        )
        .map_err(|e| {
            EventProcessingError::SigningError(format!(
                "Failed to create private key signer: {}",
                e
            ))
        })?;

        let domain = eip712_domain! {
            name: "DecryptionManager",
            version: "1",
            chain_id: 654321,
            verifying_contract: decryption_manager_address,
        };

        println!("{:?}", domain);

        let mut ct_handles: Vec<U256> = Vec::new();
        for sns_ct_material in req.snsCtMaterials {
            ct_handles.push(sns_ct_material.ctHandle);
        }
        let public_decryption_result = PublicDecryptionResult {
            handlesList: ct_handles,
            decryptedResult: decrypted_result.clone().into(),
        };

        println!("public_decryption_result {:?}", public_decryption_result);

        let hash = public_decryption_result.eip712_signing_hash(&domain);

        // Replace unwrap with proper error handling
        let signature = signer.sign_hash_sync(&hash).map_err(|e| {
            EventProcessingError::SigningError(format!("Failed to sign hash: {}", e))
        })?;

        info!("Signature: 0x{}", hex::encode(signature.as_bytes()));

        let res_data_gateway = publicDecryptionResponseCall::new((
            req.publicDecryptionId,
            decrypted_result.into(),
            signature.as_bytes().into(),
        ));

        let calldata_bytes = publicDecryptionResponseCall::abi_encode(&res_data_gateway);

        Ok(alloy::primitives::Bytes::from(calldata_bytes))
    }
}

fn get_clear_text(db_path: &str, handle: &[u8]) -> Result<Option<String>> {
    let conn = Connection::open(db_path)?;

    let hex_string = format!("0x{}", hex::encode(handle));
    let mut stmt = conn.prepare("SELECT clearText FROM ciphertexts WHERE handle = ?")?;
    let result = stmt.query_row([hex_string], |row| row.get::<_, String>(0));

    match result {
        Ok(text) => Ok(Some(text)),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("No rows found for this handle");
            Ok(None)
        }
        Err(e) => {
            println!("Error occurred: {}", e);
            Err(e)
        }
    }
}
