use crate::core::event::{
    ApiCategory, ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData,
    UserDecryptEventId, UserDecryptRequest,
};
use crate::core::utils::{colorize_event_type, colorize_request_id, OnceHandler};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::primitives::Bytes;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::info;

/// Represents the payload coming into the '/input-proof' endpoint.
#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct UserDecryptRequestJson {
    pub signature: String,
    pub userAddress: String,
    pub enc_key: String,
    pub ct_handle: String,
    pub contractAddress: String,
    pub chainId: String,
}

impl UserDecryptRequestJson {
    fn validate(&self) -> Result<(), String> {
        // Add other validations here.
        Ok(())
    }
}

/// Represents the response from the '/input-proof' endpoint.
#[derive(Debug, Serialize)]
pub struct UserDecryptResponseJson {
    pub response: UserDecryptResponsePayloadJson,
}

#[derive(Debug, Serialize)]
pub struct UserDecryptResponsePayloadJson {
    pub reencrypted_shares: Vec<Bytes>,
    pub signatures: Vec<Bytes>,
}

/// Represents the error response from the '/input-proof' endpoint.
#[derive(Debug, Serialize)]
pub struct UserDecryptErrorResponseJson {
    pub message: String,
}

pub struct UserDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> UserDecryptHandler<D> {
    pub fn new(orchestrator: Arc<Orchestrator<D, RelayerEvent>>) -> Self {
        Self { orchestrator }
    }

    /// Handles POST requests to '/input-proof'. This function is responsible only for handling
    /// the validated request and returning the corresponding response.
    pub async fn handle(&self, Json(payload): Json<UserDecryptRequestJson>) -> impl IntoResponse {
        info!("Handling user decryption request in http listener");
        // Validate the payload
        if let Err(message) = payload.validate() {
            let error_response = UserDecryptErrorResponseJson { message };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }

        let user_decrypt_request: UserDecryptRequest =
            match UserDecryptRequest::try_from(payload.clone()) {
                Ok(request) => request,
                Err(error) => {
                    let error_response = UserDecryptErrorResponseJson {
                        message: error.to_string(),
                    };
                    return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
                }
            };

        // Generate Request ID
        let request_id = self.orchestrator.new_request_id();

        info!("Validated and assigned request id: {}", request_id);

        // Register once handlers for receiving the decryption response from the gateway l2
        let (handler, rx): (OnceHandler<RelayerEvent>, oneshot::Receiver<RelayerEvent>) =
            OnceHandler::new();
        let handler = Arc::new(handler);

        self.orchestrator.register_once_handler(
            UserDecryptEventId::RespRcvdFromGw.into(),
            request_id,
            handler,
        );
        info!("Registered once handler");

        let request_data = UserDecryptEventData::ReqRcvdFromUser {
            decrypt_request: user_decrypt_request,
        };
        let event = RelayerEvent::new(
            request_id,
            ApiVersion {
                category: ApiCategory::PRODUCTION,
                number: 1,
            },
            RelayerEventData::UserDecrypt(request_data),
        );
        let _ = self.orchestrator.dispatch_event(event).await;
        info!("Dispatched event to orchestrator to initiate processing");

        info!("Waiting for user decrypt reponse event");
        // TODO(Mano): Handle failed event as well.
        // Wait for response on the rx of Onshot channel.
        let event = match rx.await {
            Ok(event) => {
                info!("Received user decrypt response event");
                event
            }
            Err(_) => {
                info!("Received errror while waiting for response event");
                let error_response = UserDecryptErrorResponseJson {
                    message: "Failed to receive response from the gateway l2.".to_string(),
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response();
            }
        };

        info!(
            event_type = %colorize_event_type(event.data.as_ref()),
            request_id = %colorize_request_id(&event.request_id),
            "Processing http event"
        );

        info!("response event type {:?}", event.data);
        match event.data {
            RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
                decrypt_response,
            }) => match UserDecryptResponseJson::try_from(decrypt_response) {
                Ok(response_json) => {
                    info!("Sending success reponse to user");
                    (StatusCode::OK, Json(response_json)).into_response()
                }
                Err(error) => {
                    info!(
                        "sending error reponse to user as response event cannot be decoded: {}",
                        error
                    );
                    let error_response = UserDecryptErrorResponseJson {
                        message: "request could not be completed".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
            },
            _ => {
                let error_response = UserDecryptErrorResponseJson {
                    message: "unexpected error".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json;

//     #[test]
//     fn test_deserialize_input_proof_request_json() {
//         // Define a sample JSON input.
//         let json_data = r#"
//         {
//                    "contractChainId": "123456",
//                    "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
//                    "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
//                    "ciphertextWithZkpok": "abcdef"
//         }
//         "#;

//         // Deserialize the JSON string into the struct.
//         let request: UserDecryptRequestJson =
//             serde_json::from_str(json_data).expect("JSON deserialization failed");

//         // Assert that each field was deserialized correctly.
//         assert_eq!(request.contractChainId, "123456");
//         assert_eq!(
//             request.contractAddress,
//             "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d"
//         );
//         assert_eq!(
//             request.userAddress,
//             "0x12B064FB845C1cc05e9493856a1D637a73e944bE"
//         );
//         assert_eq!(request.ciphertextWithZkpok, "abcdef");
//     }
// }
