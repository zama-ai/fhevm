//! Invalid requests are answered `400` without any DB access. The validation rules themselves are
//! unit tested in `src/core/validation.rs`.

mod common;

use crate::common::{
    AnyRequest, CHAIN_ID, PAD_REQUESTS_TABLE, PUBLIC_REQUESTS_TABLE, Route, USER_REQUESTS_TABLE,
    count_rows, error_body, rand_handle, rand_public_request, rand_user_request, setup_with,
};
use alloy::{primitives::Address, transports::http::reqwest::StatusCode};
use connector_utils::tests::rand::rand_pad_request;
use endpoint::core::Config;
use kms_connector_api::ErrorCode;
use rstest::rstest;
use std::{num::NonZeroUsize, time::Duration};

const MAX_BODY_BYTES: usize = 8192;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_invalid_requests_answer_400_without_db_access() -> anyhow::Result<()> {
    let endpoint = setup_with(|config| Config {
        max_body_bytes: MAX_BODY_BYTES,
        max_handles_per_decryption: NonZeroUsize::new(2).unwrap(),
        ..config
    })
    .await?;

    for route in [Route::Public, Route::User, Route::Pad] {
        let mut unknown_field = route.request().to_json();
        unknown_field["sneaky"] = 1.into();
        let malformed_bodies = [
            ("invalid JSON", "{not json".to_owned()),
            ("unknown field", unknown_field.to_string()),
            (
                "oversized body",
                format!(
                    "{}{}",
                    route.request().to_json(),
                    " ".repeat(MAX_BODY_BYTES)
                ),
            ),
        ];
        for (case, body) in malformed_bodies {
            let response = endpoint.post_raw(route.path(), body).await;
            assert_eq!(
                response.status(),
                StatusCode::BAD_REQUEST,
                "{route:?}: {case}"
            );
            let body = error_body(response).await;
            assert_eq!(body.code, ErrorCode::Malformed, "{route:?}: {case}");
            assert!(!body.retryable);
            assert_eq!(body.decryption_id, None);
        }
    }

    // Not exhaustive: the rules are unit tested, this only checks each route wires its validation.
    let invalid_requests: Vec<(&str, ErrorCode, AnyRequest)> = vec![
        // Also checks the endpoint config reaches the validation.
        ("too many handles", ErrorCode::Malformed, {
            let mut request = rand_public_request();
            request.ctHandles = vec![rand_handle(CHAIN_ID); 3];
            request.into()
        }),
        (
            "unsupported attestationType",
            ErrorCode::UnsupportedAttestationType,
            {
                let mut request = rand_user_request();
                request.attestationType = "random_attestation_type".to_owned();
                request.into()
            },
        ),
        ("label does not match its witness", ErrorCode::Malformed, {
            let mut request = rand_pad_request(CHAIN_ID);
            request.payload.pads[0].witness.consumer = Address::repeat_byte(0x34);
            request.into()
        }),
    ];
    for (needle, code, request) in invalid_requests {
        let route = request.route();
        let response = endpoint.post(request).await;
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "{route:?}: {needle}"
        );
        let body = error_body(response).await;
        assert_eq!(body.code, code, "{route:?}: {needle}");
        assert!(!body.retryable);
        assert!(body.message.contains(needle), "{}", body.message);
    }

    for table in [
        PUBLIC_REQUESTS_TABLE,
        USER_REQUESTS_TABLE,
        PAD_REQUESTS_TABLE,
    ] {
        assert_eq!(count_rows(&endpoint.db, table).await, 0, "{table}");
    }

    endpoint.stop().await
}
