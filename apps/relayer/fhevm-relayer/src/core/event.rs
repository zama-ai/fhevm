use crate::blockchain::ethereum::bindings::DecyptionManager::PublicDecryptionResponse;
use crate::http::input_http_listener::{
    InputProofRequestJson, InputProofResponseJson, InputProofResponsePayloadJson,
};
use crate::orchestrator::traits::Event;
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::{primitives::U256, rpc::types::Log};
use std::fmt::Display;
use std::str::FromStr;
use strum_macros::Display;
use uuid::Uuid;

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
            RelayerEventData::EventLogFromHostL1 { .. } => 0,
            RelayerEventData::EventLogResponseFromGwL2 { .. } => 3,
            RelayerEventData::Decrypt(decrypt_event) => match decrypt_event {
                DecryptEventData::RequestRcvd { .. } => 1,
                DecryptEventData::RequestSentToGwL2 { .. } => 2,
                DecryptEventData::ResponseRcvdFromGwL2 { .. } => 4,
                DecryptEventData::ResponseSentToHostL1 { .. } => 5,
                DecryptEventData::Failed { .. } => 6,
            },
            RelayerEventData::Input(input_event) => match input_event {
                InputEventData::ReqFromUser { .. } => 7,
                InputEventData::RequestSentToGwL2 { .. } => 8,
                InputEventData::RespFromGwL2 { .. } => 9,
                InputEventData::EventLogResponseFromGwL2 { .. } => 10,
                InputEventData::Failed { .. } => 11,
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
    // Raw event log from ethereum. Handler will check event type, decode the
    // event, store ethereum related contextual data and dispatch a decryption
    // request event.
    EventLogFromHostL1 {
        // For ethereum handler
        // TODO: Make relayer event generic of this log type, to make it blockchain agnostic.
        event_log: Log,
    },

    // Raw event log from gateway l2. Will be processed by gateway l2 handler.
    // Handler will check the event type and decode the event. After decoding,
    // it will check if gateway_l2_request_id is available in the contextual
    // data store of the handler. if not, drops the request (not meant for this
    // relayer instance). if found, creates the next event with the original
    // orchestrator request id retreived from contextual data store.
    EventLogResponseFromGwL2 {
        // For gateway l2 handler
        log: Log,
    },

    Decrypt(DecryptEventData),
    Input(InputEventData),
}

impl AsRef<str> for RelayerEventData {
    fn as_ref(&self) -> &str {
        match self {
            RelayerEventData::EventLogFromHostL1 { .. } => "EventLogFromHostL1",
            RelayerEventData::EventLogResponseFromGwL2 { .. } => "EventLogResponseFromGwL2",
            RelayerEventData::Decrypt(decrypt_event) => decrypt_event.event_name(),
            RelayerEventData::Input(input_event) => input_event.event_name(),
        }
    }
}

#[derive(Clone, Debug, Display)]
pub enum DecryptionType {
    PublicDecrypt,
    UserDecrypt { user_public_key: Vec<u8> },
}

#[derive(Clone, Debug)]
pub enum DecryptedValue {
    PublicDecrypt {
        plaintext: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    },
    UserDecrypt {
        user_encrypted_plaintext_shares: Vec<Vec<u8>>,
        signatures: Vec<Vec<u8>>,
    },
}

#[derive(Clone, Debug)]
pub enum DecryptEventData {
    // Decryption request after processing by ethereum adapter. This will be
    // picked up by gateway l2 adapter, which will send a request to decryption
    // manager contract on the gateway l2 blockchain.
    // After sending the request, it will receive a gateway_l2_request_id, which
    // it will persist in contextual data of gateway l2 adapter.
    //
    // After this system will wait until a gateway l2 listener catches a response and creates a new request.
    RequestRcvd {
        // For gateway l2 handler
        ct_handles: Vec<[u8; 32]>,
        operation: DecryptionType,
    },

    RequestSentToGwL2 {
        decryption_public_id: U256,
    },

    // Raw event log from gateway l2. Will be processed by gateway l2 handler.
    // Handler will check the event type and decode the event. After decoding,
    // it will check if gateway_l2_request_id is available in the contextual
    // data store of the handler. if not, drops the request (not meant for this
    // relayer instance). if found, creates the next event with the original
    // orchestrator request id retreived from contextual data store.
    ResponseRcvdFromGwL2 {
        // For ethereum handler
        public_decryption_response: PublicDecryptionResponse,
    },
    // This event data could be used to update the dashboard.
    ResponseSentToHostL1,
    // For no handler, just status update.
    Failed {
        // For no handler, just status updated.
        error: String,
    },
}

impl DecryptEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            DecryptEventData::RequestRcvd { .. } => "Decrypt::RequestRcvd",
            DecryptEventData::RequestSentToGwL2 { .. } => "Decrypt::RequestSendToGwL2",
            DecryptEventData::ResponseRcvdFromGwL2 { .. } => "Decrypt:ResponseRcvdFromGwL2",
            DecryptEventData::ResponseSentToHostL1 => "Decrypt::ResponseSentToHostL1",
            DecryptEventData::Failed { .. } => "Decrypt::Failed",
        }
    }
}

#[derive(Clone, Debug)]
pub enum InputEventData {
    ReqFromUser {
        input_proof_request: InputProofRequest,
    },
    RespFromGwL2 {
        input_proof_response: InputProofResponse,
    },
    RequestSentToGwL2 {
        zkpok_public_id: U256,
    },
    EventLogResponseFromGwL2 {
        log: Log,
    },
    Failed {
        error: String,
    },
}

impl InputEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            InputEventData::ReqFromUser { .. } => "Input::ReqFromUser",
            InputEventData::RespFromGwL2 { .. } => "Input::RespFromGwL2",
            InputEventData::RequestSentToGwL2 { .. } => "Input::RequestSentToGwL2",
            InputEventData::EventLogResponseFromGwL2 { .. } => "Input::EventLogResponseFromGwL2",
            InputEventData::Failed { .. } => "Input::Failed",
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
        let contract_chain_id = U256::from_str_radix(&json.contractChainId, 16)
            .map_err(|e| format!("Error parsing contractChainId: {}", e))?;

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
    fn test_input_proof_request_conversion() -> Result<(), Box<dyn std::error::Error>> {
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
