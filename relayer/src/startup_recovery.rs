//! Re-dispatch of an incomplete request, and the status-count gauges read once at startup.
//!
//! Which event a row is re-dispatched as depends on how far it got:
//! - `queued`: ReqRcvdFromUser (includes readiness check)
//! - `processing`: ReadinessCheckPassed (skips readiness check)
//! - `receipt_received`: Not recovered (gateway listener handles automatically)
//!
//! A row mid-send arrives as `processing` too: the claim rewrites `tx_in_flight` to `processing`
//! in the same `UPDATE` it returns rows from, so `tx_in_flight` never reaches this module.
//!
//! Nothing here decides *which* rows to re-dispatch, and [`init_status_counts_from_db`] is the
//! only thing here that runs at startup. The sweep ([`crate::sweep`]) chooses the rows, claims
//! each one under this pod's epoch, and calls the three `dispatch_recovered_*` builders below.

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
use anyhow::Context;
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info, warn};

/// Initialize metrics gauges from current database state.
///
/// A pure read, so it stays ungated and runs early in startup - before the HTTP server binds,
/// let alone anything that later gates dispatch on the HA lock - or the gauges sit
/// uninitialised for however long that gate takes to open. Must be called before any
/// operations that modify metrics.
pub async fn init_status_counts_from_db(repositories: &Arc<Repositories>) -> anyhow::Result<()> {
    let counts = repositories
        .public_decrypt
        .count_by_status()
        .await
        .context("Failed to count public_decrypt by status")?;
    for (status, count) in counts {
        metrics::set_req_status_count(metrics::RequestType::PublicDecrypt, status, count);
    }

    let counts = repositories
        .user_decrypt
        .count_by_status()
        .await
        .context("Failed to count user_decrypt by status")?;
    for (status, count) in counts {
        metrics::set_req_status_count(metrics::RequestType::UserDecrypt, status, count);
    }

    let counts = repositories
        .input_proof
        .count_by_status()
        .await
        .context("Failed to count input_proof by status")?;
    for (status, count) in counts {
        metrics::set_req_status_count(metrics::RequestType::InputProof, status, count);
    }

    info!("Initialized request status metrics from database");
    Ok(())
}

/// Build and dispatch the recovery event for one incomplete public decrypt row, on behalf of
/// the sweep (`sweep::run_tick`). Returns whether an event was dispatched; `false` means the
/// row was skipped - a bad `int_job_id`, an undeserializable `req`, or a status this function
/// does not recover.
pub(crate) async fn dispatch_recovered_public_decrypt(
    orchestrator: &Arc<Orchestrator>,
    int_job_id: Vec<u8>,
    req_json: Value,
    status: ReqStatus,
) -> bool {
    let request = match serde_json::from_value::<PublicDecryptRequest>(req_json.clone()) {
        Ok(r) => r,
        Err(e) => {
            warn!(
                "Failed to deserialize public decrypt request (skipping): {} - req_json: {:?}",
                e, req_json
            );
            return false;
        }
    };

    let int_job_id_len = int_job_id.len();
    let job_id: JobId = match int_job_id.try_into() {
        Ok(id) => id,
        Err(_) => {
            error!(
                alert = true,
                int_job_id_len,
                "int_job_id has invalid length in public_decrypt recovery, expected 32 bytes, skipping"
            );
            return false;
        }
    };
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    let event_data = match status {
        ReqStatus::Queued => {
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                decrypt_request: request,
            })
        }
        ReqStatus::Processing => {
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReadinessCheckPassed {
                decrypt_request: request,
            })
        }
        _ => return false,
    };

    let event = RelayerEvent::new(job_id, api_version, event_data);
    if let Err(e) = orchestrator.dispatch_event(event).await {
        warn!("Failed to recover public decrypt request: {}", e);
        false
    } else {
        true
    }
}

/// Build and dispatch the recovery event for one incomplete user decrypt row. Same contract
/// as [`dispatch_recovered_public_decrypt`].
pub(crate) async fn dispatch_recovered_user_decrypt(
    orchestrator: &Arc<Orchestrator>,
    int_job_id: Vec<u8>,
    req_json: Value,
    status: ReqStatus,
) -> bool {
    let request = match serde_json::from_value::<UserDecryptRequest>(req_json.clone()) {
        Ok(r) => r,
        Err(e) => {
            error!(
                alert = true,
                error = %e,
                "Failed to deserialize UserDecryptRequest in recovery, skipping"
            );
            return false;
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
            return false;
        }
    };
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    let event_data = match status {
        ReqStatus::Queued => RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
            decrypt_request: request,
        }),
        ReqStatus::Processing => {
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReadinessCheckPassed {
                decrypt_request: request,
            })
        }
        _ => return false,
    };

    let event = RelayerEvent::new(job_id, api_version, event_data);
    if let Err(e) = orchestrator.dispatch_event(event).await {
        warn!("Failed to recover user decrypt request: {}", e);
        false
    } else {
        true
    }
}

/// Build and dispatch the recovery event for one incomplete input proof row. Same contract
/// as [`dispatch_recovered_public_decrypt`]. Unlike the decrypt flows, input proof recovery
/// only ever re-emits `ReqRcvdFromUser` regardless of `status` - a status this cannot recover
/// from is not distinguished from a bad row.
pub(crate) async fn dispatch_recovered_input_proof(
    orchestrator: &Arc<Orchestrator>,
    int_job_id: Vec<u8>,
    req_json: Value,
) -> bool {
    let request = match serde_json::from_value::<InputProofRequest>(req_json.clone()) {
        Ok(r) => r,
        Err(e) => {
            error!(
                alert = true,
                error = %e,
                "Failed to deserialize InputProofRequest in recovery, skipping"
            );
            return false;
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
            return false;
        }
    };
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    let event_data = RelayerEventData::InputProof(InputProofEventData::ReqRcvdFromUser {
        input_proof_request: request,
    });

    let event = RelayerEvent::new(job_id, api_version, event_data);
    if let Err(e) = orchestrator.dispatch_event(event).await {
        warn!("Failed to recover input proof request: {}", e);
        false
    } else {
        true
    }
}
