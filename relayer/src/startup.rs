//! Relayer Library
//!
//! Relayer service provides an interface to request input proofs and decryptions
//! for ciphertexts on the Zama Gateway Chain. The service:
//!
//! 1. Listens for requests to from fhevm blockchain events to http endpoint.
//! 2. Forwards requests to the gateway blockchain for processing
//! 3. Receives responses from gateway blockchain 4. Relay the result back to source (fhevm blockchain or HTTP caller).
//!
//! # Architecture
//!
//! The system consists of several key components:
//! - [`Orchestrator`]: Manages event flow and dispatch
//! - [`GatewayHandler`]: Manages gateway interactions
//! - [`TransactionService`]: Handles blockchain transactions (for both fhevm and gateway)
//!
//! # Configuration
//!
//! The service is configured via:
//! - Environment variables
//! - Configuration files in the `config/` directory
//! - Command-line arguments
//!
//! See [`Settings`] for detailed configuration options.

use crate::gateway::{self, throttlers::init_throttlers};
use crate::host::{HostChainIdChecker, KeyUrlPoller, UserDecryptSignaturePreChecker};
use anyhow::Context;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::{info, span, Level};

use crate::{
    config::settings::{KeyUrlConfig, Settings},
    http::endpoints::v2::types::keyurl::KeyUrlResponseJson,
    http::server::run_http_server,
    metrics,
    orchestrator::{DispatcherLock, HealthCheck, Orchestrator, TokioEventDispatcher},
    startup_recovery,
    store::sql::repositories::Repositories,
};
use prometheus::Registry;
use std::sync::OnceLock;

// Global singleton registry for metrics
static GLOBAL_REGISTRY: OnceLock<Registry> = OnceLock::new();

// Shutdown exists to hand the dispatcher over, not to save in-flight work: a request left
// `queued` is recovered at the next start, an event whose handlers have not returned is
// re-read from the chain because its block cursor never advanced, and a transaction send
// killed mid-flight costs at most one duplicate send plus one orphaned response event - the
// gateway contracts mint a fresh id and charge a fee per call with no dedup, so the duplicate
// costs a fee plus duplicate KMS work but never a wrong result, and the orphaned response just
// retries and is dropped at debug on the losing side. What recovery cannot do is order the
// handover, so shutdown drains only the named tasks it can order against, and abandons the
// rest. Its duration is handover latency, and the total must stay below the pod's
// `terminationGracePeriodSeconds` (the Kubernetes SIGKILL deadline), which gitops leaves
// unset, so the Kubernetes default of 30s applies.

/// Bound on the HTTP server's graceful shutdown, the gateway listeners, the keyurl poller,
/// the tx/readiness processors and the cron workers all observing cancellation and returning.
/// Every one of them is cancel-safe, so this is a fallback, not an expected wait.
const STOP_WORK_TIMEOUT: Duration = Duration::from_secs(5);

/// Bound on waiting for this pod's *first* dispatcher-lock acquisition attempt to resolve,
/// before the rest of startup proceeds.
///
/// Nothing downstream *requires* the lock any more - every subsystem behind the dispatch gate
/// waits for it on its own (build-order step 7), and a pod that never acquires still binds,
/// serves `/healthz`, accepts requests and starts its metrics server. This wait only removes a
/// pointless detour on the common path: without it, a pod that is about to acquire the lock a
/// few milliseconds from now would accept its first requests as a non-holder, stamp them
/// unowned, and hand them to its own sweep a tick later instead of driving them directly.
///
/// Generous for the single-replica and solo-restart cases (acquiring an unclaimed key is one
/// Postgres round trip, well under this), short enough that a standby losing the race to a
/// healthy peer does not sit here waiting for a lock that peer may hold for hours.
const FIRST_EPOCH_WAIT_BUDGET: Duration = Duration::from_secs(5);

/// Main library function for the FHE Event Relayer service.
///
/// This function performs the following initialization steps:
/// 1. Loads and validates configuration
/// 2. Sets up transaction services for fhevm and gateway
/// 3. Creates and configures event handlers
/// 4. Starts event listeners
/// 5. Waits for a shutdown signal, then hands over and exits (see the sequence below)
pub async fn run_fhevm_relayer(
    settings: Settings,
    shutdown_token: CancellationToken,
    settings_sender: Option<oneshot::Sender<Settings>>,
) -> anyhow::Result<()> {
    // === Setup Phase ===
    // Initialize logging, metrics, and validate configuration
    info!("Starting relayer with configuration: {:?}", settings);

    let main_span = span!(Level::INFO, "main-span"); // Add other relevant top-level details
    let setup_span = span!(parent: &main_span, Level::INFO, "setup-span");
    let metrics_registry = ensure_global_init(&settings)?;
    let metrics_endpoint = settings.metrics.endpoint.clone();
    let registry_clone = metrics_registry.clone();

    // === Orchestration Phase ===
    // Create orchestrator, repositories, and gateway components.
    //
    // `detached_tasks` holds the short-lived, unbounded-in-number work that doesn't fit the
    // orchestrator's named JoinSet: per-event dispatch, per-readiness-check work, and
    // transaction sends. None of it blocks shutdown - each is resumable from what Postgres and
    // the chain cursor already hold - so shutdown abandons it rather than waiting.
    //
    // The three tokens name subsystems, not phases - they are all cancelled at the same
    // moment: `intake_shutdown` closes the sources of new work (the HTTP server, the gateway
    // listeners, the keyurl poller), `dequeue_shutdown` stops the tx/readiness processors and
    // cron workers and cancels the readiness checks already running, and `metrics_shutdown`
    // stops the metrics server.
    let detached_tasks = TaskTracker::new();
    let intake_shutdown = CancellationToken::new();
    let dequeue_shutdown = CancellationToken::new();
    let metrics_shutdown = CancellationToken::new();
    let orchestrator = Orchestrator::new(
        Arc::new(TokioEventDispatcher::new(detached_tasks.clone())),
        detached_tasks.clone(),
    );

    // === Fast Setup Phase ===
    // Repositories, health-check registration, throttlers, checkers, and the dispatcher
    // lock's dedicated connection: everything the HTTP bind below needs, and nothing that
    // touches the gateway or blocks for long.

    // Dispatcher lock: one dedicated connection, outside both pools (see
    // `orchestrator::dispatcher_lock` for why). Connecting and resolving the lock key is a
    // single fast round trip, so it belongs here; actual acquisition happens in its own
    // background task, spawned below once the heavy work has registered the rest of the
    // named tasks. Connected before the repositories below because they hold a clone of it -
    // every request-row and chain-cursor write fences against `current_epoch()` (build-order
    // step 8).
    let dispatcher_lock = DispatcherLock::connect(
        &settings.dispatcher_lock,
        &settings.storage.sql_database_url,
    )
    .await
    .context("Failed to initialize dispatcher lock")?;

    // Initialize SQL repositories
    let repositories = Arc::new(
        Repositories::new(settings.storage.clone(), dispatcher_lock.clone())
            .await
            .context("Failed to initialize SQL repositories")?,
    );
    info!("Initialized SQL repositories");

    // Register database with orchestrator for health checks
    orchestrator.add_health_check(
        "database".to_string(),
        repositories.clone() as Arc<dyn HealthCheck>,
    );

    // Pure read; must run before anything gates on it later, or the gauges sit uninitialised.
    startup_recovery::init_status_counts_from_db(&repositories)
        .await
        .context("Failed to initialize status-count metrics")?;

    let (gateway_throttlers, bouncer_throttlers) =
        init_throttlers(&settings, detached_tasks.clone());

    // Build host chain validator from config
    let host_chain_id_checker = Arc::new(HostChainIdChecker::new(
        settings.host_chains.iter().map(|hc| hc.chain_id).collect(),
    ));

    // Build the v3 signature pre-checker. Reuses the host ACL retry policy so transport
    // failures behave like ACL call failures.
    let signature_prechecker = Arc::new(UserDecryptSignaturePreChecker::new(
        &settings.host_chains,
        &settings.gateway.contracts.decryption_address,
        settings.user_decrypt_signature_check.erc1271_gas_limit,
        settings
            .gateway
            .readiness_checker
            .host_acl_check
            .retry
            .clone(),
    )?);

    let mut settings = settings;

    // === Services Phase (bind) ===
    // Gate startup on the first successful host-chain poll so `/v2/keyurl` always serves a
    // chain-sourced value; if it keeps failing the relayer exits and is restarted - unchanged
    // from before this task. What moved is only *when*: this now runs, and the HTTP server
    // binds, before the heavy work below (`initialize_gateway`, recovery, the 30s cron delay),
    // so `/healthz` doesn't wait behind work that has nothing to do with serving it.
    let pending_keyurl_poller = if settings.http.endpoint.is_some() {
        info!("Starting Relayer HTTP server");

        // `/v2/keyurl` is served from a watch channel in both modes; only `chain` runs a poller.
        let (initial_keyurl, keyurl_poller) = match &settings.keyurl {
            KeyUrlConfig::Chain {
                kms_generation_address,
                poll_interval_ms,
            } => {
                let mut poller = KeyUrlPoller::new(
                    &settings.protocol_config,
                    kms_generation_address,
                    *poll_interval_ms,
                )
                .context("Failed to build KeyUrl poller")?;
                let initial = poller
                    .initialize()
                    .await
                    .context("Failed to initialize /v2/keyurl from host chain")?;
                (initial, Some(poller))
            }
            KeyUrlConfig::Config {
                fhe_public_key,
                crs,
            } => {
                info!(
                    "Serving /v2/keyurl from static config; no host-chain KeyUrl poller is started"
                );
                (
                    KeyUrlResponseJson::new(fhe_public_key.clone(), crs.clone()),
                    None,
                )
            }
        };
        let (keyurl_tx, keyurl_rx) = tokio::sync::watch::channel(initial_keyurl);

        let addr = run_http_server(
            &settings,
            Arc::clone(&orchestrator),
            repositories.clone(),
            bouncer_throttlers,
            host_chain_id_checker,
            signature_prechecker,
            keyurl_rx,
            intake_shutdown.clone(),
        )
        .await;

        info!("HTTP server bound to actual address: {}", addr);
        settings.http.endpoint = Some(addr.to_string());

        // Under `keyurl.source: config` there is no poller to defer; the channel already holds
        // the value `/v2/keyurl` serves for the process's lifetime.
        keyurl_poller.map(|poller| (poller, keyurl_tx))
    } else {
        None
    };

    // === Heavy Startup Work ===
    // Everything here can block for a while (recovery walks the DB, the cron delay is 30s in
    // production) and now runs after the bind, so `/healthz` is already serving throughout.

    // Dispatcher lock's poll/heartbeat loop, plus a bounded best-effort wait for this pod's
    // first acquisition attempt to resolve - see `FIRST_EPOCH_WAIT_BUDGET` for why the wait is
    // a convenience rather than a dependency. Everything spawned after this gates itself on
    // `dispatcher_lock.gate()`, so a pod that loses the race carries on starting up and simply
    // drives nothing until it acquires.
    {
        let lock_for_task = dispatcher_lock.clone();
        let lock_shutdown = dequeue_shutdown.clone();
        let lock_for_wait = dispatcher_lock.clone();
        orchestrator
            .spawn_task_and_wait_ready(
                "dispatcher_lock",
                async move { lock_for_task.run(lock_shutdown).await },
                async move {
                    let _ = tokio::time::timeout(FIRST_EPOCH_WAIT_BUDGET, async {
                        // Watches the epoch channel directly, not lock state: `Held` can be set
                        // before the epoch is minted on the retry path in `heartbeat_tick` (see
                        // `subscribe_epoch`'s doc comment), and waiting on state there would
                        // wake early with the epoch still `None`.
                        let mut epoch_rx = lock_for_wait.subscribe_epoch();
                        while lock_for_wait.current_epoch().is_none() {
                            if epoch_rx.changed().await.is_err() {
                                return;
                            }
                        }
                    })
                    .await;
                    // Best-effort: a timeout (this pod never acquires) or a closed channel
                    // (never happens in practice) must not fail startup. Every subsystem past
                    // this point gates itself, so losing the race costs nothing but the
                    // convenience described on `FIRST_EPOCH_WAIT_BUDGET`.
                    anyhow::Ok(())
                },
            )
            .await
            .context("Failed to start dispatcher lock")?;
    }

    // Initialize all gateway components. The listeners, tx processors and readiness processors
    // all start, and all sit behind the dispatch gate until this pod is the confirmed
    // dispatcher (build-order step 7) - `initialize_gateway` has no construct-without-starting
    // seam, and giving each subsystem the gate is what makes one unnecessary.
    gateway::initialize_gateway(
        orchestrator.clone(),
        &settings,
        repositories.clone(),
        gateway_throttlers,
        dispatcher_lock.gate(),
        dequeue_shutdown.clone(),
        intake_shutdown.clone(),
    )
    .await
    .context("Failed to initialize gateway")?;

    // The sweep: the only Postgres -> dispatch path there is, so it starts before the cron
    // delay below rather than after it. It is what recovers this pod's own previous
    // incarnation's work (every row that incarnation owned is under an older epoch, so the
    // first tick after acquisition claims it), and making a restart wait out
    // `cron_startup_delay_after_recovery` before recovering anything would be a regression on
    // the startup pass it replaced. Gated like everything else, so on a standby it idles.
    {
        let sweep_repositories = repositories.clone();
        let sweep_orchestrator = orchestrator.clone();
        let sweep_gate = dispatcher_lock.gate();
        let sweep_config = settings.sweep.clone();
        let sweep_shutdown = dequeue_shutdown.clone();
        orchestrator
            .spawn_task_and_wait_ready(
                "sweep",
                async move {
                    crate::sweep::create_sweep_worker_future(
                        sweep_repositories,
                        sweep_orchestrator,
                        sweep_gate,
                        sweep_config,
                        sweep_shutdown,
                    )
                    .await
                },
                async { anyhow::Ok(()) },
            )
            .await
            .context("Failed to start sweep worker")?;
    }

    // Start cron workers after a configurable delay. The delay exists so the timeout worker
    // does not start expiring rows the sweep has not had a chance to re-drive yet.
    let cron = &settings.storage.cron;
    let delay = cron.cron_startup_delay_after_recovery;
    info!("Waiting {:?} before starting cron workers...", delay);
    tokio::time::sleep(delay).await;

    info!(
        expiry_enabled = cron.expiry_enabled,
        "Starting cron workers"
    );
    repositories
        .register_background_workers(
            &orchestrator,
            settings.storage.cron.clone(),
            dispatcher_lock.gate(),
            dequeue_shutdown.clone(),
        )
        .await
        .context("Failed to register background workers")?;

    // Spawn the single host-chain KeyUrl poller now the endpoint is serving. Startup is
    // already gated above, so the readiness future is trivially ready; the task is tracked by
    // the orchestrator, which stops it with the other sources of new work.
    if let Some((keyurl_poller, keyurl_tx)) = pending_keyurl_poller {
        let keyurl_intake_shutdown = intake_shutdown.clone();
        orchestrator
            .spawn_task_and_wait_ready(
                "keyurl_poller",
                async move { keyurl_poller.run(keyurl_tx, keyurl_intake_shutdown).await },
                async { anyhow::Ok(()) },
            )
            .await
            .context("Failed to start KeyUrl poller")?;
    }

    // === Services Phase (metrics) ===
    // Run metrics server
    info!("Starting Relayer metrics server");
    let actual_metrics_addr = metrics::server::run_metrics_server(
        registry_clone,
        metrics_endpoint,
        Arc::clone(&orchestrator),
        metrics_shutdown.clone(),
    )
    .await;
    info!(
        "Metrics server bound to actual address: {}",
        actual_metrics_addr
    );
    settings.metrics.endpoint = actual_metrics_addr.to_string();

    drop(setup_span);

    info!("All servers are ready and responding");

    // Send settings through the channel if provided (for tests)
    if let Some(sender) = settings_sender {
        let _ = sender.send(settings.clone());
        info!("Settings sent to test setup with actual server addresses");
    }

    // === Runtime Phase ===
    // Wait for the shutdown signal (SIGTERM/SIGINT, see `fhevm-relayer.rs`).
    shutdown_token.cancelled().await;
    info!("Shutdown signal received, starting graceful shutdown");
    orchestrator.begin_task_drain().await;

    // Step 1 - stop looking healthy, keep serving. `/healthz` answers 503 from here on while
    // the HTTP server stays up, so a request routed during the window gets a clean 503 from a
    // pod still listening rather than a refused connection.
    orchestrator.mark_not_ready();

    // Step 2 - let that reach the load balancer. Deleting a pod removes it from the service
    // endpoints at the same moment it delivers SIGTERM, so the wait covers the lag until that
    // removal reaches the ingress controller, not the readiness probe's own detection time.
    // Serving nobody is pointless, so skip it when no HTTP endpoint is configured.
    if settings.http.endpoint.is_some() {
        let wait = settings.shutdown.lb_propagation_wait;
        info!(?wait, "Health check failing, waiting for traffic to drain");
        tokio::time::sleep(wait).await;
    }

    // Step 3 - stop working. The HTTP server finishes the requests it already accepted; the
    // gateway listeners, the keyurl poller, the tx/readiness processors, the cron workers and
    // the metrics server stop, and the readiness checks already running are cancelled where
    // they wait. Nothing here is ordered against anything else: a listener that forwards one
    // last event only spawns a handler shutdown abandons, and a request left in a queue is
    // already durable in Postgres. Every task is drained under one budget because every one of
    // them is cancel-safe; the shutdown that follows is observable from its log lines, which
    // outlive the metrics endpoint either way.
    //
    // The dispatcher lock's poll/heartbeat loop is cancelled by `dequeue_shutdown` below and
    // drained with everything else - it stops polling, but does not release (see Step 4).
    //
    // The step-6 sweep ("stop pickup from Postgres") is named "sweep" among the tasks drained
    // below, cancelled by the same `dequeue_shutdown`.
    intake_shutdown.cancel();
    dequeue_shutdown.cancel();
    metrics_shutdown.cancel();
    orchestrator.drain_named_tasks(STOP_WORK_TIMEOUT).await;
    info!(
        abandoned = orchestrator.abandoned_detached_tasks(),
        "Named task drain complete, abandoning remaining detached work"
    );

    // Step 4 - exit. The pools are left to close with the process: closing them here would
    // wait on the connections held by the tasks just abandoned, reinstating the wait this
    // sequence removes. `finish_task_drain` is the guard for a task that no predicate above
    // stopped, and warns when it finds one.
    orchestrator.finish_task_drain().await;

    // Release the dispatcher lock last, after everything else has stopped: releasing it any
    // earlier would make this process the stalled ex-holder the (future) fence exists to
    // catch. The lock's own loop already stopped polling above; this is the one explicit
    // unlock, not a race with it.
    dispatcher_lock.release_last().await;

    info!("Relayer shutdown complete");

    Ok(())
}

/// Initialize all global state exactly once
fn ensure_global_init(settings: &Settings) -> anyhow::Result<&'static Registry> {
    let registry = GLOBAL_REGISTRY.get_or_init(|| {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Failed to install AWS-LC crypto provider");

        let registry = Registry::new();
        metrics::init_http_metrics(&registry, &settings.http.metrics);
        metrics::init_transaction_metrics(&registry, settings.metrics.clone());
        metrics::init_statuses_metrics(&registry, settings.metrics.clone());
        metrics::init_db_metrics(&registry, settings.metrics.clone());
        metrics::init_queue_metrics(&registry);
        metrics::init_listener_metrics(&registry);
        metrics::init_dispatcher_lock_metrics(&registry);
        metrics::init_signature_precheck_metrics(&registry);
        metrics::init_retry_after_metrics(
            &registry,
            settings
                .metrics
                .retry_after_raw_eta_histogram_bucket
                .clone(),
        );

        registry
    });

    Ok(registry)
}
