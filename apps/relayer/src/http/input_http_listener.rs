use crate::core::event::{
    ApiVersion, InputProofEventData, InputProofEventId, InputProofRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::http::ChainId;
use crate::http::{de_string_or_number, parse_and_validate, AppResponse};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::OnceHandler;
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::input_proof_repo::InputProofRepository;
use axum::{body::Bytes, extract::FromRequest, http::Request, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, Level};
use utoipa::ToSchema;
use validator::Validate;

/// Represents the payload coming into the endpoint for input proof.
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofRequestJson {
    /// Contract's chain id
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::validate_chain_id_string"))]
    pub contract_chain_id: String,
    /// Contract's address
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub contract_address: String, // Hex encoded address with 0x prefix.
    /// User's wallet address
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub user_address: String, // Hex encoded address with 0x prefix.
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_no_0x_hex")
    )]
    pub ciphertext_with_input_verification: String,
    /// Extra data field, always set to 0x00
    #[validate(custom(function = "crate::http::validate_extra_data_field"))]
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
    input_proof_repo: Arc<InputProofRepository>,
}

impl<D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>> InputProofHandler<D> {
    pub fn new(
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        api_version: ApiVersion,
        input_proof_repo: Arc<InputProofRepository>,
    ) -> Self {
        Self {
            orchestrator,
            api_version,
            input_proof_repo,
        }
    }

    ///
    // pub contractChainId: String, // Hex encoded uint256 string with 0x prefix.
    // pub contractAddress: String, // Hex encoded address with 0x prefix.
    // pub userAddress: String,     // Hex encoded address with 0x prefix.
    /// Handles requests to the endpoint for input proof.
    #[instrument(name = "handle-input", skip_all, fields(request_id))]
    pub async fn handle<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
        // Generate request ID first so it's available for all error responses
        let request_id = self.orchestrator.new_internal_request_id();
        let _span = span!(Level::INFO, "handle-input-req", request_id = %request_id);

        info!(
            "Handling input proof request, generated request id: {}",
            request_id
        );

        let body = match Bytes::from_request(req, _state).await {
            Ok(body) => body,
            Err(_) => {
                let mut response = AppResponse::<()>::request_error("Failed to read request body");
                response.set_request_id(&request_id.to_string());
                return response.into_response();
            }
        };

        let request_data: InputProofRequest =
            match parse_and_validate::<InputProofRequestJson, InputProofRequest>(&body) {
                Ok(request) => request,
                Err(parse_error) => {
                    let error_response: AppResponse<()> =
                        parse_error.to_app_response(&request_id.to_string());
                    return error_response.into_response();
                }
            };

        info!("Successfully parsed and validated request");

        // Register once handlers for receiving the decryption response from the gateway
        let (gateway_response_handler, gateway_response_rx): (
            OnceHandler<RelayerEvent>,
            oneshot::Receiver<RelayerEvent>,
        ) = OnceHandler::new();
        let gateway_response_handler = Arc::new(gateway_response_handler);

        self.orchestrator.register_once_handler(
            InputProofEventId::RespRcvdFromGw.into(),
            JobId::from_uuid_v7(request_id),
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
            JobId::from_uuid_v7(request_id),
            error_handler,
        );
        info!("Registered once handler for handling input proof failure");

        let ext_reference_id = self.orchestrator.new_ext_reference_id();
        if let Err(e) = self
            .input_proof_repo
            .insert_new_input_proof(ext_reference_id, request_id, request_data.clone())
            .await
        {
            error!("Failed to insert input proof into database: {}", e);
            return AppResponse::<()>::internal_server_error_with_request_id(
                request_id.to_string(),
            )
            .into_response();
        }

        let event_data = InputProofEventData::ReqRcvdFromUser {
            input_proof_request: request_data,
        };

        let event = RelayerEvent::new(
            JobId::from_uuid_v7(request_id),
            self.api_version,
            RelayerEventData::InputProof(event_data),
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
