use crate::{MessageResult, Receiver, Sender};
use serde::de::DeserializeOwned;
use std::{error::Error, future::Future};

#[derive(Clone)]
pub struct RedisStreamReceiver<State> {
    state: State,
    queue_name: String,
}

impl<State> RedisStreamReceiver<State> {
    /// Creates a new RedisStreamReceiver instance and initializes the connection and consumer channel.
    pub async fn new(_uri: &str, queue_name: &str, state: State) -> Self {
        Self {
            queue_name: queue_name.to_string(),
            state,
        }
    }
}

impl<Message, State> Receiver<Message, State> for RedisStreamReceiver<State>
where
    Message: DeserializeOwned + Clone + Send + 'static,
    State: Clone + Send + 'static,
{
    type Error = Box<dyn Error + Send + Sync>;
    async fn recv_and_handle<Handler, Fut>(
        &mut self,
        mut _msg_handler_fn: Handler,
    ) -> Result<(), Self::Error>
    where
        Handler: FnMut(Message, Vec<u8>, State) -> Fut + Send,
        Fut: Future<Output = Result<MessageResult, Box<dyn Error + Send + Sync>>> + Send,
    {
        unimplemented!("RedisStreamReceiver is not implemented yet");
    }
}

#[derive(Clone)]
pub struct RedisStreamSender {}
impl RedisStreamSender {
    pub async fn new() -> Self {
        Self {}
    }
}

impl Sender<&[u8]> for RedisStreamSender {
    type Error = Box<dyn Error + Send + Sync>;
    async fn send(&self, payload: &[u8]) -> Result<(), Self::Error> {
        unimplemented!("RedisStreamSender is not implemented yet");
    }
}
