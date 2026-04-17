use super::super::types::error::{
    classify_revert_error, ApiResponseStatus, RelayerV2ResponseFailed, V2ErrorResponseBody,
};
use super::super::types::user_decrypt::{
    DelegatedUserDecryptRequestJson, UserDecryptPostResponseJson, UserDecryptQueuedResult,
    UserDecryptRequestJson, UserDecryptResponseJson, UserDecryptStatusResponseJson,
};
use crate::core::errors::{
    HOST_ACL_FAILED_PREFIX, NOT_ALLOWED_ON_HOST_ACL_PREFIX, READINESS_CHECK_TIMEOUT_MSG,
    TIMEOUT_REASON_MISSING_MSG,
};
use crate::core::event::{
    ApiVersion, DelegatedUserDecryptEventData, DelegatedUserDecryptRequest, RelayerEvent,
    RelayerEventData, UserDecryptEventData, UserDecryptRequest,
};
use crate::core::job_id::JobId;
use crate::host::HostChainIdChecker;
use crate::http::retry_after::{DecryptQueueInfo, RequestStateInfo, RetryAfterState};
use crate::http::utils::BounceChecker;
use crate::http::{parse_and_validate, AppResponse};
use crate::logging::UserDecryptStep;
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::metrics::{observe_raw_eta_seconds, HttpApiVersion, RetryAfterRequestType};
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::readiness::throttler::{DelegatedUserDecryptReadinessTask, UserDecryptReadinessTask};
use crate::store::sql::models::{
    req_status_enum_model::ReqStatus, user_decrypt_req_model::UserDecryptReqData,
};
use crate::store::sql::repositories::user_decrypt_repo::{
    UserDecryptInsertResult, UserDecryptRepository,
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
use std::sync::Arc;
use tracing::{error, info, instrument, span, Level};
use uuid::Uuid;

pub type UserDecryptResponse = AppResponse<UserDecryptPostResponseJson>;

pub struct UserDecryptHandler {
    orchestrator: Arc<Orchestrator>,
    api_version: ApiVersion,
    user_decrypt_repo: Arc<UserDecryptRepository>,
    user_decrypt_shares_threshold: u32,
    user_decrypt_queue_checker: BounceChecker<UserDecryptReadinessTask>,
    delegated_queue_checker: BounceChecker<DelegatedUserDecryptReadinessTask>,
    retry_after_state: Arc<RetryAfterState>,
    host_chain_id_checker: Arc<HostChainIdChecker>,
}

impl UserDecryptHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        orchestrator: Arc<Orchestrator>,
        api_version: ApiVersion,
        user_decrypt_repo: Arc<UserDecryptRepository>,
        user_decrypt_shares_threshold: u32,
        user_decrypt_queue_checker: BounceChecker<UserDecryptReadinessTask>,
        delegated_queue_checker: BounceChecker<DelegatedUserDecryptReadinessTask>,
        retry_after_state: Arc<RetryAfterState>,
        host_chain_id_checker: Arc<HostChainIdChecker>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            user_decrypt_repo,
            user_decrypt_shares_threshold,
            user_decrypt_queue_checker,
            delegated_queue_checker,
            retry_after_state,
            host_chain_id_checker,
        }
    }

    /// Create router with user decrypt v2 routes
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new()
            .route(
                "/v2/user-decrypt",
                axum::routing::post({
                    let handler = self.clone();
                    move |req| async move { handler.user_decrypt_post_v2(req).await }
                }),
            )
            .route(
                "/v2/delegated-user-decrypt",
                axum::routing::post({
                    let handler = self.clone();
                    move |req| async move { handler.delegated_user_decrypt_post_v2(req).await }
                }),
            )
            .route(
                "/v2/user-decrypt/{job_id}",
                axum::routing::get({
                    let handler = self.clone();
                    move |path, headers: HeaderMap| async move {
                        handler.user_decrypt_get_v2(path, headers).await
                    }
                }),
            )
            .route(
                "/v2/delegated-user-decrypt/{job_id}",
                axum::routing::get({
                    let handler = self;
                    move |path, headers: HeaderMap| async move {
                        handler.user_decrypt_get_v2(path, headers).await
                    }
                }),
            )
    }

    /// POST /v2/user-decrypt - Submit request and get reference ID
    pub async fn user_decrypt_post_v2(&self, req: Request<axum::body::Body>) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::UserDecrypt,
            HttpMethod::Post,
            HttpApiVersion::V2,
            req.headers().clone(),
            async move { self.handle_post(req, &()).await },
        )
        .await
        .into_response()
    }

    /// POST /v2/delegated-user-decrypt - Submit request and get reference ID
    pub async fn delegated_user_decrypt_post_v2(
        &self,
        req: Request<axum::body::Body>,
    ) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::DelegatedUserDecrypt,
            HttpMethod::Post,
            HttpApiVersion::V2,
            req.headers().clone(),
            async move { self.handle_delegated_user_decrypt_post(req, &()).await },
        )
        .await
        .into_response()
    }

    /// GET /v2/user-decrypt/<job_id> - Check status and get result
    pub async fn user_decrypt_get_v2(
        &self,
        Path(job_id): Path<Uuid>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::UserDecrypt,
            HttpMethod::Get,
            HttpApiVersion::V2,
            headers,
            async move { self.handle_get(job_id).await },
        )
        .await
        .into_response()
    }

    #[instrument(name = "handle-user-decrypt-post", skip_all, fields(request_id))]
    pub async fn handle_post<S>(
        &self,
        req: Request<axum::body::Body>,
        _state: &S,
    ) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = Uuid::new_v4();
        let _span = span!(Level::INFO, "handle-user-decrypt-post-req", request_id = %request_id);

        info!(
            step = %UserDecryptStep::ReqReceived,
            request_id = %request_id,
            "Handling user decryption POST request"
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

        let user_decrypt_request: UserDecryptRequest =
            match parse_and_validate::<UserDecryptRequestJson, UserDecryptRequest>(&body) {
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
            .validate_u256_handles(&user_decrypt_request.ct_handle_contract_pairs)
        {
            return RelayerV2ResponseFailed::host_chain_id_not_supported(
                chain_id,
                &request_id.to_string(),
            )
            .into_response();
        }

        let int_job_id: JobId = user_decrypt_request.content_hash().into();

        // Queue full Bouncing logic.
        let active_external_job_id = self
            .user_decrypt_repo
            .find_active_ext_ref_by_int_job_id(int_job_id.as_ref())
            .await;

        match active_external_job_id {
            Ok(res) => {
                if res.is_none() {
                    // In this case, we check queue full and bounce the request with 429
                    if let Err(retry_after) = self.user_decrypt_queue_checker.check().await {
                        info!(
                            step = %UserDecryptStep::Bounced,
                            int_job_id = ?int_job_id,
                            "User decrypt v2 is bounced by full queue"
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
                    "Failed to insert/get user decrypt into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        }

        let proposed_ext_job_id = self.orchestrator.new_ext_job_id();

        let user_decrypt_request_data =
            UserDecryptReqData::UserDecrypt(user_decrypt_request.clone());

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
                    "Failed to insert/get user decrypt into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        };

        // Extract ext_job_id from any variant
        let assigned_ext_job_id = match &insert_result {
            UserDecryptInsertResult::Inserted { ext_job_id } => *ext_job_id,
            UserDecryptInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            UserDecryptInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        // Only dispatch event for new requests (deduplication)
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
                error!("Failed to dispatch event to orchestrator: {:?}", e);
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
            info!(
                step = %UserDecryptStep::Queued,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = ?int_job_id,
                "Dispatched event to orchestrator"
            );
        } else {
            info!(
                step = %UserDecryptStep::DedupHit,
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
            .compute_for_decrypt_post(
                &decrypt_queue_info,
                true, // is_user_decrypt
            )
            .await;

        // Record raw ETA for POST histogram metrics
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
            "Computed retry-after for user decrypt POST"
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

    #[instrument(
        name = "handle-delegated-user-decrypt-post",
        skip_all,
        fields(request_id)
    )]
    pub async fn handle_delegated_user_decrypt_post<S>(
        &self,
        req: Request<axum::body::Body>,
        _state: &S,
    ) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = Uuid::new_v4();
        let _span =
            span!(Level::INFO, "handle-delegated-user-decrypt-post-req", request_id = %request_id);

        info!(
            "Handling delegated user decryption POST request, generated request id: {}",
            request_id
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

        let delegated_user_decrypt_request: DelegatedUserDecryptRequest = match parse_and_validate::<
            DelegatedUserDecryptRequestJson,
            DelegatedUserDecryptRequest,
        >(&body)
        {
            Ok(request) => request,
            Err(parse_error) => {
                return RelayerV2ResponseFailed::from_parse_error(
                    &parse_error,
                    &request_id.to_string(),
                )
                .into_response();
            }
        };

        info!("Successfully parsed and validated delegated user decryption request.");

        // Check early to avoid filling the queue with handles of unsupported chains
        if let Err(chain_id) = self
            .host_chain_id_checker
            .validate_u256_handles(&delegated_user_decrypt_request.ct_handle_contract_pairs)
        {
            return RelayerV2ResponseFailed::host_chain_id_not_supported(
                chain_id,
                &request_id.to_string(),
            )
            .into_response();
        }

        let int_job_id: JobId = delegated_user_decrypt_request.content_hash().into();

        // Queue full Bouncing logic.
        let active_external_job_id = self
            .user_decrypt_repo
            .find_active_ext_ref_by_int_job_id(int_job_id.as_ref())
            .await;

        match active_external_job_id {
            Ok(res) => {
                if res.is_none() {
                    // In this case, we check queue full and bounce the request with 429
                    if let Err(retry_after) = self.delegated_queue_checker.check().await {
                        info!("Delegated user decryption v2 is bounced by full queue.");
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
                    "Failed to insert/get delegated user decryption into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        }

        let proposed_ext_job_id = self.orchestrator.new_ext_job_id();

        let delegated_user_decrypt_request_data =
            UserDecryptReqData::DelegatedUserDecrypt(delegated_user_decrypt_request.clone());

        let insert_result = match self
            .user_decrypt_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                &int_job_id[..],
                delegated_user_decrypt_request_data,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                error!(
                    int_job_id = ?int_job_id,
                    "Failed to insert/get delegated user decryption into/from database: {}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
        };

        // Extract ext_job_id from any variant
        let assigned_ext_job_id = match &insert_result {
            UserDecryptInsertResult::Inserted { ext_job_id } => *ext_job_id,
            UserDecryptInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            UserDecryptInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        // Only dispatch event for new requests (deduplication)
        if matches!(insert_result, UserDecryptInsertResult::Inserted { .. }) {
            let request_data = DelegatedUserDecryptEventData::ReqRcvdFromUser {
                decrypt_request: delegated_user_decrypt_request,
            };
            let event = RelayerEvent::new(
                int_job_id,
                self.api_version,
                RelayerEventData::DelegatedUserDecrypt(request_data),
            );

            if let Err(e) = self.orchestrator.dispatch_event(event).await {
                error!(
                    "Failed to dispatch DelegatedUserDecrypt event to orchestrator: {:?}",
                    e
                );
                return RelayerV2ResponseFailed::internal_server_error(&request_id.to_string())
                    .into_response();
            }
            info!(
                step = %UserDecryptStep::Queued,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = ?int_job_id,
                "Dispatched DelegatedUserDecrypt event to orchestrator"
            );
        } else {
            info!(
                step = %UserDecryptStep::DedupHit,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = ?int_job_id,
                "Duplicate delegated user decryption request detected"
            );
        }

        // Generate a new request_id for this HTTP request (not stored)
        let request_id_for_response = uuid::Uuid::new_v4();

        // Compute dynamic retry-after based on dual queue state
        let readiness_queue_info = self
            .delegated_queue_checker
            .readiness_throttler()
            .get_queue_info()
            .await;
        let tx_queue_info = self
            .delegated_queue_checker
            .tx_throttler()
            .get_queue_info()
            .await;
        let decrypt_queue_info = DecryptQueueInfo::new(readiness_queue_info, tx_queue_info);
        let retry_after = self
            .retry_after_state
            .compute_for_decrypt_post(
                &decrypt_queue_info,
                true, // is_user_decrypt
            )
            .await;

        // Record raw ETA for POST histogram metrics
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
            "Computed retry-after for delegated user decrypt POST"
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

    #[instrument(name = "handle-user-decrypt-get", skip_all, fields(job_id))]
    pub async fn handle_get(&self, job_id: Uuid) -> impl IntoResponse {
        // Generate a new request_id for this HTTP request
        let request_id = uuid::Uuid::new_v4();

        info!(
            "Handling user decryption GET request for job_id: {}, request_id: {}",
            job_id, request_id
        );

        // Check SQL for current status using job_id (which is the external_reference_id in DB)
        let fallback_threshold = self.user_decrypt_shares_threshold; // u32
        match self
            .user_decrypt_repo
            .find_req_and_shares_by_ext_job_id(job_id, fallback_threshold)
            .await
        {
            Ok(Some(response_model)) => {
                match response_model.req_status {
                    ReqStatus::Completed => {
                        // Use resolved_threshold from DB if available, fall back to static config.
                        // DB stores i64 (BIGINT); convert to u32 at the repo boundary.
                        let required_threshold = response_model
                            .resolved_threshold
                            .and_then(|v| u32::try_from(v).ok())
                            .unwrap_or(self.user_decrypt_shares_threshold);
                        if response_model.shares.len() >= required_threshold as usize {
                            // Convert from database model to API response
                            match UserDecryptResponseJson::try_from(response_model) {
                                Ok(api_response) => {
                                    let status_code = StatusCode::OK;

                                    info!(
                                        request_id = %request_id,
                                        http_status = status_code.as_u16(),
                                        ext_job_id = %job_id,
                                        "HTTP response"
                                    );

                                    (
                                        status_code,
                                        Json(UserDecryptStatusResponseJson {
                                            status: ApiResponseStatus::Succeeded,
                                            request_id: request_id.to_string(), // Per-request UUID
                                            result: Some(api_response),
                                            error: None,
                                        }),
                                    )
                                        .into_response()
                                }
                                Err(e) => {
                                    error!(
                                        request_id = %request_id,
                                        ext_job_id = %job_id,
                                        error = %e,
                                        "Response conversion failed"
                                    );
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(UserDecryptStatusResponseJson {
                                            status: ApiResponseStatus::Failed,
                                            request_id: request_id.to_string(),
                                            result: None,
                                            error: Some(
                                                V2ErrorResponseBody::internal_server_error(
                                                    "Internal server error",
                                                ),
                                            ),
                                        }),
                                    )
                                        .into_response()
                                }
                            }
                        } else {
                            error!(
                                "Request marked as completed but insufficient shares: got {}, needed {}",
                                response_model.shares.len(), required_threshold
                            );
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(UserDecryptStatusResponseJson {
                                    status: ApiResponseStatus::Failed,
                                    request_id: request_id.to_string(),
                                    result: None,
                                    error: Some(V2ErrorResponseBody::internal_server_error(
                                        "Internal error: completed request has insufficient shares",
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
                            Json(UserDecryptStatusResponseJson {
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
                            Json(UserDecryptStatusResponseJson {
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
                        let decrypt_queue_info = if response_model.req_status == ReqStatus::Queued {
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
                            Some(DecryptQueueInfo::new(readiness_queue_info, tx_queue_info))
                        } else {
                            None
                        };

                        let retry_after = self
                            .retry_after_state
                            .compute_for_decrypt_get(
                                decrypt_queue_info.as_ref(),
                                &state_info,
                                true, // is_user_decrypt
                            )
                            .await;

                        let status_code = StatusCode::ACCEPTED;

                        info!(
                            req_id = %request_id,
                            ext_job_id = %job_id,
                            retry_after_secs = retry_after,
                            status = ?response_model.req_status,
                            "Computed retry-after for user decrypt GET"
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
                            Json(UserDecryptStatusResponseJson {
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
                Json(UserDecryptStatusResponseJson {
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
                    Json(UserDecryptStatusResponseJson {
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
/// Submit user decryption.
#[utoipa::path(
    post,
    path = "/v2/user-decrypt",
    request_body = UserDecryptRequestJson,
    responses(
        (status = 202, description = "Request accepted for processing.", body = UserDecryptPostResponseJson),
        (status = 400, description = "Invalid request", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 429, description = "Rate limited", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
    ),
    tag = "User Decrypt"
)]
pub async fn user_decrypt_post_v2(
    handler: Arc<UserDecryptHandler>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    handler.user_decrypt_post_v2(req).await
}

/// Check user decryption status.
#[utoipa::path(
    get,
    path = "/v2/user-decrypt/{job_id}",
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
    tag = "User Decrypt"
)]
pub async fn user_decrypt_get_v2(
    handler: Arc<UserDecryptHandler>,
    Path(job_id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    handler.user_decrypt_get_v2(Path(job_id), headers).await
}

/// Submit delegated user decryption.
#[utoipa::path(
    post,
    path = "/v2/delegated-user-decrypt",
    request_body = DelegatedUserDecryptRequestJson,
    responses(
        (status = 202, description = "Request accepted for processing.", body = UserDecryptPostResponseJson),
        (status = 400, description = "Invalid request", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 429, description = "Rate limited", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
        (status = 500, description = "Internal server error", body = crate::http::endpoints::v2::types::error::RelayerV2ResponseFailed),
    ),
    tag = "Delegated User Decrypt"
)]
pub async fn delegated_user_decrypt_post_v2(
    handler: Arc<UserDecryptHandler>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    handler.delegated_user_decrypt_post_v2(req).await
}

/// Check delegated user decryption status.
#[utoipa::path(
    get,
    path = "/v2/delegated-user-decrypt/{job_id}",
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
    tag = "Delegated User Decrypt"
)]
pub async fn delegated_user_decrypt_get_v2(
    handler: Arc<UserDecryptHandler>,
    Path(job_id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    handler.user_decrypt_get_v2(Path(job_id), headers).await
}
