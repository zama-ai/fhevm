use crate::core::event::{
    ApiCategory, ApiVersion, InputProofEventData, InputProofEventId, PublicDecryptEventData,
    PublicDecryptEventId, RelayerEvent, RelayerEventData, UserDecryptEventData, UserDecryptEventId,
};
use crate::core::utils::OnceHandler;
use crate::http::input_http_listener::{
    InputProofErrorResponseJson, InputProofRequestJson, InputProofResponseJson,
};
use crate::http::public_decrypt_http_listener::{
    PublicDecryptErrorResponseJson, PublicDecryptRequestJson, PublicDecryptResponseJson,
};
use crate::http::userdecrypt_http_listener::{
    UserDecryptErrorResponseJson, UserDecryptRequestJson, UserDecryptResponseJson,
};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use aws_config;
use aws_sdk_sqs;
use futures::future::{self, Either};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

// TODO: add correlation-id
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum RequestJson {
    #[serde(rename = "relayer:input-registration:input-registration-request")]
    InputProof(InputProofRequestJson),
    #[serde(rename = "relayer:http-public-decryption:operation-request")]
    PublicDecrypt(PublicDecryptRequestJson),
    #[serde(rename = "relayer:http-private-decryption:operation-request")]
    UserDecrypt(UserDecryptRequestJson),
}

impl TryInto<RelayerEventData> for RequestJson {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RelayerEventData, Self::Error> {
        match self {
            Self::InputProof(value) => Ok(RelayerEventData::InputProof(
                InputProofEventData::ReqRcvdFromUser {
                    input_proof_request: value.try_into()?,
                },
            )),
            Self::PublicDecrypt(value) => Ok(RelayerEventData::PublicDecrypt(
                PublicDecryptEventData::ReqRcvdFromFhevm {
                    decrypt_request: value.try_into()?,
                },
            )),
            Self::UserDecrypt(value) => Ok(RelayerEventData::UserDecrypt(
                UserDecryptEventData::ReqRcvdFromUser {
                    decrypt_request: value.try_into()?,
                },
            )),
        }
    }
}

// TODO: add correlation-id
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum ResponseJson {
    #[serde(rename = "relayer:input-registration:input-registration-response")]
    InputProofResponse(InputProofResponseJson),
    #[serde(rename = "relayer:http-public-decryption:operation-response")]
    PublicDecryptResponse(PublicDecryptResponseJson),
    #[serde(rename = "relayer:http-private-decryption:operation-response")]
    UserDecryptResponse(UserDecryptResponseJson),
    #[serde(rename = "relayer:input-registration:input-registration-error")]
    InputProofError(InputProofErrorResponseJson),
    #[serde(rename = "relayer:http-public-decryption:operation-error")]
    PublicDecryptError(PublicDecryptErrorResponseJson),
    #[serde(rename = "relayer:http-private-decryption:operation-error")]
    UserDecryptError(UserDecryptErrorResponseJson),
}

impl TryFrom<RelayerEventData> for ResponseJson {
    type Error = anyhow::Error;
    fn try_from(value: RelayerEventData) -> Result<Self, Self::Error> {
        match value {
            RelayerEventData::InputProof(inner) => match inner {
                InputProofEventData::Failed { error } => {
                    Ok(ResponseJson::InputProofError(InputProofErrorResponseJson {
                        message: error,
                    }))
                }
                InputProofEventData::RespRcvdFromGw {
                    input_proof_response,
                } => Ok(ResponseJson::InputProofResponse(
                    input_proof_response.into(),
                )),
                _ => Err(anyhow::anyhow!(
                    "Couldn't convert input-proof-event-data: {:?} into response-json",
                    inner
                )),
            },
            RelayerEventData::UserDecrypt(inner) => match inner {
                UserDecryptEventData::Failed { error } => Ok(ResponseJson::UserDecryptError(
                    UserDecryptErrorResponseJson { message: error },
                )),
                UserDecryptEventData::RespRcvdFromGw { decrypt_response } => {
                    Ok(ResponseJson::UserDecryptResponse(decrypt_response.into()))
                }
                _ => Err(anyhow::anyhow!(
                    "Couldn't convert user-decrypt-event-data: {:?} into response-json",
                    inner
                )),
            },
            RelayerEventData::PublicDecrypt(inner) => match inner {
                PublicDecryptEventData::Failed { error } => Ok(ResponseJson::PublicDecryptError(
                    PublicDecryptErrorResponseJson { message: error },
                )),
                PublicDecryptEventData::RespRcvdFromGw { decrypt_response } => {
                    Ok(ResponseJson::PublicDecryptResponse(decrypt_response.into()))
                }
                _ => Err(anyhow::anyhow!(
                    "Couldn't convert public-decrypt-event-data: {:?} into response-json",
                    inner
                )),
            },

            _ => Err(anyhow::anyhow!(
                "Couldn't convert relayer-event-data: {:?} into response-json",
                value
            )),
        }
    }
}

/// A helper struct to hold the flattened Json and the correlation ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CorrelatedJson<T> {
    pub request_id: Uuid,
    #[serde(flatten)]
    pub payload: T,
}

pub fn request_json_to_response_event_id(request_json: RequestJson) -> (u8, u8) {
    match request_json {
        RequestJson::InputProof(_) => (
            InputProofEventId::RespRcvdFromGw.into(),
            InputProofEventId::Failed.into(),
        ),
        RequestJson::UserDecrypt(_) => (
            UserDecryptEventId::RespRcvdFromGw.into(),
            UserDecryptEventId::Failed.into(),
        ),
        RequestJson::PublicDecrypt(_) => (
            PublicDecryptEventId::RespRcvdFromGw.into(),
            PublicDecryptEventId::Failed.into(),
        ),
    }
}

pub async fn register_once_handler<D>(
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    request_id: Uuid,
    event_id: u8,
) -> oneshot::Receiver<RelayerEvent>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    // Register once handlers for receiving the decryption response from the gateway l2
    let (handler, rx): (OnceHandler<RelayerEvent>, oneshot::Receiver<RelayerEvent>) =
        OnceHandler::new();
    let handler = Arc::new(handler);

    orchestrator.register_once_handler(event_id, request_id, handler);
    info!("registered once handler");
    rx
}

// TODO: migrate to use anyhow errors
// TODO: define an error mitigation policy in case of SQS publishing failure
pub async fn send_message_to_sqs_queue<T>(
    sqs_client: &aws_sdk_sqs::Client,
    queue_url: &String,
    message: &T,
) -> std::result::Result<aws_sdk_sqs::operation::send_message::SendMessageOutput, std::string::String>
where
    T: serde::Serialize,
{
    let serialized_message = match serde_json::to_string(&message) {
        Ok(value) => value,
        Err(err) => {
            let err_msg = format!("Error serializing message to JSON: {:?}", err);
            return Err(err_msg);
        }
    };
    let publishing_response = match sqs_client
        .send_message()
        .queue_url(queue_url)
        .message_body(serialized_message)
        // If the queue is FIFO, you need to set .message_deduplication_id
        // and message_group_id or configure the queue for ContentBasedDeduplication.
        .send()
        .await
    {
        Err(error) => {
            let err_msg = format!("Error publishing: {:?}", error);
            return Err(err_msg);
        }
        Ok(response) => response,
    };
    Ok(publishing_response)
}

// TODO: send generic/or-not error in case of failures instead of just returns
pub async fn process_sqs_message<D>(
    content: String,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    api_version: ApiVersion,
    outbound_queue: String,
    sqs_client: aws_sdk_sqs::Client,
) where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    // Parse SQS message to any of the support json requests
    let request_json: CorrelatedJson<RequestJson> = match serde_json::from_str(content.as_str()) {
        Ok(value) => {
            debug!("Successfuly parsed relayer event: {:?} from sqs", value);
            value
        }
        Err(err) => {
            error!("Couldn't deserialize message: {content} with error {err}");
            return;
        }
    };

    // Map RequestJson to RelayerEvent
    let request_id = request_json.request_id;
    let request_data: RelayerEventData = match request_json.payload.clone().try_into() {
        Ok(event_data) => event_data,
        Err(message) => {
            // TODO: return error directly to backend
            error!("{:?}", message);
            return;
            // let error_response = InputProofErrorResponseJson { message };
            // return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
        }
    };
    let event = RelayerEvent::new(request_id, api_version, request_data);

    // Register handlers for response
    // TODO: modify this to support proper event id
    let (event_id, failure_event_id) = request_json_to_response_event_id(request_json.payload);
    let rx = register_once_handler(Arc::clone(&orchestrator), request_id, event_id);
    let error_rx = register_once_handler(Arc::clone(&orchestrator), request_id, failure_event_id);

    // Dispatch event
    // TODO: proper error handling on event dispatch
    if let Err(e) = orchestrator.dispatch_event(event.clone()).await {
        error!(
            file = file!(),
            line = line!(),
            error = %e,
            "Failed to dispatch event"
        );
        return;
    }

    // Handle result or error
    let result = match future::select(rx.await, error_rx.await).await {
        Either::Left((result, _)) => result,
        Either::Right((result, _)) => result,
    };

    let response_json: ResponseJson = match result {
        Ok(event) => match event.data.try_into() {
            Ok(value) => value,
            _ => {
                return;
            }
        },
        _ => {
            // TODO: send error response back
            return;
        }
    };
    let message = CorrelatedJson::<ResponseJson> {
        request_id,
        payload: response_json,
    };

    match send_message_to_sqs_queue(&sqs_client, &outbound_queue, &message).await {
        Ok(_) => debug!("success sending response back to sqs: {outbound_queue}"),
        Err(error) => {
            error!("Couldn't send request to sqs: {outbound_queue} with error: {error:?}");
        }
    }
}

pub async fn wait_for_response_with_id(
    sqs_client: &aws_sdk_sqs::Client,
    request_id: uuid::Uuid,
    queue: String,
) -> anyhow::Result<CorrelatedJson<ResponseJson>> {
    let wait_time_seconds: i32 = 10;
    loop {
        let rcv_message_output = match sqs_client
            .receive_message()
            .queue_url(queue.clone())
            .wait_time_seconds(wait_time_seconds)
            .visibility_timeout(0)
            .send()
            .await
        {
            Ok(value) => value,
            Err(err) => {
                warn!("SQS listening error {:?}: {:?}", queue, err);
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                continue;
            }
        };

        let messages = rcv_message_output.messages.unwrap_or_default();
        if !messages.is_empty() {
            debug!("{:?} Received {} messages from SQS.", queue, messages.len());
        } else {
            debug!(
                "{:?} Received 0 messages from SQS in the last {} seconds",
                queue, wait_time_seconds
            );
        }

        for message in messages {
            match message.body() {
                Some(content) => {
                    let content = content.to_string();
                    let request_json: CorrelatedJson<ResponseJson> =
                        match serde_json::from_str(content.as_str()) {
                            Ok(value) => {
                                debug!("Successfuly parsed relayer event: {:?} from sqs", value);
                                value
                            }
                            Err(err) => {
                                error!("Couldn't deserialize message: {content} with error {err}");
                                continue;
                            }
                        };
                    if request_json.request_id == request_id {
                        // NOTE: we need to delete messages once process otherwise they stay in the queue.
                        // The question is whether we should delete them once we get them or once they are
                        // processed (imagine we have multiple consumers).
                        match sqs_client
                            .delete_message()
                            .queue_url(&queue)
                            .set_receipt_handle(message.receipt_handle)
                            .send()
                            .await
                        {
                            Ok(_) => {
                                debug!("Deleted message");
                            }
                            Err(err) => {
                                error!("{:?}", err);
                            }
                        };

                        return Ok(request_json);
                    }
                }
                None => {
                    error!("{:?} Message is empty", queue);
                    continue;
                }
            };
        }
    }
}

// TODO: we should probably have a sqs-server instead of just a listener
// to be able to properly handle responses
pub async fn run_sqs_server<D>(
    inbound_queue: String,
    outbound_queue: String,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
) where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);
    let config = aws_config::from_env().load().await;
    info!("config endpoint url: {:?}", config.endpoint_url());
    let sqs_client = aws_sdk_sqs::Client::new(&config);
    let visibility_timeout = 30;
    let wait_time_seconds = 10;
    let wait_time_between_sqs_retries = tokio::time::Duration::from_millis(1000);

    loop {
        let rcv_message_output = match sqs_client
            .receive_message()
            .queue_url(inbound_queue.clone())
            .wait_time_seconds(wait_time_seconds)
            // TODO: DEBUG
            // NOTE: this value should be set only for debug
            .visibility_timeout(visibility_timeout)
            .send()
            .await
        {
            Ok(value) => value,
            Err(err) => {
                warn!("SQS listening error {:?}: {:?}", inbound_queue, err);
                tokio::time::sleep(wait_time_between_sqs_retries).await;
                continue;
            }
        };

        let messages = rcv_message_output.messages.unwrap_or_default();
        if !messages.is_empty() {
            debug!(
                "{:?} Received {} messages from SQS.",
                inbound_queue,
                messages.len()
            );
        } else {
            debug!(
                "{:?} Received 0 messages from SQS in the last {} seconds",
                inbound_queue, wait_time_seconds
            );
        }

        for message in messages {
            match message.body() {
                Some(content) => {
                    let content = content.to_string();
                    let orchestrator = orchestrator.clone();
                    let sqs_client = sqs_client.clone();
                    let outbound_queue = outbound_queue.clone();
                    tokio::spawn(async move {
                        process_sqs_message(
                            content,
                            orchestrator,
                            api_version,
                            outbound_queue,
                            sqs_client,
                        )
                        .await
                    });
                }
                None => {
                    error!("{:?} Message is empty", inbound_queue);
                    continue;
                }
            };

            // NOTE: we need to delete messages once process otherwise they stay in the queue.
            // The question is whether we should delete them once we get them or once they are
            // processed (imagine we have multiple consumers).
            match sqs_client
                .delete_message()
                .queue_url(&inbound_queue)
                .set_receipt_handle(message.receipt_handle)
                .send()
                .await
            {
                Ok(_) => {
                    debug!("Deleted message");
                }
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };
        }
    }
}
