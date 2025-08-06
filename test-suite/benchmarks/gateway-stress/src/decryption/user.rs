use crate::decryption::extract_id_from_receipt;
use alloy::{primitives::U256, rpc::types::TransactionReceipt, sol_types::SolEvent};
use anyhow::anyhow;
use fhevm_gateway_rust_bindings::decryption::Decryption;

fn extract_user_decryption_id_from_receipt(receipt: &TransactionReceipt) -> anyhow::Result<U256> {
    extract_id_from_receipt(
        receipt,
        Decryption::UserDecryptionRequest::SIGNATURE_HASH,
        |log| {
            Decryption::UserDecryptionRequest::decode_log_data(log, true)
                .map(|event| event.decryptionId)
                .map_err(|e| anyhow!("Failed to decode event data {e}"))
        },
    )
}
