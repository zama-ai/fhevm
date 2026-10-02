use crate::{
    config::HttpConfig,
    http::types::{HttpDecryptionRequest, RequestOutcome},
};
use anyhow::{Context, anyhow};
use kms_connector_api::{
    ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE, VersionResponse,
};
use reqwest::{
    Certificate, Client, StatusCode, Url,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::Serialize;
use std::{fmt::Display, time::Duration};
use tokio::time::Instant;
use tracing::{debug, trace, warn};

/// HTTP client of one KMS party's connector, reached through its proxy (or endpoint).
#[derive(Clone)]
pub struct HttpConnector {
    base_url: Url,
    client: Client,
}

impl HttpConnector {
    pub fn connect(http_config: &HttpConfig, index: usize) -> anyhow::Result<Self> {
        let base_url = http_config.urls[index].clone();

        let mut headers = HeaderMap::new();
        if let Some(api_key) = http_config.api_key.as_deref().filter(|k| !k.is_empty()) {
            let mut value = HeaderValue::from_str(&format!("Bearer {api_key}"))
                .map_err(|e| anyhow!("Invalid API key: {e}"))?;
            value.set_sensitive(true);
            headers.insert(AUTHORIZATION, value);
        }

        let mut builder = Client::builder()
            .use_rustls_tls()
            .default_headers(headers)
            .timeout(http_config.request_timeout)
            .connect_timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(http_config.danger_accept_invalid_certs);

        if let Some(path) = &http_config.tls_ca_cert {
            let pem = std::fs::read(path)
                .with_context(|| format!("Failed to read TLS CA cert {}", path.display()))?;
            let certs = Certificate::from_pem_bundle(&pem)
                .with_context(|| format!("Invalid PEM bundle {}", path.display()))?;
            builder = builder.tls_certs_merge(certs);
        }

        debug!("Configured HTTP client #{index} ({base_url})");
        Ok(Self {
            base_url,
            client: builder.build()?,
        })
    }

    fn route(&self, route: &str) -> anyhow::Result<Url> {
        self.base_url
            .join(route)
            .with_context(|| format!("{}: invalid route {route}", self.base_url))
    }

    /// `GET /version`: checks the party is reachable.
    pub async fn health_check(&self) -> anyhow::Result<()> {
        let resp = self
            .client
            .get(self.route(VERSION_ROUTE)?)
            .timeout(Duration::from_secs(10))
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

    /// Sends one decryption request and waits for the party's answer.
    pub async fn send(&self, request: &HttpDecryptionRequest) -> anyhow::Result<RequestOutcome> {
        let id = request.id();
        let handle_count = request.handle_count();
        match request {
            HttpDecryptionRequest::Public(body) => {
                self.post(PUBLIC_DECRYPTION_ROUTE, body, id, handle_count)
                    .await
            }
            HttpDecryptionRequest::UserV2(body) => {
                self.post(USER_DECRYPTION_ROUTE, body, id, handle_count)
                    .await
            }
        }
    }

    async fn post<B: Serialize>(
        &self,
        route: &str,
        body: &B,
        decryption_id: alloy::primitives::B256,
        handle_count: usize,
    ) -> anyhow::Result<RequestOutcome> {
        let started = Instant::now();
        let resp = self
            .client
            .post(self.route(route)?)
            .json(body)
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
