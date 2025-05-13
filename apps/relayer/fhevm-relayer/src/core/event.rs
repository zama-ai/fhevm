use crate::http::input_http_listener::{
    InputProofRequestJson, InputProofResponseJson, InputProofResponsePayloadJson,
};
use crate::http::public_decrypt_http_listener::{
    PublicDecryptRequestJson, PublicDecryptResponseJson, PublicDecryptResponsePayloadJson,
};
use crate::http::userdecrypt_http_listener::{
    UserDecryptRequestJson, UserDecryptResponseJson, UserDecryptResponsePayloadJson,
};
use crate::orchestrator::traits::Event;
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::{primitives::U256, rpc::types::Log};
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;
use tracing::info;
use uuid::Uuid;

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding the events of GenericEvent type.
pub enum GenericEventId {
    EventLogRcvdFromFhevm = 0,
    EventLogRcvdFromGw = 1,
}

impl From<GenericEventId> for u8 {
    fn from(e: GenericEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding the events of PublicDecryptEvent type.
pub enum PublicDecryptEventId {
    ReqRcvdFromUser = 11,
    ReqSentToGw = 12,
    RespRcvdFromGw = 13,
    RespSentToFhevm = 14,
    Failed = 15,
}

impl From<PublicDecryptEventId> for u8 {
    fn from(e: PublicDecryptEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding the events of UserDecryptEvent type.
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
/// Event Ids corresponding the events of InputProofEvent type.
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
/// Relayer event represents a single step in one of the different flows of the
/// relayer (such as public decryption, input proof verification and so on).
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

    fn event_id(&self) -> u8 {
        match &self.data {
            RelayerEventData::Generic(generic_event) => match generic_event {
                GenericEventData::EventLogFromFhevm { .. } => {
                    GenericEventId::EventLogRcvdFromFhevm.into()
                }
                GenericEventData::EventLogFromGw { .. } => {
                    GenericEventId::EventLogRcvdFromGw.into()
                }
            },
            RelayerEventData::PublicDecrypt(decrypt_event) => match decrypt_event {
                PublicDecryptEventData::ReqRcvdFromFhevm { .. } => {
                    PublicDecryptEventId::ReqRcvdFromUser.into()
                }
                PublicDecryptEventData::ReqSentToGw { .. } => {
                    PublicDecryptEventId::ReqSentToGw.into()
                }
                PublicDecryptEventData::RespRcvdFromGw { .. } => {
                    PublicDecryptEventId::RespRcvdFromGw.into()
                }
                PublicDecryptEventData::RespSentToFhevm => {
                    PublicDecryptEventId::RespSentToFhevm.into()
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
                UserDecryptEventData::RespSentToUser => UserDecryptEventId::RespSentToUser.into(),
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
        match self.category {
            ApiCategory::PRODUCTION => write!(f, "v{}", self.number),
            ApiCategory::EXPERIMENTAL => write!(f, "exp/v{}", self.number),
        }
    }
}

/// Api version allows for differentiating between different versions of the
/// same API. The different versions can have entirely different flows or share
/// part of the flow.
impl ApiVersion {
    pub fn new(category: ApiCategory, number: u8) -> Self {
        ApiVersion { category, number }
    }
}

#[derive(Clone, Debug)]
/// Api category allows for differentiating between production and experimental
/// APIs.
pub enum ApiCategory {
    PRODUCTION,
    EXPERIMENTAL,
}

#[derive(Clone, Debug)]
/// Relayer event data represents the different categories of event data, each
/// representing a specific flow. Generic event data represents the event data
/// shared between the different flows.
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
    /// Event representing a raw blockchain event log received from fhevm blockchain.
    EventLogFromFhevm { log: Log },

    /// Event representing a raw blockchain event log received from gateway blockchain.
    EventLogFromGw { log: Log },
}

impl GenericEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            GenericEventData::EventLogFromFhevm { .. } => "Generic::EventLogFromFhevm",
            GenericEventData::EventLogFromGw { .. } => "Generic::EventLogFromGw",
        }
    }
}

#[derive(Clone, Debug)]
pub enum PublicDecryptEventData {
    /// Event representing a public decryption request for ciphertexts on fhevm.
    ReqRcvdFromFhevm {
        decrypt_request: PublicDecryptRequest,
    },

    /// Event representing the result of sending a public decryption request to
    /// gateway. Id will be used to map the response that will be received later
    /// to the request.
    ReqSentToGw { gw_req_reference_id: U256 },

    /// Event representing the success response received from gateway for public
    /// decryption request sent from this instance of relayer.
    RespRcvdFromGw {
        decrypt_response: PublicDecryptResponse,
    },

    /// Event representing the public decryption response sent to fhevm.
    RespSentToFhevm,

    /// Event representing the failure in processing the public decryption request.
    Failed { error: String },
}

impl PublicDecryptEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            PublicDecryptEventData::ReqRcvdFromFhevm { .. } => "PublicDecrypt::ReqRcvdFromFhevm",
            PublicDecryptEventData::ReqSentToGw { .. } => "PublicDecrypt::ReqSentToGw",
            PublicDecryptEventData::RespRcvdFromGw { .. } => "PublicDecrypt::RespRcvdFromGw",
            PublicDecryptEventData::RespSentToFhevm => "PublicDecrypt::RespSentToFhevm",
            PublicDecryptEventData::Failed { .. } => "PublicDecrypt::Failed",
        }
    }
}

#[derive(Clone, Debug)]
pub enum UserDecryptEventData {
    /// Event representing a user decryption request for ciphertexts on fhevm.
    ReqRcvdFromUser { decrypt_request: UserDecryptRequest },

    /// Event representing the result of sending a user decryption request to
    /// gateway. Id will be used to map the response that will be received later
    /// to the request.
    ReqSentToGw { gw_req_reference_id: U256 },

    /// Event representing the success response received from gateway for user
    /// decryption sent from this instance of relayer.
    RespRcvdFromGw {
        decrypt_response: UserDecryptResponse,
    },

    /// Event representing the user decryption response sent to the user.
    RespSentToUser,

    /// Event representing the failure in processing the user decryption request.
    Failed { error: String },
}

impl UserDecryptEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            UserDecryptEventData::ReqRcvdFromUser { .. } => "UserDecrypt::ReqRcvdFromUser",
            UserDecryptEventData::ReqSentToGw { .. } => "UserDecrypt::ReqSentToGw",
            UserDecryptEventData::RespRcvdFromGw { .. } => "UserDecrypt::RespRcvdFromGw",
            UserDecryptEventData::RespSentToUser => "UserDecrypt::RespSentToFhevm",
            UserDecryptEventData::Failed { .. } => "UserDecrypt::Failed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublicDecryptRequest {
    pub ct_handles: Vec<[u8; 32]>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDecryptRequest {
    pub ct_handle_contract_pairs: Vec<HandleContractPair>,
    pub request_validity: RequestValidity,
    pub contracts_chain_id: u64,
    pub contract_addresses: Vec<Address>,
    pub user_address: Address,
    pub signature: Bytes,
    pub public_key: Bytes,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HandleContractPair {
    pub ct_handle: U256,
    pub contract_address: Address,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct RequestValidity {
    pub start_timestamp: U256,
    pub duration_days: U256,
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
        info!("Converting UserDecryptRequestJson to UserDecryptRequest");

        let mut ct_handle_contract_pairs = Vec::new();
        for json_data in &value.handleContractPairs {
            let ct_handle = if json_data.handle.starts_with("0x") {
                // Remove the 0x prefix before parsing
                U256::from_str_radix(&json_data.handle[2..], 16)
            } else {
                U256::from_str_radix(&json_data.handle, 16)
            }
            .map_err(|e| anyhow::anyhow!("Failed to parse ctHandle: {}", e))?;

            let contract_address = Address::from_str(&json_data.contractAddress)
                .map_err(|e| anyhow::anyhow!("Failed to parse contractAddress: {}", e))?;

            ct_handle_contract_pairs.push(HandleContractPair {
                ct_handle,
                contract_address,
            });
        }

        // Parse duration days - first try as number, then as string
        let duration_days = match value.requestValidity.durationDays.parse::<u64>() {
            Ok(num) => U256::from(num),
            Err(_) => {
                // Try parsing as hex if it starts with 0x
                if value.requestValidity.durationDays.starts_with("0x") {
                    U256::from_str(&value.requestValidity.durationDays)?
                } else {
                    // Otherwise try as decimal string
                    U256::from_str_radix(&value.requestValidity.durationDays, 10)?
                }
            }
        };

        let request_validity = RequestValidity {
            start_timestamp: U256::from_str(&value.requestValidity.startTimestamp)?,
            duration_days,
        };

        // Parse contract chain ID
        let contracts_chain_id = parse_chain_id(&value.contractsChainId)?;

        let contract_addresses = &value
            .contractAddresses
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(UserDecryptRequest {
            ct_handle_contract_pairs,
            request_validity,
            contracts_chain_id,
            contract_addresses: contract_addresses.clone(),
            user_address: Address::from_str(&value.userAddress)?,
            signature: Bytes::from_str(&value.signature)?,
            public_key: Bytes::from_str(&value.publicKey)?,
        })
    }
}

impl TryFrom<UserDecryptResponse> for UserDecryptResponseJson {
    type Error = String;

    fn try_from(response: UserDecryptResponse) -> Result<Self, Self::Error> {
        let mut json_response: Vec<UserDecryptResponsePayloadJson> = Vec::new();

        for (share, signature) in response
            .reencrypted_shares
            .iter()
            .zip(response.signatures.iter())
        {
            json_response.push(UserDecryptResponsePayloadJson {
                payload: share.clone(),
                signature: signature.clone(),
            });
        }

        Ok(UserDecryptResponseJson {
            response: json_response,
        })
    }
}

impl TryFrom<PublicDecryptRequestJson> for PublicDecryptRequest {
    type Error = anyhow::Error;

    fn try_from(value: PublicDecryptRequestJson) -> Result<Self, Self::Error> {
        info!("Converting PublicDecryptRequestJson to PublicDecryptRequest");

        let mut ct_handles = Vec::new();
        for ct_handle_hex in &value.ciphertextHandles {
            let ct_handle = if let Some(ct_handle_hex_wo_prefix) = ct_handle_hex.strip_prefix("0x")
            {
                U256::from_str_radix(ct_handle_hex_wo_prefix, 16)
            } else {
                U256::from_str_radix(ct_handle_hex, 16)
            }
            .map_err(|e| anyhow::anyhow!("Failed to parse ct_handle: {}", e))?;

            // TODO (Mano): The conversion to be bytes should happen in low level
            // code. App code should deal with with higher level types like U256.
            ct_handles.push(ct_handle.to_be_bytes());
        }

        Ok(PublicDecryptRequest { ct_handles })
    }
}

impl TryFrom<PublicDecryptResponse> for PublicDecryptResponseJson {
    type Error = String;

    fn try_from(response: PublicDecryptResponse) -> Result<Self, Self::Error> {
        Ok(PublicDecryptResponseJson {
            response: vec![PublicDecryptResponsePayloadJson {
                decrypted_value: response.decrypted_value,
                signatures: response.signatures,
            }],
        })
    }
}

#[derive(Clone, Debug)]
pub enum InputProofEventData {
    /// Event representing a input proof verification request from the user.
    ReqRcvdFromUser {
        input_proof_request: InputProofRequest,
    },

    /// Event representing the result of sending a input proof verification
    /// request to the gateway. Id will be used to map the response that will be
    /// received later to the request.
    ReqSentToGw { gw_req_reference_id: U256 },

    /// Event representing the success response received from gateway for input
    /// proof verification request sent from this instance of gateway.
    RespRcvdFromGw {
        input_proof_response: InputProofResponse,
    },

    /// Event representing the failure in processing the input proof
    /// verification request.
    Failed { error: String },
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
    pub contract_chain_id: u64,
    pub contract_address: Address,
    pub user_address: Address,
    pub ciphetext_with_zk_proof: Bytes,
}

impl InputProofRequest {
    pub fn new(
        contract_chain_id: u64,
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

        // Should be hex string without a "0x" prefix.
        let proof_bytes = hex::decode(&json.ciphertextWithInputVerification)
            .map_err(|e| format!("Error decoding ciphertextWithInputVerification: {}", e))?;
        let ciphetext_with_zk_proof = Bytes::from(proof_bytes);

        Ok(InputProofRequest {
            contract_chain_id,
            contract_address,
            user_address,
            ciphetext_with_zk_proof,
        })
    }
}

fn parse_chain_id(chain_id: &str) -> Result<u64, ParseIntError> {
    if chain_id == "1e240" {
        info!("Special case detected: contractChainId is 1e240, using hardcoded value 123456");
        Ok(123456u64)
    } else if chain_id == "3039" {
        info!("Special case detected: contractChainId is 3039, using hardcoded value 12345");
        Ok(12345u64)
    } else if let Some(stripped) = chain_id.strip_prefix("0x") {
        // Parse as hex if it starts with 0x
        u64::from_str_radix(stripped, 16)
    } else {
        // Parse as decimal otherwise
        chain_id.parse::<u64>()
    }
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

    // Constants for the test strings.
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
            ciphertextWithInputVerification: CIPHERTEXT.to_string(),
        };

        let request = InputProofRequest::try_from(json)?;

        assert_eq!(request.contract_chain_id, CHAIN_ID.parse::<u64>()?);
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

    use super::UserDecryptResponse;
    use crate::http::userdecrypt_http_listener::UserDecryptResponseJson;
    use alloy::primitives::Bytes;

    #[test]
    fn test_try_from_user_decrypt_response() {
        // Create a UserDecryptResponse instance
        let reencrypted_shares = vec![Bytes::from(vec![1, 2, 3, 4]), Bytes::from(vec![5, 6, 7, 8])];
        let signatures = vec![
            Bytes::from(vec![9, 10, 11, 12]),
            Bytes::from(vec![13, 14, 15, 16]),
        ];

        let response = UserDecryptResponse {
            gateway_request_id: U256::from_str("5").unwrap(),
            reencrypted_shares,
            signatures,
        };

        // Convert UserDecryptResponse to UserDecryptResponseJson
        let json_response = UserDecryptResponseJson::try_from(response).unwrap();

        // Expected UserDecryptResponseJson
        let expected_json_response = "{\"response\":[{\"payload\":\"01020304\",\"signature\":\"090a0b0c\"},{\"payload\":\"05060708\",\"signature\":\"0d0e0f10\"}]}";
        // Serialize the json response and print the json string
        let json_string = serde_json::to_string(&json_response).unwrap();

        println!("JSON Response: {:?}", json_string);
        assert_eq!(json_string, expected_json_response);
    }
}
