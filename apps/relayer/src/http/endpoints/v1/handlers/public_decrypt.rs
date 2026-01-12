use super::super::types::public_decrypt::{
    PublicDecryptErrorResponseJson, PublicDecryptRequestJson, PublicDecryptResponseJson,
};
use crate::core::errors::{EventProcessingError, READINESS_CHECK_TIMEOUT_MSG};
use crate::core::event::{
    ApiVersion, PublicDecryptEventData, PublicDecryptEventId, PublicDecryptRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::gateway::arbitrum::transaction::tx_throttler::{GatewayTxTask, TxThrottlingSender};
use crate::gateway::readiness_check::readiness_throttler::{
    PublicDecryptReadinessTask, ReadinessSender,
};
use crate::http::utils::public_decrypt_bounce_check;
use crate::http::{parse_and_validate, AppResponse};
use crate::metrics::http::{self as http_metrics, HttpApiVersion, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::OnceHandler;
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::store::sql::repositories::public_decrypt_repo::PublicDecryptRepository;
use axum::{body::Bytes as AxumBytes, extract::FromRequest, http::Request, response::IntoResponse};
use axum::{http::StatusCode, Json};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, Level};

pub type PublicDecryptResponse = AppResponse<PublicDecryptResponseJson>;

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
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            public_decrypt_repo,
            retry_after_seconds,
            tx_throttler,
            public_decrypt_readiness_throttler,
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
        let request_id = self.orchestrator.new_internal_request_id();
        let _span = span!(Level::INFO, "handle-public-decrypt-req", request_id = %request_id);

        info!(
            "Handling public decryption request, generated request id: {}",
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

        let int_job_id = JobId::from_sha256_hash(request.content_hash());

        // Queue full Bouncing logic.
        let active_external_job_id = self
            .public_decrypt_repo
            .find_active_ext_ref_by_int_job_id(&int_job_id.as_sha256_hash().unwrap()[..])
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
                        info!("Public decrypt v1 is bounced by full queue");
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
        let _assigned_ext_job_id = match self
            .public_decrypt_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                &int_job_id.as_sha256_hash().unwrap()[..], // Safe to wrap as we just constructed the ID.
                request.clone(),
            )
            .await
        {
            Ok(assigned_ext_job_id) => assigned_ext_job_id,
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
        // V1 is synchronous - assigned job_id not returned to user

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
        info!("Dispatched event to orchestrator to initiate processing");

        info!("Waiting for public decrypt response or error event");

        use futures::pin_mut;
        pin_mut!(response_rx);
        pin_mut!(error_rx);

        tokio::select! {
            res = &mut response_rx => {
                match res {
                    Ok(event) => {
                        info!("Received public decrypt response event");
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
