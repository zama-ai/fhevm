use std::ops::DerefMut;
use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    network::Ethereum,
    primitives::{Address, Uint},
    providers::Provider,
    rpc::types::Log,
    sol,
};
use aws_config::{retry::RetryConfig, timeout::TimeoutConfig, BehaviorVersion};
use aws_sdk_s3::{config::Builder, Client};
use futures_util::{future::join_all, StreamExt};
use sha3::Digest;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Transaction};
use tokio_util::{bytes, sync::CancellationToken};
use tracing::{error, info};

use crate::{ConfigSettings, HealthStatus};

sol!(
    #[sol(rpc)]
    InputVerification,
    "artifacts/InputVerification.sol/InputVerification.json"
);

sol!(
    #[sol(rpc)]
    KmsManagement,
    "artifacts/KmsManagement.sol/KmsManagement.json"
);

type ChainId = u64;
type KeyId = Uint<256, 4>;
type TenantId = u64;

#[derive(Clone, Copy, Debug)]
enum KeyType {
    ServerKey = 0,
    PublicKey = 1,
}

impl From<KeyType> for &'static str {
    fn from(val: KeyType) -> Self {
        match val {
            KeyType::ServerKey => "ServerKey",
            KeyType::PublicKey => "PublicKey",
        }
    }
}

impl TryFrom<u8> for KeyType {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> anyhow::Result<KeyType> {
        match value {
            0 => Ok(KeyType::ServerKey),
            1 => Ok(KeyType::PublicKey),
            _ => Err(anyhow::anyhow!("Invalid KeyType")),
        }
    }
}

struct TenantInfo {
    tenant_id: Option<TenantId>,
    chain_id: ChainId,
}

pub struct GatewayListener<P: Provider<Ethereum> + Clone + 'static> {
    input_verification_address: Address,
    kms_management_address: Address,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    provider: P,
}

impl<P: Provider<Ethereum> + Clone + 'static> GatewayListener<P> {
    pub fn new(
        input_verification_address: Address,
        kms_management_address: Address,
        conf: ConfigSettings,
        cancel_token: CancellationToken,
        provider: P,
    ) -> Self {
        GatewayListener {
            input_verification_address,
            kms_management_address,
            conf,
            cancel_token,
            provider,
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

    async fn run_loop(
        &self,
        db_pool: &Pool<Postgres>,
        sleep_duration: &mut u64,
    ) -> anyhow::Result<()> {
        let s3_client = create_s3_client(&S3Policy::DEFAULT).await;
        let input_verification =
            InputVerification::new(self.input_verification_address, &self.provider);

        let kms_management = KmsManagement::new(self.kms_management_address, &self.provider);

        let mut from_block = self.get_last_block_num(db_pool).await?;
        let chain_id = self.provider.get_chain_id().await?;
        let tenant_id = tenant_id(db_pool, chain_id).await?;
        let tenant_info = TenantInfo {
            tenant_id,
            chain_id,
        };

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
        info!("Subscribed to KmsManagement.ActivateKeyRequest events");
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
                    self.activate_key(db_pool, request, &s3_client, &tenant_info).await?;
                    info!("Received ActivateKey event");
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
        request: KmsManagement::ActivateKey,
        s3_client: &Client,
        tenant_info: &TenantInfo,
    ) -> anyhow::Result<()> {
        let key_id: KeyId = request.keyId;
        let s3_bucket_urls = request.kmsNodeS3BucketUrls;
        let digests = request.keyDigests;
        info!(
            key_id = key_id.to_string(),
            nb_s3_bucket_urls = s3_bucket_urls.len(),
            "Received ActivateKey event"
        );
        // Download keys from S3
        let mut downloads = vec![];
        let mut key_types = vec![];
        for (i_key, key_digest) in digests.iter().enumerate() {
            let key_type: KeyType = key_digest.keyType.try_into()?;
            key_types.push(key_type);
            let key_type_path: &str = key_type.into();
            info!(key_id = ?key_id, key_type = ?key_type, key = i_key, "Downloading key");
            let key_path = format!("{key_type_path}/{key_id}");
            let download = download_key_from_s3(s3_client, &s3_bucket_urls, key_path, i_key);
            downloads.push(download);
        }
        let mut downloads = join_all(downloads).await;
        let mut keys_bytes = vec![];
        for (i_key, bytes) in downloads.drain(..).enumerate() {
            // TODO: verify digests algo is the good one
            let Ok(bytes) = bytes else {
                error!(key_id = ?key_id, key = i_key, "Failed to download key, stopping");
                anyhow::bail!("Failed to download key id:{key_id}, key {}", i_key + 1);
            };
            let download_digest = sha3::Keccak256::digest(&bytes);
            if download_digest.as_slice() != digests[i_key].digest.as_ref() {
                error!(key = i_key, "Key digest mismatch, stopping");
                anyhow::bail!("Invalid Key digest for key id:{key_id}, key {}", i_key + 1);
            }
            keys_bytes.push(bytes);
        }
        let Some(tenant_id) = tenant_info.tenant_id else {
            error!(
                chain_id = tenant_info.chain_id,
                "No tenant found for chain id, stopping"
            );
            anyhow::bail!("No tenant found for chain id {}", tenant_info.chain_id);
        };
        let mut tx = db_pool.begin().await?;
        let key_id = key_id_to_bytes(key_id);
        for (i_key, key_bytes) in keys_bytes.drain(..).enumerate() {
            update_tenant_key(
                &mut tx,
                &key_id,
                key_types[i_key],
                &key_bytes,
                tenant_id,
                tenant_info.chain_id,
            )
            .await?;
        }
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

#[derive(Clone, Debug, Default)]
pub struct S3Policy {
    pub max_attempt: u32,
    pub max_backoff: Duration,
    pub max_retries_timeout: Duration,
    pub recheck_duration: Duration,
    pub regular_recheck_duration: Duration,
    pub connect_timeout: Duration,
}

impl S3Policy {
    const DEFAULT: Self = Self {
        max_attempt: 10,
        max_backoff: Duration::from_secs(20),
        max_retries_timeout: Duration::from_secs(300),
        recheck_duration: Duration::from_secs(10),
        regular_recheck_duration: Duration::from_secs(300),
        connect_timeout: Duration::from_secs(10),
    };
}

pub async fn create_s3_client(retry_policy: &S3Policy) -> aws_sdk_s3::Client {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let timeout_config = TimeoutConfig::builder()
        .connect_timeout(retry_policy.connect_timeout)
        .operation_attempt_timeout(retry_policy.max_retries_timeout)
        .build();

    let retry_config = RetryConfig::standard()
        .with_max_attempts(retry_policy.max_attempt)
        .with_max_backoff(retry_policy.max_backoff);

    let config = Builder::from(&sdk_config)
        .timeout_config(timeout_config)
        .retry_config(retry_config)
        .build();

    Client::from_conf(config)
}

async fn download_key_from_s3(
    s3_client: &Client,
    s3_bucket_urls: &[String],
    key_path: String,
    offset_bucket: usize, // to not ask the same bucket first
) -> anyhow::Result<bytes::Bytes> {
    for i_s3_bucket_url in 0..s3_bucket_urls.len() {
        // ask different order per key
        let new_index = (i_s3_bucket_url + offset_bucket) % s3_bucket_urls.len();
        let s3_bucket_url = &s3_bucket_urls[new_index];
        info!(key_path, "Downloading");
        let result = s3_client
            .get_object()
            .bucket(s3_bucket_url)
            .key(&key_path)
            .send()
            .await;
        let Ok(result) = result else {
            continue;
        };
        return Ok(result.body.collect().await?.into_bytes());
    }
    error!(key_path, "Failed to download key from all S3 buckets");
    anyhow::bail!("Failed to download key {key_path} from all S3 buckets");
}

async fn tenant_id(db_pool: &Pool<Postgres>, chain_id: u64) -> anyhow::Result<Option<TenantId>> {
    let rows = sqlx::query!(
        "SELECT tenant_id FROM tenants WHERE chain_id = $1",
        chain_id as i64
    )
    .fetch_all(db_pool)
    .await?;
    if rows.len() > 1 {
        anyhow::bail!("Multiple tenants found for chain_id {chain_id}");
    } else if rows.is_empty() {
        return Ok(None);
    }
    Ok(Some(rows[0].tenant_id as TenantId))
}

async fn update_tenant_key(
    tx: &mut Transaction<'_, Postgres>,
    key_id: &[u8],
    key_type: KeyType,
    key_bytes: &[u8],
    tenant_id: TenantId,
    chain_id: ChainId,
) -> anyhow::Result<()> {
    let query = match key_type {
        KeyType::ServerKey => {
            sqlx::query!(
                "UPDATE tenants
                SET
                    sks_key = $1,
                    key_id = $2
                WHERE tenant_id = $3 AND chain_id = $4",
                key_bytes,
                key_id,
                tenant_id as i32,
                chain_id as i64,
            )
        }
        KeyType::PublicKey => {
            sqlx::query!(
                "UPDATE tenants
                SET
                    pks_key = $1,
                    key_id = $2
                WHERE tenant_id = $3 AND chain_id = $4",
                key_bytes,
                key_id,
                tenant_id as i32,
                chain_id as i64,
            )
        }
    };
    query.execute(tx.deref_mut()).await?;
    Ok(())
}

fn key_id_to_bytes(key_id: KeyId) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let limbs = key_id.as_limbs();
    for (i, limb) in limbs.iter().enumerate() {
        let limb_bytes = limb.to_be_bytes();
        bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb_bytes);
    }
    bytes
}
