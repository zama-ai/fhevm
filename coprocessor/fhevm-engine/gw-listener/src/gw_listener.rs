use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    network::Ethereum,
    primitives::Address,
    providers::Provider,
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        Identity, ProviderBuilder, RootProvider, WsConnect,
    },
    sol,
    transports::http::reqwest::Url,
};
use futures_util::StreamExt;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::ConfigSettings;

sol!(
    #[sol(rpc)]
    InputVerification,
    "artifacts/InputVerification.sol/InputVerification.json"
);

pub trait Builder<P: Provider<Ethereum> + Clone + 'static> {
    fn create_provider(
        &self,
        gw_url: Url,
    ) -> impl std::future::Future<Output = anyhow::Result<P>> + Send;
}

type DefaultProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

#[derive(Debug, Clone, Default)]
pub struct ProviderBuilderImpl;

impl Builder<DefaultProvider> for ProviderBuilderImpl {
    async fn create_provider(&self, gw_url: Url) -> anyhow::Result<DefaultProvider> {
        let provider = ProviderBuilder::new().on_ws(WsConnect::new(gw_url)).await?;
        Ok(provider)
    }
}

pub struct GatewayListener<P: Provider<Ethereum> + Clone + 'static, B: Builder<P>> {
    input_verification_address: Address,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    provider: Option<P>,
    builder: B,
}

impl<P: Provider<Ethereum> + Clone + 'static, B: Builder<P>> GatewayListener<P, B> {
    pub fn new(
        input_verification_address: Address,
        conf: ConfigSettings,
        cancel_token: CancellationToken,
        provider: P,
        builder: B,
    ) -> Self {
        GatewayListener {
            input_verification_address,
            conf,
            cancel_token,
            provider: Some(provider),
            builder,
        }
    }

    pub async fn try_recreate_provider(&mut self) {
        info!(
            "Attempting to create a provider for gw_url: {} ...",
            self.conf.gw_url
        );

        // If the provider creation is successful, replace the existing provider.
        // If it fails, log the error and continue retrying.
        match self.builder.create_provider(self.conf.gw_url.clone()).await {
            Ok(provider) => {
                self.provider.replace(provider);
                info!("Provider created successfully");
            }
            Err(e) => {
                // Continue retrying
                self.provider.take();
                error!("Failed to recreate provider: {:?}", e);
            }
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!(
            "Starting Gateway Listener with: {:?}, InputVerification: {}",
            self.conf, self.input_verification_address
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
                        "Encountered an error: {:?}, retrying in {} seconds",
                        e, sleep_duration
                    );
                    self.sleep_with_backoff(&mut sleep_duration).await;

                    // This will attempt to recreate the provider with the same configuration.
                    // If the attempt times out, it will log the error and continue retrying
                    // after sleep_with_backoff.

                    self.try_recreate_provider().await;
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
        let provider = self
            .provider
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Provider is not initialized"))?;

        let input_verification = InputVerification::new(self.input_verification_address, provider);
        let mut from_block = self.get_last_block_num(db_pool).await?;
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
                        error!( "Event stream closed");
                        return Err(anyhow::anyhow!("Event stream closed"))
                    }
                    let (request, log) = item.unwrap()?;
                    info!( "Received event for ZK proof request ID: {}", request.zkProofId);
                    match log.block_number {
                        Some(event_block_num) => {
                            match from_block {
                                BlockNumberOrTag::Latest => {
                                    info!( "Updating from block from latest to {}", event_block_num);
                                    from_block = BlockNumberOrTag::Number(event_block_num);
                                    self.update_last_block_num(db_pool, Some(event_block_num)).await?;
                                }
                                BlockNumberOrTag::Number(from_block_num) => {
                                    if from_block_num < event_block_num {
                                        info!( "Updating from block from {} to {}", from_block_num, event_block_num);
                                        from_block = BlockNumberOrTag::Number(event_block_num);
                                        self.update_last_block_num(db_pool, Some(event_block_num)).await?;
                                    }
                                }
                                _ => unreachable!("Unexpected from block type"),
                            }
                        }
                        None => {
                            error!( "Received an event without a block number, updating from block to latest");
                            from_block = BlockNumberOrTag::Latest;
                            self.update_last_block_num(db_pool, None).await?;
                        }
                    }

                    // TODO: check if we can avoid the cast from u256 to i64
                    sqlx::query!(
                        "WITH ins AS (
                            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, input)
                            VALUES ($1, $2, $3, $4, $5)
                            ON CONFLICT(zk_proof_id) DO NOTHING
                        )
                        SELECT pg_notify($6, '')",
                        request.zkProofId.to::<i64>(),
                        request.contractChainId.to::<i32>(),
                        request.contractAddress.to_string(),
                        request.userAddress.to_string(),
                        Some(request.ciphertextWithZKProof.as_ref()),
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
        info!("Updating last block number to: {:?}", block_num);
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
}
