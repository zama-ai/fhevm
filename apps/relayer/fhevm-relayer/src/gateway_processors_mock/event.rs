use crate::orchestrator::traits::Event;

use alloy::rpc::types::Log;
use std::fmt::Display;
use strum_macros::Display;
use uuid::Uuid;

#[derive(Clone)]
pub struct GatewayProcessorsEvent {
    pub request_id: Uuid,
    pub api_version: ApiVersion,
    pub data: GatewayProcessorsEventData,
}

impl GatewayProcessorsEvent {
    pub fn new(
        request_id: Uuid,
        api_version: ApiVersion,
        data: GatewayProcessorsEventData,
    ) -> GatewayProcessorsEvent {
        GatewayProcessorsEvent {
            request_id,
            api_version,
            data,
        }
    }

    pub fn derive_next_event(
        self,
        next_event_data: GatewayProcessorsEventData,
    ) -> GatewayProcessorsEvent {
        GatewayProcessorsEvent {
            request_id: self.request_id,
            api_version: self.api_version,
            data: next_event_data,
        }
    }
}

impl Event for GatewayProcessorsEvent {
    fn event_name(&self) -> &str {
        self.data.as_ref()
    }

    //TODO: Replace boiler plate with macro based code.
    fn event_id(&self) -> u8 {
        match &self.data {
            GatewayProcessorsEventData::EventLogFromGwL2 { .. } => 3,
            GatewayProcessorsEventData::KmsInput(input_event) => match input_event {
                GatewayProcessorsInputEventData::EventLogRequestFromGwL2 { .. } => 11,
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
pub enum GatewayProcessorsEventData {
    EventLogFromGwL2 {
        // For gateway l2 handler
        log: Log,
    },
    KmsInput(GatewayProcessorsInputEventData),
}

impl AsRef<str> for GatewayProcessorsEventData {
    fn as_ref(&self) -> &str {
        match self {
            GatewayProcessorsEventData::EventLogFromGwL2 { .. } => "EventLogFromGwL2",
            GatewayProcessorsEventData::KmsInput(input_event) => input_event.event_name(),
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
pub enum GatewayProcessorsInputEventData {
    EventLogRequestFromGwL2 { log: Log },
}

impl GatewayProcessorsInputEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            GatewayProcessorsInputEventData::EventLogRequestFromGwL2 { .. } => {
                "KmsInput::EventLogRequestFromGwL2"
            }
        }
    }
}
