use crate::core::event::{
    ApiVersion, PublicDecryptEventData, PublicDecryptEventId, PublicDecryptRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::http::utils::{parse_and_validate, AppResponse, OnceHandler};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::{IndexerIdGenerator, Orchestrator};
use crate::store::sql::repositories::public_decrypt_repo::PublicDecryptRepository;
use alloy::primitives::Bytes;
use axum::{
    body::Bytes as AxumBytes,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, Level};
use utoipa::ToSchema;
use validator::Validate;

/// Represents the payload coming into the '/input-proof' endpoint.
#[derive(Debug, Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptRequestJson {
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::utils::validate_0x_hexs")
    )]
    pub ciphertext_handles: Vec<String>,
    /// Extra data field, always set to 0x00
    #[schema(value_type = String, example = "0x00")]
    #[validate(custom(function = "crate::http::utils::validate_extra_data_field"))]
    pub extra_data: String,
}

/// Represents the response from the '/input-proof' endpoint.
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct PublicDecryptResponseJson {
    pub response: Vec<PublicDecryptResponsePayloadJson>,
}

#[derive(Debug, Clone, ToSchema)]
pub struct PublicDecryptResponsePayloadJson {
    #[schema(value_type = String)]
    pub decrypted_value: Bytes,
    #[schema(value_type = Vec<String>)]
    pub signatures: Vec<Bytes>,
    #[schema(value_type = String)]
    pub extra_data: Bytes,
}

pub type PublicDecryptResponse = AppResponse<PublicDecryptResponseJson>;

/// Represents the error response from the '/input-proof' endpoint.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct PublicDecryptErrorResponseJson {
    pub message: String,
}

pub struct PublicDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> PublicDecryptHandler<D> {
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        public_decrypt_repo: Arc<PublicDecryptRepository>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            public_decrypt_repo,
        }
    }

    /// Handles requests to the endpoint for public decrypt.
    #[instrument(name = "handle-public-decrypt", skip_all, fields(request_id))]
    pub async fn handle<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        // Generate request ID first so it's available for all error responses
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

        // Register once handlers for receiving the decryption response from the gateway
        let (response_handler, response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let response_handler = Arc::new(response_handler);

        self.orchestrator.register_once_handler(
            PublicDecryptEventId::RespRcvdFromGw.into(),
            JobId::from_uuid_v7(request_id),
            response_handler,
        );
        info!("Registered once handler for response");

        // Register once handler for error/failure event
        let (error_handler, error_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let error_handler = Arc::new(error_handler);

        self.orchestrator.register_once_handler(
            PublicDecryptEventId::Failed.into(),
            JobId::from_uuid_v7(request_id),
            error_handler,
        );
        info!("Registered once handler for error");

        let ext_reference_id = self.orchestrator.new_ext_reference_id();
        let int_indexer_id = request.compute_indexer_id();
        let request_json = match serde_json::to_value(request.clone()) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize request data to JSON: {}", e);
                return AppResponse::<()>::internal_server_error_with_request_id(
                    request_id.to_string(),
                )
                .into_response();
            }
        };

        if let Err(e) = self
            .public_decrypt_repo
            .insert_data_on_conflict_and_get_ext_reference_id(
                ext_reference_id,
                &int_indexer_id[..],
                request_json,
            )
            .await
        {
            error!(
                "Failed to insert/get public decrypt into/from database: {}",
                e
            );
            return AppResponse::<()>::internal_server_error_with_request_id(
                request_id.to_string(),
            )
            .into_response();
        }

        let event_data = PublicDecryptEventData::ReqRcvdFromUser {
            decrypt_request: request,
        };
        let event = RelayerEvent::new(
            JobId::from_uuid_v7(request_id),
            self.api_version,
            RelayerEventData::PublicDecrypt(event_data),
        );
        let _ = self.orchestrator.dispatch_event(event).await;
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
                let response_json = PublicDecryptResponseJson::from(decrypt_response);
                info!("Sending success response to public");
                PublicDecryptResponse::success(response_json).into_response()
            },
                            _ => {
                                let msg = "Unexpected event data type received";
                                error!(msg);
                                PublicDecryptResponse::internal_server_error(msg).into_response()
                            }
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
                        event.into_response()
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

use serde::ser::SerializeStruct;
use serde::Serializer;

impl Serialize for PublicDecryptResponsePayloadJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PublicDecryptResponsePayloadJson", 2)?;
        // Convert decrypted_value first field to "payload" similarly to the UserDecryptResponsePayloadJson struct
        state.serialize_field(
            "decrypted_value",
            &serialize_vec_as_hex(&self.decrypted_value.to_vec()),
        )?;
        let signatures_hex: Vec<String> = self
            .signatures
            .iter()
            .map(|bytes| serialize_vec_as_hex(&bytes.to_vec()))
            .collect();
        state.serialize_field("signatures", &signatures_hex)?;
        state.end()
    }
}

fn serialize_vec_as_hex(vec: &Vec<u8>) -> String {
    hex::encode(vec)
}
