use anyhow::{Result, anyhow};
use std::env;
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use std::time::Duration;

use broker::{Broker, Topic};
use listener_core::blockchain::evm::sem_evm_rpc_provider::SemEvmRpcProvider;
use listener_core::config::BrokerType;
use listener_core::config::config::Settings;
use listener_core::core::{
    Cleaner, CleanerHandler, EvmListener, FetchHandler, Filters, ReorgHandler, UnwatchHandler,
    WatchHandler,
};
use listener_core::store::repositories::Repositories;
use listener_core::store::{PgClient, run_migrations};
use primitives::routing;
use primitives::utils::chain_id_to_namespace;
use tracing::{error, info};

const DEFAULT_CONFIG_PATH: &str = "./config.yaml";

/// Parse command line arguments and return the config file path.
/// Uses --config <path> or defaults to ./config.yaml
fn parse_config_path() -> String {
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--config" {
            if i + 1 < args.len() {
                return args[i + 1].clone();
            } else {
                error!("--config requires a path argument");
                process::exit(1);
            }
        }
        i += 1;
    }

    DEFAULT_CONFIG_PATH.to_string()
}

/// Load settings from config file and environment variables.
/// Exits the process if configuration fails to load.
fn load_settings() -> Settings {
    let config_path = parse_config_path();
    info!("Loading configuration from: {}", config_path);

    match Settings::new(Some(&config_path)) {
        Ok(settings) => {
            info!("Configuration loaded: {:?}", settings);
            settings
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    }
}

fn checked_chain_id(chain_id: u64) -> Result<i64> {
    i64::try_from(chain_id)
        .map_err(|_| anyhow!("Configured chain_id exceeds the supported i64 database range"))
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt().with_target(false).init();

    // Load configuration
    let settings = load_settings();

    // ── Telemetry: Prometheus metrics endpoint ────────────────────────────
    if settings.telemetry.enabled {
        let metrics_config = telemetry::MetricsConfig {
            listen_addr: SocketAddr::from(([0, 0, 0, 0], settings.telemetry.metrics_port)),
        };
        if let Err(e) = telemetry::init_metrics(metrics_config) {
            error!("Failed to start metrics endpoint: {}", e);
            process::exit(1);
        }
        broker::metrics::describe_metrics();
    }

    // Initialize database connection
    let pg_client = match PgClient::new(&settings.database).await {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to initialize database connection: {}", e);
            process::exit(1);
        }
    };

    // Run database migrations
    run_migrations(&pg_client, settings.database.migration_max_attempts).await;

    let arc_pg_client = Arc::new(pg_client);

    let configured_chain_id = match checked_chain_id(settings.blockchain.chain_id) {
        Ok(chain_id) => chain_id,
        Err(err) => {
            error!(chain_id = settings.blockchain.chain_id, %err);
            process::exit(1);
        }
    };
    let repositories = Repositories::new(arc_pg_client, configured_chain_id);
    let provider = match SemEvmRpcProvider::new(
        settings.blockchain.rpc_url.clone(),
        settings.blockchain.strategy.max_parallel_requests,
    ) {
        Ok(provider) => provider,
        Err(e) => {
            error!("Could not instanciate the semaphore provider: {}", e);
            process::exit(1);
        }
    };

    // Declaring broker:
    let broker = match settings.broker.broker_type {
        BrokerType::Redis => {
            match Broker::redis_with_ensure_publish(
                &settings.broker.broker_url,
                settings.broker.ensure_publish,
            )
            .await
            {
                Ok(b) => b,
                Err(e) => {
                    error!("Failed to initialize Redis broker: {}", e);
                    process::exit(1);
                }
            }
        }
        BrokerType::Amqp => {
            match Broker::amqp(&settings.broker.broker_url)
                .with_ensure_publish(settings.broker.ensure_publish)
                .build()
                .await
            {
                Ok(b) => b,
                Err(e) => {
                    error!("Failed to initialize AMQP broker: {}", e);
                    process::exit(1);
                }
            }
        }
    };

    // checking matching chain_id with rpc chain_id.
    match provider.get_chain_id().await {
        Ok(chain_id) => {
            if settings.blockchain.chain_id == chain_id {
                info!("Chain id verified");
            } else {
                error!("Chain id doesn't match with rpc chain_id");
                process::exit(1);
            }
        }
        Err(e) => {
            error!("Couldn't call chain id from rpc at instantiation: {}", e);
            process::exit(1);
        }
    };

    let chain_id = &chain_id_to_namespace(settings.blockchain.chain_id);
    let publisher = match broker.publisher(chain_id).await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to create publisher: {}", e);
            process::exit(1);
        }
    };

    let event_publisher = match broker.publisher_unscoped().await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to create event publisher: {}", e);
            process::exit(1);
        }
    };

    let seed_publisher = publisher.clone();
    let cleaner_publisher = publisher.clone();

    let repositories_for_filters = repositories.clone();
    let repositories_for_cleaner = repositories.clone();

    let evm_listener = EvmListener::new(
        provider,
        repositories,
        publisher,
        broker.clone(),
        event_publisher,
        &settings.blockchain,
    );
    // Validate strategy (This function panics if there is a problem regarding the strategy).
    // NOTE: maybe send error, and propagate error here, then process::exit(1).
    evm_listener
        .validate_strategy_and_init_block(settings.blockchain.strategy.block_start)
        .await;

    let evm_listener = Arc::new(evm_listener);

    // let network = &settings.blockchain.network;

    // Here the routing key, is basically the same than the queue name (e.g group)
    let fetch_consumer = match broker
        .consumer(&Topic::new(routing::FETCH_NEW_BLOCKS).with_namespace(chain_id))
        // .group("fetch-new-blocks")
        .group(routing::FETCH_NEW_BLOCKS)
        .prefetch(1)
        .max_retries(5)
        .redis_claim_min_idle(settings.broker.claim_min_idle)
        .redis_claim_interval(1)
        .redis_block_ms(200)
        .circuit_breaker(
            settings.broker.circuit_breaker_threshold,
            Duration::from_secs(settings.broker.circuit_breaker_cooldown_secs),
        )
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build fetch consumer: {}", e);
            process::exit(1);
        }
    };

    let reorg_consumer = match broker
        .consumer(&Topic::new(routing::BACKTRACK_REORG).with_namespace(chain_id))
        .group(routing::BACKTRACK_REORG)
        .prefetch(1)
        .max_retries(5)
        .redis_claim_min_idle(settings.broker.claim_min_idle)
        .redis_claim_interval(1)
        .redis_block_ms(200)
        .circuit_breaker(
            settings.broker.circuit_breaker_threshold,
            Duration::from_secs(settings.broker.circuit_breaker_cooldown_secs),
        )
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build reorg consumer: {}", e);
            process::exit(1);
        }
    };

    let watch_consumer = match broker
        .consumer(&Topic::new(routing::WATCH).with_namespace(chain_id))
        .group(routing::WATCH)
        .prefetch(1)
        .max_retries(5)
        .redis_claim_min_idle(settings.broker.claim_min_idle)
        .redis_claim_interval(1)
        .redis_block_ms(200)
        .circuit_breaker(
            settings.broker.circuit_breaker_threshold,
            Duration::from_secs(settings.broker.circuit_breaker_cooldown_secs),
        )
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build watch consumer: {}", e);
            process::exit(1);
        }
    };

    let unwatch_consumer = match broker
        .consumer(&Topic::new(routing::UNWATCH).with_namespace(chain_id))
        .group(routing::UNWATCH)
        .prefetch(1)
        .max_retries(5)
        .redis_claim_min_idle(settings.broker.claim_min_idle)
        .redis_claim_interval(1)
        .redis_block_ms(200)
        .circuit_breaker(
            settings.broker.circuit_breaker_threshold,
            Duration::from_secs(settings.broker.circuit_breaker_cooldown_secs),
        )
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build unwatch consumer: {}", e);
            process::exit(1);
        }
    };

    let cleaner_consumer = match broker
        .consumer(&Topic::new(routing::CLEAN_BLOCKS).with_namespace(chain_id))
        .group(routing::CLEAN_BLOCKS)
        .prefetch(1)
        .max_retries(5)
        .redis_claim_min_idle(settings.blockchain.cleaner.cron_secs + 25)
        .redis_claim_interval(1)
        .redis_block_ms(200)
        .circuit_breaker(3, Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build cleaner consumer: {}", e);
            process::exit(1);
        }
    };

    // ── Define handlers ─────────────────────────────────────────────────
    let fetch_handler = FetchHandler::new(Arc::clone(&evm_listener));
    let reorg_handler = ReorgHandler::new(Arc::clone(&evm_listener));

    let filters = Filters::new(repositories_for_filters, settings.blockchain.chain_id);
    let filters = Arc::new(filters);

    let watch_handler = WatchHandler::new(Arc::clone(&filters));
    let unwatch_handler = UnwatchHandler::new(Arc::clone(&filters));

    let cleaner = Arc::new(Cleaner::new(
        repositories_for_cleaner.blocks,
        cleaner_publisher,
        &settings.blockchain.cleaner,
    ));
    let cleaner_handler = CleanerHandler::new(Arc::clone(&cleaner));

    // ── Ensure AMQP queues/bindings exist before checking depth ────────
    // Without this, AMQP silently drops the seed message because no queue
    // is bound to the exchange yet (queues are normally created by consumer.run()).
    if let Err(e) = fetch_consumer.ensure_topology().await {
        error!(error = %e, "Failed to set up fetch consumer topology");
        process::exit(1);
    }

    if let Err(e) = reorg_consumer.ensure_topology().await {
        error!(error = %e, "Failed to set up reorg consumer topology");
        process::exit(1);
    }

    if let Err(e) = watch_consumer.ensure_topology().await {
        error!(error = %e, "Failed to set up control watch consumer topology");
        process::exit(1);
    }

    if let Err(e) = unwatch_consumer.ensure_topology().await {
        error!(error = %e, "Failed to set up control unwatch consumer topology");
        process::exit(1);
    }

    // ── Seed the cursor loop if no pending work exists ─────────────────
    // TODO: replicate this to the consumer library for non auto startup.
    let fetch_topic = Topic::new(routing::FETCH_NEW_BLOCKS).with_namespace(chain_id);
    let reorg_topic = Topic::new(routing::BACKTRACK_REORG).with_namespace(chain_id);

    let should_seed = settings.blockchain.strategy.automatic_startup
        && match broker
            .is_empty(&fetch_topic, routing::FETCH_NEW_BLOCKS)
            .await
        {
            Ok(empty) => empty,
            Err(e) => {
                error!(error = %e, "Failed to check fetch queue depth");
                process::exit(1);
            }
        }
        && match broker
            .is_empty(&reorg_topic, routing::BACKTRACK_REORG)
            .await
        {
            Ok(empty) => empty,
            Err(e) => {
                error!(error = %e, "Failed to check reorg backtrack queue depth");
                process::exit(1);
            }
        };

    if should_seed {
        info!("Fetch queue empty — publishing seed to bootstrap cursor loop");
        if let Err(e) = seed_publisher
            .publish(routing::FETCH_NEW_BLOCKS, &serde_json::Value::Null)
            .await
        {
            error!(error = %e, "Failed to publish fetch seed");
            process::exit(1);
        }
    }

    // ── Ensure cleaner topology and seed (always, not gated by automatic_startup) ──
    if let Err(e) = cleaner_consumer.ensure_topology().await {
        error!(error = %e, "Failed to set up cleaner consumer topology");
        process::exit(1);
    }

    let cleaner_topic = Topic::new(routing::CLEAN_BLOCKS).with_namespace(chain_id);
    let cleaner_is_empty = match broker.is_empty(&cleaner_topic, routing::CLEAN_BLOCKS).await {
        Ok(empty) => empty,
        Err(e) => {
            error!(error = %e, "Failed to check cleaner queue depth");
            process::exit(1);
        }
    };

    if cleaner_is_empty && settings.blockchain.cleaner.active {
        info!("Cleaner queue empty — publishing seed to bootstrap cleaner loop");
        if let Err(e) = seed_publisher
            .publish(routing::CLEAN_BLOCKS, &serde_json::Value::Null)
            .await
        {
            error!(error = %e, "Failed to publish cleaner seed");
            process::exit(1);
        }
    }
    // Remove this, its only for testing purposes.
    // TODO: Remove, only for testing purposes.
    // It simulates consumer lib to test if we are properly revcieving the events.
    // let dyn_routing = format!("coprocessor.{}", routing::NEW_EVENT);
    // let test_consumer_lib = match broker
    //     .consumer(&Topic::new(dyn_routing.clone()).with_namespace(namespace::EMPTY_NAMESPACE))
    //     .group(dyn_routing.clone())
    //     .prefetch(20)
    //     .max_retries(5)
    //     .redis_claim_min_idle(10)
    //     .redis_claim_interval(1)
    //     .redis_block_ms(200)
    //     .circuit_breaker(3, Duration::from_secs(30))
    //     .build()
    // {
    //     Ok(c) => c,
    //     Err(e) => {
    //         error!("Failed to build test consumer consumer: {}", e);
    //         process::exit(1);
    //     }
    // };

    // let consumer_lib_handler_test =
    //     AsyncHandlerPayloadOnly::new(move |event: BlockPayload| async move {
    //         println!("Consumer recieved event: {}", event);
    //         Ok::<(), EvmListenerError>(())
    //     });

    // ── Run both consumers concurrently ─────────────────────────────────
    info!("Starting consumers");

    let (consumer_name, result) = tokio::select! {
        r = fetch_consumer.run(fetch_handler) => ("Fetch", r),
        r = reorg_consumer.run(reorg_handler) => ("Reorg", r),
        r = watch_consumer.run(watch_handler) => ("Watch", r),
        r = unwatch_consumer.run(unwatch_handler) => ("Unwatch", r),
        r = cleaner_consumer.run(cleaner_handler) => ("Cleaner", r),

        // TEST CONSUMER: TO REMOVE.
        // r = test_consumer_lib.run(consumer_lib_handler_test) => ("test copro lib consumer", r),
    };

    error!("{consumer_name} consumer exited: {result:?}");
    error!("Shutting down all consumers and exiting");
    process::exit(1);
}
