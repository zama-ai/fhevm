use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::{
    EventDispatcher, EventHandler, HandlerRegistry, HookRegistry, PreDispatchHook,
};
use anyhow::Error;
use async_trait::async_trait;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

pub struct Orchestrator<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> {
    uuid_generator: Arc<UuidGenerator>,
    event_dispatcher: Arc<D>,
    pre_dispatch_hooks: Arc<RwLock<Vec<Arc<dyn PreDispatchHook<E>>>>>,

    _marker: std::marker::PhantomData<E>,
}

impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> Orchestrator<D, E> {
    pub fn new(event_dispatcher: Arc<D>, node_id: &[u8; 6]) -> Arc<Self> {
        Arc::new(Self {
            uuid_generator: Arc::new(UuidGenerator::new(
                node_id,
                DEFAULT_UUID_CONTEXT_INITIAL_VALUE,
            )),
            event_dispatcher,
            pre_dispatch_hooks: Arc::new(RwLock::new(Vec::new())),
            _marker: std::marker::PhantomData,
        })
    }

    pub fn new_request_id(&self) -> Uuid {
        self.uuid_generator.generate_id()
    }

    fn run_pre_dispatch_hooks(&self, event: E) {
        if let Ok(hooks) = self.pre_dispatch_hooks.read() {
            for hook in hooks.iter() {
                info!("Running pre-dispatch hook: {}", hook);
                hook.run(event.clone());
            }
        } else {
            panic!("Failed to acquire read lock on pre-dispatch hooks");
        }
    }
}

#[async_trait]
impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> EventDispatcher<E>
    for Orchestrator<D, E>
{
    async fn dispatch_event(&self, event: E) -> Result<(), Error> {
        self.run_pre_dispatch_hooks(event.clone());
        self.event_dispatcher.dispatch_event(event).await
    }
}

impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> HookRegistry<E> for Orchestrator<D, E> {
    fn register_pre_dispatch_hook(&self, hook: Arc<dyn PreDispatchHook<E>>) {
        if let Ok(mut hooks) = self.pre_dispatch_hooks.write() {
            hooks.push(hook);
        } else {
            panic!("Failed to acquire write lock on pre-dispatch hooks");
        }
    }
}

impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> HandlerRegistry<E>
    for Orchestrator<D, E>
{
    fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>) {
        self.event_dispatcher.register_handler(event_id, handler);
    }

    fn register_once_handler(
        &self,
        event_id: u8,
        request_id: Uuid,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.event_dispatcher
            .register_once_handler(event_id, request_id, handler);
    }
}

/// Randomly chosen value. See `UuidGenerator::context` for details.
const DEFAULT_UUID_CONTEXT_INITIAL_VALUE: u16 = 0;

struct UuidGenerator {
    /// Context holds a thread-safe, internally mutable counter that will be
    /// used to ensure uniqueness of generated UUIDs across threads, even if
    /// timestamps match by chance.
    /// Its initial value can be randomly chosen, say even to 0.
    /// It need not be same across different instances or processes of the application.
    context: Context,

    // Node ID uniquely identifies this node for UUID generation and should be
    // unique for each instance or process of the application.
    node_id: [u8; 6],
}

impl UuidGenerator {
    pub fn new(node_id: &[u8; 6], context_initial_value: u16) -> Self {
        Self {
            context: Context::new(context_initial_value),
            node_id: *node_id,
        }
    }

    pub fn generate_id(&self) -> Uuid {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let ts = Timestamp::from_unix(&self.context, now.as_secs(), now.subsec_nanos());
        Uuid::new_v1(ts, &self.node_id)
    }
}
