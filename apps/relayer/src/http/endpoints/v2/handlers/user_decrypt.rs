use super::super::types::error::{
    RelayerV2ApiError400NoDetails, RelayerV2ApiError404, RelayerV2ApiError500,
};
use super::super::types::user_decrypt::{
    UserDecryptErrorResponseJson, UserDecryptPostResponseJson, UserDecryptQueuedResult,
    UserDecryptResponseJson, UserDecryptStatusResponseJson,
};
use crate::core::event::{
    ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptRequest,
};
use crate::core::job_id::JobId;
use crate::http::endpoints::v1::types::user_decrypt::UserDecryptRequestJson;
use crate::http::{parse_and_validate, AppResponse};
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::store::sql::repositories::user_decrypt_repo::UserDecryptRepository;
use axum::{
    body::Bytes as AxumBytes,
    extract::{FromRequest, Path},
    http::Request,
    response::IntoResponse,
};
use axum::{http::StatusCode, Json};
use std::sync::Arc;
use tracing::{error, info, instrument, span, Level};
use uuid::Uuid;

pub type UserDecryptResponse = AppResponse<UserDecryptPostResponseJson>;

pub struct UserDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    user_decrypt_repo: Arc<UserDecryptRepository>,
    user_decrypt_shares_threshold: u16,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static>
    UserDecryptHandler<D>
{
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        user_decrypt_repo: Arc<UserDecryptRepository>,
        user_decrypt_shares_threshold: u16,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            user_decrypt_repo,
            user_decrypt_shares_threshold,
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
                "/v2/user-decrypt/{job_id}",
                axum::routing::get({
                    let handler = self;
                    move |path| async move { handler.user_decrypt_get_v2(path).await }
                }),
            )
    }

    /// POST /v2/user-decrypt - Submit request and get reference ID
    pub async fn user_decrypt_post_v2(&self, req: Request<axum::body::Body>) -> impl IntoResponse {
        http_metrics::with_http_metrics(HttpEndpoint::UserDecrypt, HttpMethod::Post, async move {
            self.handle_post(req, &()).await
        })
        .await
        .into_response()
    }

    /// GET /v2/user-decrypt/<job_id> - Check status and get result
    pub async fn user_decrypt_get_v2(&self, Path(job_id): Path<Uuid>) -> impl IntoResponse {
        http_metrics::with_http_metrics(HttpEndpoint::UserDecrypt, HttpMethod::Get, async move {
            self.handle_get(job_id).await
        })
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
        let request_id = self.orchestrator.new_internal_request_id();
        let _span = span!(Level::INFO, "handle-user-decrypt-post-req", request_id = %request_id);

        info!(
            "Handling user decryption POST request, generated request id: {}",
            request_id
        );

        let body = match AxumBytes::from_request(req, _state).await {
            Ok(body) => body,
            Err(_) => {
                let mut response = AppResponse::<()>::request_error("Failed to read request body");
                response.set_request_id(&request_id.to_string());
                return response.into_response();
            }
        };

        let user_decrypt_request: UserDecryptRequest =
            match parse_and_validate::<UserDecryptRequestJson, UserDecryptRequest>(&body) {
                Ok(request) => request,
                Err(parse_error) => {
                    let error_response: AppResponse<()> =
                        parse_error.to_app_response(&request_id.to_string());
                    return error_response.into_response();
                }
            };

        info!("Successfully parsed and validated request");

        let int_indexer_id = user_decrypt_request.content_hash();
        let ext_reference_id = self.orchestrator.new_ext_reference_id();

        // Insert into database immediately
        if let Err(e) = self
            .user_decrypt_repo
            .insert_data_on_conflict_and_get_ext_reference_id(
                ext_reference_id,
                &int_indexer_id[..],
                user_decrypt_request.clone(),
            )
            .await
        {
            error!(
                "Failed to insert/get user decrypt into/from database: {}",
                e
            );
            return AppResponse::<()>::internal_server_error_with_request_id(
                request_id.to_string(),
            )
            .into_response();
        }

        // Trigger orchestrator processing
        let job_id = JobId::from_sha256_hash(int_indexer_id);
        let request_data = UserDecryptEventData::ReqRcvdFromUser {
            decrypt_request: user_decrypt_request,
        };
        let event = RelayerEvent::new(
            job_id,
            self.api_version,
            RelayerEventData::UserDecrypt(request_data),
        );

        if let Err(e) = self.orchestrator.dispatch_event(event).await {
            error!("Failed to dispatch event to orchestrator: {:?}", e);
            return AppResponse::<()>::internal_server_error_with_request_id(
                request_id.to_string(),
            )
            .into_response();
        }

        info!("Dispatched event to orchestrator to initiate processing");

        // Generate a new request_id for this HTTP request (not stored)
        let request_id_for_response = uuid::Uuid::new_v4();

        // Return response immediately
        let response = UserDecryptPostResponseJson {
            status: "queued".to_string(),
            request_id: request_id_for_response.to_string(), // New per-request UUID
            result: UserDecryptQueuedResult {
                job_id: ext_reference_id.to_string(), // This is what gets stored and tracked
                retry_after_seconds: 15,
            },
        };

        (StatusCode::ACCEPTED, Json(response)).into_response()
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
        let threshold = self.user_decrypt_shares_threshold as i64;
        let _status_result = match self
            .user_decrypt_repo
            .find_req_and_shares_by_ext_reference_id(job_id, threshold)
            .await
        {
            Ok(Some(response_model)) => {
                use crate::store::sql::models::req_status_enum_model::ReqStatus;
                match response_model.req_status {
                    ReqStatus::Completed => {
                        let required_threshold = self.user_decrypt_shares_threshold as usize;
                        if response_model.shares.len() >= required_threshold {
                            // Convert from database model to API response
                            let api_response = UserDecryptResponseJson::from(response_model);
                            return (
                                StatusCode::OK,
                                Json(UserDecryptStatusResponseJson {
                                    status: "succeeded".to_string(),
                                    request_id: request_id.to_string(), // Per-request UUID
                                    result: Some(api_response),
                                    error: None,
                                }),
                            )
                                .into_response();
                        } else {
                            error!(
                                "Request marked as completed but insufficient shares: got {}, needed {}", 
                                response_model.shares.len(), required_threshold
                            );
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(UserDecryptStatusResponseJson {
                                    status: "failed".to_string(),
                                    request_id: request_id.to_string(),
                                    result: None,
                                    error: Some(RelayerV2ApiError500::internal_server_error(
                                        "Internal error: completed request has insufficient shares",
                                    )),
                                }),
                            )
                                .into_response();
                        }
                    }
                    // TODO: Implement 503/504 errors for gateway / upstream related errors.
                    // RelayerV2ApiError503, RelayerV2ApiError504,
                    ReqStatus::Failure | ReqStatus::TimedOut => {
                        let error_msg = response_model
                            .err_reason
                            .unwrap_or("Unknown error".to_string());
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(UserDecryptStatusResponseJson {
                                status: "failed".to_string(),
                                request_id: request_id.to_string(),
                                result: None,
                                error: Some(RelayerV2ApiError400NoDetails::validation_error(
                                    &error_msg,
                                )),
                            }),
                        )
                            .into_response();
                    }
                    ReqStatus::Queued | ReqStatus::Processing | ReqStatus::ReceiptReceived => {
                        // Request is still in progress, fall through to event subscription
                        response_model
                    }
                }
            }
            Ok(None) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(UserDecryptStatusResponseJson {
                        status: "failed".to_string(),
                        request_id: request_id.to_string(),
                        result: None,
                        error: Some(RelayerV2ApiError404::not_found("Request not found")),
                    }),
                )
                    .into_response();
            }
            Err(e) => {
                error!("Database error while checking status: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(UserDecryptStatusResponseJson {
                        status: "failed".to_string(),
                        request_id: request_id.to_string(),
                        result: None,
                        error: Some(RelayerV2ApiError500::internal_server_error(
                            "Database error",
                        )),
                    }),
                )
                    .into_response();
            }
        };

        // If we get here, request is in progress - set up event subscription with 5s timeout
        info!("Request still in progress, setting up event subscription");

        // TODO: Implement readiness check with timeout for user decrypt operations
        // When readiness check times out, return 504
        // if readiness_check_timed_out {
        //     return (StatusCode::GATEWAY_TIMEOUT, Json(UserDecryptStatusResponseJson {
        //         status: "failed".to_string(),
        //         request_id: request_id.to_string(),
        //         result: None,
        //         error: Some(RelayerV2ApiError504::readiness_check_timedout("Readiness check timed out")),
        //     })).into_response();
        // }

        // For now, return pending status with timeout - we can implement event subscription later
        let timeout_duration = std::time::Duration::from_secs(5);

        tokio::select! {
            _ = tokio::time::sleep(timeout_duration) => {
                // Timeout reached, return 202 with queued status
                (StatusCode::ACCEPTED, Json(UserDecryptStatusResponseJson {
                    status: "queued".to_string(),
                    request_id: request_id.to_string(),
                    result: None,
                    error: None,
                })).into_response()
            }
        }
    }
}

// OpenAPI documented endpoints as standalone functions
/// POST /v2/user-decrypt - Submit request and get reference ID
#[utoipa::path(
    post,
    path = "/v2/user-decrypt",
    request_body = UserDecryptRequestJson,
    responses(
        (status = 202, description = "Request accepted for processing", body = UserDecryptPostResponseJson),
        (status = 400, description = "Invalid request", body = UserDecryptErrorResponseJson),
        (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
        (status = 500, description = "Internal server error", body = UserDecryptErrorResponseJson),
    ),
    tag = "User Decrypt v2"
)]
pub async fn user_decrypt_post_v2<D>(
    handler: Arc<UserDecryptHandler<D>>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.user_decrypt_post_v2(req).await
}

/// GET /v2/user-decrypt/<job_id> - Check status and get result
#[utoipa::path(
    get,
    path = "/v2/user-decrypt/{job_id}",
    params(
        ("job_id" = String, Path, description = "Job ID returned from POST request")
    ),
    responses(
        (status = 200, description = "Request completed successfully", body = UserDecryptStatusResponseJson),
        (status = 202, description = "Request still processing", body = UserDecryptStatusResponseJson),
        (status = 400, description = "Request failed", body = UserDecryptStatusResponseJson),
        (status = 404, description = "Request not found", body = UserDecryptStatusResponseJson),
        (status = 500, description = "Internal server error", body = UserDecryptStatusResponseJson),
    ),
    tag = "User Decrypt v2"
)]
pub async fn user_decrypt_get_v2<D>(
    handler: Arc<UserDecryptHandler<D>>,
    Path(job_id): Path<Uuid>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.user_decrypt_get_v2(Path(job_id)).await
}
