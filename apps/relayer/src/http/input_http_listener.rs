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
    #[validate(custom(function = "crate::http::utils::validate_blockchain_address"))]
    pub contract_address: String, // Hex encoded address with 0x prefix.
    /// User's wallet address
    #[validate(custom(function = "crate::http::utils::validate_blockchain_address"))]
    pub user_address: String, // Hex encoded address with 0x prefix.
    #[validate(
        length(min = 1),
        custom(function = "crate::http::utils::validate_hex_string")
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
            return InputProofResponse::invalid_request(errors).into_response();
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
                return InputProofResponse::bad_request(message.to_string()).into_response();
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
    use super::*;
    use fake::{Dummy, Fake};
    use serde::ser::{Serialize, SerializeStruct, Serializer};
    use serde_json;

    struct HexString(pub usize);
    struct PrefixedHexString(pub usize);
    struct BlockchainAddress;

    impl Dummy<HexString> for String {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &HexString, rng: &mut R) -> String {
            // HexString(config.0).generate(rng)
            (0..config.0)
                .map(|_| format!("{:x}", rng.random_range(0..16)))
                .collect()
        }
    }

    impl Dummy<PrefixedHexString> for String {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(
            config: &PrefixedHexString,
            rng: &mut R,
        ) -> String {
            let len = config.0 - 2;
            // HexString(config.0).generate(rng)
            let s: String = (0..len)
                .map(|_| format!("{:x}", rng.random_range(0..16)))
                .collect();
            format!("0x{}", s)
        }
    }

    impl Dummy<BlockchainAddress> for String {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(
            _config: &BlockchainAddress,
            rng: &mut R,
        ) -> String {
            PrefixedHexString(42).fake_with_rng(rng)
        }
    }

    impl Dummy<()> for InputProofRequestJson {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(
            _config: &(),
            rng: &mut R,
        ) -> InputProofRequestJson {
            InputProofRequestJson {
                contract_chain_id: "123456".to_string(),
                contract_address: BlockchainAddress.fake_with_rng(rng),
                user_address: BlockchainAddress.fake_with_rng(rng),
                // TODO: check ciphertext length constraints
                // Note: hex string should be even length
                ciphertext_with_input_verification: HexString(rng.random_range(20..50) * 2)
                    .fake_with_rng(rng),
                extra_data: "0x00".to_string(),
            }
        }
    }

    impl Serialize for InputProofRequestJson {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Number of fields: 5
            let mut state = serializer.serialize_struct("InputProofRequestJson", 5)?;

            state.serialize_field("contractChainId", &self.contract_chain_id)?;
            state.serialize_field("contractAddress", &self.contract_address)?;
            state.serialize_field("userAddress", &self.user_address)?;
            state.serialize_field(
                "ciphertextWithInputVerification",
                &self.ciphertext_with_input_verification,
            )?;
            state.serialize_field("extraData", &self.extra_data)?;

            state.end()
        }
    }

    #[test]
    fn test_valid_json_with_string_id_succeeds() {
        let fake_data: InputProofRequestJson = ().fake();
        let serialized = serde_json::to_string(&fake_data).unwrap();
        assert!(serialized.contains(r#"contractChainId":"123456""#));
        let data: InputProofRequestJson = serde_json::from_str(&serialized).unwrap();
        if let Err(e) = data.validate() {
            panic!("Validation failed: {:?}", e);
        }
        assert!(data.contract_chain_id == "123456");
    }

    #[test]
    fn test_valid_json_with_numeric_id_succeeds() {
        // Note: we have to manually replace the field in the serialized JSON
        // because the field is serialized as a string
        let fake_data: InputProofRequestJson = ().fake();
        let json = serde_json::to_string(&fake_data).unwrap();
        let serialized = json.replace(
            r#""contractChainId":"123456""#,
            r#""contractChainId":123456"#,
        );
        assert!(serialized.contains(r#"contractChainId":123456"#));
        let data: InputProofRequestJson = serde_json::from_str(&serialized).unwrap();
        if let Err(e) = data.validate() {
            panic!("Validation failed: {:?}", e);
        }
        assert!(data.contract_chain_id == "123456");
    }

    #[test]
    fn test_valid_json_with_hex_id_succeeds() {
        let fake_data = InputProofRequestJson {
            contract_chain_id: "0x1e240".to_string(),
            ..().fake()
        };
        let json = serde_json::to_string(&fake_data).unwrap();
        assert!(json.contains(r#"contractChainId":"0x1e240""#));
        let data: InputProofRequestJson = serde_json::from_str(&json).unwrap();
        if let Err(e) = data.validate() {
            panic!("Validation failed: {:?}", e);
        }
        assert!(data.contract_chain_id == "0x1e240");
    }

    #[test]
    fn test_invalid_contract_address_fails() {
        for invalid_address in &[
            {
                let mut invalid_handle: String = PrefixedHexString(39).fake();
                invalid_handle.push('g');
                invalid_handle
            }, // Invalid hex character 'g'
            PrefixedHexString(39).fake(), // One character short
            PrefixedHexString(41).fake(), // One character longer
            HexString(40).fake(),         // Missing 0x prefix
            "".to_string(),               // empty string
        ] {
            let fake_data = InputProofRequestJson {
                contract_address: invalid_address.clone(),
                ..().fake()
            };
            let invalid_json = serde_json::to_string(&fake_data).unwrap();
            let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            // Check that the error is for the correct field
            assert!(errors.field_errors().contains_key("contract_address"));
        }
    }

    #[test]
    fn test_invalid_user_address_fails() {
        for invalid_address in &[
            {
                let mut invalid_handle: String = PrefixedHexString(39).fake();
                invalid_handle.push('g');
                invalid_handle
            }, // Invalid hex character 'g'
            PrefixedHexString(39).fake(), // One character short
            PrefixedHexString(41).fake(), // One character longer
            HexString(40).fake(),         // Missing 0x prefix
            "".to_string(),               // empty string
        ] {
            let fake_data = InputProofRequestJson {
                user_address: invalid_address.clone(),
                ..().fake()
            };
            let invalid_json = serde_json::to_string(&fake_data).unwrap();
            let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            // Check that the error is for the correct field
            assert!(errors.field_errors().contains_key("user_address"));
        }
    }

    #[test]
    fn test_invalid_ciphertext_fails() {
        for invalid_ciphetext in &[
            {
                let mut invalid_handle: String =
                    HexString(rand::random_range(10..50) * 2 + 1).fake();
                invalid_handle.push('g');
                invalid_handle
            }, // Invalid hex character 'g'
            PrefixedHexString(rand::random_range(10..50) * 2).fake(), // prefixed hex string
            "".to_string(),                                           // empty string
        ] {
            let fake_data = InputProofRequestJson {
                ciphertext_with_input_verification: invalid_ciphetext.clone(),
                ..().fake()
            };
            let invalid_json = serde_json::to_string(&fake_data).unwrap();
            let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            assert!(errors
                .field_errors()
                .contains_key("ciphertext_with_input_verification"));
        }
    }

    #[test]
    fn test_invalid_extra_data_fails() {
        let fake_data = InputProofRequestJson {
            extra_data: "0x01".to_string(),
            ..().fake()
        };
        let invalid_json = serde_json::to_string(&fake_data).unwrap();
        let data: InputProofRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("extra_data"));
    }
}
