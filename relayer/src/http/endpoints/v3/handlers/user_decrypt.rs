//! v3 `/v3/user-decrypt` handler (unified user-decryption).
//!
//! POST validates the typed-attestation envelope, converts it to the
//! shared `UserDecryptRequest` with `UserDecryptPayload::Unified`, runs the
//! same dedup/queue pipeline as v2, and dispatches a
//! `RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser)`
//! event. From there the orchestrator funnels the job through the unified
//! `userDecryptionRequest(HandleEntry[], …)` calldata builder and the
//! shared receipt-handling path. GET is delegated verbatim to the v2
//! handler since the response schema is unchanged.
//!
//! TODO(#1682): Solana clients currently fetch MMR proofs from the standalone
//! `solana-proof-service` (`GET /internal/solana/mmr-proof`) and embed them in
//! `extraData` before calling this route. The relayer does not own proof
//! construction; optional in-process proof fetch remains a product gap.

use crate::core::event::{
    ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptRequest,
};
use crate::core::job_id::JobId;
use crate::host::{HostChainIdChecker, SigPreCheckError, UserDecryptSignaturePreChecker};
use crate::http::endpoints::v2::handlers::user_decrypt::UserDecryptHandler as UserDecryptHandlerV2;
use crate::http::endpoints::v2::types::error::{ApiResponseStatus, RelayerV2ResponseFailed};
use crate::http::endpoints::v2::types::user_decrypt::{
    UserDecryptPostResponseJson, UserDecryptQueuedResult,
};
use crate::http::endpoints::v3::types::AttestedUserDecryptRequestJson;
use crate::http::retry_after::{DecryptQueueInfo, RetryAfterState};
use crate::http::utils::BounceChecker;
use crate::http::{parse_and_validate, AppResponse};
use crate::logging::UserDecryptStep;
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::metrics::{
    observe_raw_eta_seconds, observe_signature_precheck, HttpApiVersion, RetryAfterRequestType,
    SignaturePreCheckOutcome,
};
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::readiness::throttler::UserDecryptReadinessTask;
use crate::store::sql::models::user_decrypt_req_model::UserDecryptReqData;
use crate::store::sql::repositories::user_decrypt_repo::{
    UserDecryptInsertResult, UserDecryptRepository,
};
use axum::body::Bytes as AxumBytes;
use axum::extract::{FromRequest, Path};
use axum::http::{header, HeaderMap, Request, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use tracing::{error, info, instrument, span, warn, Level};
use uuid::Uuid;

pub type UserDecryptResponse = AppResponse<UserDecryptPostResponseJson>;

/// v3 `/v3/user-decrypt` handler. Shares the orchestrator + repo + queue
/// state with the v2 handler so v2 and v3 jobs flow through the same
/// post-submission pipeline.
pub struct UserDecryptHandler {
    orchestrator: Arc<Orchestrator>,
    api_version: ApiVersion,
    user_decrypt_repo: Arc<UserDecryptRepository>,
    user_decrypt_queue_checker: BounceChecker<UserDecryptReadinessTask>,
    retry_after_state: Arc<RetryAfterState>,
    host_chain_id_checker: Arc<HostChainIdChecker>,
    signature_prechecker: Arc<UserDecryptSignaturePreChecker>,
    /// GET is delegated to the v2 handler whose response schema is shared.
    v2_handler: Arc<UserDecryptHandlerV2>,
}

impl UserDecryptHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        orchestrator: Arc<Orchestrator>,
        api_version: ApiVersion,
        user_decrypt_repo: Arc<UserDecryptRepository>,
        user_decrypt_queue_checker: BounceChecker<UserDecryptReadinessTask>,
        retry_after_state: Arc<RetryAfterState>,
        host_chain_id_checker: Arc<HostChainIdChecker>,
        signature_prechecker: Arc<UserDecryptSignaturePreChecker>,
        v2_handler: Arc<UserDecryptHandlerV2>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            user_decrypt_repo,
            user_decrypt_queue_checker,
            retry_after_state,
            host_chain_id_checker,
            signature_prechecker,
            v2_handler,
        }
    }

    /// Create router with the v3 user-decrypt routes.
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new()
            .route(
                "/v3/user-decrypt",
                axum::routing::post({
                    let handler = self.clone();
                    move |req| async move { handler.user_decrypt_post_v3(req).await }
                }),
            )
            .route(
                "/v3/user-decrypt/{job_id}",
                axum::routing::get({
                    let handler = self;
                    move |path, headers: HeaderMap| async move {
                        handler.user_decrypt_get_v3(path, headers).await
                    }
                }),
            )
    }

    /// POST /v3/user-decrypt
    pub async fn user_decrypt_post_v3(&self, req: Request<axum::body::Body>) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::UserDecrypt,
            HttpMethod::Post,
            HttpApiVersion::V3,
            req.headers().clone(),
            async move { self.handle_post(req, &()).await },
        )
        .await
        .into_response()
    }

    /// GET /v3/user-decrypt/{job_id}
    ///
    /// Delegates to the v2 handler — the GET response schema (status +
    /// request_id + result shares) is unchanged across versions.
    pub async fn user_decrypt_get_v3(
        &self,
        Path(job_id): Path<Uuid>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::UserDecrypt,
            HttpMethod::Get,
            HttpApiVersion::V3,
            headers,
            async move { self.v2_handler.handle_get(job_id).await },
        )
        .await
        .into_response()
    }

    #[instrument(name = "handle-v3-user-decrypt-post", skip_all, fields(request_id))]
    async fn handle_post<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = Uuid::new_v4();
        let _span = span!(Level::INFO, "handle-v3-user-decrypt-post-req", request_id = %request_id);

        info!(
            step = %UserDecryptStep::ReqReceived,
            request_id = %request_id,
            "Handling v3 user decryption POST request"
        );

        let body = match AxumBytes::from_request(req, _state).await {
            Ok(body) => body,
            Err(_) => {
                return RelayerV2ResponseFailed::request_error(
                    "Failed to read request body",
                    &request_id.to_string(),
                )
                .into_response();
            }
        };

        // The attestation type selects the EVM or Solana unified request variant.
        let user_decrypt_request: UserDecryptRequest =
            match parse_and_validate::<AttestedUserDecryptRequestJson, UserDecryptRequest>(&body) {
                Ok(request) => request,
                Err(parse_error) => {
                    return RelayerV2ResponseFailed::from_parse_error(
                        &parse_error,
                        &request_id.to_string(),
                    )
                    .into_response();
                }
            };

        info!("Successfully parsed and validated v3 request");

        // Early host-chain rejection: handle prefixes encode the chain id.
        if let Err(chain_id) = self
            .host_chain_id_checker
            .validate_u256_handles(user_decrypt_request.ct_handles())
        {
            return RelayerV2ResponseFailed::host_chain_id_not_supported(
                chain_id,
                &request_id.to_string(),
            )
            .into_response();
        }

        // Signature pre-check: reject detectably-bad signatures here so the SDK caller gets
        // early feedback instead of waiting for the gateway/KMS round-trip. The KMS Connector
        // remains the authoritative verifier.
        match self
            .signature_prechecker
            .verify(&user_decrypt_request)
            .await
        {
            Ok(()) => observe_signature_precheck(SignaturePreCheckOutcome::Accepted),
            Err(SigPreCheckError::Invalid { signer, reason }) => {
                info!(
                    signer = %signer,
                    reason = %reason,
                    request_id = %request_id,
                    "v3 user-decrypt signature pre-check rejected request"
                );
                observe_signature_precheck(SignaturePreCheckOutcome::Rejected);
                return RelayerV2ResponseFailed::invalid_signature(
                    &reason,
                    &request_id.to_string(),
                )
                .into_response();
            }
            Err(SigPreCheckError::HostCallFailed(msg)) => {
                warn!(
                    error = %msg,
                    request_id = %request_id,
                    "v3 user-decrypt signature pre-check host call failed; rejecting"
                );
                observe_signature_precheck(SignaturePreCheckOutcome::HostCallFailed);
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        }

        let int_job_id: JobId = user_decrypt_request.content_hash().into();

        // Queue-full bouncing.
        let active_external_job_id = self
            .user_decrypt_repo
            .find_active_ext_ref_by_int_job_id(int_job_id.as_ref())
            .await;

        match active_external_job_id {
            Ok(res) => {
                if res.is_none() {
                    if let Err(retry_after) = self.user_decrypt_queue_checker.check().await {
                        info!(
                            step = %UserDecryptStep::Bounced,
                            int_job_id = ?int_job_id,
                            "v3 user decrypt bounced by full queue"
                        );
                        return RelayerV2ResponseFailed::protocol_overloaded(
                            "relayer is currently processing too many requests",
                            &retry_after.to_string(),
                            &request_id.to_string(),
                        )
                        .into_response();
                    }
                }
            }
            Err(e) => {
                error!(
                    int_job_id = ?int_job_id,
                    "Failed to insert/get v3 user decrypt into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        }

        let proposed_ext_job_id = self.orchestrator.new_ext_job_id();

        let user_decrypt_request_data = UserDecryptReqData::Unified(user_decrypt_request.clone());

        let insert_result = match self
            .user_decrypt_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                int_job_id.as_ref(),
                user_decrypt_request_data,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                error!(
                    int_job_id = ?int_job_id,
                    "Failed to insert/get v3 user decrypt into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        };

        let assigned_ext_job_id = match &insert_result {
            UserDecryptInsertResult::Inserted { ext_job_id } => *ext_job_id,
            UserDecryptInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            UserDecryptInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        if matches!(insert_result, UserDecryptInsertResult::Inserted { .. }) {
            let request_data = UserDecryptEventData::ReqRcvdFromUser {
                decrypt_request: user_decrypt_request,
            };
            let event = RelayerEvent::new(
                int_job_id,
                self.api_version,
                RelayerEventData::UserDecrypt(request_data),
            );

            if let Err(e) = self.orchestrator.dispatch_event(event).await {
                error!("Failed to dispatch v3 event to orchestrator: {:?}", e);
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
            info!(
                step = %UserDecryptStep::Queued,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = ?int_job_id,
                "Dispatched v3 event to orchestrator"
            );
        } else {
            info!(
                step = %UserDecryptStep::DedupHit,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = ?int_job_id,
                "Duplicate v3 request detected"
            );
        }

        let request_id_for_response = uuid::Uuid::new_v4();

        let readiness_queue_info = self
            .user_decrypt_queue_checker
            .readiness_throttler()
            .get_queue_info()
            .await;
        let tx_queue_info = self
            .user_decrypt_queue_checker
            .tx_throttler()
            .get_queue_info()
            .await;
        let decrypt_queue_info = DecryptQueueInfo::new(readiness_queue_info, tx_queue_info);
        let retry_after = self
            .retry_after_state
            .compute_for_decrypt_post(&decrypt_queue_info, true)
            .await;

        let raw_eta_ms = self
            .retry_after_state
            .compute_raw_eta_ms_for_decrypt(&decrypt_queue_info, true)
            .await;
        observe_raw_eta_seconds(
            RetryAfterRequestType::UserDecrypt,
            raw_eta_ms as f64 / 1000.0,
        );

        info!(
            req_id = %request_id_for_response,
            int_job_id = ?int_job_id,
            ext_job_id = %assigned_ext_job_id,
            retry_after_secs = retry_after,
            "Computed retry-after for v3 user decrypt POST"
        );

        let status_code = StatusCode::ACCEPTED;
        let response = UserDecryptPostResponseJson {
            status: ApiResponseStatus::Queued,
            request_id: request_id_for_response.to_string(),
            result: UserDecryptQueuedResult {
                job_id: assigned_ext_job_id.to_string(),
            },
        };

        info!(
            request_id = %request_id_for_response,
            http_status = status_code.as_u16(),
            ext_job_id = %assigned_ext_job_id,
            "HTTP response (v3)"
        );

        (
            status_code,
            [(header::RETRY_AFTER, retry_after.to_string())],
            Json(response),
        )
            .into_response()
    }
}

// OpenAPI documented endpoints as standalone functions.
/// Submit a v3 (unified EIP-712) user-decryption request.
#[utoipa::path(
    post,
    path = "/v3/user-decrypt",
    request_body = AttestedUserDecryptRequestJson,
    responses(
        (status = 202, description = "Request accepted for processing.", body = UserDecryptPostResponseJson),
        (status = 400, description = "Invalid request", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 429, description = "Rate limited", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
    ),
    tag = "User Decrypt v3"
)]
pub async fn user_decrypt_post_v3(
    handler: Arc<UserDecryptHandler>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    handler.user_decrypt_post_v3(req).await
}

/// Check v3 user-decryption status.
#[utoipa::path(
    get,
    path = "/v3/user-decrypt/{job_id}",
    params(
        ("job_id" = String, Path, format = "uuid", description = "Job ID returned from POST request")
    ),
    responses(
        (status = 200, description = "Completed.", body = crate::http::endpoints::v2::types::user_decrypt::UserDecryptSucceededStatusResponse),
        (status = 202, description = "Still processing. Poll again after Retry-After.", body = crate::http::endpoints::v2::types::error::V2StatusQueued,
            example = json!({"status": "queued", "requestId": "550e8400-e29b-41d4-a716-446655440000"})
        ),
        (status = 400, description = "Request failed", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
        (status = 404, description = "Not found", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
        (status = 503, description = "Service unavailable", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
    ),
    tag = "User Decrypt v3"
)]
pub async fn user_decrypt_get_v3(
    handler: Arc<UserDecryptHandler>,
    Path(job_id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    handler.user_decrypt_get_v3(Path(job_id), headers).await
}
