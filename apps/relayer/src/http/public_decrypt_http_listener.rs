use crate::core::event::{
    ApiVersion, PublicDecryptEventData, PublicDecryptEventId, PublicDecryptRequest, RelayerEvent,
    RelayerEventData,
};
use crate::http::utils::{AppResponse, OnceHandler};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::primitives::Bytes;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{debug, error, info, instrument, span, Level};
use utoipa::ToSchema;
use validator::Validate;

/// Represents the payload coming into the '/input-proof' endpoint.
#[derive(Debug, Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptRequestJson {
    #[validate(
        length(min = 1, message = "ciphertext_handles cannot be empty"),
        custom(function = "crate::http::utils::validate_hex_strings")
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
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> PublicDecryptHandler<D> {
    pub fn new(orchestrator: Arc<Orchestrator<D, RelayerEvent>>, api_version: ApiVersion) -> Self {
        Self {
            orchestrator,
            api_version,
        }
    }

    #[instrument(name="handle-public-decrypt", skip_all, fields(handles=?payload.ciphertext_handles))]
    pub async fn handle(&self, Json(payload): Json<PublicDecryptRequestJson>) -> impl IntoResponse {
        info!("Handling public decryption request in http listener");
        // Validate the payload
        if let Err(errors) = payload.validate() {
            debug!("Validation errors: {:?}", errors);
            return PublicDecryptResponse::bad_request(errors).into_response();
        }

        let public_decrypt_request = match PublicDecryptRequest::try_from(payload.clone()) {
            Ok(request) => request,
            Err(error) => {
                error!("Conversion failed: {}", error);

                return PublicDecryptResponse::unprocessable(format!(
                    "failed to parse request: {error}"
                ))
                .into_response();
            }
        };

        // Generate Request ID
        let request_id = self.orchestrator.new_request_id();
        let _span = span!(Level::INFO, "handle-public-decrypt-req", request_id = %request_id); // Add other relevant top-level details

        info!("Validated and assigned request id: {}", request_id);

        // Register once handlers for receiving the decryption response from the gateway
        let (response_handler, response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let response_handler = Arc::new(response_handler);

        self.orchestrator.register_once_handler(
            PublicDecryptEventId::RespRcvdFromGw.into(),
            request_id,
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
            request_id,
            error_handler,
        );
        info!("Registered once handler for error");

        let request_data = PublicDecryptEventData::ReqRcvdFromUser {
            decrypt_request: public_decrypt_request,
        };
        let event = RelayerEvent::new(
            request_id,
            self.api_version,
            RelayerEventData::PublicDecrypt(request_data),
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
                info!("Sending success reponse to public");
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    use fake::{Dummy, Fake};
    use validator::ValidationErrorsKind;

    struct HexString(pub usize);

    impl Dummy<HexString> for String {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &HexString, rng: &mut R) -> String {
            // HexString(config.0).generate(rng)
            (0..config.0)
                .map(|_| format!("{:x}", rng.random_range(0..16)))
                .collect()
        }
    }

    impl Dummy<()> for PublicDecryptRequestJson {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(
            _config: &(),
            rng: &mut R,
        ) -> PublicDecryptRequestJson {
            let size = rng.random_range(1..5);
            PublicDecryptRequestJson {
                ciphertext_handles: [1..size]
                    .iter()
                    .map(|_| HexString(rng.random_range(10..50) * 2).fake_with_rng(rng))
                    .collect(),
                extra_data: "0x00".to_string(),
            }
        }
    }

    impl Serialize for PublicDecryptRequestJson {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("PublicDecryptRequestJson", 2)?;
            state.serialize_field("ciphertextHandles", &self.ciphertext_handles)?;
            state.serialize_field("extraData", &self.extra_data)?;
            state.end()
        }
    }

    #[test]
    fn test_valid_json_succeeds() {
        let fake_data: PublicDecryptRequestJson = ().fake();
        let payload = serde_json::to_string(&fake_data).unwrap();
        let deserialized: PublicDecryptRequestJson = serde_json::from_str(&payload).unwrap();
        if let Err(e) = deserialized.validate() {
            panic!("Validation failed: {:?}", e);
        }
    }

    #[test]
    fn test_empty_ciphertext_handles_fails() {
        let fake_data = PublicDecryptRequestJson {
            ciphertext_handles: vec![],
            ..().fake()
        };
        let payload = serde_json::to_string(&fake_data).unwrap();
        let deserialized: PublicDecryptRequestJson = serde_json::from_str(&payload).unwrap();
        let errors = deserialized.validate().unwrap_err();
        println!("Errors: {:?}", errors);
        assert!(errors.errors().contains_key("ciphertext_handles"));
        match errors.errors()["ciphertext_handles"].clone() {
            ValidationErrorsKind::Field(errors) => {
                assert_eq!(errors[0].code, "length");
            }
            _ => panic!("Expected Field type for ciphertext_handles errors"),
        }
    }

    #[test]
    fn test_invalid_extra_data_fails() {
        let fake_data = PublicDecryptRequestJson {
            extra_data: "0x01".to_string(),
            ..().fake()
        };
        let payload = serde_json::to_string(&fake_data).unwrap();
        let deserialized: PublicDecryptRequestJson = serde_json::from_str(&payload).unwrap();
        let errors = deserialized.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("extra_data"));
    }

    #[test]
    fn test_public_decrypt_response_json_serialization() {
        // Create a sample payload with some Bytes values.
        let payload = PublicDecryptResponsePayloadJson {
            decrypted_value: Bytes::from(vec![1, 2, 3, 4]), // should serialize as "01020304"
            signatures: vec![
                Bytes::from(vec![5, 6, 7, 8]), // "05060708"
                Bytes::from(vec![9, 10]),      // "090a"
            ],
            extra_data: Bytes::from(vec![0x00]), // "00"
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
