use crate::blockchain::ethereum::bindings::DecyptionManager::{
    self, publicDecryptionResponseCall, PublicDecryptionRequest, PublicDecryptionResponse,
    UserDecryptionRequest,
};

use crate::blockchain::ethereum::bindings::DecyptionManager::CtHandleContractPair;
use crate::blockchain::ethereum::bindings::IDecryptionManager::RequestValidity;
use crate::blockchain::ethereum::bindings::ZKPoKManager;
use crate::blockchain::public_decrypt_handler::DecryptionRequestData;
use crate::core::errors::EventProcessingError;
use crate::core::event::UserDecryptRequest;
use alloy::primitives::{Address, Bytes, FixedBytes, Uint, U256};
use rusqlite::{Connection, Result};
use serde::Serialize;
use tracing::{debug, error, info};

use alloy::signers::SignerSync;
use alloy::{dyn_abi::DynSolValue, hex, signers::local::PrivateKeySigner};
use alloy::{
    sol,
    sol_types::SolCall,
    sol_types::{eip712_domain, SolStruct},
};
use std::str::FromStr;

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
    ) -> Result<Bytes, EventProcessingError> {
        let mut calldata = Vec::new();

        calldata.extend_from_slice(&req.callback_selector.0);

        let request_id_bytes = req.host_l1_request_id.to_be_bytes::<32>();
        calldata.extend_from_slice(&request_id_bytes);

        calldata.extend_from_slice(&public_decryption_response.decryptedResult);

        let signatures_values = &public_decryption_response
            .signatures
            .iter()
            .map(|sig| DynSolValue::Bytes(sig.to_vec()))
            .collect::<Vec<_>>();
        let array_value = DynSolValue::Array(signatures_values.to_vec());
        let encoded_signatures = array_value.abi_encode();
        calldata.extend_from_slice(&encoded_signatures[32..]); // Skip the first 32 bytes (offset part) of the encoded signatures

        println!(
            "public_decryption_response {:?}",
            &public_decryption_response.signatures
        );

        Ok(Bytes::from(calldata))
    }

    pub fn public_decryption_req(
        handles: Vec<FixedBytes<32>>,
    ) -> Result<Bytes, EventProcessingError> {
        let calldata = DecyptionManager::publicDecryptionRequestCall::new((handles,)).abi_encode();

        info!(
            "publicDecryptionRequest calldata: 0x{}",
            hex::encode(&calldata)
        );

        Ok(Bytes::from(calldata))
    }

    pub fn user_decryption_req(
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<Bytes, EventProcessingError> {
        let ct_handle_contract_pairs = user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(|d| CtHandleContractPair {
                ctHandle: d.ct_handle.into(),
                contractAddress: d.contract_address,
            })
            .collect::<Vec<_>>();

        let validity = RequestValidity {
            startTimestamp: user_decrypt_request.request_validity.start_timestamp,
            durationDays: user_decrypt_request.request_validity.duration_days,
        };

        // Create the userDecryptionRequest call
        let call = DecyptionManager::userDecryptionRequestCall::new((
            ct_handle_contract_pairs,
            validity,
            U256::from(user_decrypt_request.contracts_chain_id),
            user_decrypt_request.contract_addresses,
            user_decrypt_request.user_address,
            user_decrypt_request.public_key,
            user_decrypt_request.signature,
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
        contract_chain_id: u64,
        contract_address: Address,
        user_address: Address,
        ciphertext_with_zkproof: Bytes,
    ) -> Result<Bytes, EventProcessingError> {
        let request_call = ZKPoKManager::verifyProofRequestCall {
            contractChainId: U256::from(contract_chain_id),
            contractAddress: contract_address,
            userAddress: user_address,
            ciphertextWithZKProof: ciphertext_with_zkproof,
        };
        let calldata = request_call.abi_encode();
        Ok(Bytes::from(calldata))
    }

    /// Computes calldata for verifyProofResponse function with multiple signatures
    /// Used in gateway_processors_mock
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
        let calldata = ZKPoKManager::verifyProofResponseCall::new((
            zkpok_id,
            handles
                .into_iter()
                .map(alloy::primitives::FixedBytes::<32>::from)
                .collect(),
            signature.into(),
        ))
        .abi_encode();

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
            let handle: [u8; 32] = sns_ct_material.ctHandle.into();

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
                    // Parse the string to Uint, handle  potential parsing errors
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
            ct_handles.push(sns_ct_material.ctHandle.into());
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
