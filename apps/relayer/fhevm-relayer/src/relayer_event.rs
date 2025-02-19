use crate::orchestrator::traits::Event;
use alloy::primitives::Address;
use alloy::{primitives::U256, rpc::types::Log};
use std::fmt::Display;
use strum_macros::Display;
use uuid::Uuid;

#[derive(Clone)]
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
            RelayerEventData::DecryptRequestRcvd { .. } => 1,
            RelayerEventData::DecryptionRequestSentToGwL2 { .. } => 2,
            RelayerEventData::EventLogFromGwL2 { .. } => 3,
            RelayerEventData::DecryptionResponseRcvdFromGwL2 { .. } => 4,
            RelayerEventData::DecryptResponseSentToHostL1 { .. } => 5,
            RelayerEventData::DecryptionFailed { .. } => 6,
            RelayerEventData::Input(input_event) => match input_event {
                InputEventData::ReqFromUser { .. } => 7,
                InputEventData::RequestSentToGwL2 { .. } => 8,
                InputEventData::RespFromGwL2 { .. } => 9,
                InputEventData::EventLogFromGwL2 { .. } => 10,
            },
        }
    }

    fn request_id(&self) -> Uuid {
        self.request_id
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum RelayerEventData {
    // Raw event log from ethereum. Handler will check event type, decode the
    // event, store ethereum related contextual data and dispatch a decryption
    // request event.
    EventLogFromHostL1 {
        // For ethereum handler
        // TODO: Make relayer event generic of this log type, to make it blockchain agnostic.
        event_log: Log,
    },
    // Decryption request after processing by ethereum adapter. This will be
    // picked up by gateway l2 adapter, which will send a request to decryption
    // manager contract on the gateway l2 blockchain.
    // After sending the request, it will receive a gateway_l2_request_id, which
    // it will persist in contextual data of gateway l2 adapter.
    //
    // After this system will wait until a gateway l2 listener catches a response and creates a new request.
    DecryptRequestRcvd {
        // For gateway l2 handler
        ct_handles: Vec<[u8; 32]>,
        operation: DecryptionType,
    },

    DecryptionRequestSentToGwL2 {
        decryption_public_id: U256,
    },

    // Raw event log from gateway l2. Will be processed by gateway l2 handler.
    // Handler will check the event type and decode the event. After decoding,
    // it will check if gateway_l2_request_id is available in the contextual
    // data store of the handler. if not, drops the request (not meant for this
    // relayer instance). if found, creates the next event with the original
    // orchestrator request id retreived from contextual data store.
    EventLogFromGwL2 {
        // For gateway l2 handler
        log: Log,
    },
    DecryptionResponseRcvdFromGwL2 {
        // For ethereum handler
        decrypted_value: DecryptedValue,
    },
    // This event data could be used to update the dashboard.
    DecryptResponseSentToHostL1,
    // For no handler, just status update.
    DecryptionFailed {
        // For no handler, just status updated.
        error: String,
    },

    Input(InputEventData),
}

impl AsRef<str> for RelayerEventData {
    fn as_ref(&self) -> &str {
        match self {
            RelayerEventData::EventLogFromHostL1 { .. } => "EventLogFromHostL1",
            RelayerEventData::DecryptRequestRcvd { .. } => "DecryptRequestRcvd",
            RelayerEventData::DecryptionRequestSentToGwL2 { .. } => "DecryptionRequestSentToGwL2",
            RelayerEventData::EventLogFromGwL2 { .. } => "EventLogFromGwL2",
            RelayerEventData::DecryptionResponseRcvdFromGwL2 { .. } => {
                "DecryptionResponseRcvdFromGwL2"
            }
            RelayerEventData::DecryptResponseSentToHostL1 => "DecryptResponseSentToHostL1",
            RelayerEventData::DecryptionFailed { .. } => "DecryptionFailed",
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
pub enum InputEventData {
    ReqFromUser {
        contract_chain_id: U256,
        contract_address: Address,
        user_address: Address,
        zkpok: Vec<u8>,
    },
    RespFromGwL2 {
        handles: Vec<[u8; 32]>,
        signatures: Vec<Vec<u8>>,
    },
    RequestSentToGwL2 {
        zkpok_public_id: U256,
    },
    EventLogFromGwL2 {
        log: Log,
    },
}

impl InputEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            InputEventData::ReqFromUser { .. } => "Input::ReqFromUser",
            InputEventData::RespFromGwL2 { .. } => "Input::RespFromGwL2",
            InputEventData::RequestSentToGwL2 { .. } => "Input::RequestSentToGwL2",
            InputEventData::EventLogFromGwL2 { .. } => "Input::EventLogFromGwL2",
        }
    }
}
