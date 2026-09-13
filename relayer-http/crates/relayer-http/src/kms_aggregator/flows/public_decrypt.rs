//! Public decrypt: every node returns the same plaintext. The gateway counted signatures per identical
//! `(decryptedResult, extraData)` digest and emitted the winning group's signatures; same rule here.

use alloy::primitives::B256;
use kms_connector_api::{
    PUBLIC_DECRYPTION_ROUTE, PublicDecryptionRequest, PublicDecryptionResponse,
};
use serde::Serialize;

use super::{Flow, RejectReason, check_signature, hex};

pub struct PublicDecrypt;

/// `{ "decryptedValue", "signatures", "extraData" }`: the current relayer's public-decrypt result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptOutput {
    /// ABI-encoded plaintexts, hex without `0x`.
    pub decrypted_value: String,
    /// KMS signatures of the agreeing group only, hex without `0x`.
    pub signatures: Vec<String>,
    /// `0x`-prefixed hex.
    pub extra_data: String,
}

fn agree(a: &PublicDecryptionResponse, b: &PublicDecryptionResponse) -> bool {
    a.decrypted_result == b.decrypted_result && a.extra_data == b.extra_data
}

fn group_size(accepted: &[PublicDecryptionResponse], response: &PublicDecryptionResponse) -> usize {
    accepted.iter().filter(|a| agree(a, response)).count()
}

impl Flow for PublicDecrypt {
    type Request = PublicDecryptionRequest;
    type Response = PublicDecryptionResponse;
    type Output = PublicDecryptOutput;
    const NAME: &'static str = "public_decrypt";
    const ROUTE: &'static str = PUBLIC_DECRYPTION_ROUTE;

    fn decryption_id(request: &Self::Request) -> B256 {
        request.id()
    }

    fn handles(request: &Self::Request) -> Vec<B256> {
        request.ctHandles.clone()
    }

    fn check(accepted: &[Self::Response], response: &Self::Response) -> Result<(), RejectReason> {
        check_signature(
            accepted.iter().map(|a| a.signature.as_ref()),
            &response.signature,
        )
    }

    /// Only identical `(decryptedResult, extraData)` answers count: the size of the largest agreeing group.
    fn counted(accepted: &[Self::Response]) -> usize {
        accepted
            .iter()
            .map(|r| group_size(accepted, r))
            .max()
            .unwrap_or(0)
    }

    /// The largest group's result and its signatures only (a signature over other bytes is useless to a verifier).
    fn output(accepted: Vec<Self::Response>) -> Option<Self::Output> {
        let winner = accepted
            .iter()
            .max_by_key(|r| group_size(&accepted, r))?
            .clone();
        let signatures = accepted
            .iter()
            .filter(|a| agree(a, &winner))
            .map(|a| hex(&a.signature))
            .collect();
        Some(PublicDecryptOutput {
            decrypted_value: hex(&winner.decrypted_result),
            signatures,
            extra_data: winner.extra_data.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Bytes, b256};
    use serde_json::json;

    use super::*;
    use crate::kms_aggregator::mock::{self, PUBLIC_REQUEST_ID, PUBLIC_REQUEST_JSON};

    fn response(node: usize, variant: u8) -> PublicDecryptionResponse {
        PublicDecryptionResponse {
            decryption_id: PUBLIC_REQUEST_ID,
            decrypted_result: mock::result(variant),
            signature: mock::signature(node),
            extra_data: Bytes::from_static(&[0]),
        }
    }

    #[test]
    fn id_and_handles_come_from_the_request() {
        let request: PublicDecryptionRequest = serde_json::from_str(PUBLIC_REQUEST_JSON).unwrap();
        assert_eq!(PublicDecrypt::decryption_id(&request), PUBLIC_REQUEST_ID);
        assert_eq!(
            PublicDecrypt::handles(&request),
            vec![
                b256!("0x1111111111111111111111111111111111111111111111111111111111111111"),
                b256!("0x2222222222222222222222222222222222222222222222222222222222222222"),
            ]
        );
    }

    #[test]
    fn check_rejects_bad_or_duplicate_signatures_only() {
        let accepted = vec![response(0, 0)];
        assert_eq!(PublicDecrypt::check(&accepted, &response(1, 0)), Ok(()));
        // A divergent result is accepted (it just lands in another group).
        assert_eq!(PublicDecrypt::check(&accepted, &response(2, 1)), Ok(()));
        assert_eq!(
            PublicDecrypt::check(&accepted, &response(0, 1)),
            Err(RejectReason::Duplicate)
        );
        let mut short = response(3, 0);
        short.signature = Bytes::copy_from_slice(&mock::signature(3)[..64]);
        assert_eq!(
            PublicDecrypt::check(&accepted, &short),
            Err(RejectReason::BadSignature(64))
        );
    }

    #[test]
    fn counted_is_the_largest_agreeing_group() {
        assert_eq!(PublicDecrypt::counted(&[]), 0);
        let split = vec![
            response(0, 0),
            response(1, 1),
            response(2, 0),
            response(3, 0),
            response(4, 1),
        ];
        assert_eq!(PublicDecrypt::counted(&split), 3);
        // Different extraData = different group.
        let mut other_extra = response(5, 0);
        other_extra.extra_data = Bytes::from_static(&[1]);
        assert_eq!(PublicDecrypt::counted(&[response(0, 0), other_extra]), 1);
        // Ties: the size is the size.
        assert_eq!(PublicDecrypt::counted(&[response(0, 0), response(1, 1)]), 1);
    }

    #[test]
    fn output_is_the_winning_group_in_the_current_relayer_shape() {
        let accepted = vec![response(0, 0), response(1, 1), response(2, 0)];
        let output = PublicDecrypt::output(accepted).unwrap();
        let expected = json!({
            "decryptedValue": hex(&mock::result(0)),
            "signatures": [hex(&mock::signature(0)), hex(&mock::signature(2))],
            "extraData": "0x00",
        });
        assert_eq!(serde_json::to_value(&output).unwrap(), expected);
        assert!(!output.decrypted_value.starts_with("0x"));
        assert_eq!(PublicDecrypt::output(vec![]), None);
    }
}
