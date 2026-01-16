use super::super::types::error::{
    RelayerV2ApiError400NoDetails, RelayerV2ApiError404, RelayerV2ApiError500, RelayerV2ApiError503,
};
use super::super::types::public_decrypt::{
    PublicDecryptPostResponseJson, PublicDecryptQueuedResult, PublicDecryptResponseJson,
    PublicDecryptStatusResponseJson,
};
use crate::core::errors::TIMEOUT_REASON_MISSING_MSG;
use crate::core::event::{
    ApiVersion, PublicDecryptEventData, PublicDecryptRequest, RelayerEvent, RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::gateway::arbitrum::transaction::tx_throttler::{GatewayTxTask, TxThrottlingSender};
use crate::gateway::readiness_check::readiness_throttler::{
    PublicDecryptReadinessTask, ReadinessSender,
};
use crate::http::endpoints::v1::types::public_decrypt::PublicDecryptRequestJson;
use crate::http::retry_after::{DecryptQueueInfo, RequestStateInfo, RetryAfterState};
use crate::http::utils::public_decrypt_bounce_check;
use crate::http::{parse_and_validate, AppResponse};
use crate::logging::PublicDecryptStep;
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::metrics::{observe_raw_eta_seconds, HttpApiVersion, RetryAfterRequestType};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::repositories::public_decrypt_repo::{
    PublicDecryptInsertResult, PublicDecryptRepository,
};
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
use std::sync::Arc;
use tracing::{error, info, instrument, span, Level};
use uuid::Uuid;

pub type PublicDecryptResponse = AppResponse<PublicDecryptPostResponseJson>;

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
            RelayerV2ApiError400NoDetails::invalid_signature(error_msg),
        ),
        RevertReason::Unknown => (
            StatusCode::INTERNAL_SERVER_ERROR,
            RelayerV2ApiError500::internal_server_error(error_msg),
        ),
    }
}

pub struct PublicDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
    retry_after_seconds: u32,
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
    retry_after_state: Arc<RetryAfterState>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static>
    PublicDecryptHandler<D>
{
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        public_decrypt_repo: Arc<PublicDecryptRepository>,
        retry_after_seconds: u32,
        tx_throttler: TxThrottlingSender<GatewayTxTask>,
        public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
        retry_after_state: Arc<RetryAfterState>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            public_decrypt_repo,
            retry_after_seconds,
            tx_throttler,
            public_decrypt_readiness_throttler,
            retry_after_state,
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
                    move |path| async move { handler.public_decrypt_get_v2(path).await }
                }),
            )
    }

    /// POST /v2/public-decrypt - Submit request and get reference ID
    pub async fn public_decrypt_post_v2(
        &self,
        req: Request<axum::body::Body>,
    ) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::PublicDecrypt,
            HttpMethod::Post,
            HttpApiVersion::V2,
            async move { self.handle_post(req, &()).await },
        )
        .await
        .into_response()
    }

    /// GET /v2/public-decrypt/<job_id> - Check status and get result
    pub async fn public_decrypt_get_v2(&self, Path(job_id): Path<Uuid>) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::PublicDecrypt,
            HttpMethod::Get,
            HttpApiVersion::V2,
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
        let request_id = self.orchestrator.new_internal_request_id();
        let _span = span!(Level::INFO, "handle-public-decrypt-post-req", request_id = %request_id);

        info!(
            step = %PublicDecryptStep::ReqReceived,
            request_id = %request_id,
            "Handling public decryption POST request"
        );

        let body = match AxumBytes::from_request(req, _state).await {
            Ok(body) => body,
            Err(_) => {
                let mut response = AppResponse::<()>::request_error("Failed to read request body");
                response.set_request_id(&request_id.to_string());
                return response.into_response();
            }
        };

        let request: PublicDecryptRequest =
            match parse_and_validate::<PublicDecryptRequestJson, PublicDecryptRequest>(&body) {
                Ok(request) => request,
                Err(parse_error) => {
                    let error_response: AppResponse<()> =
                        parse_error.to_app_response(&request_id.to_string());
                    return error_response.into_response();
                }
            };

        info!("Successfully parsed and validated request");

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
                    let full = public_decrypt_bounce_check(
                        self.tx_throttler.clone(),
                        self.public_decrypt_readiness_throttler.clone(),
                    )
                    .await;
                    if full {
                        info!(
                            step = %PublicDecryptStep::Bounced,
                            int_job_id = ?int_job_id,
                            "Public decrypt v2 is bounced by full queue"
                        );
                        return AppResponse::<()>::protocol_overloaded(
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
                    "Failed to insert/get public decrypt into/from database: {}",
                    e
                );
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
                .into_response();
            }
        }

        let proposed_ext_job_id = self.orchestrator.new_ext_job_id();

        let insert_result = match self
            .public_decrypt_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                int_job_id.as_ref(),
                request.clone(),
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                error!(
                    "Failed to insert/get public decrypt into/from database: {}",
                    e
                );
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
                .into_response();
            }
        };

        // Extract ext_job_id from any variant
        let assigned_ext_job_id = match &insert_result {
            PublicDecryptInsertResult::Inserted { ext_job_id } => *ext_job_id,
            PublicDecryptInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            PublicDecryptInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        // Only dispatch event for new requests (deduplication)
        if matches!(insert_result, PublicDecryptInsertResult::Inserted { .. }) {
            let event_data = PublicDecryptEventData::ReqRcvdFromUser {
                decrypt_request: request.clone(),
            };

            let event = RelayerEvent::new(
                int_job_id,
                self.api_version,
                RelayerEventData::PublicDecrypt(event_data),
            );

            if let Err(e) = self.orchestrator.dispatch_event(event).await {
                error!("Failed to dispatch event to orchestrator: {:?}", e);
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
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
                step = %PublicDecryptStep::DedupHit,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = ?int_job_id,
                "Duplicate request detected"
            );
        }

        // Generate a new request_id for this HTTP request (not stored)
        let request_id_for_response = uuid::Uuid::new_v4();

        // Compute dynamic retry-after based on dual queue state
        let readiness_queue_info = self
            .public_decrypt_readiness_throttler
            .get_queue_info()
            .await;
        let tx_queue_info = self.tx_throttler.get_queue_info().await;
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

        let response = PublicDecryptPostResponseJson {
            status: "queued".to_string(),
            request_id: request_id_for_response.to_string(),
            result: PublicDecryptQueuedResult {
                job_id: assigned_ext_job_id.to_string(),
            },
        };

        // Add Retry-After header with the dynamically computed retry value
        (
            StatusCode::ACCEPTED,
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
                                let api_response = PublicDecryptResponseJson::from(core_response);
                                (
                                    StatusCode::OK,
                                    Json(PublicDecryptStatusResponseJson {
                                        status: "succeeded".to_string(),
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
                                        status: "failed".to_string(),
                                        request_id: request_id.to_string(),
                                        result: None,
                                        error: Some(RelayerV2ApiError500::internal_server_error(
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
                                    status: "failed".to_string(),
                                    request_id: request_id.to_string(),
                                    result: None,
                                    error: Some(RelayerV2ApiError500::internal_server_error(
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
                        (
                            StatusCode::SERVICE_UNAVAILABLE,
                            Json(PublicDecryptStatusResponseJson {
                                status: "failed".to_string(),
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
                            Json(PublicDecryptStatusResponseJson {
                                status: "failed".to_string(),
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
                        let decrypt_queue_info = if response_model.req_status == ReqStatus::Queued {
                            let readiness_queue_info = self
                                .public_decrypt_readiness_throttler
                                .get_queue_info()
                                .await;
                            let tx_queue_info = self.tx_throttler.get_queue_info().await;
                            Some(DecryptQueueInfo::new(readiness_queue_info, tx_queue_info))
                        } else {
                            None
                        };

                        let retry_after = self
                            .retry_after_state
                            .compute_for_decrypt_get(
                                decrypt_queue_info.as_ref(),
                                &state_info,
                                false, // is_user_decrypt
                            )
                            .await;

                        info!(
                            req_id = %request_id,
                            ext_job_id = %job_id,
                            retry_after_secs = retry_after,
                            status = ?response_model.req_status,
                            "Computed retry-after for public decrypt GET"
                        );

                        (
                            StatusCode::ACCEPTED,
                            [(header::RETRY_AFTER, retry_after.to_string())],
                            Json(PublicDecryptStatusResponseJson {
                                status: "queued".to_string(),
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
                    status: "failed".to_string(),
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
                    Json(PublicDecryptStatusResponseJson {
                        status: "failed".to_string(),
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
/// POST /v2/public-decrypt - Submit request and get reference ID
#[utoipa::path(
    post,
    path = "/v2/public-decrypt",
    request_body = PublicDecryptRequestJson,
    responses(
        (status = 202, description = "Request accepted for processing", body = PublicDecryptPostResponseJson),
        (status = 400, description = "Invalid request", body = crate::http::endpoints::v2::types::error::RelayerV2ApiError400NoDetails),
        (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::RelayerV2ApiError500),
    ),
    tag = "Public Decrypt v2"
)]
pub async fn public_decrypt_post_v2<D>(
    handler: Arc<PublicDecryptHandler<D>>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.public_decrypt_post_v2(req).await
}

/// GET /v2/public-decrypt/<job_id> - Check status and get result
#[utoipa::path(
    get,
    path = "/v2/public-decrypt/{job_id}",
    params(
        ("job_id" = String, Path, description = "Job ID ed from POST request")
    ),
    responses(
        (status = 200, description = "Request completed successfully", body = PublicDecryptStatusResponseJson),
        (status = 202, description = "Request still processing", body = PublicDecryptStatusResponseJson),
        (status = 400, description = "Request failed", body = PublicDecryptStatusResponseJson),
        (status = 404, description = "Request not found", body = PublicDecryptStatusResponseJson),
        (status = 500, description = "Internal server error", body = PublicDecryptStatusResponseJson),
        (status = 503, description = "Request timed out", body = PublicDecryptStatusResponseJson),
    ),
    tag = "Public Decrypt v2"
)]
pub async fn public_decrypt_get_v2<D>(
    handler: Arc<PublicDecryptHandler<D>>,
    Path(job_id): Path<Uuid>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.public_decrypt_get_v2(Path(job_id)).await
}
