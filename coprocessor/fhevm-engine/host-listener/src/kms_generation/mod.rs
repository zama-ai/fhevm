use std::time::Duration;

use alloy::primitives::{Address, Uint};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEventInterface;
use anyhow::anyhow;
use fhevm_engine_common::chain_id::ChainId;
use sqlx::{Pool, Postgres, Transaction};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::contracts::KMSGeneration;
use crate::kms_generation::aws_s3::{download_key_from_s3, AwsS3Interface};
use crate::kms_generation::database::{
    insert_crs_activation_event_tx, insert_key_activation_event_tx,
    list_finalized_pending_crs_activations,
    list_finalized_pending_key_activations,
    mark_crs_activation_digest_mismatch, mark_crs_activation_failed,
    mark_crs_activation_invalid_event, mark_key_activation_digest_mismatch,
    mark_key_activation_failed, mark_key_activation_invalid_event,
    materialize_crs_activation_tx, materialize_key_activation_tx, KeyRecord,
    PreparedCrsActivation, PreparedKeyActivation, StagedCrsActivation,
    StagedKeyActivation, StoredKeyDigest,
};
use crate::kms_generation::digest::{digest_crs, digest_key};
use crate::kms_generation::metrics::{
    ACTIVATE_CRS_FAIL_COUNTER, ACTIVATE_CRS_SUCCESS_COUNTER,
    ACTIVATE_KEY_FAIL_COUNTER, ACTIVATE_KEY_SUCCESS_COUNTER,
    CRS_DIGEST_MISMATCH_COUNTER, KEY_DIGEST_MISMATCH_COUNTER,
    KMS_EVENT_DECODE_FAIL_COUNTER, KMS_EVENT_INVALID_COUNTER,
};
use crate::kms_generation::sks_key::extract_server_key_without_ns;

pub mod aws_s3;
pub(crate) mod database;
pub(crate) mod digest;
pub(crate) mod metrics;
pub(crate) mod sks_key;

pub type KeyId = Uint<256, 4>;

const KMS_DOWNLOAD_BATCH_SIZE: i64 = 16;
const KMS_DOWNLOAD_POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, Debug)]
pub enum KeyType {
    ServerKey = 0,
    PublicKey = 1,
}

impl TryFrom<u8> for KeyType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> anyhow::Result<KeyType> {
        match value {
            0 => Ok(KeyType::ServerKey),
            1 => Ok(KeyType::PublicKey),
            _ => Err(anyhow!("Invalid KeyType")),
        }
    }
}

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

#[derive(Debug)]
struct InvalidKmsEventError {
    reason: String,
}

impl std::fmt::Display for InvalidKmsEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for InvalidKmsEventError {}

#[derive(Debug)]
enum PreparedKmsEvent {
    ActivateKey(PreparedKeyActivation),
    ActivateCrs(PreparedCrsActivation),
}

#[derive(Debug, Default)]
pub struct PreparedKmsEvents(Vec<PreparedKmsEvent>);

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

fn key_id_from_database_bytes(key_id: &[u8]) -> anyhow::Result<KeyId> {
    KeyId::try_from_be_slice(key_id)
        .ok_or_else(|| anyhow!("Invalid key ID bytes: {}", key_id.len()))
}

fn invalid_kms_event(reason: impl Into<String>) -> anyhow::Error {
    InvalidKmsEventError {
        reason: reason.into(),
    }
    .into()
}

fn transaction_hash_bytes(log: &Log) -> anyhow::Result<Vec<u8>> {
    log.transaction_hash
        .map(|hash| hash.to_vec())
        .ok_or_else(|| invalid_kms_event("KMS event missing transaction hash"))
}

fn log_index_value(log: &Log) -> anyhow::Result<i64> {
    log.log_index
        .map(|log_index| log_index as i64)
        .ok_or_else(|| invalid_kms_event("KMS event missing log index"))
}

fn encode_activate_key(
    log: &Log,
    request: KMSGeneration::ActivateKey,
) -> anyhow::Result<PreparedKeyActivation> {
    let key_digests = request
        .keyDigests
        .into_iter()
        .map(|key_digest| {
            Ok(StoredKeyDigest {
                key_type: key_digest.keyType,
                digest: key_digest.digest.0.to_vec(),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(PreparedKeyActivation {
        key_id_gw: key_id_to_database_bytes(request.keyId),
        key_digests,
        s3_bucket_urls: request.kmsNodeStorageUrls,
        transaction_hash: transaction_hash_bytes(log)?,
        log_index: log_index_value(log)?,
    })
}

fn encode_activate_crs(
    log: &Log,
    request: KMSGeneration::ActivateCrs,
) -> anyhow::Result<PreparedCrsActivation> {
    Ok(PreparedCrsActivation {
        crs_id: key_id_to_database_bytes(request.crsId),
        crs_digest: request.crsDigest.0.to_vec(),
        s3_bucket_urls: request.kmsNodeStorageUrls,
        transaction_hash: transaction_hash_bytes(log)?,
        log_index: log_index_value(log)?,
    })
}

pub fn encode_kms_generation_events(
    kms_generation_address: Address,
    logs: &[Log],
) -> PreparedKmsEvents {
    let mut prepared_events = PreparedKmsEvents::default();
    for log in logs {
        if log.address() != kms_generation_address {
            continue;
        }
        let event = match KMSGeneration::KMSGenerationEvents::decode_log(
            &log.inner,
        ) {
            Ok(event) => event,
            Err(err) => {
                KMS_EVENT_DECODE_FAIL_COUNTER.inc();
                error!(error = ?err, log = ?log, "Failed to decode KMSGeneration event log");
                continue;
            }
        };
        let prepared_event = match event.data {
            KMSGeneration::KMSGenerationEvents::ActivateCrs(a) => {
                match encode_activate_crs(log, a) {
                    Ok(activation) => {
                        Some(PreparedKmsEvent::ActivateCrs(activation))
                    }
                    Err(err) => {
                        KMS_EVENT_INVALID_COUNTER.inc();
                        error!(error = %err, log = ?log, "Invalid ActivateCrs event while preparing block ingestion");
                        None
                    }
                }
            }
            KMSGeneration::KMSGenerationEvents::ActivateKey(a) => {
                match encode_activate_key(log, a) {
                    Ok(activation) => {
                        Some(PreparedKmsEvent::ActivateKey(activation))
                    }
                    Err(err) => {
                        KMS_EVENT_INVALID_COUNTER.inc();
                        error!(error = %err, log = ?log, "Invalid ActivateKey event while preparing block ingestion");
                        None
                    }
                }
            }
            _ => None,
        };
        if let Some(prepared_event) = prepared_event {
            prepared_events.0.push(prepared_event);
        }
    }
    prepared_events
}

pub async fn insert_prepared_kms_generation_events_tx(
    tx: &mut Transaction<'_, Postgres>,
    prepared_events: &PreparedKmsEvents,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    for event in &prepared_events.0 {
        match event {
            PreparedKmsEvent::ActivateKey(activation) => {
                insert_key_activation_event_tx(
                    tx,
                    activation,
                    chain_id,
                    block_hash,
                    block_number,
                )
                .await?;
            }
            PreparedKmsEvent::ActivateCrs(activation) => {
                insert_crs_activation_event_tx(
                    tx,
                    activation,
                    chain_id,
                    block_hash,
                    block_number,
                )
                .await?;
            }
        }
    }
    Ok(())
}

pub async fn process_kms_generation_logs_tx<
    A: AwsS3Interface + Clone + 'static,
>(
    tx: &mut Transaction<'_, Postgres>,
    kms_generation_address: Address,
    _s3_client: &A,
    logs: &[Log],
    chain_id: ChainId,
    block_hash: &[u8],
) -> anyhow::Result<()> {
    let Some(block_number) = logs.iter().find_map(|log| log.block_number)
    else {
        return Ok(());
    };
    let prepared_events =
        encode_kms_generation_events(kms_generation_address, logs);
    insert_prepared_kms_generation_events_tx(
        tx,
        &prepared_events,
        chain_id,
        block_hash,
        block_number,
    )
    .await?;
    Ok(())
}

pub async fn process_kms_generation_logs<
    A: AwsS3Interface + Clone + 'static,
>(
    db_pool: &sqlx::Pool<Postgres>,
    kms_generation_address: Address,
    s3_client: &A,
    logs: &[Log],
    chain_id: ChainId,
    block_hash: &[u8],
) -> anyhow::Result<()> {
    let mut tx = db_pool.begin().await?;
    process_kms_generation_logs_tx(
        &mut tx,
        kms_generation_address,
        s3_client,
        logs,
        chain_id,
        block_hash,
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn run_kms_downloader<A: AwsS3Interface + Clone + 'static>(
    db_pool: Pool<Postgres>,
    chain_id: ChainId,
    s3_client: A,
    cancel_token: CancellationToken,
) {
    loop {
        let processed = tokio::select! {
            _ = cancel_token.cancelled() => break,
            processed = process_finalized_kms_generation_events_until_idle(&db_pool, chain_id, &s3_client) => processed,
        };
        match processed {
            Ok(0) => {
                tokio::select! {
                    _ = cancel_token.cancelled() => break,
                    _ = tokio::time::sleep(KMS_DOWNLOAD_POLL_INTERVAL) => {}
                }
            }
            Ok(_) => {}
            Err(err) => {
                error!(error = %err, "KMS downloader loop failed");
                tokio::select! {
                    _ = cancel_token.cancelled() => break,
                    _ = tokio::time::sleep(KMS_DOWNLOAD_POLL_INTERVAL) => {}
                }
            }
        }
    }
}

pub async fn process_finalized_kms_generation_events_until_idle<
    A: AwsS3Interface + Clone + 'static,
>(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
    s3_client: &A,
) -> anyhow::Result<u64> {
    let mut total_processed = 0;
    loop {
        let processed = process_finalized_kms_generation_events_batch(
            db_pool, chain_id, s3_client,
        )
        .await?;
        total_processed += processed;
        if processed == 0 {
            return Ok(total_processed);
        }
    }
}

async fn process_finalized_kms_generation_events_batch<
    A: AwsS3Interface + Clone + 'static,
>(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
    s3_client: &A,
) -> anyhow::Result<u64> {
    let key_activations = list_finalized_pending_key_activations(
        db_pool,
        chain_id,
        KMS_DOWNLOAD_BATCH_SIZE,
    )
    .await?;
    let crs_activations = list_finalized_pending_crs_activations(
        db_pool,
        chain_id,
        KMS_DOWNLOAD_BATCH_SIZE,
    )
    .await?;

    let mut processed = 0_u64;
    for activation in key_activations {
        process_key_activation(db_pool, &activation, s3_client).await?;
        processed += 1;
    }
    for activation in crs_activations {
        process_crs_activation(db_pool, &activation, s3_client).await?;
        processed += 1;
    }
    Ok(processed)
}

async fn process_key_activation<A: AwsS3Interface + Clone + 'static>(
    db_pool: &Pool<Postgres>,
    activation: &StagedKeyActivation,
    s3_client: &A,
) -> anyhow::Result<()> {
    match download_key_activation(activation, s3_client).await {
        Ok(Some(key_record)) => {
            let mut tx = db_pool.begin().await?;
            materialize_key_activation_tx(&mut tx, activation, &key_record)
                .await?;
            tx.commit().await?;
            ACTIVATE_KEY_SUCCESS_COUNTER.inc();
            info!(key_id_gw = ?activation.key_id_gw, "ActivateKey event materialized");
        }
        Ok(None) => {
            let err = DigestMismatchError {
                id: key_id_from_database_bytes(&activation.key_id_gw)?
                    .to_string(),
            };
            mark_key_activation_digest_mismatch(
                db_pool,
                activation,
                &err.to_string(),
            )
            .await?;
            KEY_DIGEST_MISMATCH_COUNTER.inc();
            error!(error = %err, key_id_gw = ?activation.key_id_gw, "Key digest mismatch, ignoring event");
        }
        Err(err) if err.is::<InvalidKmsEventError>() => {
            mark_key_activation_invalid_event(
                db_pool,
                activation,
                &err.to_string(),
            )
            .await?;
            ACTIVATE_KEY_FAIL_COUNTER.inc();
            error!(error = %err, key_id_gw = ?activation.key_id_gw, "Invalid ActivateKey event");
        }
        Err(err) => {
            mark_key_activation_failed(db_pool, activation, &err.to_string())
                .await?;
            ACTIVATE_KEY_FAIL_COUNTER.inc();
            warn!(error = %err, key_id_gw = ?activation.key_id_gw, "ActivateKey materialization failed");
        }
    }
    Ok(())
}

async fn process_crs_activation<A: AwsS3Interface + Clone + 'static>(
    db_pool: &Pool<Postgres>,
    activation: &StagedCrsActivation,
    s3_client: &A,
) -> anyhow::Result<()> {
    match download_crs_activation(activation, s3_client).await {
        Ok(Some(crs)) => {
            let mut tx = db_pool.begin().await?;
            materialize_crs_activation_tx(&mut tx, activation, &crs).await?;
            tx.commit().await?;
            ACTIVATE_CRS_SUCCESS_COUNTER.inc();
            info!(crs_id = ?activation.crs_id, "ActivateCrs event materialized");
        }
        Ok(None) => {
            let err = DigestMismatchError {
                id: key_id_from_database_bytes(&activation.crs_id)?.to_string(),
            };
            mark_crs_activation_digest_mismatch(
                db_pool,
                activation,
                &err.to_string(),
            )
            .await?;
            CRS_DIGEST_MISMATCH_COUNTER.inc();
            error!(error = %err, crs_id = ?activation.crs_id, "CRS digest mismatch, ignoring event");
        }
        Err(err) if err.is::<InvalidKmsEventError>() => {
            mark_crs_activation_invalid_event(
                db_pool,
                activation,
                &err.to_string(),
            )
            .await?;
            ACTIVATE_CRS_FAIL_COUNTER.inc();
            error!(error = %err, crs_id = ?activation.crs_id, "Invalid ActivateCrs event");
        }
        Err(err) => {
            mark_crs_activation_failed(db_pool, activation, &err.to_string())
                .await?;
            ACTIVATE_CRS_FAIL_COUNTER.inc();
            warn!(error = %err, crs_id = ?activation.crs_id, "ActivateCrs materialization failed");
        }
    }
    Ok(())
}

async fn download_key_activation<A: AwsS3Interface + Clone + 'static>(
    activation: &StagedKeyActivation,
    s3_client: &A,
) -> anyhow::Result<Option<KeyRecord>> {
    let key_id = key_id_from_database_bytes(&activation.key_id_gw)?;
    info!(
        key_id = key_id.to_string(),
        bucket_urls = ?activation.s3_bucket_urls,
        "Received ActivateKey event"
    );

    let mut downloads = JoinSet::new();
    let mut key_types = vec![];
    for (i_key, key_digest) in activation.key_digests.iter().enumerate() {
        let key_type: KeyType = key_digest
            .key_type
            .try_into()
            .map_err(|_| invalid_kms_event("Invalid KeyType"))?;
        key_types.push(key_type);
        let key_path = format!(
            "{}/{}",
            to_key_prefix(key_type),
            key_id_to_aws_key(key_id)
        );
        let s3_client = s3_client.clone();
        let s3_bucket_urls = activation.s3_bucket_urls.clone();
        downloads.spawn(async move {
            (
                i_key,
                download_key_from_s3(
                    &s3_client,
                    &s3_bucket_urls,
                    key_path,
                    i_key,
                )
                .await,
            )
        });
    }

    let mut keys_bytes = vec![None; activation.key_digests.len()];
    let mut digest_mismatch = false;
    let mut first_error = None;
    while let Some(result) = downloads.join_next().await {
        let (i_key, bytes) = match result {
            Ok(result) => result,
            Err(err) => {
                if first_error.is_none() {
                    first_error =
                        Some(anyhow!("Key download task failed: {err}"));
                }
                continue;
            }
        };
        let bytes = match bytes {
            Ok(bytes) => bytes,
            Err(err) => {
                if first_error.is_none() {
                    first_error = Some(err);
                }
                continue;
            }
        };
        let download_digest = digest_key(&bytes);
        let expected_digest = activation.key_digests[i_key].digest.as_slice();
        if download_digest != expected_digest {
            digest_mismatch = true;
            continue;
        }
        keys_bytes[i_key] = Some(bytes);
    }

    if let Some(err) = first_error {
        return Err(err);
    }
    if digest_mismatch {
        return Ok(None);
    }

    let mut key_record = KeyRecord {
        key_id_gw: activation.key_id_gw.clone().into(),
        ..Default::default()
    };
    for (i_key, key_bytes) in keys_bytes.into_iter().enumerate() {
        let key_bytes = key_bytes.ok_or_else(|| {
            invalid_kms_event(format!(
                "Missing downloaded key part {} for key id:{}",
                i_key, key_id
            ))
        })?;
        match key_types[i_key] {
            KeyType::ServerKey => {
                key_record.sks_key = extract_server_key_without_ns(&key_bytes)
                    .map_err(|err| invalid_kms_event(err.to_string()))?
                    .into();
                key_record.sns_pk = key_bytes;
            }
            KeyType::PublicKey => {
                key_record.pks_key = key_bytes;
            }
        }
    }

    if !key_record.is_valid() {
        return Err(invalid_kms_event(format!(
            "Incomplete key record for key id:{key_id}"
        )));
    }
    Ok(Some(key_record))
}

async fn download_crs_activation<A: AwsS3Interface + Clone + 'static>(
    activation: &StagedCrsActivation,
    s3_client: &A,
) -> anyhow::Result<Option<Vec<u8>>> {
    let crs_id = key_id_from_database_bytes(&activation.crs_id)?;
    info!(
        key_id = crs_id.to_string(),
        bucket_urls = ?activation.s3_bucket_urls,
        "Received ActivateCrs event"
    );

    let key_path_suffix = format!("/CRS/{}", key_id_to_aws_key(crs_id));
    let bytes = download_key_from_s3(
        s3_client,
        &activation.s3_bucket_urls,
        key_path_suffix,
        0,
    )
    .await?;
    if digest_crs(&bytes) != activation.crs_digest.as_slice() {
        return Ok(None);
    }
    Ok(Some(bytes.to_vec()))
}

#[cfg(test)]
mod test {
    use alloy::primitives::{
        Address, Bytes as AlloyBytes, Log as PrimitiveLog, LogData, B256,
    };
    use alloy::rpc::types::Log;
    use alloy::sol_types::private::IntoLogData;
    use alloy::sol_types::SolEvent;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::time::Duration;
    use tokio_util::bytes::Bytes as TokioBytes;

    use crate::contracts::{IKMSGeneration, KMSGeneration};
    use fhevm_engine_common::chain_id::ChainId;

    use super::aws_s3::AwsS3Interface;
    use super::database::{StagedKeyActivation, StoredKeyDigest};
    use super::{
        digest_key, download_key_activation, encode_kms_generation_events,
        key_id_to_aws_key, key_id_to_database_bytes, to_key_prefix, KeyType,
    };

    #[derive(Clone)]
    struct TestS3Client {
        responses: Arc<HashMap<String, TestS3Response>>,
    }

    #[derive(Clone)]
    struct TestS3Response {
        delay: Duration,
        result: TestS3Result,
    }

    #[derive(Clone)]
    enum TestS3Result {
        Ok(TokioBytes),
        Err(String),
    }

    #[async_trait]
    impl AwsS3Interface for TestS3Client {
        async fn get_bucket_key(
            &self,
            _url: &str,
            _bucket: &str,
            key: &str,
        ) -> anyhow::Result<TokioBytes> {
            let response =
                self.responses.get(key).expect("missing mocked S3 response");
            tokio::time::sleep(response.delay).await;
            match &response.result {
                TestS3Result::Ok(bytes) => Ok(bytes.clone()),
                TestS3Result::Err(message) => Err(anyhow!(message.clone())),
            }
        }
    }

    fn rpc_log_from_event<E>(
        address: Address,
        event: E,
        transaction_hash: Option<B256>,
        log_index: Option<u64>,
    ) -> Log
    where
        E: SolEvent + IntoLogData,
    {
        Log {
            inner: PrimitiveLog {
                address,
                data: event.into_log_data(),
            },
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash,
            transaction_index: Some(0),
            log_index,
            removed: false,
        }
    }

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

    #[test]
    fn encode_kms_generation_events_skips_invalid_logs() {
        let kms_address = Address::repeat_byte(7);
        let valid_log = rpc_log_from_event(
            kms_address,
            KMSGeneration::ActivateCrs {
                crsId: alloy::primitives::U256::from(11),
                kmsNodeStorageUrls: vec![
                    "https://bucket.s3.eu-west-1.amazonaws.com/".to_owned(),
                ],
                crsDigest: AlloyBytes::from(vec![1, 2, 3]),
            },
            Some(B256::repeat_byte(1)),
            Some(0),
        );
        let invalid_encoded_log = rpc_log_from_event(
            kms_address,
            KMSGeneration::ActivateKey {
                keyId: alloy::primitives::U256::from(12),
                kmsNodeStorageUrls: vec![
                    "https://bucket.s3.eu-west-1.amazonaws.com/".to_owned(),
                ],
                keyDigests: vec![IKMSGeneration::KeyDigest {
                    keyType: 0,
                    digest: AlloyBytes::from(vec![4, 5, 6]),
                }],
            },
            None,
            Some(1),
        );
        let undecodable_log = Log {
            inner: PrimitiveLog {
                address: kms_address,
                data: LogData::new_unchecked(
                    vec![],
                    AlloyBytes::from(vec![9, 9]),
                ),
            },
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(B256::repeat_byte(2)),
            transaction_index: Some(0),
            log_index: Some(2),
            removed: false,
        };

        let prepared_events = encode_kms_generation_events(
            kms_address,
            &[valid_log, invalid_encoded_log, undecodable_log],
        );

        assert_eq!(prepared_events.0.len(), 1);
        assert!(matches!(
            prepared_events.0.first(),
            Some(super::PreparedKmsEvent::ActivateCrs(_))
        ));
    }

    #[tokio::test]
    async fn download_key_activation_prioritizes_errors_over_digest_mismatch() {
        let key_id = alloy::primitives::U256::from(22);
        let activation = StagedKeyActivation {
            sequence_number: 1,
            chain_id: ChainId::try_from(12345_u64).unwrap(),
            block_hash: vec![3; 32],
            key_id_gw: key_id_to_database_bytes(key_id).to_vec(),
            key_digests: vec![
                StoredKeyDigest {
                    key_type: KeyType::PublicKey as u8,
                    digest: digest_key(b"expected-public").to_vec(),
                },
                StoredKeyDigest {
                    key_type: KeyType::ServerKey as u8,
                    digest: digest_key(b"expected-server").to_vec(),
                },
            ],
            s3_bucket_urls: vec![
                "https://bucket.s3.eu-west-1.amazonaws.com/".to_owned()
            ],
        };
        let responses = HashMap::from([
            (
                format!(
                    "{}/{}",
                    to_key_prefix(KeyType::PublicKey),
                    key_id_to_aws_key(key_id)
                ),
                TestS3Response {
                    delay: Duration::from_millis(5),
                    result: TestS3Result::Ok(TokioBytes::from_static(
                        b"wrong-public",
                    )),
                },
            ),
            (
                format!(
                    "{}/{}",
                    to_key_prefix(KeyType::ServerKey),
                    key_id_to_aws_key(key_id)
                ),
                TestS3Response {
                    delay: Duration::from_millis(25),
                    result: TestS3Result::Err(
                        "transient S3 failure".to_owned(),
                    ),
                },
            ),
        ]);

        let err = download_key_activation(
            &activation,
            &TestS3Client {
                responses: Arc::new(responses),
            },
        )
        .await
        .expect_err("expected retryable download error");

        assert!(err.to_string().contains("Failed to download key"));
    }
}
