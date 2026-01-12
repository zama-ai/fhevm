use actix_web::{HttpResponse, Responder, web};
use alloy::{hex, primitives::U256};
use connector_utils::types::db::OperationStatus;
use sqlx::Row;

use super::{ApiState, types::{HealthResponse, ShareResponse}};

pub async fn get_share_handler(
    state: web::Data<ApiState>,
    request_id: web::Path<String>,
) -> impl Responder {
    let request_id = match parse_request_id(&request_id) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::BadRequest().json(ShareResponse {
                status: "rejected".to_string(),
                request_id: request_id.into_inner(),
                request_type: None,
                share_index: None,
                encrypted_share: None,
                decrypted_value: None,
                epoch_id: None,
                signature: None,
                signer_address: None,
                anchor_block: None,
                timestamp: None,
                ttl: None,
                observed_at: None,
                estimated_ready_at: None,
                reason: Some(err),
                error_code: Some("INVALID_REQUEST".to_string()),
            });
        }
    };

    // Debug logging for request ID conversion
    tracing::debug!(
        "API /v1/share lookup: request_id_hex={}, le_bytes={}",
        format_u256(request_id),
        hex::encode(request_id.as_le_slice())
    );

    if let Ok(Some(response)) = fetch_user_response(&state, request_id).await {
        tracing::debug!("Found user decryption response for request_id={}", format_u256(request_id));
        return HttpResponse::Ok().json(response);
    }

    if let Ok(Some(response)) = fetch_public_response(&state, request_id).await {
        tracing::debug!("Found public decryption response for request_id={}", format_u256(request_id));
        return HttpResponse::Ok().json(response);
    }

    if let Ok(Some(response)) = fetch_user_request_status(&state, request_id).await {
        tracing::debug!("Found user decryption request for request_id={}", format_u256(request_id));
        return HttpResponse::Ok().json(response);
    }

    if let Ok(Some(response)) = fetch_public_request_status(&state, request_id).await {
        tracing::debug!("Found public decryption request for request_id={}", format_u256(request_id));
        return HttpResponse::Ok().json(response);
    }

    tracing::debug!("Request not found for request_id={}", format_u256(request_id));

    HttpResponse::Ok().json(ShareResponse {
        status: "not_found".to_string(),
        request_id: format_u256(request_id),
        request_type: None,
        share_index: None,
        encrypted_share: None,
        decrypted_value: None,
        epoch_id: None,
        signature: None,
        signer_address: None,
        anchor_block: None,
        timestamp: None,
        ttl: None,
        observed_at: None,
        estimated_ready_at: None,
        reason: Some("Request not observed or expired".to_string()),
        error_code: None,
    })
}

pub async fn health_handler(state: web::Data<ApiState>) -> impl Responder {
    let last_block_processed = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT MAX(block_number) FROM last_block_polled WHERE event_type IN ('PublicDecryptionRequest', 'UserDecryptionRequest')",
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(None)
    .unwrap_or(0)
    .max(0) as u64;

    let pending_public: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM public_decryption_requests WHERE status IN ('pending', 'under_process')",
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(0);
    let pending_user: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM user_decryption_requests WHERE status IN ('pending', 'under_process')",
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(0);

    let uptime = state.start_time.elapsed().as_secs();

    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        signer_address: format_address(state.signer_address),
        uptime,
        last_block_processed,
        pending_requests: (pending_public + pending_user) as u64,
    })
}

async fn fetch_user_response(
    state: &ApiState,
    request_id: U256,
) -> anyhow::Result<Option<ShareResponse>> {
    tracing::debug!(
        "Querying user_decryption_responses: decryption_id (le_bytes)={}",
        hex::encode(request_id.as_le_slice())
    );

    let row = sqlx::query(
        "SELECT user_decrypted_shares, signature, EXTRACT(EPOCH FROM created_at) AS response_at \
         FROM user_decryption_responses WHERE decryption_id = $1 AND status = 'completed'",
    )
    .bind(request_id.as_le_slice())
    .fetch_optional(&state.db_pool)
    .await?;

    if row.is_none() {
        tracing::debug!("No matching row in user_decryption_responses");
    }

    let Some(row) = row else { return Ok(None); };

    let encrypted_share: Vec<u8> = row.try_get("user_decrypted_shares")?;
    let signature: Vec<u8> = row.try_get("signature")?;
    let timestamp: f64 = row.try_get("response_at")?;

    let epoch_id = fetch_epoch_id(&state.db_pool, request_id, true).await?;

    Ok(Some(ShareResponse {
        status: "ready".to_string(),
        request_id: format_u256(request_id),
        request_type: Some("user_decryption".to_string()),
        share_index: Some(state.share_index),
        encrypted_share: Some(format_bytes(&encrypted_share)),
        decrypted_value: None,
        epoch_id: epoch_id.map(format_u256),
        signature: Some(format_bytes(&signature)),
        signer_address: Some(format_address(state.signer_address)),
        anchor_block: None,
        timestamp: Some(timestamp.max(0.0) as u64),
        ttl: Some(0),
        observed_at: None,
        estimated_ready_at: None,
        reason: None,
        error_code: None,
    }))
}

async fn fetch_public_response(
    state: &ApiState,
    request_id: U256,
) -> anyhow::Result<Option<ShareResponse>> {
    let row = sqlx::query(
        "SELECT decrypted_result, signature, EXTRACT(EPOCH FROM created_at) AS response_at \
         FROM public_decryption_responses WHERE decryption_id = $1 AND status = 'completed'",
    )
    .bind(request_id.as_le_slice())
    .fetch_optional(&state.db_pool)
    .await?;

    let Some(row) = row else { return Ok(None); };

    let decrypted_value: Vec<u8> = row.try_get("decrypted_result")?;
    let signature: Vec<u8> = row.try_get("signature")?;
    let timestamp: f64 = row.try_get("response_at")?;

    let epoch_id = fetch_epoch_id(&state.db_pool, request_id, false).await?;

    Ok(Some(ShareResponse {
        status: "ready".to_string(),
        request_id: format_u256(request_id),
        request_type: Some("public_decryption".to_string()),
        share_index: None,
        encrypted_share: None,
        decrypted_value: Some(format_bytes(&decrypted_value)),
        epoch_id: epoch_id.map(format_u256),
        signature: Some(format_bytes(&signature)),
        signer_address: Some(format_address(state.signer_address)),
        anchor_block: None,
        timestamp: Some(timestamp.max(0.0) as u64),
        ttl: Some(0),
        observed_at: None,
        estimated_ready_at: None,
        reason: None,
        error_code: None,
    }))
}

async fn fetch_user_request_status(
    state: &ApiState,
    request_id: U256,
) -> anyhow::Result<Option<ShareResponse>> {
    let row = sqlx::query(
        "SELECT status, rejection_reason, rejection_code, EXTRACT(EPOCH FROM created_at) AS observed_at \
         FROM user_decryption_requests WHERE decryption_id = $1",
    )
    .bind(request_id.as_le_slice())
    .fetch_optional(&state.db_pool)
    .await?;

    let Some(row) = row else { return Ok(None); };

    let status: OperationStatus = row.try_get("status")?;
    let observed_at: f64 = row.try_get("observed_at")?;
    let observed_at_u64 = observed_at.max(0.0) as u64;

    Ok(Some(match status {
        OperationStatus::Failed => ShareResponse {
            status: "rejected".to_string(),
            request_id: format_u256(request_id),
            request_type: Some("user_decryption".to_string()),
            share_index: None,
            encrypted_share: None,
            decrypted_value: None,
            epoch_id: None,
            signature: None,
            signer_address: None,
            anchor_block: None,
            timestamp: None,
            ttl: None,
            observed_at: None,
            estimated_ready_at: None,
            reason: row.try_get::<Option<String>, _>("rejection_reason")?.or(Some(
                "Request rejected".to_string(),
            )),
            error_code: row.try_get::<Option<String>, _>("rejection_code")?,
        },
        _ => ShareResponse {
            status: "pending".to_string(),
            request_id: format_u256(request_id),
            request_type: Some("user_decryption".to_string()),
            share_index: None,
            encrypted_share: None,
            decrypted_value: None,
            epoch_id: None,
            signature: None,
            signer_address: None,
            anchor_block: None,
            timestamp: None,
            ttl: None,
            observed_at: Some(observed_at_u64),
            estimated_ready_at: Some(observed_at_u64),
            reason: None,
            error_code: None,
        },
    }))
}

async fn fetch_public_request_status(
    state: &ApiState,
    request_id: U256,
) -> anyhow::Result<Option<ShareResponse>> {
    let row = sqlx::query(
        "SELECT status, rejection_reason, rejection_code, EXTRACT(EPOCH FROM created_at) AS observed_at \
         FROM public_decryption_requests WHERE decryption_id = $1",
    )
    .bind(request_id.as_le_slice())
    .fetch_optional(&state.db_pool)
    .await?;

    let Some(row) = row else { return Ok(None); };

    let status: OperationStatus = row.try_get("status")?;
    let observed_at: f64 = row.try_get("observed_at")?;
    let observed_at_u64 = observed_at.max(0.0) as u64;

    Ok(Some(match status {
        OperationStatus::Failed => ShareResponse {
            status: "rejected".to_string(),
            request_id: format_u256(request_id),
            request_type: Some("public_decryption".to_string()),
            share_index: None,
            encrypted_share: None,
            decrypted_value: None,
            epoch_id: None,
            signature: None,
            signer_address: None,
            anchor_block: None,
            timestamp: None,
            ttl: None,
            observed_at: None,
            estimated_ready_at: None,
            reason: row.try_get::<Option<String>, _>("rejection_reason")?.or(Some(
                "Request rejected".to_string(),
            )),
            error_code: row.try_get::<Option<String>, _>("rejection_code")?,
        },
        _ => ShareResponse {
            status: "pending".to_string(),
            request_id: format_u256(request_id),
            request_type: Some("public_decryption".to_string()),
            share_index: None,
            encrypted_share: None,
            decrypted_value: None,
            epoch_id: None,
            signature: None,
            signer_address: None,
            anchor_block: None,
            timestamp: None,
            ttl: None,
            observed_at: Some(observed_at_u64),
            estimated_ready_at: Some(observed_at_u64),
            reason: None,
            error_code: None,
        },
    }))
}

async fn fetch_epoch_id(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    request_id: U256,
    is_user: bool,
) -> anyhow::Result<Option<U256>> {
    let query = if is_user {
        "SELECT epoch_id FROM user_decryption_requests WHERE decryption_id = $1"
    } else {
        "SELECT epoch_id FROM public_decryption_requests WHERE decryption_id = $1"
    };

    let row = sqlx::query(query)
        .bind(request_id.as_le_slice())
        .fetch_optional(db_pool)
        .await?;

    let Some(row) = row else { return Ok(None); };

    let epoch_bytes: Vec<u8> = row.try_get("epoch_id")?;
    if epoch_bytes.len() != 32 {
        return Ok(None);
    }
    let epoch_id = U256::from_le_slice(&epoch_bytes);
    if epoch_id == U256::ZERO {
        Ok(None)
    } else {
        Ok(Some(epoch_id))
    }
}

fn parse_request_id(value: &str) -> Result<U256, String> {
    let trimmed = value.trim();
    let hex_str = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    let bytes = hex::decode(hex_str).map_err(|e| format!("Invalid requestId: {e}"))?;
    if bytes.len() > 32 {
        return Err("Invalid requestId length".to_string());
    }
    let mut padded = [0u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(&bytes);
    Ok(U256::from_be_bytes(padded))
}

fn format_u256(value: U256) -> String {
    format!("0x{}", hex::encode(value.to_be_bytes::<32>()))
}

fn format_bytes(value: &[u8]) -> String {
    format!("0x{}", hex::encode(value))
}

fn format_address(value: alloy::primitives::Address) -> String {
    value.to_checksum(None)
}
