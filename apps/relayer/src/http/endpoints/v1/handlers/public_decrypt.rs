use super::super::types::public_decrypt::{
    PublicDecryptErrorResponseJson, PublicDecryptRequestJson, PublicDecryptResponseJson,
};
use crate::core::errors::{EventProcessingError, READINESS_CHECK_TIMEOUT_MSG};
use crate::core::event::{
    ApiVersion, PublicDecryptEventData, PublicDecryptEventId, PublicDecryptRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::host::HostChainIdChecker;
use crate::http::utils::BounceChecker;
use crate::http::{parse_and_validate, AppResponse};
use crate::logging::PublicDecryptStep;
use crate::metrics::http::{self as http_metrics, HttpApiVersion, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::OnceHandler;
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::readiness::throttler::PublicDecryptReadinessTask;
use crate::store::sql::repositories::public_decrypt_repo::{
    PublicDecryptInsertResult, PublicDecryptRepository,
};
use axum::{body::Bytes as AxumBytes, extract::FromRequest, http::Request, response::IntoResponse};
use axum::{http::StatusCode, Json};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, Level};
use uuid::Uuid;

pub type PublicDecryptResponse = AppResponse<PublicDecryptResponseJson>;

pub struct PublicDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
    bounce_checker: BounceChecker<PublicDecryptReadinessTask>,
    host_chain_id_checker: Arc<HostChainIdChecker>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static>
    PublicDecryptHandler<D>
{
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        public_decrypt_repo: Arc<PublicDecryptRepository>,
        bounce_checker: BounceChecker<PublicDecryptReadinessTask>,
        host_chain_id_checker: Arc<HostChainIdChecker>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            public_decrypt_repo,
            bounce_checker,
            host_chain_id_checker,
        }
    }

    /// Create router with public decrypt routes
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new().route(
            "/v1/public-decrypt",
            axum::routing::post({
                let handler = self.clone();
                move |req| async move { handler.public_decrypt_v1(req).await }
            }),
        )
    }

    pub async fn public_decrypt_v1(&self, req: Request<axum::body::Body>) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::PublicDecrypt,
            HttpMethod::Post,
            HttpApiVersion::V1,
            req.headers().clone(),
            async move { self.handle(req, &()).await },
        )
        .await
        .into_response()
    }

    #[instrument(name = "handle-public-decrypt", skip_all, fields(request_id))]
    pub async fn handle<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = Uuid::new_v4();
        let _span = span!(Level::INFO, "handle-public-decrypt-req", request_id = %request_id);

        info!(
            step = %PublicDecryptStep::ReqReceived,
            request_id = %request_id,
            "Handling public decryption request"
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

        // Check early to avoid filling the queue with handles of unsupported chains
        if let Err(chain_id) = self
            .host_chain_id_checker
            .validate_handles(&request.ct_handles)
        {
            let mut resp = AppResponse::<()>::host_chain_id_not_supported(chain_id);
            resp.set_request_id(&request_id.to_string());
            return resp.into_response();
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
                            int_job_id = %int_job_id,
                            "Public decrypt v1 is bounced by full queue"
                        );
                        // NOTE: Could return 500 to not change the behaviour of the v1.
                        // Here return 429
                        return AppResponse::<()>::protocol_overloaded(
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
            PublicDecryptEventId::RespRcvdFromGw.into(),
            int_job_id,
            response_handler,
        );
        info!("Registered once handler for response");

        let (error_handler, error_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let error_handler = Arc::new(error_handler);

        self.orchestrator.register_once_handler(
            PublicDecryptEventId::Failed.into(),
            int_job_id,
            error_handler,
        );
        info!("Registered once handler for error");

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

        // Extract ext_job_id from any variant for logging
        let assigned_ext_job_id = match &insert_result {
            PublicDecryptInsertResult::Inserted { ext_job_id } => *ext_job_id,
            PublicDecryptInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            PublicDecryptInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        // Handle the three cases based on insert result
        match insert_result {
            PublicDecryptInsertResult::Inserted { ext_job_id: _ } => {
                // New request - dispatch event to orchestrator
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
                    int_job_id = %int_job_id,
                    "Dispatched event to orchestrator"
                );
            }
            PublicDecryptInsertResult::DuplicateCompleted {
                ext_job_id: _,
                response,
            } => {
                // Duplicate request that already completed - return immediately
                info!(
                    step = %PublicDecryptStep::DedupHit,
                    req_id = %request_id,
                    ext_job_id = %assigned_ext_job_id,
                    int_job_id = %int_job_id,
                    "Returning cached response"
                );

                // Clean up handlers to prevent memory leak
                self.orchestrator.unregister_once_handler(
                    PublicDecryptEventId::RespRcvdFromGw.into(),
                    int_job_id,
                );
                self.orchestrator
                    .unregister_once_handler(PublicDecryptEventId::Failed.into(), int_job_id);

                let response_json = PublicDecryptResponseJson::from(response);
                return (StatusCode::OK, Json(response_json)).into_response();
            }
            PublicDecryptInsertResult::DuplicateProcessing { ext_job_id: _ } => {
                // Duplicate request still processing - just wait for response
                info!(
                    step = %PublicDecryptStep::DedupHit,
                    req_id = %request_id,
                    ext_job_id = %assigned_ext_job_id,
                    int_job_id = %int_job_id,
                    "Duplicate request detected"
                );
            }
        }

        info!("Waiting for public decrypt response or error event");

        use futures::pin_mut;
        pin_mut!(response_rx);
        pin_mut!(error_rx);

        tokio::select! {
            res = &mut response_rx => {
                match res {
                    Ok(event) => {
                        info!(
                            step = %PublicDecryptStep::RespSent,
                            int_job_id = %int_job_id,
                            "Received public decrypt response event"
                        );
                        info!("Response event type {:?}", event.data);
                        match event.data {
                            RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespRcvdFromGw {
                                decrypt_response,
                            }) => {
                                let response_json = PublicDecryptResponseJson::from(decrypt_response.clone());
                                (StatusCode::OK, Json(response_json)).into_response()
                            }
                            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                                .into_response(),
                        }
                    }
                    Err(_) => {
                        info!("Received error while waiting for response event");
                        let error_response = PublicDecryptErrorResponseJson {
                            message: "Failed to receive response from the gateway.".to_string(),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                    }
                }
            }
            res = &mut error_rx => {
                match res {
                    Ok(event) => {
                        match event.data {
                            RelayerEventData::PublicDecrypt(PublicDecryptEventData::Failed { error }) => {
                                match error {
                                    EventProcessingError::RequestReverted(fhevm_error) => (
                                        StatusCode::BAD_REQUEST,
                                        Json(PublicDecryptErrorResponseJson {
                                            message: format!("Request reverted: {fhevm_error:?}"),
                                        }),
                                    )
                                        .into_response(),
                                    EventProcessingError::NotAllowedOnHostAcl(reason) => (
                                        StatusCode::BAD_REQUEST,
                                        Json(PublicDecryptErrorResponseJson {
                                            message: format!("Not allowed on host ACL: {reason}"),
                                        }),
                                    )
                                        .into_response(),
                                    EventProcessingError::ReadinessCheckTimedOut => (
                                        StatusCode::SERVICE_UNAVAILABLE,
                                        Json(PublicDecryptErrorResponseJson {
                                            message: READINESS_CHECK_TIMEOUT_MSG.to_string(),
                                        }),
                                    )
                                        .into_response(),
                                    _ => (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(PublicDecryptErrorResponseJson {
                                            message: format!("Internal Server Error: {error:?}"),
                                        }),
                                    )
                                        .into_response(),
                                }
                            }
                            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                                .into_response(),
                        }
                    }
                    Err(_) => {
                        info!("Received error while waiting for error event on error_rx");
                        let error_response = PublicDecryptErrorResponseJson {
                            message: "Failed to receive error response from the gateway.".to_string(),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                    }
                }
            }
        }
    }
}

/// Public decryption v1 endpoint - Requests public decryption
#[utoipa::path(
post,
path = "/v1/public-decrypt",
request_body = PublicDecryptRequestJson,
responses(
    (status = 200, description = "Successfully decrypted", body = PublicDecryptResponseJson),
    (status = 400, description = "Malformed JSON or validation failed", body = crate::http::ErrorResponse),
    (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
    (status = 500, description = "Internal server error", body = crate::http::ErrorResponse),
),
)]
pub async fn public_decrypt_v1<D>(
    handler: Arc<PublicDecryptHandler<D>>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.public_decrypt_v1(req).await
}
