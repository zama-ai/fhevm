use crate::orchestrator::traits::{Event, PreDispatchHook};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tracing::info;

/// A hook that logs every event before it's dispatched
pub struct EventLoggingHook {
    name: String,
    log_prefix: String,
}

impl EventLoggingHook {
    pub fn new(log_prefix: String) -> Arc<Self> {
        Arc::new(Self {
            name: "logger".to_string(),
            log_prefix,
        })
    }
}

impl Display for EventLoggingHook {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[async_trait::async_trait]
impl<E: Event + Send + Sync + 'static> PreDispatchHook<E> for EventLoggingHook {
    async fn run(&self, event: E) {
        info!(
            event_name = %colorize_event_type(event.event_name()),
            request_id = %colorize_request_id(event.request_id()),
            "{}", self.log_prefix
        );
    }
}

fn colorize_event_type(event_type: impl Display) -> String {
    format!("\x1b[36m{}\x1b[0m", event_type) // Cyan for event type
}

fn colorize_request_id(request_id: impl Display) -> String {
    format!("\x1b[33m{}\x1b[0m", request_id) // Yellow for request ID
}
