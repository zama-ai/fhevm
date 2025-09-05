use std::str::FromStr;

use alloy::primitives::{Address, U256};
use anyhow::anyhow;
use fhevm_gateway_bindings::{
    decryption::Decryption::SnsCiphertextMaterial, kms_management::IKmsManagement::KeyDigest,
};

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
/// Struct representing the `ParamsType` enum in the database.
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

#[derive(sqlx::Type, Copy, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "key_type")]
/// Struct representing the `ParamsType` enum in the database.
pub enum KeyType {
    Server,
    #[default]
    Public,
}

impl TryFrom<u8> for KeyType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == Self::Server as u8 {
            Ok(Self::Server)
        } else if value == Self::Public as u8 {
            Ok(Self::Public)
        } else {
            Err(anyhow!("Invalid KeyType value"))
        }
    }
}

impl FromStr for KeyType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "server" => Ok(Self::Server),
            "public" => Ok(Self::Public),
            _ => Err(anyhow!("Invalid KeyType value")),
        }
    }
}

/// Struct representing how `KeyDigest` are stored in the database.
#[derive(sqlx::Type, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "key_digest")]
pub struct KeyDigestDbItem {
    pub key_type: KeyType,
    pub digest: Vec<u8>,
}

impl From<KeyDigestDbItem> for KeyDigest {
    fn from(value: KeyDigestDbItem) -> Self {
        Self {
            keyType: value.key_type as u8,
            digest: value.digest.into(),
        }
    }
}
