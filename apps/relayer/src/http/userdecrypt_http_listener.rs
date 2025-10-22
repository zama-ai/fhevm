use crate::core::event::{
    ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptEventId,
    UserDecryptRequest,
};
use crate::http::docs_utils::ChainId;
use crate::http::utils::{de_string_or_number, serialize_vec_as_hex, AppResponse, OnceHandler};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::primitives::Bytes;
use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::info;
use tracing::{error, instrument, span, Level};
use utoipa::ToSchema;
use validator::Validate;

/// Represents the payload coming into the endpoint for user decrypt.
#[derive(Debug, Deserialize, Clone, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptRequestJson {
    #[validate(length(
        min = 1,
        message = "At least one handle-contract pair must be provided"
    ))]
    #[validate(nested)]
    pub handle_contract_pairs: Vec<HandleContractPairJson>,
    #[validate(nested)]
    pub request_validity: RequestValidityJson,
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    pub contracts_chain_id: String,

    /// Array of contract addresses
    #[validate(length(min = 1, message = "At least one contract address must be provided"))]
    #[validate(custom(function = "crate::http::utils::validate_blockchain_addresses"))]
    pub contract_addresses: Vec<String>,
    /// User's wallet address
    #[validate(custom(function = "crate::http::utils::validate_blockchain_address"))]
    pub user_address: String,
    #[validate(
        length(equal = 132, message = "Signature must be 132 characters long"),
        custom(function = "crate::http::utils::validate_hex_string")
    )]
    pub signature: String,
    /// Public key
    #[validate(custom(function = "crate::http::utils::validate_hex_string"))]
    pub public_key: String,
    /// Extra data field, always set to 0x00
    #[validate(custom(function = "crate::http::utils::validate_extra_data_field"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Hash, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HandleContractPairJson {
    #[validate(
        length(equal = 66, message = "Handle must be 66 characters long"),
        custom(function = "crate::http::utils::validate_hex_string_prefix")
    )]
    pub handle: String,
    #[validate(custom(function = "crate::http::utils::validate_blockchain_address"))]
    pub contract_address: String,
}

impl Display for HandleContractPairJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ct-handle: {}, contract-address: {}",
            self.handle, self.contract_address
        )
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestValidityJson {
    #[validate(
        length(min = 1),
        custom(function = "crate::http::utils::validate_u256_string")
    )]
    pub start_timestamp: String,
    #[validate(custom(function = "crate::http::utils::validate_u32_string"))]
    pub duration_days: String,
}

/// Represents the response from the endpoint for user decrypt.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserDecryptResponseJson {
    pub response: Vec<UserDecryptResponsePayloadJson>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UserDecryptResponsePayloadJson {
    #[schema(value_type = String)]
    pub payload: Bytes,
    #[schema(value_type = String)]
    pub signature: Bytes,
    #[schema(value_type = String)]
    pub extra_data: Bytes,
}

pub type UserDecryptResponse = AppResponse<UserDecryptRequestJson>;

/// Represents the error response from the endpoint for user decrypt.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
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
    #[instrument(name="handle-user-decrypt", skip_all, fields(user_address=%payload.user_address, cts=?payload.handle_contract_pairs))]
    pub async fn handle(&self, Json(payload): Json<UserDecryptRequestJson>) -> impl IntoResponse {
        info!("Handling user decryption request in http listener");
        // Validate the payload
        if let Err(errors) = payload.validate() {
            return UserDecryptResponse::bad_request(errors).into_response();
        }

        let user_decrypt_request = match UserDecryptRequest::try_from(payload.clone()) {
            Ok(request) => request,
            Err(error) => {
                // TODO: this should not happen anymore as we validate the payload.
                error!("Conversion failed: {}", error);
                // Try to identify exactly where it's failing
                if let Err(e) = serde_json::to_string(&payload) {
                    error!("Cannot serialize payload: {}", e);
                }
                // Try parsing individual fields
                if let Err(e) = payload.request_validity.duration_days.parse::<u32>() {
                    error!("Failed to parse durationDays: {}", e);
                }

                return UserDecryptResponse::unprocessable(format!(
                    "failed to parse request: {error}"
                ))
                .into_response();
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
                        event.into_response()
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
                        event.into_response()
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

#[cfg(test)]
mod tests {

    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use fake::{Dummy, Fake};
    use validator::ValidationErrorsKind;

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

    impl Dummy<()> for HandleContractPairJson {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &(), rng: &mut R) -> Self {
            HandleContractPairJson {
                handle: PrefixedHexString(66).fake_with_rng(rng),
                contract_address: BlockchainAddress.fake_with_rng(rng),
            }
        }
    }

    impl Dummy<()> for RequestValidityJson {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &(), rng: &mut R) -> Self {
            RequestValidityJson {
                // TODO: check which timestamp makes sense here and adjust accordingly
                // TODO: add some randomness
                start_timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
                duration_days: rng.random_range(1..30).to_string(),
            }
        }
    }

    impl Dummy<()> for UserDecryptRequestJson {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &(), rng: &mut R) -> Self {
            UserDecryptRequestJson {
                handle_contract_pairs: vec![().fake_with_rng(rng)],
                request_validity: ().fake_with_rng(rng),
                contracts_chain_id: "123456".to_string(),
                contract_addresses: vec![BlockchainAddress.fake_with_rng(rng)],
                user_address: BlockchainAddress.fake_with_rng(rng),
                signature: PrefixedHexString(132).fake_with_rng(rng),
                // Note: hex string should be even length
                public_key: PrefixedHexString(rng.random_range(10..50) * 2).fake_with_rng(rng),
                extra_data: "0x00".to_string(),
            }
        }
    }

    #[test]
    fn test_valid_json_with_string_id_succeeds() {
        let fake_data: UserDecryptRequestJson = ().fake();
        let serialized = serde_json::to_string(&fake_data).unwrap();
        let data: UserDecryptRequestJson = serde_json::from_str(&serialized).unwrap();
        assert!(serialized.contains(r#"contractsChainId":"123456""#));
        match data.validate() {
            Err(e) => panic!("Validation failed: {:?}", e),
            _ => {}
        }
        assert_eq!(data.contracts_chain_id, "123456");
    }

    #[test]
    fn test_valid_json_with_numeric_id_succeeds() {
        // Note: we have to manually replace the field in the serialized JSON
        // because the field is serialized as a string
        let fake_data: UserDecryptRequestJson = ().fake();
        let serialized = serde_json::to_string(&fake_data).unwrap();
        let serialized = serialized.replace(
            r#""contractsChainId":"123456""#,
            r#""contractsChainId":123456"#,
        );
        assert!(serialized.contains(r#"contractsChainId":123456"#));
        let data: UserDecryptRequestJson = serde_json::from_str(&serialized).unwrap();
        match data.validate() {
            Err(e) => panic!("Validation failed: {:?}", e),
            _ => {}
        }
        assert!(data.contracts_chain_id == "123456");
    }

    #[test]
    fn test_valid_json_with_hex_id_succeeds() {
        let fake_data = UserDecryptRequestJson {
            contracts_chain_id: "0x1e240".to_string(),
            ..().fake()
        };
        let serialized = serde_json::to_string(&fake_data).unwrap();
        assert!(serialized.contains(r#"contractsChainId":"0x1e240""#));
        let data: UserDecryptRequestJson = serde_json::from_str(&serialized).unwrap();
        match data.validate() {
            Err(e) => panic!("Validation failed: {:?}", e),
            _ => {}
        }
        assert!(data.contracts_chain_id == "0x1e240");
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
            let fake_data = UserDecryptRequestJson {
                contract_addresses: [invalid_address.to_string()].to_vec(),
                ..().fake()
            };
            let invalid_json = serde_json::to_string(&fake_data).unwrap();

            let data: UserDecryptRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();

            assert!(errors.errors().contains_key("contract_addresses"));
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
            let fake_data = UserDecryptRequestJson {
                user_address: invalid_address.to_string(),
                ..().fake()
            };
            let invalid_json = serde_json::to_string(&fake_data).unwrap();
            let data: UserDecryptRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            // Check that the error is for the correct field
            assert!(errors.field_errors().contains_key("user_address"));
        }
    }

    #[test]
    fn test_invalid_handle_fails() {
        for invalid_handle in &[
            {
                let mut invalid_handle: String = PrefixedHexString(66).fake();
                invalid_handle.push('g');
                invalid_handle
            }, // Invalid hex character 'g'
            PrefixedHexString(65).fake(), // One character short
            PrefixedHexString(67).fake(), // One character longer
            HexString(66).fake(),         // Missing 0x prefix
            "".to_string(),               // empty string
        ] {
            let fake_data = UserDecryptRequestJson {
                handle_contract_pairs: vec![HandleContractPairJson {
                    handle: invalid_handle.to_string(),
                    contract_address: BlockchainAddress.fake(),
                }],
                ..().fake()
            };
            let invalid_json = serde_json::to_string(&fake_data).unwrap();
            println!("Invalid JSON: {}", invalid_json);
            let data: UserDecryptRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            assert!(errors.errors().contains_key("handle_contract_pairs"));
            match errors.errors()["handle_contract_pairs"].clone() {
                ValidationErrorsKind::List(nested_errors) => {
                    println!("Nested errors: {:?}", nested_errors);
                    assert_eq!(nested_errors.len(), 1);
                    let first: usize = 0;
                    assert!(nested_errors[&first].field_errors().contains_key("handle"));
                }
                _ => panic!("Expected List type for handle_contract_pairs errors"),
            }
        }
    }

    #[test]
    fn test_invalid_paired_contract_fails() {
        for invalid_handle in &[
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
            let fake_data = UserDecryptRequestJson {
                handle_contract_pairs: vec![HandleContractPairJson {
                    handle: PrefixedHexString(66).fake(),
                    contract_address: invalid_handle.to_string(),
                }],
                ..().fake()
            };
            let invalid_json = serde_json::to_string(&fake_data).unwrap();
            let data: UserDecryptRequestJson = serde_json::from_str(&invalid_json).unwrap();
            let errors = data.validate().unwrap_err();
            assert!(errors.errors().contains_key("handle_contract_pairs"));
            match errors.errors()["handle_contract_pairs"].clone() {
                ValidationErrorsKind::List(nested_errors) => {
                    println!("Nested errors: {:?}", nested_errors);
                    assert_eq!(nested_errors.len(), 1);
                    let first: usize = 0;
                    assert!(nested_errors[&first]
                        .field_errors()
                        .contains_key("contract_address"));
                }
                _ => panic!("Expected List type for handle_contract_pairs errors"),
            }
        }
    }

    #[test]
    fn test_invalid_request_validity() {
        let fake_data = UserDecryptRequestJson {
            request_validity: {
                RequestValidityJson {
                    start_timestamp: "invalid_timestamp".to_string(),
                    duration_days: "not_a_number".to_string(),
                }
            },
            ..().fake()
        };
        let invalid_json = serde_json::to_string(&fake_data).unwrap();
        let data: UserDecryptRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors.errors().contains_key("request_validity"));
        match errors.errors()["request_validity"].clone() {
            ValidationErrorsKind::Struct(nested_errors) => {
                assert!(nested_errors.field_errors().contains_key("start_timestamp"));
                assert!(nested_errors.field_errors().contains_key("duration_days"));
            }
            _ => panic!("Expected Struct type for request_validity errors"),
        }
    }

    #[test]
    fn test_invalid_extra_data_fails() {
        let fake_data = UserDecryptRequestJson {
            extra_data: "0x01".to_string(),
            ..().fake()
        };
        let invalid_json = serde_json::to_string(&fake_data).unwrap();
        let data: UserDecryptRequestJson = serde_json::from_str(&invalid_json).unwrap();
        let errors = data.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("extra_data"));
    }
}
