//! User decrypt: every node returns a distinct signcrypted share; the SDK reconstructs from `threshold` of them.

use alloy::primitives::B256;
use kms_connector_api::{USER_DECRYPTION_ROUTE, UserDecryptionRequest, UserDecryptionResponse};
use serde::Serialize;

use super::{Flow, RejectReason, check_signature, hex};
use crate::kms_aggregator::config::UserChecks;

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
    type Checks = UserChecks;
    const NAME: &'static str = "user_decrypt";
    const ROUTE: &'static str = USER_DECRYPTION_ROUTE;

    fn decryption_id(request: &Self::Request) -> B256 {
        request.id()
    }

    fn handles(request: &Self::Request) -> Vec<B256> {
        request.payload.handles.iter().map(|h| h.handle).collect()
    }

    fn check(
        checks: &UserChecks,
        request: &Self::Request,
        accepted: &[Self::Response],
        response: &Self::Response,
    ) -> Result<(), RejectReason> {
        if checks.decryption_id_match && response.decryption_id != request.id() {
            return Err(RejectReason::IdMismatch);
        }
        check_signature(
            accepted.iter().map(|a| a.signature.as_ref()),
            &response.signature,
        )
    }

    /// Every accepted share counts; with `decryption_id_majority`, only the largest group of identical ids.
    fn counted(checks: &UserChecks, accepted: &[Self::Response]) -> usize {
        match majority_id(checks, accepted) {
            Some(id) => accepted.iter().filter(|r| r.decryption_id == id).count(),
            None => accepted.len(),
        }
    }

    /// Acceptance order = arrival order; with `decryption_id_majority`, only the majority group's shares.
    fn output(checks: &UserChecks, accepted: Vec<Self::Response>) -> Option<Self::Output> {
        let keep = majority_id(checks, &accepted);
        let result = accepted
            .into_iter()
            .filter(|r| keep.is_none_or(|id| r.decryption_id == id))
            .map(|r| UserDecryptShare {
                payload: hex(&r.user_decrypted_shares),
                signature: hex(&r.signature),
                extra_data: r.extra_data.to_string(),
            })
            .collect();
        Some(UserDecryptOutput { result })
    }
}

/// The most frequent `decryptionId` when the majority check is on (ties: the one seen last); `None` otherwise.
fn majority_id(checks: &UserChecks, accepted: &[UserDecryptionResponse]) -> Option<B256> {
    if !checks.decryption_id_majority {
        return None;
    }
    accepted
        .iter()
        .map(|r| r.decryption_id)
        .max_by_key(|id| accepted.iter().filter(|r| r.decryption_id == *id).count())
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Bytes, b256};
    use serde_json::json;

    use super::*;
    use crate::kms_aggregator::mock::{self, USER_REQUEST_ID, USER_REQUEST_JSON};

    const OFF: UserChecks = UserChecks {
        decryption_id_match: false,
        decryption_id_majority: false,
    };
    const MATCH: UserChecks = UserChecks {
        decryption_id_match: true,
        decryption_id_majority: false,
    };
    const MAJORITY: UserChecks = UserChecks {
        decryption_id_match: false,
        decryption_id_majority: true,
    };

    fn request() -> UserDecryptionRequest {
        serde_json::from_str(USER_REQUEST_JSON).unwrap()
    }

    fn response(node: usize) -> UserDecryptionResponse {
        UserDecryptionResponse {
            decryption_id: USER_REQUEST_ID,
            user_decrypted_shares: mock::share(node),
            signature: mock::signature(node),
            extra_data: Bytes::from_static(&[0]),
        }
    }

    fn wrong_id(node: usize) -> UserDecryptionResponse {
        UserDecryptionResponse {
            decryption_id: B256::repeat_byte(0xEE),
            ..response(node)
        }
    }

    #[test]
    fn id_and_handles_come_from_the_request() {
        assert_eq!(UserDecrypt::decryption_id(&request()), USER_REQUEST_ID);
        assert_eq!(
            UserDecrypt::handles(&request()),
            vec![b256!(
                "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            )]
        );
    }

    #[test]
    fn check_accepts_distinct_shares_and_rejects_bad_or_duplicate_signatures() {
        let accepted = vec![response(0), response(1)];
        let check = |r| UserDecrypt::check(&OFF, &request(), &accepted, &r);
        assert_eq!(check(response(2)), Ok(()));
        assert_eq!(check(response(1)), Err(RejectReason::Duplicate));
        let mut short = response(3);
        short.signature = Bytes::copy_from_slice(&mock::signature(3)[..64]);
        assert_eq!(check(short), Err(RejectReason::BadSignature(64)));
    }

    #[test]
    fn decryption_id_is_checked_only_when_configured() {
        let accepted = vec![response(0)];
        assert_eq!(
            UserDecrypt::check(&OFF, &request(), &accepted, &wrong_id(4)),
            Ok(())
        );
        assert_eq!(
            UserDecrypt::check(&MATCH, &request(), &accepted, &wrong_id(4)),
            Err(RejectReason::IdMismatch)
        );
        assert_eq!(
            UserDecrypt::check(&MATCH, &request(), &accepted, &response(4)),
            Ok(())
        );
    }

    #[test]
    fn every_share_counts_unless_majority_is_on() {
        let mixed = vec![
            response(0),
            wrong_id(1),
            response(2),
            wrong_id(3),
            response(4),
        ];
        assert_eq!(UserDecrypt::counted(&OFF, &[]), 0);
        assert_eq!(UserDecrypt::counted(&OFF, &mixed), 5);
        assert_eq!(UserDecrypt::counted(&MAJORITY, &[]), 0);
        assert_eq!(UserDecrypt::counted(&MAJORITY, &mixed), 3);
        // A tie counts the size of one group.
        assert_eq!(
            UserDecrypt::counted(&MAJORITY, &[response(0), wrong_id(1)]),
            1
        );
    }

    #[test]
    fn output_matches_the_current_relayer_shape() {
        let output = UserDecrypt::output(&OFF, vec![response(0), response(1)]).unwrap();
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
            UserDecrypt::output(&OFF, vec![]).unwrap(),
            UserDecryptOutput { result: vec![] }
        );
    }

    #[test]
    fn output_keeps_only_the_majority_group_when_configured() {
        let mixed = vec![response(0), wrong_id(1), response(2)];
        assert_eq!(
            UserDecrypt::output(&OFF, mixed.clone())
                .unwrap()
                .result
                .len(),
            3
        );
        let majority = UserDecrypt::output(&MAJORITY, mixed).unwrap();
        assert_eq!(majority.result.len(), 2);
        assert_eq!(majority.result[1].payload, hex(&mock::share(2)));
        assert_eq!(
            UserDecrypt::output(&MAJORITY, vec![]).unwrap(),
            UserDecryptOutput { result: vec![] }
        );
    }
}
