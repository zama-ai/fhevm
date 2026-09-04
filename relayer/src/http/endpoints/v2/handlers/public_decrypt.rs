use super::super::types::error::{
    classify_revert_error, ApiResponseStatus, RelayerV2ResponseFailed, V2ErrorResponseBody,
};
use super::super::types::public_decrypt::{
    PublicDecryptPostResponseJson, PublicDecryptQueuedResult, PublicDecryptRequestJson,
    PublicDecryptResponseJson, PublicDecryptStatusResponseJson,
};
use crate::core::errors::{
    HOST_ACL_FAILED_PREFIX, NOT_ALLOWED_ON_HOST_ACL_PREFIX, READINESS_CHECK_TIMEOUT_MSG,
    TIMEOUT_REASON_MISSING_MSG,
};
use crate::core::event::{
    ApiVersion, PublicDecryptEventData, PublicDecryptRequest, RelayerEvent, RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::host::HostChainIdChecker;
use crate::http::retry_after::{
    DecryptQueueInfo, ReadinessQueueInfo, RequestStateInfo, RetryAfterState, TxQueueInfo,
};
use crate::http::utils::BounceChecker;
use crate::http::{parse_and_validate, AppResponse};
use crate::logging::PublicDecryptStep;
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::metrics::{observe_raw_eta_seconds, HttpApiVersion, RetryAfterRequestType};
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::readiness::throttler::PublicDecryptReadinessTask;
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::repositories::public_decrypt_repo::{
    PublicDecryptInsertResult, PublicDecryptRepository,
};
use axum::http::HeaderMap;
use axum::{
    body::Bytes as AxumBytes,
    extract::{FromRequest, Path},
    http::Request,
    response::IntoResponse,
};
use axum::{
    http::{header, StatusCode},
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info, instrument, span, Level};
use uuid::Uuid;

pub type PublicDecryptResponse = AppResponse<PublicDecryptPostResponseJson>;

pub struct PublicDecryptHandler {
    orchestrator: Arc<Orchestrator>,
    api_version: ApiVersion,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
    bounce_checker: BounceChecker<PublicDecryptReadinessTask>,
    retry_after_state: Arc<RetryAfterState>,
    host_chain_id_checker: Arc<HostChainIdChecker>,
}

impl PublicDecryptHandler {
    pub fn new(
        orchestrator: Arc<Orchestrator>,
        api_version: ApiVersion,
        public_decrypt_repo: Arc<PublicDecryptRepository>,
        bounce_checker: BounceChecker<PublicDecryptReadinessTask>,
        retry_after_state: Arc<RetryAfterState>,
        host_chain_id_checker: Arc<HostChainIdChecker>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            public_decrypt_repo,
            bounce_checker,
            retry_after_state,
            host_chain_id_checker,
        }
    }

    /// Create router with public decrypt v2 routes
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new()
            .route(
                "/v2/public-decrypt",
                axum::routing::post({
                    let handler = self.clone();
                    move |req| async move { handler.public_decrypt_post_v2(req).await }
                }),
            )
            .route(
                "/v2/public-decrypt/{job_id}",
                axum::routing::get({
                    let handler = self;
                    move |path, headers: HeaderMap| async move {
                        handler.public_decrypt_get_v2(path, headers).await
                    }
                }),
            )
    }

    /// Submit public decryption.
    pub async fn public_decrypt_post_v2(
        &self,
        req: Request<axum::body::Body>,
    ) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::PublicDecrypt,
            HttpMethod::Post,
            HttpApiVersion::V2,
            req.headers().clone(),
            async move { self.handle_post(req, &()).await },
        )
        .await
        .into_response()
    }

    /// GET /v2/public-decrypt/<job_id> - Check status and get result
    pub async fn public_decrypt_get_v2(
        &self,
        Path(job_id): Path<Uuid>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::PublicDecrypt,
            HttpMethod::Get,
            HttpApiVersion::V2,
            headers,
            async move { self.handle_get(job_id).await },
        )
        .await
        .into_response()
    }

    #[instrument(name = "handle-public-decrypt-post", skip_all, fields(request_id))]
    pub async fn handle_post<S>(
        &self,
        req: Request<axum::body::Body>,
        _state: &S,
    ) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = Uuid::new_v4();
        let _span = span!(Level::INFO, "handle-public-decrypt-post-req", request_id = %request_id);

        info!(
            step = %PublicDecryptStep::ReqReceived,
            request_id = %request_id,
            "Handling public decryption POST request"
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

        let request: PublicDecryptRequest =
            match parse_and_validate::<PublicDecryptRequestJson, PublicDecryptRequest>(&body) {
                Ok(request) => request,
                Err(parse_error) => {
                    return RelayerV2ResponseFailed::from_parse_error(
                        &parse_error,
                        &request_id.to_string(),
                    )
                    .into_response();
                }
            };

        info!("Successfully parsed and validated request");

        // Check early to avoid filling the queue with handles of unsupported chains
        if let Err(chain_id) = self
            .host_chain_id_checker
            .validate_handles(&request.ct_handles)
        {
            return RelayerV2ResponseFailed::host_chain_id_not_supported(
                chain_id,
                &request_id.to_string(),
            )
            .into_response();
        }

        let int_job_id: JobId = request.content_hash().into();

        // Queue full Bouncing logic.
        let active_external_job_id = self
            .public_decrypt_repo
            .find_active_ext_ref_by_int_job_id(int_job_id.as_ref())
            .await;

        match active_external_job_id {
            Ok(res) => {
                if res.is_none() {
                    // In this case, we check queue full and bounce the request with 429
                    if let Err(retry_after) = self.bounce_checker.check().await {
                        info!(
                            step = %PublicDecryptStep::Bounced,
                            int_job_id = ?int_job_id,
                            "Public decrypt v2 is bounced by full queue"
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
                    "Failed to insert/get public decrypt into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        }

        let proposed_ext_job_id = self.orchestrator.new_ext_job_id();

        // One gate read for both decisions - see `dispatch_epoch`.
        let dispatch_epoch = self.public_decrypt_repo.dispatch_epoch();

        let insert_outcome = match self
            .public_decrypt_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                int_job_id.as_ref(),
                request.clone(),
                dispatch_epoch,
            )
            .await
        {
            Ok(outcome) => outcome,
            Err(e) => {
                error!(
                    "Failed to insert/get public decrypt into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        };

        // Extract ext_job_id from any variant
        let assigned_ext_job_id = match &insert_outcome.result {
            PublicDecryptInsertResult::Inserted { ext_job_id } => *ext_job_id,
            PublicDecryptInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            PublicDecryptInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        // Only dispatch event for new requests (deduplication)
        if matches!(
            insert_outcome.result,
            PublicDecryptInsertResult::Inserted { .. }
        ) {
            let event_data = PublicDecryptEventData::ReqRcvdFromUser {
                decrypt_request: request.clone(),
            };

            let event = RelayerEvent::new(
                int_job_id,
                self.api_version,
                RelayerEventData::PublicDecrypt(event_data),
            );

            // Only the confirmed dispatcher drives what it accepted. On any other pod
            // the row stays durable and unowned, so the holder's sweep claims it on its
            // next tick; dispatching here would make this pod a second dispatcher for a
            // request the holder is about to drive.
            if dispatch_epoch.is_some() {
                if let Err(e) = self.orchestrator.dispatch_event(event).await {
                    error!("Failed to dispatch event to orchestrator: {:?}", e);
                    return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                        .into_response();
                }

                info!(
                    step = %PublicDecryptStep::Queued,
                    req_id = %request_id,
                    ext_job_id = %assigned_ext_job_id,
                    int_job_id = ?int_job_id,
                    "Dispatched event to orchestrator"
                );
            } else {
                info!(
                    step = %PublicDecryptStep::Queued,
                    req_id = %request_id,
                    ext_job_id = %assigned_ext_job_id,
                    int_job_id = ?int_job_id,
                    "Accepted while not the dispatcher, left for the sweep to drive"
                );
            }
        } else {
            info!(
                step = %PublicDecryptStep::DedupHit,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = ?int_job_id,
                "Duplicate request detected"
            );
        }

        // Generate a new request_id for this HTTP request (not stored)
        let request_id_for_response = uuid::Uuid::new_v4();

        // Sizes come from the INSERT above, so both pods agree. max_concurrency and drain rate
        // are startup config, identical on both pods. Position is None: the request just joined
        // the back of both queues.
        let readiness_queue_info = ReadinessQueueInfo {
            size: insert_outcome.readiness_queue_size as usize,
            max_concurrency: self.bounce_checker.readiness_throttler().max_concurrency(),
            position: None,
        };
        let tx_queue_info = TxQueueInfo {
            size: insert_outcome.tx_queue_size as usize,
            drain_rate_tps: self.bounce_checker.tx_throttler().current_tps(),
            position: None,
        };
        let decrypt_queue_info = DecryptQueueInfo::new(readiness_queue_info, tx_queue_info);
        let retry_after = self
            .retry_after_state
            .compute_for_decrypt_post(
                &decrypt_queue_info,
                false, // is_user_decrypt
            )
            .await;

        // Record raw ETA for POST histogram metrics
        let raw_eta_ms = self
            .retry_after_state
            .compute_raw_eta_ms_for_decrypt(&decrypt_queue_info, false)
            .await;
        observe_raw_eta_seconds(
            RetryAfterRequestType::PublicDecrypt,
            raw_eta_ms as f64 / 1000.0,
        );

        info!(
            req_id = %request_id_for_response,
            int_job_id = ?int_job_id,
            ext_job_id = %assigned_ext_job_id,
            retry_after_secs = retry_after,
            "Computed retry-after for public decrypt POST"
        );

        let status_code = StatusCode::ACCEPTED;
        let response = PublicDecryptPostResponseJson {
            status: ApiResponseStatus::Queued,
            request_id: request_id_for_response.to_string(),
            result: PublicDecryptQueuedResult {
                job_id: assigned_ext_job_id.to_string(),
            },
        };

        info!(
            request_id = %request_id_for_response,
            http_status = status_code.as_u16(),
            ext_job_id = %assigned_ext_job_id,
            "HTTP response"
        );

        // Add Retry-After header with the dynamically computed retry value
        (
            status_code,
            [(header::RETRY_AFTER, retry_after.to_string())],
            Json(response),
        )
            .into_response()
    }

    #[instrument(name = "handle-public-decrypt-get", skip_all, fields(job_id))]
    pub async fn handle_get(&self, job_id: Uuid) -> impl IntoResponse {
        // Generate a new request_id for this HTTP request
        let request_id = uuid::Uuid::new_v4();

        info!(
            ext_job_id = %job_id,
            request_id = %request_id,
            "Handling public decryption GET request"
        );

        // Check SQL for current status using job_id (which is the external_reference_id in DB)
        match self
            .public_decrypt_repo
            .find_status_and_res_by_ext_id(job_id)
            .await
        {
            Ok(Some(response_model)) => {
                match response_model.req_status {
                    ReqStatus::Completed => {
                        if let Some(res) = response_model.res {
                            // Deserialize from database JsonValue to core event type, then convert to API response
                            if let Ok(core_response) = serde_json::from_value::<
                                crate::core::event::PublicDecryptResponse,
                            >(res)
                            {
                                let status_code = StatusCode::OK;
                                let api_response = PublicDecryptResponseJson::from(core_response);

                                info!(
                                    request_id = %request_id,
                                    http_status = status_code.as_u16(),
                                    ext_job_id = %job_id,
                                    "HTTP response"
                                );

                                (
                                    status_code,
                                    Json(PublicDecryptStatusResponseJson {
                                        status: ApiResponseStatus::Succeeded,
                                        request_id: request_id.to_string(), // Per-request UUID
                                        result: Some(api_response),
                                        error: None,
                                    }),
                                )
                                    .into_response()
                            } else {
                                error!("Failed to deserialize response from database");
                                (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(PublicDecryptStatusResponseJson {
                                        status: ApiResponseStatus::Failed,
                                        request_id: request_id.to_string(),
                                        result: None,
                                        error: Some(V2ErrorResponseBody::internal_server_error(
                                            "Failed to deserialize response data",
                                        )),
                                    }),
                                )
                                    .into_response()
                            }
                        } else {
                            error!("Request marked as completed but no response data found");
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(PublicDecryptStatusResponseJson {
                                    status: ApiResponseStatus::Failed,
                                    request_id: request_id.to_string(),
                                    result: None,
                                    error: Some(V2ErrorResponseBody::internal_server_error(
                                        "Internal error: completed request missing response data",
                                    )),
                                }),
                            )
                                .into_response()
                        }
                    }
                    ReqStatus::TimedOut => {
                        let error_msg = match response_model.err_reason {
                            Some(reason) => reason,
                            None => {
                                error!(
                                    request_id = %request_id,
                                    job_id = ?response_model.ext_job_id,
                                    "TimedOut request missing error reason in database"
                                );
                                TIMEOUT_REASON_MISSING_MSG.to_string()
                            }
                        };
                        let error_value = if error_msg == READINESS_CHECK_TIMEOUT_MSG {
                            V2ErrorResponseBody::readiness_check_timed_out(&error_msg)
                        } else {
                            V2ErrorResponseBody::response_timed_out(&error_msg)
                        };
                        (
                            StatusCode::SERVICE_UNAVAILABLE,
                            Json(PublicDecryptStatusResponseJson {
                                status: ApiResponseStatus::Failed,
                                request_id: request_id.to_string(),
                                result: None,
                                error: Some(error_value),
                            }),
                        )
                            .into_response()
                    }
                    ReqStatus::Failure => {
                        let error_msg = match response_model.err_reason {
                            Some(reason) => reason,
                            None => {
                                error!(
                                    alert = true,
                                    request_id = %request_id,
                                    "Failure request missing error reason in database"
                                );
                                "Unknown error".to_string()
                            }
                        };

                        // Classify host ACL errors before falling through to revert classification
                        let (status_code, error_value) =
                            if error_msg.starts_with(NOT_ALLOWED_ON_HOST_ACL_PREFIX) {
                                (
                                    StatusCode::BAD_REQUEST,
                                    V2ErrorResponseBody::not_allowed_on_host_acl(&error_msg),
                                )
                            } else if error_msg.starts_with(HOST_ACL_FAILED_PREFIX) {
                                (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    V2ErrorResponseBody::host_acl_failed(&error_msg),
                                )
                            } else {
                                classify_revert_error(&error_msg)
                            };

                        (
                            status_code,
                            Json(PublicDecryptStatusResponseJson {
                                status: ApiResponseStatus::Failed,
                                request_id: request_id.to_string(),
                                result: None,
                                error: Some(error_value),
                            }),
                        )
                            .into_response()
                    }
                    ReqStatus::Queued
                    | ReqStatus::Processing
                    | ReqStatus::TxInFlight
                    | ReqStatus::ReceiptReceived => {
                        // Request is still in progress, return 202 with dynamic Retry-After header
                        info!("Request still in progress, returning queued status");

                        // `updated_at` dates the current status; elapsed time drives the
                        // copro/KMS backoff escalation. A hardcoded zero pinned it to the first
                        // interval.
                        let elapsed_in_state_secs = (Utc::now() - response_model.updated_at)
                            .num_seconds()
                            .clamp(0, u32::MAX as i64)
                            as u32;
                        let state_info =
                            RequestStateInfo::new(response_model.req_status, elapsed_in_state_secs);

                        // Position comes from the same SELECT, not a per-pod throttler (empty on
                        // the passive pod). `Queued` is the readiness queue, everything else the
                        // TX queue - matching `get_decrypt_stage`. A queued request has not
                        // reached the TX queue, so that side takes the measured depth.
                        let (readiness_position, tx_position) = match response_model.req_status {
                            ReqStatus::Queued => (Some(response_model.queue_position), None),
                            _ => (None, Some(response_model.queue_position)),
                        };
                        let readiness_queue_info = ReadinessQueueInfo {
                            size: response_model.queue_position as usize,
                            max_concurrency: self
                                .bounce_checker
                                .readiness_throttler()
                                .max_concurrency(),
                            position: readiness_position.map(|p| p as usize),
                        };
                        let tx_queue_info = TxQueueInfo {
                            size: response_model.tx_queue_size as usize,
                            drain_rate_tps: self.bounce_checker.tx_throttler().current_tps(),
                            position: tx_position.map(|p| p as usize),
                        };
                        let decrypt_queue_info =
                            DecryptQueueInfo::new(readiness_queue_info, tx_queue_info);

                        let retry_after = self
                            .retry_after_state
                            .compute_for_decrypt_get(
                                &decrypt_queue_info,
                                &state_info,
                                false, // is_user_decrypt
                            )
                            .await;

                        let status_code = StatusCode::ACCEPTED;

                        info!(
                            req_id = %request_id,
                            ext_job_id = %job_id,
                            retry_after_secs = retry_after,
                            status = ?response_model.req_status,
                            "Computed retry-after for public decrypt GET"
                        );

                        info!(
                            request_id = %request_id,
                            http_status = status_code.as_u16(),
                            ext_job_id = %job_id,
                            "HTTP response"
                        );

                        (
                            status_code,
                            [(header::RETRY_AFTER, retry_after.to_string())],
                            Json(PublicDecryptStatusResponseJson {
                                status: ApiResponseStatus::Queued,
                                request_id: request_id.to_string(),
                                result: None,
                                error: None,
                            }),
                        )
                            .into_response()
                    }
                }
            }
            Ok(None) => (
                StatusCode::NOT_FOUND,
                Json(PublicDecryptStatusResponseJson {
                    status: ApiResponseStatus::Failed,
                    request_id: request_id.to_string(),
                    result: None,
                    error: Some(V2ErrorResponseBody::not_found("Request not found")),
                }),
            )
                .into_response(),
            Err(e) => {
                error!("Database error while checking status: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(PublicDecryptStatusResponseJson {
                        status: ApiResponseStatus::Failed,
                        request_id: request_id.to_string(),
                        result: None,
                        error: Some(V2ErrorResponseBody::internal_server_error("Database error")),
                    }),
                )
                    .into_response()
            }
        }
    }
}

// OpenAPI documented endpoints as standalone functions
/// Submit public decryption.
#[utoipa::path(
    post,
    path = "/v2/public-decrypt",
    request_body = PublicDecryptRequestJson,
    responses(
        (status = 202, description = "Request accepted for processing.", body = PublicDecryptPostResponseJson),
        (status = 400, description = "Invalid request", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 429, description = "Rate limited", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
    ),
    tag = "Public Decrypt"
)]
pub async fn public_decrypt_post_v2(
    handler: Arc<PublicDecryptHandler>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    handler.public_decrypt_post_v2(req).await
}

/// Check public decryption status.
#[utoipa::path(
    get,
    path = "/v2/public-decrypt/{job_id}",
    params(
        ("job_id" = String, Path, format = "uuid", description = "Job ID returned from POST request")
    ),
    responses(
        (status = 200, description = "Completed.", body = crate::http::endpoints::v2::types::public_decrypt::PublicDecryptSucceededStatusResponse),
        (status = 202, description = "Still processing. Poll again after Retry-After.", body = crate::http::endpoints::v2::types::error::V2StatusQueued,
            example = json!({"status": "queued", "requestId": "550e8400-e29b-41d4-a716-446655440000"})
        ),
        (status = 400, description = "Request failed", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
        (status = 404, description = "Not found", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
        (status = 503, description = "Service unavailable", body = crate::http::endpoints::v2::types::error::V2StatusFailed),
    ),
    tag = "Public Decrypt"
)]
pub async fn public_decrypt_get_v2(
    handler: Arc<PublicDecryptHandler>,
    Path(job_id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    handler.public_decrypt_get_v2(Path(job_id), headers).await
}
