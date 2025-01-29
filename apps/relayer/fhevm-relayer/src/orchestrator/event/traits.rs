use uuid::Uuid;

pub trait Event: Clone + Send + Sync {
    fn event_name(&self) -> &str;
    fn event_id(&self) -> u8;
    fn request_id(&self) -> Uuid;
}
