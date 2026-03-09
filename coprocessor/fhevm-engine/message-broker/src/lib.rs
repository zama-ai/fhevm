use std::{error::Error, future::Future};

#[cfg(feature = "rabbitmq")]
pub mod rabbitmq;

#[cfg(feature = "redis_stream")]
pub mod redis_stream;

/// Represents the result of processing a message
/// This abstracts over the different ack strategies of various message brokers
pub enum MessageResult {
    /// Indicates that the message was processed successfully and can be acknowledged
    Ack,
    /// Indicates that the message processing failed, but it may succeed on retry
    Nack(bool /* requeue */, u32 /* retry_count */),
    /// Indicates that the message was malformed and cannot be processed,
    /// so it should be rejected without requeuing
    Reject,
}

/// A generic sender trait that abstracts over different message brokers
pub trait Sender<Payload>: Clone + Send + Sync + 'static {
    type Error: std::fmt::Debug;
    fn send(&self, payload: Payload) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

/// A generic receiver trait that abstracts over different message brokers
pub trait Receiver<Message, State> {
    type Error: Send + Sync + std::fmt::Debug + 'static;

    /// Receives a message and processes it using the provided handler
    fn recv_and_handle<Handler, Fut>(
        &mut self,
        msg_handler_fn: Handler,
    ) -> impl Future<Output = Result<(), Self::Error>>
    where
        Handler: FnMut(Message, Vec<u8>, State) -> Fut + Send,
        Fut: Future<Output = Result<MessageResult, Box<dyn Error + Send + Sync>>> + Send;
}
