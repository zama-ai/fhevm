use crate::core::event::RelayerEvent;
use crate::orchestrator::health_checker::{HealthCheck, HealthChecker};
use crate::orchestrator::ids;
use crate::orchestrator::task_manager::TaskManager;
use crate::orchestrator::traits::{Event, EventHandler};
use crate::orchestrator::TokioEventDispatcher;
use anyhow::Error;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, RwLock};
use tokio_util::sync::CancellationToken;
use tracing::{error, instrument};
use uuid::Uuid;

pub struct Orchestrator {
    event_dispatcher: Arc<TokioEventDispatcher>,
    health_checker: Arc<RwLock<HealthChecker>>,
    task_manager: TaskManager,
}

impl Orchestrator {
    pub fn new(event_dispatcher: Arc<TokioEventDispatcher>) -> Arc<Self> {
        Arc::new(Self {
            event_dispatcher,
            health_checker: Arc::new(RwLock::new(HealthChecker::new())),
            task_manager: TaskManager::new(),
        })
    }

    pub fn new_internal_request_id(&self) -> Uuid {
        ids::new_internal_request_id()
    }

    pub fn new_ext_job_id(&self) -> Uuid {
        ids::new_external_reference_id()
    }

    /// Add a health check to the orchestrator
    pub fn add_health_check(&self, name: String, check: Arc<dyn HealthCheck>) {
        if let Ok(mut health_checker) = self.health_checker.write() {
            health_checker.add_health_check(name, check);
        } else {
            error!("Failed to acquire write lock on health checker");
        }
    }

    /// Check all registered health checks
    pub async fn check_all_health(&self) -> (bool, HashMap<String, String>) {
        // Clone the health checker to avoid holding the lock across await
        let health_checker = if let Ok(guard) = self.health_checker.read() {
            // Clone the internal HashMap to release the lock before awaiting
            let checks = guard.checks.clone();
            HealthChecker { checks }
        } else {
            error!("Failed to acquire read lock on health checker");
            let mut error_result = HashMap::new();
            error_result.insert("health_checker".to_string(), "error".to_string());
            return (false, error_result);
        };

        health_checker.check_all().await
    }

    /// Spawn a task and wait for it to be ready before continuing
    pub async fn spawn_task_and_wait_ready<F, R>(
        &self,
        name: &str,
        task_future: F,
        ready_future: R,
    ) -> anyhow::Result<()>
    where
        F: Future<Output = ()> + Send + 'static,
        R: Future<Output = anyhow::Result<()>>,
    {
        self.task_manager
            .spawn_task_and_wait_ready(name, task_future, ready_future)
            .await
    }

    /// Wait for shutdown signal and gracefully shutdown all tasks
    pub async fn run_until_shutdown(
        &self,
        shutdown_token: CancellationToken,
    ) -> anyhow::Result<()> {
        self.task_manager.run_until_shutdown(shutdown_token).await
    }

    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=?event.job_id()))]
    pub async fn dispatch_event(&self, event: RelayerEvent) -> Result<(), Error> {
        self.event_dispatcher.dispatch_event(event).await
    }

    pub fn register_handler(&self, event_ids: &[u8], handler: Arc<dyn EventHandler<RelayerEvent>>) {
        self.event_dispatcher.register_handler(event_ids, handler);
    }
}

#[cfg(test)]
mod tests {

    use crate::core::event::{
        ApiCategory, ApiVersion, PublicDecryptEventData, RelayerEvent, RelayerEventData,
    };
    use crate::orchestrator::traits::{Event, EventHandler};
    use crate::orchestrator::{Orchestrator, TokioEventDispatcher};
    use alloy::primitives::U256;
    use std::sync::Arc;

    struct SimpleEventHandler;

    #[async_trait::async_trait]
    impl EventHandler<RelayerEvent> for SimpleEventHandler {
        async fn handle_event(&self, event: RelayerEvent) {
            println!("Handling event: {:?}", event.event_name());
        }
    }

    #[tokio::test]
    async fn test_orchestrator() {
        let pubsub = Arc::new(TokioEventDispatcher::new());
        let orchestrator = Orchestrator::new(pubsub.clone());

        let _id = orchestrator.new_internal_request_id();

        let event = RelayerEvent::new(
            crate::core::job_id::INTERNAL_EVENT_JOB_ID,
            ApiVersion {
                category: ApiCategory::PRODUCTION,
                number: 1,
            },
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqSentToGw {
                gw_req_reference_id: U256::default(),
            }),
        );

        let handler = Arc::new(SimpleEventHandler);
        pubsub.register_handler(&[event.event_id()], handler);
        _ = orchestrator.dispatch_event(event).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
