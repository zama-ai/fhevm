use super::super::types::user_decrypt::{
    UserDecryptErrorResponseJson, UserDecryptRequestJson, UserDecryptResponseJson,
};
use crate::core::errors::{EventProcessingError, READINESS_CHECK_TIMEOUT_MSG};
use crate::core::event::{
    ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptEventId,
    UserDecryptRequest,
};
use crate::core::job_id::JobId;
use crate::gateway::arbitrum::transaction::tx_throttler::{GatewayTxTask, TxThrottlingSender};
use crate::gateway::readiness_check::readiness_throttler::{
    ReadinessSender, UserDecryptReadinessTask,
};
use crate::http::utils::user_decrypt_bounce_check;
use crate::http::{parse_and_validate, AppResponse};
use crate::metrics::http::{self as http_metrics, HttpApiVersion, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::OnceHandler;
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::store::sql::repositories::user_decrypt_repo::{
    UserDecryptInsertResult, UserDecryptRepository,
};
use axum::{body::Bytes as AxumBytes, extract::FromRequest, http::Request, response::IntoResponse};
use axum::{http::StatusCode, Json};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, Level};

pub type UserDecryptResponse =
    AppResponse<super::super::types::user_decrypt::UserDecryptResponseJson>;

/// User decryption v1 endpoint
///
/// Requests a Private decryption
#[utoipa::path(
    post,
    path = "/v1/user-decrypt",
    request_body = UserDecryptRequestJson,
    responses(
        (status = 200, description = "Successfully decrypted", body = UserDecryptResponseJson),
        (status = 400, description = "Malformed JSON or validation failed", body = crate::http::ErrorResponse),
        (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ErrorResponse),
    ),
    )]
pub async fn user_decrypt_v1<D>(
    handler: Arc<UserDecryptHandler<D>>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    http_metrics::with_http_metrics(
        HttpEndpoint::UserDecrypt,
        HttpMethod::Post,
        HttpApiVersion::V1,
        async move { handler.handle(req, &()).await },
    )
    .await
    .into_response()
}

pub struct UserDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    user_decrypt_repo: Arc<UserDecryptRepository>,
    retry_after_seconds: u32,
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static>
    UserDecryptHandler<D>
{
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        user_decrypt_repo: Arc<UserDecryptRepository>,
        retry_after_seconds: u32,
        tx_throttler: TxThrottlingSender<GatewayTxTask>,
        user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            user_decrypt_repo,
            retry_after_seconds,
            tx_throttler,
            user_decrypt_readiness_throttler,
        }
    }

    /// Create router with user decrypt routes
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new().route(
            "/v1/user-decrypt",
            axum::routing::post({
                let handler = self.clone();
                move |req| async move { handler.user_decrypt_v1(req).await }
            }),
        )
    }

    pub async fn user_decrypt_v1(&self, req: Request<axum::body::Body>) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::UserDecrypt,
            HttpMethod::Post,
            HttpApiVersion::V1,
            async move { self.handle(req, &()).await },
        )
        .await
        .into_response()
    }

    #[instrument(name = "handle-user-decrypt", skip_all, fields(request_id))]
    pub async fn handle<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = self.orchestrator.new_internal_request_id();
        let _span = span!(Level::INFO, "handle-user-decrypt-req", request_id = %request_id);

        info!(
            "Handling user decryption request, generated request id: {}",
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

        let int_job_id = JobId::from_sha256_hash(user_decrypt_request.content_hash());

        // Queue full Bouncing logic.
        let active_external_job_id = self
            .user_decrypt_repo
            .find_active_ext_ref_by_int_job_id(&int_job_id.as_sha256_hash().unwrap()[..])
            .await;

        match active_external_job_id {
            Ok(res) => {
                if res.is_none() {
                    // In this case, we check queue full and bounce the request with 429
                    let full = user_decrypt_bounce_check(
                        self.tx_throttler.clone(),
                        self.user_decrypt_readiness_throttler.clone(),
                    )
                    .await;
                    if full {
                        info!("User decrypt v1 is bounced by full queue");
                        // NOTE: Could return 500 to not change the behaviour of the v1.
                        // Here return 429
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
                    "Failed to insert/get user decrypt into/from database: {}",
                    e
                );
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
                .into_response();
            }
        }

        let (response_handler, response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let response_handler = Arc::new(response_handler);

        self.orchestrator.register_once_handler(
            UserDecryptEventId::RespRcvdFromGw.into(),
            int_job_id,
            response_handler,
        );
        info!("Registered once handler for user decrypt response");

        let (error_handler, error_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let error_handler = Arc::new(error_handler);

        self.orchestrator.register_once_handler(
            UserDecryptEventId::Failed.into(),
            int_job_id,
            error_handler,
        );
        info!("Registered once handler for user decrypt failure");

        let proposed_ext_job_id = self.orchestrator.new_ext_job_id();
        let insert_result = match self
            .user_decrypt_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                &int_job_id.as_sha256_hash().unwrap()[..], // Safe to wrap as we just constructed the ID.
                user_decrypt_request.clone(),
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                error!(
                    "Failed to insert/get user decrypt into/from database: {}",
                    e
                );
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
                .into_response();
            }
        };

        // Handle the three cases based on insert result
        match insert_result {
            UserDecryptInsertResult::Inserted { ext_job_id: _ } => {
                // ext_job_id unused - V1 is synchronous, job_id not returned to client
                // New request - dispatch event to orchestrator
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
                    return AppResponse::<()>::internal_server_error_with_request_id(
                        request_id.to_string(),
                    )
                    .into_response();
                }
                info!("Dispatched event to orchestrator to initiate processing");
            }
            UserDecryptInsertResult::DuplicateCompleted {
                ext_job_id: _,
                response,
            } => {
                // ext_job_id unused - V1 is synchronous, job_id not returned to client
                // Duplicate request that already completed - return immediately
                info!("Duplicate request found completed result, returning immediately");

                // Clean up handlers to prevent memory leak
                self.orchestrator
                    .unregister_once_handler(UserDecryptEventId::RespRcvdFromGw.into(), int_job_id);
                self.orchestrator
                    .unregister_once_handler(UserDecryptEventId::Failed.into(), int_job_id);

                match UserDecryptResponseJson::try_from(response) {
                    Ok(response_json) => {
                        return (StatusCode::OK, Json(response_json)).into_response();
                    }
                    Err(e) => {
                        error!(error = %e, "Internal error: failed to convert response");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(UserDecryptErrorResponseJson {
                                message: "Internal Server Error".to_string(),
                            }),
                        )
                            .into_response();
                    }
                }
            }
            UserDecryptInsertResult::DuplicateProcessing { ext_job_id: _ } => {
                // ext_job_id unused - V1 is synchronous, job_id not returned to client
                // Duplicate request still processing - just wait for response
                info!("Duplicate request detected, waiting for original request to complete");
            }
        }

        use futures::pin_mut;
        pin_mut!(response_rx);
        pin_mut!(error_rx);

        info!("Waiting for user decrypt response or error event");
        tokio::select! {
            res = &mut response_rx => {
                match res {
                    Ok(event) => {
                        info!("Received user decrypt response event");
                        info!("Response event type {:?}", event.data);
                        match event.data {
                            RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
                                decrypt_response,
                            }) => {
                                match UserDecryptResponseJson::try_from(decrypt_response.clone()) {
                                    Ok(response_json) => {
                                        (StatusCode::OK, Json(response_json)).into_response()
                                    }
                                    Err(e) => {
                                        error!(error = %e, "Internal error: failed to convert response");
                                        (
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            Json(UserDecryptErrorResponseJson {
                                                message: "Internal Server Error".to_string(),
                                            }),
                                        )
                                            .into_response()
                                    }
                                }
                            }
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(UserDecryptErrorResponseJson {
                                    message: "Internal Server Error".to_string(),
                                }),
                            )
                                .into_response(),
                        }
                    }
                    Err(_) => {
                        info!("Received error while waiting for user decrypt response event");
                        UserDecryptResponse::internal_server_error("Failed to receive response from the gateway.").into_response()
                    }
                }
            }
            res = &mut error_rx => {
                match res {
                    Ok(event) => {
                        match event.data {
                            RelayerEventData::UserDecrypt(UserDecryptEventData::Failed { error }) => {
                                match error {
                                    EventProcessingError::RequestReverted(fhevm_error) => {
                                        let error_response = UserDecryptErrorResponseJson {
                                            message: format!("Request reverted on gateway chain: {fhevm_error:?}"),
                                        };
                                        (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
                                    }
                                    EventProcessingError::ReadinessCheckTimedOut => (
                                        StatusCode::SERVICE_UNAVAILABLE,
                                        Json(UserDecryptErrorResponseJson {
                                            message: READINESS_CHECK_TIMEOUT_MSG.to_string(),
                                        }),
                                    )
                                        .into_response(),
                                    _ => (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(UserDecryptErrorResponseJson {
                                            message: format!("Internal Server Error: {error:?}"),
                                        }),
                                    )
                                        .into_response(),
                                }
                            }
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(UserDecryptErrorResponseJson {
                                    message: "Internal Server Error".to_string(),
                                }),
                            )
                                .into_response(),
                        }
                    }
                    Err(_) => {
                        info!("Received error while waiting for error event on error_rx");
                        UserDecryptResponse::internal_server_error("Failed to receive error response from the gateway.").into_response()
                    }
                }
            }
        }
    }
}
