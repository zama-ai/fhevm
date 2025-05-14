use crate::events::*;
use alloy::primitives::Address;
use axum::routing::{get, post};
use axum::{debug_handler, extract::State};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Json, Router,
};
use fhevm_relayer::blockchain::ethereum::{
    ChainName, ContractAndTopicsFilter, EthereumJsonRPCWsClient,
};

use fhevm_relayer::core::utils::OnceHandler;
use fhevm_relayer::{
    config::settings::KeyUrl,
    core::event::{InputProofRequest, UserDecryptRequest},
    http::{
        input_http_listener::{
            InputProofErrorResponseJson, InputProofRequestJson, InputProofResponseJson,
            InputProofResponsePayloadJson,
        },
        keyurl_http_listener,
        public_decrypt_http_listener::PublicDecryptErrorResponseJson,
        userdecrypt_http_listener::{
            UserDecryptErrorResponseJson, UserDecryptRequestJson, UserDecryptResponseJson,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
    },
};
use futures::future::{self, Either};
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub async fn register_once_handler(
    orchestrator: Arc<Orchestrator<TokioEventDispatcher<ZwsRelayerEvent>, ZwsRelayerEvent>>,
    request_id: Uuid,
    event_id: u8,
) -> oneshot::Receiver<ZwsRelayerEvent> {
    // Register once handlers for receiving the decryption response from the gateway l2
    let (handler, rx): (
        OnceHandler<ZwsRelayerEvent>,
        oneshot::Receiver<ZwsRelayerEvent>,
    ) = OnceHandler::new();
    let handler = Arc::new(handler);

    orchestrator.register_once_handler(event_id, request_id, handler);
    info!("registered once handler");
    rx
}

pub struct HTTPListenerState {
    sqs_client: aws_sdk_sqs::Client,
    relayer_queue_url: String,
    orchestrator: Arc<Orchestrator<TokioEventDispatcher<ZwsRelayerEvent>, ZwsRelayerEvent>>,
}

// TODO: change payload type
#[debug_handler]
pub async fn public_decryption_handler(
    State(_listener_state): State<Arc<HTTPListenerState>>,
    Json(_payload): Json<InputProofRequestJson>,
) -> impl IntoResponse {
    debug!("Handling http public decryption request");
    // Validate the payload
    let error_message = "Public decryption isn't implemented yet".to_string();
    let error_response = PublicDecryptErrorResponseJson {
        message: error_message.clone(),
    };
    error!(error_message);
    (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
}

// TODO: make sure that data-types are correct
// it looks like multiple ciphertext can be packed in the same request
#[debug_handler]
pub async fn private_decryption_handler(
    State(listener_state): State<Arc<HTTPListenerState>>,
    Json(payload): Json<UserDecryptRequestJson>,
) -> impl IntoResponse {
    debug!("Handling http private decryption request");
    // Validate the payload
    if let Err(message) = payload.validate() {
        let error_response = InputProofErrorResponseJson { message };
        return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
    }

    let request_id = Uuid::new_v4();

    // Prepare and send an event
    let request_data: UserDecryptRequest = match payload.try_into() {
        Ok(event_data) => event_data,
        Err(message) => {
            let error_response = UserDecryptErrorResponseJson {
                message: message.to_string(),
            };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    };

    let event = ZwsRelayerEvent::HTTPPrivateDecryptionRequest(PrivateDecryptionRequest {
        request_id,
        ct_handle_contract_pairs: request_data.ct_handle_contract_pairs,
        request_validity: request_data.request_validity,
        contracts_chain_id: request_data.contracts_chain_id,
        contract_addresses: request_data.contract_addresses,
        user_address: request_data.user_address,
        public_key: request_data.public_key,
        signature: request_data.signature,
    });

    let rx = register_once_handler(
        Arc::clone(&listener_state.orchestrator),
        request_id,
        PrivateDecryptionResponse::event_id(),
    );
    let error_rx = register_once_handler(
        Arc::clone(&listener_state.orchestrator),
        request_id,
        UnrecoverableError::event_id(),
    );

    match send_message_to_sqs_queue(
        true,
        &listener_state.sqs_client,
        &listener_state.relayer_queue_url,
        &event,
    )
    .await
    {
        Ok(_) => debug!("success sending request"),
        Err(error) => {
            error!(
                "Couldn't send request to sqs: {:?}",
                listener_state.relayer_queue_url
            );
            let error_response = UserDecryptErrorResponseJson { message: error };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    }

    let result = match future::select(rx.await, error_rx.await).await {
        Either::Left((result, _)) => result,
        Either::Right((result, _)) => result,
    };

    match result {
        Ok(event) => {
            match event {
                ZwsRelayerEvent::HTTPPrivateDecryptionResponse(value) => {
                    info!("Received response event.");
                    (
                        StatusCode::OK,
                        Json(UserDecryptResponseJson {
                            response: value.responses,
                        }),
                    )
                        .into_response()
                }
                ZwsRelayerEvent::UnrecoverableError(value) => {
                    error!(
                        "Unrecoverable error return value in http handler: {} from {}",
                        value, value.event,
                    );
                    let error_response = InputProofErrorResponseJson {
                        message: "Failed to handle input registration.".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
                // Should be unreachable
                _ => {
                    let error_response = UserDecryptErrorResponseJson {
                        message: "Failed to handle input registration.".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
            }
        }
        _ => {
            let message = "".to_string();
            let error_response = UserDecryptErrorResponseJson { message };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }
}

#[debug_handler]
pub async fn input_registration_handler(
    State(listener_state): State<Arc<HTTPListenerState>>,
    Json(payload): Json<InputProofRequestJson>,
) -> impl IntoResponse {
    debug!("Handling http input proof request");
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
    let event = ZwsRelayerEvent::HTTPInputRegistrationRequest(HTTPInputRegistrationRequest {
        request_id,
        contract_chain_id: request_data.contract_chain_id,
        contract_address: request_data.contract_address,
        user_address: request_data.user_address,
        ciphetext_with_zk_proof: request_data.ciphetext_with_zk_proof,
    });

    let rx = register_once_handler(
        Arc::clone(&listener_state.orchestrator),
        request_id,
        HTTPInputRegistrationResponse::event_id(),
    );
    let error_rx = register_once_handler(
        Arc::clone(&listener_state.orchestrator),
        request_id,
        UnrecoverableError::event_id(),
    );

    match send_message_to_sqs_queue(
        true,
        &listener_state.sqs_client,
        &listener_state.relayer_queue_url,
        &event,
    )
    .await
    {
        Ok(_) => debug!("success sending request"),
        Err(error) => {
            error!(
                "Couldn't send request to sqs: {:?}",
                listener_state.relayer_queue_url,
            );
            let error_response = InputProofErrorResponseJson { message: error };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    }

    let result = match future::select(rx.await, error_rx.await).await {
        Either::Left((result, _)) => result,
        Either::Right((result, _)) => result,
    };

    match result {
        Ok(event) => {
            match event {
                ZwsRelayerEvent::HTTPInputRegistrationResponse(value) => {
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
                ZwsRelayerEvent::UnrecoverableError(value) => {
                    error!(
                        "Unrecoverable error return value in http handler: {} from {}",
                        value, value.event,
                    );
                    let error_response = InputProofErrorResponseJson {
                        message: "Failed to handle input registration.".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
                // Should be unreachable
                _ => {
                    let error_response = InputProofErrorResponseJson {
                        message: "Failed to handle input registration.".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
            }
        }
        _ => {
            let message = "".to_string();
            let error_response = InputProofErrorResponseJson { message };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }
}

pub fn key_url_route(key_url: KeyUrl) -> keyurl_http_listener::KeyUrlResponseJson {
    debug!("{} {}", key_url.fhe_public_key.url, key_url.crs.url);
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
    key_url: KeyUrl,
    orchestrator: Arc<Orchestrator<TokioEventDispatcher<ZwsRelayerEvent>, ZwsRelayerEvent>>,
    port: u64,
    host: String,
) {
    let app = Router::new()
        // Input registration
        .route("/v1/input-proof", post(input_registration_handler))
        .with_state(Arc::new(HTTPListenerState {
            sqs_client: sqs_client.clone(),
            relayer_queue_url: relayer_queue_url.clone(),
            orchestrator: orchestrator.clone(),
        }))
        .route("/v1/user-decrypt", post(private_decryption_handler))
        .with_state(Arc::new(HTTPListenerState {
            sqs_client: sqs_client.clone(),
            relayer_queue_url: relayer_queue_url.clone(),
            orchestrator: orchestrator.clone(),
        }))
        .route("/v1/public-decrypt", post(public_decryption_handler))
        .with_state(Arc::new(HTTPListenerState {
            sqs_client: sqs_client.clone(),
            relayer_queue_url: relayer_queue_url.clone(),
            orchestrator: orchestrator.clone(),
        }))
        // Key-URL
        .route(
            "/v1/keyurl",
            get(|| async {
                info!("Received GET request to '/keyurl'");
                // TODO: implement -> should be in config back
                Json(key_url_route(key_url))
            }),
        )
        // Root
        .route(
            "/",
            get({
                move || async move {
                    info!("root");
                    Html("<p>Welcome to the relayer!</p>")
                }
            }),
        );
    // .with_state(Arc::new(KeyUrlState { key_url }));

    // Define the socket address for the server to listen on.
    // TODO: set this as a config parameter
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

// TODO: we should probably add a name to listeners
pub async fn sqs_listener<F>(
    sqs_client: aws_sdk_sqs::Client,
    request_queue_url: String,
    retry_wait_time: Option<u64>,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<ZwsRelayerEvent> + HandlerRegistry<ZwsRelayerEvent>,
            ZwsRelayerEvent,
        >,
    >,
    filter: Option<F>,
    name: Option<&str>,
    visibility_timeout: i32,
) where
    F: Fn(&ZwsRelayerEvent) -> bool,
{
    // TODO: SQS client
    let url = &request_queue_url.clone();
    loop {
        let rcv_message_output = match sqs_client
            .receive_message()
            .queue_url(url)
            .wait_time_seconds(10)
            // TODO: DEBUG
            // NOTE: this value should be set only for debug
            .visibility_timeout(visibility_timeout)
            .send()
            .await
        {
            Ok(value) => value,
            Err(err) => {
                warn!("SQS listening error {:?}: {:?}", url, err);
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    retry_wait_time.unwrap_or(1000),
                ))
                .await;
                continue;
            }
        };

        let messages = rcv_message_output.messages.unwrap_or_default();
        if !messages.is_empty() {
            debug!("{:?} Received {} messages from SQS.", name, messages.len());
        }

        for message in messages {
            let event = match message.body() {
                Some(content) => {
                    let payload: ZwsRelayerEvent = match serde_json::from_str(content) {
                        Ok(value) => {
                            debug!("Successfuly parsed relayer event: {} from sqs", value);
                            value
                        }
                        Err(err) => {
                            error!(
                                "{:?} Couldn't deserialize message: {content} with error {err}",
                                name
                            );
                            continue;
                        }
                    };
                    if let Some(ref filter_function) = filter {
                        if !filter_function(&payload) {
                            debug!(
                                "{:?} Skipping {:?} because it didn't pass filter.",
                                payload, name
                            );
                            continue;
                        }
                    }
                    payload
                }
                None => {
                    error!("{:?} Message is empty", name);
                    continue;
                }
            };

            let id = orchestrator.new_request_id();
            debug!(
                file = file!(),
                line = line!(),
                event_id = ?id,
                name = name,
                "Dispatching event"
            );

            // TODO: ERROR handling on event dispatch

            // Dispatch with error logging
            if let Err(e) = orchestrator.dispatch_event(event.clone()).await {
                error!(
                    file = file!(),
                    line = line!(),
                    error = %e,
                    name = name,
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
                    debug!("Deleted message {}", &event);
                }
                Err(err) => {
                    error!("{:?}", err);
                    continue;
                }
            };
        }
    }
}

// NOTE: Listener per contract type?
// TODO: Find a cleaner way to handle ctrl+c events
pub async fn blockchain_event_listener(
    // mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    chain_name: ChainName,
    ws_url: String,
    chain_id: u64,
    contract_addresses: Vec<Address>,
    retry_wait_time: Option<u64>,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<ZwsRelayerEvent> + HandlerRegistry<ZwsRelayerEvent>,
            ZwsRelayerEvent,
        >,
    >,
    name: String,
) {
    let retry_wait = retry_wait_time.unwrap_or(1000);
    'outer: loop {
        // Try to create client with timeout to avoid blocking the task indefinitely
        let client_future = EthereumJsonRPCWsClient::new(chain_name.clone(), ws_url.as_str());
        let client = tokio::select! {
            result = client_future => {
                match result {
                    Ok(value) => value,
                    Err(error) => {
                        warn!(
                            "Couldn't create EthereumJsonRPCWsClient {:?}, {:?} blockchain at {:?}: {:?}",
                            chain_name, name, ws_url, error
                        );

                        // Check for ctrl+c during the retry wait
                        tokio::select! {
                            _ = tokio::time::sleep(tokio::time::Duration::from_millis(retry_wait)) => {
                                continue 'outer;
                            },
                            _ = tokio::signal::ctrl_c() => {
                                info!(blockchain = name, "Received ctrl + c signal while reconnecting, stopping {:?}, {:?} listener...", chain_name, name);
                                break 'outer;
                            }
                        }
                    }
                }
            },
            _ = tokio::signal::ctrl_c() => {
                info!(blockchain = name, "Received ctrl + c signal during connection attempt, stopping {:?}, {:?} listener...", chain_name, name);
                break 'outer;
            }
        };

        let client = Arc::new(client);
        let filter_httpz_host = ContractAndTopicsFilter::new(contract_addresses.clone(), vec![]);

        // Try to create subscription with timeout
        let subscription_future = client.new_subscription(filter_httpz_host.clone(), None);
        let subscription = tokio::select! {
            result = subscription_future => {
                match result {
                    Ok(value) => value,
                    Err(error) => {
                        warn!(
                            "Couldn't create subscription to {:?}, {:?} blockchain {:?} : {:?}",
                            chain_name, name, filter_httpz_host, error
                        );

                        // Check for ctrl+c during the retry wait
                        tokio::select! {
                            _ = tokio::time::sleep(tokio::time::Duration::from_millis(retry_wait)) => {
                                continue 'outer;
                            },
                            _ = tokio::signal::ctrl_c() => {
                                info!(blockchain = name, "Received ctrl + c signal while reconnecting subscription, stopping {:?}, {:?} listener...", chain_name, name);
                                break 'outer;
                            }
                        }
                    }
                }
            },
            _ = tokio::signal::ctrl_c() => {
                info!(blockchain = name, "Received ctrl + c signal during subscription creation, stopping {:?}, {:?} listener...", chain_name, name);
                break 'outer;
            }
        };

        let mut subscription = subscription;

        info!(
            blockchain = name,
            "Successfully connected to {:?}, {:?} blockchain", chain_name, name
        );

        'inner: loop {
            tokio::select! {
                event = subscription.next() => match event {
                    Some(event_log) => {
                        // NOTE: we should probably parse event log here instead of in the handler
                        // and populate the event accordingly
                        let id = orchestrator.new_request_id();
                        let event = ZwsRelayerEvent::BlockchainEvent(BlockchainEvent{
                            request_id: id,
                            event_log,
                            chain_id,
                        });

                        debug!(
                            file = file!(),
                            line = line!(),
                            event_id = ?id,
                            blockchain = name,
                            "Dispatching event"
                        );

                        // Dispatch with error logging
                        // TODO: add mitigation policy in case of dispatch failure
                        if let Err(e) = orchestrator.dispatch_event(event).await {
                            error!(
                                file = file!(),
                                line = line!(),
                                blockchain = name,
                                error = %e,
                                "Failed to dispatch event"
                            );
                        }
                        continue;
                    }
                    None => {
                        info!(blockchain = name,
                            "Subscription stream ended");
                        break 'inner;
                    }
                },
                _ = tokio::signal::ctrl_c() => {
                    info!(blockchain = name, "Received ctrl + c signal, stopping {:?}, {:?} listener...", chain_name, name);
                    break 'outer;
                }
                else => {
                    info!(blockchain = name,"Else , stopping {:?}, {:?} listener...", chain_name, name);
                    break 'outer;
                }
            };
        }
    }
}
