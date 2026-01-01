use crate::gateway::arbitrum::transaction::{
    helper::TransactionHelper,
    pool::{GatewayTask, Mempool},
};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

/// The Processor is responsible for bridging the Mempool (Queue) and the TransactionHelper (Executor).
pub struct GatewayTxProcessor;

impl GatewayTxProcessor {
    /// Spawns the background worker.
    /// Includes a "Supervisor Loop" to ensure the task persists indefinitely.
    pub fn spawn(
        mempool: Arc<Mempool<GatewayTask>>,
        tx_helper: Arc<TransactionHelper>,
    ) -> tokio::task::JoinHandle<()> {
        info!("Spawning Gateway Transaction Processor (Supervisor Mode)...");

        tokio::spawn(async move {
            // SUPERVISOR LOOP
            // This loop ensures that if run_consumer EVER returns (which it shouldn't),
            // we log it, wait a bit, and restart it immediately.
            loop {
                info!("Starting GatewayTxProcessor consumer loop...");

                // We clone the Arcs here to pass them into the consumer closure
                // The originals stay alive in this outer loop scope.
                let mp_runner = mempool.clone();
                let helper_runner = tx_helper.clone();

                // Run the consumer.
                // This function contains its own recovery logic, so it should technically run forever.
                // If it returns, it means we hit a logic state that broke the internal loop.
                mp_runner
                    .run_consumer(move |task: GatewayTask| {
                        let helper = helper_runner.clone();

                        async move {
                            Self::process_single_task(helper, task).await;
                        }
                    })
                    .await;

                // If we reach this line, something unexpected happened.
                error!("CRITICAL: GatewayTxProcessor consumer loop exited unexpectedly! Restarting in 5 seconds...");

                // Prevent a tight loop CPU spike if it keeps crashing instantly
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        })
    }

    /// Internal logic to process a single task.
    async fn process_single_task(helper: Arc<TransactionHelper>, task: GatewayTask) {
        // Dereference hook
        let hook_ref = &*task.hook.0;

        // Adapt Calldata
        let calldata_bytes = task.calldata.clone();
        let calldata_fn = move || Ok(calldata_bytes.clone());

        // Execute
        let result = helper
            .send_raw_transaction_sync(
                task.transaction_type,
                task.job_id.clone(),
                hook_ref,
                task.target,
                calldata_fn,
            )
            .await;

        // TODO: Change error manager here, we need to dispatch error properly
        // Log Infrastructure Errors
        if let Err(e) = result {
            error!(
                job_id = %task.job_id,
                error = ?e,
                "GatewayTxProcessor: Failed to submit transaction"
            );
        }
    }
}
