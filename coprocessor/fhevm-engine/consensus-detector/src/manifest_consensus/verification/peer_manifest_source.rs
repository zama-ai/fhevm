use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use alloy_primitives::{Address, B256, U256};
use block_manifest::{ManifestVersion, MAX_MANIFEST_BYTES};
use tracing::{debug, info};

use crate::manifest_consensus::peer_http;
use crate::manifest_consensus::ExecutionError;

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
    pub consensus_epoch: String,
    pub coprocessor_context_id: U256,
    pub host_chain_id: i64,
    pub publication_block_number: i64,
    pub publication_block_hash: B256,
    /// Exclusive floor: already-archived valid revisions, and anything older,
    /// are not fetched again.
    pub highest_archived_revision: Option<u64>,
    /// Listed keys that cannot become valid (NoSuchKey, auth, scope, oversize).
    pub rejected_object_keys: HashSet<String>,
}

impl PeerDownloadRequest {
    pub(super) fn considers_object(&self, object_key: &str, revision: u64) -> bool {
        if self.rejected_object_keys.contains(object_key) {
            return false;
        }
        match self.highest_archived_revision {
            Some(best) => revision > best,
            None => true,
        }
    }
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

fn manifest_prefix(request: &PeerDownloadRequest) -> String {
    block_manifest::manifest_object_prefix(
        request.version,
        request.coprocessor_context_id,
        U256::from(request.host_chain_id as u64),
        U256::from(request.publication_block_number as u64),
        request.publication_block_hash,
        &request.consensus_epoch,
    )
}

/// A probe reads no body, and the listing runs inside the caller's own budget,
/// so one slow peer must not consume it.
const MANIFEST_PROBE_TIMEOUT: Duration = Duration::from_secs(5);

/// The probes run one after the other, so a slow peer can spend more than the
/// caller allows the whole listing. Deriving this from the caller's own bound
/// keeps the listing stopping first, with an error that names the budget,
/// instead of the caller cancelling mid-probe.
const MANIFEST_LIST_BUDGET: Duration =
    super::peer_downloader::PEER_LIST_TIMEOUT.saturating_sub(Duration::from_secs(5));

/// A manifest is larger than a probe, so the body read waits longer.
const MANIFEST_FETCH_TIMEOUT: Duration = Duration::from_secs(30);

/// What one probe of an object says about it.
enum Probe {
    Present,
    Absent,
    /// The origin refuses the method. The caller retries with another one.
    MethodRefused,
}

/// Reads peer manifests over HTTP from the registered URL, the channel the
/// state-hash poll already uses.
///
/// The registered URL names an endpoint, not a bucket, and the read is
/// anonymous. The reader checks the publisher's signature, so the transport
/// carries no integrity.
pub(super) struct HttpPeerManifestSource {
    http: reqwest::Client,
}

impl HttpPeerManifestSource {
    pub(super) fn new(http: reqwest::Client) -> Self {
        Self { http }
    }

    async fn send_probe(
        &self,
        probe: reqwest::RequestBuilder,
        url: &str,
        object_key: &str,
    ) -> Result<Probe, ExecutionError> {
        let response = probe
            .timeout(MANIFEST_PROBE_TIMEOUT)
            .send()
            .await
            .map_err(|err| {
                ExecutionError::S3TransientError(format!(
                    "failed to probe peer manifest {object_key} at {url}: {err}"
                ))
            })?;
        let status = response.status();
        if status.is_success() {
            return Ok(Probe::Present);
        }
        match status {
            reqwest::StatusCode::NOT_FOUND => Ok(Probe::Absent),
            reqwest::StatusCode::METHOD_NOT_ALLOWED => Ok(Probe::MethodRefused),
            // Every other status is an error, so a bucket that answers 403 for
            // an absent key never passes as the end of the revision set.
            _ => Err(ExecutionError::S3TransientError(format!(
                "peer manifest probe {object_key} at {url} returned {status}"
            ))),
        }
    }

    async fn object_exists(&self, url: &str, object_key: &str) -> Result<bool, ExecutionError> {
        match self
            .send_probe(self.http.head(url), url, object_key)
            .await?
        {
            Probe::Present => return Ok(true),
            Probe::Absent => return Ok(false),
            Probe::MethodRefused => {}
        }
        // An origin may serve GET alone. One byte answers the same question and
        // still reads no manifest.
        let ranged = self
            .http
            .get(url)
            .header(reqwest::header::RANGE, "bytes=0-0");
        match self.send_probe(ranged, url, object_key).await? {
            Probe::Present => Ok(true),
            Probe::Absent => Ok(false),
            Probe::MethodRefused => Err(ExecutionError::S3TransientError(format!(
                "peer manifest probe {object_key} at {url} refuses HEAD and GET"
            ))),
        }
    }

    /// One probe, refused once the listing budget is gone.
    async fn probe_within(
        &self,
        deadline: Instant,
        request: &PeerDownloadRequest,
        prefix: &str,
        revision: u64,
    ) -> Result<bool, ExecutionError> {
        let spent = || {
            ExecutionError::S3TransientError(format!(
                "peer manifest revision probe of {} ran past {MANIFEST_LIST_BUDGET:?}",
                request.s3_bucket_url
            ))
        };
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(spent());
        }
        let object_key = format!("{prefix}{revision}");
        let url = peer_http::object_url(&request.s3_bucket_url, &object_key);
        tokio::time::timeout(remaining, self.object_exists(&url, &object_key))
            .await
            .map_err(|_| spent())?
    }

    /// Whether the peer holds `first`, once `first + 1` is proved absent.
    ///
    /// A block carries one revision today. A second one would be invisible to
    /// this reader, and one above an absent `first` would pin the peer to a
    /// stale manifest for good. The probe past `first` refuses both. It reads
    /// that one revision only, so a peer that skips a revision still passes.
    async fn single_revision(
        &self,
        request: &PeerDownloadRequest,
        prefix: &str,
        first: u64,
    ) -> Result<bool, ExecutionError> {
        let deadline = Instant::now() + MANIFEST_LIST_BUDGET;
        let held = self.probe_within(deadline, request, prefix, first).await?;
        let above = first.saturating_add(1);
        if self.probe_within(deadline, request, prefix, above).await? {
            return Err(ExecutionError::InternalError(format!(
                "peer manifests from {} hold revision {above}, which this reader does not read",
                request.s3_bucket_url
            )));
        }
        Ok(held)
    }

    async fn get_object(&self, url: &str, object_key: &str) -> Result<Vec<u8>, ExecutionError> {
        let object = format!("peer manifest {object_key} from {url}");
        let response = self
            .http
            .get(url)
            .timeout(MANIFEST_FETCH_TIMEOUT)
            .send()
            .await
            .map_err(|err| {
                ExecutionError::S3TransientError(format!("failed to request {object}: {err}"))
            })?;
        let response = peer_http::check_status(response, &object)?;
        peer_http::read_capped(response, MAX_MANIFEST_BYTES, &object, || {
            ExecutionError::PeerManifestTooLarge(format!(
                "{object} exceeds {MAX_MANIFEST_BYTES} bytes"
            ))
        })
        .await
    }
}

impl PeerManifestSource for HttpPeerManifestSource {
    async fn list_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<String>, ExecutionError> {
        let prefix = manifest_prefix(request);
        debug!(
            publisher = %request.publisher,
            bucket_url = request.s3_bucket_url,
            "Probing numbered peer manifest revisions over HTTP"
        );
        let first = request
            .highest_archived_revision
            .map_or(0, |archived| archived.saturating_add(1));
        if !self.single_revision(request, &prefix, first).await? {
            return Ok(Vec::new());
        }
        let object_key = format!("{prefix}{first}");
        if !request.considers_object(&object_key, first) {
            return Ok(Vec::new());
        }
        Ok(vec![object_key])
    }

    async fn fetch_manifest(
        &self,
        request: &PeerDownloadRequest,
        object_key: &str,
    ) -> Result<PeerManifestObject, ExecutionError> {
        let url = peer_http::object_url(&request.s3_bucket_url, object_key);
        let signed_bytes = self.get_object(&url, object_key).await?;
        info!(
            url,
            object_key,
            bytes = signed_bytes.len(),
            "Downloaded peer manifest"
        );
        Ok(PeerManifestObject {
            object_key: object_key.to_owned(),
            signed_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use alloy_primitives::{Address, B256, U256};
    use axum::http::{Method, StatusCode, Uri};
    use axum::Router;
    use block_manifest::ManifestVersion;
    use tokio::net::TcpListener;

    use super::{HttpPeerManifestSource, PeerDownloadRequest, PeerManifestSource};
    use crate::manifest_consensus::peer_http;
    use crate::manifest_consensus::ExecutionError;

    /// A peer origin that holds a run of revisions and counts what it is asked.
    ///
    /// `probes` counts every request that reads no manifest, which is what the
    /// listing is charged for. `head_status` lets a test refuse `HEAD`, as an
    /// origin that serves `GET` alone does.
    struct PeerOrigin {
        base_url: String,
        probes: Arc<AtomicUsize>,
    }

    impl PeerOrigin {
        async fn start(revisions: u64, head_status: StatusCode) -> Self {
            Self::holding((0..revisions).collect(), head_status).await
        }

        async fn holding(held: HashSet<u64>, head_status: StatusCode) -> Self {
            let probes = Arc::new(AtomicUsize::new(0));
            let counter = Arc::clone(&probes);
            let held = Arc::new(held);
            let app = Router::new().fallback(move |method: Method, uri: Uri| {
                let counter = Arc::clone(&counter);
                let held = Arc::clone(&held);
                async move {
                    let held = uri
                        .path()
                        .rsplit('/')
                        .next()
                        .and_then(|revision| revision.parse::<u64>().ok())
                        .is_some_and(|revision| held.contains(&revision));
                    if method == Method::HEAD {
                        counter.fetch_add(1, Ordering::SeqCst);
                        if head_status != StatusCode::OK {
                            return (head_status, Vec::new());
                        }
                        let status = if held {
                            StatusCode::OK
                        } else {
                            StatusCode::NOT_FOUND
                        };
                        return (status, Vec::new());
                    }
                    if !held {
                        return (StatusCode::NOT_FOUND, Vec::new());
                    }
                    if method == Method::GET {
                        counter.fetch_add(1, Ordering::SeqCst);
                    }
                    (StatusCode::OK, MANIFEST_BODY.to_vec())
                }
            });
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let base_url = format!("http://{}", listener.local_addr().unwrap());
            tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
            Self { base_url, probes }
        }

        fn request(&self, highest_archived_revision: Option<u64>) -> PeerDownloadRequest {
            PeerDownloadRequest {
                publisher: Address::ZERO,
                s3_bucket_url: self.base_url.clone(),
                version: ManifestVersion::V1,
                consensus_epoch: "1/block_1".to_owned(),
                coprocessor_context_id: U256::from(1),
                host_chain_id: 1,
                publication_block_number: 1,
                publication_block_hash: B256::ZERO,
                highest_archived_revision,
                rejected_object_keys: HashSet::new(),
            }
        }
    }

    const MANIFEST_BODY: &[u8] = b"signed-manifest";

    fn revisions_of(keys: &[String]) -> Vec<u64> {
        keys.iter()
            .map(|key| key.rsplit('/').next().unwrap().parse().unwrap())
            .collect()
    }

    #[tokio::test]
    async fn one_revision_costs_two_probes() {
        let origin = PeerOrigin::start(1, StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let keys = source.list_manifests(&origin.request(None)).await.unwrap();

        assert_eq!(revisions_of(&keys), vec![0]);
        // One probe finds revision 0 and the next proves the peer holds no
        // more. A listing that walked the whole budget would cost far more
        // against every peer.
        assert_eq!(origin.probes.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn an_empty_peer_costs_two_probes() {
        let origin = PeerOrigin::start(0, StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let keys = source.list_manifests(&origin.request(None)).await.unwrap();

        assert!(keys.is_empty());
        assert_eq!(origin.probes.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn a_revision_above_the_first_is_an_error() {
        // A second revision would go unread, and a revision above an absent
        // first would pin this peer to a stale manifest for good. Both must
        // name themselves instead of reading as an empty set.
        for held in [HashSet::from([0, 1]), HashSet::from([1])] {
            let origin = PeerOrigin::holding(held, StatusCode::OK).await;
            let source = HttpPeerManifestSource::new(reqwest::Client::new());
            let error = source
                .list_manifests(&origin.request(None))
                .await
                .expect_err("a revision above the first must not be ignored");

            assert!(matches!(error, ExecutionError::InternalError(_)), "{error}");
        }
    }

    #[tokio::test]
    async fn the_archived_floor_picks_the_revision_above_it() {
        let origin = PeerOrigin::start(4, StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let keys = source
            .list_manifests(&origin.request(Some(2)))
            .await
            .unwrap();

        assert_eq!(revisions_of(&keys), vec![3]);
    }

    #[tokio::test]
    async fn a_refused_head_falls_back_to_a_ranged_get() {
        let origin = PeerOrigin::start(1, StatusCode::METHOD_NOT_ALLOWED).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let keys = source.list_manifests(&origin.request(None)).await.unwrap();

        assert_eq!(revisions_of(&keys), vec![0]);
    }

    #[tokio::test]
    async fn a_forbidden_probe_is_an_error_not_the_end_of_the_set() {
        let origin = PeerOrigin::start(1, StatusCode::FORBIDDEN).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let error = source
            .list_manifests(&origin.request(None))
            .await
            .expect_err("403 must not read as an empty peer");

        assert!(
            matches!(error, ExecutionError::S3TransientError(_)),
            "{error}"
        );
    }

    #[tokio::test]
    async fn fetch_returns_the_signed_bytes() {
        let origin = PeerOrigin::start(1, StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let request = origin.request(None);
        let keys = source.list_manifests(&request).await.unwrap();
        let object = source.fetch_manifest(&request, &keys[0]).await.unwrap();

        assert_eq!(object.object_key, keys[0]);
        assert_eq!(object.signed_bytes, MANIFEST_BODY);
    }

    #[tokio::test]
    async fn a_missing_object_key_reports_not_found() {
        let origin = PeerOrigin::start(0, StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let error = source
            .fetch_manifest(&origin.request(None), "manifests/v_1/revision/0")
            .await
            .expect_err("an absent manifest is not found");

        assert!(
            matches!(error, ExecutionError::S3ObjectNotFound(_)),
            "{error}"
        );
    }

    #[test]
    fn joins_an_object_key_onto_the_registered_base() {
        for base in [
            "https://coprocessor-3.dev.zama.cloud",
            "https://coprocessor-3.dev.zama.cloud/",
        ] {
            assert_eq!(
                peer_http::object_url(base, "manifests/v_1/revision/0"),
                "https://coprocessor-3.dev.zama.cloud/manifests/v_1/revision/0"
            );
        }
    }
}
