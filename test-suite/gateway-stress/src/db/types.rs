use alloy::primitives::{Address, U256};
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use sqlx::{Row, postgres::PgRow, types::time::PrimitiveDateTime};

// Copy-pasted from kms-connector's code
// Ideally, we would use `connector-utils` as a dependency crate to avoid duplication.
// But as both `kms` and `kms-connector` used pinned dependencies, they are conflicting and cannot
// be both used by the `gateway-stress` tool

#[derive(sqlx::Type, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "sns_ciphertext_material")]
pub struct SnsCiphertextMaterialDbItem {
    ct_handle: [u8; 32],
    key_id: [u8; 32],
    sns_ciphertext_digest: [u8; 32],
    coprocessor_tx_sender_addresses: Vec<[u8; 20]>,
}

impl From<&SnsCiphertextMaterial> for SnsCiphertextMaterialDbItem {
    fn from(value: &SnsCiphertextMaterial) -> Self {
        Self {
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

pub struct DecryptionRequestDbMetadata {
    pub id: U256,
    pub created_at: PrimitiveDateTime,
}

pub struct DecryptionResponseDbMetadata {
    pub id: U256,
    pub created_at: PrimitiveDateTime,
}

impl From<PgRow> for DecryptionRequestDbMetadata {
    fn from(row: PgRow) -> Self {
        Self {
            id: U256::from_le_slice(row.get("decryption_id")),
            created_at: row.get("created_at"),
        }
    }
}
