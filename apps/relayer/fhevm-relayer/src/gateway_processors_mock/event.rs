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
    pub timestamp: u64,
}

impl GatewayProcessorsEvent {
    pub fn new(
        request_id: Uuid,
        api_version: ApiVersion,
        data: GatewayProcessorsEventData,
    ) -> GatewayProcessorsEvent {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        GatewayProcessorsEvent {
            request_id,
            api_version,
            data,
            timestamp,
        }
    }

    pub fn derive_next_event(
        self,
        next_event_data: GatewayProcessorsEventData,
    ) -> GatewayProcessorsEvent {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        GatewayProcessorsEvent {
            request_id: self.request_id,
            api_version: self.api_version,
            data: next_event_data,
            timestamp,
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
            GatewayProcessorsEventData::KmsInput(input_event) => match input_event {
                GatewayProcessorsInputEventData::EventLogRequestFromGw { .. } => 5,
            },
            GatewayProcessorsEventData::UserDecrypt(user_decrypt_event) => match user_decrypt_event
            {
                UserDecryptionEventData::EventLogRequestFromGw { .. } => 4,
            },
            GatewayProcessorsEventData::PublicDecrypt(decrypt_event) => match decrypt_event {
                PublicDecryptionEventData::EventLogRequestFromGw { .. } => 3,
            },
        }
    }

    fn request_id(&self) -> Uuid {
        self.request_id
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
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
    KmsInput(GatewayProcessorsInputEventData),
    UserDecrypt(UserDecryptionEventData),
    PublicDecrypt(PublicDecryptionEventData),
}

impl AsRef<str> for GatewayProcessorsEventData {
    fn as_ref(&self) -> &str {
        match self {
            GatewayProcessorsEventData::KmsInput(input_event) => input_event.event_name(),
            GatewayProcessorsEventData::UserDecrypt(user_decrypt_event) => {
                user_decrypt_event.event_name()
            }
            GatewayProcessorsEventData::PublicDecrypt(public_decrypt_event) => {
                public_decrypt_event.event_name()
            }
        }
    }
}

#[derive(Clone, Debug, Display)]
pub enum DecryptionType {
    PublicDecrypt,
    UserDecrypt,
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
    EventLogRequestFromGw { log: Log },
}

impl GatewayProcessorsInputEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            GatewayProcessorsInputEventData::EventLogRequestFromGw { .. } => {
                "KmsInput::EventLogRequestFromGw"
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum UserDecryptionEventData {
    EventLogRequestFromGw { log: Log },
}

impl UserDecryptionEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            UserDecryptionEventData::EventLogRequestFromGw { .. } => {
                "UserDecryption::EventLogRequestFromGw"
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum PublicDecryptionEventData {
    EventLogRequestFromGw { log: Log },
}

impl PublicDecryptionEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            PublicDecryptionEventData::EventLogRequestFromGw { .. } => {
                "UserDecryption::EventLogRequestFromGw"
            }
        }
    }
}
