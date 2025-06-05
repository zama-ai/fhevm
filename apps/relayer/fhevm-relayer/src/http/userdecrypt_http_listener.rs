use crate::core::event::{
    ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptEventId,
    UserDecryptRequest,
};
use crate::core::utils::OnceHandler;
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::primitives::Bytes;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::info;
use tracing::{error, instrument, span, Level};

/// Represents the payload coming into the endpoint for user decrypt.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct UserDecryptRequestJson {
    pub handleContractPairs: Vec<HandleContractPairJson>,
    pub requestValidity: RequestValidityJson,
    pub contractsChainId: String,
    pub contractAddresses: Vec<String>,
    pub userAddress: String,
    pub signature: String,
    pub publicKey: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct HandleContractPairJson {
    pub handle: String,
    pub contractAddress: String,
}

impl Display for HandleContractPairJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ct-handle: {}, contract-address: {}",
            self.handle, self.contractAddress
        )
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct RequestValidityJson {
    pub startTimestamp: String,
    pub durationDays: String,
}

impl UserDecryptRequestJson {
    pub fn validate(&self) -> Result<(), String> {
        // Add other validations here.
        Ok(())
    }
}

/// Represents the response from the endpoint for user decrypt.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDecryptResponseJson {
    pub response: Vec<UserDecryptResponsePayloadJson>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserDecryptResponsePayloadJson {
    pub payload: Bytes,
    pub signature: Bytes,
}

/// Represents the error response from the endpoint for user decrypt.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserDecryptErrorResponseJson {
    pub message: String,
}

pub struct UserDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> UserDecryptHandler<D> {
    pub fn new(orchestrator: Arc<Orchestrator<D, RelayerEvent>>, api_version: ApiVersion) -> Self {
        Self {
            orchestrator,
            api_version,
        }
    }

    /// Handles requests to the endpoint for user decrypt.
    #[instrument(name="handle-user-decrypt", skip_all, fields(user_address=%payload.userAddress, cts=?payload.handleContractPairs))]
    pub async fn handle(&self, Json(payload): Json<UserDecryptRequestJson>) -> impl IntoResponse {
        info!("Handling user decryption request in http listener");
        // Validate the payload
        if let Err(message) = payload.validate() {
            let error_response = UserDecryptErrorResponseJson { message };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }

        let user_decrypt_request = match UserDecryptRequest::try_from(payload.clone()) {
            Ok(request) => request,
            Err(error) => {
                error!("Conversion failed: {}", error);
                // Try to identify exactly where it's failing
                if let Err(e) = serde_json::to_string(&payload) {
                    error!("Cannot serialize payload: {}", e);
                }
                // Try parsing individual fields
                if let Err(e) = payload.requestValidity.durationDays.parse::<u32>() {
                    error!("Failed to parse durationDays: {}", e);
                }

                let error_response = UserDecryptErrorResponseJson {
                    message: format!("parsing request data: {}", error),
                };
                return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
            }
        };

        let request_id = self.orchestrator.new_request_id();
        let _span = span!(Level::INFO, "handle-user-decrypt-req", request_id = %request_id); // Add other relevant top-level details

        info!("Validated and assigned request id: {}", request_id);

        // Register once handlers for receiving the decryption response from the gateway.
        let (response_handler, response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let response_handler = Arc::new(response_handler);

        self.orchestrator.register_once_handler(
            UserDecryptEventId::RespRcvdFromGw.into(),
            request_id,
            response_handler,
        );
        info!("Registered once handler for user decrypt response");

        // Register once handler for error/failure event
        let (error_handler, error_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let error_handler = Arc::new(error_handler);

        self.orchestrator.register_once_handler(
            UserDecryptEventId::Failed.into(),
            request_id,
            error_handler,
        );
        info!("Registered once handler for user decrypt failure");

        let request_data = UserDecryptEventData::ReqRcvdFromUser {
            decrypt_request: user_decrypt_request,
        };
        let event = RelayerEvent::new(
            request_id,
            self.api_version,
            RelayerEventData::UserDecrypt(request_data),
        );
        let _ = self.orchestrator.dispatch_event(event).await;
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
                                let response_json = UserDecryptResponseJson::from(decrypt_response);
                                info!("Sending success reponse to user");
                                (StatusCode::OK, Json(response_json)).into_response()
                            },
                            _ => {
                                let error_response = UserDecryptErrorResponseJson {
                                    message: "unexpected error".to_string(),
                                };
                                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                            }
                        }
                    }
                    Err(_) => {
                        info!("Received error while waiting for user decrypt response event");
                        let error_response = UserDecryptErrorResponseJson {
                            message: "Failed to receive response from the gateway.".to_string(),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                    }
                }
            }
            res = &mut error_rx => {
                match res {
                    Ok(_event) => {
                        info!("Received error event on error_rx");
                        let error_response = UserDecryptErrorResponseJson {
                            message: "REQUEST FAILED RESPONSE".to_string(),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                    }
                    Err(_) => {
                        info!("Received error while waiting for error event on error_rx");
                        let error_response = UserDecryptErrorResponseJson {
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

impl Serialize for UserDecryptResponsePayloadJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UserDecryptResponsePayloadJson", 2)?;
        state.serialize_field("payload", &serialize_vec_as_hex(&self.payload.to_vec()))?;
        state.serialize_field("signature", &serialize_vec_as_hex(&self.signature.to_vec()))?;
        state.end()
    }
}

fn serialize_vec_as_hex(vec: &Vec<u8>) -> String {
    hex::encode(vec)
}
