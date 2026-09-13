//! User decrypt: every node returns a distinct signcrypted share; the SDK reconstructs from `threshold` of them.

use alloy::primitives::B256;
use kms_connector_api::{USER_DECRYPTION_ROUTE, UserDecryptionRequest, UserDecryptionResponse};
use serde::Serialize;

use super::{Flow, RejectReason, check_signature, hex};

pub struct UserDecrypt;

/// `{ "result": [ { "payload", "signature", "extraData" } ] }`: the current relayer's user-decrypt result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserDecryptOutput {
    pub result: Vec<UserDecryptShare>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptShare {
    /// Signcrypted share, hex without `0x`.
    pub payload: String,
    /// KMS signature over the share, hex without `0x`.
    pub signature: String,
    /// `0x`-prefixed hex.
    pub extra_data: String,
}

impl Flow for UserDecrypt {
    type Request = UserDecryptionRequest;
    type Response = UserDecryptionResponse;
    type Output = UserDecryptOutput;
    const NAME: &'static str = "user_decrypt";
    const ROUTE: &'static str = USER_DECRYPTION_ROUTE;

    fn decryption_id(request: &Self::Request) -> B256 {
        request.id()
    }

    fn handles(request: &Self::Request) -> Vec<B256> {
        request.handles.iter().map(|h| h.handle).collect()
    }

    fn check(accepted: &[Self::Response], response: &Self::Response) -> Result<(), RejectReason> {
        check_signature(
            accepted.iter().map(|a| a.signature.as_ref()),
            &response.signature,
        )
    }

    /// Every accepted share counts (shares differ by design, as on the gateway).
    fn counted(accepted: &[Self::Response]) -> usize {
        accepted.len()
    }

    /// Acceptance order = arrival order, which is the share index the gateway assigned.
    fn output(accepted: Vec<Self::Response>) -> Option<Self::Output> {
        let result = accepted
            .into_iter()
            .map(|r| UserDecryptShare {
                payload: hex(&r.user_decrypted_shares),
                signature: hex(&r.signature),
                extra_data: r.extra_data.to_string(),
            })
            .collect();
        Some(UserDecryptOutput { result })
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Bytes, b256};
    use serde_json::json;

    use super::*;
    use crate::kms_aggregator::mock::{self, USER_REQUEST_ID, USER_REQUEST_JSON};

    fn response(node: usize) -> UserDecryptionResponse {
        UserDecryptionResponse {
            decryption_id: USER_REQUEST_ID,
            user_decrypted_shares: mock::share(node),
            signature: mock::signature(node),
            extra_data: Bytes::from_static(&[0]),
        }
    }

    #[test]
    fn id_and_handles_come_from_the_request() {
        let request: UserDecryptionRequest = serde_json::from_str(USER_REQUEST_JSON).unwrap();
        assert_eq!(UserDecrypt::decryption_id(&request), USER_REQUEST_ID);
        assert_eq!(
            UserDecrypt::handles(&request),
            vec![b256!(
                "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            )]
        );
    }

    #[test]
    fn check_accepts_distinct_shares_and_rejects_bad_or_duplicate_signatures() {
        let accepted = vec![response(0), response(1)];
        assert_eq!(UserDecrypt::check(&accepted, &response(2)), Ok(()));
        assert_eq!(
            UserDecrypt::check(&accepted, &response(1)),
            Err(RejectReason::Duplicate)
        );
        let mut short = response(3);
        short.signature = Bytes::copy_from_slice(&mock::signature(3)[..64]);
        assert_eq!(
            UserDecrypt::check(&accepted, &short),
            Err(RejectReason::BadSignature(64))
        );
        // A wrong decryption id is a correlation problem, not a rejection.
        let mut other_id = response(4);
        other_id.decryption_id = B256::repeat_byte(0xEE);
        assert_eq!(UserDecrypt::check(&accepted, &other_id), Ok(()));
    }

    #[test]
    fn every_share_counts() {
        assert_eq!(UserDecrypt::counted(&[]), 0);
        assert_eq!(
            UserDecrypt::counted(&[response(0), response(1), response(2)]),
            3
        );
    }

    #[test]
    fn output_matches_the_current_relayer_shape() {
        let output = UserDecrypt::output(vec![response(0), response(1)]).unwrap();
        let expected = json!({
            "result": [
                { "payload": hex(&mock::share(0)), "signature": hex(&mock::signature(0)), "extraData": "0x00" },
                { "payload": hex(&mock::share(1)), "signature": hex(&mock::signature(1)), "extraData": "0x00" },
            ]
        });
        assert_eq!(serde_json::to_value(&output).unwrap(), expected);
        assert!(!output.result[0].payload.starts_with("0x"));
        assert_eq!(output.result[0].signature.len(), 130);
        assert_eq!(
            UserDecrypt::output(vec![]).unwrap(),
            UserDecryptOutput { result: vec![] }
        );
    }
}
