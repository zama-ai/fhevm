use crate::core::event::{
    ApiVersion, InputProofEventData, InputProofEventId, InputProofRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::utils::OnceHandler;
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::info;

/// Represents the payload coming into the '/input-proof' endpoint.
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct InputProofRequestJson {
    pub contractChainId: String, // Hex encoded uint256 string with 0x prefix.
    pub contractAddress: String, // Hex encoded address with 0x prefix.
    pub userAddress: String,     // Hex encoded address with 0x prefix.
    pub ciphertextWithInputVerification: String, // List of hex encoded binary proof without 0x prefix.
}

impl InputProofRequestJson {
    pub fn validate(&self) -> Result<(), String> {
        // Add other validations here.
        if self.ciphertextWithInputVerification.is_empty() {
            Err("Input Verification cannot be empty.".to_string())
        } else {
            Ok(())
        }
    }
}

/// Represents the response from the '/input-proof' endpoint.
#[derive(Debug, Serialize)]
pub struct InputProofResponseJson {
    pub response: InputProofResponsePayloadJson,
}

#[derive(Debug, Serialize)]
pub struct InputProofResponsePayloadJson {
    pub handles: Vec<String>, // Ordered List of hex encoded handles with 0x prefix.
    pub signatures: Vec<String>, // Attestation signatures for Input verification for the ordered list of handles.
}

/// Represents the error response from the '/input-proof' endpoint.
#[derive(Debug, Serialize)]
pub struct InputProofErrorResponseJson {
    pub message: String,
}

pub struct InputProofHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> InputProofHandler<D> {
    pub fn new(orchestrator: Arc<Orchestrator<D, RelayerEvent>>, api_version: ApiVersion) -> Self {
        Self {
            orchestrator,
            api_version,
        }
    }

    /// Handles POST requests to '/input-proof'. This function is responsible only for handling
    /// the validated request and returning the corresponding response.
    pub async fn handle(&self, Json(payload): Json<InputProofRequestJson>) -> impl IntoResponse {
        info!("Handling input proof request");
        // Validate the payload
        if let Err(message) = payload.validate() {
            let error_response = InputProofErrorResponseJson { message };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }

        // Generate Request ID
        let request_id = self.orchestrator.new_request_id();

        info!("Validated and assigned request id: {}", request_id);

        // Register once handlers for receiving the decryption response from the gateway l2
        let (handler, rx): (OnceHandler<RelayerEvent>, oneshot::Receiver<RelayerEvent>) =
            OnceHandler::new();
        let handler = Arc::new(handler);

        self.orchestrator.register_once_handler(
            InputProofEventId::RespRcvdFromGw.into(),
            request_id,
            handler,
        );
        info!("Registered once handler");

        // Prepare and send an event
        let request_data: InputProofRequest = match payload.try_into() {
            Ok(event_data) => event_data,
            Err(message) => {
                let error_response = InputProofErrorResponseJson { message };
                return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
            }
        };
        let request_data = InputProofEventData::ReqRcvdFromUser {
            input_proof_request: request_data,
        };

        let event = RelayerEvent::new(
            request_id,
            self.api_version.clone(),
            RelayerEventData::InputProof(request_data),
        );
        let _ = self.orchestrator.dispatch_event(event).await;
        info!("dispatched event to orchestrator to initiate processing");

        info!("waiting for reponse event");
        // Wait for response on the rx of Onshot channel.
        let event = match rx.await {
            Ok(event) => {
                info!("received response event");
                event
            }
            Err(_) => {
                info!("received errror while waiting for response event");
                let error_response = InputProofErrorResponseJson {
                    message: "Failed to receive response from the gateway l2.".to_string(),
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response();
            }
        };

        info!("Response event type {:?}", event.data);
        match event.data {
            RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                input_proof_response,
            }) => match InputProofResponseJson::try_from(input_proof_response) {
                Ok(response_json) => {
                    info!("Sending success reponse to user");
                    (StatusCode::OK, Json(response_json)).into_response()
                }
                Err(_) => {
                    info!("sending error reponse to user as response event cannot be decoded");
                    let error_response = InputProofErrorResponseJson {
                        message: "request could not be completed 2".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
            },
            _ => {
                info!("sending error reponse to user as response event is not expected type");
                let error_response = InputProofErrorResponseJson {
                    message: "request could not be completed 3".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_input_proof_request_json() {
        // Define a sample JSON input.
        let json_data = r#"
        {
                   "contractChainId": "123456",
                   "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
                   "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
                   "ciphertextWithInputVerification": "abcdef"
        }
        "#;

        // Deserialize the JSON string into the struct.
        let request: InputProofRequestJson =
            serde_json::from_str(json_data).expect("JSON deserialization failed");

        // Assert that each field was deserialized correctly.
        assert_eq!(request.contractChainId, "123456");
        assert_eq!(
            request.contractAddress,
            "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d"
        );
        assert_eq!(
            request.userAddress,
            "0x12B064FB845C1cc05e9493856a1D637a73e944bE"
        );
        assert_eq!(request.ciphertextWithInputVerification, "abcdef");
    }
}
