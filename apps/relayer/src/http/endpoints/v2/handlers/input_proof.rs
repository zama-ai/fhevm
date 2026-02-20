use super::super::types::error::{
    ApiResponseStatus, RelayerV2ApiError400NoDetails, RelayerV2ApiError404, RelayerV2ApiError500,
    RelayerV2ApiError503, RelayerV2ResponseFailed,
};
use super::super::types::input_proof::{
    InputProofPostResponseJson, InputProofQueuedResult, InputProofResponseJson,
    InputProofStatusResponseJson,
};
use crate::core::errors::TIMEOUT_REASON_MISSING_MSG;
use crate::core::event::{
    ApiVersion, InputProofEventData, InputProofRequest, RelayerEvent, RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::gateway::arbitrum::transaction::tx_throttler::{GatewayTxTask, TxThrottlingSender};
use crate::http::endpoints::v1::types::input_proof::InputProofRequestJson;
use crate::http::retry_after::{RequestStateInfo, RetryAfterState};
use crate::http::utils::bounce_check;
use crate::http::{parse_and_validate, AppResponse};
use crate::logging::InputProofStep;
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::metrics::{observe_raw_eta_seconds, HttpApiVersion, RetryAfterRequestType};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::ContentHasher;
use crate::orchestrator::Orchestrator;
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::repositories::input_proof_repo::{
    InputProofInsertResult, InputProofRepository,
};
use axum::http::HeaderMap;
use axum::{
    body::Bytes,
    extract::{FromRequest, Path},
    http::Request,
    response::IntoResponse,
};
use axum::{
    http::{header, StatusCode},
    Json,
};
use std::sync::Arc;
use tracing::{error, info, instrument, span, warn, Level};
use uuid::Uuid;

pub type InputProofResponse = AppResponse<InputProofPostResponseJson>;

/// Helper to classify error messages and return appropriate HTTP status and error response
///
/// Parses error selector from message, classifies the revert reason,
/// and returns appropriate HTTP status and error response.
fn classify_error(error_msg: &str) -> (StatusCode, serde_json::Value) {
    use crate::gateway::utils::{classify_revert_selector, extract_revert_selector, RevertReason};

    // Parse selector and classify revert reason
    let reason = if let Some(selector) = extract_revert_selector(error_msg) {
        classify_revert_selector(&selector)
    } else {
        RevertReason::Unknown
    };

    // Map to HTTP response
    match reason {
        RevertReason::ContractPaused => (
            StatusCode::SERVICE_UNAVAILABLE,
            RelayerV2ApiError503::protocol_paused(error_msg),
        ),
        RevertReason::InsufficientBalance => (
            StatusCode::SERVICE_UNAVAILABLE,
            RelayerV2ApiError503::insufficient_balance(error_msg),
        ),
        RevertReason::InsufficientAllowance => (
            StatusCode::SERVICE_UNAVAILABLE,
            RelayerV2ApiError503::insufficient_allowance(error_msg),
        ),
        RevertReason::InvalidSignature => (
            StatusCode::BAD_REQUEST,
            RelayerV2ApiError400NoDetails::invalid_signature(),
        ),
        RevertReason::Unknown => (
            StatusCode::INTERNAL_SERVER_ERROR,
            RelayerV2ApiError500::internal_server_error(error_msg),
        ),
    }
}

pub struct InputProofHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    input_proof_repo: Arc<InputProofRepository>,
    retry_after_seconds: u32,
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    retry_after_state: Arc<RetryAfterState>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static>
    InputProofHandler<D>
{
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        input_proof_repo: Arc<InputProofRepository>,
        retry_after_seconds: u32,
        tx_throttler: TxThrottlingSender<GatewayTxTask>,
        retry_after_state: Arc<RetryAfterState>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            input_proof_repo,
            retry_after_seconds,
            tx_throttler,
            retry_after_state,
        }
    }

    /// Create router with input proof v2 routes
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new()
            .route(
                "/v2/input-proof",
                axum::routing::post({
                    let handler = self.clone();
                    move |req| async move { handler.input_proof_post_v2(req).await }
                }),
            )
            .route(
                "/v2/input-proof/{job_id}",
                axum::routing::get({
                    let handler = self;
                    move |path, headers: HeaderMap| async move {
                        handler.input_proof_get_v2(path, headers).await
                    }
                }),
            )
    }

    /// POST /v2/input-proof - Submit request and get reference ID
    pub async fn input_proof_post_v2(&self, req: Request<axum::body::Body>) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::InputProof,
            HttpMethod::Post,
            HttpApiVersion::V2,
            req.headers().clone(),
            async move { self.handle_post(req, &()).await },
        )
        .await
        .into_response()
    }

    /// GET /v2/input-proof/<job_id> - Check status and get result
    pub async fn input_proof_get_v2(
        &self,
        Path(job_id): Path<Uuid>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::InputProof,
            HttpMethod::Get,
            HttpApiVersion::V2,
            headers,
            async move { self.handle_get(job_id).await },
        )
        .await
        .into_response()
    }

    #[instrument(name = "handle-input-proof-post", skip_all, fields(request_id))]
    pub async fn handle_post<S>(
        &self,
        req: Request<axum::body::Body>,
        _state: &S,
    ) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = Uuid::new_v4();
        let _span = span!(Level::INFO, "handle-input-proof-post-req", request_id = %request_id);

        info!(
            step = %InputProofStep::ReqReceived,
            req_id = %request_id,
            "Handling input proof v2 POST request"
        );

        let body = match Bytes::from_request(req, _state).await {
            Ok(body) => body,
            Err(_) => {
                return RelayerV2ResponseFailed::request_error(
                    "Failed to read request body",
                    &request_id.to_string(),
                )
                .into_response();
            }
        };

        let request_data: InputProofRequest =
            match parse_and_validate::<InputProofRequestJson, InputProofRequest>(&body) {
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

        let int_job_id: JobId = request_data.content_hash().into();

        // Queue full bouncing logic: check if active request exists first
        let active_external_job_id = self
            .input_proof_repo
            .find_active_ext_ref_by_int_job_id(int_job_id.as_ref())
            .await;

        match active_external_job_id {
            Ok(res) => {
                if res.is_none() {
                    // No active request exists, check if queue is full and bounce if needed
                    let full = bounce_check(self.tx_throttler.clone()).await;
                    if full {
                        warn!(
                            step = %InputProofStep::Bounced,
                            int_job_id = %int_job_id,
                            "Input proof v2 request bounced by full queue"
                        );
                        return RelayerV2ResponseFailed::protocol_overloaded(
                            "relayer is currently processing too many requests",
                            &self.retry_after_seconds.to_string(),
                            &request_id.to_string(),
                        )
                        .into_response();
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to check for active input proof request in database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        }

        let proposed_ext_job_id = self.orchestrator.new_ext_job_id();

        // Insert into database or get existing result for duplicate
        let insert_result = match self
            .input_proof_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                int_job_id.as_ref(),
                request_data.clone(),
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                error!("Failed to insert input proof into database: {}", e);
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        };

        // Extract ext_job_id from any variant
        let assigned_ext_job_id = match &insert_result {
            InputProofInsertResult::Inserted { ext_job_id } => *ext_job_id,
            InputProofInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            InputProofInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        // Only dispatch event for new requests (deduplication)
        if matches!(insert_result, InputProofInsertResult::Inserted { .. }) {
            let event_data = InputProofEventData::ReqRcvdFromUser {
                input_proof_request: request_data,
            };

            let event = RelayerEvent::new(
                int_job_id,
                self.api_version,
                RelayerEventData::InputProof(event_data),
            );

            if let Err(e) = self.orchestrator.dispatch_event(event).await {
                error!("Failed to dispatch event to orchestrator: {:?}", e);
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
            info!(
                step = %InputProofStep::Queued,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = %int_job_id,
                "Dispatched event to orchestrator"
            );
        } else {
            info!(
                step = %InputProofStep::DedupHit,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = %int_job_id,
                "Duplicate request detected"
            );
        }

        // Generate a new request_id for this HTTP request (not stored)
        let request_id_for_response = uuid::Uuid::new_v4();

        // Compute dynamic retry-after based on queue state
        let tx_queue_info = self.tx_throttler.get_queue_info().await;
        let retry_after = self
            .retry_after_state
            .compute_for_input_proof_post(&tx_queue_info)
            .await;

        // Record raw ETA for POST histogram metrics
        let raw_eta_ms = self
            .retry_after_state
            .compute_raw_eta_ms_for_input_proof(&tx_queue_info)
            .await;
        observe_raw_eta_seconds(
            RetryAfterRequestType::InputProof,
            raw_eta_ms as f64 / 1000.0,
        );

        info!(
            req_id = %request_id_for_response,
            int_job_id = %int_job_id,
            ext_job_id = %assigned_ext_job_id,
            retry_after_secs = retry_after,
            "Computed retry-after for input proof POST"
        );

        // Return response immediately
        let status_code = StatusCode::ACCEPTED;
        let response = InputProofPostResponseJson {
            status: ApiResponseStatus::Queued,
            request_id: request_id_for_response.to_string(), // New per-request UUID
            result: InputProofQueuedResult {
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

    #[instrument(name = "handle-input-proof-get", skip_all, fields(job_id))]
    pub async fn handle_get(&self, job_id: Uuid) -> impl IntoResponse {
        // Generate a new request_id for this HTTP request
        let request_id = uuid::Uuid::new_v4();

        info!(
            "Handling input proof GET request for job_id: {}, request_id: {}",
            job_id, request_id
        );

        // Check SQL for current status using job_id (which is the external_reference_id in DB)
        match self.input_proof_repo.find_status_by_ext_id(job_id).await {
            Ok(Some(response_model)) => {
                match response_model.req_status {
                    ReqStatus::Completed => {
                        if response_model.accepted.unwrap_or(false) {
                            if let Some(res) = response_model.res {
                                // Deserialize from database JsonValue to core event type, then convert to API response
                                if let Ok(core_response) = serde_json::from_value::<
                                    crate::core::event::InputProofResponse,
                                >(res)
                                {
                                    let status_code = StatusCode::OK;
                                    let api_response = InputProofResponseJson::from(core_response);

                                    info!(
                                        request_id = %request_id,
                                        http_status = status_code.as_u16(),
                                        ext_job_id = %job_id,
                                        "HTTP response"
                                    );

                                    (
                                        status_code,
                                        Json(InputProofStatusResponseJson {
                                            status: ApiResponseStatus::Succeeded,
                                            request_id: request_id.to_string(), // Per-request UUID
                                            result: Some(api_response),
                                            error: None,
                                        }),
                                    )
                                        .into_response()
                                } else {
                                    error!(
                                        "Failed to deserialize input proof response from database"
                                    );
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(InputProofStatusResponseJson {
                                            status: ApiResponseStatus::Failed,
                                            request_id: request_id.to_string(),
                                            result: None,
                                            error: Some(
                                                RelayerV2ApiError500::internal_server_error(
                                                    "Failed to deserialize response data",
                                                ),
                                            ),
                                        }),
                                    )
                                        .into_response()
                                }
                            } else {
                                error!("Request marked as completed and accepted but no response data found");
                                (StatusCode::INTERNAL_SERVER_ERROR, Json(InputProofStatusResponseJson {
                                    status: ApiResponseStatus::Failed,
                                    request_id: request_id.to_string(),
                                    result: None,
                                    error: Some(RelayerV2ApiError500::internal_server_error("Internal error: completed request missing response data")),
                                })).into_response()
                            }
                        } else {
                            // Request was rejected
                            let error_msg = match response_model.err_reason {
                                Some(reason) => reason,
                                None => {
                                    error!(
                                        alert = true,
                                        request_id = %request_id,
                                        "Rejected request missing error reason in database"
                                    );
                                    "Proof rejected".to_string()
                                }
                            };

                            // Classify the error to determine appropriate status code and label
                            let (status_code, error_value) = classify_error(&error_msg);

                            (
                                status_code,
                                Json(InputProofStatusResponseJson {
                                    status: ApiResponseStatus::Failed,
                                    request_id: request_id.to_string(),
                                    result: None,
                                    error: Some(error_value),
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
                                    "TimedOut request missing error reason in database"
                                );
                                TIMEOUT_REASON_MISSING_MSG.to_string()
                            }
                        };
                        (
                            StatusCode::SERVICE_UNAVAILABLE,
                            Json(InputProofStatusResponseJson {
                                status: ApiResponseStatus::Failed,
                                request_id: request_id.to_string(),
                                result: None,
                                error: Some(RelayerV2ApiError503::response_timed_out(&error_msg)),
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

                        // Classify the error to determine appropriate status code and label
                        let (status_code, error_value) = classify_error(&error_msg);

                        (
                            status_code,
                            Json(InputProofStatusResponseJson {
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

                        // Compute dynamic retry-after based on current state
                        let state_info = RequestStateInfo::new(response_model.req_status, 0, 0);

                        // For Queued status, also get queue info for more accurate ETA
                        let tx_queue_info = if response_model.req_status == ReqStatus::Queued {
                            Some(self.tx_throttler.get_queue_info().await)
                        } else {
                            None
                        };

                        let retry_after = self
                            .retry_after_state
                            .compute_for_input_proof_get(tx_queue_info.as_ref(), &state_info)
                            .await;

                        let status_code = StatusCode::ACCEPTED;

                        info!(
                            req_id = %request_id,
                            ext_job_id = %job_id,
                            retry_after_secs = retry_after,
                            status = ?response_model.req_status,
                            "Computed retry-after for input proof GET"
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
                            Json(InputProofStatusResponseJson {
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
                Json(InputProofStatusResponseJson {
                    status: ApiResponseStatus::Failed,
                    request_id: request_id.to_string(),
                    result: None,
                    error: Some(RelayerV2ApiError404::not_found("Request not found")),
                }),
            )
                .into_response(),
            Err(e) => {
                error!("Database error while checking status: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(InputProofStatusResponseJson {
                        status: ApiResponseStatus::Failed,
                        request_id: request_id.to_string(),
                        result: None,
                        error: Some(RelayerV2ApiError500::internal_server_error(
                            "Database error",
                        )),
                    }),
                )
                    .into_response()
            }
        }
    }
}

// OpenAPI documented endpoints as standalone functions
/// POST /v2/input-proof - Submit input proof verification request and get reference ID
#[utoipa::path(
    post,
    path = "/v2/input-proof",
    request_body = crate::http::endpoints::v1::types::input_proof::InputProofRequestJson,
    responses(
        (status = 202, description = "Request accepted for processing", body = crate::http::endpoints::v2::types::input_proof::InputProofPostResponseJson),
        (status = 400, description = "Invalid request", body = crate::http::endpoints::v2::types::error::RelayerV2ApiError400NoDetails),
        (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::RelayerV2ApiError500),
    ),
    tag = "Input Proof v2"
)]
pub async fn input_proof_post_v2<D>(
    handler: Arc<InputProofHandler<D>>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.input_proof_post_v2(req).await
}

/// GET /v2/input-proof/<job_id> - Check status and get result
#[utoipa::path(
    get,
    path = "/v2/input-proof/{job_id}",
    params(
        ("job_id" = String, Path, description = "Job ID returned from POST request")
    ),
    responses(
        (status = 200, description = "Request completed successfully", body = crate::http::endpoints::v2::types::input_proof::InputProofStatusResponseJson),
        (status = 202, description = "Request still processing", body = crate::http::endpoints::v2::types::input_proof::InputProofStatusResponseJson),
        (status = 400, description = "Request failed", body = crate::http::endpoints::v2::types::input_proof::InputProofStatusResponseJson),
        (status = 404, description = "Request not found", body = crate::http::endpoints::v2::types::input_proof::InputProofStatusResponseJson),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::input_proof::InputProofStatusResponseJson),
        (status = 503, description = "Request timed out", body = crate::http::endpoints::v2::types::input_proof::InputProofStatusResponseJson),
    ),
    tag = "Input Proof v2"
)]
pub async fn input_proof_get_v2<D>(
    handler: Arc<InputProofHandler<D>>,
    Path(job_id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.input_proof_get_v2(Path(job_id), headers).await
}
