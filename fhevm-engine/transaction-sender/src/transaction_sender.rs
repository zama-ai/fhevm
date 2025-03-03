use alloy::{
    network::Ethereum, primitives::Address, providers::Provider, signers::local::PrivateKeySigner,
};
use futures_util::FutureExt;
use sqlx::postgres::PgListener;
use std::{sync::Arc, time::Duration};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::{ops, ConfigSettings, TXN_SENDER_TARGET};

#[derive(Clone)]
pub struct TransactionSender<P: Provider<Ethereum> + Clone + 'static> {
    cancel_token: CancellationToken,
    conf: ConfigSettings,
    operations: Vec<Arc<dyn ops::TransactionOperation<P>>>,
    zkpok_manager_address: Address,
    ciphertext_storage_address: Address,
}

impl<P: Provider<Ethereum> + Clone + 'static> TransactionSender<P> {
    pub async fn new(
        zkpok_manager_address: Address,
        ciphertext_storage_address: Address,
        signer: PrivateKeySigner,
        provider: P,
        cancel_token: CancellationToken,
        conf: ConfigSettings,
        gas: Option<u64>,
    ) -> anyhow::Result<Self> {
        let operations: Vec<Arc<dyn ops::TransactionOperation<P>>> = vec![
            Arc::new(
                ops::verify_proof::VerifyProofOperation::new(
                    zkpok_manager_address,
                    provider.clone(),
                    signer.clone(),
                    conf.clone(),
                    gas,
                )
                .await?,
            ),
            Arc::new(ops::add_ciphertext::AddCiphertextOperation::new(
                ciphertext_storage_address,
                provider.clone(),
                conf.clone(),
            )),
        ];
        Ok(Self {
            cancel_token,
            conf,
            operations,
            zkpok_manager_address,
            ciphertext_storage_address,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!(target: TXN_SENDER_TARGET, "Starting Transaction Sender with: {:?}, ZKPoKManager: {}, CiphertextStorage: {}",
            self.conf, self.zkpok_manager_address, self.ciphertext_storage_address);

        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.conf.database_pool_size)
            .connect(&self.conf.database_url)
            .await?;
        let mut join_set = JoinSet::new();

        for op in self.operations.clone() {
            let op_channel = op.channel().to_owned();
            let db_pool = db_pool.clone();
            let token = self.cancel_token.clone();
            let db_polling_interval_secs = self.conf.db_polling_interval_secs;
            join_set.spawn({
                let sender = self.clone();
                info!(target: TXN_SENDER_TARGET, "Spawning operation loop {}", op_channel);
                async move {
                    let mut sleep_duration = sender.conf.error_sleep_initial_secs as u64;
                    let mut listener = PgListener::connect_with(&db_pool).await?;
                    listener.listen(&op_channel).await?;
                    loop {
                        if token.is_cancelled() {
                            info!(target: TXN_SENDER_TARGET, "Operation {} stopping", op_channel);
                            break;
                        }

                        match op.execute(&db_pool).await {
                            Err(e) => {
                                error!(target: TXN_SENDER_TARGET,
                                    "Operation {} error: {}. Retrying after {} seconds",
                                    op_channel, e, sleep_duration);
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
                                        info!(target: TXN_SENDER_TARGET, "Operation {} stopping", op_channel);
                                        break;
                                    }
                                    n = notification => {
                                        match n {
                                            Ok(Some(_)) => {
                                                debug!(target: TXN_SENDER_TARGET,
                                                    "Operation {} received notification, rechecking for work", op_channel);
                                            },
                                            Ok(None) => {
                                                debug!(target: TXN_SENDER_TARGET,
                                                    "Operation {} received empty notification, sleeping for {} seconds",
                                                    op_channel, sleep_duration);
                                                sender.sleep_with_backoff(&mut sleep_duration).await;
                                            }
                                            Err(e) => {
                                                error!(target: TXN_SENDER_TARGET,
                                                    "Operation {} notification error: {}, sleeping for {} seconds",
                                                    op_channel, e, sleep_duration);
                                                sender.sleep_with_backoff(&mut sleep_duration).await;
                                            }
                                        }
                                    }
                                    _ = tokio::time::sleep(Duration::from_secs(db_polling_interval_secs.into())) => {
                                        debug!(target: TXN_SENDER_TARGET,
                                            "Operation {} timeout reached, rechecking for work", op_channel);
                                    }
                                }
                            }
                        }
                    }
                    Ok::<(), anyhow::Error>(())
                }
            });
        }

        while let Some(res) = join_set.join_next().await {
            res??;
        }
        Ok(())
    }

    fn reset_sleep_duration(&self, sleep_duration: &mut u64) {
        *sleep_duration = self.conf.error_sleep_initial_secs as u64;
    }

    async fn sleep_with_backoff(&self, sleep_duration: &mut u64) {
        tokio::time::sleep(Duration::from_secs(*sleep_duration)).await;
        *sleep_duration = std::cmp::min(*sleep_duration * 2, self.conf.error_sleep_max_secs as u64);
    }
}
