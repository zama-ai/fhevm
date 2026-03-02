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
            error_redact::redact_alloy_error,
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
    /// 1. Checks host chain ACL permissions.
    /// 2. Calls GwCiphertextChecker (via ReadinessChecker).
    /// 3. On Success: Dispatches `ReadinessCheckPassed` event.
    /// 4. On Failure: Dispatches appropriate failure event.
    async fn process_single_task(
        checker: Arc<ReadinessChecker>,
        task: UserDecryptReadinessTask,
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    ) {
        // 1. HOST ACL CHECK
        if let Err(acl_err) = checker
            .check_host_acl_user_decrypt(&task.job_id, &task.request)
            .await
        {
            error!(job_id = %task.job_id, error = ?acl_err, "Host ACL check failed");
            Self::dispatch_failure(
                &dispatcher,
                &task.request,
                task.job_id,
                EventProcessingError::from(acl_err),
            )
            .await;
            return;
        }

        // 2. GATEWAY CIPHERTEXT CHECK
        let contract_pairs: Vec<_> = task
            .request
            .ct_handle_contract_pairs
            .iter()
            .map(Decryption::CtHandleContractPair::from)
            .collect();

        let result = checker
            .check_user_decryption_readiness(
                &task.job_id,
                contract_pairs,
                task.request.extra_data.clone(),
            )
            .await;

        // 3. DISPATCH RESULT
        match result {
            Ok(()) => {
                info!("Readiness check passed for {}", task.job_id);

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

            Err(ReadinessCheckError::GwTimeout) => {
                error!(job_id = %task.job_id, "Readiness check timed out");

                Self::dispatch_timeout(
                    &dispatcher,
                    &task.request,
                    task.job_id,
                    EventProcessingError::ReadinessCheckTimedOut,
                )
                .await;
            }

            Err(ReadinessCheckError::GwContractError(e)) => {
                error!(job_id = %task.job_id, error = ?e, "Readiness check contract error");

                Self::dispatch_failure(
                    &dispatcher,
                    &task.request,
                    task.job_id,
                    EventProcessingError::ContractCallFailed(redact_alloy_error(&e)),
                )
                .await;
            }

            Err(e @ ReadinessCheckError::NotAllowedOnHostAcl(_))
            | Err(e @ ReadinessCheckError::HostAclFailed(_)) => {
                error!(job_id = %task.job_id, error = ?e, "Unexpected ACL error in ciphertext check path");
                Self::dispatch_failure(
                    &dispatcher,
                    &task.request,
                    task.job_id,
                    EventProcessingError::from(e),
                )
                .await;
            }
        }
    }

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
