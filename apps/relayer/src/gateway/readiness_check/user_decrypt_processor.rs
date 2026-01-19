use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            ApiCategory, ApiVersion, RelayerEvent, RelayerEventData, UserDecryptEventData,
            UserDecryptRequest,
        },
        job_id::JobId,
    },
    gateway::{
        arbitrum::bindings::Decryption,
        readiness_check::{
            readiness_checker::{ReadinessCheckError, ReadinessChecker},
            readiness_throttler::{ReadinessWorker, UserDecryptReadinessTask},
        },
    },
    orchestrator::{traits::EventDispatcher, Orchestrator, TokioEventDispatcher},
};
use std::sync::Arc;
use tracing::{error, info};

/// The Processor responsible for bridging the Readiness Throttler (Queue) and the ReadinessChecker (Executor).
pub struct UserDecryptReadinessProcessor;

impl UserDecryptReadinessProcessor {
    /// Spawns the background worker.
    ///
    /// Registers with the Orchestrator. The worker runs indefinitely, consuming tasks
    /// and executing the readiness check logic without touching the database.
    pub async fn orchestrator_spawn_task(
        throttler_worker: ReadinessWorker<UserDecryptReadinessTask>,
        readiness_checker: Arc<ReadinessChecker>,
        orchestrator: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    ) -> anyhow::Result<()> {
        let task_name = "user_decrypt_readiness_processor";

        let dispatcher = orchestrator.clone();

        let task_future = async move {
            info!("GatewayReadinessProcessor started.");

            // Run the consumer loop
            throttler_worker
                .run_consumer(move |task: UserDecryptReadinessTask| {
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
        task: UserDecryptReadinessTask,
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    ) {
        // Prepare handles
        let contract_pairs: Vec<_> = task
            .request
            .ct_handle_contract_pairs
            .iter()
            .map(Decryption::CtHandleContractPair::from)
            .collect();

        // 1. EXECUTE CHECK
        let result = checker
            .check_user_decryption_readiness(
                &task.job_id,
                task.request.user_address,
                contract_pairs,
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
                    RelayerEventData::UserDecrypt(UserDecryptEventData::ReadinessCheckPassed {
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
        decrypt_request: &UserDecryptRequest,
        job_id: JobId,
        error: EventProcessingError,
    ) {
        let event = RelayerEvent::new(
            job_id,
            ApiVersion {
                category: ApiCategory::PRODUCTION,
                number: 1,
            },
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReadinessCheckTimedOut {
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
        decrypt_request: &UserDecryptRequest,
        job_id: JobId,
        error: EventProcessingError,
    ) {
        let event = RelayerEvent::new(
            job_id,
            ApiVersion {
                category: ApiCategory::PRODUCTION,
                number: 1,
            },
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReadinessCheckFailed {
                decrypt_request: decrypt_request.clone(),
                error,
            }),
        );

        if let Err(e) = dispatcher.dispatch_event(event).await {
            error!(error = ?e, "CRITICAL: Failed to dispatch readiness failure event");
        }
    }
}
