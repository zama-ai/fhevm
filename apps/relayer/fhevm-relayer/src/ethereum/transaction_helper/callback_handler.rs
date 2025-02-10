use crate::errors::EventProcessingError;
use crate::ethereum_host_l1_handlers::DecryptionRequestData;
use alloy::primitives::{Bytes, U256};

pub struct CallbackHandler;

impl CallbackHandler {
    pub fn prepare_callback_data(
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
}
