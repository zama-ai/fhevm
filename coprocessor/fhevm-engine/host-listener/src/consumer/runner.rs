//! Process services outlive delivery subscriptions. Drift restarts only the latter.
use std::sync::Arc;

use anyhow::{Context, Result};
use consumer::{BlockPayload, Broker, ListenerConsumer};
use fhevm_engine_common::{
    chain_id::ChainId,
    drift_revert::{DriftRevertSignal, POLL_QUERY_TIMEOUT},
    healthz_server::HttpServer as HealthHttpServer,
    utils::HeartBeat,
    versioning::{run_stack_version_listener, StackMode},
};
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use super::{
    catchup::LiveReference,
    drift_recovery::{self, RecoveryPlan},
    ingestion::BlockIngestor,
    ConsumerConfig,
};
use crate::{
    database::{ingest::IngestOptions, tfhe_event_propagate::Database},
    health_check::HealthCheck,
};

struct Runner {
    config: ConsumerConfig,
    db: Database,
    broker: Broker,
    contracts: Vec<alloy::primitives::Address>,
    options: IngestOptions,
    stack_mode: Arc<StackMode>,
    blockchain_tick: HeartBeat,
    stop: CancellationToken,
    identity: String,
}

pub async fn run_consumer(config: ConsumerConfig) -> Result<()> {
    config.manual_catchup.validate()?;
    let chain_id = ChainId::try_from(config.chain_id.parse::<u64>()?)?;
    let identity = format!(
        "{}.{}.{}",
        config.service_name,
        config.chain_id,
        uuid::Uuid::new_v4()
    );
    let broker = Broker::from_url(&config.url).await?;
    let db = new_database(&config, chain_id).await?;
    db.tick.update();
    let stop = CancellationToken::new();
    let blockchain_tick = HeartBeat::new();
    let health_check = HealthCheck {
        blockchain_timeout_tick: HeartBeat::new(),
        blockchain_tick: blockchain_tick.clone(),
        blockchain_provider: Arc::new(RwLock::new(None)),
        database_pool: db.pool.clone(),
        database_tick: db.tick.clone(),
    };
    let server = HealthHttpServer::new(
        Arc::new(health_check),
        config.health_port,
        stop.clone(),
    );
    let stack_mode = StackMode::new(config.gcs_mode);
    let pool = db.pool().await;
    let mode = stack_mode.clone();
    let version_stop = stop.clone();
    let mut services = JoinSet::new();
    services.spawn(async move { server.start().await });
    services.spawn(async move {
        run_stack_version_listener(pool, mode, version_stop).await
    });
    let signal_stop = stop.clone();
    let signal_task = tokio::spawn(async move {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => signal_stop.cancel(),
            _ = signal_stop.cancelled() => {},
        }
    });
    let mut contracts = vec![config.acl_address, config.tfhe_address];
    contracts.extend(config.protocol_config_address);
    contracts.extend(config.kms_generation_address);
    contracts.extend(config.confidential_bridge_address);
    let options = IngestOptions {
        dependence_by_connexity: config.dependence_by_connexity,
        dependence_cross_block: config.dependence_cross_block,
        dependent_ops_max_per_chain: config.dependent_ops_max_per_chain,
        is_protocol_config_listener:
            crate::protocol_config::resolve_protocol_config_listener(
                config.canonical_protocol_config_chain_id,
                chain_id.as_u64(),
            )?,
        disable_synthetic_ops: config.disable_synthetic_ops,
    };
    let runner = Runner {
        config,
        db,
        broker,
        contracts,
        options,
        stack_mode,
        blockchain_tick,
        stop: stop.clone(),
        identity,
    };
    // The loop owns cleanup; do not drop it on shutdown while handlers are active.
    let result = runner.run().await;
    stop.cancel();
    signal_task.await?;
    while let Some(result) = services.join_next().await {
        if let Err(error) = result
            .context("consumer service panicked")
            .and_then(|result| result)
        {
            warn!(%error, "Consumer service stopped with an error");
        }
    }
    result
}

async fn new_database(
    config: &ConsumerConfig,
    chain_id: ChainId,
) -> Result<Database> {
    Database::new_with_gcs_mode(
        &config.database_url,
        chain_id,
        config.dependence_cache_size,
        config.gcs_mode,
    )
    .await
}

impl Runner {
    async fn run(&self) -> Result<()> {
        let mut plan = RecoveryPlan::default();
        // Create a fresh subscription at startup and after each drift-revert
        // completes, so old deliveries cannot enter the new subscription.
        let mut subscription_id = 0_u64;
        loop {
            let drift_recovery::CleanupOutcome::Ready(accepted) =
                drift_recovery::wait_for_cleanup(
                    &self.db, &mut plan, &self.stop,
                )
                .await?
            else {
                return Ok(());
            };
            if self.stop.is_cancelled() {
                return Ok(());
            }
            // Fresh dependency caches after cleanup, just as with a process restart.
            let mut db = tokio::select! {
                _ = self.stop.cancelled() => return Ok(()),
                result = new_database(&self.config, self.db.chain_id) => result?,
            };
            db.tick = self.db.tick.clone();
            super::SLOW_LANE_PROMOTION_ATTEMPTED
                .store(false, std::sync::atomic::Ordering::Release);
            let id = format!("{}.s{subscription_id}", self.identity);
            let client = ListenerConsumer::new(
                &self.broker,
                self.db.chain_id.as_u64(),
                &id,
            );
            info!(subscription_id, consumer_id = %id, "Starting consumer subscription");
            match self
                .run_subscription(&client, db, accepted, &mut plan)
                .await?
            {
                None => return Ok(()),
                Some(signal) => {
                    plan.observe(&signal)?;
                    info!(subscription_id, drift_id = signal.id, start = signal.offending_host_block_number, "Drift stopped both flows; waiting for cleanup before replay");
                    subscription_id = subscription_id
                        .checked_add(1)
                        .context("consumer subscription overflow")?;
                }
            }
        }
    }

    async fn run_subscription(
        &self,
        client: &ListenerConsumer,
        db: Database,
        accepted: Option<DriftRevertSignal>,
        plan: &mut RecoveryPlan,
    ) -> Result<Option<DriftRevertSignal>> {
        let mut tasks = JoinSet::new();
        let (drift_tx, mut drift_rx) = mpsc::unbounded_channel();
        let (live_reference, head_rx) =
            LiveReference::new(db.chain_id.as_u64());
        let observed_reference = live_reference.clone();
        let ingestor = Arc::new(BlockIngestor {
            db: db.clone(),
            chain_id: db.chain_id,
            config: self.config.clone(),
            options: self.options.clone(),
            live_reference,
            checkpoint: accepted.clone(),
            drift_tx,
            mode: self.stack_mode.clone(),
            tick: self.blockchain_tick.clone(),
            cancel: client.cancel_token.clone(),
            in_flight_handlers: RwLock::new(()),
        });
        let handler_ingestor = ingestor.clone();
        let handle = move |payload: BlockPayload,
                           _cancel: CancellationToken| {
            let ingestor = Arc::clone(&handler_ingestor);
            async move { ingestor.ingest(payload).await }
        };
        let run = async {
            client.ensure_consumer().await?;
            client.ensure_catchup_consumer().await?;
            client.register_contracts(&self.contracts).await?;
            let live = client.consume(handle.clone());
            let catchup = client.consume_catchup(handle);
            tasks.spawn(async move { live.await.map_err(anyhow::Error::from) });
            tasks.spawn(
                async move { catchup.await.map_err(anyhow::Error::from) },
            );
            let stats_db = db.clone();
            let chain = self.config.chain_id.clone();
            let cancel = client.cancel_token.clone();
            tasks.spawn(async move {
                tokio::select! {
                    _ = cancel.cancelled() => {},
                    _ = super::run_consumer_stats_observer(stats_db, chain, cancel.clone()) => {},
                }
                Ok(())
            });
            let startup = async {
                let reference =
                    head_rx.await.context("live reference channel closed")?;
                if let Some((start, end)) =
                    plan.resolve(&self.config.manual_catchup, reference)?
                {
                    info!(
                        reference,
                        start,
                        end,
                        "Requesting recovery replay; live processing continues"
                    );
                    // Atomic WATCH plus a live block from this subscription proves
                    // filters are active. No readiness retry is needed after UNWATCH.
                    client.request_catchup(start, end).await?;
                }
                Ok::<_, anyhow::Error>(())
            };
            tokio::select! {
                result = startup => { result?; },
                result = tasks.join_next() => return unexpected_worker(result),
            }
            unexpected_worker(tasks.join_next().await)
        };
        let result = tokio::select! {
            biased;
            _ = self.stop.cancelled() => Ok(None),
            signal = drift_rx.recv() => signal.context("drift notification channel closed").map(Some),
            signal = drift_recovery::wait_for_drift(&db, accepted.as_ref()) => signal.map(Some),
            result = run => result.map(|()| None),
        };
        ingestor.stop().await;
        // Preserve relative manual bounds even if drift won the startup select
        // immediately after the first live payload was observed.
        let reference = observed_reference.observed().await;
        client.cancel();
        while let Some(result) = tasks.join_next().await {
            if let Err(error) = result
                .context("consumer task panicked")
                .and_then(|result| result)
            {
                warn!(%error, "Consumer task failed while stopping subscription");
            }
        }
        // Old queues remain isolated; synchronous producer/queue retirement is
        // a separate core API. UNWATCH stops future fanout after it is processed.
        match tokio::time::timeout(POLL_QUERY_TIMEOUT, client.unregister_contracts(&self.contracts)).await {
            Ok(Ok(())) => {},
            other => warn!(?other, consumer_id = client.consumer_id(), "Could not deregister old subscription; broker resources need cleanup"),
        }
        if let Some(reference) = reference {
            plan.resolve(&self.config.manual_catchup, reference)?;
        }
        result
    }
}

fn unexpected_worker(
    result: Option<Result<Result<()>, tokio::task::JoinError>>,
) -> Result<()> {
    if let Some(result) = result {
        result.context("consumer worker panicked")??;
    }
    anyhow::bail!("consumer worker exited unexpectedly")
}

#[cfg(test)]
mod tests;
