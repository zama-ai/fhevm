use alloy::primitives::U256;
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;

/// Struct representing how `SnsCiphertextMaterial` are stored in the database.
#[derive(sqlx::Type, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "sns_ciphertext_material")]
pub struct SnsCiphertextMaterialDbItem {
    pub ct_handle: [u8; 32],
    pub key_id: [u8; 32],
    pub sns_ciphertext_digest: [u8; 32],
    pub storage_urls: Vec<String>,
}

impl From<&SnsCiphertextMaterialDbItem> for SnsCiphertextMaterial {
    fn from(value: &SnsCiphertextMaterialDbItem) -> Self {
        Self {
            ctHandle: value.ct_handle.into(),
            keyId: U256::from_le_bytes(value.key_id),
            snsCiphertextDigest: value.sns_ciphertext_digest.into(),
        }
    }
}

impl SnsCiphertextMaterialDbItem {
    pub fn new(sns_ct: &SnsCiphertextMaterial, storage_urls: Vec<String>) -> Self {
        Self {
            ct_handle: *sns_ct.ctHandle,
            key_id: sns_ct.keyId.to_le_bytes(),
            sns_ciphertext_digest: *sns_ct.snsCiphertextDigest,
            storage_urls,
        }
    }
}
