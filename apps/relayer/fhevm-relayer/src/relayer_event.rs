use super::traits::Event;
use std::fmt::Display;
use strum_macros::{AsRefStr, Display};
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
}

impl Event for RelayerEvent {
    fn event_name(&self) -> &str {
        self.data.as_ref()
    }

    //TODO: Replace boiler plate with macro based code.
    fn event_id(&self) -> u8 {
        match &self.data {
            RelayerEventData::DecryptionRequestReceived { .. } => 0,
            RelayerEventData::HttpzRequestSent { .. } => 1,
            RelayerEventData::HttpzResponseReceived { .. } => 2,
            RelayerEventData::DecryptionResultSent { .. } => 3,
            RelayerEventData::DecryptionFailed { .. } => 4,
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

#[derive(Clone, AsRefStr)]
pub enum RelayerEventData {
    DecryptionRequestReceived {
        ct_handle: String,
        operation: DecryptionType,
    },
    HttpzRequestSent {
        ct_handle: String,
        operation: DecryptionType,
    },
    HttpzResponseReceived {
        decrypted_value: DecryptedValue,
    },
    DecryptionResultSent {
        host_l1_tx_id: String,
    },

    DecryptionFailed {
        error: String,
    },
}

#[derive(Clone, Debug, Display)]
pub enum DecryptionType {
    PublicDecrypt,
    UserDecrypt { user_public_key: Vec<u8> },
}

#[derive(Clone)]
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
