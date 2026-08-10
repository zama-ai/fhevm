use crate::core::{config::Config, event_processor::ProcessingError};
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
pub struct KMSGenerationProcessor {
    /// The EIP712 domain of the `KMSGeneration` contract.
    domain: Eip712DomainMsg,
}

impl KMSGenerationProcessor {
    pub fn new(config: &Config) -> Self {
        let domain = Eip712DomainMsg {
            name: config.kms_generation_contract.domain_name.clone(),
            version: config.kms_generation_contract.domain_version.clone(),
            chain_id: U256::from(config.ethereum_chain_id).to_be_bytes_vec(),
            verifying_contract: config.kms_generation_contract.address.to_string(),
            salt: None,
        };

        Self { domain }
    }

    pub async fn prepare_prep_keygen_request(
        &self,
        prep_keygen_request: &PrepKeygenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data = parse_extra_data(&prep_keygen_request.extraData)
            .map_err(ProcessingError::Irrecoverable)?;

        Ok(KmsGrpcRequest::PrepKeygen(KeyGenPreprocRequest {
            request_id: Some(u256_to_request_id(prep_keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: prep_keygen_request.paramsType as i32,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: prep_keygen_request.extraData.to_vec(),
            keyset_config: Some(keyset_config(prep_keygen_request.existingKeyId)),
        }))
    }

    pub async fn prepare_keygen_request(
        &self,
        keygen_request: &KeygenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&keygen_request.extraData).map_err(ProcessingError::Irrecoverable)?;

        let existing_key_id = keygen_request.existingKeyId;

        Ok(KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id: Some(u256_to_request_id(keygen_request.keyId)),
            preproc_id: Some(u256_to_request_id(keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: None,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: keygen_request.extraData.to_vec(),
            keyset_config: Some(keyset_config(existing_key_id)),
            keyset_added_info: keyset_added_info(existing_key_id),
        }))
    }

    pub async fn prepare_crsgen_request(
        &self,
        crsgen_request: &CrsgenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&crsgen_request.extraData).map_err(ProcessingError::Irrecoverable)?;

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

fn keyset_config(existing_key_id: U256) -> KeySetConfig {
    if existing_key_id.is_zero() {
        COMPRESSED_XOF_KEY_SET_CONFIG
    } else {
        COMPRESSED_MIGRATION_KEY_SET_CONFIG
    }
}

fn keyset_added_info(existing_key_id: U256) -> Option<KeySetAddedInfo> {
    // Fresh keygen has no source key. Migration reads the existing shares and preserves their tag.
    (!existing_key_id.is_zero()).then(|| KeySetAddedInfo {
        from_keyset_id_decompression_only: None,
        to_keyset_id_decompression_only: None,
        existing_keyset_id: Some(u256_to_request_id(existing_key_id)),
        use_existing_key_tag: true,
        copy_compressed_key_to_original: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::fresh(U256::ZERO)]
    #[case::migration(U256::from(42))]
    #[tokio::test]
    async fn prepares_keygen_request(#[case] existing_key_id: U256) {
        let processor = KMSGenerationProcessor::new(&Config::default());
        let prep_keygen_id = U256::from(7);
        let key_id = U256::from(8);
        let is_migration = !existing_key_id.is_zero();
        let expected_config = if is_migration {
            COMPRESSED_MIGRATION_KEY_SET_CONFIG
        } else {
            COMPRESSED_XOF_KEY_SET_CONFIG
        };
        let expected_added_info = is_migration.then(|| KeySetAddedInfo {
            from_keyset_id_decompression_only: None,
            to_keyset_id_decompression_only: None,
            existing_keyset_id: Some(u256_to_request_id(existing_key_id)),
            use_existing_key_tag: true,
            copy_compressed_key_to_original: true,
        });

        let KmsGrpcRequest::PrepKeygen(prep_request) = processor
            .prepare_prep_keygen_request(&PrepKeygenRequest {
                prepKeygenId: prep_keygen_id,
                paramsType: 0,
                existingKeyId: existing_key_id,
                extraData: Default::default(),
            })
            .await
            .unwrap()
        else {
            panic!("expected preprocessing request");
        };
        assert_eq!(prep_request.keyset_config, Some(expected_config));

        let KmsGrpcRequest::Keygen(keygen_request) = processor
            .prepare_keygen_request(&KeygenRequest {
                prepKeygenId: prep_keygen_id,
                keyId: key_id,
                existingKeyId: existing_key_id,
                extraData: Default::default(),
            })
            .await
            .unwrap()
        else {
            panic!("expected key generation request");
        };
        assert_eq!(keygen_request.request_id, Some(u256_to_request_id(key_id)));
        assert_eq!(
            keygen_request.preproc_id,
            Some(u256_to_request_id(prep_keygen_id))
        );
        assert_eq!(keygen_request.keyset_config, Some(expected_config));
        assert_eq!(keygen_request.keyset_added_info, expected_added_info);
    }
}
