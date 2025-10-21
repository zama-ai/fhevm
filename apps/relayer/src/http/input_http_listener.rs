use crate::core::event::{
    ApiVersion, InputProofEventData, InputProofEventId, InputProofRequest, RelayerEvent,
    RelayerEventData,
};
use crate::http::docs_utils::ChainId;
use crate::http::utils::{de_string_or_number, AppResponse, OnceHandler};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use axum::{extract::Json, response::IntoResponse};
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

pub type InputProofResponse = AppResponse<InputProofResponseJson>;
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
            return InputProofResponse::bad_request(errors).into_response();
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
                // TODO: check if this is an unprocessable content or an internal server error
                return InputProofResponse::unprocessable(message.to_string()).into_response();
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
                        return InputProofResponse::internal_server_error("Failed to receive response from the gateway.").into_response();
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
                        return InputProofResponse::internal_server_error("Failed to receive error response from the gateway.").into_response();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::event::{ApiCategory, InputProofResponse},
        orchestrator::{
            traits::{Event, EventHandler},
            TokioEventDispatcher,
        },
    };

    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{self, Request, Response, StatusCode},
        Router,
    };
    use serde_json;
    use tower::ServiceExt;

    const VALID_JSON: &str = r#"
        {
                   "contractChainId": "123456",
                   "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
                   "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
                   "ciphertextWithInputVerification": "abcdef123456",
                   "extraData": "0x00"
        }
    "#;

    #[test]
    fn test_valid_json_with_string_id_succeeds() {
        let data: InputProofRequestJson = serde_json::from_str(VALID_JSON).unwrap();
        assert!(VALID_JSON.contains(r#"contractChainId": "123456""#));
        assert!(data.validate().is_ok());
        assert!(data.contract_chain_id == "123456");
    }

    #[test]
    fn test_valid_json_with_numeric_id_succeeds() {
        let json = VALID_JSON.replace("\"123456\"", "123456");
        assert!(json.contains(r#"contractChainId": 123456"#));
        let data: InputProofRequestJson = serde_json::from_str(&json).unwrap();
        assert!(data.validate().is_ok());
        assert!(data.contract_chain_id == "123456");
    }

    #[test]
    fn test_valid_json_with_hex_id_succeeds() {
        let json = VALID_JSON.replace("\"123456\"", "\"0x1e240\"");
        assert!(json.contains(r#"contractChainId": "0x1e240""#));
        let data: InputProofRequestJson = serde_json::from_str(&json).unwrap();
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
            let invalid_json = VALID_JSON.replace(
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
            let invalid_json = VALID_JSON.replace(
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
        let invalid_json = VALID_JSON.replace("abcdef123456", "");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors
            .field_errors()
            .contains_key("ciphertext_with_input_verification"));
    }

    #[test]
    fn test_invalid_ciphertext_fails_with_prefix() {
        // Ciphertext has a "0x" prefix, which is not allowed
        let invalid_json = VALID_JSON.replace("abcdef123456", "0xabcdef123456");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors
            .field_errors()
            .contains_key("ciphertext_with_input_verification"));
    }

    #[test]
    fn test_invalid_ciphertext_fails_non_hex() {
        // Ciphertext contains an invalid character 'G'
        let invalid_json = VALID_JSON.replace("abcdef123456", "abcdef123456G");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors
            .field_errors()
            .contains_key("ciphertext_with_input_verification"));
    }

    #[test]
    fn test_invalid_extra_data_fails() {
        let invalid_json = VALID_JSON.replace("0x00", "0x01");
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("extra_data"));
    }

    #[test]
    fn test_wrong_json_type_fails_at_deserialization() {
        // "userAddress" is a boolean, which doesn't match the struct's String type.
        // This error comes from `serde`, not `validator`.
        let invalid_json =
            VALID_JSON.replace("\"0x12B064FB845C1cc05e9493856a1D637a73e944bE\"", "true");
        let result: Result<InputProofRequestJson, _> = serde_json::from_str(&invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_input_proof_request_json() {
        // Deserialize the JSON string into the struct.
        let request: InputProofRequestJson =
            serde_json::from_str(VALID_JSON).expect("JSON deserialization failed");

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

    struct SimpleHandler {
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        response: InputProofResponse,
    }

    impl SimpleHandler {
        fn new(
            dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
            response: InputProofResponse,
        ) -> Self {
            Self {
                dispatcher,
                response,
            }
        }
    }

    #[async_trait::async_trait]
    impl EventHandler<RelayerEvent> for SimpleHandler {
        async fn handle_event(&self, event: RelayerEvent) {
            println!("Handling event: {:?}", event.event_name());
            self.dispatcher
                .dispatch_event(RelayerEvent {
                    request_id: event.request_id,
                    api_version: event.api_version,
                    data: RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                        input_proof_response: self.response.clone(),
                    }),
                    timestamp: event.timestamp,
                })
                .await
                .unwrap();
        }
    }

    fn app(response: InputProofResponse) -> Router {
        let dispatcher = Arc::new(TokioEventDispatcher::<RelayerEvent>::new());
        let handler = Arc::new(SimpleHandler::new(dispatcher.clone(), response));
        dispatcher.register_handler(InputProofEventId::ReqRcvdFromUser.into(), handler);
        let orchestrator = Orchestrator::new(dispatcher.clone());

        let handler = Arc::new(InputProofHandler::new(
            orchestrator,
            ApiVersion::new(ApiCategory::PRODUCTION, 1),
        ));

        Router::new().route(
            "/input-proof",
            axum::routing::post(move |payload| {
                let handler = handler.clone();
                async move { handler.handle(payload).await }
            }),
        )
    }

    async fn post(app: Router, payload: String) -> Response<Body> {
        app.oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/input-proof")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn e2e_valid_payload_returns_ok() {
        let app = app(InputProofResponse {
            handles: vec![],
            signatures: vec![],
        });

        let response = post(app, String::from(VALID_JSON)).await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn e2e_empty_payload_returns_bad_request() {
        let app = app(InputProofResponse {
            handles: vec![],
            signatures: vec![],
        });

        let response = post(app, String::from("")).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn e2e_invalid_addresses_returns_bad_request() {
        let app = app(InputProofResponse {
            handles: vec![],
            signatures: vec![],
        });
        //contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
        //    "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
        let invalid_payload = VALID_JSON
            .replace(
                "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
                // Invalid character 'G' at the end
                "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15G",
            )
            .replace(
                "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
                // Address too short
                "0x12B064FB845C1cc05e9493856a1D637a73e944b",
            );

        let response = post(app, invalid_payload).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // Check that the response body contains the specific validation error.
        let errors = &body["errors"];
        assert!(errors.is_object());
        assert!(errors["contractAddress"].is_array());
        assert!(errors["contractAddress"][0].is_object());
        assert_eq!(
            errors["contractAddress"][0]["code"],
            "invalid_hex_characters"
        );

        assert!(errors["userAddress"].is_array());
        assert!(errors["userAddress"][0].is_object());
        assert_eq!(errors["userAddress"][0]["code"], "invalid_length");
    }

    #[tokio::test]
    async fn e2e_invalid_ciphertext_returns_bad_request() {
        let app = app(InputProofResponse {
            handles: vec![],
            signatures: vec![],
        });
        let invalid_payload = VALID_JSON.replace("abcdef123456", "abcdef123456G");

        let response = post(app, invalid_payload).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // Check that the response body contains the specific validation error.
        let errors = &body["errors"];
        assert!(errors.is_object());
        let field_errors = &errors["ciphertextWithInputVerification"];
        assert!(field_errors.is_array());
        assert_eq!(field_errors[0]["code"], "invalid_hex_characters");
    }

    #[tokio::test]
    async fn e2e_invalid_extra_data_returns_bad_request() {
        let app = app(InputProofResponse {
            handles: vec![],
            signatures: vec![],
        });
        let invalid_payload = VALID_JSON.replace("\"0x00\"", "\"0x01\"");

        let response = post(app, invalid_payload).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // Check that the response body contains the specific validation error.
        let errors = &body["errors"];
        assert!(errors.is_object());
        let field_errors = &errors["extraData"];
        assert!(field_errors.is_array());
        assert_eq!(field_errors[0]["code"], "invalid_value");
    }
}
