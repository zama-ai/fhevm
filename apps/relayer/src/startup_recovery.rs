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
    metrics,
    orchestrator::Orchestrator,
    store::sql::{models::req_status_enum_model::ReqStatus, repositories::Repositories},
};
use std::sync::Arc;
use tracing::{error, info, warn};

/// Initialize metrics gauges from current database state.
/// Must be called before any operations that modify metrics.
async fn init_status_counts_from_db(repositories: &Arc<Repositories>) -> eyre::Result<()> {
    let counts = repositories
        .public_decrypt
        .count_by_status()
        .await
        .map_err(|e| eyre::eyre!("Failed to count public_decrypt by status: {}", e))?;
    for (status, count) in counts {
        metrics::set_req_status_count(metrics::RequestType::PublicDecrypt, status, count);
    }

    let counts = repositories
        .user_decrypt
        .count_by_status()
        .await
        .map_err(|e| eyre::eyre!("Failed to count user_decrypt by status: {}", e))?;
    for (status, count) in counts {
        metrics::set_req_status_count(metrics::RequestType::UserDecrypt, status, count);
    }

    let counts = repositories
        .input_proof
        .count_by_status()
        .await
        .map_err(|e| eyre::eyre!("Failed to count input_proof by status: {}", e))?;
    for (status, count) in counts {
        metrics::set_req_status_count(metrics::RequestType::InputProof, status, count);
    }

    info!("Initialized request status metrics from database");
    Ok(())
}

/// Reset all tx_in_flight requests to processing before recovery.
/// This ensures clean state transitions and proper idempotency checks.
async fn reset_tx_in_flight_requests(repositories: &Arc<Repositories>) -> eyre::Result<()> {
    let public_decrypt_count = repositories
        .public_decrypt
        .reset_tx_in_flight_to_processing()
        .await
        .map_err(|e| eyre::eyre!("Failed to reset public_decrypt: {}", e))?;

    if public_decrypt_count > 0 {
        info!(
            "Reset {} public_decrypt requests from tx_in_flight to processing",
            public_decrypt_count
        );
    }

    let user_decrypt_count = repositories
        .user_decrypt
        .reset_tx_in_flight_to_processing()
        .await
        .map_err(|e| eyre::eyre!("Failed to reset user_decrypt: {}", e))?;

    if user_decrypt_count > 0 {
        info!(
            "Reset {} user_decrypt requests from tx_in_flight to processing",
            user_decrypt_count
        );
    }

    let input_proof_count = repositories
        .input_proof
        .reset_tx_in_flight_to_processing()
        .await
        .map_err(|e| eyre::eyre!("Failed to reset input_proof: {}", e))?;

    if input_proof_count > 0 {
        info!(
            "Reset {} input_proof requests from tx_in_flight to processing",
            input_proof_count
        );
    }

    Ok(())
}

/// Recover incomplete requests by re-dispatching events based on their status.
pub async fn recover_incomplete_requests(
    orchestrator: &Arc<Orchestrator>,
    repositories: &Arc<Repositories>,
) -> eyre::Result<usize> {
    info!("Starting request recovery...");

    // Initialize metrics from DB before any operations that modify them
    init_status_counts_from_db(repositories).await?;

    // Reset tx_in_flight to processing
    reset_tx_in_flight_requests(repositories).await?;

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
    orchestrator: &Arc<Orchestrator>,
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
                let int_job_id_len = int_job_id.len();
                let job_id: JobId = match int_job_id.try_into() {
                    Ok(id) => id,
                    Err(_) => {
                        error!(
                            alert = true,
                            int_job_id_len,
                            "int_job_id has invalid length in public_decrypt recovery, expected 32 bytes, skipping"
                        );
                        continue;
                    }
                };
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
    orchestrator: &Arc<Orchestrator>,
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
        let request = match serde_json::from_value::<UserDecryptRequest>(req_json.clone()) {
            Ok(r) => r,
            Err(e) => {
                error!(
                    alert = true,
                    error = %e,
                    "Failed to deserialize UserDecryptRequest in recovery, skipping"
                );
                continue;
            }
        };
        let int_job_id_len = int_job_id.len();
        let job_id: JobId = match int_job_id.try_into() {
            Ok(id) => id,
            Err(_) => {
                error!(
                    alert = true,
                    int_job_id_len,
                    "int_job_id has invalid length in user_decrypt recovery, expected 32 bytes, skipping"
                );
                continue;
            }
        };
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

    Ok(recovered)
}

/// Recover incomplete input proof requests
async fn recover_input_proof_requests(
    orchestrator: &Arc<Orchestrator>,
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

    for (int_job_id, req_json, _status, _updated_at) in requests {
        let request = match serde_json::from_value::<InputProofRequest>(req_json.clone()) {
            Ok(r) => r,
            Err(e) => {
                error!(
                    alert = true,
                    error = %e,
                    "Failed to deserialize InputProofRequest in recovery, skipping"
                );
                continue;
            }
        };
        let int_job_id_len = int_job_id.len();
        let job_id: JobId = match int_job_id.try_into() {
            Ok(id) => id,
            Err(_) => {
                error!(
                    alert = true,
                    int_job_id_len,
                    "int_job_id has invalid length in input_proof recovery, expected 32 bytes, skipping"
                );
                continue;
            }
        };
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

    Ok(recovered)
}
