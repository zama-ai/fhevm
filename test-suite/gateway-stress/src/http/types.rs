use alloy::primitives::B256;
use kms_connector_api::{ErrorResponse, PublicDecryptionRequest, UserDecryptionRequest};
use std::{fmt, time::Duration};

/// A decryption request body of the RFC 033 `v1` HTTP interface.
#[derive(Clone)]
pub enum HttpDecryptionRequest {
    Public(PublicDecryptionRequest),
    UserV2(UserDecryptionRequest),
}

impl HttpDecryptionRequest {
    /// The content-derived decryption id the connector will compute for this body.
    pub fn id(&self) -> B256 {
        match self {
            HttpDecryptionRequest::Public(r) => r.id(),
            HttpDecryptionRequest::UserV2(r) => r.id(),
        }
    }

    /// The number of ciphertext handles decrypted by this request.
    pub fn handle_count(&self) -> usize {
        match self {
            HttpDecryptionRequest::Public(r) => r.ctHandles.len(),
            HttpDecryptionRequest::UserV2(r) => r.payload.handles.len(),
        }
    }

    pub fn type_str(&self) -> &'static str {
        match self {
            HttpDecryptionRequest::Public(_) => "PublicDecryptionRequest",
            HttpDecryptionRequest::UserV2(_) => "UserDecryptionRequestV2",
        }
    }
}

impl fmt::Debug for HttpDecryptionRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpDecryptionRequest::{}({})",
            self.type_str(),
            self.id()
        )
    }
}

/// The outcome of one decryption request sent to one party.
#[derive(Debug)]
pub struct RequestOutcome {
    pub decryption_id: B256,
    pub http_status: u16,
    /// Time between the request being sent and its full response being received.
    pub elapsed: Duration,
    /// The number of handles decrypted, `0` on error.
    pub handle_count: usize,
    /// The decoded error body, when the party answered a non-2xx status.
    pub error: Option<ErrorResponse>,
}

impl RequestOutcome {
    pub fn is_success(&self) -> bool {
        self.http_status == 200
    }
}
