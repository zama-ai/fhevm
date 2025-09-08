use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag, network::Ethereum, primitives::Address, providers::Provider,
    rpc::types::Log, sol,
};
use futures_util::{future::join_all, StreamExt};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::aws_s3::{download_key_from_s3, AwsS3Interface};
use crate::database::{tenant_id, update_tenant_crs, update_tenant_key};
use crate::digest::{digest_crs, digest_key};
use crate::sks_key::extract_server_key_without_ns;
use crate::{ChainId, ConfigSettings, HealthStatus, KeyId, KeyType};

sol!(
    #[sol(rpc)]
    InputVerification,
    "./../../../gateway-contracts/artifacts/contracts/InputVerification.sol/InputVerification.json"
);

sol!(
    #[sol(rpc)]
    KMSManagement,
    "./../../../gateway-contracts/artifacts/contracts/KMSManagement.sol/KMSManagement.json"
);

pub struct GatewayListener<P: Provider<Ethereum> + Clone + 'static, A: AwsS3Interface> {
    input_verification_address: Address,
    kms_management_address: Address,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    provider: P,
    aws_s3_client: A,
}

impl<P: Provider<Ethereum> + Clone + 'static, A: AwsS3Interface> GatewayListener<P, A> {
    pub fn new(
        input_verification_address: Address,
        kms_management_address: Address,
        conf: ConfigSettings,
        cancel_token: CancellationToken,
        provider: P,
        aws_client: A,
    ) -> Self {
        GatewayListener {
            input_verification_address,
            kms_management_address,
            conf,
            cancel_token,
            provider,
            aws_s3_client: aws_client,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!(
            conf = ?self.conf,
            self.input_verification_address = %self.input_verification_address,
            self.kms_management_address = %self.kms_management_address,
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

    pub async fn run_loop(
        &self,
        db_pool: &Pool<Postgres>,
        sleep_duration: &mut u64,
    ) -> anyhow::Result<()> {
        let input_verification =
            InputVerification::new(self.input_verification_address, &self.provider);

        let kms_management = KMSManagement::new(self.kms_management_address, &self.provider);

        let mut from_block = self.get_last_block_num(db_pool).await?;
        let host_chain_id = self.conf.host_chain_id;

        // We call `from_block` here, but we expect that most nodes will not honour it. That doesn't lead to issues as described below.
        //
        // We assume that the provider will reconnect internally if the connection is lost and stream.next() will eventually return successfully.
        // We ensure that by requiring the `provider_max_retries` and `provider_retry_interval` to be set to high enough values in the configuration s.t. retry is essentially infinite.
        //
        // That might lead to skipped events, but that is acceptable for input verification requests as the client will eventually retry.
        // Furthermore, replaying old input verification requests is unnecessary as input verification is a synchronous request/response interaction on the client side.
        // Finally, no data on the GW will be left in an inconsistent state.
        let mut verify_proof_request = input_verification
            .VerifyProofRequest_filter()
            .from_block(from_block)
            .subscribe()
            .await?
            .into_stream()
            .fuse();
        info!("Subscribed to InputVerification.VerifyProofRequest events");
        let mut activate_key = kms_management
            .ActivateKey_filter()
            .from_block(from_block)
            .subscribe()
            .await?
            .into_stream()
            .fuse();
        info!("Subscribed to KMSManagement.ActivateKeyRequest events");
        let mut activate_crs = kms_management
            .ActivateCrs_filter()
            .from_block(from_block)
            .subscribe()
            .await?
            .into_stream()
            .fuse();
        info!("Subscribed to KMSManagement.ActivateKeyRequest events");
        loop {
            tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    break;
                }
                item = verify_proof_request.next() => {
                    let Some(item) = item else {
                        error!("Block stream closed");
                        return Err(anyhow::anyhow!("Block stream closed"));
                    };
                    let (request, log) = item?;
                    self.verify_proof_request(db_pool, &mut from_block, request, log).await?;
                }
                item = activate_key.next() => {
                    let Some(item) = item else {
                        error!("Block stream closed");
                        return Err(anyhow::anyhow!("Block stream closed"));
                    };
                    let (request, _log) = item?;
                    self.activate_key(db_pool, request, &self.aws_s3_client, host_chain_id).await?;
                    info!("ActivateKey event successful");
                }
                item = activate_crs.next() => {
                    let Some(item) = item else {
                        error!("Block stream closed");
                        return Err(anyhow::anyhow!("Block stream closed"));
                    };
                    let (request, _log) = item?;
                    self.activate_crs(db_pool, request, &self.aws_s3_client, host_chain_id).await?;
                    info!("ActivateCrs event successful");
                }
            }
            // Reset sleep duration on successful iteration.
            self.reset_sleep_duration(sleep_duration);
        }
        Ok(())
    }

    async fn verify_proof_request(
        &self,
        db_pool: &Pool<Postgres>,
        from_block: &mut BlockNumberOrTag,
        request: InputVerification::VerifyProofRequest,
        log: Log,
    ) -> anyhow::Result<()> {
        info!(zk_proof_id = %request.zkProofId, "Received ZK proof request event");
        self.update_last_block_num(db_pool, from_block, &log)
            .await?;
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
        Ok(())
    }

    async fn activate_key(
        &self,
        db_pool: &Pool<Postgres>,
        request: KMSManagement::ActivateKey,
        s3_client: &A,
        host_chain_id: ChainId,
    ) -> anyhow::Result<()> {
        let key_id: KeyId = request.keyId;
        let s3_bucket_urls = request.kmsNodeStorageUrls;
        let digests = request.keyDigests;
        info!(
            key_id = key_id.to_string(),
            bucket_urls = ?s3_bucket_urls,
            "Received ActivateKey event"
        );
        // Download keys from S3
        let mut downloads = vec![];
        let mut key_types = vec![];
        for (i_key, key_digest) in digests.iter().enumerate() {
            let key_type: KeyType = key_digest.keyType.try_into()?;
            let key_type_path: &str = to_bucket_key_prefix(key_type);
            key_types.push(key_type);
            let key_id_no_0x = key_id_to_key_bucket(key_id);
            let key_path = format!("{key_type_path}/{key_id_no_0x}");
            let download = download_key_from_s3(s3_client, &s3_bucket_urls, key_path, i_key);
            downloads.push(download);
        }
        let mut downloads = join_all(downloads).await;
        let mut keys_bytes = vec![];
        for (i_key, bytes) in downloads.drain(..).enumerate() {
            let Ok(bytes) = bytes else {
                error!(key_id = ?key_id, key = i_key, "Failed to download key, stopping");
                anyhow::bail!("Failed to download key id:{key_id}, key {}", i_key + 1);
            };
            let download_digest = digest_key(&bytes);
            let expected_digest = digests[i_key].digest.0.as_ref();
            if download_digest != expected_digest {
                error!(key = i_key, download_digest = ?download_digest, expected_digest = ?expected_digest, "Key digest mismatch, stopping");
                anyhow::bail!("Invalid Key digest for key id:{key_id}, key {}", i_key + 1);
            }
            keys_bytes.push(bytes);
        }
        let Some(tenant_id) = tenant_id(db_pool, host_chain_id).await? else {
            error!(host_chain_id, "No tenant found for chain id, stopping");
            anyhow::bail!("No tenant found for chain id {}", host_chain_id);
        };
        let key_id = key_id_to_database_bytes(key_id);
        let mut tx = db_pool.begin().await?;
        for (i_key, key_bytes) in keys_bytes.drain(..).enumerate() {
            let reduced_key_bytes = match key_types[i_key] {
                KeyType::ServerKey => Some(extract_server_key_without_ns(&key_bytes)?),
                KeyType::PublicKey => None,
            };
            update_tenant_key(
                &mut tx,
                &key_id,
                key_types[i_key],
                &key_bytes,
                reduced_key_bytes,
                tenant_id,
                host_chain_id,
            )
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn activate_crs(
        &self,
        db_pool: &Pool<Postgres>,
        request: KMSManagement::ActivateCrs,
        s3_client: &A,
        host_chain_id: ChainId,
    ) -> anyhow::Result<()> {
        let crs_id: KeyId = request.crsId;
        let s3_bucket_urls = request.kmsNodeStorageUrls;
        let digest = request.crsDigest;
        info!(
            key_id = crs_id.to_string(),
            bucket_urls = ?s3_bucket_urls,
            "Received ActivateCrs event"
        );
        // Download keys from S3
        let crs_id_no_0x = key_id_to_key_bucket(crs_id);
        let key_path = format!("PUB/CRS/{crs_id_no_0x}");
        let Ok(bytes) = download_key_from_s3(s3_client, &s3_bucket_urls, key_path, 0).await else {
            error!(key_id = ?crs_id, "Failed to download crs, stopping");
            anyhow::bail!("Failed to download crs key id:{crs_id}");
        };
        let download_digest = digest_crs(&bytes);
        let expected_digest = digest.0.as_ref();
        if download_digest != expected_digest {
            error!(download_digest = ?download_digest, expected_digest = ?expected_digest, "Key digest mismatch, stopping");
            anyhow::bail!("Invalid Key digest for key id:{crs_id}");
        }
        let Some(tenant_id) = tenant_id(db_pool, host_chain_id).await? else {
            error!(host_chain_id, "No tenant found for chain id, stopping");
            anyhow::bail!("No tenant found for chain id {}", host_chain_id);
        };
        let mut tx = db_pool.begin().await?;
        update_tenant_crs(&mut tx, &bytes, tenant_id, host_chain_id).await?;
        tx.commit().await?;
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
        from_block: &mut BlockNumberOrTag,
        log: &Log,
    ) -> anyhow::Result<()> {
        match log.block_number {
            Some(event_block_num) => match *from_block {
                BlockNumberOrTag::Latest => {
                    info!(event_block_num = event_block_num, "Updating from block");
                    *from_block = BlockNumberOrTag::Number(event_block_num);
                }
                BlockNumberOrTag::Number(from_block_num) => {
                    if from_block_num < event_block_num {
                        info!(
                            from_block_num = from_block_num,
                            event_block_num = event_block_num,
                            "Updating from block"
                        );
                        *from_block = BlockNumberOrTag::Number(event_block_num);
                    }
                    return Ok(());
                }
                _ => unreachable!("Unexpected from block type"),
            },
            None => {
                error!("Received an event without a block number, updating from block to latest");
                *from_block = BlockNumberOrTag::Latest;
            }
        };
        info!(last_block_num = ?log.block_number, "Updating last block number");
        sqlx::query!(
            "INSERT into gw_listener_last_block (dummy_id, last_block_num)
            VALUES (true, $1)
            ON CONFLICT (dummy_id) DO UPDATE SET last_block_num = EXCLUDED.last_block_num",
            log.block_number
                .map::<i64, _>(|n| n.try_into().expect("Invalid block number for update"))
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

fn key_id_to_database_bytes(key_id: KeyId) -> [u8; 32] {
    key_id.to_be_bytes()
}

pub fn to_bucket_key_prefix(val: KeyType) -> &'static str {
    match val {
        // TODO: configurable
        KeyType::ServerKey => "PUB/ServerKey",
        KeyType::PublicKey => "PUB/PublicKey",
    }
}

pub fn key_id_to_key_bucket(key_id: KeyId) -> String {
    format!("{:064x}", key_id).to_owned()
}

mod test {
    #[test]
    fn test_key_id_consistency() {
        use super::{key_id_to_database_bytes, key_id_to_key_bucket};
        use alloy::hex;
        use alloy::primitives::U256;

        let key_id = U256::from_limbs([0, 1, 2, u64::MAX]);
        let database_bytes = key_id_to_database_bytes(key_id);
        assert_eq!(
            hex::encode(database_bytes),
            key_id_to_key_bucket(key_id).as_str(),
        )
    }
}
