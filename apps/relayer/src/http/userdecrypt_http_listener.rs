use crate::core::event::{
    ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptEventId,
    UserDecryptRequest,
};
use crate::core::job_id::JobId;
use crate::http::docs_utils::ChainId;
use crate::http::utils::{
    de_string_or_number, parse_and_validate, serialize_vec_as_hex, AppResponse, OnceHandler,
};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::{ContentHasher, Orchestrator};
use crate::store::sql::repositories::user_decrypt_repo::UserDecryptRepository;
use alloy::primitives::Bytes;
use axum::{body::Bytes as AxumBytes, extract::FromRequest, http::Request, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, Level};
use utoipa::ToSchema;
use validator::Validate;

/// Represents the payload coming into the endpoint for user decrypt.
#[derive(Debug, Deserialize, Clone, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptRequestJson {
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::utils::validate_handle_contract_pairs")
    )]
    pub handle_contract_pairs: Vec<HandleContractPairJson>,
    #[validate(custom(function = "crate::http::utils::validate_request_validity"))]
    pub request_validity: RequestValidityJson,
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::utils::validate_chain_id_string"))]
    pub contracts_chain_id: String,

    /// Array of contract addresses
    #[validate(length(min = 1, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::utils::validate_blockchain_addresses"))]
    pub contract_addresses: Vec<String>,
    /// User's wallet address
    #[validate(custom(function = "crate::http::utils::validate_blockchain_address"))]
    pub user_address: String,
    // TODO: change validator function here for checking the rights signatures.
    #[validate(
        length(equal = 130, message = "Must be 130 characters long"),
        custom(function = "crate::http::utils::validate_no_0x_hex")
    )]
    pub signature: String,
    /// Public key
    #[validate(length(min = 2, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::utils::validate_no_0x_hex"))]
    pub public_key: String,
    /// Extra data field, always set to 0x00
    #[validate(custom(function = "crate::http::utils::validate_extra_data_field"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Hash, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HandleContractPairJson {
    pub handle: String,
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
    pub start_timestamp: String,
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

pub type UserDecryptResponse = AppResponse<UserDecryptResponseJson>;

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
    user_decrypt_repo: Arc<UserDecryptRepository>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> UserDecryptHandler<D> {
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        user_decrypt_repo: Arc<UserDecryptRepository>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            user_decrypt_repo,
        }
    }

    /// Handles requests to the endpoint for user decrypt.
    #[instrument(name = "handle-user-decrypt", skip_all, fields(request_id))]
    pub async fn handle<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        // Generate request ID first so it's available for all error responses
        let request_id = self.orchestrator.new_internal_request_id();
        let _span = span!(Level::INFO, "handle-user-decrypt-req", request_id = %request_id);

        info!(
            "Handling user decryption request, generated request id: {}",
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

        let user_decrypt_request: UserDecryptRequest =
            match parse_and_validate::<UserDecryptRequestJson, UserDecryptRequest>(&body) {
                Ok(request) => request,
                Err(parse_error) => {
                    let error_response: AppResponse<()> =
                        parse_error.to_app_response(&request_id.to_string());
                    return error_response.into_response();
                }
            };

        info!("Successfully parsed and validated request");

        // Compute content-based JobId for deduplication
        let int_indexer_id = user_decrypt_request.content_hash();
        let job_id = JobId::from_sha256_hash(int_indexer_id);

        // Register once handlers for receiving the decryption response from the gateway.
        let (response_handler, response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let response_handler = Arc::new(response_handler);

        self.orchestrator.register_once_handler(
            UserDecryptEventId::RespRcvdFromGw.into(),
            job_id,
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
            job_id,
            error_handler,
        );
        info!("Registered once handler for user decrypt failure");

        let ext_reference_id = self.orchestrator.new_ext_reference_id();
        let request_json = match serde_json::to_value(user_decrypt_request.clone()) {
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
            .user_decrypt_repo
            .insert_data_on_conflict_and_get_ext_reference_id(
                ext_reference_id,
                &int_indexer_id[..],
                request_json,
            )
            .await
        {
            error!(
                "Failed to insert/get user decrypt into/from database: {}",
                e
            );
            return AppResponse::<()>::internal_server_error_with_request_id(
                request_id.to_string(),
            )
            .into_response();
        }

        let request_data = UserDecryptEventData::ReqRcvdFromUser {
            decrypt_request: user_decrypt_request,
        };
        let event = RelayerEvent::new(
            job_id,
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
