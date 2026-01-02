use crate::{
    core::event::{
        ApiCategory, ApiVersion, InputProofEventData, PublicDecryptEventData, RelayerEvent,
        RelayerEventData, UserDecryptEventData,
    },
    gateway::arbitrum::transaction::{
        helper::{TransactionHelper, TransactionType},
        throttler::{GatewayTxTask, MemoryThrottler},
    },
    orchestrator::{traits::EventDispatcher, Orchestrator, TokioEventDispatcher},
};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

/// The Processor is responsible for bridging the throttler (Queue) and the TransactionHelper (Executor).
pub struct GatewayTxProcessor;

impl GatewayTxProcessor {
    /// Spawns the background worker with a Supervisor Loop.
    ///
    /// This function registers the task with the Orchestrator.
    /// 1. It runs `run_consumer` which has internal self-healing for channel/logic errors.
    /// 2. If `run_consumer` ever exits (e.g. panic unwinding or logic bug), the outer loop restarts it.
    /// 3. It only stops when the Orchestrator initiates a shutdown.
    pub async fn orchestrator_spawn_task(
        throttler: Arc<MemoryThrottler<GatewayTxTask>>,
        tx_helper: Arc<TransactionHelper>,
        orchestrator: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    ) -> anyhow::Result<()> {
        let task_name = "gateway_tx_processor";

        // Clone Arcs for the long-running task
        let throttler_runner = throttler.clone();
        let helper_runner = tx_helper.clone();
        let orchestrator_runner = orchestrator.clone();

        // Define the Supervisor Task
        let task_future = async move {
            info!("GatewayTxProcessor supervisor started.");

            // SUPERVISOR LOOP
            // This loop ensures that if run_consumer EVER returns, we restart it.
            loop {
                info!("Starting GatewayTxProcessor consumer loop...");

                // Clone inner refs for the consumer closure
                let mp = throttler_runner.clone();
                let helper = helper_runner.clone();
                let dispatcher = orchestrator_runner.clone();

                // Run the consumer.
                // This function blocks indefinitely unless a critical error occurs.
                mp.run_consumer(move |task: GatewayTxTask| {
                    let helper = helper.clone();
                    let dispatcher = dispatcher.clone();

                    async move {
                        Self::process_single_task(helper, task, dispatcher).await;
                    }
                })
                .await;

                // If we reach this line, the consumer exited. This is unexpected.
                error!("CRITICAL: GatewayTxProcessor consumer loop exited unexpectedly! Restarting in 5 seconds...");

                // Prevent CPU spin loop in case of instant crash loop
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        };

        // Define Readiness (Ready immediately)
        let ready_future = async { Ok(()) };

        // Register with Orchestrator
        // This ensures the task is tracked and killed properly on shutdown.
        orchestrator
            .spawn_task_and_wait_ready(task_name, task_future, ready_future)
            .await?;

        info!("Gateway Transaction Processor spawned and registered.");
        Ok(())
    }

    /// Internal logic to process a single task.
    async fn process_single_task(
        helper: Arc<TransactionHelper>,
        task: GatewayTxTask,
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    ) {
        // declare dispatcher
        let arc_dispatcher = Arc::clone(&dispatcher);

        // Dereference hook
        let hook_ref = &*task.hook.0;

        // Adapt Calldata
        let calldata_bytes = task.calldata.clone();
        let calldata_fn = move || Ok(calldata_bytes.clone());

        // Execute
        let result = helper
            .send_raw_transaction_sync(
                task.transaction_type,
                task.job_id,
                hook_ref,
                task.target,
                calldata_fn,
            )
            .await;

        // Error dispatcher.
        if let Err(e) = result {
            error!(
                job_id = %task.job_id,
                error = ?e,
                "GatewayTxProcessor: Failed to submit transaction"
            );

            let next_event_data: RelayerEventData = match task.transaction_type {
                TransactionType::InputRequest => {
                    RelayerEventData::InputProof(InputProofEventData::Failed { error: e })
                }
                TransactionType::PublicDecryptRequest => {
                    RelayerEventData::PublicDecrypt(PublicDecryptEventData::Failed { error: e })
                }
                TransactionType::UserDecryptRequest => {
                    RelayerEventData::UserDecrypt(UserDecryptEventData::Failed { error: e })
                }
            };

            let next_event = RelayerEvent::new(
                task.job_id,
                ApiVersion {
                    category: ApiCategory::PRODUCTION,
                    number: 1,
                },
                next_event_data,
            );

            if let Err(e) = arc_dispatcher.dispatch_event(next_event).await {
                error!(?e, "CRITICAL: Failed to dispatch processor response event");
            } else {
                info!(
                    "Processor event response successfully sent for {}",
                    task.job_id
                );
            }
        }
    }
}
