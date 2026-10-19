use crate::{
    config::HttpConfig,
    http::types::{HttpDecryptionRequest, RequestOutcome},
};
use alloy::{
    primitives::{Address, B256},
    signers::local::PrivateKeySigner,
    sol_types::Eip712Domain,
};
use anyhow::{Context, anyhow};
use fhevm_sig_auth::PathAndQuery;
use kms_connector_api::{
    ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE, VersionResponse,
};
use reqwest::{
    Certificate, Client, StatusCode, Url,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue},
};
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::time::Instant;
use tracing::{debug, trace, warn};

/// Timeout used for connection establishment and health checks.
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// The `FhevmSig` material shared by the clients of every party.
pub struct FhevmSigAuth {
    pub signer: PrivateKeySigner,
    pub domain: Eip712Domain,
    pub validity: Duration,
}

/// A decryption request serialized and signed for one party, ready to be sent.
pub struct SignedRequest {
    route: &'static str,
    body: Vec<u8>,
    authorization: HeaderValue,
    decryption_id: B256,
    handle_count: usize,
}

/// HTTP client of one KMS party's connector, reached through its proxy.
#[derive(Clone)]
pub struct HttpConnector {
    base_url: Url,
    client: Client,
    auth: Arc<FhevmSigAuth>,
    /// This party's KMS tx-sender address, the `audience` its proxy accepts.
    audience: Address,
}

impl HttpConnector {
    pub fn connect(
        http_config: &HttpConfig,
        index: usize,
        auth: Arc<FhevmSigAuth>,
    ) -> anyhow::Result<Self> {
        let base_url = http_config.urls[index].clone();
        let audience = http_config.tx_sender_addresses[index];

        let mut builder = Client::builder()
            .use_rustls_tls()
            .timeout(http_config.request_timeout)
            .connect_timeout(CONNECTION_TIMEOUT)
            .danger_accept_invalid_certs(http_config.danger_accept_invalid_certs);

        if let Some(path) = &http_config.tls_ca_cert {
            let pem = std::fs::read(path)
                .with_context(|| format!("Failed to read TLS CA cert {}", path.display()))?;
            let certs = Certificate::from_pem_bundle(&pem)
                .with_context(|| format!("Invalid PEM bundle {}", path.display()))?;
            builder = builder.tls_certs_merge(certs);
        }

        debug!("Configured HTTP client #{index} ({base_url}, audience {audience})");
        Ok(Self {
            base_url,
            client: builder.build()?,
            auth,
            audience,
        })
    }

    /// The `FhevmSig` `Authorization` value for `body` sent to `route`.
    async fn authorization(&self, route: &'static str, body: &[u8]) -> anyhow::Result<HeaderValue> {
        let header = fhevm_sig_auth::sign_valid_for(
            &self.auth.signer,
            &self.auth.domain,
            self.audience,
            &PathAndQuery::from_static(route),
            body,
            self.auth.validity,
        )
        .await
        .with_context(|| format!("{}: failed to sign {route}", self.base_url))?;
        Ok(header.to_header_value())
    }

    fn route(&self, route: &str) -> anyhow::Result<Url> {
        self.base_url
            .join(route)
            .with_context(|| format!("{}: invalid route {route}", self.base_url))
    }

    /// `GET /version`: checks the party is reachable.
    pub async fn health_check(&self) -> anyhow::Result<()> {
        let authorization = self.authorization(VERSION_ROUTE, &[]).await?;
        let resp = self
            .client
            .get(self.route(VERSION_ROUTE)?)
            .header(AUTHORIZATION, authorization)
            .timeout(CONNECTION_TIMEOUT)
            .send()
            .await
            .with_context(|| format!("{}: version request failed", self.base_url))?;
        let status = resp.status();
        if status != StatusCode::OK {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!(
                "{}: GET {VERSION_ROUTE} -> {status} {body}",
                self.base_url
            ));
        }
        let version: VersionResponse = resp.json().await?;
        debug!("{}: interface version {}", self.base_url, version.version);
        Ok(())
    }

    /// Serializes `request` and signs its `FhevmSig` header, ready to be sent to this party.
    pub async fn sign(&self, request: &HttpDecryptionRequest) -> anyhow::Result<SignedRequest> {
        let route = match request {
            HttpDecryptionRequest::Public(_) => PUBLIC_DECRYPTION_ROUTE,
            HttpDecryptionRequest::UserV2(_) => USER_DECRYPTION_ROUTE,
        };
        // The proxy checks the digest of the exact bytes received: sign the bytes that are sent.
        let body = match request {
            HttpDecryptionRequest::Public(body) => serde_json::to_vec(body)?,
            HttpDecryptionRequest::UserV2(body) => serde_json::to_vec(body)?,
        };
        let authorization = self.authorization(route, &body).await?;
        Ok(SignedRequest {
            route,
            body,
            authorization,
            decryption_id: request.id(),
            handle_count: request.handle_count(),
        })
    }

    /// Sends one signed decryption request and waits for the party's answer.
    pub async fn send_signed(&self, request: SignedRequest) -> anyhow::Result<RequestOutcome> {
        let SignedRequest {
            route,
            body,
            authorization,
            decryption_id,
            handle_count,
        } = request;

        let started = Instant::now();
        let resp = self
            .client
            .post(self.route(route)?)
            .header(AUTHORIZATION, authorization)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .with_context(|| {
                format!("{}: POST {route} failed for {decryption_id}", self.base_url)
            })?;
        let http_status = resp.status().as_u16();
        let bytes = resp.bytes().await?;
        let elapsed = started.elapsed();

        if http_status == 200 {
            trace!(%decryption_id, ?elapsed, "{}: decryption succeeded", self.base_url);
            return Ok(RequestOutcome {
                decryption_id,
                http_status,
                elapsed,
                handle_count,
                error: None,
            });
        }

        let error = serde_json::from_slice::<ErrorResponse>(&bytes).ok();
        warn!(
            %decryption_id,
            "{}: POST {route} -> {http_status} {}",
            self.base_url,
            error
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| String::from_utf8_lossy(&bytes).into_owned()),
        );
        Ok(RequestOutcome {
            decryption_id,
            http_status,
            elapsed,
            handle_count: 0,
            error,
        })
    }
}

impl Display for HttpConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base_url)
    }
}
