use crate::core::{
    config::Config,
    event_processor::{ContextManager, ProcessingError},
};
use alloy::primitives::U256;
use connector_utils::types::{KmsGrpcRequest, extra_data::parse_extra_data, u256_to_request_id};
use fhevm_host_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeygenRequest, MigrationKeygenRequest, PrepKeygenRequest,
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
        if let Some(context_id) = parsed_extra_data.context_id {
            self.context_manager.validate_context(context_id).await?;
        }
        // TODO: validation of epoch_id during RFC-005 implementation

        Ok(KmsGrpcRequest::PrepKeygen(KeyGenPreprocRequest {
            request_id: Some(u256_to_request_id(prep_keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: prep_keygen_request.paramsType as i32,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: prep_keygen_request.extraData.to_vec(),
            // Used to generate other types of key, but not planned to be supported by the Gateway
            keyset_config: Some(UNCOMPRESSED_KEY_SET_CONFIG),
        }))
    }

    pub async fn prepare_keygen_request(
        &self,
        keygen_request: &KeygenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&keygen_request.extraData).map_err(ProcessingError::Irrecoverable)?;
        if let Some(context_id) = parsed_extra_data.context_id {
            self.context_manager.validate_context(context_id).await?;
        }
        // TODO: validation of epoch_id during RFC-005 implementation

        Ok(KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id: Some(u256_to_request_id(keygen_request.keyId)),
            preproc_id: Some(u256_to_request_id(keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: None,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: keygen_request.extraData.to_vec(),
            keyset_config: Some(UNCOMPRESSED_KEY_SET_CONFIG),
            keyset_added_info: None,
        }))
    }

    /// RFC-029: a migration keygen drives a keygen-from-existing-shares — re-derive the existing
    /// key's public keyset in compressed form and (optionally) copy it onto the original key id. The
    /// migration parameters come straight off the typed `MigrationKeygenRequest` event, so a
    /// migration can NEVER silently run as an ordinary `GenerateAll` keygen. Its extra_data is the
    /// standard v2 context+epoch, signed by the KMS exactly like a normal keygen.
    pub async fn prepare_migration_keygen_request(
        &self,
        request: &MigrationKeygenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&request.extraData).map_err(ProcessingError::Irrecoverable)?;
        if let Some(context_id) = parsed_extra_data.context_id {
            self.context_manager.validate_context(context_id).await?;
        }
        // TODO: validation of epoch_id during RFC-005 implementation

        Ok(KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id: Some(u256_to_request_id(request.keyId)),
            preproc_id: Some(u256_to_request_id(request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: None,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: request.extraData.to_vec(),
            keyset_config: Some(MIGRATION_KEY_SET_CONFIG),
            keyset_added_info: Some(KeySetAddedInfo {
                existing_keyset_id: Some(u256_to_request_id(request.existingKeyId)),
                copy_compressed_key_to_original: request.copyToOriginal,
                ..Default::default()
            }),
        }))
    }

    pub async fn prepare_crsgen_request(
        &self,
        crsgen_request: &CrsgenRequest,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(&crsgen_request.extraData).map_err(ProcessingError::Irrecoverable)?;
        if let Some(context_id) = parsed_extra_data.context_id {
            self.context_manager.validate_context(context_id).await?;
        }
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
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
        }))
    }
}

const UNCOMPRESSED_KEY_SET_CONFIG: KeySetConfig = KeySetConfig {
    keyset_type: KeySetType::Standard as i32,
    standard_keyset_config: Some(StandardKeySetConfig {
        compute_key_type: ComputeKeyType::Cpu as i32,
        secret_key_config: KeyGenSecretKeyConfig::GenerateAll as i32,
        compressed_key_config: CompressedKeyConfig::CompressedNone as i32,
    }),
};

/// RFC-029 keygen-from-existing: re-derive the existing key's public material in compressed form.
const MIGRATION_KEY_SET_CONFIG: KeySetConfig = KeySetConfig {
    keyset_type: KeySetType::Standard as i32,
    standard_keyset_config: Some(StandardKeySetConfig {
        compute_key_type: ComputeKeyType::Cpu as i32,
        secret_key_config: KeyGenSecretKeyConfig::UseExisting as i32,
        compressed_key_config: CompressedKeyConfig::CompressedAll as i32,
    }),
};

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;
    use connector_utils::tests::rand::rand_u256;

    struct MockContextManager;

    impl ContextManager for MockContextManager {
        async fn validate_context(&self, _context_id: U256) -> Result<(), ProcessingError> {
            Ok(())
        }
    }

    fn test_processor() -> KMSGenerationProcessor<MockContextManager> {
        KMSGenerationProcessor::new(&Config::default(), MockContextManager)
    }

    fn into_keygen(req: KmsGrpcRequest) -> KeyGenRequest {
        match req {
            KmsGrpcRequest::Keygen(k) => k,
            other => panic!("expected a Keygen gRPC request, got {other:?}"),
        }
    }

    /// A migration keygen MUST map to keygen-from-existing (UseExisting + copy-to-original), never to
    /// the GenerateAll path a normal keygen uses — this is the core "no silent degrade" invariant.
    #[tokio::test]
    async fn migration_keygen_maps_to_use_existing_not_generate_all() {
        let existing_key_id = rand_u256();
        let request = MigrationKeygenRequest {
            prepKeygenId: rand_u256(),
            keyId: rand_u256(),
            existingKeyId: existing_key_id,
            copyToOriginal: true,
            extraData: Bytes::new(),
        };

        let keygen = into_keygen(
            test_processor()
                .prepare_migration_keygen_request(&request)
                .await
                .unwrap(),
        );

        let standard = keygen
            .keyset_config
            .and_then(|c| c.standard_keyset_config)
            .expect("migration keygen must carry a standard keyset config");
        assert_eq!(
            standard.secret_key_config,
            KeyGenSecretKeyConfig::UseExisting as i32,
            "migration keygen must re-use existing shares, never GenerateAll"
        );
        assert_eq!(
            standard.compressed_key_config,
            CompressedKeyConfig::CompressedAll as i32
        );
        let added = keygen
            .keyset_added_info
            .expect("migration keygen must carry keyset_added_info");
        assert!(added.copy_compressed_key_to_original);
        assert_eq!(
            added.existing_keyset_id,
            Some(u256_to_request_id(existing_key_id))
        );
    }

    /// A normal keygen MUST stay GenerateAll with no keyset_added_info — the migration branch must not
    /// leak into the ordinary path.
    #[tokio::test]
    async fn normal_keygen_stays_generate_all() {
        let request = KeygenRequest {
            prepKeygenId: rand_u256(),
            keyId: rand_u256(),
            extraData: Bytes::new(),
        };

        let keygen = into_keygen(
            test_processor()
                .prepare_keygen_request(&request)
                .await
                .unwrap(),
        );

        let standard = keygen
            .keyset_config
            .and_then(|c| c.standard_keyset_config)
            .expect("keygen must carry a standard keyset config");
        assert_eq!(
            standard.secret_key_config,
            KeyGenSecretKeyConfig::GenerateAll as i32
        );
        assert!(keygen.keyset_added_info.is_none());
    }
}
