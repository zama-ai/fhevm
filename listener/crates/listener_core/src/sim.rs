//! In-process simulation of an external consumer subscribing to
//! `coprocessor.new-event` and `coprocessor.catchup-event`.
//!
//! Strictly for flow testing. Not part of production wiring — `main.rs`
//! invokes [`run`] only when local end-to-end testing is desired, and can
//! comment the call out to disable the simulation entirely.

use std::time::Duration;

use alloy::primitives::Address;
use broker::{AsyncHandlerPayloadOnly, Broker, BrokerError, Publisher, Topic};
use primitives::event::{BlockPayload, FilterCommand};
use primitives::{namespace, routing};
use tracing::{error, info};

use crate::core::EvmListenerError;

/// Tether contract address used as the seed `WATCH` filter for the simulation.
const TETHER_CONTRACT: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

/// Build the two simulated consumers (`coprocessor.new-event` and
/// `coprocessor.catchup-event`), publish a hardcoded Tether `WATCH` filter,
/// and run both consumers concurrently.
///
/// Returns the same `(name, result)` shape used by the production arms of
/// the top-level `tokio::select!` in `main.rs`, so the call site stays a
/// single select branch.
pub async fn run(
    broker: Broker,
    publisher: Publisher,
) -> (&'static str, Result<(), BrokerError>) {
    let new_event_routing = format!("coprocessor.{}", routing::NEW_EVENT);
    let test_consumer_lib = match broker
        .consumer(&Topic::new(new_event_routing.clone()).with_namespace(namespace::EMPTY_NAMESPACE))
        .group(new_event_routing.clone())
        .prefetch(20)
        .max_retries(5)
        .redis_claim_min_idle(10)
        .redis_claim_interval(1)
        .redis_block_ms(200)
        .circuit_breaker(3, Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build test consumer: {}", e);
            return ("sim setup", Err(e));
        }
    };

    let consumer_lib_handler_test =
        AsyncHandlerPayloadOnly::new(move |event: BlockPayload| async move {
            info!("GETTING EVENT FROM BLOCK: {}", event.block_number);
            Ok::<(), EvmListenerError>(())
        });

    let watch_tether_contract: FilterCommand = FilterCommand {
        consumer_id: "coprocessor".to_string(),
        from: None,
        log_address: Some(
            TETHER_CONTRACT
                .parse::<Address>()
                .expect("Invalid address"),
        ),
        to: None,
    };

    if let Err(e) = publisher
        .publish(routing::WATCH, &watch_tether_contract)
        .await
    {
        error!(error = %e, "Failed to publish simulation WATCH filter");
        return ("sim setup", Err(e));
    }

    let catchup_event_routing = format!("coprocessor.{}", routing::CATCHUP_EVENT);
    let test_consumer_lib_catchup = match broker
        .consumer(
            &Topic::new(catchup_event_routing.clone())
                .with_namespace(namespace::EMPTY_NAMESPACE),
        )
        .group(catchup_event_routing.clone())
        .prefetch(20)
        .max_retries(5)
        .redis_claim_min_idle(10)
        .redis_claim_interval(1)
        .redis_block_ms(200)
        .circuit_breaker(3, Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build test catchup consumer: {}", e);
            return ("sim setup", Err(e));
        }
    };

    let consumer_lib_handler_test_catchup =
        AsyncHandlerPayloadOnly::new(move |event: BlockPayload| async move {
            info!("GETTING EVENT FROM CATCHUP BLOCK: {}", event.block_number);
            for tx in event.transactions {
                info!("transaction hash: {}", tx.hash);
            }
            Ok::<(), EvmListenerError>(())
        });

    tokio::select! {
        r = test_consumer_lib.run(consumer_lib_handler_test) => ("test copro lib consumer", r),
        r = test_consumer_lib_catchup.run(consumer_lib_handler_test_catchup) => ("test copro lib consumer catchup", r),
    }
}
