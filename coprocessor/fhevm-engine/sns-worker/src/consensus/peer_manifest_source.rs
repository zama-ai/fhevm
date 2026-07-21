use std::{collections::HashSet, sync::Arc};

use alloy_primitives::{Address, B256, U256};
use aws_sdk_s3::Client;
use ciphertext_attestation::manifest::{ManifestReference, ManifestVersion};
use tracing::debug;
use url::Url;

use crate::ExecutionError;

const MAX_LIST_PAGES_PER_ATTEMPT: usize = 10;
const MAX_MANIFEST_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Debug)]
pub(super) struct PeerManifestObject {
    pub object_key: String,
    pub signed_bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
pub(super) struct PeerDownloadRequest {
    pub publisher: Address,
    pub s3_bucket_url: String,
    pub version: ManifestVersion,
    pub coprocessor_context_id: U256,
    pub host_chain_id: i64,
    pub publication_block_number: i64,
    pub publication_block_hash: B256,
    pub known_revisions: HashSet<u64>,
}

pub(super) trait PeerManifestSource: Send + Sync {
    async fn list_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<String>, ExecutionError>;

    async fn fetch_manifest(
        &self,
        request: &PeerDownloadRequest,
        object_key: &str,
    ) -> Result<PeerManifestObject, ExecutionError>;
}

#[derive(Clone)]
pub(super) struct S3PeerManifestSource {
    client: Arc<Client>,
}

struct ListedManifestObject {
    revision: u64,
    canonical_key: String,
}

impl S3PeerManifestSource {
    pub(super) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl PeerManifestSource for S3PeerManifestSource {
    async fn list_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<String>, ExecutionError> {
        debug!(
            publisher = %request.publisher,
            bucket_url = request.s3_bucket_url,
            "Listing numbered peer manifest revisions"
        );
        let location = s3_bucket_location(&request.s3_bucket_url)?;
        let prefix = location.object_key(manifest_prefix(request));
        let listed = self
            .list_manifest_objects(request, &location, &prefix)
            .await?;
        Ok(listed
            .into_iter()
            .map(|object| object.canonical_key)
            .collect())
    }

    async fn fetch_manifest(
        &self,
        request: &PeerDownloadRequest,
        object_key: &str,
    ) -> Result<PeerManifestObject, ExecutionError> {
        let location = s3_bucket_location(&request.s3_bucket_url)?;
        let s3_key = location.object_key(object_key.to_owned());
        Ok(PeerManifestObject {
            object_key: object_key.to_owned(),
            signed_bytes: self.download_manifest(&location.bucket, &s3_key).await?,
        })
    }
}

impl S3PeerManifestSource {
    async fn list_manifest_objects(
        &self,
        request: &PeerDownloadRequest,
        location: &S3BucketLocation,
        prefix: &str,
    ) -> Result<Vec<ListedManifestObject>, ExecutionError> {
        let mut continuation_token = None;
        let mut objects = Vec::new();
        let mut fully_listed = false;

        for _ in 0..MAX_LIST_PAGES_PER_ATTEMPT {
            let mut list = self
                .client
                .list_objects_v2()
                .bucket(&location.bucket)
                .prefix(prefix);
            if let Some(token) = continuation_token.as_deref() {
                list = list.continuation_token(token);
            }
            let response = list.send().await.map_err(|err| {
                ExecutionError::S3TransientError(format!(
                    "failed to list peer manifest prefix {prefix} in {}: {err}",
                    location.bucket,
                ))
            })?;
            for object in response.contents() {
                let Some(key) = object.key() else {
                    continue;
                };
                let Some(revision) = revision_below_prefix(key, prefix) else {
                    continue;
                };
                if !request.known_revisions.contains(&revision) {
                    objects.push(ListedManifestObject {
                        revision,
                        canonical_key: location.canonical_key(key),
                    });
                }
            }
            if response.is_truncated() != Some(true) {
                fully_listed = true;
                break;
            }
            continuation_token = response.next_continuation_token().map(ToOwned::to_owned);
            if continuation_token.is_none() {
                break;
            }
        }
        if !fully_listed {
            return Err(ExecutionError::S3TransientError(format!(
                "peer manifest prefix {prefix} in {} exceeded the bounded listing budget",
                location.bucket,
            )));
        }

        objects.sort_unstable_by_key(|object| std::cmp::Reverse(object.revision));
        objects.dedup_by_key(|object| object.revision);
        Ok(objects)
    }

    async fn download_manifest(&self, bucket: &str, key: &str) -> Result<Vec<u8>, ExecutionError> {
        let response = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|err| {
                ExecutionError::S3TransientError(format!(
                    "failed to download peer manifest {key} from {bucket}: {err}"
                ))
            })?;
        if response
            .content_length()
            .is_some_and(|length| length < 0 || length as usize > MAX_MANIFEST_BYTES)
        {
            return Err(internal(format!(
                "peer manifest {key} from {bucket} exceeds {MAX_MANIFEST_BYTES} bytes"
            )));
        }
        let mut stream = response.body;
        let mut body = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|err| {
                ExecutionError::S3TransientError(format!(
                    "failed to read peer manifest {key} from {bucket}: {err}"
                ))
            })?;
            if body.len().saturating_add(chunk.len()) > MAX_MANIFEST_BYTES {
                return Err(internal(format!(
                    "peer manifest {key} from {bucket} exceeds {MAX_MANIFEST_BYTES} bytes"
                )));
            }
            body.extend_from_slice(&chunk);
        }
        Ok(body)
    }
}

fn manifest_prefix(request: &PeerDownloadRequest) -> String {
    format!(
        "manifests/v{}/{}/{}/{}/{}/",
        u8::from(request.version),
        request.coprocessor_context_id,
        request.host_chain_id,
        request.publication_block_number,
        hex::encode(request.publication_block_hash),
    )
}

pub(super) fn referenced_manifest_object_key(
    request: &PeerDownloadRequest,
    reference: &ManifestReference,
) -> String {
    format!(
        "manifests/v{}/{}/{}/{}/{}/{}",
        u8::from(request.version),
        request.coprocessor_context_id,
        request.host_chain_id,
        reference.block_number,
        hex::encode(reference.block_hash),
        reference.revision,
    )
}

fn revision_below_prefix(key: &str, prefix: &str) -> Option<u64> {
    key.strip_prefix(prefix)?.parse().ok()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct S3BucketLocation {
    pub(super) bucket: String,
    pub(super) key_prefix: String,
}

impl S3BucketLocation {
    fn object_key(&self, suffix: String) -> String {
        if self.key_prefix.is_empty() {
            suffix
        } else {
            format!("{}/{suffix}", self.key_prefix)
        }
    }

    fn canonical_key(&self, s3_key: &str) -> String {
        if self.key_prefix.is_empty() {
            s3_key.to_owned()
        } else {
            s3_key
                .strip_prefix(&format!("{}/", self.key_prefix))
                .unwrap_or(s3_key)
                .to_owned()
        }
    }
}

pub(super) fn s3_bucket_location(bucket_url: &str) -> Result<S3BucketLocation, ExecutionError> {
    let url = Url::parse(bucket_url)
        .map_err(|err| internal(format!("invalid peer S3 bucket URL {bucket_url}: {err}")))?;
    let host = url
        .host_str()
        .ok_or_else(|| internal(format!("peer S3 bucket URL {bucket_url} has no host")))?;
    let segments = url
        .path_segments()
        .map(|segments| {
            segments
                .filter(|segment| !segment.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let first_host_label = host.split('.').next().unwrap_or_default();
    let virtual_hosted = url.scheme() == "s3"
        || host.contains(".s3.")
        || host.contains(".s3-")
        || host.ends_with(".s3.amazonaws.com");
    let (bucket, key_segments) = if virtual_hosted {
        (first_host_label, segments.as_slice())
    } else {
        let Some((bucket, key_segments)) = segments.split_first() else {
            return Err(internal(format!(
                "cannot determine bucket name from peer S3 URL {bucket_url}"
            )));
        };
        (*bucket, key_segments)
    };
    if bucket.is_empty() || bucket == "s3" {
        return Err(internal(format!(
            "cannot determine bucket name from peer S3 URL {bucket_url}"
        )));
    }
    Ok(S3BucketLocation {
        bucket: bucket.to_owned(),
        key_prefix: key_segments.join("/"),
    })
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
