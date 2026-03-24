use crate::core::job_id::JobId;
use async_trait::async_trait;

pub trait Event: Clone + Send + Sync + 'static {
    fn event_name(&self) -> &str;
    fn event_id(&self) -> u8;
    fn job_id(&self) -> JobId;
    fn timestamp(&self) -> u64;
}

#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    async fn handle_event(&self, event: E);
}
