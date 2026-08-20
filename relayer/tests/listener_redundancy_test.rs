//! Listener-redundancy integration tests.
//!
//! These do not test any single flow's behaviour — they test the infrastructure
//! every flow shares: a pool of redundant blockchain listeners must converge on
//! the same result no matter how the gateway's response events are spread
//! across the listeners. Some listeners see every event, some see none, some
//! see duplicates; the relayer must deduplicate and still reach a terminal
//! `succeeded` state.
//!
//! The scenario table lives in `common::redundancy`; the per-flow request
//! plumbing lives in `common::flows`. This suite only wires the two together,
//! which is why it is a standalone test binary rather than three tests scattered
//! across the flow suites.
//!
//! Only the v2 endpoints are exercised for user-decrypt. v3 differs solely in
//! the request event emitted inside the transaction receipt
//! (`UserDecryptionRequest_1` vs `_0`); the ten `UserDecryptionResponse` events
//! that the listeners actually subscribe to are byte-identical, so a v3 matrix
//! would re-test the same listener path.

mod common;

use crate::common::flows::{input_proof, public_decrypt, user_decrypt};
use crate::common::redundancy::{
    common_redundancy_cases, expand_targets, user_only_redundancy_cases, RedundancyCase,
    USER_DECRYPT_EVENT_COUNT,
};
use crate::common::utils::TestSetup;
use ethereum_rpc_mock::fhevm::UserDecryptKind;
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Setups are keyed by listener count and reused across cases: spinning up a
/// relayer plus its mocks is by far the most expensive part of the matrix.
struct SetupPool {
    setups: HashMap<usize, TestSetup>,
}

impl SetupPool {
    fn new() -> Self {
        Self {
            setups: HashMap::new(),
        }
    }

    async fn get(&mut self, listener_count: usize) -> &TestSetup {
        if let Entry::Vacant(entry) = self.setups.entry(listener_count) {
            let setup = TestSetup::new_with_listeners(listener_count)
                .await
                .expect("Failed to create test setup with listeners");
            entry.insert(setup);
        }
        self.setups
            .get(&listener_count)
            .expect("Missing test setup for listener count")
    }

    async fn shutdown(self) {
        for setup in self.setups.into_values() {
            setup.shutdown().await;
        }
    }
}

fn assert_succeeded(flow: &str, case: &str, status: reqwest::StatusCode, body: ApiResponseStatus) {
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "{flow} redundancy case `{case}`: expected terminal 200"
    );
    assert_eq!(
        body,
        ApiResponseStatus::Succeeded,
        "{flow} redundancy case `{case}`: expected succeeded"
    );
}

/// User decrypt: the ten response events (3+3+3+1 blocks) are spread across the
/// listener pool per the case's target list, so a single request exercises
/// intra-request deduplication as well as the cross-listener kind.
#[tokio::test]
async fn test_listener_redundancy_user_decrypt_matrix() {
    let mut cases = common_redundancy_cases();
    cases.extend(user_only_redundancy_cases());
    let mut pool = SetupPool::new();

    for case in cases {
        let setup = pool.get(case.listener_count).await;
        let user_address = user_decrypt::random_address();
        let contract_address = user_decrypt::random_address();
        println!("user-decrypt redundancy case: {}", case.name);

        for _ in 0..case.requests {
            let payload = user_decrypt::create_user_decrypt_payload(
                &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
                contract_address,
                user_address,
            );
            let handles = user_decrypt::extract_ciphertext_handles_from_user_payload(&payload);
            let per_event_targets =
                expand_targets(USER_DECRYPT_EVENT_COUNT, &case.targets_per_event);

            setup.fhevm_mock.on_user_decrypt_success_with_targets(
                UserDecryptKind::Direct,
                handles,
                user_address,
                per_event_targets,
            );

            // Polling to a terminal state already waits for the consensus event
            // that lands a block after the shares, so no extra sleep is needed.
            let job_id = user_decrypt::submit_request(setup, &payload).await;
            let (status, body) = user_decrypt::poll_until_terminal(setup, &job_id).await;

            assert_succeeded("user-decrypt", case.name, status, body.status);
            assert!(
                body.result.is_some_and(|r| !r.result.is_empty()),
                "user-decrypt redundancy case `{}`: expected shares in result",
                case.name
            );
        }
    }

    pool.shutdown().await;
}

/// Public decrypt emits a single response event per request, so the case's
/// target list is spread across requests rather than across events.
#[tokio::test]
async fn test_listener_redundancy_public_decrypt_matrix() {
    let cases: Vec<RedundancyCase> = common_redundancy_cases();
    let mut pool = SetupPool::new();

    for case in cases {
        let setup = pool.get(case.listener_count).await;
        println!("public-decrypt redundancy case: {}", case.name);

        for target in expand_targets(case.requests, &case.targets_per_event) {
            let payload = public_decrypt::create_public_decrypt_payload();
            let handles = public_decrypt::extract_ciphertext_handles_from_public_payload(&payload);
            let plaintext_values = public_decrypt::random_plaintext_values(handles.len());

            setup
                .fhevm_mock
                .on_public_decrypt_success(handles, plaintext_values, target);

            let job_id = public_decrypt::submit_request(setup, &payload).await;
            let (status, body) = public_decrypt::poll_until_terminal(setup, &job_id).await;

            assert_succeeded("public-decrypt", case.name, status, body.status);
            assert!(
                body.result.is_some(),
                "public-decrypt redundancy case `{}`: expected a result",
                case.name
            );
        }
    }

    pool.shutdown().await;
}

/// Input proof, like public decrypt, emits one response event per request.
#[tokio::test]
async fn test_listener_redundancy_input_proof_matrix() {
    let cases: Vec<RedundancyCase> = common_redundancy_cases();
    let mut pool = SetupPool::new();

    for case in cases {
        let setup = pool.get(case.listener_count).await;
        println!("input-proof redundancy case: {}", case.name);

        for target in expand_targets(case.requests, &case.targets_per_event) {
            let (payload, user_address, ciphertext_data) =
                input_proof::create_input_proof_payload(setup);

            // One success pattern per request, carrying that request's target.
            setup
                .fhevm_mock
                .on_input_proof_success(user_address, ciphertext_data, 1, target);

            let job_id = input_proof::submit_request(setup, &payload).await;
            let (status, body) = input_proof::poll_until_terminal(setup, &job_id).await;

            assert_succeeded("input-proof", case.name, status, body.status);
            assert!(
                body.result.is_some(),
                "input-proof redundancy case `{}`: expected a result",
                case.name
            );
        }
    }

    pool.shutdown().await;
}
