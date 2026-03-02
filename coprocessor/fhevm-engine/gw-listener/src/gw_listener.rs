use std::time::Duration;

use alloy::rpc::types::Filter;
use alloy::sol_types::SolEventInterface;
use alloy::{network::Ethereum, primitives::Address, providers::Provider, rpc::types::Log};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::utils::to_hex;
use futures_util::future::join_all;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::database::{insert_crs, insert_key, KeyRecord};

use crate::aws_s3::{download_key_from_s3, AwsS3Interface};
use crate::digest::{digest_crs, digest_key};
use crate::metrics::{
    ACTIVATE_CRS_FAIL_COUNTER, ACTIVATE_CRS_SUCCESS_COUNTER, ACTIVATE_KEY_FAIL_COUNTER,
    ACTIVATE_KEY_SUCCESS_COUNTER, CRS_DIGEST_MISMATCH_COUNTER, GET_BLOCK_NUM_FAIL_COUNTER,
    GET_BLOCK_NUM_SUCCESS_COUNTER, GET_LOGS_FAIL_COUNTER, GET_LOGS_SUCCESS_COUNTER,
    KEY_DIGEST_MISMATCH_COUNTER, VERIFY_PROOF_FAIL_COUNTER, VERIFY_PROOF_SUCCESS_COUNTER,
};
use crate::sks_key::extract_server_key_without_ns;
use crate::ConfigSettings;
use crate::HealthStatus;
use crate::KeyId;
use crate::KeyType;
use fhevm_engine_common::chain_id::ChainId;

use fhevm_gateway_bindings::input_verification::InputVerification;
use fhevm_gateway_bindings::kms_generation::KMSGeneration;

#[derive(Debug)]
struct DigestMismatchError {
    id: String,
}

impl std::fmt::Display for DigestMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Key digest for key ID {}", self.id)
    }
}

impl std::error::Error for DigestMismatchError {}

#[derive(Clone)]
pub struct GatewayListener<
    P: Provider<Ethereum> + Clone + 'static,
    A: AwsS3Interface + Clone + 'static,
> {
    input_verification_address: Address,
    kms_generation_address: Address,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    provider: P,
    aws_s3_client: A,
}

impl<P: Provider<Ethereum> + Clone + 'static, A: AwsS3Interface + Clone + 'static>
    GatewayListener<P, A>
{
    pub fn new(
        input_verification_address: Address,
        kms_generation_address: Address,
        conf: ConfigSettings,
        cancel_token: CancellationToken,
        provider: P,
        aws_client: A,
    ) -> Self {
        GatewayListener {
            input_verification_address,
            kms_generation_address,
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
            self.kms_generation_address = %self.kms_generation_address,
            "Starting Gateway Listener",
        );
        let db_pool = PgPoolOptions::new()
            .max_connections(self.conf.database_pool_size)
            .connect(self.conf.database_url.as_str())
            .await?;

        let get_logs_handle = {
            let s = self.clone();
            let d = db_pool.clone();
            tokio::spawn(async move {
                let mut replay_from_block = s.conf.replay_from_block;
                let mut sleep_duration = s.conf.error_sleep_initial_secs as u64;
                loop {
                    match s
                        .run_get_logs(&d, &mut sleep_duration, &mut replay_from_block)
                        .await
                    {
                        Ok(_) => {
                            info!("run_get_logs() stopped");
                            break;
                        }
                        Err(e) => {
                            error!(error = %e, "run_get_logs() failed");
                            s.sleep_with_backoff(&mut sleep_duration).await;
                        }
                    }
                }
            })
        };

        get_logs_handle.await?;

        Ok(())
    }

    async fn run_get_logs(
        &self,
        db_pool: &Pool<Postgres>,
        sleep_duration: &mut u64,
        replay_from_block: &mut Option<i64>,
    ) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.conf.get_logs_poll_interval);
        let mut last_processed_block_num = self.get_last_processed_block_num(db_pool).await?;
        let mut number_of_last_processed_updates: u64 = 0;
        if let Some(from_block) = *replay_from_block {
            info!(from_block, "Replay starts");
            let from_block = if from_block >= 0 {
                // start from specified block
                from_block
            } else {
                // go N block in past
                let current_block = self.provider.get_block_number().await?;
                current_block as i64 + from_block
            };
            // clipped to positive block number
            // note, we cannot replay block 0
            last_processed_block_num = Some((from_block - 1).try_into().unwrap_or(0));
        }

        loop {
            tokio::select! {
                biased;

                _ = self.cancel_token.cancelled() => {
                    break;
                }

                _ = ticker.tick() => {
                    let current_block = self.provider.get_block_number().await.inspect(|_| {
                        GET_BLOCK_NUM_SUCCESS_COUNTER.inc();
                    }).inspect_err(|_| {
                        GET_BLOCK_NUM_FAIL_COUNTER.inc();
                    })?;

                    let from_block = if let Some(last) = last_processed_block_num {
                        if last >= current_block {
                            if last > current_block {
                                error!(last_processed_block = last, current_block = current_block,
                                    "Unexpectedly, last processed is ahead of current block, skipping this iteration");
                            }
                            continue;
                        }
                        last + 1
                    } else {
                        current_block
                    };

                    let to_block = {
                        let max = from_block.saturating_add(self.conf.get_logs_block_batch_size.saturating_sub(1));
                        std::cmp::min(max, current_block)
                    };

                    let filter = Filter::new()
                        .address(vec![self.kms_generation_address, self.input_verification_address])
                        .from_block(from_block)
                        .to_block(to_block);

                    let mut verify_proof_success = 0;
                    let mut activate_crs_success = 0;
                    let mut crs_digest_mismatch = 0;
                    let mut activate_key_success = 0;
                    let mut key_digest_mismatch = 0;

                    let logs = self.provider.get_logs(&filter).await.inspect(|_| {
                        GET_LOGS_SUCCESS_COUNTER.inc();
                    }).inspect_err(|_| {
                        GET_LOGS_FAIL_COUNTER.inc();
                    })?;
                    if replay_from_block.is_some() && from_block < current_block {
                        info!(from_block, to_block, nb_events=logs.len(), "Replay get_logs");
                    }
                    for log in logs {
                        if log.address() == self.input_verification_address {
                            if replay_from_block.is_some() && self.conf.replay_skip_verify_proof {
                                debug!(log = ?log, "Skipping VerifyProofRequest during replay");
                                continue;
                            }
                            if let Ok(event) = InputVerification::InputVerificationEvents::decode_log(&log.inner) {
                                match event.data {
                                    InputVerification::InputVerificationEvents::VerifyProofRequest(request) => {
                                        self.verify_proof_request(db_pool, request, log.clone()).await.
                                            inspect(|_| {
                                                verify_proof_success += 1;
                                            }).inspect_err(|e| {
                                                error!(error = %e, "VerifyProofRequest processing failed");
                                                VERIFY_PROOF_FAIL_COUNTER.inc();
                                        })?;
                                    },
                                    _ => {
                                        error!(log = ?log, "Unknown InputVerification event");
                                    }
                                }
                            } else {
                                error!(log = ?log, "Failed to decode InputVerification event log");
                            }
                        } else if log.address() == self.kms_generation_address {
                            if let Ok(event) = KMSGeneration::KMSGenerationEvents::decode_log(&log.inner) {
                                match event.data {
                                    KMSGeneration::KMSGenerationEvents::ActivateCrs(a) => {
                                        // IMPORTANT: If we ignore the event due to digest mismatch, this might lead to inconsistency between coprocessors.
                                        // We choose to ignore the event and then manually fix if it happens.
                                        match self.activate_crs(db_pool, a, &self.aws_s3_client).await {
                                            Ok(_) => {
                                                activate_crs_success += 1;
                                                info!("ActivateCrs event successful");
                                            },
                                            Err(e) if e.is::<DigestMismatchError>() => {
                                                crs_digest_mismatch += 1;
                                                error!(error = %e, "CRS digest mismatch, ignoring event");
                                            }
                                            Err(e) => {
                                                ACTIVATE_CRS_FAIL_COUNTER.inc();
                                                return Err(e);
                                            }
                                        }
                                    },
                                    // IMPORTANT: See comment above.
                                    KMSGeneration::KMSGenerationEvents::ActivateKey(a) => {
                                        match self.activate_key(db_pool, a, &self.aws_s3_client).await {
                                            Ok(_) => {
                                                activate_key_success += 1;
                                                info!("ActivateKey event successful");
                                            }
                                            Err(e) if e.is::<DigestMismatchError>() => {
                                                key_digest_mismatch += 1;
                                                error!(error = %e, "Key digest mismatch, ignoring event");
                                            }
                                            Err(e) => {
                                                ACTIVATE_KEY_FAIL_COUNTER.inc();
                                                return Err(e);
                                            }
                                        };
                                    },
                                    _ => {
                                        error!(log = ?log, "Unknown KMSGeneration event")
                                    }
                                }
                            } else {
                                error!(log = ?log, "Failed to decode KMSGeneration event log");
                            }
                        } else {
                            error!(log = ?log, "Unexpected log address");
                        }
                    }
                    last_processed_block_num = Some(to_block);
                    if replay_from_block.is_some() {
                        if to_block == current_block {
                            info!("Replay finished");
                            *replay_from_block = None;
                        } else {
                            // if an error happens replay will restart here
                            *replay_from_block = Some(to_block as i64 + 1);
                            info!(replay_from_block, "Replay continues");
                        }
                    }
                    self.update_last_block_num(db_pool, last_processed_block_num, &mut number_of_last_processed_updates).await?;

                    // Update metrics only after a successful DB update as we don't want to consider events that will be processed again
                    // if the DB update fails.
                    VERIFY_PROOF_SUCCESS_COUNTER.inc_by(verify_proof_success);
                    ACTIVATE_CRS_SUCCESS_COUNTER.inc_by(activate_crs_success);
                    CRS_DIGEST_MISMATCH_COUNTER.inc_by(crs_digest_mismatch);
                    ACTIVATE_KEY_SUCCESS_COUNTER.inc_by(activate_key_success);
                    KEY_DIGEST_MISMATCH_COUNTER.inc_by(key_digest_mismatch);

                    if to_block < current_block {
                        debug!(to_block = to_block,
                            current_block = current_block,
                            get_logs_poll_interval = ?self.conf.get_logs_poll_interval,
                            "More blocks available, not waiting for poll interval");
                        ticker.reset_immediately();
                    }
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
        request: InputVerification::VerifyProofRequest,
        log: Log,
    ) -> anyhow::Result<()> {
        let transaction_id = log.transaction_hash.map(|h| h.to_vec()).unwrap_or_default();
        info!(zk_proof_id = %request.zkProofId, tid = %to_hex(&transaction_id), "Received ZK proof request event");

        let chain_id = ChainId::try_from(request.contractChainId)?;

        let _ = telemetry::try_begin_transaction(
            db_pool,
            chain_id,
            &transaction_id,
            log.block_number.unwrap_or_default(),
        )
        .await;

        // TODO: check if we can avoid the cast from u256 to i64
        sqlx::query!(
            "WITH ins AS (
                INSERT INTO verify_proofs (zk_proof_id, host_chain_id, contract_address, user_address, input, extra_data, transaction_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT(zk_proof_id) DO NOTHING
            )
            SELECT pg_notify($8, '')",
            request.zkProofId.to::<i64>(),
            chain_id.as_i64(),
            request.contractAddress.to_string(),
            request.userAddress.to_string(),
            Some(request.ciphertextWithZKProof.as_ref()),
            request.extraData.as_ref(),
            transaction_id,
            self.conf.verify_proof_req_db_channel
        )
        .execute(db_pool)
        .await.
        inspect(|_| {
            VERIFY_PROOF_SUCCESS_COUNTER.inc();
        }).inspect_err(|_| {
            VERIFY_PROOF_FAIL_COUNTER.inc();
        })?;
        Ok(())
    }

    async fn activate_key(
        &self,
        db_pool: &Pool<Postgres>,
        request: KMSGeneration::ActivateKey,
        s3_client: &A,
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
            let key_type_path: &str = to_key_prefix(key_type);
            key_types.push(key_type);
            let key_id_no_0x = key_id_to_aws_key(key_id);
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
                return Err(DigestMismatchError {
                    id: key_id.to_string(),
                }
                .into());
            }
            keys_bytes.push(bytes);
        }
        let key_id_bytes = key_id_to_database_bytes(key_id);
        let mut tx = db_pool.begin().await?;
        let mut key_record = KeyRecord {
            key_id_gw: key_id_bytes.into(),
            ..Default::default()
        };
        for (i_key, key_bytes) in keys_bytes.drain(..).enumerate() {
            match key_types[i_key] {
                KeyType::ServerKey => {
                    key_record.sks_key = extract_server_key_without_ns(&key_bytes)?.into();
                    key_record.sns_pk = key_bytes;
                }
                KeyType::PublicKey => {
                    key_record.pks_key = key_bytes;
                }
            }
        }
        if !key_record.is_valid() {
            anyhow::bail!("Incomplete key record for key id:{key_id}");
        }
        insert_key(&mut tx, &key_record).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn activate_crs(
        &self,
        db_pool: &Pool<Postgres>,
        request: KMSGeneration::ActivateCrs,
        s3_client: &A,
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
        let crs_id_no_0x = key_id_to_aws_key(crs_id);
        let key_path_suffix = format!("/CRS/{crs_id_no_0x}");
        let Ok(bytes) = download_key_from_s3(s3_client, &s3_bucket_urls, key_path_suffix, 0).await
        else {
            error!(key_id = ?crs_id, "Failed to download crs, stopping");
            anyhow::bail!("Failed to download crs key id:{crs_id}");
        };
        let download_digest = digest_crs(&bytes);
        let expected_digest = digest.0.as_ref();
        if download_digest != expected_digest {
            error!(download_digest = ?download_digest, expected_digest = ?expected_digest, "Key digest mismatch, stopping");
            return Err(DigestMismatchError {
                id: crs_id.to_string(),
            }
            .into());
        }
        let mut tx = db_pool.begin().await?;
        insert_crs(&mut tx, &key_id_to_database_bytes(crs_id), &bytes).await?;
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

    async fn get_last_processed_block_num(
        &self,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<Option<u64>> {
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

        Ok(rows.first().and_then(|row| {
            row.last_block_num
                .map(|n| n.try_into().expect("Got an invalid block number"))
        }))
    }

    async fn update_last_block_num(
        &self,
        db_pool: &Pool<Postgres>,
        last_block: Option<u64>,
        number_of_last_processed_updates: &mut u64,
    ) -> anyhow::Result<()> {
        let last_block = last_block.map(i64::try_from).transpose()?;
        sqlx::query!(
            "INSERT into gw_listener_last_block (dummy_id, last_block_num)
            VALUES (true, $1)
            ON CONFLICT (dummy_id) DO UPDATE SET last_block_num = EXCLUDED.last_block_num",
            last_block
        )
        .execute(db_pool)
        .await?;

        *number_of_last_processed_updates += 1;
        if (*number_of_last_processed_updates)
            .is_multiple_of(self.conf.log_last_processed_every_number_of_updates)
        {
            info!(
                last_block = last_block,
                "Updated last processed block number"
            );
        }
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
            .connect(self.conf.database_url.as_str())
            .await;

        match db_pool_result {
            Ok(pool) => {
                // Simple query to verify connection is working
                match sqlx::query("SELECT 1").execute(&pool).await {
                    Ok(_) => {
                        database_connected = true;
                        info!("Database connection healthy");
                    }
                    Err(e) => {
                        error!(error = %e, "Database check failed");
                        error_details.push(format!("Database query error: {}", e));
                    }
                }
            }
            Err(e) => {
                error!(error = %e, "Database connection error");
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
                error!(error = %e, "Blockchain connection error");
                error_details.push(format!("Blockchain connection error: {}", e));
            }
            Err(_) => {
                error!("Blockchain connection timeout");
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

pub fn key_id_to_database_bytes(key_id: KeyId) -> [u8; 32] {
    key_id.to_be_bytes()
}

pub fn to_key_prefix(val: KeyType) -> &'static str {
    match val {
        KeyType::ServerKey => "/ServerKey",
        KeyType::PublicKey => "/PublicKey",
    }
}

pub fn key_id_to_aws_key(key_id: KeyId) -> String {
    format!("{:064x}", key_id).to_owned()
}

mod test {
    #[test]
    fn test_key_id_consistency() {
        use super::{key_id_to_aws_key, key_id_to_database_bytes};
        use alloy::hex;
        use alloy::primitives::U256;

        let key_id = U256::from_limbs([0, 1, 2, u64::MAX]);
        let database_bytes = key_id_to_database_bytes(key_id);
        assert_eq!(
            hex::encode(database_bytes),
            key_id_to_aws_key(key_id).as_str(),
        )
    }
}
