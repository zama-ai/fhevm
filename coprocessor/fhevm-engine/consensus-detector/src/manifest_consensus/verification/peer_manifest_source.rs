use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use alloy_primitives::{Address, B256, U256};
use block_manifest::{ManifestVersion, MAX_MANIFEST_BYTES};
use tracing::{debug, info, warn};

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
    Refused,
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
            // A bucket that grants reads alone answers 403 for a key it does
            // not hold, and some origins refuse `HEAD`. A ranged `GET` decides.
            reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::METHOD_NOT_ALLOWED => {
                Ok(Probe::Refused)
            }
            _ => Err(ExecutionError::S3TransientError(format!(
                "peer manifest probe {object_key} at {url} returned {status}"
            ))),
        }
    }

    async fn probe_object(&self, url: &str, object_key: &str) -> Result<Probe, ExecutionError> {
        match self
            .send_probe(self.http.head(url), url, object_key)
            .await?
        {
            Probe::Refused => {}
            probe => return Ok(probe),
        }
        // An origin may serve GET alone. One byte answers the same question and
        // still reads no manifest.
        let ranged = self
            .http
            .get(url)
            .header(reqwest::header::RANGE, "bytes=0-0");
        self.send_probe(ranged, url, object_key).await
    }

    /// One probe, refused once the listing budget is gone.
    async fn probe_within(
        &self,
        deadline: Instant,
        request: &PeerDownloadRequest,
        prefix: &str,
        revision: u64,
    ) -> Result<Probe, ExecutionError> {
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
        tokio::time::timeout(remaining, self.probe_object(&url, &object_key))
            .await
            .map_err(|_| spent())?
    }

    /// The revisions the peer holds at `first` and just above it, newest first.
    ///
    /// A block carries one revision today, so a pair covers every set. An
    /// absent `first` does not end the set: the revision above it supersedes
    /// the missing one. A peer that holds `above` gets one more probe, which
    /// names a publisher that outgrew this pair. A peer that skips a revision
    /// gets no such probe, so {0, 2} reads as 0 alone.
    async fn held_revisions(
        &self,
        request: &PeerDownloadRequest,
        prefix: &str,
        first: u64,
    ) -> Result<Vec<u64>, ExecutionError> {
        let deadline = Instant::now() + MANIFEST_LIST_BUDGET;
        let above = first.saturating_add(1);
        let holds_first = match self.probe_within(deadline, request, prefix, first).await? {
            Probe::Present => true,
            Probe::Absent => false,
            // This peer may hold the manifest and grant no way to read it, so
            // name it once. A refusal above `first` is what a bucket that
            // grants reads alone answers for a key it does not hold.
            Probe::Refused => {
                warn!(
                    bucket_url = request.s3_bucket_url,
                    revision = first,
                    "peer refused the probe of its first revision; reading it as absent"
                );
                false
            }
        };
        let mut held = Vec::new();
        if matches!(
            self.probe_within(deadline, request, prefix, above).await?,
            Probe::Present
        ) {
            let beyond = above.saturating_add(1);
            if matches!(
                self.probe_within(deadline, request, prefix, beyond).await?,
                Probe::Present
            ) {
                return Err(ExecutionError::InternalError(format!(
                    "peer manifests from {} hold revision {beyond}, above the pair this reader reads",
                    request.s3_bucket_url
                )));
            }
            held.push(above);
        }
        if holds_first {
            held.push(first);
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
        let mut keys = Vec::new();
        for revision in self.held_revisions(request, &prefix, first).await? {
            let object_key = format!("{prefix}{revision}");
            if request.considers_object(&object_key, revision) {
                keys.push(object_key);
            }
        }
        Ok(keys)
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
            Self::answering(held, head_status, StatusCode::NOT_FOUND).await
        }

        /// What the origin answers for a key it does not hold. A bucket that
        /// grants reads alone answers 403, not 404.
        async fn answering(
            held: HashSet<u64>,
            head_status: StatusCode,
            absent_status: StatusCode,
        ) -> Self {
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
                        let status = if held { StatusCode::OK } else { absent_status };
                        return (status, Vec::new());
                    }
                    if !held {
                        return (absent_status, Vec::new());
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
    async fn a_missing_first_revision_does_not_hide_the_one_above_it() {
        // Revision 1 supersedes the missing revision 0. A refusal here would
        // spend the retries of the task on evidence that is already present.
        let origin = PeerOrigin::holding(HashSet::from([1]), StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let keys = source.list_manifests(&origin.request(None)).await.unwrap();

        assert_eq!(revisions_of(&keys), vec![1]);
    }

    #[tokio::test]
    async fn both_revisions_of_the_pair_are_listed_newest_first() {
        // The caller falls back through this order.
        let origin = PeerOrigin::start(2, StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let keys = source.list_manifests(&origin.request(None)).await.unwrap();

        assert_eq!(revisions_of(&keys), vec![1, 0]);
        assert_eq!(origin.probes.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn a_revision_above_the_pair_is_an_error() {
        // A result of 1 and 0 would leave this peer on older evidence, and say
        // nothing.
        let origin = PeerOrigin::start(3, StatusCode::OK).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let error = source
            .list_manifests(&origin.request(None))
            .await
            .expect_err("a revision above the pair must not be ignored");

        assert!(matches!(error, ExecutionError::InternalError(_)), "{error}");
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
        for refusal in [StatusCode::METHOD_NOT_ALLOWED, StatusCode::FORBIDDEN] {
            let origin = PeerOrigin::start(1, refusal).await;
            let source = HttpPeerManifestSource::new(reqwest::Client::new());
            let keys = source.list_manifests(&origin.request(None)).await.unwrap();

            assert_eq!(revisions_of(&keys), vec![0], "{refusal}");
        }
    }

    #[tokio::test]
    async fn a_bucket_that_forbids_a_missing_key_still_lists_what_it_holds() {
        // Refusing this peer would drop a manifest that it serves.
        let origin =
            PeerOrigin::answering(HashSet::from([0]), StatusCode::OK, StatusCode::FORBIDDEN).await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());
        let keys = source.list_manifests(&origin.request(None)).await.unwrap();

        assert_eq!(revisions_of(&keys), vec![0]);
    }

    #[tokio::test]
    async fn a_peer_that_refuses_every_read_lists_nothing() {
        // The peer counts as missing either way, so an error here would only
        // spend the retries of the task on a grant that never arrives.
        let origin =
            PeerOrigin::answering(HashSet::new(), StatusCode::FORBIDDEN, StatusCode::FORBIDDEN)
                .await;
        let source = HttpPeerManifestSource::new(reqwest::Client::new());

        assert!(source
            .list_manifests(&origin.request(None))
            .await
            .unwrap()
            .is_empty());
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
