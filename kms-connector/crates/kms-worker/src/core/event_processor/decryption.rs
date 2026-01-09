use crate::core::{
    config::Config,
    event_processor::{AclChecker, CoprocessorApi, ProcessingError},
};
use alloy::{
    primitives::{Address, FixedBytes, U256},
};
use connector_utils::types::{
    KmsGrpcRequest,
    fhe::{extract_fhe_type_from_handle, format_request_id},
};
use kms_grpc::kms::v1::{
    CiphertextFormat, Eip712DomainMsg, PublicDecryptionRequest, RequestId, TypedCiphertext,
    UserDecryptionRequest,
};
use sqlx::{Pool, Postgres};
use tracing::{info, warn};

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests (V2).
pub struct DecryptionProcessor<P: ProviderBounds> {
    /// The EIP712 domain of the `DecryptionRegistry` contract.
    domain: Eip712DomainMsg,

    /// Direct ACL checker for host chains.
    acl_checker: AclChecker<P>,

    /// Coprocessor API client for ciphertext retrieval.
    coprocessor_api: CoprocessorApi<P>,

    /// DB connection pool (used to store rejection metadata).
    db_pool: Pool<Postgres>,
}

pub trait ProviderBounds: alloy::providers::Provider + Clone + Send + Sync + 'static {}
impl<T> ProviderBounds for T where T: alloy::providers::Provider + Clone + Send + Sync + 'static {}

impl<P> DecryptionProcessor<P>
where
    P: ProviderBounds,
{
    pub fn new(
        config: &Config,
        acl_checker: AclChecker<P>,
        coprocessor_api: CoprocessorApi<P>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        let domain = Eip712DomainMsg {
            name: config.decryption_contract.domain_name.clone(),
            version: config.decryption_contract.domain_version.clone(),
            chain_id: U256::from(config.chain_id).to_be_bytes_vec(),
            verifying_contract: config.decryption_contract.address.to_string(),
            salt: None,
        };

        Self {
            acl_checker,
            coprocessor_api,
            db_pool,
            domain,
        }
    }

    pub async fn check_decryption_not_already_done(
        &self,
        request_id: U256,
    ) -> Result<(), ProcessingError> {
        // V2: DecryptionRegistry doesn't track decryption completion state on-chain
        // Decryption results are served via HTTP API, not on-chain transactions
        info!(
            "V2: Skipping on-chain decryption check for request_id: {}",
            request_id
        );
        Ok(())
    }

    pub async fn prepare_public_decryption_request(
        &self,
        request_id: U256,
        handles: &[FixedBytes<32>],
        contract_addresses: &[Address],
        chain_id: U256,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        self.ensure_handle_consistency(handles, contract_addresses)?;

        let chain_id_u64 = u256_to_u64(chain_id)?;
        for handle in handles {
            match self
                .acl_checker
                .check_public_access(chain_id_u64, *handle)
                .await
            {
                Ok(true) => (),
                Ok(false) => {
                    self.record_rejection(request_id, false, "ACL check failed", "ACL_DENIED")
                        .await;
                    return Err(ProcessingError::Irrecoverable(anyhow::anyhow!(
                        "ACL denied for public decryption"
                    )));
                }
                Err(err) => {
                    return Err(ProcessingError::Recoverable(err));
                }
            }
        }

        let ciphertexts = self.fetch_ciphertexts(handles).await?;
        let (key_id, epoch_id) = self.validate_key_and_epoch(&ciphertexts)?;
        self.store_epoch_id(request_id, false, epoch_id).await;

        let typed_ciphertexts = build_typed_ciphertexts(ciphertexts)?;
        let request_id_msg = Some(RequestId {
            request_id: format_request_id(request_id),
        });

        let public_decryption_request = PublicDecryptionRequest {
            request_id: request_id_msg,
            ciphertexts: typed_ciphertexts,
            key_id: Some(RequestId {
                request_id: format_request_id(key_id),
            }),
            domain: Some(self.domain.clone()),
            extra_data: vec![],
            epoch_id: Some(RequestId {
                request_id: format_request_id(epoch_id),
            }),
            context_id: None,
        };

        Ok(public_decryption_request.into())
    }

    pub async fn prepare_user_decryption_request(
        &self,
        request_id: U256,
        handles: &[FixedBytes<32>],
        contract_addresses: &[Address],
        user_address: Address,
        public_key: Vec<u8>,
        user_signature: Vec<u8>,
        chain_id: U256,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        self.ensure_handle_consistency(handles, contract_addresses)?;

        let chain_id_u64 = u256_to_u64(chain_id)?;
        for handle in handles {
            match self
                .acl_checker
                .check_user_access(chain_id_u64, *handle, user_address)
                .await
            {
                Ok(true) => (),
                Ok(false) => {
                    self.record_rejection(request_id, true, "ACL check failed", "ACL_DENIED")
                        .await;
                    return Err(ProcessingError::Irrecoverable(anyhow::anyhow!(
                        "ACL denied for user decryption"
                    )));
                }
                Err(err) => {
                    return Err(ProcessingError::Recoverable(err));
                }
            }
        }

        let ciphertexts = self.fetch_ciphertexts(handles).await?;
        let (key_id, epoch_id) = self.validate_key_and_epoch(&ciphertexts)?;
        self.store_epoch_id(request_id, true, epoch_id).await;

        let typed_ciphertexts = build_typed_ciphertexts(ciphertexts)?;
        let request_id_msg = Some(RequestId {
            request_id: format_request_id(request_id),
        });

        let user_decryption_request = UserDecryptionRequest {
            request_id: request_id_msg,
            client_address: user_address.to_checksum(None),
            key_id: Some(RequestId {
                request_id: format_request_id(key_id),
            }),
            domain: Some(self.domain.clone()),
            enc_key: public_key,
            typed_ciphertexts,
            extra_data: user_signature,
            epoch_id: Some(RequestId {
                request_id: format_request_id(epoch_id),
            }),
            context_id: None,
        };

        Ok(user_decryption_request.into())
    }

    fn ensure_handle_consistency(
        &self,
        handles: &[FixedBytes<32>],
        contract_addresses: &[Address],
    ) -> Result<(), ProcessingError> {
        if handles.is_empty() {
            return Err(ProcessingError::Recoverable(anyhow::anyhow!(
                "No handles provided for decryption request"
            )));
        }
        if handles.len() != contract_addresses.len() {
            return Err(ProcessingError::Irrecoverable(anyhow::anyhow!(
                "Handles length does not match contract addresses length"
            )));
        }
        Ok(())
    }

    async fn fetch_ciphertexts(
        &self,
        handles: &[FixedBytes<32>],
    ) -> Result<Vec<crate::core::event_processor::coprocessor_api::CiphertextMaterial>, ProcessingError>
    {
        let mut ciphertexts = Vec::with_capacity(handles.len());
        for handle in handles {
            let ciphertext = self
                .coprocessor_api
                .fetch_ciphertext(*handle)
                .await
                .map_err(ProcessingError::Recoverable)?;
            ciphertexts.push(ciphertext);
        }
        Ok(ciphertexts)
    }

    fn validate_key_and_epoch(
        &self,
        ciphertexts: &[crate::core::event_processor::coprocessor_api::CiphertextMaterial],
    ) -> Result<(U256, U256), ProcessingError> {
        let Some(first) = ciphertexts.first() else {
            return Err(ProcessingError::Recoverable(anyhow::anyhow!(
                "No ciphertexts fetched"
            )));
        };

        let key_id = first.key_id;
        let epoch_id = first.epoch_id;

        for ct in ciphertexts.iter().skip(1) {
            if ct.key_id != key_id {
                return Err(ProcessingError::Recoverable(anyhow::anyhow!(
                    "Mismatched key IDs across ciphertexts"
                )));
            }
            if ct.epoch_id != epoch_id {
                return Err(ProcessingError::Recoverable(anyhow::anyhow!(
                    "Mismatched epoch IDs across ciphertexts"
                )));
            }
        }

        Ok((key_id, epoch_id))
    }

    async fn record_rejection(
        &self,
        request_id: U256,
        is_user: bool,
        reason: &str,
        code: &str,
    ) {
        let query = if is_user {
            sqlx::query(
                "UPDATE user_decryption_requests SET rejection_reason = $1, rejection_code = $2 WHERE decryption_id = $3",
            )
        } else {
            sqlx::query(
                "UPDATE public_decryption_requests SET rejection_reason = $1, rejection_code = $2 WHERE decryption_id = $3",
            )
        };

        if let Err(err) = query
            .bind(reason)
            .bind(code)
            .bind(request_id.as_le_slice())
            .execute(&self.db_pool)
            .await
        {
            warn!("Failed to store rejection metadata: {err}");
        }
    }

    async fn store_epoch_id(&self, request_id: U256, is_user: bool, epoch_id: U256) {
        let query = if is_user {
            sqlx::query(
                "UPDATE user_decryption_requests SET epoch_id = $1 WHERE decryption_id = $2",
            )
        } else {
            sqlx::query(
                "UPDATE public_decryption_requests SET epoch_id = $1 WHERE decryption_id = $2",
            )
        };

        if let Err(err) = query
            .bind(epoch_id.as_le_slice())
            .bind(request_id.as_le_slice())
            .execute(&self.db_pool)
            .await
        {
            warn!("Failed to store epoch_id: {err}");
        }
    }
}

fn build_typed_ciphertexts(
    ciphertexts: Vec<crate::core::event_processor::coprocessor_api::CiphertextMaterial>,
) -> Result<Vec<TypedCiphertext>, ProcessingError> {
    ciphertexts
        .into_iter()
        .map(|ct| {
            let fhe_type = extract_fhe_type_from_handle(ct.handle.as_slice())?;
            Ok(TypedCiphertext {
                ciphertext: ct.sns_ciphertext,
                fhe_type: fhe_type as i32,
                external_handle: ct.handle.as_slice().to_vec(),
                ciphertext_format: CiphertextFormat::BigExpanded.into(),
            })
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()
        .map_err(ProcessingError::Recoverable)
}

fn u256_to_u64(value: U256) -> Result<u64, ProcessingError> {
    let bytes = value.to_be_bytes::<32>();
    if bytes[..24].iter().any(|b| *b != 0) {
        return Err(ProcessingError::Irrecoverable(anyhow::anyhow!(
            "Value does not fit in u64: {value}"
        )));
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[24..]);
    Ok(u64::from_be_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::{mock::Asserter, ProviderBuilder};
    use std::collections::HashMap;
    use alloy::sol_types::Eip712Domain;
    use alloy::transports::http::reqwest::Client;
    use fhevm_gateway_bindings::gateway_config::GatewayConfig;

    #[tokio::test]
    async fn check_decryption_not_already_done_v2() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());

        let config = Config::default();
        let acl_checker =
            AclChecker::with_host_chains(HashMap::new(), HashMap::new(), mock_provider.clone());
        let domain = Eip712Domain::new(
            Some(config.input_verification_contract.domain_name.clone().into()),
            Some(config.input_verification_contract.domain_version.clone().into()),
            Some(U256::from(config.chain_id)),
            Some(config.input_verification_contract.address),
            None,
        );
        let gateway_config =
            GatewayConfig::new(config.gateway_config_contract.address, mock_provider.clone());
        let coprocessor_api = CoprocessorApi::new(gateway_config, Client::new(), domain);
        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://postgres:postgres@localhost/kms-connector")
            .unwrap();
        let decryption_processor =
            DecryptionProcessor::new(&config, acl_checker, coprocessor_api, db_pool);

        // V2: Should always succeed as we don't check on-chain state
        decryption_processor
            .check_decryption_not_already_done(U256::ZERO)
            .await
            .unwrap();
    }

}
