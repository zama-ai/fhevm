use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            ApiCategory, ApiVersion, PublicDecryptEventData, PublicDecryptRequest, RelayerEvent,
            RelayerEventData,
        },
        job_id::JobId,
    },
    gateway::readiness_check::{
        readiness_checker::{ReadinessCheckError, ReadinessChecker},
        readiness_throttler::{PublicDecryptReadinessTask, ReadinessWorker},
    },
    orchestrator::{traits::EventDispatcher, Orchestrator, TokioEventDispatcher},
};
use alloy::primitives::FixedBytes;
use std::sync::Arc;
use tracing::{error, info};

/// The Processor responsible for bridging the Readiness Throttler (Queue) and the ReadinessChecker (Executor).
pub struct PublicDecryptReadinessProcessor;

impl PublicDecryptReadinessProcessor {
    /// Spawns the background worker.
    ///
    /// Registers with the Orchestrator. The worker runs indefinitely, consuming tasks
    /// and executing the readiness check logic without touching the database.
    pub async fn orchestrator_spawn_task(
        throttler_worker: ReadinessWorker<PublicDecryptReadinessTask>,
        readiness_checker: Arc<ReadinessChecker>,
        orchestrator: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    ) -> anyhow::Result<()> {
        let task_name = "public_decrypt_readiness_processor";

        let dispatcher = orchestrator.clone();

        let task_future = async move {
            info!("GatewayReadinessProcessor started.");

            // Run the consumer loop
            throttler_worker
                .run_consumer(move |task: PublicDecryptReadinessTask| {
                    // Clone dependencies for the individual task execution
                    let checker = readiness_checker.clone();
                    Self::process_single_task(checker, task, dispatcher.clone())
                })
                .await;

            info!("GatewayReadinessProcessor stopped.");
        };

        let ready_future = async { Ok(()) };

        orchestrator
            .spawn_task_and_wait_ready(task_name, task_future, ready_future)
            .await?;

        Ok(())
    }

    /// Internal logic to process a single Readiness Check.
    ///
    /// 1. Calls ReadinessChecker.
    /// 2. On Success: Dispatches `ReadinessVerified` event (to trigger the Transaction flow).
    /// 3. On Failure: Dispatches `Failed` event (to notify user/DB).
    async fn process_single_task(
        checker: Arc<ReadinessChecker>,
        task: PublicDecryptReadinessTask,
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    ) {
        // Prepare handles
        let handles_fixed_bytes: Vec<FixedBytes<32>> = task
            .request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        // 1. EXECUTE CHECK
        let result = checker
            .check_public_decryption_readiness(
                &task.job_id,
                handles_fixed_bytes,
                task.request.extra_data.clone(),
            )
            .await;

        // 2. DISPATCH RESULT
        match result {
            // --- SUCCESS ---
            Ok(()) => {
                info!("Readiness check passed for {}", task.job_id);

                // Dispatch Success Event.
                // The Handler will catch this event to trigger the DB update + Transaction Push.
                let next_event_data =
                    RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReadinessCheckPassed {
                        decrypt_request: task.request,
                    });

                let next_event = RelayerEvent::new(
                    task.job_id,
                    ApiVersion {
                        category: ApiCategory::PRODUCTION,
                        number: 1,
                    },
                    next_event_data,
                );

                if let Err(e) = dispatcher.dispatch_event(next_event).await {
                    error!(error = ?e, "CRITICAL: Failed to dispatch readiness success event");
                }
            }

            // --- TIMEOUT ---
            Err(ReadinessCheckError::Timeout) => {
                error!(job_id = %task.job_id, "Readiness check timed out");

                // Map to EventProcessingError
                Self::dispatch_timeout(
                    &dispatcher,
                    &task.request,
                    task.job_id,
                    EventProcessingError::ReadinessCheckTimedOut,
                )
                .await;
            }

            // --- CONTRACT ERROR ---
            Err(ReadinessCheckError::ContractError(e)) => {
                error!(job_id = %task.job_id, error = ?e, "Readiness check contract error");

                Self::dispatch_failure(
                    &dispatcher,
                    &task.request,
                    task.job_id,
                    EventProcessingError::ContractCallFailed(e.to_string()),
                )
                .await;
            }
        }
    }

    /// Helper to dispatch timed_out events
    async fn dispatch_timeout(
        dispatcher: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        decrypt_request: &PublicDecryptRequest,
        job_id: JobId,
        error: EventProcessingError,
    ) {
        let event = RelayerEvent::new(
            job_id,
            ApiVersion {
                category: ApiCategory::PRODUCTION,
                number: 1,
            },
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReadinessCheckTimedOut {
                decrypt_request: decrypt_request.clone(),
                error,
            }),
        );

        if let Err(e) = dispatcher.dispatch_event(event).await {
            error!(error = ?e, "CRITICAL: Failed to dispatch readiness failure event");
        }
    }

    /// Helper to dispatch failure events
    async fn dispatch_failure(
        dispatcher: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        decrypt_request: &PublicDecryptRequest,
        job_id: JobId,
        error: EventProcessingError,
    ) {
        let event = RelayerEvent::new(
            job_id,
            ApiVersion {
                category: ApiCategory::PRODUCTION,
                number: 1,
            },
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReadinessCheckFailed {
                decrypt_request: decrypt_request.clone(),
                error,
            }),
        );

        if let Err(e) = dispatcher.dispatch_event(event).await {
            error!(error = ?e, "CRITICAL: Failed to dispatch readiness failure event");
        }
    }
}
