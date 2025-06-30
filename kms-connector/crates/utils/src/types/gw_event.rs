use std::fmt::Display;

use crate::types::db::SnsCiphertextMaterialDbItem;
use alloy::primitives::U256;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{
        PublicDecryptionRequest, SnsCiphertextMaterial, UserDecryptionRequest,
    },
    kmsmanagement::KmsManagement::{
        CrsgenRequest, KeygenRequest, KskgenRequest, PreprocessKeygenRequest,
        PreprocessKskgenRequest,
    },
};
use sqlx::{Row, postgres::PgRow};

/// The events emitted by the Gateway which are monitored by the KMS Connector.
#[derive(Clone, Debug, PartialEq)]
pub enum GatewayEvent {
    PublicDecryption(PublicDecryptionRequest),
    UserDecryption(UserDecryptionRequest),
    PreprocessKeygen(PreprocessKeygenRequest),
    PreprocessKskgen(PreprocessKskgenRequest),
    Keygen(KeygenRequest),
    Kskgen(KskgenRequest),
    Crsgen(CrsgenRequest),
}

impl GatewayEvent {
    pub fn from_public_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let sns_ct_materials = row
            .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
            .iter()
            .map(SnsCiphertextMaterial::from)
            .collect();

        Ok(GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            snsCtMaterials: sns_ct_materials,
        }))
    }

    pub fn from_user_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let sns_ct_materials = row
            .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
            .iter()
            .map(SnsCiphertextMaterial::from)
            .collect();

        Ok(GatewayEvent::UserDecryption(UserDecryptionRequest {
            decryptionId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            snsCtMaterials: sns_ct_materials,
            userAddress: row.try_get::<[u8; 20], _>("user_address")?.into(),
            publicKey: row.try_get::<Vec<u8>, _>("public_key")?.into(),
        }))
    }

    pub fn from_pre_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::PreprocessKeygen(PreprocessKeygenRequest {
            preKeygenRequestId: U256::from_le_bytes(
                row.try_get::<[u8; 32], _>("pre_keygen_request_id")?,
            ),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
        }))
    }

    pub fn from_pre_kskgen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::PreprocessKskgen(PreprocessKskgenRequest {
            preKskgenRequestId: U256::from_le_bytes(
                row.try_get::<[u8; 32], _>("pre_kskgen_request_id")?,
            ),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
        }))
    }

    pub fn from_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Keygen(KeygenRequest {
            preKeyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_key_id")?),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
        }))
    }

    pub fn from_kskgen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Kskgen(KskgenRequest {
            preKskId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_ksk_id")?),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
            sourceKeyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("source_key_id")?),
            destKeyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("dest_key_id")?),
        }))
    }

    pub fn from_crsgen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Crsgen(CrsgenRequest {
            crsgenRequestId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crsgen_request_id")?),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
        }))
    }
}

impl Display for GatewayEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayEvent::PublicDecryption(e) => {
                write!(f, "PublicDecryptionRequest #{}", e.decryptionId)
            }
            GatewayEvent::UserDecryption(e) => {
                write!(f, "UserDecryptionRequest #{}", e.decryptionId)
            }
            GatewayEvent::PreprocessKeygen(e) => {
                write!(f, "PreprocessKeygenRequest #{}", e.preKeygenRequestId)
            }
            GatewayEvent::PreprocessKskgen(e) => {
                write!(f, "PreprocessKskgenRequest #{}", e.preKskgenRequestId)
            }
            GatewayEvent::Keygen(e) => write!(f, "KeygenRequest #{}", e.preKeyId),
            GatewayEvent::Kskgen(e) => write!(f, "KskgenRequest #{}", e.preKskId),
            GatewayEvent::Crsgen(e) => write!(f, "CrsgenRequest #{}", e.crsgenRequestId),
        }
    }
}

impl From<PublicDecryptionRequest> for GatewayEvent {
    fn from(value: PublicDecryptionRequest) -> Self {
        Self::PublicDecryption(value)
    }
}

impl From<UserDecryptionRequest> for GatewayEvent {
    fn from(value: UserDecryptionRequest) -> Self {
        Self::UserDecryption(value)
    }
}

impl From<PreprocessKeygenRequest> for GatewayEvent {
    fn from(value: PreprocessKeygenRequest) -> Self {
        Self::PreprocessKeygen(value)
    }
}

impl From<PreprocessKskgenRequest> for GatewayEvent {
    fn from(value: PreprocessKskgenRequest) -> Self {
        Self::PreprocessKskgen(value)
    }
}

impl From<KeygenRequest> for GatewayEvent {
    fn from(value: KeygenRequest) -> Self {
        Self::Keygen(value)
    }
}

impl From<KskgenRequest> for GatewayEvent {
    fn from(value: KskgenRequest) -> Self {
        Self::Kskgen(value)
    }
}

impl From<CrsgenRequest> for GatewayEvent {
    fn from(value: CrsgenRequest) -> Self {
        Self::Crsgen(value)
    }
}
