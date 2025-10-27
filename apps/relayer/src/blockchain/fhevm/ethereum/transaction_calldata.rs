use crate::blockchain::PublicDecryptFhevmRequestData;
use crate::core::errors::EventProcessingError;
use crate::core::event::PublicDecryptResponse;
use alloy::dyn_abi::DynSolValue;
use alloy::primitives::Bytes;
use tracing::debug;

pub struct ComputeCalldata;

impl ComputeCalldata {
    /// Computes calldata for FHEVM callback function
    ///
    /// This function constructs the callback data to send decryption results
    /// back to the FHEVM contract that initiated the request.
    pub fn callback_req(
        req: &PublicDecryptFhevmRequestData,
        public_decryption_response: PublicDecryptResponse,
    ) -> Result<Bytes, EventProcessingError> {
        let cleartexts = &public_decryption_response.decrypted_value.to_vec();

        // Construct decryptionProof: numSigners (1 byte) + signatures (65 bytes each) + extraData
        let mut decryption_proof = Vec::new();
        let num_signers = public_decryption_response.signatures.len() as u8;
        decryption_proof.push(num_signers);
        for signature in &public_decryption_response.signatures {
            decryption_proof.extend_from_slice(signature);
        }
        decryption_proof.extend_from_slice(&public_decryption_response.extra_data);

        let params: Vec<DynSolValue> = vec![
            DynSolValue::Uint(req.fhevm_request_id, 256),
            DynSolValue::Bytes(cleartexts.clone()),
            DynSolValue::Bytes(decryption_proof),
        ];
        let encoded_params = DynSolValue::Tuple(params).abi_encode_params();

        let mut calldata = Vec::new();
        calldata.extend_from_slice(&req.callback_selector.0);
        calldata.extend_from_slice(&encoded_params);

        debug!(
            "Callback calldata constructed with {} signers, extraData length: {}, cleartexts length: {}",
            num_signers,
            public_decryption_response.extra_data.len(),
            cleartexts.len()
        );

        Ok(Bytes::from(calldata))
    }
}
