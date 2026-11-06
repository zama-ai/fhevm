use crate::core::{config::Config, event_processor::ProcessingError};
use alloy::{
    eips::BlockId,
    primitives::{FixedBytes, Keccak256, U256, keccak256},
    providers::Provider,
    rpc::types::Filter,
    sol_types::{SolEvent, SolValue},
};
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest, db::KeyType, extra_data::extra_data_v2_payload, u256_to_request_id,
};
use fhevm_host_bindings::{
    kms_generation::{
        IKMSGeneration::KeyInfo,
        KMSGeneration::{self, KMSGenerationInstance},
    },
    protocol_config::{
        IKMSGeneration::KeyDigest,
        IProtocolConfig::{EpochCrsResult, EpochKeyResult},
        ProtocolConfig::{
            KmsContextDestroyed, KmsEpochDestroyed, KmsNode, NewKmsContext, NewKmsEpoch,
            ProtocolConfigInstance,
        },
    },
};
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{
    CrsInfo, DestroyMpcContextRequest, DestroyMpcEpochRequest, Eip712DomainMsg, FheParameter,
    KeyDigest as GrpcKeyDigest, KeyInfo as GrpcKeyInfo, MpcContext, MpcNode, NewMpcContextRequest,
    NewMpcEpochRequest, PcrValues, PreviousEpochInfo, SchemeDigest, SigningSchemeType,
};

/// The signature schemes requested from the KMS Core.
///
/// Currently hardcoded to Ecdsa256k1 as it is the only scheme used for Ethereum.
const SIGNING_SCHEMES: [i32; 1] = [SigningSchemeType::Ecdsa256k1 as i32];

/// Builder for the KMS Core gRPC requests triggered by `ProtocolConfig` events.
#[derive(Clone)]
pub struct ProtocolConfigProcessor<P: Provider> {
    /// EIP-712 domain of the `ProtocolConfig` contract.
    domain: Eip712DomainMsg,

    /// The `ProtocolConfig` contract instance, used to fetch the context event anchors.
    ///
    /// See [RFC-005](https://github.com/zama-ai/tech-spec/blob/main/rfcs/005-key-resharing.md#cross-context-party-communication)
    /// for the anchor definition.
    protocol_config_contract: ProtocolConfigInstance<P>,

    /// The `KMSGeneration` contract instance, used to fetch the previous epoch material.
    kms_generation_contract: KMSGenerationInstance<P>,
}

impl<P: Provider + Clone> ProtocolConfigProcessor<P> {
    pub fn new(config: &Config, ethereum_provider: P) -> Self {
        let domain = Eip712DomainMsg {
            name: config.protocol_config_contract.domain_name.clone(),
            version: config.protocol_config_contract.domain_version.clone(),
            chain_id: U256::from(config.ethereum_chain_id).to_be_bytes_vec(),
            verifying_contract: config.protocol_config_contract.address.to_string(),
            salt: None,
        };
        let protocol_config_contract = ProtocolConfigInstance::new(
            config.protocol_config_contract.address,
            ethereum_provider.clone(),
        );
        let kms_generation_contract =
            KMSGeneration::new(config.kms_generation_contract.address, ethereum_provider);
        Self {
            domain,
            protocol_config_contract,
            kms_generation_contract,
        }
    }
}

impl<P: Provider> ProtocolConfigProcessor<P> {
    pub async fn prepare_new_kms_context_request(
        &self,
        event: &NewKmsContext,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let new = self.build_new_mpc_context_request(event)?;
        let previous_context_event = self
            .fetch_previous_context_creation_event(event)
            .await
            .map_err(ProcessingError::transient)?;
        let old = self.build_new_mpc_context_request(&previous_context_event)?;
        Ok(KmsGrpcRequest::NewMpcContext {
            new: Box::new(new),
            old: Box::new(old),
        })
    }

    pub fn prepare_destroy_kms_context_request(
        &self,
        event: &KmsContextDestroyed,
    ) -> KmsGrpcRequest {
        KmsGrpcRequest::DestroyMpcContext(DestroyMpcContextRequest {
            context_id: Some(u256_to_request_id(event.kmsContextId)),
            domain: Some(self.domain.clone()),
            extra_data: vec![],
            signing_schemes: SIGNING_SCHEMES.to_vec(),
        })
    }

    pub fn prepare_destroy_kms_epoch_request(&self, event: &KmsEpochDestroyed) -> KmsGrpcRequest {
        KmsGrpcRequest::DestroyMpcEpoch(DestroyMpcEpochRequest {
            epoch_id: Some(u256_to_request_id(event.epochId)),
            domain: Some(self.domain.clone()),
            extra_data: vec![],
            signing_schemes: SIGNING_SCHEMES.to_vec(),
        })
    }

    /// Builds the `NewMpcContext` gRPC request of a `NewKmsContext` on-chain event.
    fn build_new_mpc_context_request(
        &self,
        event: &NewKmsContext,
    ) -> Result<NewMpcContextRequest, ProcessingError> {
        Ok(NewMpcContextRequest {
            new_context: Some(build_mpc_context_from_event(event)?),
            previous_context_id: Some(u256_to_request_id(event.previousContextId)),
            node_config_hash: compute_node_config_hash(event).to_vec(),
            domain: Some(self.domain.clone()),
            extra_data: vec![],
            signing_schemes: SIGNING_SCHEMES.to_vec(),
        })
    }

    pub async fn prepare_new_kms_epoch_request(
        &self,
        event: &NewKmsEpoch,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let previous_epoch = self
            .fetch_epoch_material(event.materialBlockNumber.saturating_to())
            .await?
            .previous_epoch_info(event)?;

        Ok(KmsGrpcRequest::NewMpcEpoch(NewMpcEpochRequest {
            context_id: Some(u256_to_request_id(event.kmsContextId)),
            epoch_id: Some(u256_to_request_id(event.epochId)),
            previous_epoch: Some(previous_epoch),
            domain: Some(self.domain.clone()),
            extra_data: vec![],
            signing_schemes: SIGNING_SCHEMES.to_vec(),
            material_extra_data: extra_data_v2_payload(event.kmsContextId, event.epochId),
        }))
    }

    /// Returns the ABI-encoded `EpochKeyResult[]` and `EpochCrsResult[]` arrays of the
    /// `confirmEpochActivation` response tied to the `NewKmsEpoch` event.
    pub async fn fetch_epoch_activation_material(
        &self,
        event: &NewKmsEpoch,
    ) -> Result<(Vec<u8>, Vec<u8>), ProcessingError> {
        let material = self
            .fetch_epoch_material(event.materialBlockNumber.saturating_to())
            .await?;
        Ok(material.abi_encode())
    }

    /// Fetch the previous context creation event for the given `NewKmsContext` event.
    ///
    /// Uses the `previousContextId` field to fetch the anchor via `getKmsContextAnchor`, then
    /// uses the block number to filter for the event.
    async fn fetch_previous_context_creation_event(
        &self,
        new_context_event: &NewKmsContext,
    ) -> anyhow::Result<NewKmsContext> {
        let previous_context_anchor = self
            .protocol_config_contract
            .getKmsContextAnchor(new_context_event.previousContextId)
            .block(BlockId::finalized())
            .call()
            .await?;
        let previous_context_block: u64 =
            previous_context_anchor.emissionBlockNumber.saturating_to();

        let filter = Filter::new()
            .address(*self.protocol_config_contract.address())
            .event_signature(NewKmsContext::SIGNATURE_HASH)
            .from_block(previous_context_block)
            .to_block(previous_context_block);
        let anchor_logs = self
            .protocol_config_contract
            .provider()
            .get_logs(&filter)
            .await?;

        match anchor_logs.as_slice() {
            [] => Err(anyhow!(
                "No event found at anchor {} for previous context {:#066x}",
                previous_context_anchor.emissionBlockNumber,
                new_context_event.previousContextId
            )),
            [anchor_log] => {
                let previous_context_event = NewKmsContext::decode_log(&anchor_log.inner)?.data;
                let event_hash = compute_anchor_event_hash(&previous_context_event);
                if event_hash != previous_context_anchor.contextInfoHash {
                    return Err(anyhow!(
                        "Previous context hash verification failed: computed={:?}, on-chain={:?}",
                        event_hash,
                        previous_context_anchor.contextInfoHash,
                    ));
                }
                Ok(previous_context_event)
            }
            logs => Err(anyhow!(
                "Too many events found at anchor {previous_context_anchor:?}: {logs:?}"
            )),
        }
    }

    /// Fetch the material of the previous epoch from `KMSGeneration` contract.
    async fn fetch_epoch_material(
        &self,
        material_block_number: u64,
    ) -> Result<EpochMaterial, ProcessingError> {
        let key_ids = self
            .kms_generation_contract
            .getCompletedKeyIds()
            .block(material_block_number.into())
            .call()
            .await
            .map_err(|e| {
                ProcessingError::transient(anyhow!("getCompletedKeyIds call failed: {e}"))
            })?;
        let crs_ids = self
            .kms_generation_contract
            .getCompletedCrsIds()
            .block(material_block_number.into())
            .call()
            .await
            .map_err(|e| {
                ProcessingError::transient(anyhow!("getCompletedCrsIds call failed: {e}"))
            })?;

        let mut keys = Vec::with_capacity(key_ids.len());
        for key_id in key_ids {
            let key_info = self
                .kms_generation_contract
                .getKeyInfo(key_id)
                .block(material_block_number.into())
                .call()
                .await
                .map_err(|e| ProcessingError::transient(anyhow!("getKeyInfo call failed: {e}")))?;
            keys.push(key_info);
        }

        let mut crs_list = Vec::with_capacity(crs_ids.len());
        for crs_id in crs_ids {
            let crs_material = self
                .kms_generation_contract
                .getCrsMaterials(crs_id)
                .block(material_block_number.into())
                .call()
                .await
                .map_err(|e| {
                    ProcessingError::transient(anyhow!("getCrsMaterials call failed: {e}"))
                })?;
            let max_bit_length = self
                .kms_generation_contract
                .getCrsMaxBitLength(crs_id)
                .block(material_block_number.into())
                .call()
                .await
                .map_err(|e| {
                    ProcessingError::transient(anyhow!("getCrsMaxBitLength call failed: {e}"))
                })?;
            crs_list.push(EpochCrsResult {
                crsId: crs_id,
                maxBitLength: max_bit_length,
                crsDigest: crs_material._1,
            });
        }

        Ok(EpochMaterial { keys, crs_list })
    }
}

/// The key and CRS material of an epoch, as stored in `KMSGeneration`.
struct EpochMaterial {
    keys: Vec<KeyInfo>,
    crs_list: Vec<EpochCrsResult>,
}

impl EpochMaterial {
    /// Builds the `PreviousEpochInfo` of the `NewMpcEpoch` request.
    fn previous_epoch_info(
        &self,
        event: &NewKmsEpoch,
    ) -> Result<PreviousEpochInfo, ProcessingError> {
        let mut keys_info = Vec::with_capacity(self.keys.len());
        for key_info in &self.keys {
            let mut key_digests = Vec::with_capacity(key_info.keyDigests.len());
            for d in key_info.keyDigests.iter() {
                key_digests.push(GrpcKeyDigest {
                    key_type: key_type_to_string(d.keyType)
                        .map_err(|e| ProcessingError::irrecoverable(ErrorCode::Unprocessable, e))?,
                    digest: d.digest.to_vec(),
                });
            }
            keys_info.push(GrpcKeyInfo {
                key_id: Some(u256_to_request_id(key_info.keyId)),
                preproc_id: Some(u256_to_request_id(key_info.prepKeygenId)),
                key_parameters: params_type_to_fhe_parameter(key_info.paramsType)
                    .map_err(|e| ProcessingError::irrecoverable(ErrorCode::Unprocessable, e))?,
                key_digests,
            });
        }

        let crs_info = self
            .crs_list
            .iter()
            .map(|crs| CrsInfo {
                crs_id: Some(u256_to_request_id(crs.crsId)),
                crs_digest: crs.crsDigest.to_vec(),
            })
            .collect();

        Ok(PreviousEpochInfo {
            // We use the `previousContextId` field of the event because:
            // * KMS Context switch: the previous epoch belongs to the previous context
            // * Same set resharing: the previous epoch belongs to the current context, but in such case,
            //   `previousContextId` == `kmsContextId`
            context_id: Some(u256_to_request_id(event.previousContextId)),
            epoch_id: Some(u256_to_request_id(event.previousEpochId)),
            keys_info,
            crs_info,
        })
    }

    /// ABI-encodes the `EpochKeyResult[]` and `EpochCrsResult[]` arrays, sorted by id.
    fn abi_encode(self) -> (Vec<u8>, Vec<u8>) {
        let mut keys: Vec<EpochKeyResult> = self
            .keys
            .into_iter()
            .map(|key_info| EpochKeyResult {
                prepKeygenId: key_info.prepKeygenId,
                keyId: key_info.keyId,
                keyDigests: key_info
                    .keyDigests
                    .into_iter()
                    .map(|d| KeyDigest {
                        keyType: d.keyType,
                        digest: d.digest,
                    })
                    .collect(),
            })
            .collect();
        keys.sort_by_key(|k| k.keyId);
        let mut crs_list = self.crs_list;
        crs_list.sort_by_key(|c| c.crsId);
        (keys.abi_encode(), crs_list.abi_encode())
    }
}

fn build_mpc_context_from_event(event: &NewKmsContext) -> Result<MpcContext, ProcessingError> {
    let mpc_nodes = event
        .kmsNodeParams
        .iter()
        .map(|n| MpcNode {
            mpc_identity: n.mpcIdentity.clone(),
            party_id: n.partyId,
            external_url: n.ipAddress.clone(),
            ca_cert: Some(n.caCert.to_vec()),
            public_storage_url: n.storageUrl.clone(),
            // Public key used to verify the signature of this KMS node
            signer_address: Some(n.signerAddress.to_vec()),
            public_storage_prefix: Some(n.storagePrefix.clone()),
            // Public keys allowed to sign transactions on behalf of this KMS node, i.e. the
            // connector transaction sender's address of this node
            extra_signer_addresses: vec![n.txSenderAddress.to_vec()],
            // Scheme-specific digests of the KMS node's signing keys. The Gateway only exposes
            // an ECDSA signer address for now, so this is the only entry.
            scheme_digests: vec![SchemeDigest {
                scheme: SigningSchemeType::Ecdsa256k1 as i32,
                digest: n.signerAddress.to_vec(),
            }],
        })
        .collect();

    let pcr_values = event
        .pcrValues
        .iter()
        .map(|p| PcrValues {
            pcr0: p.pcr0.to_vec(),
            pcr1: p.pcr1.to_vec(),
            pcr2: p.pcr2.to_vec(),
        })
        .collect();

    // `KmsThresholds` carries four thresholds on chain (publicDecryption, userDecryption,
    // kmsGen, mpc). The Core's `MpcContext` only exposes a single `threshold` field — the
    // MPC corruption threshold — so we forward `mpc` and rely on the contract to enforce
    // the others.
    let threshold = i32::try_from(event.thresholds.mpc).map_err(|e| {
        ProcessingError::irrecoverable(
            ErrorCode::Unprocessable,
            anyhow!("Invalid threshold value: {e}"),
        )
    })?;

    Ok(MpcContext {
        mpc_nodes,
        context_id: Some(u256_to_request_id(event.contextId)),
        software_version: event.softwareVersion.clone(),
        threshold,
        pcr_values,
    })
}

fn params_type_to_fhe_parameter(params: u8) -> anyhow::Result<i32> {
    if params == FheParameter::Test as u8 {
        Ok(FheParameter::Test as i32)
    } else if params == FheParameter::Default as u8 {
        Ok(FheParameter::Default as i32)
    } else {
        Err(anyhow::anyhow!("Unknown FheParameter variant: {}", params))
    }
}

fn key_type_to_string(key_type: u8) -> anyhow::Result<String> {
    match KeyType::try_from(key_type)? {
        KeyType::Server => Ok("ServerKey".to_string()),
        KeyType::Public => Ok("PublicKey".to_string()),
        KeyType::CompressedKeySet => Ok("CompressedXofKeySet".to_string()),
    }
}

/// Computes the `nodeConfigHash` signed in the `ContextCreationConfirmation` of a new context.
///
/// Mirrors `ProtocolConfig`: `keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds))`,
/// where each `KmsNode` is the projection of a `KmsNodeParams` stored by the contract.
pub fn compute_node_config_hash(event: &NewKmsContext) -> FixedBytes<32> {
    let nodes: Vec<KmsNode> = event
        .kmsNodeParams
        .iter()
        .map(|n| KmsNode {
            txSenderAddress: n.txSenderAddress,
            signerAddress: n.signerAddress,
            ipAddress: n.ipAddress.clone(),
            storageUrl: n.storageUrl.clone(),
        })
        .collect();
    keccak256((nodes, event.thresholds.clone()).abi_encode_sequence())
}

/// Computes the hash of a `NewKmsContext` event.
///
/// Used to be compared with the `contextInfoHash` field in the acnhor.
pub fn compute_anchor_event_hash(event: &NewKmsContext) -> FixedBytes<32> {
    // keccak256(abi.encode(initialKmsNodeParams, initialThresholds, softwareVersion, pcrValues))
    let encoded_data = (
        event.kmsNodeParams.as_slice(),
        event.thresholds.clone(),
        &event.softwareVersion,
        event.pcrValues.as_slice(),
    )
        .abi_encode_sequence();

    let mut hasher = Keccak256::new();
    hasher.update(encoded_data);
    hasher.finalize()
}
