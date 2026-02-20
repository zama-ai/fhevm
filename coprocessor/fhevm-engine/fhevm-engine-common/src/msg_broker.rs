use lapin::message::Delivery;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use lapin::{Channel, Consumer};
use serde::de::DeserializeOwned;
use std::error::Error;
use tracing::error;

/// Initializes a RabbitMQ consumer channel
pub async fn create_recv_channel(
    uri: &str,
    node_name: &str,
    queue_name: &str,
    prefetch_count: u16,
) -> lapin::Result<Consumer> {
    let conn = Connection::connect(uri, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Set prefetch count for fair dispatch
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

    let consumer_tag = format!("{}_{}", node_name, queue_name);
    let consumer = channel
        .basic_consume(
            queue.name().as_str(),
            consumer_tag.as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(consumer)
}

/// Initializes a RabbitMQ publishing channel
pub async fn create_send_channel(uri: &str, queue_name: &str) -> lapin::Result<Channel> {
    let conn = Connection::connect(uri, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    let _ = channel.confirm_select(Default::default()).await?;

    // Ensure queue exists (safe if already declared)
    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(channel)
}

pub fn extract_delivery(
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

/// Attempts to decode a deserializable message and returning None if it fails.
///
/// On deserialization failure, the message is rejected without requeuing,
/// as it's likely malformed and will fail again.
pub async fn try_decode<T: DeserializeOwned>(delivery: &Delivery) -> Option<T> {
    let decoded: T = match postcard::from_bytes(&delivery.data) {
        Ok(p) => p,
        Err(e) => {
            error!({ error = ?e }, "Failed to deserialize partition, reject message");

            delivery
                .reject(BasicRejectOptions { requeue: false })
                .await
                .ok();
            return None;
        }
    };

    Some(decoded)
}
