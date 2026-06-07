//! Model-2 (EVM-parity) Solana user-decryption runner.
//!
//! Bridges a Solana-native user-decryption REQUEST (ed25519 `signMessage`, seam a) to the
//! standard KMS-core user-decryption, then stores the standard signcrypted response for the
//! relayer to return to the user (who de-signcrypts off-chain — exactly as on EVM). Only the
//! request channel is Solana-native; the response mirrors EVM (no ed25519 envelope).
//!
//! Flow per request:
//!  1. `solana_flow` admits the request: RPC-verified on-chain ACL + ed25519 `signMessage`
//!     (the authorization — the KMS core trusts the connector for ACL on both EVM and Solana).
//!  2. Build a `UserDecryptionRequest` with the Solana identity carried as
//!     `client_address = "solana:" + hex(pubkey)` (the kms-core branch binds the response via
//!     `compute_link_solana`), the user's ML-KEM `enc_key`, and the handles' SNS ciphertext
//!     material fetched from S3.
//!  3. Call the KMS core and store the standard signcrypted response keyed by the request hash.

use std::sync::Arc;

use kms_grpc::kms::v1::{RequestId, UserDecryptionRequest};
use sqlx::{Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use connector_utils::types::{KmsGrpcRequest, KmsGrpcResponse};

use crate::core::event_processor::s3::S3Service;
use crate::core::event_processor::KmsClient;
use crate::core::solana_acl::{SolanaAclVerifier, SolanaNativeRequestLimits};
use crate::core::solana_flow::{SolanaNativeLiveRequestProcessor, SolanaNativeLiveRequestReleaseV0};
use crate::core::solana_live::{SolanaNativeLiveRequestPolicy, SolanaNativeAccountFetcher};
use crate::core::solana_native::SolanaNativeRequestAdmission;
use crate::core::solana_replay::SolanaNativeReplayStore;
use crate::core::solana_rpc::SolanaJsonRpcAccountFetcher;
use crate::core::solana_store::DbSolanaNativeDecryptionStore;

use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;

/// How many pending native requests to claim per poll.
const PICK_BATCH: u8 = 8;

/// Runs the Model-2 Solana user-decryption processing loop.
pub struct SolanaUserDecryptRunner<S, P: alloy::providers::Provider> {
    store: DbSolanaNativeDecryptionStore,
    processor: SolanaNativeLiveRequestProcessor<S, SolanaJsonRpcAccountFetcher>,
    kms_client: KmsClient,
    s3_service: S3Service<P>,
    /// Coprocessor tx-sender addresses (S3 material sources) for SNS ciphertext retrieval.
    coprocessor_tx_sender_addresses: Vec<alloy::primitives::Address>,
    db_pool: Pool<Postgres>,
    poll_interval: std::time::Duration,
}

impl<S, P> SolanaUserDecryptRunner<S, P>
where
    S: SolanaNativeReplayStore + Sync + Send + 'static,
    P: alloy::providers::Provider + Clone + Send + Sync + 'static,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        store: DbSolanaNativeDecryptionStore,
        processor: SolanaNativeLiveRequestProcessor<S, SolanaJsonRpcAccountFetcher>,
        kms_client: KmsClient,
        s3_service: S3Service<P>,
        coprocessor_tx_sender_addresses: Vec<alloy::primitives::Address>,
        db_pool: Pool<Postgres>,
        poll_interval: std::time::Duration,
    ) -> Self {
        Self {
            store,
            processor,
            kms_client,
            s3_service,
            coprocessor_tx_sender_addresses,
            db_pool,
            poll_interval,
        }
    }

    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting Solana user-decryption runner (Model-2 EVM-parity)");
        let runner = Arc::new(self);
        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    info!("Stopping Solana user-decryption runner");
                    return;
                }
                _ = tokio::time::sleep(runner.poll_interval) => {
                    if let Err(e) = runner.poll_once().await {
                        error!("Solana user-decryption poll failed: {e}");
                    }
                }
            }
        }
    }

    async fn poll_once(&self) -> anyhow::Result<()> {
        let requests = self.store.pick_pending_requests_v0(PICK_BATCH).await?;
        for request in requests {
            let request_hash = request.request_hash;
            if let Err(e) = self.process_request(&request).await {
                warn!(
                    "Solana user-decryption request {} failed: {e}",
                    alloy::hex::encode(request_hash)
                );
                let _ = self.store_failure(&request_hash, &e.to_string()).await;
            }
        }
        Ok(())
    }

    async fn process_request(
        &self,
        request: &crate::core::solana_store::SolanaNativeDbRequestV0,
    ) -> anyhow::Result<()> {
        let release = self
            .processor
            .admit_request_bytes_for_release_v0(&request.request_bytes)
            .await?;

        // Fetch the SNS ciphertext material for each handle from S3, then assemble the request.
        let sns_materials = self.build_sns_materials(&release);
        let typed_ciphertexts = self
            .s3_service
            .retrieve_sns_ciphertext_materials(&sns_materials)
            .await
            .map_err(|e| anyhow::anyhow!("SNS ciphertext retrieval failed: {e}"))?;
        let grpc_request = self.build_user_decryption_request(&release, typed_ciphertexts)?;
        let kms_request = KmsGrpcRequest::UserDecryption(grpc_request);

        let (_, send_result) = self.kms_client.send_request(&kms_request).await;
        send_result?;
        let (_, poll_result) = self.kms_client.poll_result(kms_request).await;
        let response = poll_result?;
        let raw_response_body = match response {
            KmsGrpcResponse::UserDecryption { grpc_response, .. } => {
                bc2wrap::serialize(&grpc_response)?
            }
            other => anyhow::bail!("unexpected KMS response kind for user decryption: {other:?}"),
        };

        self.store_response(&request.request_hash, &raw_response_body)
            .await?;
        info!(
            "Solana user-decryption request {} completed",
            alloy::hex::encode(request.request_hash)
        );
        Ok(())
    }

    /// SNS ciphertext material per handle: handle + the witness-committed key_id/sns digest +
    /// the configured coprocessor tx-sender S3 sources.
    fn build_sns_materials(
        &self,
        release: &SolanaNativeLiveRequestReleaseV0,
    ) -> Vec<SnsCiphertextMaterial> {
        release
            .parsed_request
            .entries
            .iter()
            .map(|entry| SnsCiphertextMaterial {
                ctHandle: alloy::primitives::FixedBytes::<32>::from(entry.handle),
                keyId: alloy::primitives::U256::from_be_bytes(entry.expected_key_id),
                snsCiphertextDigest: alloy::primitives::FixedBytes::<32>::from(
                    entry.material_commitment_hash,
                ),
                coprocessorTxSenderAddresses: self.coprocessor_tx_sender_addresses.clone(),
            })
            .collect()
    }

    fn build_user_decryption_request(
        &self,
        release: &SolanaNativeLiveRequestReleaseV0,
        typed_ciphertexts: Vec<kms_grpc::kms::v1::TypedCiphertext>,
    ) -> anyhow::Result<UserDecryptionRequest> {
        let parsed = &release.parsed_request;
        let payload = &parsed.payload;
        let first = parsed
            .entries
            .first()
            .ok_or_else(|| anyhow::anyhow!("Solana user-decrypt request has no handles"))?;

        // Solana identity → "solana:<hex>" client_address (the kms-core branch parses this and
        // binds the response via compute_link_solana; EVM client_address parsing is bypassed).
        let client_address = format!("solana:{}", alloy::hex::encode(payload.request_signer_pubkey));

        Ok(UserDecryptionRequest {
            request_id: Some(RequestId {
                request_id: alloy::hex::encode(payload.nonce),
            }),
            typed_ciphertexts,
            key_id: Some(RequestId {
                request_id: alloy::hex::encode(first.expected_key_id),
            }),
            client_address,
            enc_key: parsed.user_reencryption_public_key.clone(),
            // No EVM domain on the Solana path; the kms-core branch uses a placeholder domain and
            // binds via compute_link_solana, so the request carries none.
            domain: None,
            extra_data: parsed.raw_extra_data.clone(),
            context_id: Some(RequestId {
                request_id: alloy::hex::encode(payload.kms_context_id),
            }),
            epoch_id: None,
        })
    }

    async fn store_response(
        &self,
        request_hash: &[u8; 32],
        raw_response_body: &[u8],
    ) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO solana_user_decrypt_responses_v0 (request_hash, raw_response_body, status)
             VALUES ($1, $2, 'completed')
             ON CONFLICT (request_hash) DO UPDATE
               SET raw_response_body = EXCLUDED.raw_response_body,
                   status = 'completed',
                   updated_at = NOW()",
        )
        .bind(request_hash.as_slice())
        .bind(raw_response_body)
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn store_failure(&self, request_hash: &[u8; 32], reason: &str) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO solana_user_decrypt_responses_v0 (request_hash, raw_response_body, status, error_reason)
             VALUES ($1, ''::bytea, 'failed', $2)
             ON CONFLICT (request_hash) DO UPDATE
               SET status = 'failed', error_reason = EXCLUDED.error_reason, updated_at = NOW()",
        )
        .bind(request_hash.as_slice())
        .bind(reason)
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }
}

/// Builds the request admission processor from the live Solana ACL config.
pub fn build_solana_user_decrypt_processor(
    verifier: SolanaAclVerifier,
    replay_store: impl SolanaNativeReplayStore,
    limits: SolanaNativeRequestLimits,
    policy: SolanaNativeLiveRequestPolicy,
    rpc_url: impl Into<String>,
) -> SolanaNativeLiveRequestProcessor<impl SolanaNativeReplayStore, SolanaJsonRpcAccountFetcher> {
    let admission = SolanaNativeRequestAdmission::new(verifier, replay_store, limits);
    let fetcher = SolanaJsonRpcAccountFetcher::new(rpc_url);
    SolanaNativeLiveRequestProcessor::new(limits, policy, admission, fetcher)
}

// Silence unused-import lints for items kept for the next wiring step.
#[allow(unused_imports)]
use SolanaNativeAccountFetcher as _AccountFetcherTraitInScope;
