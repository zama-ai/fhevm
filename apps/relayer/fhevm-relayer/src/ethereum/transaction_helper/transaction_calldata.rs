use crate::errors::EventProcessingError;
use crate::ethereum_host_l1_handlers::DecryptionRequestData;
use alloy::primitives::{keccak256, Address, Bytes, Uint, U256};
use tracing::{debug, info};

pub struct ComputeCalldata;

impl ComputeCalldata {
    pub fn callback_req(
        req: &DecryptionRequestData,
        decrypted_value: U256,
        signature_number: u8,
    ) -> Result<Bytes, EventProcessingError> {
        let mut calldata = Vec::new();

        // 2. Encode main parameters following AbiCoder format:
        // ['uint256', 'uint64', 'bytes[]']
        // [requestID, decrypted_value, signatures]

        // 1. Selector
        calldata.extend_from_slice(req.callback_selector.as_ref());

        // 2. RequestID
        calldata.extend_from_slice(&req.host_l1_request_id.to_be_bytes::<32>());

        // 3. Value
        calldata.extend_from_slice(&decrypted_value.to_be_bytes::<32>());

        // 4. Offset to array (0x60 = 96)
        let mut offset_bytes = [0u8; 32];
        offset_bytes[31] = 0x60;
        calldata.extend_from_slice(&offset_bytes);

        // 5. Array length (4 signatures)
        let mut length_bytes = [0u8; 32];
        length_bytes[31] = signature_number;
        calldata.extend_from_slice(&length_bytes);

        // 6. Offsets to each signature
        // First signature starts at 0x80 (128)
        let mut offset = 0x80u32;
        for _ in 0..signature_number {
            let mut sig_offset = [0u8; 32];
            sig_offset[28..].copy_from_slice(&offset.to_be_bytes());
            calldata.extend_from_slice(&sig_offset);
            offset += 0x80; // Each signature block is 128 bytes
        }

        // 7. Four signatures
        for i in 1..=signature_number {
            // Length prefix for each signature (65 bytes)
            let mut sig_length = [0u8; 32];
            sig_length[31] = 0x41; // 65 in hex
            calldata.extend_from_slice(&sig_length);

            // Signature data (65 bytes filled with number i)
            let sig = vec![i; 65];
            calldata.extend_from_slice(&sig);

            // Padding to 32 byte boundary
            let padding = vec![0u8; 32 - (65 % 32)];
            calldata.extend_from_slice(&padding);
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
        _signature_number: u8, // For backward compatibility, we'll just use the first signature
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

        // Signature data (65 bytes filled with number 1)
        let sig = vec![1u8; 65];
        calldata.extend_from_slice(&sig);

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
}
