use crate::{
    monitoring::otlp::PropagationContext,
    tests::rand::{rand_digest, rand_signature, rand_u256},
    types::{
        CrsgenResponse, EpochResultResponse, KeygenResponse, KmsResponseKind,
        NewKmsContextResponse, PrepKeygenResponse, PublicDecryptionResponse,
        UserDecryptionResponse,
        db::{KeyDigestDbItem, KeyType, OperationStatus},
    },
};
use alloy::{primitives::U256, sol_types::SolValue};
use fhevm_host_bindings::protocol_config::IProtocolConfig::{EpochCrsResult, EpochKeyResult};
use sqlx::{Pool, Postgres, types::chrono::Utc};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TestResponseType {
    PublicDecryption,
    UserDecryption,
    PrepKeygen,
    Keygen,
    Crsgen,
    NewKmsContext,
    EpochResult,
}

impl Display for TestResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::PublicDecryption => "PublicDecryptionResponse",
            Self::UserDecryption => "UserDecryptionResponse",
            Self::PrepKeygen => "PrepKeygenResponse",
            Self::Keygen => "KeygenResponse",
            Self::Crsgen => "CrsgenResponse",
            Self::NewKmsContext => "NewKmsContextResponse",
            Self::EpochResult => "EpochResultResponse",
        };
        f.write_str(s)
    }
}

pub async fn insert_rand_response(
    db: &Pool<Postgres>,
    response_type: TestResponseType,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<KmsResponseKind> {
    let inserted_response = match response_type {
        TestResponseType::PublicDecryption => KmsResponseKind::PublicDecryption(
            insert_rand_public_decrypt_response(db, id, status).await?,
        ),
        TestResponseType::UserDecryption => KmsResponseKind::UserDecryption(
            insert_rand_user_decrypt_response(db, id, status).await?,
        ),
        TestResponseType::PrepKeygen => {
            KmsResponseKind::PrepKeygen(insert_rand_prep_keygen_response(db, id, status).await?)
        }
        TestResponseType::Keygen => {
            KmsResponseKind::Keygen(insert_rand_keygen_response(db, id, status).await?)
        }
        TestResponseType::Crsgen => {
            KmsResponseKind::Crsgen(insert_rand_crsgen_response(db, id, status).await?)
        }
        TestResponseType::NewKmsContext => KmsResponseKind::NewKmsContext(
            insert_rand_new_kms_context_response(db, id, status).await?,
        ),
        TestResponseType::EpochResult => {
            KmsResponseKind::EpochResult(insert_rand_epoch_result_response(db, id, status).await?)
        }
    };
    Ok(inserted_response)
}

pub async fn insert_rand_public_decrypt_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<PublicDecryptionResponse> {
    let decryption_id = id.unwrap_or_else(rand_u256);
    let decrypted_result = rand_signature();
    let signature = rand_signature();
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO public_decryption_responses(
            decryption_id, decrypted_result, signature, extra_data, created_at, otlp_context, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        decrypted_result,
        signature,
        vec![],
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(PublicDecryptionResponse {
        decryption_id,
        decrypted_result,
        signature,
        extra_data: vec![],
    })
}

pub async fn insert_rand_user_decrypt_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<UserDecryptionResponse> {
    let decryption_id = id.unwrap_or_else(rand_u256);
    let user_decrypted_shares = rand_signature();
    let signature = rand_signature();
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO user_decryption_responses(
            decryption_id, user_decrypted_shares, signature, extra_data, created_at, otlp_context, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        user_decrypted_shares,
        signature,
        vec![],
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(UserDecryptionResponse {
        decryption_id,
        user_decrypted_shares,
        signature,
        extra_data: vec![],
    })
}

pub async fn insert_rand_prep_keygen_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<PrepKeygenResponse> {
    let prep_keygen_id = id.unwrap_or_else(rand_u256);
    let signature = rand_signature();
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO prep_keygen_responses(prep_keygen_id, signature, created_at, otlp_context, status)
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        prep_keygen_id.as_le_slice(),
        signature,
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(PrepKeygenResponse {
        prep_keygen_id,
        signature,
    })
}

pub async fn insert_rand_keygen_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<KeygenResponse> {
    let key_id = id.unwrap_or_else(rand_u256);
    let key_digests = vec![KeyDigestDbItem {
        key_type: KeyType::Public,
        digest: rand_digest().to_vec(),
    }];
    let signature = rand_signature();
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO keygen_responses(key_id, key_digests, signature, created_at, otlp_context, status)
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        key_id.as_le_slice(),
        key_digests.clone() as Vec<KeyDigestDbItem>,
        signature,
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(KeygenResponse {
        key_id,
        is_migration: false,
        key_digests,
        signature,
    })
}

pub async fn insert_rand_crsgen_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<CrsgenResponse> {
    let crs_id = id.unwrap_or_else(rand_u256);
    let crs_digest = rand_digest().to_vec();
    let signature = rand_signature();
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO crsgen_responses(crs_id, crs_digest, signature, created_at, otlp_context, status)
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        crs_digest.clone(),
        signature,
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(CrsgenResponse {
        crs_id,
        crs_digest,
        signature,
    })
}

pub async fn insert_rand_new_kms_context_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<NewKmsContextResponse> {
    let context_id = id.unwrap_or_else(rand_u256);
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO new_kms_context_responses(context_id, created_at, otlp_context, status)
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        context_id.as_le_slice(),
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(NewKmsContextResponse { context_id })
}

pub async fn insert_rand_epoch_result_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<EpochResultResponse> {
    let epoch_id = id.unwrap_or_else(rand_u256);
    let context_id = rand_u256();
    let keys = Vec::<EpochKeyResult>::new().abi_encode();
    let crs_list = Vec::<EpochCrsResult>::new().abi_encode();
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO epoch_result_responses(
            context_id, epoch_id, keys, crs_list, created_at, otlp_context, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        context_id.as_le_slice(),
        epoch_id.as_le_slice(),
        keys.clone(),
        crs_list.clone(),
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(EpochResultResponse {
        context_id,
        epoch_id,
        keys,
        crs_list,
    })
}
