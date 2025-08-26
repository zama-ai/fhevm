use crate::orchestrator::traits::{Event, PreDispatchHook};
use std::env;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::sync::OnceLock;
use tracing::info;

static NO_COLOR: OnceLock<bool> = OnceLock::new();

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

fn should_disable_color() -> bool {
    *NO_COLOR.get_or_init(|| env::var("NO_COLOR").is_ok_and(|v| !v.is_empty()))
}

fn colorize_event_type(event_type: impl Display) -> String {
    if should_disable_color() {
        format!("{event_type}")
    } else {
        format!("\x1b[36m{event_type}\x1b[0m") // Cyan for event type
    }
}

fn colorize_request_id(request_id: impl Display) -> String {
    if should_disable_color() {
        format!("{request_id}")
    } else {
        format!("\x1b[33m{request_id}\x1b[0m") // Yellow for request ID
    }
}
