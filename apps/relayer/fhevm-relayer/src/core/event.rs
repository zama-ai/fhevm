use crate::http::input_http_listener::{
    InputProofRequestJson, InputProofResponseJson, InputProofResponsePayloadJson,
};
use crate::http::userdecrypt_http_listener::{
    UserDecryptRequestJson, UserDecryptResponseJson, UserDecryptResponsePayloadJson,
};
use crate::orchestrator::traits::Event;
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::{primitives::U256, rpc::types::Log};
use std::fmt::Display;
use std::str::FromStr;
use tracing::info;
use uuid::Uuid;

#[repr(u8)]
#[derive(Debug)]
pub enum GenericEventId {
    EventLogRcvdFromHostBc = 0,
    EventLogRcvdFromGw = 1,
}

impl From<GenericEventId> for u8 {
    fn from(e: GenericEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum PublicDecryptEventId {
    ReqRcvdFromUser = 11,
    ReqSentToGw = 12,
    RespRcvdFromGw = 13,
    RespSentToHostBc = 14,
    Failed = 15,
}

impl From<PublicDecryptEventId> for u8 {
    fn from(e: PublicDecryptEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum UserDecryptEventId {
    ReqRcvdFromUser = 20,
    ReqSentToGw = 21,
    RespRcvdFromGw = 22,
    RespSentToUser = 23,
    Failed = 24,
}

impl From<UserDecryptEventId> for u8 {
    fn from(e: UserDecryptEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum InputProofEventId {
    ReqRcvdFromUser = 30,
    ReqSentToGw = 31,
    RespRcvdFromGw = 32,
    Failed = 33,
}

impl From<InputProofEventId> for u8 {
    fn from(e: InputProofEventId) -> u8 {
        e as u8
    }
}

#[derive(Clone, Debug)]
pub struct RelayerEvent {
    pub request_id: Uuid,
    pub api_version: ApiVersion,
    pub data: RelayerEventData,
}

impl RelayerEvent {
    pub fn new(request_id: Uuid, api_version: ApiVersion, data: RelayerEventData) -> RelayerEvent {
        RelayerEvent {
            request_id,
            api_version,
            data,
        }
    }

    pub fn derive_next_event(self, next_event_data: RelayerEventData) -> RelayerEvent {
        RelayerEvent {
            request_id: self.request_id,
            api_version: self.api_version,
            data: next_event_data,
        }
    }
}

impl Event for RelayerEvent {
    fn event_name(&self) -> &str {
        self.data.as_ref()
    }

    //TODO: Replace boiler plate with macro based code.
    fn event_id(&self) -> u8 {
        match &self.data {
            RelayerEventData::Generic(generic_event) => match generic_event {
                GenericEventData::EventLogFromHostBc { .. } => {
                    GenericEventId::EventLogRcvdFromHostBc.into()
                }
                GenericEventData::EventLogFromGw { .. } => {
                    GenericEventId::EventLogRcvdFromGw.into()
                }
            },
            RelayerEventData::PublicDecrypt(decrypt_event) => match decrypt_event {
                PublicDecryptEventData::ReqRcvdFromHostBc { .. } => {
                    PublicDecryptEventId::ReqRcvdFromUser.into()
                }
                PublicDecryptEventData::ReqSentToGw { .. } => {
                    PublicDecryptEventId::ReqSentToGw.into()
                }
                PublicDecryptEventData::RespRcvdFromGw { .. } => {
                    PublicDecryptEventId::RespRcvdFromGw.into()
                }
                PublicDecryptEventData::RespSentToHostBc { .. } => {
                    PublicDecryptEventId::RespSentToHostBc.into()
                }
                PublicDecryptEventData::Failed { .. } => PublicDecryptEventId::Failed.into(),
            },
            RelayerEventData::UserDecrypt(decrypt_event) => match decrypt_event {
                UserDecryptEventData::ReqRcvdFromUser { .. } => {
                    UserDecryptEventId::ReqRcvdFromUser.into()
                }
                UserDecryptEventData::ReqSentToGw { .. } => UserDecryptEventId::ReqSentToGw.into(),
                UserDecryptEventData::RespRcvdFromGw { .. } => {
                    UserDecryptEventId::RespRcvdFromGw.into()
                }
                UserDecryptEventData::RespSentToHostBc { .. } => {
                    UserDecryptEventId::RespSentToUser.into()
                }
                UserDecryptEventData::Failed { .. } => UserDecryptEventId::Failed.into(),
            },
            RelayerEventData::InputProof(input_event) => match input_event {
                InputProofEventData::ReqRcvdFromUser { .. } => {
                    InputProofEventId::ReqRcvdFromUser.into()
                }
                InputProofEventData::ReqSentToGw { .. } => InputProofEventId::ReqSentToGw.into(),
                InputProofEventData::RespRcvdFromGw { .. } => {
                    InputProofEventId::RespRcvdFromGw.into()
                }
                InputProofEventData::Failed { .. } => InputProofEventId::Failed.into(),
            },
        }
    }

    fn request_id(&self) -> Uuid {
        self.request_id
    }
}

#[derive(Clone, Debug)]
pub struct ApiVersion {
    pub category: ApiCategory,
    pub number: u8,
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/v{}", self.category, self.number)
    }
}

#[derive(Clone, Debug)]
pub enum ApiCategory {
    PRODUCTION,
    EXPERIMENTAL,
}

#[derive(Clone, Debug)]
pub enum RelayerEventData {
    Generic(GenericEventData),
    PublicDecrypt(PublicDecryptEventData),
    UserDecrypt(UserDecryptEventData),
    InputProof(InputProofEventData),
}

impl AsRef<str> for RelayerEventData {
    fn as_ref(&self) -> &str {
        match self {
            RelayerEventData::Generic(generic_event) => generic_event.event_name(),
            RelayerEventData::PublicDecrypt(decrypt_event) => decrypt_event.event_name(),
            RelayerEventData::UserDecrypt(decrypt_event) => decrypt_event.event_name(),
            RelayerEventData::InputProof(input_event) => input_event.event_name(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum GenericEventData {
    // Raw event log from ethereum. Handler will check event type, decode the
    // event, store ethereum related contextual data and dispatch a decryption
    // request event.
    EventLogFromHostBc {
        // For ethereum handler
        // TODO: Make relayer event generic of this log type, to make it blockchain agnostic.
        log: Log,
    },

    // Raw event log from gateway l2. Will be processed by gateway l2 handler.
    // Handler will check the event type and decode the event. After decoding,
    // it will check if gateway_l2_request_id is available in the contextual
    // data store of the handler. if not, drops the request (not meant for this
    // relayer instance). if found, creates the next event with the original
    // orchestrator request id retreived from contextual data store.
    EventLogFromGw {
        // For gateway l2 handler
        log: Log,
    },
}

impl GenericEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            GenericEventData::EventLogFromHostBc { .. } => "Generic::EventLogFromHostBc",
            GenericEventData::EventLogFromGw { .. } => "Generic::EventLogFromGw",
        }
    }
}

#[derive(Clone, Debug)]
pub enum PublicDecryptEventData {
    // Decryption request after processing by ethereum adapter. This will be
    // picked up by gateway l2 adapter, which will send a request to decryption
    // manager contract on the gateway l2 blockchain.
    // After sending the request, it will receive a gateway_l2_request_id, which
    // it will persist in contextual data of gateway l2 adapter.
    //
    // After this system will wait until a gateway l2 listener catches a response and creates a new request.
    //
    // For gateway l2 handler
    ReqRcvdFromHostBc {
        decrypt_request: PublicDecryptRequest,
    },

    ReqSentToGw {
        public_decryption_id: U256,
    },

    // Raw event log from gateway l2. Will be processed by gateway l2 handler.
    // Handler will check the event type and decode the event. After decoding,
    // it will check if gateway_l2_request_id is available in the contextual
    // data store of the handler. if not, drops the request (not meant for this
    // relayer instance). if found, creates the next event with the original
    // orchestrator request id retreived from contextual data store.
    //
    // For ethereum handler
    RespRcvdFromGw {
        decrypt_response: PublicDecryptResponse,
    },

    // This event data could be used to update the dashboard.
    RespSentToHostBc,

    // For no handler, just status update.
    Failed {
        // For no handler, just status updated.
        error: String,
    },
}

impl PublicDecryptEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            PublicDecryptEventData::ReqRcvdFromHostBc { .. } => "PublicDecrypt::ReqRcvdFromHostBc",
            PublicDecryptEventData::ReqSentToGw { .. } => "PublicDecrypt::ReqSentToGw",
            PublicDecryptEventData::RespRcvdFromGw { .. } => "PublicDecrypt::RespRcvdFromGw",
            PublicDecryptEventData::RespSentToHostBc => "PublicDecrypt::RespSentToHostBc",
            PublicDecryptEventData::Failed { .. } => "PublicDecrypt::Failed",
        }
    }
}

#[derive(Clone, Debug)]
pub enum UserDecryptEventData {
    ReqRcvdFromUser {
        decrypt_request: UserDecryptRequest,
    },

    ReqSentToGw {
        user_decryption_id: U256,
    },

    RespRcvdFromGw {
        decrypt_response: UserDecryptResponse,
    },

    // This event data could be used to update the dashboard.
    RespSentToHostBc,

    // For no handler, just status update.
    Failed {
        // For no handler, just status updated.
        error: String,
    },
}

impl UserDecryptEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            UserDecryptEventData::ReqRcvdFromUser { .. } => "UserDecrypt::ReqRcvdFromUser",
            UserDecryptEventData::ReqSentToGw { .. } => "UserDecrypt::ReqSentToGw",
            UserDecryptEventData::RespRcvdFromGw { .. } => "UserDecrypt::RespRcvdFromGw",
            UserDecryptEventData::RespSentToHostBc => "UserDecrypt::RespSentToHostBc",
            UserDecryptEventData::Failed { .. } => "UserDecrypt::Failed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublicDecryptRequest {
    pub ct_handles: Vec<[u8; 32]>,
}

#[derive(Debug, Clone)]
pub struct UserDecryptRequest {
    pub ct_handles: Vec<Bytes>,
    pub encryption_key: Bytes,
    pub user_address: Address,
    pub contract_address: Address,
    pub contracts_chain_id: U256,
    pub signature: Bytes,
}

#[derive(Debug, Clone)]
pub struct PublicDecryptResponse {
    pub gateway_request_id: U256,
    pub decrypted_value: Bytes,
    pub signatures: Vec<Bytes>,
}

#[derive(Debug, Clone)]
pub struct UserDecryptResponse {
    pub gateway_request_id: U256,
    pub reencrypted_shares: Vec<Bytes>,
    pub signatures: Vec<Bytes>,
}

impl TryFrom<UserDecryptRequestJson> for UserDecryptRequest {
    type Error = anyhow::Error;

    fn try_from(value: UserDecryptRequestJson) -> Result<Self, Self::Error> {
        let ct_handles: Vec<Bytes> = vec![(Bytes::from_str(&value.ct_handle)?)];

        Ok(UserDecryptRequest {
            ct_handles,
            encryption_key: Bytes::from_str(&value.enc_key)?,
            user_address: Address::from_str(&value.userAddress)?,
            contract_address: Address::from_str(&value.contractAddress)?,
            contracts_chain_id: parse_chain_id(&value.chainId)?,
            signature: Bytes::from_str(&value.signature)?,
        })
    }
}

impl TryFrom<UserDecryptResponse> for UserDecryptResponseJson {
    type Error = String;

    fn try_from(response: UserDecryptResponse) -> Result<Self, Self::Error> {
        Ok(UserDecryptResponseJson {
            response: UserDecryptResponsePayloadJson {
                reencrypted_shares: response.reencrypted_shares,
                signatures: response.signatures,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub enum InputProofEventData {
    ReqRcvdFromUser {
        input_proof_request: InputProofRequest,
    },
    ReqSentToGw {
        zkproof_id: U256,
    },
    RespRcvdFromGw {
        input_proof_response: InputProofResponse,
    },
    Failed {
        error: String,
    },
}

impl InputProofEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            InputProofEventData::ReqRcvdFromUser { .. } => "Input::ReqRcvdFromUser",
            InputProofEventData::RespRcvdFromGw { .. } => "Input::RespRcvdFromGw",
            InputProofEventData::ReqSentToGw { .. } => "Input::ReqSentToGw",
            InputProofEventData::Failed { .. } => "Input::Failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputProofRequest {
    pub contract_chain_id: U256,
    pub contract_address: Address,
    pub user_address: Address,
    pub ciphetext_with_zk_proof: Bytes,
}

impl InputProofRequest {
    pub fn new(
        contract_chain_id: U256,
        contract_address: Address,
        user_address: Address,
        ciphetext_with_zk_proof: Bytes,
    ) -> InputProofRequest {
        InputProofRequest {
            contract_chain_id,
            contract_address,
            user_address,
            ciphetext_with_zk_proof,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputProofResponse {
    pub handles: Vec<FixedBytes<32>>,
    pub signatures: Vec<Bytes>,
}

impl InputProofResponse {
    pub fn new(handles: Vec<FixedBytes<32>>, signatures: Vec<Bytes>) -> InputProofResponse {
        InputProofResponse {
            handles,
            signatures,
        }
    }
}

impl TryFrom<InputProofRequestJson> for InputProofRequest {
    type Error = String;

    fn try_from(json: InputProofRequestJson) -> Result<Self, Self::Error> {
        info!("json.contractChainId: {:?}", json.contractChainId);
        let contract_chain_id = parse_chain_id(&json.contractChainId)
            .map_err(|e| format!("Error parsing contractChainId: {:?}", e))?;
        info!("contract_chain_id decoded: {:?}", contract_chain_id);

        let contract_address = Address::from_str(&json.contractAddress)
            .map_err(|e| format!("Error parsing contractAddress: {:?}", e))?;

        let user_address = Address::from_str(&json.userAddress)
            .map_err(|e| format!("Error parsing userAddress: {:?}", e))?;

        // Convert ciphertext_with_zk_proof.
        // This field is assumed to be a hex string without a "0x" prefix.
        let proof_bytes = hex::decode(&json.ciphertextWithZkpok)
            .map_err(|e| format!("Error decoding ciphertextWithZkpok: {}", e))?;
        let ciphetext_with_zk_proof = Bytes::from(proof_bytes);

        Ok(InputProofRequest {
            contract_chain_id,
            contract_address,
            user_address,
            ciphetext_with_zk_proof,
        })
    }
}

fn parse_chain_id(
    chain_id: &str,
) -> Result<alloy::primitives::Uint<256, 4>, alloy::primitives::ruint::ParseError> {
    let contract_chain_id = if chain_id == "1e240" {
        info!("Special case detected: contractChainId is 1e240, using hardcoded value 123456");
        U256::from(123456u64)
    } else if chain_id == "3039" {
        info!("Special case detected: contractChainId is 3039, using hardcoded value 12345");
        U256::from(12345u64)
    } else if chain_id.starts_with("0x") {
        // Parse as hex if it starts with 0x
        U256::from_str(chain_id)?
    } else {
        // Parse as decimal otherwise
        U256::from_str_radix(chain_id, 10)?
    };
    Ok(contract_chain_id)
}

impl TryFrom<InputProofResponse> for InputProofResponseJson {
    type Error = String;

    fn try_from(response: InputProofResponse) -> Result<Self, Self::Error> {
        Ok(InputProofResponseJson {
            response: InputProofResponsePayloadJson {
                handles: response
                    .handles
                    .into_iter()
                    .map(|handle| format!("{:#x}", handle))
                    .collect(),
                signatures: response
                    .signatures
                    .into_iter()
                    .map(|sig| format!("{:#x}", sig))
                    .collect(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::{TryFrom, TryInto};
    use std::str::FromStr;

    // Define constants for the test strings.
    const CHAIN_ID: &str = "123456";
    const CONTRACT_ADDRESS: &str = "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d";
    const USER_ADDRESS: &str = "0x12B064FB845C1cc05e9493856a1D637a73e944bE";
    const CIPHERTEXT: &str =
        "12B06C1cc05e9493856a1D637a74FAb30999D17FAAB8c95B2eCD500cFeFc8f658f15dB8453e944bE";

    #[test]
    #[ignore]
    fn test_input_proof_request_conversion_() -> Result<(), Box<dyn std::error::Error>> {
        let json = InputProofRequestJson {
            contractChainId: CHAIN_ID.to_string(),
            contractAddress: CONTRACT_ADDRESS.to_string(),
            userAddress: USER_ADDRESS.to_string(),
            ciphertextWithZkpok: CIPHERTEXT.to_string(),
        };

        let request = InputProofRequest::try_from(json)?;

        assert_eq!(
            request.contract_chain_id,
            U256::from_str_radix(CHAIN_ID, 16)?
        );
        assert_eq!(
            request.contract_address,
            Address::from_str(CONTRACT_ADDRESS)?
        );
        assert_eq!(request.user_address, Address::from_str(USER_ADDRESS)?);

        let expected_bytes = hex::decode(CIPHERTEXT)?;
        assert_eq!(request.ciphetext_with_zk_proof, Bytes::from(expected_bytes));

        Ok(())
    }

    const FIXED_BYTE_VALUE: u8 = 0x11;
    const SIGNATURE_BYTE_VALUE: u8 = 0x22;

    #[test]

    fn test_input_proof_response_conversion() {
        let fixed = FixedBytes::<32>::from([FIXED_BYTE_VALUE; 32]);
        let signature = Bytes::from(vec![SIGNATURE_BYTE_VALUE; 32]);

        let response = InputProofResponse {
            handles: vec![fixed],
            signatures: vec![signature],
        };

        let json: InputProofResponseJson = response.try_into().unwrap();

        // Using alloy's formatting we expect a 0x‑prefixed hex string.
        let expected_handle = format!("{:#x}", FixedBytes::<32>::from([FIXED_BYTE_VALUE; 32]));
        let expected_signature = format!("{:#x}", Bytes::from(vec![SIGNATURE_BYTE_VALUE; 32]));

        assert_eq!(json.response.handles, vec![expected_handle.clone()]);
        assert_eq!(json.response.signatures, vec![expected_signature]);
    }
}
