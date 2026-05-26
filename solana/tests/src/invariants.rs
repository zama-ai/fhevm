//! Cross-surface consistency checks for PoC flows.

use anchor_litesvm::Signer;
use litesvm::types::{TransactionMetadata, TransactionResult};
use solana_sdk::pubkey::Pubkey;
use zama_host::AclRecord;

use crate::{
    acl::read_acl_record,
    events::{
        binary_op_events, collect_zama_host_events, count_acl_allowed_events,
        count_tfhe_host_events,
    },
    fixture::TokenFixture,
    token_account, TransferScenario,
};

/// Balance ACL records grant exactly owner + mint compute signer (PoC shape).
pub fn assert_balance_acl_subjects(record: &AclRecord, owner: Pubkey, compute_signer: Pubkey) {
    let subjects = crate::acl::record_subjects(record);
    assert!(
        subjects.contains(&owner),
        "balance ACL must allow owner {owner}"
    );
    assert!(
        subjects.contains(&compute_signer),
        "balance ACL must allow compute_signer {compute_signer}"
    );
    assert_eq!(
        subjects.len(),
        2,
        "PoC balance ACL uses owner + compute_signer only"
    );
}

pub fn assert_tfhe_event_count(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
    expected: usize,
) {
    let events = collect_zama_host_events(meta, account_keys, program_id);
    assert_eq!(
        count_tfhe_host_events(&events),
        expected,
        "unexpected TFHE event count"
    );
}

/// EVM invariant: no FHE compute graph is emitted when ACL checks fail mid-frame.
pub fn assert_no_zama_host_events_on_failure(
    result: TransactionResult,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) {
    let err = result.expect_err("transaction should fail");
    let events = collect_zama_host_events(&err.meta, account_keys, program_id);
    assert_eq!(
        count_tfhe_host_events(&events),
        0,
        "failed frame must not emit TFHE events"
    );
    assert_eq!(
        count_acl_allowed_events(&events),
        0,
        "failed frame must not emit ACL events"
    );
}

/// Transfer: TFHE event result, output ACL handle, and token account pointer agree.
pub fn assert_transfer_output_invariants(fixture: &TokenFixture, scenario: &TransferScenario) {
    let events = binary_op_events(
        &scenario.meta,
        &scenario.account_keys,
        scenario.host_program_id,
    );
    assert_eq!(events.len(), 2, "expected sub + add events");
    assert_tfhe_event_count(
        &scenario.meta,
        &scenario.account_keys,
        scenario.host_program_id,
        2,
    );

    let alice_acl = read_acl_record(&fixture.svm, scenario.output.alice).expect("alice output ACL");
    let bob_acl = read_acl_record(&fixture.svm, scenario.output.bob).expect("bob output ACL");

    assert_eq!(events[0].result, alice_acl.handle, "sub event vs alice ACL");
    assert_eq!(events[1].result, bob_acl.handle, "add event vs bob ACL");
    assert_eq!(scenario.new_alice_handle, alice_acl.handle);
    assert_eq!(scenario.new_bob_handle, bob_acl.handle);

    assert_balance_acl_subjects(&alice_acl, fixture.alice.pubkey(), fixture.compute_signer);
    assert_balance_acl_subjects(&bob_acl, fixture.bob.pubkey(), fixture.compute_signer);

    let alice_token = token_account(&fixture.svm, fixture.alice_token);
    let bob_token = token_account(&fixture.svm, fixture.bob_token);
    assert_eq!(alice_token.balance_handle, alice_acl.handle);
    assert_eq!(alice_token.balance_acl_record, scenario.output.alice);
    assert_eq!(bob_token.balance_handle, bob_acl.handle);
    assert_eq!(bob_token.balance_acl_record, scenario.output.bob);
}

/// Wrap: add event result matches output ACL and token account (amount trivial-encrypt is internal).
pub fn assert_wrap_output_invariants(
    fixture: &TokenFixture,
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    output_acl: Pubkey,
) {
    let events = binary_op_events(meta, account_keys, fixture.host_program_id);
    assert_eq!(
        events.len(),
        1,
        "wrap exposes one add over balance + amount"
    );
    assert_tfhe_event_count(meta, account_keys, fixture.host_program_id, 2);

    let acl = read_acl_record(&fixture.svm, output_acl).expect("wrap output ACL");
    assert_eq!(events[0].result, acl.handle, "add event vs output ACL");
    assert_balance_acl_subjects(&acl, fixture.alice.pubkey(), fixture.compute_signer);

    let token = token_account(&fixture.svm, fixture.alice_token);
    assert_eq!(token.balance_handle, acl.handle);
    assert_eq!(token.balance_acl_record, output_acl);
}
