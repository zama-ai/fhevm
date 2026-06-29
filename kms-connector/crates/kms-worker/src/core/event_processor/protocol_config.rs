use crate::core::{config::Config, event_processor::ProcessingError};
use alloy::{
    primitives::{FixedBytes, Keccak256, U256},
    providers::Provider,
    rpc::types::Filter,
    sol_types::{SolEvent, SolValue},
};
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest, db::KeyType, extra_data::extra_data_v2_payload, u256_to_request_id,
};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::{self, KMSGenerationInstance},
    protocol_config::ProtocolConfig::{NewKmsContext, NewKmsEpoch, ProtocolConfigInstance},
};
use kms_grpc::kms::v1::{
    CrsInfo, Eip712DomainMsg, FheParameter, KeyDigest, KeyInfo, MpcContext, MpcNode,
    NewMpcContextRequest, NewMpcEpochRequest, PcrValues, PreviousEpochInfo,
};

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
        let new = build_new_kms_context_grpc_from_event(event)?;
        let previous_context_event = self
            .fetch_previous_context_creation_event(event)
            .await
            .map_err(ProcessingError::Recoverable)?;
        let old = build_new_kms_context_grpc_from_event(&previous_context_event)?;
        Ok(KmsGrpcRequest::NewMpcContext { new, old })
    }

    pub async fn prepare_new_kms_epoch_request(
        &self,
        event: &NewKmsEpoch,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let previous_epoch = self
            .fetch_previous_epoch_info(
                event.kmsContextId,
                event.previousEpochId,
                event.materialBlockNumber.saturating_to(),
            )
            .await
            .map_err(ProcessingError::Recoverable)?;

        Ok(KmsGrpcRequest::NewMpcEpoch(NewMpcEpochRequest {
            context_id: Some(u256_to_request_id(event.kmsContextId)),
            epoch_id: Some(u256_to_request_id(event.epochId)),
            previous_epoch: Some(previous_epoch),
            domain: Some(self.domain.clone()),
            extra_data: extra_data_v2_payload(event.kmsContextId, event.epochId),
        }))
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
                "No event found at anchor {} for previous context {}",
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
    async fn fetch_previous_epoch_info(
        &self,
        context_id: U256,
        previous_epoch_id: U256,
        material_block_number: u64,
    ) -> anyhow::Result<PreviousEpochInfo> {
        let key_ids = self
            .kms_generation_contract
            .getCompletedKeyIds()
            .block(material_block_number.into())
            .call()
            .await?;
        let crs_ids = self
            .kms_generation_contract
            .getCompletedCrsIds()
            .block(material_block_number.into())
            .call()
            .await?;

        let mut keys_info = vec![];
        for key_id in key_ids {
            let key_info = self
                .kms_generation_contract
                .getKeyInfo(key_id)
                .call()
                .await?;
            let mut key_digests = vec![];
            for d in key_info.keyDigests.iter() {
                key_digests.push(KeyDigest {
                    key_type: key_type_to_string(d.keyType)?,
                    digest: d.digest.to_vec(),
                });
            }
            keys_info.push(KeyInfo {
                key_id: Some(u256_to_request_id(key_id)),
                preproc_id: Some(u256_to_request_id(key_info.prepKeygenId)),
                key_parameters: params_type_to_fhe_parameter(key_info.paramsType)?,
                key_digests,
            });
        }

        let mut crs_info = vec![];
        for crs_id in crs_ids {
            let crs_material = self
                .kms_generation_contract
                .getCrsMaterials(crs_id)
                .call()
                .await?;
            crs_info.push(CrsInfo {
                crs_id: Some(u256_to_request_id(crs_id)),
                crs_digest: crs_material._1.to_vec(),
            });
        }

        Ok(PreviousEpochInfo {
            context_id: Some(u256_to_request_id(context_id)),
            epoch_id: Some(u256_to_request_id(previous_epoch_id)),
            keys_info,
            crs_info,
        })
    }
}

fn build_new_kms_context_grpc_from_event(
    event: &NewKmsContext,
) -> Result<NewMpcContextRequest, ProcessingError> {
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
    let threshold = i32::try_from(event.thresholds.mpc)
        .map_err(|e| ProcessingError::Irrecoverable(anyhow!("Invalid threshold value: {e}")))?;

    Ok(NewMpcContextRequest {
        new_context: Some(MpcContext {
            mpc_nodes,
            context_id: Some(u256_to_request_id(event.contextId)),
            software_version: event.softwareVersion.clone(),
            threshold,
            pcr_values,
        }),
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
    }
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
