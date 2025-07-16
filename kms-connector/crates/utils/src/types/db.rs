use alloy::primitives::{Address, U256};
use fhevm_gateway_rust_bindings::decryption::Decryption::SnsCiphertextMaterial;

/// Struct representing how `SnsCiphertextMaterial` are stored in the database.
#[derive(sqlx::Type, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "sns_ciphertext_material")]
pub struct SnsCiphertextMaterialDbItem {
    coprocessor_context_id: [u8; 32],
    ct_handle: [u8; 32],
    key_id: [u8; 32],
    sns_ciphertext_digest: [u8; 32],
    coprocessor_tx_sender_addresses: Vec<[u8; 20]>,
}

impl From<&SnsCiphertextMaterial> for SnsCiphertextMaterialDbItem {
    fn from(value: &SnsCiphertextMaterial) -> Self {
        Self {
            coprocessor_context_id: value.coprocessorContextId.to_le_bytes(),
            ct_handle: *value.ctHandle,
            key_id: value.keyId.to_le_bytes(),
            sns_ciphertext_digest: *value.snsCiphertextDigest,
            coprocessor_tx_sender_addresses: value
                .coprocessorTxSenderAddresses
                .iter()
                .map(|a| *a.0)
                .collect(),
        }
    }
}

impl From<&SnsCiphertextMaterialDbItem> for SnsCiphertextMaterial {
    fn from(value: &SnsCiphertextMaterialDbItem) -> Self {
        Self {
            coprocessorContextId: U256::from_le_bytes(value.coprocessor_context_id),
            ctHandle: value.ct_handle.into(),
            keyId: U256::from_le_bytes(value.key_id),
            snsCiphertextDigest: value.sns_ciphertext_digest.into(),
            coprocessorTxSenderAddresses: value
                .coprocessor_tx_sender_addresses
                .iter()
                .map(Address::from)
                .collect(),
        }
    }
}
