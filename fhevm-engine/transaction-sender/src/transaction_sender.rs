use alloy::{
    network::Ethereum, primitives::Address, providers::Provider, signers::local::PrivateKeySigner,
};
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
}

impl<P: Provider<Ethereum> + Clone + 'static> TransactionSender<P> {
    pub fn new(
        zkpok_manager_address: &Address,
        ciphertext_storage_address: &Address,
        signer: PrivateKeySigner,
        provider: Arc<P>,
        cancel_token: CancellationToken,
        conf: ConfigSettings,
    ) -> Self {
        let operations: Vec<Arc<dyn ops::TransactionOperation<P>>> = vec![
            Arc::new(ops::verify_proofs::VerifyProofsOperation::new(
                *zkpok_manager_address,
                provider.clone(),
                signer.clone(),
                conf.clone(),
            )),
            Arc::new(ops::add_ciphertexts::AddCiphertextsOperation::new(
                *ciphertext_storage_address,
                provider.clone(),
            )),
        ];
        Self {
            cancel_token,
            conf,
            operations,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.conf.db_pool_size)
            .connect(&self.conf.db_url)
            .await?;
        let mut join_set = JoinSet::new();

        for op in self.operations.clone() {
            let op_channel = op.channel().to_owned();
            let db_pool = db_pool.clone();
            let token = self.cancel_token.clone();
            let db_polling_interval_secs = self.conf.db_polling_interval_secs;
            join_set.spawn({
                let sender = self.clone();
                async move {
                    let mut sleep_duration = sender.conf.error_sleep_initial_secs as u64;
                    let mut listener = PgListener::connect_with(&db_pool).await?;
                    listener.listen(&op_channel).await?;
                    loop {
                        match op.execute(&db_pool).await {
                            Err(e) => {
                                error!(target: TXN_SENDER_TARGET,
                                    "Operation {} error: {}. Retrying after {} seconds",
                                    op_channel, e, sleep_duration);
                                sender.sleep_with_backoff(&mut sleep_duration).await;
                            }
                            Ok(true) => {
                                // Maybe we have more work to do, don't wait and immediately run the loop again.
                                sender.reset_sleep_duration(&mut sleep_duration);
                            }
                            Ok(false) => {
                                sender.reset_sleep_duration(&mut sleep_duration);
                                tokio::select! {
                                    _ = token.cancelled() => {
                                        info!(target: TXN_SENDER_TARGET, "Operation {} cancelling", op_channel);
                                        break;
                                    }
                                    notif = listener.try_recv() => {
                                        match notif {
                                            Ok(Some(_)) => {
                                                debug!(target: TXN_SENDER_TARGET,
                                                    "Operation {} received notification, rechecking work", op_channel);
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
                                            "Operation {} timeout reached, rechecking work", op_channel);
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
