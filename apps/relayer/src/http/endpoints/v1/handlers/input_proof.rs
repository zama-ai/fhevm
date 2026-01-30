use super::super::types::input_proof::{
    InputProofErrorResponseJson, InputProofRequestJson, InputProofResponseJson,
};
use crate::core::errors::EventProcessingError;
use crate::core::event::{
    ApiVersion, InputProofEventData, InputProofEventId, InputProofRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::gateway::arbitrum::transaction::tx_throttler::{GatewayTxTask, TxThrottlingSender};
use crate::http::utils::bounce_check;
use crate::http::{parse_and_validate, AppResponse};
use crate::logging::InputProofStep;
use crate::metrics::http::{self as http_metrics, HttpApiVersion, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::ContentHasher;
use crate::orchestrator::OnceHandler;
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::input_proof_repo::{
    InputProofInsertResult, InputProofRepository,
};
use axum::{body::Bytes, extract::FromRequest, http::Request, response::IntoResponse};
use axum::{http::StatusCode, Json};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, warn, Level};
use uuid::Uuid;

pub type InputProofResponse = AppResponse<super::super::types::input_proof::InputProofResponseJson>;

pub struct InputProofHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    input_proof_repo: Arc<InputProofRepository>,
    retry_after_seconds: u32,
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
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
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            input_proof_repo,
            retry_after_seconds,
            tx_throttler,
        }
    }

    /// Create router with input proof routes
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new().route(
            "/v1/input-proof",
            axum::routing::post({
                let handler = self.clone();
                move |req| async move { handler.input_proof_v1(req).await }
            }),
        )
    }

    pub async fn input_proof_v1(&self, req: Request<axum::body::Body>) -> impl IntoResponse {
        http_metrics::with_http_metrics(
            HttpEndpoint::InputProof,
            HttpMethod::Post,
            HttpApiVersion::V1,
            async move { self.handle(req, &()).await },
        )
        .await
        .into_response()
    }

    #[instrument(name = "handle-input", skip_all, fields(request_id))]
    pub async fn handle<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        let request_id = Uuid::new_v4();
        let _span = span!(Level::INFO, "handle-input-req", request_id = %request_id);

        info!(
            step = %InputProofStep::ReqReceived,
            int_job_id = %request_id,
            "Handling input proof v1 request"
        );

        let body = match Bytes::from_request(req, _state).await {
            Ok(body) => body,
            Err(_) => {
                let mut response = AppResponse::<()>::request_error("Failed to read request body");
                response.set_request_id(&request_id.to_string());
                return response.into_response();
            }
        };

        let request_data: InputProofRequest =
            match parse_and_validate::<InputProofRequestJson, InputProofRequest>(&body) {
                Ok(request) => request,
                Err(parse_error) => {
                    let error_response: AppResponse<()> =
                        parse_error.to_app_response(&request_id.to_string());
                    return error_response.into_response();
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
                            "Input proof v1 request bounced by full queue"
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
                    "Failed to check for active input proof request in database: {}",
                    e
                );
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
                .into_response();
            }
        }

        let (gateway_response_handler, gateway_response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let gateway_response_handler = Arc::new(gateway_response_handler);

        self.orchestrator.register_once_handler(
            InputProofEventId::RespRcvdFromGw.into(),
            int_job_id,
            gateway_response_handler,
        );
        info!("Registered once handler for handling input proof gateway response");

        let (error_handler, error_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let error_handler = Arc::new(error_handler);

        self.orchestrator.register_once_handler(
            InputProofEventId::Failed.into(),
            int_job_id,
            error_handler,
        );
        info!("Registered once handler for handling input proof failure");

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
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
                .into_response();
            }
        };

        // Extract ext_job_id from any variant for logging
        let assigned_ext_job_id = match &insert_result {
            InputProofInsertResult::Inserted { ext_job_id } => *ext_job_id,
            InputProofInsertResult::DuplicateCompleted { ext_job_id, .. } => *ext_job_id,
            InputProofInsertResult::DuplicateProcessing { ext_job_id } => *ext_job_id,
        };

        // Handle DuplicateCompleted immediately - return cached response
        if let InputProofInsertResult::DuplicateCompleted {
            accepted, response, ..
        } = insert_result
        {
            info!(
                step = %InputProofStep::DedupHit,
                req_id = %request_id,
                ext_job_id = %assigned_ext_job_id,
                int_job_id = %int_job_id,
                "Returning cached response"
            );
            if accepted {
                if let Some(resp) = response {
                    let response_json = InputProofResponseJson::from(resp);
                    return (StatusCode::OK, Json(response_json)).into_response();
                } else {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(InputProofErrorResponseJson {
                            message: "Internal error: accepted proof with no response data"
                                .to_string(),
                        }),
                    )
                        .into_response();
                }
            } else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(InputProofErrorResponseJson {
                        message: "Proof Rejected".to_string(),
                    }),
                )
                    .into_response();
            }
        }

        // Only dispatch event for newly inserted requests
        if matches!(insert_result, InputProofInsertResult::Inserted { .. }) {
            let event_data = InputProofEventData::ReqRcvdFromUser {
                input_proof_request: request_data,
            };

            let event = RelayerEvent::new(
                int_job_id,
                self.api_version,
                RelayerEventData::InputProof(event_data),
            );
            let _ = self.orchestrator.dispatch_event(event).await;
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

        let _waiting_for_response_span =
            span!(Level::INFO, "waiting-for-response", request_id = %request_id);
        info!("waiting for response event");

        use futures::pin_mut;
        pin_mut!(gateway_response_rx);
        pin_mut!(error_rx);

        tokio::select! {
            res = &mut gateway_response_rx => {
                match res {
                    Ok(event) => {
                        info!("Response event type {:?}", event.data);
                        match event.data {
                            RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                                accepted,
                                input_proof_response,
                            }) => {
                                if accepted {
                                    if let Some(response) = input_proof_response {
                                        let response_json = InputProofResponseJson::from(response.clone());
                                        (StatusCode::OK, Json(response_json)).into_response()
                                    } else {
                                        (
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            Json(InputProofErrorResponseJson {
                                                message: "Internal error: accepted proof with no response data"
                                                    .to_string(),
                                            }),
                                        )
                                            .into_response()
                                    }
                                } else {
                                    (
                                        StatusCode::BAD_REQUEST,
                                        Json(InputProofErrorResponseJson {
                                            message: "Proof Rejected".to_string(),
                                        }),
                                    )
                                        .into_response()
                                }
                            }
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(InputProofErrorResponseJson {
                                    message: "Internal Server Error".to_string(),
                                }),
                            )
                                .into_response(),
                        }
                    }
                    Err(_) => {
                        info!("received error while waiting for response event");
                        return InputProofResponse::internal_server_error("Failed to receive response from the gateway.").into_response();
                    }
                }
            }
            res = &mut error_rx => {
                match res {
                    Ok(event) => {
                        info!("received error event on error_rx");
                        match event.data {
                            RelayerEventData::InputProof(InputProofEventData::Failed { error }) => {
                                match error {
                                    EventProcessingError::RequestReverted(fhevm_error) => (
                                        StatusCode::BAD_REQUEST,
                                        Json(InputProofErrorResponseJson {
                                            message: format!("Request reverted: {fhevm_error:?}"),
                                        }),
                                    )
                                        .into_response(),
                                    EventProcessingError::TransactionError(error) => (
                                        StatusCode::BAD_REQUEST,
                                        Json(InputProofErrorResponseJson {
                                            message: format!("Transaction rejected: {error:?}"),
                                        }),
                                    )
                                        .into_response(),
                                    _ => (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(InputProofErrorResponseJson {
                                            message: format!("Internal Server Error: {error:?}"),
                                        }),
                                    )
                                        .into_response(),
                                }
                            }
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(InputProofErrorResponseJson {
                                    message: "Internal Server Error".to_string(),
                                }),
                            )
                                .into_response(),
                        }
                    }
                    Err(_) => {
                        info!("received error while waiting for error event on error_rx");
                        return InputProofResponse::internal_server_error("Failed to receive error response from the gateway.").into_response();
                    }
                }
            }
        }
    }
}

/// Input proof v1 endpoint - Requests input proof verification
#[utoipa::path(
post,
path = "/v1/input-proof",
request_body = InputProofRequestJson,
responses(
    (status = 200, description = "Successfully verified input proof", body = InputProofResponseJson),
    (status = 400, description = "Malformed JSON or validation failed", body = crate::http::ErrorResponse),
    (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
    (status = 500, description = "Internal server error", body = crate::http::ErrorResponse),
),
)]
pub async fn input_proof_v1<D>(
    handler: Arc<InputProofHandler<D>>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    handler.input_proof_v1(req).await
}
