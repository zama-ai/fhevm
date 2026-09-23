use crate::{
    core::event::{
        ApiCategory, ApiVersion, InputProofEventData, PublicDecryptEventData, RelayerEvent,
        RelayerEventData, UserDecryptEventData,
    },
    gateway::arbitrum::transaction::{
        helper::{TransactionHelper, TransactionType},
        tx_throttler::{GatewayTxTask, TxThrottlingWorker},
    },
    logging::WorkerStep,
    orchestrator::{DispatchGate, Orchestrator},
};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// The Processor is responsible for bridging the throttler (Queue) and the TransactionHelper (Executor).
pub struct GatewayTxProcessor;

impl GatewayTxProcessor {
    /// Spawns the background worker.
    ///
    /// It registers with the Orchestrator so it starts/stops cleanly with the application.
    /// It delegates the infinite loop logic to `throttler.run_consumer`.
    ///
    /// Three of these run under one orchestrator, so `task_name` must be distinct per
    /// instance or the drain cannot say which one failed to stop.
    pub async fn orchestrator_spawn_task(
        task_name: &str,
        throttler_worker: TxThrottlingWorker<GatewayTxTask>,
        tx_helper: Arc<TransactionHelper>,
        orchestrator: Arc<Orchestrator>,
        gate: DispatchGate,
        shutdown: CancellationToken,
    ) -> anyhow::Result<()> {
        // Prepare Arcs for the background task
        let dispatcher = orchestrator.clone();
        let worker_name = task_name.to_string();

        // The Task Future
        let task_future = async move {
            info!(
                step = %WorkerStep::WorkerStarted,
                worker = %worker_name,
                "Worker started"
            );

            throttler_worker
                .run_consumer(gate, shutdown, move |task: GatewayTxTask| {
                    // Call the async function directly.
                    // It returns the Future that the consumer expects.
                    Self::process_single_task(tx_helper.clone(), task, dispatcher.clone())
                })
                .await;

            info!(
                step = %WorkerStep::WorkerStopped,
                worker = %worker_name,
                "Worker stopped"
            );
        };

        // Define Readiness (Ready immediately)
        let ready_future = async { Ok(()) };

        // Register with Orchestrator
        orchestrator
            .spawn_task_and_wait_ready(task_name, task_future, ready_future)
            .await?;

        Ok(())
    }

    /// Internal logic to process a single task.
    ///
    /// If this function fails (returns Err), it handles the error by dispatching a failure event.
    /// It does NOT crash the main worker loop.
    async fn process_single_task(
        helper: Arc<TransactionHelper>,
        task: GatewayTxTask,
        dispatcher: Arc<Orchestrator>,
    ) {
        // Dereference hook
        let hook_ref = task.hook.as_inner();

        // Adapt Calldata for the helper closure
        let calldata_bytes = task.calldata.clone();

        // Execute Transaction
        // If this fails, it returns Err. It does NOT panic.
        let result = helper
            .send_raw_transaction_sync(
                task.transaction_type,
                task.job_id,
                hook_ref,
                task.target,
                calldata_bytes,
            )
            .await;

        // Handle Errors (Dispatch Event)
        if let Err(e) = result {
            error!(
                job_id = %task.job_id,
                error = ?e,
                "GatewayTxProcessor: Failed to submit transaction"
            );

            // Construct the failure event data based on transaction type
            let next_event_data: RelayerEventData = match task.transaction_type {
                TransactionType::InputRequest => {
                    RelayerEventData::InputProof(InputProofEventData::InternalFailure { error: e })
                }
                TransactionType::PublicDecryptRequest => {
                    RelayerEventData::PublicDecrypt(PublicDecryptEventData::InternalFailure {
                        error: e,
                    })
                }
                TransactionType::UserDecryptRequest => {
                    RelayerEventData::UserDecrypt(UserDecryptEventData::InternalFailure {
                        error: e,
                    })
                }
            };

            // Create the new event
            // NOTE: change api version here with gateway task.
            let next_event = RelayerEvent::new(
                task.job_id,
                ApiVersion {
                    category: ApiCategory::PRODUCTION,
                    number: 1,
                },
                next_event_data,
            );

            // Dispatch
            if let Err(dispatch_err) = dispatcher.dispatch_event(next_event).await {
                error!(error = ?dispatch_err, "CRITICAL: Failed to dispatch processor failure event");
            } else {
                info!(
                    "Processor event failure response successfully sent for {:?}",
                    task.job_id
                );
            }
        }
    }
}
