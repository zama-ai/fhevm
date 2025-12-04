use crate::{
    config::settings::{AppConfigError, ContractConfig},
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, InputProofEventData, InputProofRequest, InputProofResponse,
            RelayerEvent, RelayerEventData,
        },
        job_id::JobId,
    },
    gateway::arbitrum::transaction::helper::{TransactionHelper, TransactionType},
    gateway::arbitrum::{bindings::InputVerification, ComputeCalldata},
    gateway::utils::sql_errors,
    orchestrator::{
        traits::{Event, EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    store::sql::repositories::input_proof_repo::InputProofRepository,
};
use std::str::FromStr;

use alloy::primitives::{Address, FixedBytes, TxHash, U256};

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct InputProofGatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
    input_proof_repo: Arc<InputProofRepository>,
}

impl InputProofGatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        tx_helper: Arc<TransactionHelper>,
        contracts: ContractConfig,
        input_proof_repo: Arc<InputProofRepository>,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper,
            contracts,
            input_proof_repo,
        }
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for InputProofGatewayHandler {
    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn handle_event(&self, event: RelayerEvent) {
        match &event.data {
            RelayerEventData::InputProof(input_event) => match input_event {
                InputProofEventData::ReqRcvdFromUser {
                    input_proof_request,
                } => {
                    info!("Processing input proof request {}", event.job_id);
                    self.send_input_proof_request(event.clone(), input_proof_request.clone())
                        .await;
                }
                _ => {
                    warn!("unexpected event received in input handler")
                }
            },

            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                ref log,
                tx_hash,
            }) => {
                if let Some(topic0) = log.topic0() {
                    let topic0_fixed = FixedBytes::<32>::from_slice(topic0.as_slice());

                    match topic0_fixed {
                        InputVerification::VerifyProofResponse::SIGNATURE_HASH => {
                            info!(
                                "Processing accepted proof response for request {}",
                                event.job_id
                            );
                            self.complete_proof_verification(event.clone(), log, *tx_hash)
                                .await;
                        }
                        InputVerification::RejectProofResponse::SIGNATURE_HASH => {
                            info!(
                                "Processing rejected proof response for request {}",
                                event.job_id
                            );
                            self.reject_proof_verification(event.clone(), log, *tx_hash)
                                .await;
                        }
                        _ => {}
                    }
                };
            }
            _ => {}
        }
    }
}

impl InputProofGatewayHandler {
    async fn send_input_proof_request(
        &self,
        event: RelayerEvent,
        input_proof_request: InputProofRequest,
    ) {
        info!(
            "Sending input proof request to gateway for {}",
            event.job_id
        );

        match self.send_to_gateway(&input_proof_request).await {
            Ok((input_verification_id, tx_hash)) => {
                info!("Input proof request sent to gateway for {}", event.job_id);
                self.store_request_receipt(event, input_verification_id, tx_hash)
                    .await;
            }
            Err(e) => {
                self.mark_failed_and_notify(event, e).await;
            }
        }
    }

    async fn send_to_gateway(
        &self,
        input_proof_request: &InputProofRequest,
    ) -> Result<(U256, TxHash), EventProcessingError> {
        let input_verification_address =
            Address::from_str(&self.contracts.input_verification_address).map_err(|_| {
                EventProcessingError::ConfigError(AppConfigError::InvalidAddress(
                    "contracts.input_verification_address".to_owned(),
                ))
            })?;

        info!(
            "input_verification_address used for input request {:?}",
            input_verification_address
        );

        let receipt = self
            .tx_helper
            .send_raw_transaction_sync(
                TransactionType::InputRequest,
                input_verification_address,
                || {
                    ComputeCalldata::verify_proof_req(
                        input_proof_request.contract_chain_id,
                        input_proof_request.contract_address,
                        input_proof_request.user_address,
                        input_proof_request.ciphetext_with_zk_proof.clone(),
                        input_proof_request.extra_data.clone(),
                    )
                },
            )
            .await?;

        // Extract gateway reference ID from the VerifyProofRequest event
        let gw_reference_id = TransactionHelper::extract_gateway_id_from_receipt::<
            InputVerification::VerifyProofRequest,
        >(
            &receipt,
            InputVerification::VerifyProofRequest::SIGNATURE_HASH,
            |event| event.zkProofId,
        )?;

        Ok((gw_reference_id, receipt.transaction_hash))
    }

    async fn complete_proof_verification(
        &self,
        event: RelayerEvent,
        log: &alloy::rpc::types::Log,
        tx_hash: TxHash,
    ) {
        match InputVerification::VerifyProofResponse::decode_log_data(log.data()) {
            Ok(request_event) => {
                info!(
                    input_verification_id = ?request_event.zkProofId,
                    handles = ?request_event.ctHandles,
                    signatures = ?request_event.signatures,
                    "Processing InputResponse event"
                );

                let input_proof_response = InputProofResponse {
                    handles: request_event.ctHandles,
                    signatures: request_event.signatures,
                };

                let tx_hash_str = format!("{:?}", tx_hash);
                let int_request_id = match self
                    .input_proof_repo
                    .accept_and_complete_input_proof_req(
                        request_event.zkProofId,
                        input_proof_response.clone(),
                        &tx_hash_str,
                    )
                    .await
                {
                    Ok(int_request_id) => int_request_id,
                    Err(e) => {
                        error!(
                            conversion_error = %e,
                            "Failed to convert U256 zkproof ID to i64"
                        );
                        self.notify_failed(
                            event,
                            EventProcessingError::ValidationFailed {
                                field: "zkproof_id".to_string(),
                                reason: "value too large for i64".to_string(),
                            },
                        )
                        .await;
                        return;
                    }
                };

                let next_event_data: RelayerEventData =
                    RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                        accepted: true,
                        input_proof_response: Some(input_proof_response),
                    });

                let next_event = RelayerEvent::new(
                    JobId::from_uuid_v7(int_request_id),
                    event.api_version,
                    next_event_data,
                );

                if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                    error!(?e, "Failed to dispatch input proof response event");
                } else {
                    info!(
                        "Input proof response successfully sent for {}",
                        event.job_id
                    );
                }
            }
            Err(err) => {
                error!(?err, "Failed to decode VerifyProofResponse event");
                self.notify_failed(
                    event,
                    EventProcessingError::EventDecodingFailed {
                        event_type: "VerifyProofResponse".to_string(),
                        reason: err.to_string(),
                    },
                )
                .await;
            }
        }
    }

    async fn reject_proof_verification(
        &self,
        event: RelayerEvent,
        log: &alloy::rpc::types::Log,
        tx_hash: TxHash,
    ) {
        match InputVerification::RejectProofResponse::decode_log_data(log.data()) {
            Ok(reject_proof_response) => {
                match self
                    .input_proof_repo
                    .reject_and_complete_input_proof_req(
                        reject_proof_response.zkProofId,
                        "Proof Rejected".to_string(),
                        &format!("{:?}", tx_hash),
                    )
                    .await
                {
                    Ok(int_request_id) => {
                        let next_event_data: RelayerEventData =
                            RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                                accepted: false,
                                input_proof_response: None,
                            });

                        let next_event = RelayerEvent::new(
                            JobId::from_uuid_v7(int_request_id),
                            event.api_version,
                            next_event_data,
                        );

                        let _ = self.dispatcher.dispatch_event(next_event).await;
                    }
                    Err(e) => {
                        sql_errors::input_proof_sql_error(
                            &self.dispatcher,
                            event,
                            "input_proof.reject_and_complete_input_proof_req",
                            &e,
                        )
                        .await;
                    }
                };
            }
            Err(err) => {
                error!(?err, "Failed to decode RejectProofResponse event");
                self.notify_failed(
                    event,
                    EventProcessingError::EventDecodingFailed {
                        event_type: "RejectProofResponse".to_string(),
                        reason: err.to_string(),
                    },
                )
                .await;
            }
        }
    }

    async fn store_request_receipt(
        &self,
        event: RelayerEvent,
        input_verification_id: U256,
        tx_hash: TxHash,
    ) {
        let int_request_id = match event.job_id.as_uuid_v7() {
            Some(uuid) => uuid,
            None => {
                error!(job_id = %event.job_id, "job_id is not uuid");
                return self
                    .notify_failed(
                        event,
                        EventProcessingError::ValidationFailed {
                            field: "job_id".to_string(),
                            reason: "not a valid UUID".to_string(),
                        },
                    )
                    .await;
            }
        };

        let tx_hash_str = format!("{:?}", tx_hash);
        if self
            .input_proof_repo
            .update_input_proof_status_to_receipt_received(
                int_request_id,
                &tx_hash_str,
                input_verification_id,
            )
            .await
            .is_err()
        {
            sql_errors::input_proof_sql_error(
                &self.dispatcher,
                event,
                "input_proof.update_input_proof_status_to_receipt_received",
                &"SQL update failed",
            )
            .await;
            return;
        }
    }

    async fn mark_failed_and_notify(&self, event: RelayerEvent, error: EventProcessingError) {
        let int_request_id = match event.job_id.as_uuid_v7() {
            Some(uuid) => uuid,
            None => {
                error!(
                    job_id = %event.job_id,
                    "job_id is not uuid"
                );
                self.notify_failed(
                    event,
                    EventProcessingError::ValidationFailed {
                        field: "job_id".to_string(),
                        reason: "not a valid UUID".to_string(),
                    },
                )
                .await;
                return;
            }
        };

        // TODO(xyz): Handle error
        let _result = self
            .input_proof_repo
            .update_status_to_failure(int_request_id, &error.to_string())
            .await;
        self.notify_failed(event, error).await;
    }

    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn notify_failed(&self, event: RelayerEvent, error: EventProcessingError) {
        error!(
            error = ?error,
            "Failed to process input request"
        );

        let error_event =
            event.derive_next_event(RelayerEventData::InputProof(InputProofEventData::Failed {
                error,
            }));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }
}
