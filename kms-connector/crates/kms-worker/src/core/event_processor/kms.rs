use crate::core::{
    config::Config,
    event_processor::{ContextManager, ProcessingError},
};
use alloy::primitives::U256;
use connector_utils::types::{KmsGrpcRequest, extra_data::parse_extra_data, u256_to_request_id};
use fhevm_host_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeygenRequest, PrepKeygenRequest,
};
use kms_grpc::kms::v1::{CrsGenRequest, Eip712DomainMsg, KeyGenPreprocRequest, KeyGenRequest};
use tracing::error;

#[derive(Clone)]
/// The struct responsible of processing incoming key management requests.
pub struct KMSGenerationProcessor<C> {
    /// The EIP712 domain of the `KMSGeneration` contract.
    domain: Eip712DomainMsg,

    /// The entity used to validate KMS context.
    context_manager: C,
}

impl<C> KMSGenerationProcessor<C>
where
    C: ContextManager,
{
    pub fn new(config: &Config, context_manager: C) -> Self {
        let domain = Eip712DomainMsg {
            name: config.kms_generation_contract.domain_name.clone(),
            version: config.kms_generation_contract.domain_version.clone(),
            chain_id: U256::from(config.gateway_chain_id).to_be_bytes_vec(),
            verifying_contract: config.kms_generation_contract.address.to_string(),
            salt: None,
        };

        Self {
            domain,
            context_manager,
        }
    }

    pub async fn prepare_prep_keygen_request(
        &self,
        prep_keygen_request: &PrepKeygenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data = parse_extra_data(&prep_keygen_request.extraData)
            .map_err(ProcessingError::Irrecoverable)?;
        self.context_manager
            .validate_context(parsed_extra_data.context_id)
            .await?;
        // TODO: validation of epoch_id during RFC-005 implementation

        Ok(KmsGrpcRequest::PrepKeygen(KeyGenPreprocRequest {
            request_id: Some(u256_to_request_id(prep_keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: prep_keygen_request.paramsType as i32,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: Some(u256_to_request_id(parsed_extra_data.context_id)),
            // Used to generate other types of key, but not planned to be supported by the Gateway
            keyset_config: None,
        }))
    }

    pub async fn prepare_keygen_request(
        &self,
        keygen_request: &KeygenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&keygen_request.extraData).map_err(ProcessingError::Irrecoverable)?;
        self.context_manager
            .validate_context(parsed_extra_data.context_id)
            .await?;
        // TODO: validation of epoch_id during RFC-005 implementation

        Ok(KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id: Some(u256_to_request_id(keygen_request.keyId)),
            preproc_id: Some(u256_to_request_id(keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: None,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: Some(u256_to_request_id(parsed_extra_data.context_id)),
            extra_data: keygen_request.extraData.to_vec(),
            // Used to generate other types of key, but not planned to be supported by the Gateway
            keyset_config: None,
            keyset_added_info: None,
        }))
    }

    pub async fn prepare_crsgen_request(
        &self,
        crsgen_request: &CrsgenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&crsgen_request.extraData).map_err(ProcessingError::Irrecoverable)?;
        self.context_manager
            .validate_context(parsed_extra_data.context_id)
            .await?;
        // TODO: validation of epoch_id during RFC-005 implementation

        let max_num_bits = crsgen_request
            .maxBitLength
            .as_le_slice()
            .get(0..4) // Get least significant bits
            .and_then(|s| {
                s.try_into()
                    .inspect_err(|e| error!("Failed to parse `max_num_bits`: {e}"))
                    .map(u32::from_le_bytes)
                    .ok()
            });

        Ok(KmsGrpcRequest::Crsgen(CrsGenRequest {
            request_id: Some(u256_to_request_id(crsgen_request.crsId)),
            domain: Some(self.domain.clone()),
            params: crsgen_request.paramsType as i32,
            extra_data: crsgen_request.extraData.to_vec(),
            max_num_bits,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: Some(u256_to_request_id(parsed_extra_data.context_id)),
        }))
    }
}
