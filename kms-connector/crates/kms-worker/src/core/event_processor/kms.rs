use crate::core::{
    config::Config,
    event_processor::{ContextManager, ProcessingError, RequestCheckError},
};
use alloy::primitives::U256;
use connector_utils::types::{KmsGrpcRequest, extra_data::parse_extra_data, u256_to_request_id};
use fhevm_host_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeygenRequest, PrepKeygenRequest,
};
use kms_grpc::kms::v1::{
    CompressedKeyConfig, ComputeKeyType, CrsGenRequest, Eip712DomainMsg, KeyGenPreprocRequest,
    KeyGenRequest, KeyGenSecretKeyConfig, KeySetAddedInfo, KeySetConfig, KeySetType,
    StandardKeySetConfig,
};
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
            chain_id: U256::from(config.ethereum_chain_id).to_be_bytes_vec(),
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
            .validate_context(&parsed_extra_data)
            .await
            .map_err(RequestCheckError::record)?;

        Ok(KmsGrpcRequest::PrepKeygen(KeyGenPreprocRequest {
            request_id: Some(u256_to_request_id(prep_keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: prep_keygen_request.paramsType as i32,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: prep_keygen_request.extraData.to_vec(),
            // Explicitly request the compressed XOF keyset layout expected by GPU workers.
            keyset_config: Some(COMPRESSED_XOF_KEY_SET_CONFIG),
        }))
    }

    pub async fn prepare_keygen_request(
        &self,
        keygen_request: &KeygenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&keygen_request.extraData).map_err(ProcessingError::Irrecoverable)?;
        self.context_manager
            .validate_context(&parsed_extra_data)
            .await
            .map_err(RequestCheckError::record)?;

        let existing_key_id = (!keygen_request.existingKeyId.is_zero())
            .then(|| u256_to_request_id(keygen_request.existingKeyId));
        let is_migration = existing_key_id.is_some();

        Ok(KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id: Some(u256_to_request_id(keygen_request.requestId)),
            preproc_id: Some(u256_to_request_id(keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: None,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: keygen_request.extraData.to_vec(),
            keyset_config: Some(if is_migration {
                COMPRESSED_MIGRATION_KEY_SET_CONFIG
            } else {
                COMPRESSED_XOF_KEY_SET_CONFIG
            }),
            keyset_added_info: existing_key_id.map(|existing_keyset_id| KeySetAddedInfo {
                from_keyset_id_decompression_only: None,
                to_keyset_id_decompression_only: None,
                existing_keyset_id: Some(existing_keyset_id),
                use_existing_key_tag: true,
                copy_compressed_key_to_original: false,
            }),
        }))
    }

    pub async fn prepare_crsgen_request(
        &self,
        crsgen_request: &CrsgenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&crsgen_request.extraData).map_err(ProcessingError::Irrecoverable)?;
        self.context_manager
            .validate_context(&parsed_extra_data)
            .await
            .map_err(RequestCheckError::record)?;

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
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
        }))
    }
}

const COMPRESSED_MIGRATION_KEY_SET_CONFIG: KeySetConfig = KeySetConfig {
    keyset_type: KeySetType::Standard as i32,
    standard_keyset_config: Some(StandardKeySetConfig {
        compute_key_type: ComputeKeyType::Cpu as i32,
        secret_key_config: KeyGenSecretKeyConfig::UseExisting as i32,
        compressed_key_config: CompressedKeyConfig::CompressedAll as i32,
    }),
};

const COMPRESSED_XOF_KEY_SET_CONFIG: KeySetConfig = KeySetConfig {
    keyset_type: KeySetType::Standard as i32,
    standard_keyset_config: Some(StandardKeySetConfig {
        compute_key_type: ComputeKeyType::Cpu as i32,
        secret_key_config: KeyGenSecretKeyConfig::GenerateAll as i32,
        compressed_key_config: CompressedKeyConfig::CompressedAll as i32,
    }),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_reuses_the_existing_secret_key() {
        let config = COMPRESSED_MIGRATION_KEY_SET_CONFIG
            .standard_keyset_config
            .expect("standard keyset config must be present");

        assert_eq!(
            config.secret_key_config,
            KeyGenSecretKeyConfig::UseExisting as i32
        );
        assert_eq!(
            config.compressed_key_config,
            CompressedKeyConfig::CompressedAll as i32
        );
    }
}
