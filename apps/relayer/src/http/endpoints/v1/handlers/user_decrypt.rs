use super::super::types::user_decrypt::{
    UserDecryptErrorResponseJson, UserDecryptRequestJson, UserDecryptResponseJson,
};
use crate::core::errors::{EventProcessingError, READINESS_CHECK_TIMEOUT_MSG};
use crate::core::event::{
    ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptEventId,
    UserDecryptRequest,
};
use crate::core::job_id::JobId;
use crate::http::{parse_and_validate, AppResponse};
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::OnceHandler;
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::store::sql::repositories::user_decrypt_repo::UserDecryptRepository;
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
    http_metrics::with_http_metrics(HttpEndpoint::UserDecrypt, HttpMethod::Post, async move {
        handler.handle(req, &()).await
    })
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
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static>
    UserDecryptHandler<D>
{
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        user_decrypt_repo: Arc<UserDecryptRepository>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            user_decrypt_repo,
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
        http_metrics::with_http_metrics(HttpEndpoint::UserDecrypt, HttpMethod::Post, async move {
            self.handle(req, &()).await
        })
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
        let _assigned_ext_job_id = match self
            .user_decrypt_repo
            .insert_data_on_conflict_and_get_ext_job_id(
                proposed_ext_job_id,
                &int_job_id.as_sha256_hash().unwrap()[..], // Safe to wrap as we just constructed the ID.
                user_decrypt_request.clone(),
            )
            .await
        {
            Ok(assigned_ext_job_id) => assigned_ext_job_id,
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
        // V1 is synchronous - assigned job_id not returned to user

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
                                let response_json = UserDecryptResponseJson::from(decrypt_response.clone());
                                (StatusCode::OK, Json(response_json)).into_response()
                            }
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(UserDecryptErrorResponseJson {
                                    message: "INTERNAL CONVERSION ERROR".to_string(),
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
                                    EventProcessingError::ReadinessCheckFailed => (
                                        StatusCode::GATEWAY_TIMEOUT,
                                        Json(UserDecryptErrorResponseJson {
                                            message: READINESS_CHECK_TIMEOUT_MSG.to_string(),
                                        }),
                                    )
                                        .into_response(),
                                    _ => (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(UserDecryptErrorResponseJson {
                                            message: format!("{error:?}"),
                                        }),
                                    )
                                        .into_response(),
                                }
                            }
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(UserDecryptErrorResponseJson {
                                    message: "INTERNAL CONVERSION ERROR".to_string(),
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
