//! Startup recovery for incomplete requests
//!
//! Recovers requests interrupted by crashes/restarts by re-dispatching events based on status:
//! - `queued`: ReqRcvdFromUser (includes readiness check)
//! - `processing`/`tx_in_flight`: ReadinessCheckPassed (skips readiness check)
//! - `receipt_received`: Not recovered (gateway listener handles automatically)

use crate::{
    core::event::{
        ApiCategory, ApiVersion, InputProofEventData, InputProofRequest, PublicDecryptEventData,
        PublicDecryptRequest, RelayerEvent, RelayerEventData, UserDecryptEventData,
        UserDecryptRequest,
    },
    core::job_id::JobId,
    orchestrator::{traits::EventDispatcher, Orchestrator, TokioEventDispatcher},
    store::sql::{models::req_status_enum_model::ReqStatus, repositories::Repositories},
};
use std::sync::Arc;
use tracing::{info, warn};

/// Recover incomplete requests by re-dispatching events based on their status.
pub async fn recover_incomplete_requests(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    repositories: &Arc<Repositories>,
) -> eyre::Result<usize> {
    info!("Starting request recovery...");

    let mut total_recovered = 0;

    // Recover public decrypt requests
    total_recovered += recover_public_decrypt_requests(orchestrator, repositories).await?;

    // Recover user decrypt requests
    total_recovered += recover_user_decrypt_requests(orchestrator, repositories).await?;

    // Recover input proof requests
    total_recovered += recover_input_proof_requests(orchestrator, repositories).await?;

    info!(
        "Request recovery completed. Recovered {} requests",
        total_recovered
    );

    Ok(total_recovered)
}

/// Recover incomplete public decrypt requests
async fn recover_public_decrypt_requests(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    repositories: &Arc<Repositories>,
) -> eyre::Result<usize> {
    let mut requests = repositories
        .public_decrypt
        .find_incomplete_requests()
        .await
        .map_err(|e| eyre::eyre!("Failed to query incomplete public decrypt requests: {}", e))?;

    // Process furthest-along requests first: TxInFlight > Processing > Queued
    requests.sort_by_key(|(_, _, status, _)| match status {
        ReqStatus::TxInFlight => 0,
        ReqStatus::Processing => 1,
        ReqStatus::Queued => 2,
        _ => 3,
    });

    let mut recovered = 0;

    for (int_job_id, req_json, status, _updated_at) in requests {
        match serde_json::from_value::<PublicDecryptRequest>(req_json.clone()) {
            Ok(request) => {
                let job_id = JobId::from_sha256_hash(int_job_id.try_into().unwrap_or([0u8; 32]));
                let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

                let event_data = match status {
                    ReqStatus::Queued => {
                        RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                            decrypt_request: request,
                        })
                    }
                    ReqStatus::Processing | ReqStatus::TxInFlight => {
                        RelayerEventData::PublicDecrypt(
                            PublicDecryptEventData::ReadinessCheckPassed {
                                decrypt_request: request,
                            },
                        )
                    }
                    _ => continue,
                };

                let event = RelayerEvent::new(job_id, api_version, event_data);
                if let Err(e) = orchestrator.dispatch_event(event).await {
                    warn!("Failed to recover public decrypt request: {}", e);
                } else {
                    recovered += 1;
                }
            }
            Err(e) => {
                warn!(
                    "Failed to deserialize public decrypt request (skipping): {} - req_json: {:?}",
                    e, req_json
                );
            }
        }
    }

    Ok(recovered)
}

/// Recover incomplete user decrypt requests
async fn recover_user_decrypt_requests(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    repositories: &Arc<Repositories>,
) -> eyre::Result<usize> {
    let mut requests = repositories
        .user_decrypt
        .find_incomplete_requests()
        .await
        .map_err(|e| eyre::eyre!("Failed to query incomplete user decrypt requests: {}", e))?;

    requests.sort_by_key(|(_, _, status, _)| match status {
        ReqStatus::TxInFlight => 0,
        ReqStatus::Processing => 1,
        ReqStatus::Queued => 2,
        _ => 3,
    });

    let mut recovered = 0;

    for (int_job_id, req_json, status, _updated_at) in requests {
        if let Ok(request) = serde_json::from_value::<UserDecryptRequest>(req_json) {
            let job_id = JobId::from_sha256_hash(int_job_id.try_into().unwrap_or([0u8; 32]));
            let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

            let event_data = match status {
                ReqStatus::Queued => {
                    RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
                        decrypt_request: request,
                    })
                }
                ReqStatus::Processing | ReqStatus::TxInFlight => {
                    RelayerEventData::UserDecrypt(UserDecryptEventData::ReadinessCheckPassed {
                        decrypt_request: request,
                    })
                }
                _ => continue,
            };

            let event = RelayerEvent::new(job_id, api_version, event_data);
            if let Err(e) = orchestrator.dispatch_event(event).await {
                warn!("Failed to recover user decrypt request: {}", e);
            } else {
                recovered += 1;
            }
        }
    }

    Ok(recovered)
}

/// Recover incomplete input proof requests
async fn recover_input_proof_requests(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    repositories: &Arc<Repositories>,
) -> eyre::Result<usize> {
    let mut requests = repositories
        .input_proof
        .find_incomplete_requests()
        .await
        .map_err(|e| eyre::eyre!("Failed to query incomplete input proof requests: {}", e))?;

    requests.sort_by_key(|(_, _, status, _)| match status {
        ReqStatus::TxInFlight => 0,
        ReqStatus::Processing => 1,
        ReqStatus::Queued => 2,
        _ => 3,
    });

    let mut recovered = 0;

    for (int_request_id, req_json, _status, _updated_at) in requests {
        if let Ok(request) = serde_json::from_value::<InputProofRequest>(req_json) {
            let job_id = JobId::from_uuid_v7(int_request_id);
            let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

            let event_data = RelayerEventData::InputProof(InputProofEventData::ReqRcvdFromUser {
                input_proof_request: request,
            });

            let event = RelayerEvent::new(job_id, api_version, event_data);
            if let Err(e) = orchestrator.dispatch_event(event).await {
                warn!("Failed to recover input proof request: {}", e);
            } else {
                recovered += 1;
            }
        }
    }

    Ok(recovered)
}
