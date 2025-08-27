use alloy::primitives::{Address, U256};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;

/// Struct representing how `SnsCiphertextMaterial` are stored in the database.
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

#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
#[sqlx(type_name = "params_type")]
pub enum ParamsTypeDb {
    Default,
    Test,
}

impl TryFrom<u8> for ParamsTypeDb {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == Self::Default as u8 {
            Ok(Self::Default)
        } else if value == Self::Test as u8 {
            Ok(Self::Test)
        } else {
            Err(anyhow!("Invalid ParamsType value"))
        }
    }
}
