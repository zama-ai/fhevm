use alloy::{
    network::Ethereum, primitives::Address, providers::Provider, signers::local::PrivateKeySigner,
};
use futures_util::FutureExt;
use sqlx::{postgres::PgListener, Pool, Postgres};
use std::{sync::Arc, time::Duration};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::{nonce_managed_provider::NonceManagedProvider, ops, ConfigSettings, TXN_SENDER_TARGET};

#[derive(Clone)]
pub struct TransactionSender<P: Provider<Ethereum> + Clone + 'static> {
    cancel_token: CancellationToken,
    conf: ConfigSettings,
    operations: Vec<Arc<dyn ops::TransactionOperation<P>>>,
    input_verification_address: Address,
    ciphertext_commits_address: Address,
    db_pool: Pool<Postgres>,
}

impl<P: Provider<Ethereum> + Clone + 'static> TransactionSender<P> {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        input_verification_address: Address,
        ciphertext_commits_address: Address,
        multichain_acl_address: Address,
        signer: PrivateKeySigner,
        provider: NonceManagedProvider<P>,
        cancel_token: CancellationToken,
        conf: ConfigSettings,
        gas: Option<u64>,
    ) -> anyhow::Result<Self> {
        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(conf.database_pool_size)
            .connect(&conf.database_url)
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
            Arc::new(ops::allow_handle::MultichainAclOperation::new(
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
            db_pool,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!(target: TXN_SENDER_TARGET, "Starting Transaction Sender with: {:?}, InputVerification: {}, CiphertextCommits: {}",
            self.conf, self.input_verification_address, self.ciphertext_commits_address);

        let mut join_set = JoinSet::new();

        for op in self.operations.clone() {
            let op_channel = op.channel().to_owned();
            let token = self.cancel_token.clone();
            let db_polling_interval_secs = self.conf.db_polling_interval_secs;
            join_set.spawn({
                let sender = self.clone();
                info!(target: TXN_SENDER_TARGET, "Spawning operation loop {}", op_channel);
                async move {
                    let mut sleep_duration = sender.conf.error_sleep_initial_secs as u64;
                    let mut listener = PgListener::connect_with(&sender.db_pool).await?;
                    listener.listen(&op_channel).await?;
                    loop {
                        if token.is_cancelled() {
                            info!(target: TXN_SENDER_TARGET, "Operation {} stopping", op_channel);
                            break;
                        }

                        match op.execute().await {
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
