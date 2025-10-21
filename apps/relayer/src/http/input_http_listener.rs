use crate::core::event::{
    ApiVersion, InputProofEventData, InputProofEventId, InputProofRequest, RelayerEvent,
    RelayerEventData,
};
use crate::http::docs_utils::ChainId;
use crate::http::utils::{de_string_or_number, OnceHandler};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{info, instrument, span, Level};
use utoipa::ToSchema;
use validator::Validate;

/// Represents the payload coming into the endpoint for input proof.
#[derive(Debug, Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofRequestJson {
    /// Contract's chain id
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    pub contract_chain_id: String,
    /// Contract's address
    #[validate(
        length(equal = 42),
        custom(function = "crate::http::utils::validate_blockchain_address")
    )]
    pub contract_address: String, // Hex encoded address with 0x prefix.
    /// User's wallet address
    #[validate(custom(function = "crate::http::utils::validate_blockchain_address"))]
    pub user_address: String, // Hex encoded address with 0x prefix.
    #[validate(
        length(min = 1),
        custom(function = "crate::http::utils::validate_hex_string_no_prefix")
    )]
    pub ciphertext_with_input_verification: String,
    /// Extra data field, always set to 0x00
    #[validate(custom(function = "crate::http::utils::validate_extra_data_field"))]
    #[schema(example = "0x00")]
    pub extra_data: String, // Hex encoded Bytes array with 0x prefix.
}

/// Represents the response from the endpoint for input proof.
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct InputProofResponseJson {
    pub response: InputProofResponsePayloadJson,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct InputProofResponsePayloadJson {
    pub handles: Vec<String>, // Ordered List of hex encoded handles with 0x prefix.
    pub signatures: Vec<String>, // Attestation signatures for Input verification for the ordered list of handles.
}

/// Represents the error response from the endpoint for input proof.
#[derive(Debug, Serialize, Clone, ToSchema)]
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

    ///
    // pub contractChainId: String, // Hex encoded uint256 string with 0x prefix.
    // pub contractAddress: String, // Hex encoded address with 0x prefix.
    // pub userAddress: String,     // Hex encoded address with 0x prefix.
    /// Handles requests to the endpoint for input proof.
    #[instrument(name="handle-input", skip_all, fields(contract=%payload.contract_address, contract_chain_id=%payload.contract_chain_id, userAddress=%payload.user_address))]
    pub async fn handle(&self, Json(payload): Json<InputProofRequestJson>) -> impl IntoResponse {
        info!("Handling input proof request");
        // Validate the payload
        if let Err(errors) = payload.validate() {
            let error_response = InputProofErrorResponseJson {
                message: errors.to_string(),
            };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }

        let request_id = self.orchestrator.new_request_id();
        let _span = span!(Level::INFO, "handle-input-req", request_id = %request_id); // Add other relevant top-level details

        info!("Validated and assigned request id: {}", request_id);

        // Register once handlers for receiving the decryption response from the gateway
        let (gateway_response_handler, gateway_response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let gateway_response_handler = Arc::new(gateway_response_handler);

        self.orchestrator.register_once_handler(
            InputProofEventId::RespRcvdFromGw.into(),
            request_id,
            gateway_response_handler,
        );
        info!("Registered once handler for handling input proof gateway response");

        // Register once handlers for receiving the decryption response from the gateway
        let (error_handler, error_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let error_handler = Arc::new(error_handler);

        self.orchestrator.register_once_handler(
            InputProofEventId::Failed.into(),
            request_id,
            error_handler,
        );
        info!("Registered once handler for handling input proof failure");

        let request_data: InputProofRequest = match payload.try_into() {
            Ok(event_data) => event_data,
            Err(message) => {
                let error_response = InputProofErrorResponseJson {
                    message: message.to_string(),
                };
                return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
            }
        };
        let request_data = InputProofEventData::ReqRcvdFromUser {
            input_proof_request: request_data,
        };

        let event = RelayerEvent::new(
            request_id,
            self.api_version,
            RelayerEventData::InputProof(request_data),
        );
        let _ = self.orchestrator.dispatch_event(event).await;
        info!("dispatched event to orchestrator to initiate processing");

        let _waiting_for_response_span =
            span!(Level::INFO, "waiting-for-response", request_id = %request_id);
        info!("waiting for response event");

        // Wait for response or error on the rx of Oneshot channels concurrently.
        use futures::pin_mut;
        pin_mut!(gateway_response_rx);
        pin_mut!(error_rx);

        tokio::select! {
            res = &mut gateway_response_rx => {
                match res {
                    Ok(event) => {
                        info!("Response event type {:?}", event.data);
                        event.into_response()
                    }
                    Err(_) => {
                        info!("received error while waiting for response event");
                        let error_response = InputProofErrorResponseJson {
                            message: "Failed to receive response from the gateway.".to_string(),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                    }
                }
            }
            res = &mut error_rx => {
                match res {
                    Ok(event) => {
                        info!("received error event on error_rx");
                        event.into_response()
                    }
                    Err(_) => {
                        info!("received error while waiting for error event on error_rx");
                        let error_response = InputProofErrorResponseJson {
                            message: "Failed to receive error response from the gateway.".to_string(),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    const VALID_JSON_STR_ID: &str = r#"
        {
                   "contractChainId": "123456",
                   "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
                   "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
                   "ciphertextWithInputVerification": "abcdef123456",
                   "extraData": "0x00"
        }
    "#;

    const VALID_JSON_NUM_ID: &str = r#"
        {
                   "contractChainId": 123456,
                   "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
                   "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
                   "ciphertextWithInputVerification": "abcdef123456",
                   "extraData": "0x00"
        }
    "#;

    const VALID_JSON_HEX_ID: &str = r#"
        {
                   "contractChainId": "0x1e240",
                   "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
                   "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
                   "ciphertextWithInputVerification": "abcdef123456",
                   "extraData": "0x00"
        }
    "#;

    #[test]
    fn test_valid_json_with_string_id_succeeds() {
        let data: InputProofRequestJson = serde_json::from_str(VALID_JSON_STR_ID).unwrap();
        assert!(data.validate().is_ok());
        assert!(data.contract_chain_id == "123456");
    }

    #[test]
    fn test_valid_json_with_numeric_id_succeeds() {
        let data: InputProofRequestJson = serde_json::from_str(VALID_JSON_NUM_ID).unwrap();
        assert!(data.validate().is_ok());
        assert!(data.contract_chain_id == "123456");
    }

    #[test]
    fn test_valid_json_with_hex_id_succeeds() {
        let data: InputProofRequestJson = serde_json::from_str(VALID_JSON_HEX_ID).unwrap();
        assert!(data.validate().is_ok());
        assert!(data.contract_chain_id == "0x1e240");
    }

    #[test]
    fn test_invalid_contract_address_fails() {
        for invalid_address in &[
            "0xGHIJKL99D17FAAB8c95B2eCD500cFeFc8f658f15d", // Invalid hex character 'G'
            "1234567890abcdef1234567890abcdef12345678",    // Missing 0x prefix
            "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15",   // One character short
            "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15da", // One character longer
            "",                                            // empty string
        ] {
            let invalid_json = VALID_JSON_STR_ID.replace(
                "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
                invalid_address,
            );
            let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            // Check that the error is for the correct field
            assert!(errors.field_errors().contains_key("contract_address"));
        }
    }

    #[test]
    fn test_invalid_user_address_fails() {
        for invalid_address in &[
            "0xGHIJKL845C1cc05e9493856a1D637a73e944bE", // Invalid hex character 'G'
            "12B064FB845C1cc05e9493856a1D637a73e944bE", // Missing 0x prefix
            "0x12B064FB845C1cc05e9493856a1D637a73e944", // One character short
            "0x12B064FB845C1cc05e9493856a1D637a73e944bEE", // One character longer
            "",                                         // empty string
        ] {
            let invalid_json = VALID_JSON_STR_ID.replace(
                "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
                invalid_address,
            );
            let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            // Check that the error is for the correct field
            assert!(errors.field_errors().contains_key("user_address"));
        }
    }

    #[test]
    fn test_invalid_ciphertext_fails_empty() {
        // Ciphertext has a "0x" prefix, which is not allowed
        let invalid_json = VALID_JSON_STR_ID.replace("abcdef123456", "");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors
            .field_errors()
            .contains_key("ciphertext_with_input_verification"));
    }

    #[test]
    fn test_invalid_ciphertext_fails_with_prefix() {
        // Ciphertext has a "0x" prefix, which is not allowed
        let invalid_json = VALID_JSON_STR_ID.replace("abcdef123456", "0xabcdef123456");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors
            .field_errors()
            .contains_key("ciphertext_with_input_verification"));
    }

    #[test]
    fn test_invalid_ciphertext_fails_non_hex() {
        // Ciphertext contains an invalid character 'G'
        let invalid_json = VALID_JSON_STR_ID.replace("abcdef123456", "abcdef123456G");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors
            .field_errors()
            .contains_key("ciphertext_with_input_verification"));
    }

    #[test]
    fn test_invalid_extra_data_fails() {
        let invalid_json = VALID_JSON_STR_ID.replace("0x00", "0x01");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("extra_data"));
    }

    #[test]
    fn test_wrong_json_type_fails_at_deserialization() {
        // "userAddress" is a boolean, which doesn't match the struct's String type.
        // This error comes from `serde`, not `validator`.
        let invalid_json =
            VALID_JSON_STR_ID.replace("\"0x12B064FB845C1cc05e9493856a1D637a73e944bE\"", "true");
        let result: Result<InputProofRequestJson, _> = serde_json::from_str(&invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_input_proof_request_json() {
        // Deserialize the JSON string into the struct.
        let request: InputProofRequestJson =
            serde_json::from_str(VALID_JSON_STR_ID).expect("JSON deserialization failed");

        // Assert that each field was deserialized correctly.
        assert_eq!(request.contract_chain_id, "123456");
        assert_eq!(
            request.contract_address,
            "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d"
        );
        assert_eq!(
            request.user_address,
            "0x12B064FB845C1cc05e9493856a1D637a73e944bE"
        );
        assert_eq!(request.ciphertext_with_input_verification, "abcdef123456");
        assert_eq!(request.extra_data, "0x00");
    }
}
