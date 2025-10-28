use crate::core::{config::Config, event_processor::eip712::alloy_to_protobuf_domain};
use alloy::{hex, primitives::U256, sol_types::Eip712Domain};
use connector_utils::types::KmsGrpcRequest;
use fhevm_gateway_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeyReshareSameSet, KeygenRequest, PrepKeygenRequest,
};
use kms_grpc::kms::v1::{
    CrsGenRequest, InitRequest, InitiateResharingRequest, KeyGenPreprocRequest, KeyGenRequest,
    RequestId,
};
use std::borrow::Cow;
use tracing::{error, info};

#[derive(Clone)]
/// The struct responsible of processing incoming key management requests.
pub struct KMSGenerationProcessor {
    /// The EIP712 domain of the `KMSGeneration` contract.
    domain: Eip712Domain,
}

impl KMSGenerationProcessor {
    pub fn new(config: &Config) -> Self {
        let domain = Eip712Domain {
            name: Some(Cow::Owned(
                config.kms_generation_contract.domain_name.clone(),
            )),
            version: Some(Cow::Owned(
                config.kms_generation_contract.domain_version.clone(),
            )),
            chain_id: Some(U256::from(config.chain_id)),
            verifying_contract: Some(config.kms_generation_contract.address),
            salt: None,
        };

        Self { domain }
    }

    pub fn prepare_prep_keygen_request(
        &self,
        prep_keygen_request: PrepKeygenRequest,
    ) -> anyhow::Result<KmsGrpcRequest> {
        let domain_msg = alloy_to_protobuf_domain(&self.domain)?;
        info!("Eip712Domain constructed: {domain_msg:?}",);

        let request_id = Some(RequestId {
            request_id: hex::encode(prep_keygen_request.prepKeygenId.to_be_bytes::<32>()),
        });

        Ok(KmsGrpcRequest::PrepKeygen(KeyGenPreprocRequest {
            request_id,
            domain: Some(domain_msg),
            params: prep_keygen_request.paramsType as i32,
            epoch_id: None,
            context_id: None,
            // Used to generate other types of key, but not planned to be supported by the Gateway
            keyset_config: None,
        }))
    }

    pub fn prepare_keygen_request(
        &self,
        keygen_request: KeygenRequest,
    ) -> anyhow::Result<KmsGrpcRequest> {
        let domain_msg = alloy_to_protobuf_domain(&self.domain)?;
        info!("Eip712Domain constructed: {domain_msg:?}",);

        let request_id = Some(RequestId {
            request_id: hex::encode(keygen_request.keyId.to_be_bytes::<32>()),
        });
        let preproc_id = Some(RequestId {
            request_id: hex::encode(keygen_request.prepKeygenId.to_be_bytes::<32>()),
        });

        Ok(KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id,
            preproc_id,
            domain: Some(domain_msg),
            params: None,
            epoch_id: None,
            context_id: None,
            // Used to generate other types of key, but not planned to be supported by the Gateway
            keyset_config: None,
            keyset_added_info: None,
        }))
    }

    pub fn prepare_crsgen_request(
        &self,
        crsgen_request: CrsgenRequest,
    ) -> anyhow::Result<KmsGrpcRequest> {
        let domain_msg = alloy_to_protobuf_domain(&self.domain)?;
        info!("Eip712Domain constructed: {domain_msg:?}",);

        let request_id = Some(RequestId {
            request_id: hex::encode(crsgen_request.crsId.to_be_bytes::<32>()),
        });
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
            request_id,
            domain: Some(domain_msg),
            params: crsgen_request.paramsType as i32,
            max_num_bits,
            context_id: None,
        }))
    }

    pub fn prepare_prss_init_request(&self, id: U256) -> KmsGrpcRequest {
        let request_id = Some(RequestId {
            request_id: hex::encode(id.to_be_bytes::<32>()),
        });
        KmsGrpcRequest::PrssInit(InitRequest { request_id })
    }

    pub fn prepare_initiate_resharing_request(
        &self,
        req: KeyReshareSameSet,
    ) -> anyhow::Result<KmsGrpcRequest> {
        let request_id = Some(RequestId {
            request_id: hex::encode(req.keyReshareId.to_be_bytes::<32>()),
        });

        let domain_msg = alloy_to_protobuf_domain(&self.domain)?;
        info!("Eip712Domain constructed: {domain_msg:?}");

        Ok(KmsGrpcRequest::KeyReshareSameSet(
            InitiateResharingRequest {
                request_id,
                key_id: Some(RequestId {
                    request_id: hex::encode(req.keyId.to_be_bytes::<32>()),
                }),
                preproc_id: Some(RequestId {
                    request_id: hex::encode(req.prepKeygenId.to_be_bytes::<32>()),
                }),
                key_parameters: req.paramsType as i32,
                domain: Some(domain_msg),
                epoch_id: None,
                context_id: None,
            },
        ))
    }
}
