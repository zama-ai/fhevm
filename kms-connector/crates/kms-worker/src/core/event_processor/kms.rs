use crate::core::config::Config;
use alloy::primitives::U256;
use connector_utils::types::{KmsGrpcRequest, u256_to_request_id};
use fhevm_gateway_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeyReshareSameSet, KeygenRequest, PrepKeygenRequest,
};
use kms_grpc::kms::v1::{
    CrsGenRequest, Eip712DomainMsg, InitRequest, InitiateResharingRequest, KeyGenPreprocRequest,
    KeyGenRequest,
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
            chain_id: U256::from(config.gateway_chain_id).to_be_bytes_vec(),
            verifying_contract: config.kms_generation_contract.address.to_string(),
            salt: None,
        };

        Self { domain }
    }

    pub fn prepare_prep_keygen_request(
        &self,
        prep_keygen_request: &PrepKeygenRequest,
    ) -> KmsGrpcRequest {
        KmsGrpcRequest::PrepKeygen(KeyGenPreprocRequest {
            request_id: Some(u256_to_request_id(prep_keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: prep_keygen_request.paramsType as i32,
            epoch_id: None,
            context_id: None,
            // Used to generate other types of key, but not planned to be supported by the Gateway
            keyset_config: None,
        })
    }

    pub fn prepare_keygen_request(&self, keygen_request: &KeygenRequest) -> KmsGrpcRequest {
        KmsGrpcRequest::Keygen(KeyGenRequest {
            request_id: Some(u256_to_request_id(keygen_request.keyId)),
            preproc_id: Some(u256_to_request_id(keygen_request.prepKeygenId)),
            domain: Some(self.domain.clone()),
            params: None,
            epoch_id: None,
            context_id: None,
            // Used to generate other types of key, but not planned to be supported by the Gateway
            keyset_config: None,
            keyset_added_info: None,
        })
    }

    pub fn prepare_crsgen_request(&self, crsgen_request: &CrsgenRequest) -> KmsGrpcRequest {
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

        KmsGrpcRequest::Crsgen(CrsGenRequest {
            request_id: Some(u256_to_request_id(crsgen_request.crsId)),
            domain: Some(self.domain.clone()),
            params: crsgen_request.paramsType as i32,
            max_num_bits,
            context_id: None,
        })
    }

    pub fn prepare_prss_init_request(&self, id: U256) -> KmsGrpcRequest {
        KmsGrpcRequest::PrssInit(InitRequest {
            request_id: Some(u256_to_request_id(id)),
            context_id: None, // TODO: update once context is implemented
        })
    }

    pub fn prepare_initiate_resharing_request(&self, req: &KeyReshareSameSet) -> KmsGrpcRequest {
        KmsGrpcRequest::KeyReshareSameSet(InitiateResharingRequest {
            request_id: Some(u256_to_request_id(req.keyReshareId)),
            key_id: Some(u256_to_request_id(req.keyId)),
            key_digests: vec![], // TODO: update once resharing is implemented
            preproc_id: Some(u256_to_request_id(req.prepKeygenId)),
            key_parameters: req.paramsType as i32,
            domain: Some(self.domain.clone()),
            epoch_id: None,
            context_id: None,
        })
    }
}
