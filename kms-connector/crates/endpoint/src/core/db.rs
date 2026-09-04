//! SQL layer of the endpoint: request upserts, response reads and id encoding helpers.

use alloy::{
    hex,
    primitives::{B256, U256},
};
use anyhow::anyhow;
use connector_utils::monitoring::otlp::PropagationContext;
use kms_connector_api::{PublicDecryptionRequest, UserDecryptionRequest};
use sqlx::{
    PgExecutor,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};

/// A `public_decryption_responses` row, either a payload or a worker-side error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicDecryptionResponseRow {
    pub decrypted_result: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
    pub extra_data: Vec<u8>,
    pub error_code: Option<String>,
    pub error_details: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// A `user_decryption_responses` row, either a payload or a worker-side error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDecryptionResponseRow {
    pub user_decrypted_shares: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
    pub extra_data: Vec<u8>,
    pub error_code: Option<String>,
    pub error_details: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Reads the HTTP-sourced public decryption response of `id`, if any.
pub async fn read_public_decryption_response<'e>(
    executor: impl PgExecutor<'e>,
    id: B256,
) -> sqlx::Result<Option<PublicDecryptionResponseRow>> {
    let db_id = id_to_db_bytes(id);
    sqlx::query_as!(
        PublicDecryptionResponseRow,
        "SELECT decrypted_result, signature, extra_data, error_code, error_details, created_at
        FROM public_decryption_responses WHERE decryption_id = $1 AND source = 'http'",
        db_id.as_slice(),
    )
    .fetch_optional(executor)
    .await
}

/// Reads the HTTP-sourced user decryption response of `id`, if any.
pub async fn read_user_decryption_response<'e>(
    executor: impl PgExecutor<'e>,
    id: B256,
) -> sqlx::Result<Option<UserDecryptionResponseRow>> {
    let db_id = id_to_db_bytes(id);
    sqlx::query_as!(
        UserDecryptionResponseRow,
        "SELECT user_decrypted_shares, signature, extra_data, error_code, error_details, created_at
        FROM user_decryption_responses WHERE decryption_id = $1 AND source = 'http'",
        db_id.as_slice(),
    )
    .fetch_optional(executor)
    .await
}

/// Upserts an HTTP-sourced public decryption request, or re-arms it if it previously `failed`.
///
/// Only HTTP-sourced rows are ever re-armed: a Gateway row can never be touched by the endpoint.
pub async fn upsert_public_decryption_request<'e>(
    executor: impl PgExecutor<'e>,
    id: B256,
    request: &PublicDecryptionRequest,
    otlp_ctx: &PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let ct_handles: Vec<Vec<u8>> = request.ctHandles.iter().map(|h| h.to_vec()).collect();
    let db_id = id_to_db_bytes(id);

    sqlx::query!(
        "INSERT INTO public_decryption_requests AS existing (
            decryption_id, ct_handles, extra_data, tx_hash, created_at, otlp_context, source
        )
        VALUES ($1, $2, $3, NULL, $4, $5, 'http')
        ON CONFLICT (decryption_id) DO UPDATE SET
            status = 'pending',
            created_at = EXCLUDED.created_at,
            otlp_context = EXCLUDED.otlp_context
        WHERE existing.status = 'failed' AND existing.source = 'http'",
        db_id.as_slice(),
        &ct_handles,
        request.extraData.as_ref(),
        Utc::now(),
        bc2wrap::serialize(otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

/// Upserts an HTTP-sourced user decryption request, or re-arms it if it previously `failed`.
pub async fn upsert_user_decryption_request<'e>(
    executor: impl PgExecutor<'e>,
    id: B256,
    request: &UserDecryptionRequest,
    otlp_ctx: &PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let mut ct_handles: Vec<Vec<u8>> = Vec::with_capacity(request.handles.len());
    let mut handle_owner_addresses: Vec<Vec<u8>> = Vec::with_capacity(request.handles.len());
    let mut handle_contract_addresses: Vec<Vec<u8>> = Vec::with_capacity(request.handles.len());
    for entry in &request.handles {
        ct_handles.push(entry.handle.to_vec());
        handle_owner_addresses.push(entry.ownerAddress.to_vec());
        handle_contract_addresses.push(entry.contractAddress.to_vec());
    }
    let allowed_contracts: Vec<Vec<u8>> = request
        .allowedContracts
        .iter()
        .map(|a| a.to_vec())
        .collect();

    let start_timestamp: i64 = request
        .requestValidity
        .startTimestamp
        .try_into()
        .map_err(|_| anyhow!("startTimestamp does not fit in i64"))?;
    let duration_seconds: i64 = request
        .requestValidity
        .durationSeconds
        .try_into()
        .map_err(|_| anyhow!("durationSeconds does not fit in i64"))?;
    let db_id = id_to_db_bytes(id);

    sqlx::query!(
        "INSERT INTO user_decryption_requests AS existing (
            decryption_id, ct_handles, user_address, public_key, extra_data, tx_hash,
            created_at, otlp_context, handle_owner_addresses, handle_contract_addresses,
            allowed_contracts, start_timestamp, duration_seconds, signature, source
        )
        VALUES ($1, $2, $3, $4, $5, NULL, $6, $7, $8, $9, $10, $11, $12, $13, 'http')
        ON CONFLICT (decryption_id) DO UPDATE SET
            status = 'pending',
            created_at = EXCLUDED.created_at,
            otlp_context = EXCLUDED.otlp_context
        WHERE existing.status = 'failed' AND existing.source = 'http'",
        db_id.as_slice(),
        &ct_handles,
        request.userAddress.as_slice(),
        request.publicKey.as_ref(),
        request.extraData.as_ref(),
        Utc::now(),
        bc2wrap::serialize(otlp_ctx)?,
        &handle_owner_addresses,
        &handle_contract_addresses,
        &allowed_contracts,
        start_timestamp,
        duration_seconds,
        request.signature.as_ref(),
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

/// Converts an interface `decryption_id` into the bytes stored as primary key in the database.
pub fn id_to_db_bytes(id: B256) -> [u8; 32] {
    U256::from_be_bytes(id.0).to_le_bytes()
}

/// Converts a database primary key back into the interface `decryption_id`.
pub fn id_from_db_bytes(bytes: &[u8]) -> anyhow::Result<B256> {
    if bytes.len() != 32 {
        return Err(anyhow!(
            "Invalid decryption_id length: {} bytes, expected 32",
            bytes.len()
        ));
    }
    Ok(B256::from(U256::from_le_slice(bytes)))
}

/// Decodes the payload of a `http_*_decryption_response_available` notification, which is the
/// hex-encoded id of the response row.
pub fn id_from_notification_payload(payload: &str) -> anyhow::Result<B256> {
    let bytes = hex::decode(payload)
        .map_err(|e| anyhow!("Failed to hex-decode notification payload: {e}"))?;
    id_from_db_bytes(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_round_trips_through_db_encoding() {
        let id = B256::repeat_byte(0xab);
        assert_eq!(id_from_db_bytes(&id_to_db_bytes(id)).unwrap(), id);

        let id = B256::from(U256::from(1u64).to_be_bytes::<32>());
        let db = id_to_db_bytes(id);
        // Little-endian: the least significant byte comes first.
        assert_eq!(db[0], 1);
        assert!(db[1..].iter().all(|b| *b == 0));
        assert_eq!(id_from_db_bytes(&db).unwrap(), id);
    }

    #[test]
    fn db_encoding_matches_gw_listener_encoding() {
        // The gw-listener stores `decryption_id.as_le_slice()` of a `U256`; the interface id
        // is the big-endian representation of the same number.
        let number = U256::from_str_radix("deadbeef0123456789", 16).unwrap();
        let interface_id = B256::from(number.to_be_bytes::<32>());
        assert_eq!(
            id_to_db_bytes(interface_id).as_slice(),
            number.as_le_slice()
        );
    }

    #[test]
    fn notification_payload_decodes() {
        let id = B256::repeat_byte(0x42);
        let payload = hex::encode(id_to_db_bytes(id));
        assert_eq!(id_from_notification_payload(&payload).unwrap(), id);

        assert!(id_from_notification_payload("zz").is_err());
        assert!(id_from_notification_payload("00").is_err());
    }
}
