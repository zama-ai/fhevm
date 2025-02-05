use tracing::info;

use crate::handlers_ethereum::handle_event;
use alloy::rpc::types::Log;
use futures_util::StreamExt;

pub async fn event_listener(mut subscription: alloy::pubsub::SubscriptionStream<Log>) {
    loop {
        let ethereum_events_listener = tokio::select! {
            event = subscription.next() => match event {
                Some(event) => {
                    handle_event(event).unwrap();
                    // info!(?event, "Received event");
                }
                None => {
                    info!("Subscription stream ended");
                    break;
                }
            },
            _ = tokio::signal::ctrl_c() => {
                info!("Received ctrl + c signal, stopping...");
                break;
            }
        };
        ethereum_events_listener
    }
}
