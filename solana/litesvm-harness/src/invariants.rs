//! Cross-surface consistency checks for PoC flows.

use litesvm::types::TransactionMetadata;
use solana_sdk::pubkey::Pubkey;

use crate::{
    acl::read_acl_record,
    events::binary_op_events,
    fixture::TokenFixture,
    token_account, TransferScenario,
};

/// Transfer: TFHE event result, output ACL handle, and token account pointer agree.
pub fn assert_transfer_output_invariants(
    fixture: &TokenFixture,
    scenario: &TransferScenario,
) {
    let events = binary_op_events(
        &scenario.meta,
        &scenario.account_keys,
        scenario.host_program_id,
    );
    assert_eq!(events.len(), 2, "expected sub + add events");

    let alice_acl = read_acl_record(&fixture.svm, scenario.output.alice)
        .expect("alice output ACL");
    let bob_acl = read_acl_record(&fixture.svm, scenario.output.bob)
        .expect("bob output ACL");

    assert_eq!(events[0].result, alice_acl.handle, "sub event vs alice ACL");
    assert_eq!(events[1].result, bob_acl.handle, "add event vs bob ACL");
    assert_eq!(scenario.new_alice_handle, alice_acl.handle);
    assert_eq!(scenario.new_bob_handle, bob_acl.handle);

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
    assert_eq!(events.len(), 1, "wrap uses one add over balance + amount");

    let acl = read_acl_record(&fixture.svm, output_acl).expect("wrap output ACL");
    assert_eq!(events[0].result, acl.handle, "add event vs output ACL");

    let token = token_account(&fixture.svm, fixture.alice_token);
    assert_eq!(token.balance_handle, acl.handle);
    assert_eq!(token.balance_acl_record, output_acl);
}
