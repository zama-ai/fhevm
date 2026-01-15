use crate::core::job_id::JobId;
use crate::orchestrator::health_checker::{HealthCheck, HealthChecker};
use crate::orchestrator::ids;
use crate::orchestrator::task_manager::TaskManager;
use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::{
    EventDispatcher, EventHandler, HandlerRegistry, HookRegistry, PreDispatchHook,
};
use anyhow::Error;
use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, RwLock};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, instrument};
use uuid::Uuid;

pub struct Orchestrator<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> {
    event_dispatcher: Arc<D>,
    pre_dispatch_hooks: Arc<RwLock<Vec<Arc<dyn PreDispatchHook<E>>>>>,
    health_checker: Arc<RwLock<HealthChecker>>,
    task_manager: TaskManager,
    _marker: std::marker::PhantomData<E>,
}

impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> Orchestrator<D, E> {
    pub fn new(event_dispatcher: Arc<D>) -> Arc<Self> {
        Arc::new(Self {
            event_dispatcher,
            pre_dispatch_hooks: Arc::new(RwLock::new(Vec::new())),
            health_checker: Arc::new(RwLock::new(HealthChecker::new())),
            task_manager: TaskManager::new(),
            _marker: std::marker::PhantomData,
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

    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=%(event.job_id())))]
    async fn run_pre_dispatch_hooks_sequentially(&self, event: E) {
        // Acquire lock and prepare all hooks as Futures.
        let hooks: Vec<_> = if let Ok(hooks_guard) = self.pre_dispatch_hooks.read() {
            hooks_guard.iter().cloned().collect()
        } else {
            error!("Failed to acquire read lock on pre-dispatch hooks");
            return;
        };

        // Execute the futures sequentially.
        for hook in hooks {
            debug!("Running pre-dispatch hook: {}", hook);
            hook.run(event.clone()).await;
        }
    }
}

#[async_trait]
impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> EventDispatcher<E>
    for Orchestrator<D, E>
{
    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=%(event.job_id())))]
    async fn dispatch_event(&self, event: E) -> Result<(), Error> {
        self.run_pre_dispatch_hooks_sequentially(event.clone())
            .await;
        self.event_dispatcher.dispatch_event(event).await
    }
}
#[async_trait::async_trait]
impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> HookRegistry<E> for Orchestrator<D, E> {
    fn register_pre_dispatch_hook(&self, hook: Arc<dyn PreDispatchHook<E>>) {
        if let Ok(mut hooks) = self.pre_dispatch_hooks.write() {
            hooks.push(hook);
        } else {
            error!("Failed to acquire write lock on pre-dispatch hooks");
        }
    }
}

impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> HandlerRegistry<E>
    for Orchestrator<D, E>
{
    fn register_handler(&self, event_ids: &[u8], handler: Arc<dyn EventHandler<E>>) {
        self.event_dispatcher.register_handler(event_ids, handler);
    }

    fn register_once_handler(
        &self,
        event_id: u8,
        job_id: JobId,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.event_dispatcher
            .register_once_handler(event_id, job_id, handler);
    }

    fn unregister_once_handler(&self, event_id: u8, job_id: JobId) {
        self.event_dispatcher
            .unregister_once_handler(event_id, job_id);
    }
}

#[cfg(test)]
mod tests {

    use crate::core::event::{
        ApiCategory, ApiVersion, PublicDecryptEventData, RelayerEvent, RelayerEventData,
    };
    use crate::core::job_id::JobId;
    use crate::orchestrator::traits::{Event, EventDispatcher, EventHandler, HandlerRegistry};
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
        let pubsub = Arc::new(TokioEventDispatcher::<RelayerEvent>::new());
        let orchestrator = Orchestrator::new(pubsub.clone());

        let id = orchestrator.new_internal_request_id();

        let event = RelayerEvent::new(
            JobId::from_uuid_v7(id),
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
