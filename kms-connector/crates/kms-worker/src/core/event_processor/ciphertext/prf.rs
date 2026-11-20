//! Retrieval of PRF outputs from the Coprocessors' PRF endpoints.

use super::pool::Slot;
use crate::core::Config;
use alloy::{
    primitives::{Address, keccak256},
    sol_types::Eip712Domain,
    transports::http::reqwest::{
        Client, Method, Response, StatusCode,
        header::{AUTHORIZATION, WWW_AUTHENTICATE},
    },
};
use anyhow::{Context, anyhow, bail};
use bytes::Bytes;
use ciphertext_attestation::{
    ConsensusRound, CoprocessorEntry, CoprocessorRegistrySnapshot, PrfOutputAttestation,
    PrfOutputRef, ResolvedConsensus,
};
use connector_utils::config::KmsWallet;
use coprocessor_api::{PRF_ATTESTATION_HEADER, prf_output_path};
use fhevm_sig_auth::{FhevmSigHeader, PathAndQuery, SignError};
use futures::{StreamExt, future::try_join_all, stream::FuturesUnordered};
use rand::seq::SliceRandom;
use std::{collections::HashMap, time::Duration};
use tracing::warn;
use url::Url;

/// Signs and sends the requests to the Coprocessors' PRF endpoints.
#[derive(Clone)]
pub struct PrfClient {
    client: Client,
    wallet: KmsWallet,
    /// The `FhevmSig` domain.
    domain: Eip712Domain,
    /// Time budget of the retrieval of one PRF output, and validity of its signatures.
    pub(super) deadline: Duration,
}

impl PrfClient {
    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
        // Boxed: the AWS SDK futures are otherwise too deep for rustc in the worker's startup.
        let wallet = Box::pin(KmsWallet::from_config(
            config.private_key.as_ref(),
            config.aws_kms_config.clone(),
            None,
        ))
        .await?;
        Ok(Self::new(
            wallet,
            fhevm_sig_auth::domain(
                config.ethereum_chain_id,
                config.protocol_config_contract.address,
            ),
            config.prf_retrieval_deadline,
        ))
    }

    pub(super) fn new(wallet: KmsWallet, domain: Eip712Domain, deadline: Duration) -> Self {
        Self {
            client: Client::new(),
            wallet,
            domain,
            deadline,
        }
    }

    /// Retrieves the PRF output of `output` into `slot`, as attested by a majority of `registry`.
    ///
    /// The body is asked from the Coprocessor at index `body_source`, picked at random by the
    /// caller, and from the members of the quorum if it does not match the attested digest.
    /// `endpoint` gives the base URL of the PRF endpoint of a Coprocessor.
    pub(super) async fn retrieve(
        &self,
        output: PrfOutputRef,
        registry: &CoprocessorRegistrySnapshot,
        body_source: usize,
        mut slot: Slot,
        // Will disappear once endpoint lands in `CoprocessorEntry`
        endpoint: impl Fn(&CoprocessorEntry) -> Url,
    ) -> anyhow::Result<Bytes> {
        let path = &PathAndQuery::try_from(prf_output_path(&output))?;
        let coprocessors = &registry.coprocessors;
        let auths = &self.sign_all(coprocessors, path).await?;
        let endpoint = &endpoint;

        let mut round = ConsensusRound::<PrfOutputAttestation>::open(
            output,
            coprocessors.iter().cloned(),
            registry.threshold,
        );
        let mut buffer = Some(slot.buffer_mut());
        let mut served = None;
        let consensus = {
            // Dropped at the end of the block: cancels the `GET` to `body_source` if still
            // running.
            let mut requests: FuturesUnordered<_> = coprocessors
                .iter()
                .enumerate()
                .map(|(i, coprocessor)| {
                    let buffer = buffer.take_if(|_| i == body_source);
                    async move {
                        let auth = &auths[&coprocessor.signer];
                        let url = endpoint(coprocessor);
                        let reply = match buffer {
                            Some(buffer) => self
                                .get(url, path, auth, buffer)
                                .await
                                .map(|(attestation, len)| Reply::Get(attestation, len)),
                            None => self.head(url, path, auth).await.map(Reply::Head),
                        };
                        (coprocessor.signer, reply)
                    }
                })
                .collect();
            loop {
                let Some((signer, reply)) = requests.next().await else {
                    break round.close();
                };
                let outcome = match reply {
                    Ok(Reply::Head(attestation)) => round.record_attestation(signer, &attestation),
                    Ok(Reply::Get(attestation, len)) => {
                        served = Some(len);
                        round.record_attestation(signer, &attestation)
                    }
                    Err(e) => {
                        warn!(%signer, %output, "No PRF output attestation: {e:#}");
                        round.record_no_reply(signer)
                    }
                };
                if let Some(outcome) = outcome {
                    break outcome;
                }
            }
        }
        .map_err(|e| anyhow!("{e}"))?;

        match served {
            // PRF output was already served, with the attested digest, so we return it
            Some(len) if keccak256(&slot.buffer_mut()[..len]) == consensus.material => {
                Ok(slot.into_bytes(len))
            }
            // Fallback to the quorum otherwise
            _ => {
                self.get_from_quorum(output, consensus, path, auths, slot, endpoint)
                    .await
            }
        }
    }

    /// Gets the PRF output into `slot` from the members of the quorum of `consensus`, until one
    /// serves a body matching the attested digest.
    async fn get_from_quorum(
        &self,
        output: PrfOutputRef,
        consensus: ResolvedConsensus<PrfOutputAttestation>,
        path: &PathAndQuery,
        auths: &HashMap<Address, FhevmSigHeader>,
        mut slot: Slot,
        endpoint: impl Fn(&CoprocessorEntry) -> Url,
    ) -> anyhow::Result<Bytes> {
        let digest = consensus.material;
        let mut winners = consensus.winners;
        // Spreads the fallback `GET`s across the quorum instead of always hitting the same member.
        winners.shuffle(&mut rand::rng());
        for winner in &winners {
            let auth = &auths[&winner.signer];
            match self
                .get(endpoint(winner), path, auth, slot.buffer_mut())
                .await
            {
                Ok((_, len)) if keccak256(&slot.buffer_mut()[..len]) == digest => {
                    return Ok(slot.into_bytes(len));
                }
                Ok(_) => {
                    warn!(signer = %winner.signer, %output, "PRF output does not match the quorum digest")
                }
                Err(e) => {
                    warn!(signer = %winner.signer, %output, "Failed to get the PRF output: {e:#}")
                }
            }
        }
        bail!("no quorum member served the PRF output")
    }

    /// Signs a request to `path` for each of the `coprocessors`, keyed by signer.
    async fn sign_all(
        &self,
        coprocessors: &[CoprocessorEntry],
        path: &PathAndQuery,
    ) -> Result<HashMap<Address, FhevmSigHeader>, SignError> {
        try_join_all(coprocessors.iter().map(|coprocessor| async {
            let auth = fhevm_sig_auth::sign_valid_for(
                &self.wallet,
                &self.domain,
                coprocessor.signer,
                path,
                &[],
                self.deadline,
            )
            .await?;
            Ok((coprocessor.signer, auth))
        }))
        .await
        .map(HashMap::from_iter)
    }

    /// Sends a `HEAD` to the PRF endpoint at `url` and returns the attestation.
    async fn head(
        &self,
        url: Url,
        path: &PathAndQuery,
        auth: &FhevmSigHeader,
    ) -> anyhow::Result<PrfOutputAttestation> {
        Ok(self.send(Method::HEAD, url, path, auth).await?.0)
    }

    /// Sends a `GET` to the PRF endpoint at `url`, writes the body into `buffer`, and returns the
    /// attestation with the length of the body.
    async fn get(
        &self,
        url: Url,
        path: &PathAndQuery,
        auth: &FhevmSigHeader,
        buffer: &mut [u8],
    ) -> anyhow::Result<(PrfOutputAttestation, usize)> {
        let (attestation, mut response) = self.send(Method::GET, url, path, auth).await?;
        let mut len = 0;
        while let Some(chunk) = response.chunk().await? {
            let end = len + chunk.len();
            let Some(dst) = buffer.get_mut(len..end) else {
                bail!("PRF output larger than {} bytes", buffer.len());
            };
            dst.copy_from_slice(&chunk);
            len = end;
        }
        Ok((attestation, len))
    }

    /// Sends `method` to the PRF endpoint at `url`, and returns the attestation with the response.
    async fn send(
        &self,
        method: Method,
        url: Url,
        path: &PathAndQuery,
        auth: &FhevmSigHeader,
    ) -> anyhow::Result<(PrfOutputAttestation, Response)> {
        let url = url.join(path.as_str())?;
        let response = self
            .client
            .request(method, url)
            .header(AUTHORIZATION, auth.to_header_value())
            .send()
            .await?;
        if response.status() != StatusCode::OK {
            let challenge = response.headers().get(WWW_AUTHENTICATE);
            bail!("status {}, challenge {challenge:?}", response.status());
        }

        let attestation = response
            .headers()
            .get(PRF_ATTESTATION_HEADER)
            .map(|value| serde_json::from_slice(value.as_bytes()))
            .context("no attestation header")??;
        Ok((attestation, response))
    }
}

/// A reply to a first-round request.
enum Reply {
    Head(PrfOutputAttestation),
    // Attestation + body length
    Get(PrfOutputAttestation, usize),
}

/// The base URL of the PRF endpoint of `coprocessor`.
// TODO: read the URL from the `CoprocessorConfig` contract once it registers the PRF endpoints.
pub(super) fn prf_endpoint(coprocessor: &CoprocessorEntry) -> Url {
    Url::parse(&format!("http://{:x}.prf.invalid", coprocessor.signer))
        .expect("an address is a valid host label")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ciphertext::pool::CiphertextPool;
    use alloy::{primitives::B256, signers::local::PrivateKeySigner};
    use ciphertext_attestation::{PrfOutputAttestationPayload, prf::Version};
    use rstest::rstest;
    use std::num::NonZeroUsize;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
        task::JoinHandle,
    };

    // TODO: Review the tests once rebased on CoprocessorConfig migration!

    const BODY: &[u8] = b"PRF output";

    /// What a mock Coprocessor answers to a request.
    #[derive(Clone)]
    enum Answer {
        /// A `200` attesting `attested`, with `served` as the `GET` body.
        Output {
            attested: &'static [u8],
            served: &'static [u8],
        },
        /// An error: its status line, then its headers.
        Error(String),
    }

    fn ok() -> Option<Answer> {
        Some(Answer::Output {
            attested: BODY,
            served: BODY,
        })
    }

    fn output() -> PrfOutputRef {
        PrfOutputRef::new(3, B256::repeat_byte(0xaa), 256)
    }

    /// A Coprocessor giving `answer` to every request, or down without one. Returns its registry
    /// entry, its URL, and its accept loop to abort.
    async fn coprocessor(answer: Option<Answer>) -> (CoprocessorEntry, Url, JoinHandle<()>) {
        let signer = PrivateKeySigner::random();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap())
            .parse()
            .unwrap();
        let entry = CoprocessorEntry {
            tx_sender: Address::ZERO,
            signer: signer.address(),
            bucket: String::new(),
        };
        let Some(answer) = answer else {
            // Dropping the listener closes the port.
            return (entry, url, tokio::spawn(async {}));
        };
        let (head, body) = match answer {
            Answer::Output { attested, served } => {
                let attestation = PrfOutputAttestationPayload::new(
                    Version::V1,
                    output().prf_id,
                    output().label,
                    output().pad_length,
                    B256::ZERO,
                    keccak256(attested),
                )
                .sign(&signer)
                .await
                .unwrap();
                let head = format!(
                    "200 OK\r\n{PRF_ATTESTATION_HEADER}: {}\r\nContent-Length: {}",
                    serde_json::to_string(&attestation).unwrap(),
                    served.len()
                );
                (head, served)
            }
            Answer::Error(head) => (format!("{head}\r\nContent-Length: 0"), &[][..]),
        };

        let accept_loop = tokio::spawn(async move {
            while let Ok((mut stream, _)) = listener.accept().await {
                let mut request = [0u8; 4096];
                let _ = stream.read(&mut request).await;
                let head = format!("HTTP/1.1 {head}\r\nConnection: close\r\n\r\n");
                let _ = stream.write_all(head.as_bytes()).await;
                if !request.starts_with(b"HEAD") {
                    let _ = stream.write_all(body).await;
                }
                let _ = stream.shutdown().await;
            }
        });
        (entry, url, accept_loop)
    }

    /// Retrieves the output from Coprocessors giving `answers`, with the first one asked for the
    /// body. A Coprocessor with no answer is down.
    async fn retrieve(answers: Vec<Option<Answer>>, threshold: usize) -> anyhow::Result<Bytes> {
        let mut entries = Vec::new();
        let mut urls = HashMap::new();
        let mut accept_loops = Vec::new();
        for answer in answers {
            let (entry, url, accept_loop) = coprocessor(answer).await;
            urls.insert(entry.signer, url);
            entries.push(entry);
            accept_loops.push(accept_loop);
        }
        let registry = CoprocessorRegistrySnapshot::new(entries, threshold.try_into().unwrap());
        let client = PrfClient::new(
            KmsWallet::Local(PrivateKeySigner::random()),
            fhevm_sig_auth::domain(1, Address::ZERO),
            Duration::from_secs(60),
        );
        let pool = CiphertextPool::new(NonZeroUsize::MIN, NonZeroUsize::new(1024).unwrap());
        let slot = pool.reserve(1).await.unwrap().pop().unwrap();

        let result = tokio::time::timeout(
            Duration::from_secs(10),
            client.retrieve(output(), &registry, 0, slot, |c| urls[&c.signer].clone()),
        )
        .await
        .expect("the retrieval should end");
        accept_loops.iter().for_each(JoinHandle::abort);
        result
    }

    fn error() -> Option<Answer> {
        Some(Answer::Error("503 Service Unavailable".to_string()))
    }

    #[rstest]
    #[case::honest(vec![ok(), ok(), ok()], 2)]
    #[case::wrong_body_from_first(
        vec![Some(Answer::Output { attested: BODY, served: b"garbage" }), ok(), ok()],
        2
    )]
    #[case::first_down(vec![None, ok(), ok()], 2)]
    #[case::first_error(vec![error(), ok(), ok()], 2)]
    #[tokio::test]
    async fn the_output_attested_by_a_majority_is_retrieved(
        #[case] answers: Vec<Option<Answer>>,
        #[case] threshold: usize,
    ) {
        let body = retrieve(answers, threshold).await.unwrap();
        assert_eq!(&body[..], BODY);
    }

    #[rstest]
    #[case::disagreement(
        vec![ok(), Some(Answer::Output { attested: b"other", served: b"other" })],
        2
    )]
    #[case::error(vec![ok(), error()], 2)]
    #[tokio::test]
    async fn no_output_without_a_majority(
        #[case] answers: Vec<Option<Answer>>,
        #[case] threshold: usize,
    ) {
        assert!(retrieve(answers, threshold).await.is_err());
    }
}
