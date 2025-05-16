use crate::core::event::{
    ApiVersion, PublicDecryptEventData, PublicDecryptEventId, PublicDecryptRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::utils::OnceHandler;
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::primitives::Bytes;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::info;
use tracing::{error, instrument, span, Level};

/// Represents the payload coming into the '/input-proof' endpoint.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct PublicDecryptRequestJson {
    pub ciphertextHandles: Vec<String>,
}

impl PublicDecryptRequestJson {
    fn validate(&self) -> Result<(), String> {
        // Add other validations here.
        Ok(())
    }
}

/// Represents the response from the '/input-proof' endpoint.
#[derive(Debug, Serialize)]
pub struct PublicDecryptResponseJson {
    pub response: Vec<PublicDecryptResponsePayloadJson>,
}

#[derive(Debug)]
pub struct PublicDecryptResponsePayloadJson {
    pub decrypted_value: Bytes,
    pub signatures: Vec<Bytes>,
}

/// Represents the error response from the '/input-proof' endpoint.
#[derive(Debug, Serialize)]
pub struct PublicDecryptErrorResponseJson {
    pub message: String,
}

pub struct PublicDecryptHandler<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> PublicDecryptHandler<D> {
    pub fn new(orchestrator: Arc<Orchestrator<D, RelayerEvent>>, api_version: ApiVersion) -> Self {
        Self {
            orchestrator,
            api_version,
        }
    }

    #[instrument(name="handle-public-decrypt", skip_all, fields(handles=?payload.ciphertextHandles))]
    pub async fn handle(&self, Json(payload): Json<PublicDecryptRequestJson>) -> impl IntoResponse {
        info!("Handling public decryption request in http listener");
        // Validate the payload
        if let Err(message) = payload.validate() {
            let error_response = PublicDecryptErrorResponseJson { message };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }

        let public_decrypt_request = match PublicDecryptRequest::try_from(payload.clone()) {
            Ok(request) => request,
            Err(error) => {
                error!("Conversion failed: {}", error);
                // Try to identify exactly where it's failing
                if let Err(e) = serde_json::to_string(&payload) {
                    error!("Cannot serialize payload: {}", e);
                }

                let error_response = PublicDecryptErrorResponseJson {
                    message: format!("parsing request data: {}", error),
                };
                return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
            }
        };

        // Generate Request ID
        let request_id = self.orchestrator.new_request_id();
        let _span = span!(Level::INFO, "handle-public-decrypt-req", request_id = %request_id); // Add other relevant top-level details

        info!("Validated and assigned request id: {}", request_id);

        // Register once handlers for receiving the decryption response from the gateway
        let (handler, rx): (OnceHandler<RelayerEvent>, oneshot::Receiver<RelayerEvent>) =
            OnceHandler::new();
        let handler = Arc::new(handler);

        self.orchestrator.register_once_handler(
            PublicDecryptEventId::RespRcvdFromGw.into(),
            request_id,
            handler,
        );
        info!("Registered once handler");

        let request_data = PublicDecryptEventData::ReqRcvdFromFhevm {
            decrypt_request: public_decrypt_request,
        };
        let event = RelayerEvent::new(
            request_id,
            self.api_version,
            RelayerEventData::PublicDecrypt(request_data),
        );
        let _ = self.orchestrator.dispatch_event(event).await;
        info!("Dispatched event to orchestrator to initiate processing");

        info!("Waiting for public decrypt reponse event");
        // TODO(Mano): Handle failed event as well.
        // Wait for response on the rx of Onshot channel.
        let event = match rx.await {
            Ok(event) => {
                info!("Received public decrypt response event");
                event
            }
            Err(_) => {
                info!("Received errror while waiting for response event");
                let error_response = PublicDecryptErrorResponseJson {
                    message: "Failed to receive response from the gateway.".to_string(),
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response();
            }
        };

        info!("Response event type {:?}", event.data);
        match event.data {
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespRcvdFromGw {
                decrypt_response,
            }) => match PublicDecryptResponseJson::try_from(decrypt_response) {
                Ok(response_json) => {
                    info!("Sending success reponse to public");
                    (StatusCode::OK, Json(response_json)).into_response()
                }
                Err(error) => {
                    info!(
                        "sending error reponse to public as response event cannot be decoded: {}",
                        error
                    );
                    let error_response = PublicDecryptErrorResponseJson {
                        message: "request could not be completed".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
            },
            _ => {
                let error_response = PublicDecryptErrorResponseJson {
                    message: "unexpected error".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_public_decrypt_response_json_serialization() {
        // Create a sample payload with some Bytes values.
        let payload = PublicDecryptResponsePayloadJson {
            decrypted_value: Bytes::from(vec![1, 2, 3, 4]), // should serialize as "01020304"
            signatures: vec![
                Bytes::from(vec![5, 6, 7, 8]), // "05060708"
                Bytes::from(vec![9, 10]),      // "090a"
            ],
        };

        let response = PublicDecryptResponseJson {
            response: vec![payload],
        };

        let serialized = serde_json::to_string(&response).unwrap();

        // The expected JSON string.
        let expected =
            r#"{"response":[{"decrypted_value":"01020304","signatures":["05060708","090a"]}]}"#;

        assert_eq!(serialized, expected);
    }
}
