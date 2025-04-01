use crate::events::*;
use alloy::rpc::types::Log;
use axum::routing::{get, post};
use axum::{debug_handler, extract::State};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Json, Router,
};
use fhevm_relayer::orchestrator::traits::Event;
use fhevm_relayer::{
    config::settings::KeyUrl,
    core::event::InputProofRequest,
    core::utils::OnceHandler,
    http::{
        input_http_listener::{
            InputProofErrorResponseJson, InputProofRequestJson, InputProofResponseJson,
            InputProofResponsePayloadJson,
        },
        keyurl_http_listener,
    },
    orchestrator::{
        traits::{EventDispatcher, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
    },
};
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub struct ProofHandlerState<T>
where
    T: EventDispatcher<ZwsRelayerEvent> + HandlerRegistry<ZwsRelayerEvent>,
{
    orchestrator: Arc<Orchestrator<T, ZwsRelayerEvent>>,
}

pub async fn wait_for_response_with_id(
    sqs_client: &aws_sdk_sqs::Client,
    request_queue_url: &str,
    request_id: Uuid,
) -> ZwsRelayerEvent {
    loop {
        let rcv_message_output = match sqs_client
            .receive_message()
            .queue_url(request_queue_url)
            .wait_time_seconds(10)
            .send()
            .await
        {
            Ok(value) => value,
            Err(err) => {
                warn!("{:?}", err);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
        };

        let messages = rcv_message_output.messages.unwrap_or_default();
        if !messages.is_empty() {
            debug!("Received {} messages from SQS.", messages.len());
        }

        for message in messages {
            match message.body() {
                Some(content) => {
                    match serde_json::from_str::<ZwsRelayerEvent>(content) {
                        Ok(value) => {
                            debug!("successfuly parsed content from sqs: {:?}", content);
                            if value.request_id() == request_id {
                                match sqs_client
                                    .delete_message()
                                    .queue_url(request_queue_url)
                                    .set_receipt_handle(message.receipt_handle)
                                    .send()
                                    .await
                                {
                                    Ok(_) => {
                                        debug!("message deleted");
                                    }
                                    Err(err) => {
                                        error!("error deleting message: {:?}", err);
                                    }
                                };
                                return value;
                            }
                        }
                        Err(err) => {
                            error!("Couldn't deserialize message: {content} with error {err}");
                            continue;
                        }
                    };
                }
                None => {
                    error!("Message is empty");
                    continue;
                }
            };
        }
    }
}

pub struct HTTPListenerState {
    sqs_client: aws_sdk_sqs::Client,
    relayer_queue_url: String,
    orchestrator_queue_url: String,
}

#[debug_handler]
pub async fn input_registration_handler_sqs(
    State(listener_state): State<Arc<HTTPListenerState>>,
    Json(payload): Json<InputProofRequestJson>,
) -> impl IntoResponse {
    debug!("Handling input proof request");
    // Validate the payload
    if let Err(message) = payload.validate() {
        let error_response = InputProofErrorResponseJson { message };
        return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
    }

    let request_id = Uuid::new_v4();

    // Prepare and send an event
    let request_data: InputProofRequest = match payload.try_into() {
        Ok(event_data) => event_data,
        Err(message) => {
            let error_response = InputProofErrorResponseJson { message };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    };

    // NOTE: we could use SNS insteaf of the orchestrator dispatch here
    // but since it's just a mock it should be fine
    let event =
        ZwsRelayerEvent::SQSRelayerInputRegistrationRequest(SQSRelayerInputRegistrationRequest {
            request_id,
            contract_chain_id: request_data.contract_chain_id,
            contract_address: request_data.contract_address,
            user_address: request_data.user_address,
            ciphetext_with_zk_proof: request_data.ciphetext_with_zk_proof,
        });

    match send_message_to_sqs_queue(
        true,
        &listener_state.sqs_client,
        &listener_state.relayer_queue_url,
        event,
    )
    .await
    {
        Ok(_) => debug!("success sending request"),
        Err(error) => {
            error!("Couldn't send request to sqs");
            let error_response = InputProofErrorResponseJson { message: error };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    }

    // Wait for response on the rx of Onshot channel.
    loop {
        match wait_for_response_with_id(
            &listener_state.sqs_client,
            &listener_state.orchestrator_queue_url,
            request_id,
        )
        .await
        {
            ZwsRelayerEvent::SQSRelayerInputRegistrationResponse(value) => {
                info!("Received response event.");
                return (
                    StatusCode::OK,
                    Json(InputProofResponseJson {
                        response: InputProofResponsePayloadJson {
                            handles: value.handles.iter().map(|elt| elt.to_string()).collect(),
                            signatures: value
                                .signatures
                                .iter()
                                .map(|elt| elt.to_string())
                                .collect(),
                        },
                    }),
                )
                    .into_response();
            }
            _ => {
                debug!("Received an event but not the response yet")
            }
        };
    }
}

#[debug_handler]
pub async fn input_registration_handler_orchestrator(
    State(input_handler_state): State<
        Arc<ProofHandlerState<TokioEventDispatcher<ZwsRelayerEvent>>>,
    >,
    Json(payload): Json<InputProofRequestJson>,
) -> impl IntoResponse {
    debug!("Handling input proof request");
    // Validate the payload
    if let Err(message) = payload.validate() {
        let error_response = InputProofErrorResponseJson { message };
        return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
    }

    // Generate Request ID
    let request_id = input_handler_state.orchestrator.new_request_id();

    info!("validated and assigned request id: {}", request_id);

    // Register once handlers for receiving the decryption response from the gateway l2
    let (handler, rx): (
        OnceHandler<ZwsRelayerEvent>,
        oneshot::Receiver<ZwsRelayerEvent>,
    ) = OnceHandler::new();
    let handler = Arc::new(handler);

    input_handler_state.orchestrator.register_once_handler(
        SQSRelayerInputRegistrationResponse::event_id(),
        request_id,
        handler,
    );
    info!("registered once handler");

    // Prepare and send an event
    let request_data: InputProofRequest = match payload.try_into() {
        Ok(event_data) => event_data,
        Err(message) => {
            let error_response = InputProofErrorResponseJson { message };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    };

    // NOTE: we could use SNS insteaf of the orchestrator dispatch here
    // but since it's just a mock it should be fine
    let event =
        ZwsRelayerEvent::SQSRelayerInputRegistrationRequest(SQSRelayerInputRegistrationRequest {
            request_id,
            contract_chain_id: request_data.contract_chain_id,
            contract_address: request_data.contract_address,
            user_address: request_data.user_address,
            ciphetext_with_zk_proof: request_data.ciphetext_with_zk_proof,
        });

    let _ = input_handler_state.orchestrator.dispatch_event(event).await;
    debug!("dispatched event to orchestrator to initiate processing");

    debug!("waiting for reponse event");

    // Wait for response on the rx of Onshot channel.
    match rx.await {
        Ok(event) => {
            match event {
                ZwsRelayerEvent::SQSRelayerInputRegistrationResponse(value) => {
                    info!("Received response event.");
                    (
                        StatusCode::OK,
                        Json(InputProofResponseJson {
                            response: InputProofResponsePayloadJson {
                                handles: value.handles.iter().map(|elt| elt.to_string()).collect(),
                                signatures: value
                                    .signatures
                                    .iter()
                                    .map(|elt| elt.to_string())
                                    .collect(),
                            },
                        }),
                    )
                        .into_response()
                }
                _ => {
                    // TODO: properly manage errors here
                    let error_response = InputProofErrorResponseJson {
                        message: "Failed to handle input registration.".to_string(),
                    };

                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
            }
        }
        Err(error) => {
            debug!(
                "Received errror while waiting for response event: {:?}",
                error
            );
            // TODO: properly manage errors here
            let error_response = InputProofErrorResponseJson {
                message: "Failed to handle input registration.".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

pub fn key_url_route(key_url: KeyUrl) -> keyurl_http_listener::KeyUrlResponseJson {
    keyurl_http_listener::KeyUrlResponseJson {
        response: keyurl_http_listener::Response {
            fhe_key_info: vec![keyurl_http_listener::FheKeyInfo {
                fhe_public_key: keyurl_http_listener::KeyData {
                    data_id: key_url.fhe_public_key.data_id,
                    urls: vec![key_url.fhe_public_key.url],
                },
            }],
            crs: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "2048".to_string(),
                    keyurl_http_listener::KeyData {
                        data_id: key_url.crs.data_id,
                        urls: vec![key_url.crs.url],
                    },
                );
                map
            },
        },
    }
}

// TODO: remove the orchestrator and send messages to SQS instead
pub async fn http_listener(
    sqs_client: aws_sdk_sqs::Client,
    relayer_queue_url: String,
    orchestrator_queue_url: String,
    key_url: KeyUrl,
    _orchestrator: Arc<Orchestrator<TokioEventDispatcher<ZwsRelayerEvent>, ZwsRelayerEvent>>,
) {
    // let input_handler = Arc::new(InputProofHandler::new(orchestrator));
    // let shared_state = Arc::new(orchestrator);
    //
    //

    // TODO: add private/user-decryption route
    let app = Router::new()
        .route("/input-proof", post(input_registration_handler_sqs))
        .with_state(Arc::new(HTTPListenerState {
            sqs_client,
            relayer_queue_url,
            orchestrator_queue_url,
        }))
        .route(
            "/",
            get({
                move || async move {
                    info!("root");
                    Html("<p>Welcome to the relayer!</p>")
                }
            }),
        )
        .route(
            "/keyurl",
            get(|| async {
                info!("Received GET request to '/keyurl'");
                // TODO: implement -> should be in config back
                Json(key_url_route(key_url))
            }),
        );
    // .with_state(Arc::new(KeyUrlState { key_url }));

    // Define the socket address for the server to listen on.
    let host = "0.0.0.0";
    let port = 4324;
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address");

    println!("Server listening on http://{}", addr);

    // Start the server with hyper underneath.
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(error) => {
            let msg_value = format!(
                "Error creating TcpListener with address: {:?}. Error: {:?}",
                addr, error
            );
            error!(msg_value);
            return;
        }
    };
    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(error) => {
            let msg_value = format!("Error serving with axum: {:?}", error);
            error!(msg_value);
        }
    };
}

pub async fn sqs_listener(
    sqs_client: aws_sdk_sqs::Client,
    request_queue_url: String,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<ZwsRelayerEvent> + HandlerRegistry<ZwsRelayerEvent>,
            ZwsRelayerEvent,
        >,
    >,
) {
    // TODO: SQS client
    let url = &request_queue_url.clone();
    loop {
        let rcv_message_output = match sqs_client
            .receive_message()
            .queue_url(url)
            .wait_time_seconds(10)
            .send()
            .await
        {
            Ok(value) => value,
            Err(err) => {
                warn!("{:?}", err);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
        };

        let messages = rcv_message_output.messages.unwrap_or_default();
        if !messages.is_empty() {
            debug!("Received {} messages from SQS.", messages.len());
        }

        for message in messages {
            let event = match message.body() {
                Some(content) => {
                    let payload: ZwsRelayerEvent = match serde_json::from_str(content) {
                        Ok(value) => {
                            debug!("successfuly parsed content from sqs: {:?}", content);
                            value
                        }
                        Err(err) => {
                            error!("Couldn't deserialize message: {content} with error {err}");
                            continue;
                        }
                    };
                    payload
                }
                None => {
                    error!("Message is empty");
                    continue;
                }
            };

            let id = orchestrator.new_request_id();
            debug!(
                file = file!(),
                line = line!(),
                event_id = ?id,
                "Dispatching event"
            );

            // TODO: ERROR handling on event dispatch

            // Dispatch with error logging
            if let Err(e) = orchestrator.dispatch_event(event).await {
                error!(
                    file = file!(),
                    line = line!(),
                    error = %e,
                    "Failed to dispatch event"
                );
            }

            // NOTE: we need to delete messages once process otherwise they stay in the queue.
            // The question is whether we should delete them once we get them or once they are
            // processed (imagine we have multiple consumers).
            match sqs_client
                .delete_message()
                .queue_url(url)
                .set_receipt_handle(message.receipt_handle)
                .send()
                .await
            {
                Ok(_) => {
                    debug!("message deleted");
                }
                Err(err) => {
                    error!("{:?}", err);
                    continue;
                }
            };
        }
    }
}

// Listener per contract type?
pub async fn blockchain_event_listener(
    mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<ZwsRelayerEvent> + HandlerRegistry<ZwsRelayerEvent>,
            ZwsRelayerEvent,
        >,
    >,
    name: String,
) {
    loop {
        tokio::select! {
            event = subscription.next() => match event {
                Some(event_log) => {
                    // NOTE: we should probably parse event log here instead of in the handler
                    // and populate the event accordingly
                    let id = orchestrator.new_request_id();
                    let event = ZwsRelayerEvent::BlockchainEvent(BlockchainEvent{
                        request_id: id,
                        event_log,
                    });

                    debug!(
                        file = file!(),
                        line = line!(),
                        event_id = ?id,
                        blockchain = name,
                        "Dispatching event"
                    );

                    // Dispatch with error logging
                    if let Err(e) = orchestrator.dispatch_event(event).await {
                        error!(
                            file = file!(),
                            line = line!(),
                            blockchain = name,
                            error = %e,
                            "Failed to dispatch event"
                        );
                    }
                }
                None => {
                    info!(blockchain = name,
                        "Subscription stream ended");
                    break;
                }
            },
            _ = tokio::signal::ctrl_c() => {
                info!(blockchain = name,"Received ctrl + c signal, stopping...");
                break;
            }
        };
    }
}
