//! Re-dispatch of an incomplete request, and the status-count gauges read once at startup.
//!
//! Which event a row is re-dispatched as depends on how far it got:
//! - `queued`: ReqRcvdFromUser (includes readiness check)
//! - `processing`/`tx_in_flight`: ReadinessCheckPassed (skips readiness check)
//! - `receipt_received`: Not recovered (gateway listener handles automatically)
//!
//! Nothing here decides *which* rows to re-dispatch, and nothing here runs at startup any more
//! except [`init_status_counts_from_db`]. The sweep ([`crate::sweep`]) chooses the rows, claims
//! each one under this pod's epoch first, and calls the three `dispatch_recovered_*` builders
//! below. A startup pass that re-dispatched without claiming used to live here; it would now
//! race the sweep's own claim of the same rows, since the claim no longer waits out a staleness
//! window before taking an unowned or older-epoch row. A restart recovers by minting a higher
//! epoch instead, which makes every row the previous incarnation owned claimable on the first
//! tick after acquisition.

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

/// Build and dispatch the recovery event for one incomplete public decrypt row. Shared by
/// startup recovery and the step-6 sweep (`sweep::run_tick`), so there is one re-dispatch
/// mechanism for this request type rather than two. Returns whether an event was dispatched;
/// `false` covers both "skipped" (bad `int_job_id`, undeserializable `req`, or a status this
/// function does not recover) and "dispatch itself failed".
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
        ReqStatus::Processing | ReqStatus::TxInFlight => {
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

/// Build and dispatch the recovery event for one incomplete user decrypt row. Shared by
/// startup recovery and the step-6 sweep - see
/// [`dispatch_recovered_public_decrypt`] for why. Returns whether an event was dispatched.
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
        ReqStatus::Processing | ReqStatus::TxInFlight => {
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

/// Build and dispatch the recovery event for one incomplete input proof row. Shared by
/// startup recovery and the step-6 sweep - see
/// [`dispatch_recovered_public_decrypt`] for why. Returns whether an event was dispatched.
/// Unlike the decrypt flows, input proof recovery only ever re-emits `ReqRcvdFromUser`
/// regardless of `status` (matching the original recovery loop, which ignored status here
/// too) - a status this cannot recover from is not distinguished from a bad row.
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
