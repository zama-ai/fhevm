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

/// The test runner schedules tests, not loop iterations, so cases sharing one
/// test run one after another. A test per case lets them overlap instead, and
/// names the failing scenario in the report.
macro_rules! redundancy_cases {
    ($module:ident, $runner:path, [$($case:ident),+ $(,)?]) => {
        mod $module {
            use super::*;

            /// Compared against the case table by the coverage test below.
            pub const COVERED: &[&str] = &[$(stringify!($case)),+];

            $(
                #[tokio::test]
                async fn $case() {
                    $runner(stringify!($case)).await;
                }
            )+
        }
    };
}

redundancy_cases!(
    user_decrypt_matrix,
    run_user_decrypt_case,
    [
        all_listeners,
        first_only,
        last_only,
        two_listeners_alternate,
        three_listeners_round_robin,
        duplicate_adjacent,
        duplicate_non_adjacent,
        broadcast_then_single,
        broadcast_then_duplicate,
        duplicate_then_alternate,
        request_level_alternate,
    ]
);

redundancy_cases!(
    public_decrypt_matrix,
    run_public_decrypt_case,
    [
        all_listeners,
        first_only,
        last_only,
        two_listeners_alternate,
        three_listeners_round_robin,
        duplicate_adjacent,
        duplicate_non_adjacent,
        broadcast_then_single,
        broadcast_then_duplicate,
        duplicate_then_alternate,
    ]
);

redundancy_cases!(
    input_proof_matrix,
    run_input_proof_case,
    [
        all_listeners,
        first_only,
        last_only,
        two_listeners_alternate,
        three_listeners_round_robin,
        duplicate_adjacent,
        duplicate_non_adjacent,
        broadcast_then_single,
        broadcast_then_duplicate,
        duplicate_then_alternate,
    ]
);

fn user_decrypt_table() -> Vec<RedundancyCase> {
    let mut table = common_redundancy_cases();
    table.extend(user_only_redundancy_cases());
    table
}

fn lookup(table: Vec<RedundancyCase>, name: &str) -> RedundancyCase {
    table
        .into_iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("redundancy case `{name}` is not in the table"))
}

/// A case added to the table but not to a `redundancy_cases!` list would never
/// run, which is the hole this guards.
fn assert_every_case_covered(flow: &str, covered: &[&str], table: &[RedundancyCase]) {
    for case in table {
        assert!(
            covered.contains(&case.name),
            "{flow}: case `{}` has no test. Add it to the redundancy_cases! list.",
            case.name
        );
    }
    assert_eq!(
        covered.len(),
        table.len(),
        "{flow}: the generated tests and the case table disagree"
    );
}

#[test]
fn user_decrypt_runs_every_case() {
    assert_every_case_covered(
        "user-decrypt",
        user_decrypt_matrix::COVERED,
        &user_decrypt_table(),
    );
}

#[test]
fn public_decrypt_runs_every_case() {
    assert_every_case_covered(
        "public-decrypt",
        public_decrypt_matrix::COVERED,
        &common_redundancy_cases(),
    );
}

#[test]
fn input_proof_runs_every_case() {
    assert_every_case_covered(
        "input-proof",
        input_proof_matrix::COVERED,
        &common_redundancy_cases(),
    );
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
async fn run_user_decrypt_case(name: &str) {
    let case = lookup(user_decrypt_table(), name);
    let setup = TestSetup::new_with_listeners(case.listener_count)
        .await
        .expect("Failed to create test setup with listeners");

    let user_address = user_decrypt::random_address();
    let contract_address = user_decrypt::random_address();

    for _ in 0..case.requests {
        let payload = user_decrypt::create_user_decrypt_payload(
            &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
            contract_address,
            user_address,
        );
        let handles = user_decrypt::extract_ciphertext_handles_from_user_payload(&payload);
        let per_event_targets = expand_targets(USER_DECRYPT_EVENT_COUNT, &case.targets_per_event);

        setup.fhevm_mock.on_user_decrypt_success_with_targets(
            UserDecryptKind::Direct,
            handles,
            user_address,
            per_event_targets,
        );

        // Polling to a terminal state already waits for the consensus event
        // that lands a block after the shares, so no extra sleep is needed.
        let job_id = user_decrypt::submit_request(&setup, &payload).await;
        let (status, body) = user_decrypt::poll_until_terminal(&setup, &job_id).await;

        assert_succeeded("user-decrypt", case.name, status, body.status);
        assert!(
            body.result.is_some_and(|r| !r.result.is_empty()),
            "user-decrypt redundancy case `{}`: expected shares in result",
            case.name
        );
    }

    setup.shutdown().await;
}

/// Public decrypt emits a single response event per request, so the case's
/// target list is spread across requests rather than across events.
async fn run_public_decrypt_case(name: &str) {
    let case = lookup(common_redundancy_cases(), name);
    let setup = TestSetup::new_with_listeners(case.listener_count)
        .await
        .expect("Failed to create test setup with listeners");

    for target in expand_targets(case.requests, &case.targets_per_event) {
        let payload = public_decrypt::create_public_decrypt_payload();
        let handles = public_decrypt::extract_ciphertext_handles_from_public_payload(&payload);
        let plaintext_values = public_decrypt::random_plaintext_values(handles.len());

        setup
            .fhevm_mock
            .on_public_decrypt_success(handles, plaintext_values, target);

        let job_id = public_decrypt::submit_request(&setup, &payload).await;
        let (status, body) = public_decrypt::poll_until_terminal(&setup, &job_id).await;

        assert_succeeded("public-decrypt", case.name, status, body.status);
        assert!(
            body.result.is_some(),
            "public-decrypt redundancy case `{}`: expected a result",
            case.name
        );
    }

    setup.shutdown().await;
}

/// Input proof, like public decrypt, emits one response event per request.
async fn run_input_proof_case(name: &str) {
    let case = lookup(common_redundancy_cases(), name);
    let setup = TestSetup::new_with_listeners(case.listener_count)
        .await
        .expect("Failed to create test setup with listeners");

    for target in expand_targets(case.requests, &case.targets_per_event) {
        let (payload, user_address, ciphertext_data) =
            input_proof::create_input_proof_payload(&setup);

        // One success pattern per request, carrying that request's target.
        setup
            .fhevm_mock
            .on_input_proof_success(user_address, ciphertext_data, 1, target);

        let job_id = input_proof::submit_request(&setup, &payload).await;
        let (status, body) = input_proof::poll_until_terminal(&setup, &job_id).await;

        assert_succeeded("input-proof", case.name, status, body.status);
        assert!(
            body.result.is_some(),
            "input-proof redundancy case `{}`: expected a result",
            case.name
        );
    }

    setup.shutdown().await;
}
