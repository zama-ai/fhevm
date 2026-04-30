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
    CatchupHandler, Cleaner, CleanerHandler, EvmListener, FetchHandler, Filters, ReorgHandler,
    UnwatchHandler, WatchHandler,
};
use listener_core::logging;
use listener_core::store::repositories::Repositories;
use listener_core::store::{FlowLock, PgClient, run_migrations};
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
                eprintln!("FATAL: --config requires a path argument");
                process::exit(1);
            }
        }
        i += 1;
    }

    DEFAULT_CONFIG_PATH.to_string()
}

/// Load settings from config file and environment variables.
/// Uses `eprintln!` because tracing is not yet initialized at this point.
fn load_settings() -> Settings {
    let config_path = parse_config_path();

    match Settings::new(Some(&config_path)) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("FATAL: Failed to load configuration from {config_path}: {e}");
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
    // Load configuration first (before tracing — uses eprintln for errors)
    let settings = load_settings();

    // Initialize structured logging from config
    let _root_guard = logging::init_logging(
        &settings.log,
        &settings.name,
        &settings.blockchain.network,
        settings.blockchain.chain_id,
    );

    info!("Configuration loaded: {:?}", settings);

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
        listener_core::metrics::describe_metrics();
        listener_core::metrics::init_gauges(settings.blockchain.chain_id);
        listener_core::metrics::init_counters(settings.blockchain.chain_id);
    }

    // Initialize database connection
    let pg_client = if settings.database.use_iam_auth() {
        #[cfg(feature = "iam-auth")]
        {
            let cancel = tokio_util::sync::CancellationToken::new();
            let pool = listener_core::store::connect_iam(&settings.database, cancel)
                .await
                .unwrap_or_else(|e| {
                    error!("Failed to initialize IAM database connection: {e}");
                    process::exit(1);
                });
            PgClient::from_pool(pool)
        }
        #[cfg(not(feature = "iam-auth"))]
        {
            // validate() already catches this, but belt-and-suspenders
            error!("IAM auth requested but iam-auth feature is not enabled");
            process::exit(1);
        }
    } else {
        match PgClient::new(&settings.database).await {
            Ok(client) => client,
            Err(e) => {
                error!("Failed to initialize database connection: {}", e);
                process::exit(1);
            }
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
    let repositories = Repositories::new(Arc::clone(&arc_pg_client), configured_chain_id);
    let provider = match SemEvmRpcProvider::new(
        settings.blockchain.rpc_url.clone(),
        settings.blockchain.strategy.max_parallel_requests,
    ) {
        Ok(provider) => provider,
        Err(e) => {
            error!("Could not instantiate the semaphore provider: {}", e);
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
    let handler_publisher = publisher;

    let repositories_for_filters = repositories.clone();
    let repositories_for_cleaner = repositories.clone();

    let evm_listener = EvmListener::new(
        provider,
        repositories,
        broker.clone(),
        event_publisher,
        &settings.blockchain,
    );
    // Validate strategy (This function panics if there is a problem regarding the strategy).
    // NOTE: maybe send error, and propagate error here, then process::exit(1).
    evm_listener
        .validate_strategy_and_init_block(
            settings
                .blockchain
                .strategy
                .block_start_on_first_start
                .clone(),
        )
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

    let catchup_consumer = match broker
        .consumer(&Topic::new(routing::CATCHUP).with_namespace(chain_id))
        .group(routing::CATCHUP)
        .prefetch(settings.blockchain.catchup.prefetch)
        .max_retries(5)
        .redis_claim_min_idle(settings.blockchain.catchup.claim_min_idle_secs)
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
            error!("Failed to build catchup consumer: {}", e);
            process::exit(1);
        }
    };

    // ── Define handlers ─────────────────────────────────────────────────
    let flow_lock = FlowLock::new(Arc::clone(&arc_pg_client), configured_chain_id);
    let fetch_handler = FetchHandler::new(
        Arc::clone(&evm_listener),
        flow_lock.clone(),
        handler_publisher.clone(),
    );
    let reorg_handler = ReorgHandler::new(Arc::clone(&evm_listener), flow_lock, handler_publisher);

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

    let catchup_handler = CatchupHandler::new(Arc::clone(&evm_listener));

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

    // ── Ensure catchup topology (no seed — catchup messages come from external producers) ──
    if let Err(e) = catchup_consumer.ensure_topology().await {
        error!(error = %e, "Failed to set up catchup consumer topology");
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
    //         // println!("Consumer received event: {}", event);
    //         info!("GETTING EVENT FROM BLOCK: {}", event.block_number);
    //         Ok::<(), EvmListenerError>(())
    //     });

    // ── Periodic queue depth poller ──────────────────────────────────────
    // Polls broker.queue_depths() every 15s for each listener topic and emits
    // the values as broker_queue_depth_* Prometheus gauges.
    let queue_depth_cancel = tokio_util::sync::CancellationToken::new();
    let _queue_depth_task = broker::metrics::spawn_queue_depth_poller(
        broker.clone(),
        vec![
            broker::metrics::QueueDepthPollTarget::new(
                fetch_topic.clone(),
                routing::FETCH_NEW_BLOCKS,
            ),
            broker::metrics::QueueDepthPollTarget::new(
                reorg_topic.clone(),
                routing::BACKTRACK_REORG,
            ),
            broker::metrics::QueueDepthPollTarget::new(
                Topic::new(routing::WATCH).with_namespace(chain_id),
                routing::WATCH,
            ),
            broker::metrics::QueueDepthPollTarget::new(
                Topic::new(routing::UNWATCH).with_namespace(chain_id),
                routing::UNWATCH,
            ),
            broker::metrics::QueueDepthPollTarget::new(
                cleaner_topic.clone(),
                routing::CLEAN_BLOCKS,
            ),
        ],
        Duration::from_secs(15),
        queue_depth_cancel,
    );

    // ── Start the shared HTTP server (hosts /livez + /readyz today) ─────
    // Bound to `settings.http_port` — a single application port designed to
    // host all operational routes (health now, metrics/admin/... later).
    // `/livez` is a stateless OK beacon; `/readyz` probes DB + broker.
    {
        let readyz = listener_core::health::ReadinessChecker::new(
            broker.clone(),
            arc_pg_client.pool().clone(),
        );
        let app = listener_core::health::router(readyz);
        let addr = SocketAddr::from(([0, 0, 0, 0], settings.http_port));
        if let Err(e) = listener_core::health::serve(addr, app).await {
            error!(addr = %addr, error = %e, "Failed to bind HTTP server");
            process::exit(1);
        }
    }

    // ── Run both consumers concurrently ─────────────────────────────────
    info!("Starting consumers");

    let (consumer_name, result) = tokio::select! {
        r = fetch_consumer.run(fetch_handler) => ("Fetch", r),
        r = reorg_consumer.run(reorg_handler) => ("Reorg", r),
        r = watch_consumer.run(watch_handler) => ("Watch", r),
        r = unwatch_consumer.run(unwatch_handler) => ("Unwatch", r),
        r = cleaner_consumer.run(cleaner_handler) => ("Cleaner", r),
        r = catchup_consumer.run(catchup_handler) => ("Catchup", r),

        // TEST CONSUMER: TO REMOVE.
        // r = test_consumer_lib.run(consumer_lib_handler_test) => ("test copro lib consumer", r),
    };

    error!("{consumer_name} consumer exited: {result:?}");
    error!("Shutting down all consumers and exiting");
    process::exit(1);
}
