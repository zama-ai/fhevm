use alloy::primitives::Uint;
use alloy::rpc::types::Log;
use anyhow::anyhow;
use fhevm_engine_common::chain_id::ChainId;
use sqlx::{Pool, Postgres, Transaction};
use tracing::{error, info, warn};

use crate::contracts::KMSGeneration::{self, KMSGenerationEvents};
use crate::kms_generation::aws_s3::{download_key_from_s3, AwsS3Interface};
use crate::kms_generation::database::{
    activate_ready_crs_activations, activate_ready_key_activations,
    all_pending_crs_activations_to_download,
    all_pending_key_activations_to_download, cancel_orphaned_crs_activations,
    cancel_orphaned_key_activations, count_crs_activation_remaining_pending,
    count_key_activation_remaining_pending, insert_crs_activation_event,
    insert_key_activation_event, mark_crs_activation_error,
    mark_key_activation_error, set_ready_crs_activation,
    set_ready_key_activation, PendingCrsActivation, PendingKeyActivation,
};
use crate::kms_generation::digest::{digest_crs, digest_key};
use crate::kms_generation::metrics::{
    ACTIVATE_CRS_FAIL_COUNTER, ACTIVATE_CRS_SUCCESS_COUNTER,
    ACTIVATE_KEY_FAIL_COUNTER, ACTIVATE_KEY_SUCCESS_COUNTER,
    CRS_DIGEST_MISMATCH_COUNTER, KEY_DIGEST_MISMATCH_COUNTER,
};
use crate::kms_generation::sks_key::{
    prepare_legacy_server_key_for_db, prepare_xof_key_set_for_db,
};

pub mod aws_s3;
pub(crate) mod database;
pub(crate) mod digest;
pub(crate) mod metrics;
pub(crate) mod sks_key;

pub type KeyId = Uint<256, 4>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
struct KeyDigestMismatchError {
    id: String,
}

impl std::fmt::Display for KeyDigestMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Key digest for key ID {}", self.id)
    }
}

impl std::error::Error for KeyDigestMismatchError {}

#[derive(Debug)]
struct CrsDigestMismatchError {
    id: String,
}

impl std::fmt::Display for CrsDigestMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid CRS digest for CRS ID {}", self.id)
    }
}

impl std::error::Error for CrsDigestMismatchError {}

pub fn key_id_to_database_bytes(key_id: KeyId) -> [u8; 32] {
    key_id.to_be_bytes()
}

/// S3 prefix kms-core uses when publishing keys in the new
/// CompressedXofKeySet format. Tried first by
/// [`download_server_key_for_activation`].
const XOF_KEY_SET_S3_PREFIX: &str = "/CompressedXofKeySet";
/// S3 prefix kms-core used for the pre-XOF plain ServerKey format.
/// Probed only when the XOF prefix is absent.
const LEGACY_SERVER_KEY_S3_PREFIX: &str = "/ServerKey";
/// S3 prefix kms-core uses when publishing compact public keys.
const PUBLIC_KEY_S3_PREFIX: &str = "/PublicKey";

pub fn key_id_to_aws_key(key_id: KeyId) -> String {
    format!("{:064x}", key_id)
}

fn key_id_from_database_bytes(key_id: &[u8]) -> anyhow::Result<KeyId> {
    KeyId::try_from_be_slice(key_id)
        .ok_or_else(|| anyhow!("Invalid key ID bytes: {}", key_id.len()))
}

pub async fn insert_kms_generation_events_tx(
    tx: &mut Transaction<'_, Postgres>,
    events: Vec<(KMSGenerationEvents, Log)>,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    for (event, log) in events {
        match event {
            KMSGeneration::KMSGenerationEvents::ActivateKey(activation) => {
                insert_key_activation_event(
                    tx,
                    activation,
                    log,
                    chain_id,
                    block_hash,
                    block_number,
                )
                .await?;
            }
            KMSGeneration::KMSGenerationEvents::ActivateCrs(activation) => {
                insert_crs_activation_event(
                    tx,
                    activation,
                    log,
                    chain_id,
                    block_hash,
                    block_number,
                )
                .await?;
            }
            _ => {
                warn!(
                    ?log,
                    ?event,
                    "Unsupported KMSGeneration event type, skipping"
                );
            }
        }
    }
    Ok(())
}

pub async fn process_kms_generation_activations<
    A: AwsS3Interface + Clone + 'static,
>(
    db_pool: Pool<Postgres>,
    s3_client: A,
) -> anyhow::Result<u64> {
    //first we handle every thing that is ready to be cancelled or activated
    let mut tx = db_pool.begin().await?;
    cancel_orphaned_key_activations(&mut tx).await?;
    cancel_orphaned_crs_activations(&mut tx).await?;
    activate_ready_key_activations(&mut tx).await?;
    activate_ready_crs_activations(&mut tx).await?;
    tx.commit().await?;

    // second we download and check keys and preprocess in background in advance so it's ready when block is finalized
    // rows are locked so there's no double work
    let mut tx = db_pool.begin().await?;
    let key_activations =
        all_pending_key_activations_to_download(&mut tx).await?;
    let crs_activations =
        all_pending_crs_activations_to_download(&mut tx).await?;
    if key_activations.is_empty() && crs_activations.is_empty() {
        info!("No pending KMSGeneration activation to download");
        return Ok(0);
    }
    info!(
        "Pending {} KMSGeneration activation to download",
        key_activations.len()
    );
    info!(
        "Pending {} CRS activation to download",
        crs_activations.len()
    );
    // do all downloads
    download_and_store_key_activations(&mut tx, &s3_client, key_activations)
        .await?;
    download_and_store_crs_activations(&mut tx, &s3_client, crs_activations)
        .await?;
    info!("Downloading succeeded for KMSGeneration and CRS");
    tx.commit().await?;
    let remain_pending = count_key_activation_remaining_pending(&db_pool)
        .await?
        + count_crs_activation_remaining_pending(&db_pool).await?;
    Ok(remain_pending)
}

async fn download_and_store_key_activations<
    A: AwsS3Interface + Clone + 'static,
>(
    tx: &mut Transaction<'_, Postgres>,
    s3_client: &A,
    key_activations: Vec<PendingKeyActivation>,
) -> anyhow::Result<()> {
    for key_activation in key_activations {
        if let Err(err) =
            download_and_store_key_activation(tx, s3_client, &key_activation)
                .await
        {
            error!(error = %err, key_id = ?key_activation.key_id, "Failed to download and store key activation");
            mark_key_activation_error(tx, &err.to_string(), key_activation)
                .await;
            ACTIVATE_KEY_FAIL_COUNTER.inc();
        } else {
            ACTIVATE_KEY_SUCCESS_COUNTER.inc();
        }
    }
    Ok(())
}

async fn download_and_store_crs_activations<
    A: AwsS3Interface + Clone + 'static,
>(
    tx: &mut Transaction<'_, Postgres>,
    s3_client: &A,
    crs_activations: Vec<PendingCrsActivation>,
) -> anyhow::Result<()> {
    for crs_activation in crs_activations {
        if let Err(err) =
            download_and_store_crs_activation(tx, s3_client, &crs_activation)
                .await
        {
            error!(error = %err, crs_id = ?crs_activation.crs_id, "Failed to download and store CRS activation");
            mark_crs_activation_error(tx, &err.to_string(), crs_activation)
                .await;
            ACTIVATE_CRS_FAIL_COUNTER.inc();
        } else {
            ACTIVATE_CRS_SUCCESS_COUNTER.inc();
        }
    }
    Ok(())
}

/// Server-key payload pulled from S3, tagged with the format kms-core
/// wrote it in. The host-listener has to know this to pick the right
/// `prepare_*_for_db` variant.
#[derive(Clone, Debug)]
enum DownloadedServerKey {
    Xof(Vec<u8>),
    Legacy(Vec<u8>),
}

async fn download_and_store_key_activation<
    A: AwsS3Interface + Clone + 'static,
>(
    tx: &mut Transaction<'_, Postgres>,
    s3_client: &A,
    activation: &PendingKeyActivation,
) -> anyhow::Result<()> {
    let server_key =
        download_server_key_for_activation(activation, s3_client).await;
    let public_key =
        download_public_key_activation(activation, s3_client).await;
    match (server_key, public_key) {
        (Ok(server_key), Ok(public_key)) => {
            info!(
                key_id = ?activation.key_id,
                server_key_downloaded = server_key.is_some(),
                public_key_downloaded = public_key.is_some(),
                "Finished downloading keys for activation"
            );
            let prepared = match server_key {
                Some(DownloadedServerKey::Xof(bytes)) => {
                    Some(prepare_xof_key_set_for_db(&bytes)?)
                }
                Some(DownloadedServerKey::Legacy(bytes)) => {
                    Some(prepare_legacy_server_key_for_db(&bytes)?)
                }
                None => None,
            };
            Ok(
                set_ready_key_activation(tx, activation, prepared, public_key)
                    .await?,
            )
        }
        (Err(err), _) | (_, Err(err)) => anyhow::bail!(err),
    }
}

async fn download_and_store_crs_activation<
    A: AwsS3Interface + Clone + 'static,
>(
    tx: &mut Transaction<'_, Postgres>,
    s3_client: &A,
    activation: &PendingCrsActivation,
) -> anyhow::Result<()> {
    let crs = download_crs_activation(activation, s3_client).await?;
    set_ready_crs_activation(tx, activation, crs).await
}

/// Probes the configured S3 buckets for the server-key blob at
/// `/CompressedXofKeySet/{key_id}` first, falling back to
/// `/ServerKey/{key_id}` if every bucket is missing the XOF blob.
///
/// The fallback only triggers on "nothing downloaded" — digest
/// mismatches on the XOF blob still bail loudly, because that signals a
/// kms-core bug rather than a format-version difference.
async fn download_server_key_for_activation<
    A: AwsS3Interface + Clone + 'static,
>(
    activation: &PendingKeyActivation,
    s3_client: &A,
) -> anyhow::Result<Option<DownloadedServerKey>> {
    let expected_digest = match &activation.digest_server {
        Some(digest) if !activation.has_server_key => digest,
        _ => {
            info!(
                key_id = ?activation.key_id,
                "Server key already marked as downloaded or not needed, skipping download"
            );
            return Ok(None);
        }
    };

    let key_id = key_id_from_database_bytes(&activation.key_id)?;
    info!(
        key_id = key_id.to_string(),
        bucket_urls = ?activation.storage_urls,
        "Received ActivateKey event"
    );

    let xof_path =
        format!("{}/{}", XOF_KEY_SET_S3_PREFIX, key_id_to_aws_key(key_id));
    match download_key_from_s3(s3_client, &activation.storage_urls, xof_path, 0)
        .await
    {
        Ok(bytes) => {
            verify_server_key_digest(&bytes, expected_digest, &key_id)?;
            return Ok(Some(DownloadedServerKey::Xof(bytes.to_vec())));
        }
        Err(err) => {
            // The S3 layer collapses missing-key, network, auth, and
            // parse errors into a single anyhow::Error, so we cannot
            // tell whether the XOF prefix is genuinely absent or just
            // unreachable. Log the real error so an operator can
            // distinguish, and fall through to the legacy prefix; if
            // the cause was transient the legacy probe will fail too
            // and the activation will be retried.
            info!(
                key_id = key_id.to_string(),
                xof_error = %err,
                "CompressedXofKeySet probe failed across all buckets (could be missing, transient, or permissions); trying legacy ServerKey prefix"
            );
        }
    }

    let legacy_path = format!(
        "{}/{}",
        LEGACY_SERVER_KEY_S3_PREFIX,
        key_id_to_aws_key(key_id)
    );
    match download_key_from_s3(
        s3_client,
        &activation.storage_urls,
        legacy_path,
        0,
    )
    .await
    {
        Ok(bytes) => {
            verify_server_key_digest(&bytes, expected_digest, &key_id)?;
            Ok(Some(DownloadedServerKey::Legacy(bytes.to_vec())))
        }
        Err(_) => Err(anyhow!(
            "Failed to download server key {:?} from any prefix or any bucket, urls {:?}",
            key_id,
            activation.storage_urls.join(", ")
        )),
    }
}

fn verify_server_key_digest(
    bytes: &[u8],
    expected_digest: &[u8],
    key_id: &KeyId,
) -> anyhow::Result<()> {
    let downloaded_digest = digest_key(bytes);
    if downloaded_digest == expected_digest {
        Ok(())
    } else {
        KEY_DIGEST_MISMATCH_COUNTER.inc();
        anyhow::bail!(KeyDigestMismatchError {
            id: key_id.to_string(),
        });
    }
}

async fn download_public_key_activation<A: AwsS3Interface + Clone + 'static>(
    activation: &PendingKeyActivation,
    s3_client: &A,
) -> anyhow::Result<Option<Vec<u8>>> {
    let expected_digest = match &activation.digest_public {
        Some(digest) if !activation.has_public_key => digest,
        _ => {
            info!(
                key_id = ?activation.key_id,
                "Public key already marked as downloaded or not needed, skipping download"
            );
            return Ok(None);
        }
    };

    let key_id = key_id_from_database_bytes(&activation.key_id)?;
    info!(
        key_id = key_id.to_string(),
        bucket_urls = ?activation.storage_urls,
        "Received ActivateKey event"
    );
    let key_path =
        format!("{}/{}", PUBLIC_KEY_S3_PREFIX, key_id_to_aws_key(key_id));
    let Ok(bytes) =
        download_key_from_s3(s3_client, &activation.storage_urls, key_path, 0)
            .await
    else {
        return Err(anyhow!(
            "Failed to download public key {:?}, urls {:?}",
            key_id,
            activation.storage_urls.join(", ")
        ));
    };

    let downloaded_digest = digest_key(&bytes);
    if downloaded_digest == expected_digest.as_slice() {
        Ok(Some(bytes.to_vec()))
    } else {
        KEY_DIGEST_MISMATCH_COUNTER.inc();
        anyhow::bail!(KeyDigestMismatchError {
            id: key_id.to_string(),
        });
    }
}

async fn download_crs_activation<A: AwsS3Interface + Clone + 'static>(
    activation: &PendingCrsActivation,
    s3_client: &A,
) -> anyhow::Result<Option<Vec<u8>>> {
    let crs_id = key_id_from_database_bytes(&activation.crs_id)?;
    info!(
        key_id = crs_id.to_string(),
        bucket_urls = ?activation.storage_urls,
        "Received ActivateCrs event"
    );

    let key_path_suffix = format!("/CRS/{}", key_id_to_aws_key(crs_id));
    let bytes = download_key_from_s3(
        s3_client,
        &activation.storage_urls,
        key_path_suffix,
        0,
    )
    .await?;
    let downloaded_digest = digest_crs(&bytes);
    let expected_digest = activation.digest.as_slice();
    if downloaded_digest == expected_digest {
        Ok(Some(bytes.to_vec()))
    } else {
        CRS_DIGEST_MISMATCH_COUNTER.inc();
        anyhow::bail!(CrsDigestMismatchError {
            id: crs_id.to_string(),
        });
    }
}

#[cfg(test)]
mod test {
    use anyhow::anyhow;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::time::Duration;
    use tokio_util::bytes::Bytes as TokioBytes;

    use crate::kms_generation::database::PendingKeyActivation;
    use fhevm_engine_common::chain_id::ChainId;

    use super::aws_s3::AwsS3Interface;
    use super::{
        digest_key, download_server_key_for_activation, key_id_to_aws_key,
        key_id_to_database_bytes, DownloadedServerKey,
        LEGACY_SERVER_KEY_S3_PREFIX, XOF_KEY_SET_S3_PREFIX,
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
            // Mimic S3 NoSuchKey: keys not registered in the mock
            // return a recoverable error instead of panicking, so the
            // probing logic in download_server_key_for_activation can
            // observe a clean "not found" and fall through.
            let Some(response) = self.responses.get(key) else {
                return Err(anyhow!("mocked S3 NoSuchKey for {key}"));
            };
            tokio::time::sleep(response.delay).await;
            match &response.result {
                TestS3Result::Ok(bytes) => Ok(bytes.clone()),
                TestS3Result::Err(message) => Err(anyhow!(message.clone())),
            }
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

    fn pending_activation(
        key_id: alloy::primitives::U256,
        block_hash: Vec<u8>,
        expected_server_digest: Vec<u8>,
    ) -> PendingKeyActivation {
        PendingKeyActivation {
            chain_id: ChainId::try_from(12345_u64).unwrap(),
            block_hash,
            key_id: key_id_to_database_bytes(key_id).to_vec(),
            digest_public: None,
            digest_server: Some(expected_server_digest),
            has_public_key: true,
            has_server_key: false,
            storage_urls: vec![
                "https://bucket.s3.eu-west-1.amazonaws.com/".to_owned()
            ],
        }
    }

    #[tokio::test]
    async fn download_server_key_fails_when_neither_prefix_is_reachable() {
        let key_id = alloy::primitives::U256::from(22);
        let activation = pending_activation(
            key_id,
            vec![3; 32],
            digest_key(b"expected-server").to_vec(),
        );
        // XOF prefix is registered but fails transiently; legacy prefix
        // isn't registered at all, so the mock surfaces NoSuchKey for
        // it. Both probe attempts fail → top-level bail.
        let responses = HashMap::from([(
            format!("{}/{}", XOF_KEY_SET_S3_PREFIX, key_id_to_aws_key(key_id)),
            TestS3Response {
                delay: Duration::from_millis(5),
                result: TestS3Result::Err("transient S3 failure".to_owned()),
            },
        )]);

        let err = download_server_key_for_activation(
            &activation,
            &TestS3Client {
                responses: Arc::new(responses),
            },
        )
        .await
        .expect_err("expected retryable download error");

        assert!(err.to_string().contains("Failed to download server key"));
    }

    #[tokio::test]
    async fn download_server_key_uses_compressed_xof_prefix_when_present() {
        let key_id = alloy::primitives::U256::from(23);
        let server_key_bytes = TokioBytes::from_static(b"compressed-xof-key");
        let activation = pending_activation(
            key_id,
            vec![4; 32],
            digest_key(&server_key_bytes).to_vec(),
        );
        let responses = HashMap::from([(
            format!("{}/{}", XOF_KEY_SET_S3_PREFIX, key_id_to_aws_key(key_id)),
            TestS3Response {
                delay: Duration::from_millis(1),
                result: TestS3Result::Ok(server_key_bytes.clone()),
            },
        )]);

        let downloaded = download_server_key_for_activation(
            &activation,
            &TestS3Client {
                responses: Arc::new(responses),
            },
        )
        .await
        .expect("download should succeed")
        .expect("server key was requested, so it should be present");

        match downloaded {
            DownloadedServerKey::Xof(bytes) => {
                assert_eq!(bytes, server_key_bytes.to_vec())
            }
            DownloadedServerKey::Legacy(_) => {
                panic!("expected XOF blob, got legacy")
            }
        }
    }

    #[tokio::test]
    async fn download_server_key_falls_back_to_legacy_server_key_prefix() {
        let key_id = alloy::primitives::U256::from(24);
        let legacy_server_key_bytes =
            TokioBytes::from_static(b"legacy-server-key");
        let activation = pending_activation(
            key_id,
            vec![5; 32],
            digest_key(&legacy_server_key_bytes).to_vec(),
        );
        // Only the legacy prefix has a blob: the XOF probe returns
        // NoSuchKey from the mock, the fallback hits the legacy
        // ServerKey blob and accepts it.
        let responses = HashMap::from([(
            format!(
                "{}/{}",
                LEGACY_SERVER_KEY_S3_PREFIX,
                key_id_to_aws_key(key_id)
            ),
            TestS3Response {
                delay: Duration::from_millis(1),
                result: TestS3Result::Ok(legacy_server_key_bytes.clone()),
            },
        )]);

        let downloaded = download_server_key_for_activation(
            &activation,
            &TestS3Client {
                responses: Arc::new(responses),
            },
        )
        .await
        .expect("download should succeed")
        .expect("server key was requested, so it should be present");

        match downloaded {
            DownloadedServerKey::Legacy(bytes) => {
                assert_eq!(bytes, legacy_server_key_bytes.to_vec())
            }
            DownloadedServerKey::Xof(_) => {
                panic!("expected legacy blob, got XOF")
            }
        }
    }

    #[tokio::test]
    async fn download_server_key_bails_on_xof_digest_mismatch_without_fallback()
    {
        let key_id = alloy::primitives::U256::from(25);
        let xof_bytes = TokioBytes::from_static(b"compressed-xof-key");
        let legacy_bytes = TokioBytes::from_static(b"legacy-server-key");
        // The activation's expected digest is the legacy bytes' digest;
        // the XOF blob is corrupt. Probe must NOT silently fall through
        // — kms-core publishing bad XOF bytes is a bug, not a missing
        // format.
        let activation = pending_activation(
            key_id,
            vec![6; 32],
            digest_key(&legacy_bytes).to_vec(),
        );
        let responses = HashMap::from([
            (
                format!(
                    "{}/{}",
                    XOF_KEY_SET_S3_PREFIX,
                    key_id_to_aws_key(key_id)
                ),
                TestS3Response {
                    delay: Duration::from_millis(1),
                    result: TestS3Result::Ok(xof_bytes.clone()),
                },
            ),
            (
                format!(
                    "{}/{}",
                    LEGACY_SERVER_KEY_S3_PREFIX,
                    key_id_to_aws_key(key_id)
                ),
                TestS3Response {
                    delay: Duration::from_millis(1),
                    result: TestS3Result::Ok(legacy_bytes.clone()),
                },
            ),
        ]);

        let err = download_server_key_for_activation(
            &activation,
            &TestS3Client {
                responses: Arc::new(responses),
            },
        )
        .await
        .expect_err("expected digest mismatch on XOF blob");

        assert!(err.to_string().contains("Invalid Key digest"));
    }
}
