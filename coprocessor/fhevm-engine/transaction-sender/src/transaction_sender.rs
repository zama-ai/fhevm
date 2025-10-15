use alloy::{network::Ethereum, primitives::Address, providers::Provider};
use futures_util::FutureExt;
use sqlx::{postgres::PgListener, Pool, Postgres};
use std::{sync::Arc, time::Duration};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::{
    is_backend_gone, nonce_managed_provider::NonceManagedProvider, ops, AbstractSigner,
    ConfigSettings, HealthStatus,
};

#[derive(Clone)]
pub struct TransactionSender<P: Provider<Ethereum> + Clone + 'static> {
    cancel_token: CancellationToken,
    conf: ConfigSettings,
    operations: Vec<Arc<dyn ops::TransactionOperation<P>>>,
    input_verification_address: Address,
    ciphertext_commits_address: Address,
    multichain_acl_address: Address,
    db_pool: Pool<Postgres>,
    provider: NonceManagedProvider<P>,
}

impl<P: Provider<Ethereum> + Clone + 'static> TransactionSender<P> {
    #[expect(clippy::too_many_arguments)]
    pub async fn new(
        input_verification_address: Address,
        ciphertext_commits_address: Address,
        multichain_acl_address: Address,
        signer: AbstractSigner,
        provider: NonceManagedProvider<P>,
        cancel_token: CancellationToken,
        conf: ConfigSettings,
        gas: Option<u64>,
    ) -> anyhow::Result<Self> {
        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(conf.database_pool_size)
            .connect(conf.database_url.as_str())
            .await?;

        let operations: Vec<Arc<dyn ops::TransactionOperation<P>>> = vec![
            Arc::new(
                ops::verify_proof::VerifyProofOperation::new(
                    input_verification_address,
                    provider.clone(),
                    signer.clone(),
                    conf.clone(),
                    gas,
                    db_pool.clone(),
                )
                .await?,
            ),
            Arc::new(ops::add_ciphertext::AddCiphertextOperation::new(
                ciphertext_commits_address,
                provider.clone(),
                conf.clone(),
                gas,
                db_pool.clone(),
            )),
            Arc::new(ops::allow_handle::MultichainACLOperation::new(
                multichain_acl_address,
                provider.clone(),
                conf.clone(),
                gas,
                db_pool.clone(),
            )),
        ];
        Ok(Self {
            cancel_token,
            conf,
            operations,
            input_verification_address,
            ciphertext_commits_address,
            multichain_acl_address,
            db_pool,
            provider,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!(
            conf = ?self.conf,
            input_verification_address = %self.input_verification_address,
            ciphertext_commits_address = %self.ciphertext_commits_address,
            multichain_acl_address = %self.multichain_acl_address,
            "Starting Transaction Sender"
        );

        let mut join_set = JoinSet::new();

        for op in self.operations.clone() {
            let op_channel = op.channel().to_owned();
            let token = self.cancel_token.clone();
            let db_polling_interval_secs = self.conf.db_polling_interval_secs;
            join_set.spawn({
                let sender = self.clone();
                info!(channel = op_channel, "Spawning operation loop");
                async move {
                    let mut sleep_duration = sender.conf.error_sleep_initial_secs as u64;
                    let mut listener = PgListener::connect_with(&sender.db_pool).await?;
                    listener.listen(&op_channel).await?;
                    loop {
                        if token.is_cancelled() {
                            info!(channel = op_channel, "Operation stopping");
                            break;
                        }

                        match op.execute().await {
                            Err(e) => {
                                if is_backend_gone(&e) {
                                    error!(
                                        channel = op_channel,
                                        error = %e,
                                        "Backend gone error, stopping operation and signalling other operations to stop"
                                    );
                                    token.cancel();
                                    return Err(e);
                                }
                                error!(
                                    channel = op_channel,
                                    error = %e,
                                    sleep_duration = sleep_duration,
                                    "Operation error, retrying after sleep"
                                );
                                sender.sleep_with_backoff(&mut sleep_duration).await;
                                continue;
                            }
                            Ok(true) => {
                                // Maybe we have more work to do, don't wait and immediately run the loop again.
                                sender.reset_sleep_duration(&mut sleep_duration);
                                continue;
                            }
                            Ok(false) => {

                                // Maybe no more work to do, go and wait for the next notification.
                                sender.reset_sleep_duration(&mut sleep_duration);

                                let notification = listener.try_recv().fuse();
                                tokio::select! {
                                    _ = token.cancelled() => {
                                        info!(channel = op_channel, "Operation stopping");
                                        break;
                                    }
                                    n = notification => {
                                        match n {
                                            Ok(Some(_)) => {
                                                debug!(
                                                    channel = op_channel,
                                                    "Received notification, rechecking for work"
                                                );
                                            },
                                            Ok(None) => {
                                                debug!(
                                                    channel = op_channel,
                                                    sleep_duration = sleep_duration,
                                                    "Received empty notification, sleeping"
                                                );
                                                sender.sleep_with_backoff(&mut sleep_duration).await;
                                            }
                                            Err(e) => {
                                                error!(
                                                    channel = op_channel,
                                                    error = %e,
                                                    sleep_duration = sleep_duration,
                                                    "Notification error, sleeping"
                                                );
                                                sender.sleep_with_backoff(&mut sleep_duration).await;
                                            }
                                        }
                                    }
                                    _ = tokio::time::sleep(Duration::from_secs(db_polling_interval_secs.into())) => {
                                        debug!(
                                            channel = op_channel,
                                            "Timeout reached, rechecking for work"
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Ok::<(), anyhow::Error>(())
                }
            });
        }

        self.cancel_token.cancelled().await;
        info!("Cancellation requested, waiting for operations to stop");
        // Make sure we don't wait indefinitely.
        let timeout_future = tokio::time::sleep(self.conf.graceful_shutdown_timeout);
        tokio::pin!(timeout_future);
        loop {
            tokio::select! {
                _ = &mut timeout_future => {
                    error!("Graceful shutdown timeout reached, some operations may not have stopped gracefully");
                    break Err(anyhow::anyhow!("Timeout reached during graceful shutdown"));
                }

                n = join_set.join_next() => {
                    match n {
                        Some(Ok(Ok(_))) => {
                            info!("An operation stopped gracefully");
                        }
                        Some(Ok(Err(e))) => {
                            error!(error = %e, "An operation returned an error");
                            break Err(e);
                        }
                        Some(Err(e)) => {
                            error!(error = %e, "Join failed with an error");
                            break Err(e.into());
                        }
                        None => {
                            info!("All operations stopped");
                            break Ok(())
                        }
                    }
                }
            }
        }
    }

    fn reset_sleep_duration(&self, sleep_duration: &mut u64) {
        *sleep_duration = self.conf.error_sleep_initial_secs as u64;
    }

    async fn sleep_with_backoff(&self, sleep_duration: &mut u64) {
        tokio::time::sleep(Duration::from_secs(*sleep_duration)).await;
        *sleep_duration = std::cmp::min(*sleep_duration * 2, self.conf.error_sleep_max_secs as u64);
    }

    /// Checks the health of the transaction sender's connections
    pub async fn health_check(&self) -> HealthStatus {
        let mut database_connected = false;
        let mut blockchain_connected = false;
        let mut error_details = Vec::new();

        // Check database connection
        match sqlx::query("SELECT 1").execute(&self.db_pool).await {
            Ok(_) => {
                database_connected = true;
            }
            Err(e) => {
                error_details.push(format!("Database query error: {}", e));
            }
        }

        // Check blockchain connection by getting the last block number.
        // The provider internal retry may last a long time, so we set a timeout.
        match tokio::time::timeout(
            self.conf.health_check_timeout,
            self.provider.get_block_number(),
        )
        .await
        {
            Ok(Ok(_)) => {
                blockchain_connected = true;
            }
            Ok(Err(e)) => {
                error_details.push(format!("Blockchain connection error: {}", e));
            }
            Err(_) => {
                error_details.push("Blockchain connection timeout".to_string());
            }
        }

        // Determine overall health status
        if database_connected && blockchain_connected {
            HealthStatus::healthy()
        } else {
            HealthStatus::unhealthy(
                database_connected,
                blockchain_connected,
                error_details.join("; "),
            )
        }
    }
}
