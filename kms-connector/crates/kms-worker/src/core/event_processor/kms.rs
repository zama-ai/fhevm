use crate::core::{
    config::Config,
    event_processor::{ContextManager, ProcessingError},
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
use sqlx::{Pool, Postgres, Row};
use tracing::error;

/// RFC-029 migration mapping recovered from the `migration_keygen` table by `key_id`.
struct MigrationKeygen {
    existing_key_id: U256,
    copy_to_original: bool,
}

#[derive(Clone)]
/// The struct responsible of processing incoming key management requests.
pub struct KMSGenerationProcessor<C> {
    /// The EIP712 domain of the `KMSGeneration` contract.
    domain: Eip712DomainMsg,

    /// The entity used to validate KMS context.
    context_manager: C,

    /// The DB pool used to look up RFC-029 migration mappings by `key_id`.
    db_pool: Pool<Postgres>,
}

impl<C> KMSGenerationProcessor<C>
where
    C: ContextManager,
{
    pub fn new(config: &Config, context_manager: C, db_pool: Pool<Postgres>) -> Self {
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
            db_pool,
        }
    }

    /// RFC-029: looks up the migration mapping persisted by the gw-listener for `key_id`. Returns
    /// `None` for an ordinary (non-migration) keygen.
    ///
    /// Runtime `sqlx::query` (not the `query!` macro) is used so no offline `.sqlx` cache is needed.
    async fn fetch_migration_keygen(
        &self,
        key_id: U256,
    ) -> Result<Option<MigrationKeygen>, ProcessingError> {
        let row = sqlx::query(
            "SELECT existing_key_id, copy_to_original FROM migration_keygen WHERE key_id = $1",
        )
        .bind(key_id.as_le_slice())
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| ProcessingError::Recoverable(e.into()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let existing_key_id_bytes: Vec<u8> = row
            .try_get("existing_key_id")
            .map_err(|e| ProcessingError::Irrecoverable(e.into()))?;
        let copy_to_original: bool = row
            .try_get("copy_to_original")
            .map_err(|e| ProcessingError::Irrecoverable(e.into()))?;
        let existing_key_id = U256::try_from_le_slice(&existing_key_id_bytes).ok_or_else(|| {
            ProcessingError::Irrecoverable(anyhow::anyhow!(
                "Invalid existing_key_id length in migration_keygen row: {} bytes",
                existing_key_id_bytes.len()
            ))
        })?;

        Ok(Some(MigrationKeygen {
            existing_key_id,
            copy_to_original,
        }))
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

        // RFC-029: a migration keygen drives a keygen-from-existing-shares — re-derive the public
        // keyset from the existing private shares, compressed, and (optionally) copy the result to
        // the original key id. The mapping is recovered from the `migration_keygen` table (persisted
        // by the gw-listener when it saw `MigrationKeygenRequested`), keyed by this `key_id`. The
        // keygen's own extra_data is ordinary v2 context+epoch.
        let (keyset_config, keyset_added_info) =
            match self.fetch_migration_keygen(keygen_request.keyId).await? {
                Some(migration) => (
                    Some(KeySetConfig {
                        keyset_type: KeySetType::Standard as i32,
                        standard_keyset_config: Some(StandardKeySetConfig {
                            compute_key_type: ComputeKeyType::Cpu as i32,
                            secret_key_config: KeyGenSecretKeyConfig::UseExisting as i32,
                            compressed_key_config: CompressedKeyConfig::CompressedAll as i32,
                        }),
                    }),
                    Some(KeySetAddedInfo {
                        existing_keyset_id: Some(u256_to_request_id(migration.existing_key_id)),
                        copy_compressed_key_to_original: migration.copy_to_original,
                        ..Default::default()
                    }),
                ),
                // Used to generate other types of key, but not planned to be supported by the Gateway
                None => (Some(UNCOMPRESSED_KEY_SET_CONFIG), None),
            };

        Ok(KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id: Some(u256_to_request_id(keygen_request.keyId)),
            preproc_id: Some(u256_to_request_id(keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: None,
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            extra_data: keygen_request.extraData.to_vec(),
            keyset_config,
            keyset_added_info,
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
