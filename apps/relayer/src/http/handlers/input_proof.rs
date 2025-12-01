use crate::core::event::{
    ApiVersion, InputProofEventData, InputProofEventId, InputProofRequest, RelayerEvent,
    RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::http::types::input_proof::InputProofRequestJson;
use crate::http::{parse_and_validate, AppResponse};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::OnceHandler;
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::input_proof_repo::InputProofRepository;
use axum::{body::Bytes, extract::FromRequest, http::Request, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, instrument, span, Level};

pub type InputProofResponse = AppResponse<crate::http::types::input_proof::InputProofResponseJson>;

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

    #[instrument(name = "handle-input", skip_all, fields(request_id))]
    pub async fn handle<S>(&self, req: Request<axum::body::Body>, _state: &S) -> impl IntoResponse
    where
        S: Send + Sync,
    {
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
