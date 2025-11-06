use alloy::primitives::{Address, U256};
use anyhow::anyhow;
use fhevm_gateway_bindings::{
    decryption::Decryption::SnsCiphertextMaterial, kms_generation::IKMSGeneration::KeyDigest,
};
use std::{fmt::Display, str::FromStr};

use crate::types::GatewayEventKind;

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

/// Struct representing the `ParamsType` enum in the database.
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
            Err(anyhow!("Invalid ParamsType value: {value}"))
        }
    }
}

/// Struct representing the `KeyType` enum in the database.
#[derive(sqlx::Type, Copy, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "key_type")]
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
            Err(anyhow!("Invalid KeyType value: {value}"))
        }
    }
}

impl FromStr for KeyType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ServerKey" => Ok(Self::Server),
            "PublicKey" => Ok(Self::Public),
            _ => Err(anyhow!("Invalid KeyType value: {s}")),
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

/// Struct representing the `ParamsType` enum in the database.
#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
#[sqlx(type_name = "event_type")]
pub enum EventType {
    PublicDecryptionRequest,
    UserDecryptionRequest,
    PrepKeygenRequest,
    KeygenRequest,
    CrsgenRequest,
    PrssInit,
    KeyReshareSameSet,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::PublicDecryptionRequest => write!(f, "PublicDecryptionRequest"),
            EventType::UserDecryptionRequest => write!(f, "UserDecryptionRequest"),
            EventType::PrepKeygenRequest => write!(f, "PrepKeygenRequest"),
            EventType::KeygenRequest => write!(f, "KeygenRequest"),
            EventType::CrsgenRequest => write!(f, "CrsgenRequest"),
            EventType::PrssInit => write!(f, "PrssInit"),
            EventType::KeyReshareSameSet => write!(f, "KeyReshareSameSet"),
        }
    }
}

impl From<&GatewayEventKind> for EventType {
    fn from(value: &GatewayEventKind) -> Self {
        match value {
            GatewayEventKind::PublicDecryption(_) => Self::PublicDecryptionRequest,
            GatewayEventKind::UserDecryption(_) => Self::UserDecryptionRequest,
            GatewayEventKind::PrepKeygen(_) => Self::PrepKeygenRequest,
            GatewayEventKind::Keygen(_) => Self::KeygenRequest,
            GatewayEventKind::Crsgen(_) => Self::CrsgenRequest,
            GatewayEventKind::PrssInit(_) => Self::PrssInit,
            GatewayEventKind::KeyReshareSameSet(_) => Self::KeyReshareSameSet,
        }
    }
}
