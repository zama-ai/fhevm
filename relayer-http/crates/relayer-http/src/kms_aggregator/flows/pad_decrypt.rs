//! Pad decrypt (RFC-030): every node returns a distinct encrypted payload of signed pad shares. The relayer cannot
//! read them (they are encrypted to the user), so it counts and returns every accepted response; the SDK checks the
//! signatures and the per-label `sessionId`, and reconstructs each pad from `2t + 1` consistent shares.

use kms_connector_api::{PAD_DECRYPTION_ROUTE, PadDecryptionRequest, PadDecryptionResponse};
use serde::Serialize;

use super::{Flow, RejectReason, check_signature};
use crate::metrics::FlowName;

pub struct PadDecrypt;

/// `{ "responses": [ { "decryptionId", "encryptedPayload", "signature" } ] }`: the nodes' responses as they are,
/// `0x` hex. Each `decryptionId` is the request's content hash that the node signed (the KMS core's `requestId`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PadDecryptOutput {
    pub responses: Vec<PadDecryptionResponse>,
}

impl Flow for PadDecrypt {
    type Request = PadDecryptionRequest;
    type Response = PadDecryptionResponse;
    type Output = PadDecryptOutput;
    /// No optional check: the `decryptionId` is not compared, the SDK verifies what each node signed.
    type Checks = ();
    const NAME: FlowName = FlowName::PadDecrypt;
    const ROUTE: &'static str = PAD_DECRYPTION_ROUTE;

    fn check(
        (): &(),
        _request: &Self::Request,
        accepted: &[Self::Response],
        response: &Self::Response,
    ) -> Result<(), RejectReason> {
        check_signature(
            accepted.iter().map(|a| a.signature.as_ref()),
            &response.signature,
        )
    }

    /// Every accepted response counts.
    fn counted((): &(), accepted: &[Self::Response]) -> usize {
        accepted.len()
    }

    /// Every accepted response, in acceptance order.
    fn output((): &(), accepted: Vec<Self::Response>) -> Option<Self::Output> {
        Some(PadDecryptOutput {
            responses: accepted,
        })
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{B256, Bytes};
    use serde_json::json;

    use super::*;
    use crate::kms_aggregator::flows::hex;
    use crate::kms_aggregator::mock::{self, PAD_REQUEST_ID, PAD_REQUEST_JSON};

    fn request() -> PadDecryptionRequest {
        serde_json::from_str(PAD_REQUEST_JSON).unwrap()
    }

    fn response(node: usize) -> PadDecryptionResponse {
        PadDecryptionResponse {
            decryption_id: PAD_REQUEST_ID,
            encrypted_payload: mock::share(node),
            signature: mock::signature(node),
        }
    }

    #[test]
    fn check_rejects_bad_or_duplicate_signatures_only() {
        let accepted = vec![response(0)];
        let check = |r| PadDecrypt::check(&(), &request(), &accepted, &r);
        assert_eq!(check(response(1)), Ok(()));
        assert_eq!(check(response(0)), Err(RejectReason::Duplicate));
        let mut short = response(2);
        short.signature = Bytes::copy_from_slice(&mock::signature(2)[..64]);
        assert_eq!(check(short), Err(RejectReason::BadSignature(64)));
        // The decryption id is never compared.
        let mut other_id = response(3);
        other_id.decryption_id = B256::repeat_byte(0xEE);
        assert_eq!(check(other_id), Ok(()));
    }

    #[test]
    fn every_response_counts_and_is_returned_as_is() {
        assert_eq!(PadDecrypt::counted(&(), &[]), 0);
        assert_eq!(PadDecrypt::counted(&(), &[response(0), response(1)]), 2);
        let output = PadDecrypt::output(&(), vec![response(1), response(0)]).unwrap();
        let expected = json!({
            "responses": [
                {
                    "decryptionId": PAD_REQUEST_ID.to_string(),
                    "encryptedPayload": format!("0x{}", hex(&mock::share(1))),
                    "signature": format!("0x{}", hex(&mock::signature(1))),
                },
                {
                    "decryptionId": PAD_REQUEST_ID.to_string(),
                    "encryptedPayload": format!("0x{}", hex(&mock::share(0))),
                    "signature": format!("0x{}", hex(&mock::signature(0))),
                },
            ]
        });
        assert_eq!(serde_json::to_value(&output).unwrap(), expected);
        assert_eq!(
            PadDecrypt::output(&(), vec![]).unwrap(),
            PadDecryptOutput { responses: vec![] }
        );
    }
}
