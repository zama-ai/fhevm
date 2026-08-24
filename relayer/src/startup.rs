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
    orchestrator::{HealthCheck, Orchestrator, TokioEventDispatcher},
    startup_recovery,
    store::sql::repositories::Repositories,
};
use prometheus::Registry;
use std::sync::OnceLock;

// Global singleton registry for metrics
static GLOBAL_REGISTRY: OnceLock<Registry> = OnceLock::new();

// Shutdown exists to hand the dispatcher over, not to save in-flight work: a request left
// `queued` is recovered at the next start, and a gateway event that dies mid-handling is lost
// exactly as a hard kill would lose it - which the commit after this one fixes by holding the
// block cursor. What recovery cannot do is order the handover, so shutdown settles the one
// effect that leaves the process - an outbound transaction - and abandons everything else.
// Its duration is handover latency, and
// the total must stay below the pod's `terminationGracePeriodSeconds` (the Kubernetes SIGKILL
// deadline), which gitops leaves unset, so the Kubernetes default of 30s applies.

/// Bound on the HTTP server's graceful shutdown, the gateway listeners, the keyurl poller,
/// the tx/readiness processors and the cron workers all observing cancellation and returning.
/// Every one of them is cancel-safe, so this is a fallback, not an expected wait.
const STOP_WORK_TIMEOUT: Duration = Duration::from_secs(5);
/// Bound on in-flight transaction sends. `send_raw_transaction_sync` submits and gets the
/// receipt in one RPC call against a happy path of one to two seconds, so this covers one
/// call per already-dequeued task rather than an open-ended wait. Keeping it short is what
/// keeps the `owner_epoch` fencing surface to the send path alone.
const INFLIGHT_TX_DRAIN_TIMEOUT: Duration = Duration::from_secs(3);

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
    // The two trackers below hold the short-lived, unbounded-in-number tasks that don't fit
    // the orchestrator's named JoinSet, split by what shutdown owes them: `inflight_sends`
    // holds transaction submissions, which it drains, and `detached_tasks` holds per-event
    // dispatch and per-readiness-check work, which it abandons. Which tracker a task is
    // spawned into is the whole policy.
    //
    // The three tokens name subsystems, not phases - they are all cancelled at the same
    // moment: `intake_shutdown` closes the sources of new work (the HTTP server, the gateway
    // listeners, the keyurl poller), `dequeue_shutdown` stops the tx/readiness processors and
    // cron workers and cancels the readiness checks already running, and `metrics_shutdown`
    // stops the metrics server.
    let inflight_sends = TaskTracker::new();
    let detached_tasks = TaskTracker::new();
    let intake_shutdown = CancellationToken::new();
    let dequeue_shutdown = CancellationToken::new();
    let metrics_shutdown = CancellationToken::new();
    let orchestrator = Orchestrator::new(
        Arc::new(TokioEventDispatcher::new(detached_tasks.clone())),
        inflight_sends.clone(),
        detached_tasks.clone(),
    );

    // Initialize SQL repositories
    let repositories = Arc::new(
        Repositories::new(settings.storage.clone())
            .await
            .context("Failed to initialize SQL repositories")?,
    );
    info!("Initialized SQL repositories");

    // Register database with orchestrator for health checks
    orchestrator.add_health_check(
        "database".to_string(),
        repositories.clone() as Arc<dyn HealthCheck>,
    );

    let (gateway_throttlers, bouncer_throttlers) =
        init_throttlers(&settings, inflight_sends, detached_tasks.clone());

    // Initialize all gateway components
    gateway::initialize_gateway(
        orchestrator.clone(),
        &settings,
        repositories.clone(),
        gateway_throttlers,
        dequeue_shutdown.clone(),
        intake_shutdown.clone(),
    )
    .await
    .context("Failed to initialize gateway")?;

    // Recover incomplete requests from previous runs
    info!("Recovering incomplete requests...");
    startup_recovery::recover_incomplete_requests(&orchestrator, &repositories)
        .await
        .context("Failed to recover incomplete requests")?;

    // Start cron workers after configurable delay
    let cron = &settings.storage.cron;
    let delay = cron.cron_startup_delay_after_recovery;
    info!(
        "Recovery complete. Waiting {:?} before starting cron workers...",
        delay
    );
    tokio::time::sleep(delay).await;

    info!(
        expiry_enabled = cron.expiry_enabled,
        "Starting cron workers"
    );
    repositories
        .register_background_workers(
            &orchestrator,
            settings.storage.cron.clone(),
            dequeue_shutdown.clone(),
        )
        .await
        .context("Failed to register background workers")?;

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

    // === Services Phase ===
    // Start HTTP server, metrics server, and initialize handlers
    if settings.http.endpoint.is_some() {
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
                // Gate startup on the first successful host-chain poll; if it keeps failing the
                // relayer exits and is restarted.
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

        // Spawn the single host-chain KeyUrl poller now the endpoint is serving. Startup is
        // already gated above, so the readiness future is trivially ready; the task is tracked
        // by the orchestrator, which stops it with the other sources of new work. Under
        // `keyurl.source: config` there is no poller at all - the watch channel is seeded from
        // config at startup - so there is nothing to spawn or to shut down.
        if let Some(keyurl_poller) = keyurl_poller {
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
    };

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
    // TODO(seam): "stop pickup from Postgres" (the activation sweep / 0.5s peek) belongs
    // here too, once that work exists.
    intake_shutdown.cancel();
    dequeue_shutdown.cancel();
    metrics_shutdown.cancel();
    orchestrator.drain_named_tasks(STOP_WORK_TIMEOUT).await;

    // Step 4 - settle the outbound effects. Each already-dequeued send is at most one
    // `eth_sendRawTransactionSync` call away from done (see `provider.rs`), and dropping it
    // after the RPC leaves the socket loses the hash that detects the duplicate a re-dispatch
    // would produce.
    orchestrator
        .drain_inflight_sends(INFLIGHT_TX_DRAIN_TIMEOUT)
        .await;

    // Step 5 - exit. The pools are left to close with the process: closing them here would
    // wait on the connections held by the tasks just abandoned, reinstating the wait this
    // sequence removes. `finish_task_drain` is the guard for a task that no predicate above
    // stopped, and warns when it finds one.
    //
    // TODO(seam): once the advisory-lock/owner_epoch work lands, "release the dispatcher
    // lock" is the last thing before returning - releasing it any earlier makes this process
    // the stalled ex-holder the fence exists to catch.
    orchestrator.finish_task_drain().await;

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
