use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::{Dispatcher, HandleRegistry};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

pub struct Orchestrator<D: Dispatcher<E> + HandleRegistry<E>, E: Event> {
    /// Used for generation UUIDs to uniquely identify incoming requests.
    /// TODO: Document more details.
    pub uuid_generator: Arc<UuidGenerator>,

    pub event_dispatcher: Arc<D>,

    _marker: std::marker::PhantomData<E>,
}

impl<D: Dispatcher<E> + HandleRegistry<E>, E: Event> Orchestrator<D, E> {
    pub fn new(event_dispatcher: Arc<D>, node_id: &[u8]) -> Self {
        Self {
            uuid_generator: Arc::new(UuidGenerator::new(
                node_id,
                DEFAULT_UUID_CONTEXT_INITIAL_VALUE,
            )),
            event_dispatcher,
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct UuidGenerator {
    /// Context holds a thread-safe counter that will be used to ensure
    /// uniqueness of generated UUIDs across threads, even if timestamps match
    /// by chance.
    /// Its initial value can be randomly chosen, say even to 0.
    /// It need not be same across different instances or processes of the application.
    context: Context,

    // Node ID uniquely identifies this node for UUID generation and should be
    // unique for each instance or process of the application.
    node_id: Vec<u8>,
}

/// Randomly chosen value. See `UuidGenerator::context` for details.
pub const DEFAULT_UUID_CONTEXT_INITIAL_VALUE: u16 = 0;

impl UuidGenerator {
    pub fn new(node_id: &[u8], context_initial_value: u16) -> Self {
        Self {
            context: Context::new(context_initial_value),
            node_id: node_id.to_vec(),
        }
    }

    pub fn generate_id(&self) -> Uuid {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let ts = Timestamp::from_unix(&self.context, now.as_secs(), now.subsec_nanos());
        Uuid::new_v1(ts, &self.node_id).unwrap()
    }
}
