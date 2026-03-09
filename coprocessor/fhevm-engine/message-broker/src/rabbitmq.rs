use futures_util::stream::StreamExt;
use lapin::message::Delivery;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use lapin::{Channel, Consumer};
use serde::de::DeserializeOwned;
use std::error::Error;
use tracing::{error, info};

use crate::{MessageResult, Receiver, Sender};

#[derive(Clone)]
pub struct RabbitMQReceiver<State> {
    state: State,
    consumer: lapin::Consumer,
    queue_name: String,
}

impl<State> RabbitMQReceiver<State> {
    /// Creates a new RabbitMQReceiver instance and initializes the connection and consumer channel.
    pub async fn new(uri: &str, queue_name: &str, consumer_tag: &str, state: State) -> Self {
        let prefetch_count = 1u16;

        let consumer = Self::create_recv_channel(uri, queue_name, consumer_tag, prefetch_count)
            .await
            .expect("valid RabbitMQ configuration");

        Self {
            consumer,
            queue_name: queue_name.to_string(),
            state,
        }
    }

    fn extract_delivery(
        maybe_delivery: Option<Result<Delivery, lapin::Error>>,
        queue_name: &str,
    ) -> Result<Delivery, Box<dyn Error + Send + Sync>> {
        let result = match maybe_delivery {
            Some(res) => res,
            None => {
                return Err(format!("Channel closed unexpectedly: {}", queue_name).into());
            }
        };

        let delivery = result.map_err(|e| {
            Box::<dyn Error + Send + Sync>::from(format!(
                "Failed to receive message from {}: {}",
                queue_name, e
            ))
        })?;

        Ok(delivery)
    }

    /// Attempts to decode a deserializable message
    async fn try_decode<T: DeserializeOwned>(delivery: &Delivery) -> Option<T> {
        match postcard::from_bytes(&delivery.data) {
            Ok(decoded) => Some(decoded),
            Err(e) => {
                error!(error = ?e, "Failed to deserialize message");
                None
            }
        }
    }

    /// Initializes a RabbitMQ consumer channel
    async fn create_recv_channel(
        uri: &str,
        queue_name: &str,
        consumer_tag: &str,
        prefetch_count: u16,
    ) -> lapin::Result<Consumer> {
        let conn = Connection::connect(uri, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        channel
            .basic_qos(prefetch_count, BasicQosOptions::default())
            .await?;

        let queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let consumer = channel
            .basic_consume(
                queue.name().as_str(),
                consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(consumer)
    }
}

impl<Message, State> Receiver<Message, State> for RabbitMQReceiver<State>
where
    Message: DeserializeOwned + Clone + Send + 'static,
    State: Clone + Send + 'static,
{
    type Error = Box<dyn Error + Send + Sync>;

    async fn recv_and_handle<Handler, Fut>(
        &mut self,
        mut msg_handler_fn: Handler,
    ) -> Result<(), Self::Error>
    where
        Handler: FnMut(Message, Vec<u8>, State) -> Fut + Send,
        Fut: std::future::Future<Output = Result<MessageResult, Box<dyn Error + Send + Sync>>>
            + Send,
    {
        let res = self.consumer.next().await;

        let delivery = Self::extract_delivery(res, &self.queue_name)?;
        let raw_payload = delivery.data.clone();

        let Some(msg) = Self::try_decode::<Message>(&delivery).await else {
            error!("Failed to decode message, rejecting");
            delivery
                .reject(BasicRejectOptions { requeue: false })
                .await
                .ok();
            return Ok(());
        };

        let state = self.state.clone();
        let msg_res = msg_handler_fn(msg, raw_payload, state).await?;

        match msg_res {
            MessageResult::Ack => {
                delivery.ack(BasicAckOptions::default()).await.ok();
            }
            MessageResult::Nack(requeue, _) => {
                delivery.reject(BasicRejectOptions { requeue }).await.ok();
            }
            MessageResult::Reject => {
                delivery
                    .reject(BasicRejectOptions { requeue: false })
                    .await
                    .ok();
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct RabbitMQSender {
    channel: Channel,
    routing_key: String,
    exchange: String,
}

impl RabbitMQSender {
    pub async fn new(uri: &str, queue_name: &str, exchange: &str, routing_key: &str) -> Self {
        let channel = create_send_channel(uri, queue_name)
            .await
            .expect("Failed to create RabbitMQ send channel");

        Self {
            channel,
            routing_key: routing_key.to_string(),
            exchange: exchange.to_string(),
        }
    }
}

impl Sender<Vec<u8>> for RabbitMQSender {
    type Error = Box<dyn Error + Send + Sync>;

    async fn send(&self, payload: Vec<u8>) -> Result<(), Self::Error> {
        let confirm = self
            .channel
            .basic_publish(
                self.exchange.as_str(),
                self.routing_key.as_str(),
                BasicPublishOptions::default(),
                &payload,
                lapin::BasicProperties::default(),
            )
            .await?;

        let confirm = confirm.await?;

        info!(confirm = ?confirm, "Sent message");

        Ok(())
    }
}

/// Initializes a RabbitMQ publishing channel
pub async fn create_send_channel(uri: &str, queue_name: &str) -> lapin::Result<Channel> {
    let conn = Connection::connect(uri, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    channel.confirm_select(Default::default()).await?;

    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(channel)
}
