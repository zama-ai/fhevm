use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag, network::Ethereum, primitives::Address, providers::Provider, sol,
};
use futures_util::StreamExt;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{ConfigSettings, HealthStatus};

sol!(
    #[sol(rpc)]
    InputVerification,
    "artifacts/InputVerification.sol/InputVerification.json"
);

pub struct GatewayListener<P: Provider<Ethereum> + Clone + 'static> {
    input_verification_address: Address,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    provider: P,
}

impl<P: Provider<Ethereum> + Clone + 'static> GatewayListener<P> {
    pub fn new(
        input_verification_address: Address,
        conf: ConfigSettings,
        cancel_token: CancellationToken,
        provider: P,
    ) -> Self {
        GatewayListener {
            input_verification_address,
            conf,
            cancel_token,
            provider,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!(
            conf = ?self.conf,
            self.input_verification_address = %self.input_verification_address,
            "Starting Gateway Listener",
        );
        let db_pool = PgPoolOptions::new()
            .max_connections(self.conf.database_pool_size)
            .connect(&self.conf.database_url)
            .await?;

        let mut sleep_duration = self.conf.error_sleep_initial_secs as u64;
        loop {
            if self.cancel_token.is_cancelled() {
                info!("Stopping");
                break;
            }

            match self.run_loop(&db_pool, &mut sleep_duration).await {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        error = %e,
                        sleep_duration = %sleep_duration,
                        "Encountered an error, retrying",
                    );
                    self.sleep_with_backoff(&mut sleep_duration).await;
                }
            }
        }
        Ok(())
    }

    async fn run_loop(
        &self,
        db_pool: &Pool<Postgres>,
        sleep_duration: &mut u64,
    ) -> anyhow::Result<()> {
        let input_verification =
            InputVerification::new(self.input_verification_address, &self.provider);

        let mut from_block = self.get_last_block_num(db_pool).await?;

        // We call `from_block` here, but we expect that most nodes will not honour it. That doesn't lead to issues as described below.
        //
        // We assume that the provider will reconnect internally if the connection is lost and stream.next() will eventually return successfully.
        // We ensure that by requiring the `provider_max_retries` and `provider_retry_interval` to be set to high enough values in the configuration s.t. retry is essentially infinite.
        //
        // That might lead to skipped events, but that is acceptable for input verification requests as the client will eventually retry.
        // Furthermore, replaying old input verification requests is unnecessary as input verification is a synchronous request/response interaction on the client side.
        // Finally, no data on the GW will be left in an inconsistent state.
        let filter = input_verification
            .VerifyProofRequest_filter()
            .from_block(from_block)
            .subscribe()
            .await?;
        info!("Subscribed to InputVerification.VerifyProofRequest events");
        let mut stream = filter.into_stream().fuse();
        loop {
            tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    break;
                }
                item = stream.next() => {
                    if item.is_none() {
                        error!("Event stream closed");
                        return Err(anyhow::anyhow!("Event stream closed"))
                    }
                    let (request, log) = item.unwrap()?;
                    info!(zk_proof_id = %request.zkProofId, "Received ZK proof request event");
                    match log.block_number {
                        Some(event_block_num) => {
                            match from_block {
                                BlockNumberOrTag::Latest => {
                                    info!(event_block_num = event_block_num, "Updating from block");
                                    from_block = BlockNumberOrTag::Number(event_block_num);
                                    self.update_last_block_num(db_pool, Some(event_block_num)).await?;
                                }
                                BlockNumberOrTag::Number(from_block_num) => {
                                    if from_block_num < event_block_num {
                                        info!(from_block_num = from_block_num, event_block_num = event_block_num, "Updating from block");
                                        from_block = BlockNumberOrTag::Number(event_block_num);
                                        self.update_last_block_num(db_pool, Some(event_block_num)).await?;
                                    }
                                }
                                _ => unreachable!("Unexpected from block type"),
                            }
                        }
                        None => {
                            error!("Received an event without a block number, updating from block to latest");
                            from_block = BlockNumberOrTag::Latest;
                            self.update_last_block_num(db_pool, None).await?;
                        }
                    }

                    // TODO: check if we can avoid the cast from u256 to i64
                    sqlx::query!(
                        "WITH ins AS (
                            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, input, extra_data)
                            VALUES ($1, $2, $3, $4, $5, $6)
                            ON CONFLICT(zk_proof_id) DO NOTHING
                        )
                        SELECT pg_notify($7, '')",
                        request.zkProofId.to::<i64>(),
                        request.contractChainId.to::<i64>(),
                        request.contractAddress.to_string(),
                        request.userAddress.to_string(),
                        Some(request.ciphertextWithZKProof.as_ref()),
                        request.extraData.as_ref(),
                        self.conf.verify_proof_req_db_channel
                    )
                    .execute(db_pool)
                    .await?;
                }
            }
            // Reset sleep duration on successful iteration.
            self.reset_sleep_duration(sleep_duration);
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

    async fn get_last_block_num(
        &self,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<BlockNumberOrTag> {
        let rows = sqlx::query!(
            "SELECT last_block_num
            FROM gw_listener_last_block
            WHERE dummy_id = true"
        )
        .fetch_all(db_pool)
        .await?;
        assert!(
            rows.len() <= 1,
            "Expected at most one row in gw_listener_last_block, found {}",
            rows.len()
        );

        Ok(rows.first().map_or(BlockNumberOrTag::Latest, |row| {
            if let Some(n) = row.last_block_num {
                BlockNumberOrTag::Number(n.try_into().expect("Got an invalid block number"))
            } else {
                BlockNumberOrTag::Latest
            }
        }))
    }

    async fn update_last_block_num(
        &self,
        db_pool: &Pool<Postgres>,
        block_num: Option<u64>,
    ) -> anyhow::Result<()> {
        info!(last_block_num = block_num, "Updating last block number");
        sqlx::query!(
            "INSERT into gw_listener_last_block (dummy_id, last_block_num)
            VALUES (true, $1)
            ON CONFLICT (dummy_id) DO UPDATE SET last_block_num = EXCLUDED.last_block_num",
            block_num.map::<i64, _>(|n| n.try_into().expect("Invalid block number for update"))
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    /// Checks the health of the gateway listener's connections
    pub async fn health_check(&self) -> HealthStatus {
        let mut database_connected = false;
        let mut blockchain_connected = false;
        let mut error_details = Vec::new();

        // Check database connection
        let db_pool_result = PgPoolOptions::new()
            .max_connections(self.conf.database_pool_size)
            .connect(&self.conf.database_url)
            .await;

        match db_pool_result {
            Ok(pool) => {
                // Simple query to verify connection is working
                match sqlx::query("SELECT 1").execute(&pool).await {
                    Ok(_) => {
                        database_connected = true;
                    }
                    Err(e) => {
                        error_details.push(format!("Database query error: {}", e));
                    }
                }
            }
            Err(e) => {
                error_details.push(format!("Database connection error: {}", e));
            }
        }

        // The provider internal retry may last a long time, so we set a timeout
        match tokio::time::timeout(
            self.conf.health_check_timeout,
            self.provider.get_block_number(),
        )
        .await
        {
            Ok(Ok(block_num)) => {
                blockchain_connected = true;
                info!(
                    "Blockchain connection healthy, current block: {}",
                    block_num
                );
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
