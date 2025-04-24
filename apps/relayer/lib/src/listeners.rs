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
use fhevm_relayer::orchestrator::traits::Event;
use fhevm_relayer::{
    config::settings::KeyUrl,
    core::event::{InputProofRequest, UserDecryptRequest},
    http::{
        input_http_listener::{
            InputProofErrorResponseJson, InputProofRequestJson, InputProofResponseJson,
            InputProofResponsePayloadJson,
        },
        keyurl_http_listener,
        publicdecrypt_http_listener::PublicDecryptErrorResponseJson,
        userdecrypt_http_listener::{
            UserDecryptErrorResponseJson, UserDecryptRequestJson, UserDecryptResponseJson,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
    },
};
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

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
                warn!("SQS listenning error: {:?}", err);
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
            let error_response = UserDecryptErrorResponseJson { message: error };
            return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    }

    // TODO: implement the callback
    // Wait for response on the rx of Onshot channel.
    loop {
        match wait_for_response_with_id(
            &listener_state.sqs_client,
            &listener_state.orchestrator_queue_url,
            request_id,
        )
        .await
        {
            ZwsRelayerEvent::HTTPPrivateDecryptionResponse(value) => {
                info!("Received response event.");
                return (
                    StatusCode::OK,
                    Json(UserDecryptResponseJson {
                        response: value.responses,
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
            ZwsRelayerEvent::HTTPInputRegistrationResponse(value) => {
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
    let app = Router::new()
        // Input registration
        .route("/v1/input-proof", post(input_registration_handler))
        .with_state(Arc::new(HTTPListenerState {
            sqs_client: sqs_client.clone(),
            relayer_queue_url: relayer_queue_url.clone(),
            orchestrator_queue_url: orchestrator_queue_url.clone(),
        }))
        .route("/v1/user-decrypt", post(private_decryption_handler))
        .with_state(Arc::new(HTTPListenerState {
            sqs_client: sqs_client.clone(),
            relayer_queue_url: relayer_queue_url.clone(),
            orchestrator_queue_url: orchestrator_queue_url.clone(),
        }))
        .route("/v1/public-decrypt", post(public_decryption_handler))
        .with_state(Arc::new(HTTPListenerState {
            sqs_client: sqs_client.clone(),
            relayer_queue_url: relayer_queue_url.clone(),
            orchestrator_queue_url: orchestrator_queue_url.clone(),
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
    retry_wait_time: Option<u64>,
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
                warn!("SQS listenning error: {:?}", err);
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    retry_wait_time.unwrap_or(1000),
                ))
                .await;
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

// NOTE: Listener per contract type?
// TODO: Find a cleaner way to handle ctrl+c events
pub async fn blockchain_event_listener(
    // mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    chain_name: ChainName,
    ws_url: String,
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
                            "Couldn't create EthereumJsonRPCWsClient {:?}, {:?} blockchain: {:?}",
                            chain_name, name, error
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
                    info!(blockchain = name,"Received ctrl + c signal, stopping {:?}, {:?} listener...", chain_name, name);
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
