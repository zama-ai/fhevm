use std::str::FromStr;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use alloy::hex;
use alloy::primitives::{keccak256, Address, Bytes, FixedBytes, U256};
use alloy::{network::Ethereum, providers::Provider};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::Eip712Domain;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{Pool, Postgres, Row};
use tracing::{error, info, warn};

use crate::api::types::{
    CiphertextResponse, CiphertextStatus, HealthResponse, HealthStatus, ResponseStatus,
    VerifyInputRequest, VerifyInputResponse,
};
use crate::api::signing::{sign_ciphertext_response, sign_input_verification_attestation};
use crate::aws_s3::AwsS3Interface;
use crate::gw_listener::GatewayListener;

pub struct ApiState<P, A>
where
    P: Provider<Ethereum> + Clone + Send + Sync + 'static,
    A: AwsS3Interface + Clone + Send + Sync + 'static,
{
    pub listener: Arc<GatewayListener<P, A>>,
    pub db_pool: Pool<Postgres>,
    pub signer: Arc<PrivateKeySigner>,
    pub input_eip712_domain: Eip712Domain,
    pub ciphertext_eip712_domain: Eip712Domain,
    pub signer_address: Address,
    pub start_time: Instant,
}

pub async fn verify_input_handler<P, A>(
    State(state): State<Arc<ApiState<P, A>>>,
    Json(request): Json<VerifyInputRequest>,
) -> impl IntoResponse
where
    P: Provider<Ethereum> + Clone + Send + Sync + 'static,
    A: AwsS3Interface + Clone + Send + Sync + 'static,
{
    info!(
        request_id = %request.request_id,
        user_address = %request.user_address,
        contract_address = %request.contract_address,
        "Received verify-input request"
    );

    let computed_commitment = keccak256(&request.ciphertext_with_zkpok);
    if computed_commitment != request.commitment {
        warn!(
            request_id = %request.request_id,
            expected = %request.commitment,
            computed = %computed_commitment,
            "Commitment mismatch"
        );
        return (
            StatusCode::OK,
            Json(VerifyInputResponse {
                status: ResponseStatus::Rejected,
                request_id: request.request_id,
                handles: None,
                epoch_id: None,
                signature: None,
                signer_address: None,
                timestamp: None,
                reason: Some("Payload hash does not match on-chain commitment".to_string()),
                error_code: Some("COMMITMENT_MISMATCH".to_string()),
            }),
        );
    }

    let request_record = match fetch_input_verification_request(&state.db_pool, request.request_id).await {
        Ok(record) => record,
        Err(e) => {
            error!(error = %e, "Failed to fetch request record");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(VerifyInputResponse {
                    status: ResponseStatus::Rejected,
                    request_id: request.request_id,
                    handles: None,
                    epoch_id: None,
                    signature: None,
                    signer_address: None,
                    timestamp: None,
                    reason: Some("Internal error".to_string()),
                    error_code: Some("INTERNAL_ERROR".to_string()),
                }),
            );
        }
    };

    let Some(request_record) = request_record else {
        warn!(request_id = %request.request_id, "Request not found in registry");
        return (
            StatusCode::OK,
            Json(VerifyInputResponse {
                status: ResponseStatus::Rejected,
                request_id: request.request_id,
                handles: None,
                epoch_id: None,
                signature: None,
                signer_address: None,
                timestamp: None,
                reason: Some("Request not observed on Gateway".to_string()),
                error_code: Some("REQUEST_NOT_FOUND".to_string()),
            }),
        );
    };

    if !request_matches(&request_record, &request) {
        warn!(request_id = %request.request_id, "Request metadata mismatch");
        return (
            StatusCode::OK,
            Json(VerifyInputResponse {
                status: ResponseStatus::Rejected,
                request_id: request.request_id,
                handles: None,
                epoch_id: None,
                signature: None,
                signer_address: None,
                timestamp: None,
                reason: Some("Request metadata mismatch".to_string()),
                error_code: Some("REQUEST_MISMATCH".to_string()),
            }),
        );
    }

    if let Err(e) = insert_verify_proof_request(&state.db_pool, &request).await {
        error!(error = %e, request_id = %request.request_id, "Failed to insert verify proof request");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(VerifyInputResponse {
                status: ResponseStatus::Rejected,
                request_id: request.request_id,
                handles: None,
                epoch_id: None,
                signature: None,
                signer_address: None,
                timestamp: None,
                reason: Some("Failed to queue verification".to_string()),
                error_code: Some("INTERNAL_ERROR".to_string()),
            }),
        );
    }

    match fetch_verification_status(&state.db_pool, request.request_id).await {
        Ok(Some(status)) => {
            if status.verified == Some(true) {
                let handles = match parse_handles(&status.handles) {
                    Ok(handles) => handles,
                    Err(e) => {
                        error!(error = %e, request_id = %request.request_id, "Invalid handles data");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(VerifyInputResponse {
                                status: ResponseStatus::Rejected,
                                request_id: request.request_id,
                                handles: None,
                                epoch_id: None,
                                signature: None,
                                signer_address: None,
                                timestamp: None,
                                reason: Some("Invalid handles data".to_string()),
                                error_code: Some("INTERNAL_ERROR".to_string()),
                            }),
                        );
                    }
                };

                let epoch_id = match fetch_epoch_id(&state.db_pool, request_record.contract_chain_id).await {
                    Ok(Some(epoch_id)) => epoch_id,
                    Ok(None) => {
                        error!(request_id = %request.request_id, "Missing epoch id for chain");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(VerifyInputResponse {
                                status: ResponseStatus::Rejected,
                                request_id: request.request_id,
                                handles: None,
                                epoch_id: None,
                                signature: None,
                                signer_address: None,
                                timestamp: None,
                                reason: Some("Epoch id not available".to_string()),
                                error_code: Some("INTERNAL_ERROR".to_string()),
                            }),
                        );
                    }
                    Err(e) => {
                        error!(error = %e, request_id = %request.request_id, "Failed to fetch epoch id");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(VerifyInputResponse {
                                status: ResponseStatus::Rejected,
                                request_id: request.request_id,
                                handles: None,
                                epoch_id: None,
                                signature: None,
                                signer_address: None,
                                timestamp: None,
                                reason: Some("Internal error".to_string()),
                                error_code: Some("INTERNAL_ERROR".to_string()),
                            }),
                        );
                    }
                };

                let signature = match sign_input_verification_attestation(
                    state.signer.as_ref(),
                    &state.input_eip712_domain,
                    handles.clone(),
                    request_record.user_address,
                    request_record.contract_address,
                    request_record.contract_chain_id,
                    Bytes::from(vec![0u8]),
                ) {
                    Ok(signature) => signature,
                    Err(e) => {
                        error!(error = %e, request_id = %request.request_id, "Failed to sign attestation");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(VerifyInputResponse {
                                status: ResponseStatus::Rejected,
                                request_id: request.request_id,
                                handles: None,
                                epoch_id: None,
                                signature: None,
                                signer_address: None,
                                timestamp: None,
                                reason: Some("Internal error".to_string()),
                                error_code: Some("INTERNAL_ERROR".to_string()),
                            }),
                        );
                    }
                };

                let timestamp = status.verified_at_epoch.unwrap_or_else(now_epoch_seconds);

                return (
                    StatusCode::OK,
                    Json(VerifyInputResponse {
                        status: ResponseStatus::Verified,
                        request_id: request.request_id,
                        handles: Some(handles),
                        epoch_id: Some(epoch_id),
                        signature: Some(signature.as_bytes().to_vec().into()),
                        signer_address: Some(state.signer_address),
                        timestamp: Some(timestamp),
                        reason: None,
                        error_code: None,
                    }),
                );
            }

            if status.verified == Some(false) {
                let reason = status
                    .last_error
                    .unwrap_or_else(|| "ZKPoK verification failed".to_string());
                return (
                    StatusCode::OK,
                    Json(VerifyInputResponse {
                        status: ResponseStatus::Rejected,
                        request_id: request.request_id,
                        handles: None,
                        epoch_id: None,
                        signature: None,
                        signer_address: None,
                        timestamp: None,
                        reason: Some(reason),
                        error_code: Some("ZKPOK_INVALID".to_string()),
                    }),
                );
            }
        }
        Ok(None) => {}
        Err(e) => {
            error!(error = %e, request_id = %request.request_id, "Failed to fetch verification status");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(VerifyInputResponse {
                    status: ResponseStatus::Rejected,
                    request_id: request.request_id,
                    handles: None,
                    epoch_id: None,
                    signature: None,
                    signer_address: None,
                    timestamp: None,
                    reason: Some("Internal error".to_string()),
                    error_code: Some("INTERNAL_ERROR".to_string()),
                }),
            );
        }
    }

    info!(request_id = %request.request_id, "Verification request queued, returning pending status");

    (
        StatusCode::OK,
        Json(VerifyInputResponse {
            status: ResponseStatus::Pending,
            request_id: request.request_id,
            handles: None,
            epoch_id: None,
            signature: None,
            signer_address: Some(state.signer_address),
            timestamp: None,
            reason: None,
            error_code: None,
        }),
    )
}

pub async fn get_ciphertext_handler<P, A>(
    State(state): State<Arc<ApiState<P, A>>>,
    Path(handle): Path<String>,
) -> impl IntoResponse
where
    P: Provider<Ethereum> + Clone + Send + Sync + 'static,
    A: AwsS3Interface + Clone + Send + Sync + 'static,
{
    let handle_bytes: FixedBytes<32> = match handle.parse() {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CiphertextResponse {
                    status: CiphertextStatus::NotFound,
                    handle: FixedBytes::ZERO,
                    key_id: None,
                    sns_ciphertext: None,
                    sns_ciphertext_digest: None,
                    ciphertext_format: None,
                    epoch_id: None,
                    timestamp: None,
                    signature: None,
                    signer_address: None,
                    reason: Some("Invalid handle format".to_string()),
                }),
            );
        }
    };

    let s3_client = state.listener.aws_s3_client();
    match fetch_ciphertext(&state.db_pool, &s3_client, handle_bytes).await {
        Ok(Some(ct_data)) => {
                let signature = match sign_ciphertext_response(
                    state.signer.as_ref(),
                    &state.ciphertext_eip712_domain,
                    handle_bytes,
                    ct_data.key_id,
                    ct_data.sns_ciphertext_digest,
                    ct_data.epoch_id,
            ) {
                Ok(signature) => signature,
                Err(e) => {
                    error!(error = %e, handle = %handle, "Failed to sign ciphertext response");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CiphertextResponse {
                            status: CiphertextStatus::NotFound,
                            handle: handle_bytes,
                            key_id: None,
                            sns_ciphertext: None,
                            sns_ciphertext_digest: None,
                            ciphertext_format: None,
                            epoch_id: None,
                            timestamp: None,
                            signature: None,
                            signer_address: None,
                            reason: Some("Internal error".to_string()),
                        }),
                    );
                }
            };

            (
                StatusCode::OK,
                Json(CiphertextResponse {
                    status: CiphertextStatus::Found,
                    handle: handle_bytes,
                    key_id: Some(ct_data.key_id),
                    sns_ciphertext: Some(ct_data.sns_ciphertext.into()),
                    sns_ciphertext_digest: Some(ct_data.sns_ciphertext_digest),
                    ciphertext_format: Some(ct_data.ciphertext_format),
                    epoch_id: Some(ct_data.epoch_id),
                    timestamp: Some(ct_data.timestamp),
                    signature: Some(signature.as_bytes().to_vec().into()),
                    signer_address: Some(state.signer_address),
                    reason: None,
                }),
            )
        }
        Ok(None) => (
            StatusCode::OK,
            Json(CiphertextResponse {
                status: CiphertextStatus::NotFound,
                handle: handle_bytes,
                key_id: None,
                sns_ciphertext: None,
                sns_ciphertext_digest: None,
                ciphertext_format: None,
                epoch_id: None,
                timestamp: None,
                signature: None,
                signer_address: None,
                reason: Some("Ciphertext not stored by this coprocessor".to_string()),
            }),
        ),
        Err(e) => {
            error!(error = %e, handle = %handle, "Failed to fetch ciphertext");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CiphertextResponse {
                    status: CiphertextStatus::NotFound,
                    handle: handle_bytes,
                    key_id: None,
                    sns_ciphertext: None,
                    sns_ciphertext_digest: None,
                    ciphertext_format: None,
                    epoch_id: None,
                    timestamp: None,
                    signature: None,
                    signer_address: None,
                    reason: Some("Internal error".to_string()),
                }),
            )
        }
    }
}

pub async fn health_handler_v2<P, A>(
    State(state): State<Arc<ApiState<P, A>>>,
) -> impl IntoResponse
where
    P: Provider<Ethereum> + Clone + Send + Sync + 'static,
    A: AwsS3Interface + Clone + Send + Sync + 'static,
{
    let health = state.listener.health_check().await;

    let status = if health.healthy {
        HealthStatus::Healthy
    } else if health.database_connected || health.blockchain_connected {
        HealthStatus::Degraded
    } else {
        HealthStatus::Unhealthy
    };

    let last_block = get_last_processed_block(&state.db_pool).await.unwrap_or(0);
    let (stored_ciphertexts, pending_verifications) =
        get_stats(&state.db_pool).await.unwrap_or((0, 0));

    let uptime = state.start_time.elapsed().as_secs();

    let http_status = match status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    (
        http_status,
        Json(HealthResponse {
            status,
            version: env!("CARGO_PKG_VERSION").to_string(),
            signer_address: state.signer_address,
            uptime,
            last_block_processed: last_block,
            stored_ciphertexts: Some(stored_ciphertexts),
            pending_verifications: Some(pending_verifications),
        }),
    )
}

async fn insert_verify_proof_request(
    pool: &Pool<Postgres>,
    request: &VerifyInputRequest,
) -> anyhow::Result<()> {
    let request_id_bytes = request.request_id.to_be_bytes::<32>();
    let chain_id_i64: i64 = request.contract_chain_id.try_into().unwrap_or(i64::MAX);

    sqlx::query(
        r#"
        INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, input, extra_data, transaction_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT(zk_proof_id) DO NOTHING
        "#,
    )
    .bind(request_id_bytes.as_slice())
    .bind(chain_id_i64)
    .bind(request.contract_address.to_string())
    .bind(request.user_address.to_string())
    .bind(request.ciphertext_with_zkpok.as_ref())
    .bind(&[] as &[u8])
    .bind(&[] as &[u8])
    .execute(pool)
    .await?;

    sqlx::query("SELECT pg_notify('event_zkpok_new_work', '')")
        .execute(pool)
        .await?;

    Ok(())
}

struct InputVerificationRequestRecord {
    commitment: FixedBytes<32>,
    user_address: Address,
    contract_chain_id: U256,
    contract_address: Address,
    user_signature: Vec<u8>,
}

struct VerificationStatus {
    handles: Vec<u8>,
    verified: Option<bool>,
    last_error: Option<String>,
    verified_at_epoch: Option<u64>,
}

struct CiphertextData {
    key_id: U256,
    sns_ciphertext: Vec<u8>,
    sns_ciphertext_digest: FixedBytes<32>,
    ciphertext_format: i16,
    epoch_id: U256,
    timestamp: u64,
}

/// Fetches the SquashedNoise ciphertext from S3 (ct128 bucket) using the digest stored in ciphertext_digest table.
/// This is required because SNS worker stores ct128 in S3 and only keeps the digest in the database.
async fn fetch_ciphertext<A: AwsS3Interface>(
    pool: &Pool<Postgres>,
    s3_client: &A,
    handle: FixedBytes<32>,
) -> anyhow::Result<Option<CiphertextData>> {
    let handle_bytes = handle.as_slice();

    // Query the ciphertext_digest table for the ct128 digest and tenant's key_id
    let result = sqlx::query(
        r#"
        SELECT
            cd.ciphertext128 as ct128_digest,
            cd.ciphertext128_format,
            t.key_id,
            EXTRACT(EPOCH FROM cd.created_at)::bigint as created_at_epoch
        FROM ciphertext_digest cd
        JOIN ciphertexts ct ON cd.handle = ct.handle AND cd.tenant_id = ct.tenant_id
        JOIN tenants t ON ct.tenant_id = t.tenant_id
        WHERE cd.handle = $1
        "#,
    )
    .bind(handle_bytes)
    .fetch_optional(pool)
    .await?;

    match result {
        Some(row) => {
            let ct128_digest: Option<Vec<u8>> = row.get("ct128_digest");
            let ciphertext_format: i16 = row.get("ciphertext128_format");
            let key_id_vec: Option<Vec<u8>> = row.get("key_id");
            let created_at_epoch: i64 = row.get("created_at_epoch");

            // Check if ct128 digest exists (SNS worker has processed this ciphertext)
            let Some(ct128_digest) = ct128_digest else {
                warn!(handle = %handle, "ct128 digest not found - SNS worker may not have processed this ciphertext yet");
                return Ok(None);
            };

            let Some(key_id_vec) = key_id_vec else {
                anyhow::bail!("Missing key_id for ciphertext");
            };

            // Convert digest to hex for S3 key
            let s3_key = hex::encode(&ct128_digest);
            info!(handle = %handle, s3_key = %s3_key, "Fetching ct128 from S3");

            // Fetch the actual SquashedNoise ciphertext from S3 ct128 bucket
            let sns_ciphertext = match s3_client.get_object_by_key("ct128", &s3_key).await {
                Ok(bytes) => bytes.to_vec(),
                Err(e) => {
                    error!(handle = %handle, s3_key = %s3_key, error = %e, "Failed to fetch ct128 from S3");
                    anyhow::bail!("Failed to fetch ct128 from S3: {}", e);
                }
            };

            // Use the stored digest (not recomputed) for signature
            let digest_bytes: [u8; 32] = ct128_digest
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid ct128 digest length - expected 32 bytes"))?;

            let key_id_bytes: [u8; 32] = key_id_vec
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid key_id length"))?;

            let key_id = U256::from_be_bytes(key_id_bytes);

            info!(
                handle = %handle,
                key_id = %key_id,
                ciphertext_len = sns_ciphertext.len(),
                ciphertext_format = ciphertext_format,
                "Successfully fetched ct128 (SquashedNoise) from S3"
            );

            Ok(Some(CiphertextData {
                key_id,
                sns_ciphertext,
                sns_ciphertext_digest: FixedBytes::from(digest_bytes),
                ciphertext_format,
                epoch_id: key_id,
                timestamp: created_at_epoch as u64,
            }))
        }
        None => Ok(None),
    }
}

async fn get_last_processed_block(pool: &Pool<Postgres>) -> anyhow::Result<u64> {
    let result: Option<(Option<i64>,)> = sqlx::query_as(
        "SELECT last_block_num FROM gw_listener_last_block WHERE dummy_id = true"
    )
    .fetch_optional(pool)
    .await?;

    Ok(result
        .and_then(|r| r.0)
        .map(|n| n as u64)
        .unwrap_or(0))
}

async fn get_stats(pool: &Pool<Postgres>) -> anyhow::Result<(u64, u64)> {
    let ct_result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ciphertexts")
        .fetch_one(pool)
        .await?;
    let ciphertexts = ct_result.0 as u64;

    let pending_result: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM verify_proofs WHERE verified IS NULL"
    )
    .fetch_one(pool)
    .await?;
    let pending = pending_result.0 as u64;

    Ok((ciphertexts, pending))
}

async fn fetch_input_verification_request(
    pool: &Pool<Postgres>,
    request_id: U256,
) -> anyhow::Result<Option<InputVerificationRequestRecord>> {
    let request_id_bytes = request_id.to_be_bytes::<32>();
    let result = sqlx::query(
        "SELECT commitment, user_address, contract_chain_id, contract_address, user_signature \
        FROM input_verification_requests WHERE request_id = $1",
    )
    .bind(request_id_bytes.as_slice())
    .fetch_optional(pool)
    .await?;

    let Some(row) = result else {
        return Ok(None);
    };

    let commitment_vec: Vec<u8> = row.get("commitment");
    let user_address_str: String = row.get("user_address");
    let contract_chain_id: i64 = row.get("contract_chain_id");
    let contract_address_str: String = row.get("contract_address");
    let user_signature: Vec<u8> = row.get("user_signature");

    let commitment_bytes: [u8; 32] = commitment_vec
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid commitment length"))?;

    Ok(Some(InputVerificationRequestRecord {
        commitment: FixedBytes::from(commitment_bytes),
        user_address: Address::from_str(&user_address_str)?,
        contract_chain_id: U256::from(contract_chain_id as u64),
        contract_address: Address::from_str(&contract_address_str)?,
        user_signature,
    }))
}

async fn fetch_verification_status(
    pool: &Pool<Postgres>,
    request_id: U256,
) -> anyhow::Result<Option<VerificationStatus>> {
    let request_id_bytes = request_id.to_be_bytes::<32>();
    let result = sqlx::query(
        "SELECT handles, verified, last_error, \
        EXTRACT(EPOCH FROM verified_at)::bigint as verified_at_epoch \
        FROM verify_proofs WHERE zk_proof_id = $1",
    )
    .bind(request_id_bytes.as_slice())
    .fetch_optional(pool)
    .await?;

    let Some(row) = result else {
        return Ok(None);
    };

    let handles: Option<Vec<u8>> = row.get("handles");
    let verified: Option<bool> = row.get("verified");
    let last_error: Option<String> = row.get("last_error");
    let verified_at_epoch: Option<i64> = row.get("verified_at_epoch");

    Ok(Some(VerificationStatus {
        handles: handles.unwrap_or_default(),
        verified,
        last_error,
        verified_at_epoch: verified_at_epoch.map(|v| v as u64),
    }))
}

async fn fetch_epoch_id(pool: &Pool<Postgres>, chain_id: U256) -> anyhow::Result<Option<U256>> {
    let chain_id_i64: i64 = chain_id.to::<i64>();
    let result = sqlx::query("SELECT key_id FROM tenants WHERE chain_id = $1")
        .bind(chain_id_i64)
        .fetch_optional(pool)
        .await?;

    let Some(row) = result else {
        return Ok(None);
    };

    let key_id_vec: Option<Vec<u8>> = row.get("key_id");
    let Some(key_id_vec) = key_id_vec else {
        return Ok(None);
    };

    let key_id_bytes: [u8; 32] = key_id_vec
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid key_id length"))?;

    Ok(Some(U256::from_be_bytes(key_id_bytes)))
}

fn parse_handles(handles_bytes: &[u8]) -> anyhow::Result<Vec<FixedBytes<32>>> {
    if handles_bytes.is_empty() {
        return Ok(vec![]);
    }
    if handles_bytes.len() % 32 != 0 {
        anyhow::bail!("Invalid handles length");
    }

    Ok(handles_bytes
        .chunks_exact(32)
        .map(|chunk| FixedBytes::<32>::from_slice(chunk))
        .collect())
}

fn request_matches(
    record: &InputVerificationRequestRecord,
    request: &VerifyInputRequest,
) -> bool {
    record.commitment == request.commitment
        && record.contract_chain_id == request.contract_chain_id
        && record.contract_address == request.contract_address
        && record.user_address == request.user_address
        && record.user_signature.as_slice() == request.user_signature.as_ref()
}

fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
