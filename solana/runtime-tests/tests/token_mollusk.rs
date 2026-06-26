#[allow(dead_code)]
mod support;

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, AnchorDeserialize,
    Discriminator, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
use confidential_token as token;
use confidential_token_receiver as receiver;
use mollusk_svm::{
    result::{types::TransactionResult, Check},
    Mollusk,
};
use solana_sdk::{
    account::Account,
    ed25519_program,
    instruction::{AccountMeta, Instruction},
    native_loader,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use support::fhe_runtime::{ClearValue, CleartextBackend, FheBackend, TypedClearValue};
use zama_host as host;

const DEFAULT_INPUT_NONCE_SEQUENCE: u64 = 0;
const DEFAULT_REQUEST_EXPIRES_SLOT: u64 = 1_000;
const BALANCE_FHE_TYPE: u8 = 5;

fn request_nonce(seed: u8) -> [u8; 32] {
    [seed; 32]
}

#[test]
fn mollusk_confidential_self_transfer_is_no_op() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(9, BALANCE_FHE_TYPE);
    let output = SelfTransferOutputAccounts::canonical(&fixture, 0);
    let ix = self_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    assert!(result.inner_instructions.is_empty());
    let token_account = read_token_account(&context, fixture.alice_token);
    assert_eq!(token_account.balance_handle, fixture.alice_initial);
    // Self-transfer is a no-op: balance lineage untouched, amount nonce not advanced.
    assert_eq!(token_account.next_amount_nonce_sequence, 0);
    assert_empty_system_account(&context, output.alice);
    assert_empty_system_account(&context, output.transferred);
}

#[test]
fn mollusk_confidential_self_transfer_rejects_noncanonical_unused_output_accounts() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(94, BALANCE_FHE_TYPE);
    let mut output = SelfTransferOutputAccounts::canonical(&fixture, 1);
    output.to_output = token_balance_acl_address(fixture.mint, Pubkey::new_unique(), 1);
    let ix = self_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
    seed_empty_system_account(&context, output.to_output);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    let token_account = read_token_account(&context, fixture.alice_token);
    assert_eq!(token_account.balance_handle, fixture.alice_initial);
    assert_eq!(token_account.next_amount_nonce_sequence, 0);
    assert_empty_system_account(&context, output.alice);
    assert_empty_system_account(&context, output.to_output);
}

#[test]
fn mollusk_confidential_transfer_rotates_accounts_and_acl_records() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let ix = direct_transfer_ix(&fixture, output, amount_handle);
    let mut context = fixture.context_with_input_amount(amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_balance_lineages(&context);
    let acl_records_before = acl_record_count(&context);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    assert!(!result.inner_instructions.is_empty());
    // Balances rotate the lineages in place; only the one-shot transferred-amount
    // record is minted.
    assert_eq!(acl_record_count(&context), acl_records_before + 1);
    assert_transfer_scratch_acl_records_absent(&context, &fixture, 1);
    let alice_token = read_token_account(&context, fixture.alice_token);
    let bob_token = read_token_account(&context, fixture.bob_token);
    let from_lineage =
        read_encrypted_value_acl(&context, output.from_output).expect("sender balance lineage");
    let to_lineage =
        read_encrypted_value_acl(&context, output.to_output).expect("recipient balance lineage");
    let transferred_acl = read_acl_record(&context, output.transferred);

    assert_eq!(from_lineage.current_handle, alice_token.balance_handle);
    assert_ne!(alice_token.balance_handle, fixture.alice_initial);
    assert_eq!(alice_token.next_amount_nonce_sequence, 1);
    assert_eq!(to_lineage.current_handle, bob_token.balance_handle);
    assert_ne!(bob_token.balance_handle, fixture.bob_initial);
    // The recipient's amount nonce is not advanced by a transfer (only the sender's).
    assert_eq!(bob_token.next_amount_nonce_sequence, 0);
    // The sender's balance lineage authorizes its owner + compute signer.
    assert!(from_lineage.is_subject(fixture.owner.to_bytes()));
    assert!(from_lineage.is_subject(fixture.compute_signer.to_bytes()));
    assert!(to_lineage.is_subject(fixture.bob_owner.to_bytes()));
    assert!(to_lineage.is_subject(fixture.compute_signer.to_bytes()));

    assert_acl_record(
        &transferred_acl,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::transferred_amount_label(),
        ),
        0,
        fixture.mint,
        fixture.alice_token,
        token::transferred_amount_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.bob_owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
}

#[test]
fn mollusk_confidential_transfer_allows_distinct_payer_for_output_rent() {
    let fixture = TokenMolluskFixture::new();
    let payer = Pubkey::new_unique();
    let amount_handle = handle_for_chain(29, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let ix = direct_transfer_ix_with_payer(&fixture, payer, output, amount_handle);
    let mut context = fixture.context_with_input_amount(amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_balance_lineages(&context);
    seed_account(&context, payer, system_account(5_000_000_000));
    let acl_records_before = acl_record_count(&context);

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    // The distinct payer funds the one minted record but is not an ACL subject of it.
    assert_eq!(acl_record_count(&context), acl_records_before + 1);
    let from_lineage =
        read_encrypted_value_acl(&context, output.from_output).expect("sender balance lineage");
    let alice_token = read_token_account(&context, fixture.alice_token);
    let transferred_acl = read_acl_record(&context, output.transferred);
    assert_eq!(from_lineage.current_handle, alice_token.balance_handle);
    assert_eq!(transferred_acl.app_account, fixture.alice_token);
    assert!(transferred_acl.inline_subject_index(payer).is_none());
}

#[test]
fn mollusk_confidential_transfer_rejects_payer_scoped_amount_acl() {
    let fixture = TokenMolluskFixture::new();
    let payer = Pubkey::new_unique();
    let amount_handle = handle_for_chain(30, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let ix = direct_transfer_ix_with_acls(
        &fixture,
        payer,
        amount_acl_address(fixture.mint, payer, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    );
    let mut context = fixture.context_with_input_amount_for_authority(amount_handle, payer);
    context.mollusk.sysvars.warp_to_slot(10);
    seed_account(&context, payer, system_account(5_000_000_000));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_direct_transfer_failure_preserved_state(
        &context,
        &fixture,
        fixture.alice_initial,
        0,
        fixture.bob_initial,
        0,
        output,
    );
}

#[test]
fn mollusk_confidential_transfer_replays_token_event_cpis() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(22, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let ix = direct_transfer_ix(&fixture, output, amount_handle);
    let mut context = fixture.context_with_input_amount(amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_balance_lineages(&context);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let from_lineage =
        read_encrypted_value_acl(&context, output.from_output).expect("sender balance lineage");
    let to_lineage =
        read_encrypted_value_acl(&context, output.to_output).expect("recipient balance lineage");
    let transferred_acl = read_acl_record(&context, output.transferred);
    let transfer_events: Vec<token::ConfidentialTransferEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(transfer_events.len(), 1);
    assert_eq!(transfer_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(transfer_events[0].mint, fixture.mint);
    assert_eq!(transfer_events[0].from_owner, fixture.owner);
    assert_eq!(transfer_events[0].from_token_account, fixture.alice_token);
    assert_eq!(transfer_events[0].to_owner, fixture.bob_owner);
    assert_eq!(transfer_events[0].to_token_account, fixture.bob_token);
    assert_eq!(
        transfer_events[0].transferred_handle,
        transferred_acl.handle
    );
    assert_eq!(
        transfer_events[0].transferred_acl_record,
        output.transferred
    );

    assert_eq!(balance_events.len(), 2);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::TransferDebit
    );
    assert_eq!(balance_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(balance_events[0].mint, fixture.mint);
    assert_eq!(balance_events[0].owner, fixture.owner);
    assert_eq!(balance_events[0].token_account, fixture.alice_token);
    assert_eq!(balance_events[0].old_handle, fixture.alice_initial);
    // Balance rotations no longer carry per-rotation ACL records.
    assert_eq!(balance_events[0].new_handle, from_lineage.current_handle);
    assert_eq!(
        balance_events[1].reason,
        token::BalanceHandleUpdateReason::TransferCredit
    );
    assert_eq!(balance_events[1].version, token::APP_EVENT_VERSION);
    assert_eq!(balance_events[1].mint, fixture.mint);
    assert_eq!(balance_events[1].owner, fixture.bob_owner);
    assert_eq!(balance_events[1].token_account, fixture.bob_token);
    assert_eq!(balance_events[1].old_handle, fixture.bob_initial);
    assert_eq!(balance_events[1].new_handle, to_lineage.current_handle);
}

/// A replayed transfer that reuses the already-consumed amount-nonce slot (the
/// transferred-amount record from the first transfer) must fail and leave both
/// balances untouched.
#[test]
fn mollusk_confidential_transfer_rejects_stale_amount_slot_without_creating_outputs() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(95, BALANCE_FHE_TYPE);
    let first_output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let mut context = fixture.context_with_input_amount(amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_balance_lineages(&context);
    let first_ix = direct_transfer_ix(&fixture, first_output, amount_handle);

    context.process_and_validate_instruction(&first_ix, &[Check::success()]);
    let alice_after_first = read_token_account(&context, fixture.alice_token);
    let bob_after_first = read_token_account(&context, fixture.bob_token);
    // Reuse the same (now-consumed) amount-nonce-0 transferred-amount target.
    let stale_ix = direct_transfer_ix(&fixture, first_output, amount_handle);

    let result = context.process_instruction(&stale_ix);

    assert!(result.raw_result.is_err());
    let alice_token = read_token_account(&context, fixture.alice_token);
    let bob_token = read_token_account(&context, fixture.bob_token);
    assert_eq!(alice_token.balance_handle, alice_after_first.balance_handle);
    assert_eq!(
        alice_token.next_amount_nonce_sequence,
        alice_after_first.next_amount_nonce_sequence
    );
    assert_eq!(bob_token.balance_handle, bob_after_first.balance_handle);
}

#[test]
fn mollusk_confidential_transfer_rejects_wrong_amount_acl_label_without_creating_outputs() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(96, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    let wrong_amount_acl = seed_amount_acl_with_subject_entries(
        &context,
        &fixture,
        fixture.owner,
        token::burn_amount_label(),
        amount_handle,
        &[host::AclSubjectEntry::compute(fixture.compute_signer)],
    );
    let ix = direct_transfer_ix_with_acls(
        &fixture,
        fixture.owner,
        wrong_amount_acl,
        output,
        amount_handle,
    );

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_direct_transfer_failure_preserved_state(
        &context,
        &fixture,
        fixture.alice_initial,
        0,
        fixture.bob_initial,
        0,
        output,
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_output_acl_for_wrong_token_account_without_creating_outputs(
) {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(97, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let wrong_output = DirectTransferOutputAccounts {
        from_output: output.to_output,
        to_output: output.from_output,
        ..output
    };
    let ix = direct_transfer_ix(&fixture, wrong_output, amount_handle);
    let mut context = fixture.context_with_input_amount(amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_direct_transfer_failure_preserved_state(
        &context,
        &fixture,
        fixture.alice_initial,
        0,
        fixture.bob_initial,
        0,
        wrong_output,
    );
}

#[test]
fn mollusk_request_disclose_amount_rejects_acl_without_compute_authority() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(24, BALANCE_FHE_TYPE);
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_acl = seed_amount_acl_with_subject_entries(
        &context,
        &fixture,
        fixture.owner,
        token::transfer_amount_label(),
        amount_handle,
        &[host::AclSubjectEntry::user(fixture.owner)],
    );
    seed_material_commitment_for_acl(&context, amount_acl, 123);
    let ix = request_disclose_amount_ix(&fixture, amount_acl, amount_handle);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, amount_acl);
    assert!(!record.public_decrypt);
}

#[test]
fn mollusk_request_disclose_amount_rejects_non_amount_acl_label() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(26, BALANCE_FHE_TYPE);
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_acl = seed_amount_acl_with_subject_entries(
        &context,
        &fixture,
        fixture.owner,
        token::balance_label(),
        amount_handle,
        &[
            host::AclSubjectEntry::user(fixture.owner),
            host::AclSubjectEntry::compute(fixture.compute_signer),
        ],
    );
    seed_material_commitment_for_acl(&context, amount_acl, 133);
    let ix = request_disclose_amount_ix(&fixture, amount_acl, amount_handle);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, amount_acl);
    assert!(!record.public_decrypt);
}

#[test]
fn mollusk_transfer_receiver_hook_records_same_transaction_callback_metadata() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(43, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(44, 0);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    fixture.seed_balance_lineages(&context);
    let sent_handle = predicted_transfer_sent_handle(&fixture, &context, amount_handle);
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    seed_callback_success_acl(
        &context,
        &fixture,
        callback_success_acl,
        callback_success_handle,
    );

    let receiver_data = accept_transfer_receiver_data(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    let transfer_ix = direct_transfer_ix(&fixture, output, amount_handle);
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        receiver::id(),
        receiver_data,
    );

    let result = process_transaction(&context, &[transfer_ix, hook_ix]);

    assert!(result.raw_result.is_ok());
    let sent_acl = read_acl_record(&context, output.transferred);
    assert_eq!(sent_acl.handle, sent_handle);
    let hook_record_address = token::transfer_receiver_hook_address(fixture.mint, sent_handle).0;
    let hook_record = read_transfer_receiver_hook_call(&context, hook_record_address);
    assert_eq!(hook_record.mint, fixture.mint);
    assert_eq!(hook_record.from_token_account, fixture.alice_token);
    assert_eq!(hook_record.to_token_account, fixture.bob_token);
    assert_eq!(hook_record.sent_handle, sent_handle);
    assert_eq!(hook_record.sent_acl_record, output.transferred);
    assert_eq!(hook_record.callback_success_handle, callback_success_handle);
    assert_eq!(
        hook_record.callback_success_acl_record,
        callback_success_acl
    );
    assert_eq!(hook_record.receiver_program, receiver::id());
    assert_eq!(hook_record.caller, fixture.owner);

    let transfer_events: Vec<token::ConfidentialTransferEvent> = result
        .inner_instructions
        .iter()
        .flatten()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .flatten()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(transfer_events.len(), 1);
    assert_eq!(transfer_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(transfer_events[0].mint, fixture.mint);
    assert_eq!(transfer_events[0].from_owner, fixture.owner);
    assert_eq!(transfer_events[0].from_token_account, fixture.alice_token);
    assert_eq!(transfer_events[0].to_owner, fixture.bob_owner);
    assert_eq!(transfer_events[0].to_token_account, fixture.bob_token);
    assert_eq!(transfer_events[0].transferred_handle, sent_handle);
    assert_eq!(
        transfer_events[0].transferred_acl_record,
        output.transferred
    );
    assert_eq!(balance_events.len(), 2);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::TransferDebit
    );
    assert_eq!(
        balance_events[1].reason,
        token::BalanceHandleUpdateReason::TransferCredit
    );
    // Balances are encrypted-value ACL lineages now — balance events carry no
    // per-rotation ACL record.
}

#[test]
fn mollusk_transfer_receiver_hook_is_one_shot_per_sent_handle() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(51, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(52, 0);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    fixture.seed_balance_lineages(&context);
    let sent_handle = predicted_transfer_sent_handle(&fixture, &context, amount_handle);
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    seed_callback_success_acl(
        &context,
        &fixture,
        callback_success_acl,
        callback_success_handle,
    );

    let receiver_data = accept_transfer_receiver_data(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    let transfer_ix = direct_transfer_ix(&fixture, output, amount_handle);
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        receiver::id(),
        receiver_data.clone(),
    );

    let result = process_transaction(&context, &[transfer_ix, hook_ix]);

    assert!(result.raw_result.is_ok());
    let hook_record_address = token::transfer_receiver_hook_address(fixture.mint, sent_handle).0;
    let data_before = context
        .account_store
        .borrow()
        .get(&hook_record_address)
        .expect("expected receiver hook marker")
        .data
        .clone();
    let duplicate_hook_ix = call_transfer_receiver_ix(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        receiver::id(),
        receiver_data,
    );

    let duplicate_result = process_transaction(&context, &[duplicate_hook_ix]);

    assert!(duplicate_result.raw_result.is_err());
    let data_after = context
        .account_store
        .borrow()
        .get(&hook_record_address)
        .expect("expected receiver hook marker")
        .data
        .clone();
    assert_eq!(data_after, data_before);
}

#[test]
fn mollusk_transfer_callback_prepare_requires_receiver_hook_marker() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(53, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(54, 0);
    let transfer_output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let transfer_ix = direct_transfer_ix(&fixture, transfer_output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
    fixture.seed_balance_lineages(&context);
    seed_callback_success_acl(
        &context,
        &fixture,
        callback_success_acl_address(
            fixture.mint,
            fixture.bob_owner,
            DEFAULT_INPUT_NONCE_SEQUENCE,
        ),
        callback_success_handle,
    );

    context.process_and_validate_instruction(&transfer_ix, &[Check::success()]);

    let sent_acl = read_acl_record(&context, transfer_output.transferred);
    let sent_handle = sent_acl.handle;
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let output = CallbackSettlementOutputAccounts::canonical(&fixture, sent_handle, 2, 2);
    let prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.to_output,
        transfer_output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        output,
    );

    let result = process_transaction(&context, &[prepare_ix]);

    assert!(result.raw_result.is_err());
    assert!(!transfer_callback_settlement_exists(
        &context,
        output.settlement
    ));
    assert!(!acl_record_exists(
        &context,
        callback_requested_refund_acl_address(&fixture, 2)
    ));
}

#[test]
fn mollusk_transfer_receiver_hook_rejects_standalone_prior_transfer() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(55, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(56, 0);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let transfer_ix = direct_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
    fixture.seed_balance_lineages(&context);
    seed_callback_success_acl(
        &context,
        &fixture,
        callback_success_acl_address(
            fixture.mint,
            fixture.bob_owner,
            DEFAULT_INPUT_NONCE_SEQUENCE,
        ),
        callback_success_handle,
    );

    context.process_and_validate_instruction(&transfer_ix, &[Check::success()]);

    let sent_acl = read_acl_record(&context, output.transferred);
    let sent_handle = sent_acl.handle;
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let receiver_data = accept_transfer_receiver_data(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        receiver::id(),
        receiver_data,
    );

    let result = process_transaction(&context, &[hook_ix]);

    assert!(result.raw_result.is_err());
    let hook_record_address = token::transfer_receiver_hook_address(fixture.mint, sent_handle).0;
    assert!(!transfer_receiver_hook_call_exists(
        &context,
        hook_record_address
    ));
}

#[test]
fn mollusk_transfer_receiver_hook_rejects_mismatched_callback_return() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(57, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(58, 0);
    let mut wrong_callback_success_handle = callback_success_handle;
    wrong_callback_success_handle[0] ^= 0xff;
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    fixture.seed_balance_lineages(&context);
    let sent_handle = predicted_transfer_sent_handle(&fixture, &context, amount_handle);
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    seed_callback_success_acl(
        &context,
        &fixture,
        callback_success_acl,
        callback_success_handle,
    );

    let receiver_data = accept_transfer_receiver_data(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        wrong_callback_success_handle,
    );
    let transfer_ix = direct_transfer_ix(&fixture, output, amount_handle);
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        receiver::id(),
        receiver_data,
    );

    let result = process_transaction(&context, &[transfer_ix, hook_ix]);

    assert!(result.raw_result.is_err());
    let hook_record_address = token::transfer_receiver_hook_address(fixture.mint, sent_handle).0;
    assert!(!transfer_receiver_hook_call_exists(
        &context,
        hook_record_address
    ));
}

#[test]
fn mollusk_transfer_receiver_hook_rejects_extra_accounts_for_empty_receiver_contract() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(59, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(60, 0);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    fixture.seed_balance_lineages(&context);
    let sent_handle = predicted_transfer_sent_handle(&fixture, &context, amount_handle);
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    seed_callback_success_acl(
        &context,
        &fixture,
        callback_success_acl,
        callback_success_handle,
    );

    let receiver_data = accept_transfer_receiver_data(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    let transfer_ix = direct_transfer_ix(&fixture, output, amount_handle);
    let mut hook_ix = call_transfer_receiver_ix(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        receiver::id(),
        receiver_data,
    );
    hook_ix
        .accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));

    let result = process_transaction(&context, &[transfer_ix, hook_ix]);

    assert!(result.raw_result.is_err());
    let hook_record_address = token::transfer_receiver_hook_address(fixture.mint, sent_handle).0;
    assert!(!transfer_receiver_hook_call_exists(
        &context,
        hook_record_address
    ));
}

#[test]
fn mollusk_transfer_receiver_hook_rejects_oversized_instruction_data() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(63, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(64, 0);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    fixture.seed_balance_lineages(&context);
    let sent_handle = predicted_transfer_sent_handle(&fixture, &context, amount_handle);
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    seed_callback_success_acl(
        &context,
        &fixture,
        callback_success_acl,
        callback_success_handle,
    );

    let transfer_ix = direct_transfer_ix(&fixture, output, amount_handle);
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        receiver::id(),
        vec![0; token::MAX_RECEIVER_HOOK_DATA_LEN + 1],
    );

    let result = process_transaction(&context, &[transfer_ix, hook_ix]);

    assert!(result.raw_result.is_err());
    let hook_record_address = token::transfer_receiver_hook_address(fixture.mint, sent_handle).0;
    assert!(!transfer_receiver_hook_call_exists(
        &context,
        hook_record_address
    ));
}

#[test]
fn mollusk_callback_settlement_refunds_failed_callback() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(42, 0);
    let transfer_output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let transfer_ix = direct_transfer_ix(&fixture, transfer_output, amount_handle);
    let mut context = fixture.context_with_input_amount(amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_balance_lineages(&context);

    context.process_and_validate_instruction(&transfer_ix, &[Check::success()]);

    let sent_acl = read_acl_record(&context, transfer_output.transferred);
    let sent_handle = sent_acl.handle;
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    // After the alice->bob transfer: alice amount nonce = 1 (keys finalize's
    // final-transferred record), bob amount nonce = 0 (keys prepare's refund).
    let output = CallbackSettlementOutputAccounts::canonical(&fixture, sent_handle, 1, 0);
    seed_account(
        &context,
        token::transfer_receiver_hook_address(fixture.mint, sent_handle).0,
        transfer_receiver_hook_account(
            &fixture,
            sent_handle,
            transfer_output.transferred,
            callback_success_handle,
            callback_success_acl,
        ),
    );
    seed_account(
        &context,
        callback_success_acl,
        acl_record_account(
            callback_success_handle,
            token::nonce_key(
                fixture.mint,
                fixture.bob_owner,
                token::callback_success_label(),
            ),
            DEFAULT_INPUT_NONCE_SEQUENCE,
            fixture.mint,
            fixture.bob_owner,
            token::callback_success_label(),
            &[host::AclSubjectEntry::compute(fixture.compute_signer)],
        ),
    );
    // Seed only the genuinely-new one-shot records (settlement, refund,
    // final-transferred); the balance lineages already exist from the transfer.
    seed_empty_system_account(&context, output.settlement);
    seed_empty_system_account(&context, output.refund);
    seed_empty_system_account(&context, output.final_transferred);

    let prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.to_output,
        transfer_output.transferred,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        output,
    );
    let prepare_result = context.process_and_validate_instruction(&prepare_ix, &[Check::success()]);

    let bob_token = read_token_account(&context, fixture.bob_token);
    let refund_acl = read_acl_record(&context, output.refund);
    let bob_lineage =
        read_encrypted_value_acl(&context, output.to_output).expect("recipient balance lineage");
    let prepared = read_transfer_callback_settlement(&context, output.settlement);
    let prepare_balance_events: Vec<token::BalanceHandleUpdatedEvent> = prepare_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(bob_lineage.current_handle, bob_token.balance_handle);
    // Prepare advances the recipient's amount nonce (refund record at nonce 0).
    assert_eq!(bob_token.next_amount_nonce_sequence, 1);
    assert_eq!(prepared.status, token::CALLBACK_SETTLEMENT_PREPARED);
    assert_eq!(prepared.mint, fixture.mint);
    assert_eq!(prepared.from_owner, fixture.owner);
    assert_eq!(prepared.from_token_account, fixture.alice_token);
    assert_eq!(prepared.to_owner, fixture.bob_owner);
    assert_eq!(prepared.to_token_account, fixture.bob_token);
    assert_eq!(prepared.sent_handle, sent_handle);
    assert_eq!(prepared.sent_acl_record, transfer_output.transferred);
    assert_eq!(prepared.callback_success_handle, callback_success_handle);
    assert_eq!(prepared.callback_success_acl_record, callback_success_acl);
    assert_eq!(prepared.refund_handle, refund_acl.handle);
    assert_eq!(prepared.refund_acl_record, output.refund);
    assert_eq!(prepared.to_balance_handle, bob_token.balance_handle);
    // Balance rotations no longer record a per-rotation ACL record pointer.
    assert_eq!(prepared.to_balance_acl_record, Pubkey::default());
    assert_acl_record(
        &refund_acl,
        token::nonce_key(
            fixture.mint,
            fixture.bob_token,
            token::callback_refund_amount_label(),
        ),
        0,
        fixture.mint,
        fixture.bob_token,
        token::callback_refund_amount_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.bob_owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert!(bob_lineage.is_subject(fixture.bob_owner.to_bytes()));
    assert!(bob_lineage.is_subject(fixture.compute_signer.to_bytes()));
    assert_eq!(prepare_balance_events.len(), 1);
    assert_eq!(
        prepare_balance_events[0].reason,
        token::BalanceHandleUpdateReason::TransferCallbackRefundDebit
    );
    assert_eq!(
        prepare_balance_events[0].new_handle,
        bob_token.balance_handle
    );

    let finalize_ix = finalize_transfer_callback_ix(
        &fixture,
        transfer_output.from_output,
        transfer_output.transferred,
        output,
    );
    let finalize_result =
        context.process_and_validate_instruction(&finalize_ix, &[Check::success()]);

    let alice_token = read_token_account(&context, fixture.alice_token);
    let alice_lineage =
        read_encrypted_value_acl(&context, output.from_output).expect("sender balance lineage");
    let final_transferred_acl = read_acl_record(&context, output.final_transferred);
    let finalized = read_transfer_callback_settlement(&context, output.settlement);
    let finalize_balance_events: Vec<token::BalanceHandleUpdatedEvent> = finalize_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let finalize_transfer_events: Vec<token::ConfidentialTransferEvent> = finalize_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(alice_lineage.current_handle, alice_token.balance_handle);
    // Finalize advances the sender's amount nonce (final-transferred at nonce 1).
    assert_eq!(alice_token.next_amount_nonce_sequence, 2);
    assert_eq!(finalized.status, token::CALLBACK_SETTLEMENT_FINALIZED);
    assert_eq!(finalized.from_balance_handle, alice_token.balance_handle);
    assert_eq!(finalized.from_balance_acl_record, Pubkey::default());
    assert_eq!(finalized.transferred_handle, final_transferred_acl.handle);
    assert_eq!(finalized.transferred_acl_record, output.final_transferred);
    assert!(alice_lineage.is_subject(fixture.owner.to_bytes()));
    assert!(alice_lineage.is_subject(fixture.compute_signer.to_bytes()));
    assert_acl_record(
        &final_transferred_acl,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::callback_final_transferred_label(),
        ),
        1,
        fixture.mint,
        fixture.alice_token,
        token::callback_final_transferred_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.bob_owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert_eq!(finalize_transfer_events.len(), 1);
    assert_eq!(finalize_transfer_events[0].from_owner, fixture.bob_owner);
    assert_eq!(
        finalize_transfer_events[0].from_token_account,
        fixture.bob_token
    );
    assert_eq!(finalize_transfer_events[0].to_owner, fixture.owner);
    assert_eq!(
        finalize_transfer_events[0].to_token_account,
        fixture.alice_token
    );
    assert_eq!(
        finalize_transfer_events[0].transferred_handle,
        refund_acl.handle
    );
    assert_eq!(
        finalize_transfer_events[0].transferred_acl_record,
        output.refund
    );
    assert_eq!(finalize_balance_events.len(), 1);
    assert_eq!(
        finalize_balance_events[0].reason,
        token::BalanceHandleUpdateReason::TransferCallbackRefundCredit
    );
    assert_eq!(
        finalize_balance_events[0].new_handle,
        alice_token.balance_handle
    );
}

#[test]
fn mollusk_wrap_usdc_escrows_spl_tokens_and_rotates_confidential_balance() {
    let fixture = TokenMolluskFixture::new();
    let amount = 100_000_000;
    let output = WrapOutputAccounts::canonical(&fixture, 1);
    let ix = wrap_usdc_ix(&fixture, output, amount);
    let mut context = fixture.context_with_wrap_accounts();
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_wrap_lineages(&context);

    let alice_usdc_before = read_spl_token_amount(&context, fixture.user_usdc);
    let vault_usdc_before = read_spl_token_amount(&context, fixture.vault_usdc);
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    assert_eq!(
        read_spl_token_amount(&context, fixture.user_usdc),
        alice_usdc_before - amount
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        vault_usdc_before + amount
    );

    let token_account = read_token_account(&context, fixture.alice_token);
    let mint_account = read_confidential_mint(&context, fixture.mint);
    let balance_lineage =
        read_encrypted_value_acl(&context, output.balance).expect("balance lineage created");
    let total_supply_lineage = read_encrypted_value_acl(&context, output.total_supply)
        .expect("total-supply lineage created");
    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let supply_events: Vec<token::TotalSupplyHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(balance_lineage.current_handle, token_account.balance_handle);
    // Wrap rotates the balance/supply lineages without advancing the amount nonce.
    assert_eq!(token_account.next_amount_nonce_sequence, 0);
    assert_eq!(
        total_supply_lineage.current_handle,
        mint_account.total_supply_handle
    );
    assert!(balance_lineage.is_subject(fixture.owner.to_bytes()));
    assert!(balance_lineage.is_subject(fixture.compute_signer.to_bytes()));
    assert!(total_supply_lineage.is_subject(fixture.compute_signer.to_bytes()));

    let wrap_handles = predicted_wrap_handles(
        &fixture,
        &context,
        amount,
        fixture.alice_initial,
        fixture.total_supply_initial,
        0,
        0,
    );
    assert_eq!(wrap_handles.balance, token_account.balance_handle);
    assert_eq!(wrap_handles.total_supply, mint_account.total_supply_handle);

    assert_eq!(balance_events.len(), 1);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::Wrap
    );
    assert_eq!(balance_events[0].old_handle, fixture.alice_initial);
    assert_eq!(balance_events[0].new_handle, token_account.balance_handle);
    assert_eq!(supply_events.len(), 1);
    assert_eq!(
        supply_events[0].reason,
        token::TotalSupplyUpdateReason::Wrap
    );
    assert_eq!(supply_events[0].old_handle, fixture.total_supply_initial);
    assert_eq!(
        supply_events[0].new_handle,
        mint_account.total_supply_handle
    );
}

#[test]
fn mollusk_disclose_balance_public_appends_public_decrypt_leaf() {
    let fixture = TokenMolluskFixture::new();
    let amount = 100_000_000;
    let output = WrapOutputAccounts::canonical(&fixture, 1);
    let wrap_ix = wrap_usdc_ix(&fixture, output, amount);
    let mut context = fixture.context_with_wrap_accounts();
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_wrap_lineages(&context);
    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    let before =
        read_encrypted_value_acl(&context, output.balance).expect("balance lineage created");

    let ix = disclose_balance_public_ix(&fixture, fixture.owner, output.balance);
    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let after =
        read_encrypted_value_acl(&context, output.balance).expect("balance lineage still present");

    // Marking the balance public appends exactly one public-decrypt leaf for the
    // current handle, without rotating it or changing membership.
    assert_eq!(after.leaf_count, before.leaf_count + 1);
    assert_eq!(after.current_handle, before.current_handle);
    assert!(after.is_subject(fixture.owner.to_bytes()));
    assert!(after.is_subject(fixture.compute_signer.to_bytes()));
}

#[test]
fn mollusk_disclose_balance_public_rejects_non_owner() {
    let fixture = TokenMolluskFixture::new();
    let amount = 100_000_000;
    let output = WrapOutputAccounts::canonical(&fixture, 1);
    let wrap_ix = wrap_usdc_ix(&fixture, output, amount);
    let mut context = fixture.context_with_wrap_accounts();
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_wrap_lineages(&context);
    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    let before =
        read_encrypted_value_acl(&context, output.balance).expect("balance lineage created");

    // A stranger signs `owner` but targets alice's token account: the
    // `[b"token-account", mint, owner]` seeds no longer derive alice's PDA, so the
    // disclosure is rejected and the lineage is untouched.
    let stranger = Pubkey::new_unique();
    let ix = disclose_balance_public_ix(&fixture, stranger, output.balance);
    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());

    let after =
        read_encrypted_value_acl(&context, output.balance).expect("balance lineage still present");
    assert_eq!(after.leaf_count, before.leaf_count);
    assert_eq!(after.current_handle, before.current_handle);
}

#[test]
fn mollusk_wrap_usdc_rejects_noncanonical_vault_account() {
    let fixture = TokenMolluskFixture::new();
    let amount = 100_000_000;
    let output = WrapOutputAccounts::canonical(&fixture, 1);
    let context = fixture.context_with_wrap_accounts();
    let noncanonical_vault = seed_noncanonical_vault_token_account(&context, &fixture, 100);
    let ix = wrap_usdc_ix_with_vault(&fixture, output, amount, noncanonical_vault);

    let user_usdc_before = read_spl_token_amount(&context, fixture.user_usdc);
    let canonical_vault_before = read_spl_token_amount(&context, fixture.vault_usdc);
    let noncanonical_vault_before = read_spl_token_amount(&context, noncanonical_vault);
    let token_account_before = read_token_account(&context, fixture.alice_token);
    let mint_account_before = read_confidential_mint(&context, fixture.mint);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_eq!(
        read_spl_token_amount(&context, fixture.user_usdc),
        user_usdc_before
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        canonical_vault_before
    );
    assert_eq!(
        read_spl_token_amount(&context, noncanonical_vault),
        noncanonical_vault_before
    );

    let token_account_after = read_token_account(&context, fixture.alice_token);
    assert_eq!(
        token_account_after.balance_handle,
        token_account_before.balance_handle
    );
    assert_eq!(
        token_account_after.next_amount_nonce_sequence,
        token_account_before.next_amount_nonce_sequence
    );
    let mint_account_after = read_confidential_mint(&context, fixture.mint);
    assert_eq!(
        mint_account_after.total_supply_handle,
        mint_account_before.total_supply_handle
    );
    assert_empty_system_account(&context, output.balance);
    assert_empty_system_account(&context, output.total_supply);
}

#[test]
fn mollusk_confidential_burn_after_wrap_rotates_balance_and_total_supply() {
    let fixture = TokenMolluskFixture::new();
    let wrap_amount = 100_000_000;
    let burn_amount_handle = handle_for_chain(52, BALANCE_FHE_TYPE);
    let wrap_output = WrapOutputAccounts::canonical(&fixture, 1);
    let burn_output = BurnOutputAccounts::canonical(&fixture, 0, 0);
    let mut context = fixture.context_with_wrap_and_burn_amount(burn_amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_wrap_lineages(&context);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    let wrapped_balance = read_encrypted_value_acl(&context, wrap_output.balance)
        .expect("balance lineage")
        .current_handle;
    let wrapped_supply = read_encrypted_value_acl(&context, wrap_output.total_supply)
        .expect("total-supply lineage")
        .current_handle;
    let vault_before_burn = read_spl_token_amount(&context, fixture.vault_usdc);
    let burn_ix = burn_ix(
        &fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        burn_output,
        burn_amount_handle,
    );
    let burn_result = context.process_and_validate_instruction(&burn_ix, &[Check::success()]);

    let token_account = read_token_account(&context, fixture.alice_token);
    let mint_account = read_confidential_mint(&context, fixture.mint);
    let balance_lineage = read_encrypted_value_acl(&context, burn_output.balance)
        .expect("balance lineage")
        .current_handle;
    let burned_acl = read_acl_record(&context, burn_output.burned);
    let total_supply_lineage = read_encrypted_value_acl(&context, burn_output.total_supply)
        .expect("total-supply lineage")
        .current_handle;
    let burn_events: Vec<token::ConfidentialBurnEvent> = burn_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = burn_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let supply_events: Vec<token::TotalSupplyHandleUpdatedEvent> = burn_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        vault_before_burn
    );
    assert_eq!(token_account.balance_handle, balance_lineage);
    // Burn mints the one-shot `burned` record and advances the amount nonce.
    assert_eq!(token_account.next_amount_nonce_sequence, 1);
    assert_eq!(mint_account.total_supply_handle, total_supply_lineage);

    assert_acl_record(
        &burned_acl,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::burned_amount_label(),
        ),
        0,
        fixture.mint,
        fixture.alice_token,
        token::burned_amount_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );

    let burn_handles = predicted_burn_handles(
        &fixture,
        &context,
        burn_amount_handle,
        wrapped_balance,
        wrapped_supply,
        0,
        0,
    );
    assert_eq!(burn_handles.balance, balance_lineage);
    assert_eq!(burn_handles.burned, burned_acl.handle);
    assert_eq!(burn_handles.total_supply, total_supply_lineage);

    assert_eq!(burn_events.len(), 1);
    assert_eq!(burn_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(burn_events[0].mint, fixture.mint);
    assert_eq!(burn_events[0].owner, fixture.owner);
    assert_eq!(burn_events[0].token_account, fixture.alice_token);
    assert_eq!(burn_events[0].burned_handle, burned_acl.handle);
    assert_eq!(burn_events[0].burned_acl_record, burn_output.burned);
    assert_eq!(balance_events.len(), 1);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::BurnDebit
    );
    assert_eq!(balance_events[0].old_handle, wrapped_balance);
    assert_eq!(balance_events[0].new_handle, balance_lineage);
    assert_eq!(supply_events.len(), 1);
    assert_eq!(
        supply_events[0].reason,
        token::TotalSupplyUpdateReason::Burn
    );
    assert_eq!(supply_events[0].old_handle, wrapped_supply);
    assert_eq!(supply_events[0].new_handle, total_supply_lineage);
}

#[test]
fn mollusk_confidential_burn_over_balance_burns_zero_without_underflow() {
    let fixture = TokenMolluskFixture::new();
    let wrap_amount = 100_000_000;
    let burn_amount_handle = handle_for_chain(78, BALANCE_FHE_TYPE);
    let wrap_output = WrapOutputAccounts::canonical(&fixture, 1);
    let burn_output = BurnOutputAccounts::canonical(&fixture, 0, 0);
    let mut context = fixture.context_with_wrap_and_burn_amount(burn_amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_wrap_lineages(&context);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.total_supply_initial, TypedClearValue::uint64(0));

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    let wrapped_balance = read_encrypted_value_acl(&context, wrap_output.balance)
        .expect("balance lineage")
        .current_handle;
    let wrapped_supply = read_encrypted_value_acl(&context, wrap_output.total_supply)
        .expect("total-supply lineage")
        .current_handle;
    let wrap_handles = predicted_wrap_handles(
        &fixture,
        &context,
        wrap_amount,
        fixture.alice_initial,
        fixture.total_supply_initial,
        0,
        0,
    );
    assert_eq!(wrap_handles.balance, wrapped_balance);
    assert_eq!(wrap_handles.total_supply, wrapped_supply);
    cleartext.seed_cleartext(wrapped_balance, TypedClearValue::uint64(100_000_125));
    cleartext.seed_cleartext(wrapped_supply, TypedClearValue::uint64(100_000_000));
    cleartext.seed_cleartext(wrap_handles.amount, TypedClearValue::uint64(wrap_amount));
    cleartext.seed_cleartext(burn_amount_handle, TypedClearValue::uint64(200_000_000));
    let burn_ix = burn_ix(
        &fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        burn_output,
        burn_amount_handle,
    );
    context.process_and_validate_instruction(&burn_ix, &[Check::success()]);

    let token_account = read_token_account(&context, fixture.alice_token);
    let mint_account = read_confidential_mint(&context, fixture.mint);
    let balance_lineage = read_encrypted_value_acl(&context, burn_output.balance)
        .expect("balance lineage")
        .current_handle;
    let burned_acl = read_acl_record(&context, burn_output.burned);
    let total_supply_lineage = read_encrypted_value_acl(&context, burn_output.total_supply)
        .expect("total-supply lineage")
        .current_handle;

    assert_eq!(token_account.balance_handle, balance_lineage);
    assert_eq!(token_account.next_amount_nonce_sequence, 1);
    assert_eq!(mint_account.total_supply_handle, total_supply_lineage);
    let burn_handles = predicted_burn_handles(
        &fixture,
        &context,
        burn_amount_handle,
        wrapped_balance,
        wrapped_supply,
        0,
        0,
    );
    assert_eq!(burn_handles.balance, balance_lineage);
    assert_eq!(burn_handles.burned, burned_acl.handle);
    assert_eq!(burn_handles.total_supply, total_supply_lineage);
    cleartext.seed_cleartext(
        burn_handles.success,
        TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        },
    );
    cleartext.seed_cleartext(balance_lineage, TypedClearValue::uint64(100_000_125));
    cleartext.seed_cleartext(burned_acl.handle, TypedClearValue::uint64(0));
    cleartext.seed_cleartext(total_supply_lineage, TypedClearValue::uint64(100_000_000));
    assert_eq!(
        cleartext.decrypt_cleartext(burn_handles.success),
        Some(TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        })
    );
    assert_eq!(
        cleartext.decrypt_cleartext(balance_lineage),
        Some(TypedClearValue::uint64(100_000_125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(burned_acl.handle),
        Some(TypedClearValue::uint64(0))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(total_supply_lineage),
        Some(TypedClearValue::uint64(100_000_000))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(wrapped_balance),
        Some(TypedClearValue::uint64(100_000_125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(wrapped_supply),
        Some(TypedClearValue::uint64(100_000_000))
    );
}

#[test]
fn mollusk_confidential_burn_rejects_transfer_amount_acl_label() {
    let fixture = TokenMolluskFixture::new();
    let wrap_amount = 100_000_000;
    let amount_handle = handle_for_chain(79, BALANCE_FHE_TYPE);
    let wrap_output = WrapOutputAccounts::canonical(&fixture, 1);
    let burn_output = BurnOutputAccounts::canonical(&fixture, 0, 0);
    let mut context = fixture.context_with_wrap_accounts();
    context.mollusk.sysvars.warp_to_slot(10);
    fixture.seed_wrap_lineages(&context);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);
    let wrapped_balance = read_encrypted_value_acl(&context, wrap_output.balance)
        .expect("balance lineage")
        .current_handle;
    let wrapped_supply = read_encrypted_value_acl(&context, wrap_output.total_supply)
        .expect("total-supply lineage")
        .current_handle;

    let transfer_amount_acl =
        amount_acl_address(fixture.mint, fixture.owner, DEFAULT_INPUT_NONCE_SEQUENCE);
    seed_account(
        &context,
        transfer_amount_acl,
        acl_record_account(
            amount_handle,
            token::nonce_key(fixture.mint, fixture.owner, token::transfer_amount_label()),
            DEFAULT_INPUT_NONCE_SEQUENCE,
            fixture.mint,
            fixture.owner,
            token::transfer_amount_label(),
            &[host::AclSubjectEntry::compute(fixture.compute_signer)],
        ),
    );
    // The balance/total-supply lineages already exist from the wrap; only the
    // burned-amount record would be newly minted.
    seed_empty_system_account(&context, burn_output.burned);

    let ix = burn_ix_with_amount_acl(
        &fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        transfer_amount_acl,
        burn_output,
        amount_handle,
    );
    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    // The rejected burn left both lineages and the token account untouched.
    assert_eq!(
        read_token_account(&context, fixture.alice_token).balance_handle,
        wrapped_balance
    );
    assert_eq!(
        read_encrypted_value_acl(&context, wrap_output.balance)
            .expect("balance lineage")
            .current_handle,
        wrapped_balance
    );
    assert_eq!(
        read_confidential_mint(&context, fixture.mint).total_supply_handle,
        wrapped_supply
    );
    assert!(!acl_record_exists(&context, burn_output.burned));
}

#[test]
fn mollusk_redeem_burned_amount_secp_releases_vault_with_kms_certificate() {
    let fixture = TokenMolluskFixture::new();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
    let cleartext_amount = 9;
    let burn_amount_handle = handle_for_chain(53, BALANCE_FHE_TYPE);
    let (_wrap_output, burn_output, context, burned_handle) =
        wrap_and_burn_for_redeem(&fixture, burn_amount_handle);

    // Switch the context to the gateway KMS (secp256k1) trust model: the KMS host config carries
    // the gateway verifier params + active context id, and the KMS context holds the signer set.
    seed_account(
        &context,
        fixture.host_config,
        kms_host_config_account(fixture.owner),
    );
    let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&key));
    seed_account(&context, kms_ctx, kms_ctx_account);

    seed_material_commitment_for_acl(&context, burn_output.burned, 140);
    let release_ix = request_burn_redemption_ix(&fixture, burn_output.burned, burned_handle);
    context.process_and_validate_instruction(&release_ix, &[Check::success()]);

    let redemption_record = token::burn_redemption_address(fixture.mint, burned_handle).0;
    let vault_before = read_spl_token_amount(&context, fixture.vault_usdc);
    let destination_before = read_spl_token_amount(&context, fixture.user_usdc);

    // No Ed25519 ix: the cert is verified on-chain via secp256k1_recover from the signatures arg.
    let signatures = kms_public_decrypt_signatures(&key, burned_handle, cleartext_amount);
    let redeem_ix = redeem_burned_amount_secp_ix(
        &fixture,
        burn_output.burned,
        redemption_record,
        burned_handle,
        cleartext_amount,
        fixture.vault_usdc,
        signatures,
    );
    let redeem_result = process_transaction(&context, &[redeem_ix]);

    assert!(
        redeem_result.raw_result.is_ok(),
        "redeem_secp failed: raw={:?} program={:?}",
        redeem_result.raw_result,
        redeem_result.program_result
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        vault_before - cleartext_amount
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.user_usdc),
        destination_before + cleartext_amount
    );
    let redemption = read_burn_redemption(&context, redemption_record);
    assert_eq!(redemption.burned_handle, burned_handle);
    assert_eq!(redemption.cleartext_amount, cleartext_amount);

    let redeem_events: Vec<token::BurnRedeemedEvent> = redeem_result
        .inner_instructions
        .iter()
        .flatten()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(redeem_events.len(), 1);
    assert_eq!(redeem_events[0].burned_handle, burned_handle);
    assert_eq!(redeem_events[0].cleartext_amount, cleartext_amount);
}

#[test]
fn mollusk_redeem_burned_amount_secp_rejects_unauthorized_signer() {
    let fixture = TokenMolluskFixture::new();
    let configured = k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
    let attacker = k256::ecdsa::SigningKey::from_bytes(&[0x66u8; 32].into()).unwrap();
    let cleartext_amount = 9;
    let burn_amount_handle = handle_for_chain(53, BALANCE_FHE_TYPE);
    let (_wrap_output, burn_output, context, burned_handle) =
        wrap_and_burn_for_redeem(&fixture, burn_amount_handle);

    seed_account(
        &context,
        fixture.host_config,
        kms_host_config_account(fixture.owner),
    );
    let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&configured));
    seed_account(&context, kms_ctx, kms_ctx_account);

    seed_material_commitment_for_acl(&context, burn_output.burned, 140);
    let release_ix = request_burn_redemption_ix(&fixture, burn_output.burned, burned_handle);
    context.process_and_validate_instruction(&release_ix, &[Check::success()]);

    let redemption_record = token::burn_redemption_address(fixture.mint, burned_handle).0;
    // A signature from a key outside the KMS context must be rejected.
    let signatures = kms_public_decrypt_signatures(&attacker, burned_handle, cleartext_amount);
    let redeem_ix = redeem_burned_amount_secp_ix(
        &fixture,
        burn_output.burned,
        redemption_record,
        burned_handle,
        cleartext_amount,
        fixture.vault_usdc,
        signatures,
    );
    let redeem_result = process_transaction(&context, &[redeem_ix]);
    assert!(redeem_result.raw_result.is_err());
}

#[test]
fn mollusk_confidential_token_account_rejects_wrong_bump_or_length() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(96, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let token_bump = token::token_account_address(fixture.mint, fixture.owner).1;
    let context = fixture.context_with_input_amount(amount_handle);
    seed_account(
        &context,
        fixture.alice_token,
        confidential_token_account_with_bump_and_extra(
            fixture.owner,
            fixture.mint,
            fixture.alice_initial,
            fixture.alice_current_compute_acl,
            token_bump.wrapping_add(1),
            0,
        ),
    );
    let ix = direct_transfer_ix(&fixture, output, amount_handle);

    let wrong_bump_result = context.process_instruction(&ix);

    assert!(wrong_bump_result.raw_result.is_err());
    assert_empty_system_account(&context, output.from_output);
    assert_empty_system_account(&context, output.to_output);
    assert_empty_system_account(&context, output.transferred);

    seed_account(
        &context,
        fixture.alice_token,
        confidential_token_account_with_bump_and_extra(
            fixture.owner,
            fixture.mint,
            fixture.alice_initial,
            fixture.alice_current_compute_acl,
            token_bump,
            1,
        ),
    );
    let oversized_result = context.process_instruction(&ix);

    assert!(oversized_result.raw_result.is_err());
    assert_empty_system_account(&context, output.from_output);
    assert_empty_system_account(&context, output.to_output);
    assert_empty_system_account(&context, output.transferred);
    let account = context
        .account_store
        .borrow()
        .get(&fixture.alice_token)
        .expect("token account should remain oversized")
        .clone();
    assert_eq!(
        account.data.len(),
        8 + token::ConfidentialTokenAccount::SPACE + 1
    );
}

#[test]
fn mollusk_confidential_mint_rejects_wrong_compute_signer_or_length() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(97, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 0, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    seed_account(
        &context,
        fixture.mint,
        confidential_mint_account_with_compute_signer_and_extra(&fixture, Pubkey::new_unique(), 0),
    );
    let ix = direct_transfer_ix(&fixture, output, amount_handle);

    let wrong_compute_signer_result = context.process_instruction(&ix);

    assert!(wrong_compute_signer_result.raw_result.is_err());
    assert_empty_system_account(&context, output.from_output);
    assert_empty_system_account(&context, output.to_output);
    assert_empty_system_account(&context, output.transferred);

    seed_account(
        &context,
        fixture.mint,
        confidential_mint_account_with_compute_signer_and_extra(
            &fixture,
            fixture.compute_signer,
            1,
        ),
    );
    let oversized_result = context.process_instruction(&ix);

    assert!(oversized_result.raw_result.is_err());
    assert_empty_system_account(&context, output.from_output);
    assert_empty_system_account(&context, output.to_output);
    assert_empty_system_account(&context, output.transferred);
    let account = context
        .account_store
        .borrow()
        .get(&fixture.mint)
        .expect("mint account should remain oversized")
        .clone();
    assert_eq!(account.data.len(), 8 + token::ConfidentialMint::SPACE + 1);
}

#[test]
fn mollusk_initialize_mint_creates_total_supply_acl() {
    let authority = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let underlying_mint = Pubkey::new_unique();
    let compute_signer = token::compute_signer_address(mint).0;
    let total_supply_authority = token::total_supply_authority_address(mint).0;
    let total_supply_value_acl =
        token::total_supply_value_acl_address(mint, total_supply_authority).0;
    let host_config = host::host_config_address().0;
    let mut context = mollusk().with_context(HashMap::from([
        (authority, system_account(5_000_000_000)),
        (mint, system_account(0)),
        (underlying_mint, spl_mint_account(authority, 6, 0)),
        (compute_signer, system_account(0)),
        (total_supply_authority, system_account(0)),
        (total_supply_value_acl, system_account(0)),
        (host_config, host_config_account(authority)),
        (event_authority(host::id()), system_account(0)),
        (event_authority(token::id()), system_account(0)),
    ]));
    context.mollusk.sysvars.warp_to_slot(10);
    let ix = initialize_mint_ix(
        authority,
        mint,
        underlying_mint,
        compute_signer,
        total_supply_authority,
        total_supply_value_acl,
        host_config,
    );

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_confidential_mint(&context, mint);
    let supply_lineage = read_encrypted_value_acl(&context, total_supply_value_acl)
        .expect("init created the total-supply lineage");
    let trivial_events: Vec<host::events::TrivialEncryptEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let supply_events: Vec<token::TotalSupplyHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(stored.authority, authority);
    assert_eq!(stored.acl_domain_key, mint);
    assert_eq!(stored.compute_signer, compute_signer);
    assert_eq!(stored.underlying_mint, underlying_mint);
    assert_eq!(stored.decimals, 6);
    assert_eq!(stored.total_supply_handle, supply_lineage.current_handle);
    assert_eq!(supply_lineage.leaf_count, 0);
    assert_eq!(supply_lineage.acl_domain_key, mint.to_bytes());
    assert_eq!(
        supply_lineage.app_account,
        total_supply_authority.to_bytes()
    );
    assert!(supply_lineage.is_subject(compute_signer.to_bytes()));

    assert_eq!(trivial_events.len(), 1);
    assert_eq!(trivial_events[0].subject, compute_signer.to_bytes());
    assert_eq!(trivial_events[0].plaintext, amount_plaintext(0));
    assert_eq!(trivial_events[0].fhe_type, BALANCE_FHE_TYPE);
    assert_eq!(trivial_events[0].result, supply_lineage.current_handle);
    assert_eq!(supply_events.len(), 1);
    assert_eq!(supply_events[0].mint, mint);
    assert_eq!(supply_events[0].old_handle, [0; 32]);
    assert_eq!(supply_events[0].new_handle, supply_lineage.current_handle);
    assert_eq!(
        supply_events[0].reason,
        token::TotalSupplyUpdateReason::Initialize
    );
}

#[test]
fn mollusk_initialize_token_account_creates_initial_balance_acl() {
    let fixture = TokenMolluskFixture::new();
    let owner = Pubkey::new_unique();
    let (token_account, token_bump) = token::token_account_address(fixture.mint, owner);
    let balance_value_acl = token::balance_value_acl_address(fixture.mint, token_account).0;
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_value_acl, system_account(0));
    let mut context = mollusk().with_context(accounts);
    context.mollusk.sysvars.warp_to_slot(10);
    let ix = initialize_token_account_ix(
        owner,
        fixture.mint,
        fixture.compute_signer,
        token_account,
        fixture.host_config,
        0,
    );

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_token_account(&context, token_account);
    let balance_lineage = read_encrypted_value_acl(&context, balance_value_acl)
        .expect("init created the balance lineage");
    let trivial_events: Vec<host::events::TrivialEncryptEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(stored.owner, owner);
    assert_eq!(stored.mint, fixture.mint);
    assert_eq!(stored.balance_handle, balance_lineage.current_handle);
    assert_eq!(stored.next_amount_nonce_sequence, 0);
    assert_eq!(stored.bump, token_bump);
    assert_eq!(balance_lineage.leaf_count, 0);
    assert_eq!(balance_lineage.acl_domain_key, fixture.mint.to_bytes());
    assert_eq!(balance_lineage.app_account, token_account.to_bytes());
    assert!(balance_lineage.is_subject(owner.to_bytes()));
    assert!(balance_lineage.is_subject(fixture.compute_signer.to_bytes()));

    assert_eq!(trivial_events.len(), 1);
    assert_eq!(trivial_events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(trivial_events[0].plaintext, amount_plaintext(0));
    assert_eq!(trivial_events[0].fhe_type, BALANCE_FHE_TYPE);
    assert_eq!(trivial_events[0].result, balance_lineage.current_handle);
    assert_eq!(balance_events.len(), 1);
    assert_eq!(balance_events[0].mint, fixture.mint);
    assert_eq!(balance_events[0].owner, owner);
    assert_eq!(balance_events[0].token_account, token_account);
    assert_eq!(balance_events[0].old_handle, [0; 32]);
    assert_eq!(balance_events[0].new_handle, balance_lineage.current_handle);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::Initialize
    );
}

#[test]
fn mollusk_initialize_token_account_rejects_nonzero_initial_balance() {
    let fixture = TokenMolluskFixture::new();
    let owner = Pubkey::new_unique();
    let token_account = token::token_account_address(fixture.mint, owner).0;
    let balance_value_acl = token::balance_value_acl_address(fixture.mint, token_account).0;
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_value_acl, system_account(0));
    let mut context = mollusk().with_context(accounts);
    context.mollusk.sysvars.warp_to_slot(10);
    let ix = initialize_token_account_ix(
        owner,
        fixture.mint,
        fixture.compute_signer,
        token_account,
        fixture.host_config,
        1,
    );

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_empty_system_account(&context, token_account);
    assert_empty_system_account(&context, balance_value_acl);
}

#[test]
fn mollusk_create_random_amount_advances_nonce_and_emits_events() {
    let fixture = TokenMolluskFixture::new();
    let transfer_acl = amount_acl_address(fixture.mint, fixture.owner, 0);
    let burn_acl = burn_amount_acl_address(fixture.mint, fixture.owner, 1);
    let mut accounts = fixture.base_accounts();
    accounts.insert(transfer_acl, system_account(0));
    accounts.insert(burn_acl, system_account(0));
    let context = warped_context(accounts);
    let upper_bound = amount_plaintext(8);

    let transfer_result = context.process_and_validate_instruction(
        &create_random_bounded_amount_ix(
            &fixture,
            transfer_acl,
            token::ConfidentialAmountKind::Transfer,
            upper_bound,
        ),
        &[Check::success()],
    );

    let token_account_after_transfer = read_token_account(&context, fixture.alice_token);
    let transfer_record = read_acl_record(&context, transfer_acl);
    let bounded_events: Vec<host::events::FheRandBoundedEvent> = transfer_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let transfer_created_events: Vec<token::RandomAmountCreatedEvent> = transfer_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(token_account_after_transfer.next_amount_nonce_sequence, 1);
    assert_eq!(
        token_account_after_transfer.balance_handle,
        fixture.alice_initial
    );
    assert_acl_record(
        &transfer_record,
        token::nonce_key(fixture.mint, fixture.owner, token::transfer_amount_label()),
        0,
        fixture.mint,
        fixture.owner,
        token::transfer_amount_label(),
        BALANCE_FHE_TYPE,
        &[(fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT)],
    );
    assert_eq!(bounded_events.len(), 1);
    assert_eq!(bounded_events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(bounded_events[0].upper_bound, upper_bound);
    assert_eq!(bounded_events[0].fhe_type, BALANCE_FHE_TYPE);
    assert_eq!(bounded_events[0].result, transfer_record.handle);
    assert_eq!(transfer_created_events.len(), 1);
    assert_eq!(transfer_created_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(transfer_created_events[0].mint, fixture.mint);
    assert_eq!(transfer_created_events[0].owner, fixture.owner);
    assert_eq!(
        transfer_created_events[0].token_account,
        fixture.alice_token
    );
    assert_eq!(
        transfer_created_events[0].amount_kind,
        token::ConfidentialAmountKind::Transfer
    );
    assert!(transfer_created_events[0].bounded);
    assert_eq!(transfer_created_events[0].upper_bound, upper_bound);
    assert_eq!(transfer_created_events[0].handle, transfer_record.handle);
    assert_eq!(transfer_created_events[0].acl_record, transfer_acl);
    assert_eq!(transfer_created_events[0].nonce_sequence, 0);

    let burn_result = context.process_and_validate_instruction(
        &create_random_amount_ix(&fixture, burn_acl, token::ConfidentialAmountKind::Burn),
        &[Check::success()],
    );

    let token_account_after_burn = read_token_account(&context, fixture.alice_token);
    let burn_record = read_acl_record(&context, burn_acl);
    let rand_events: Vec<host::events::FheRandEvent> = burn_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let burn_created_events: Vec<token::RandomAmountCreatedEvent> = burn_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(token_account_after_burn.next_amount_nonce_sequence, 2);
    assert_eq!(
        token_account_after_burn.balance_handle,
        fixture.alice_initial
    );
    assert_acl_record(
        &burn_record,
        token::nonce_key(fixture.mint, fixture.owner, token::burn_amount_label()),
        1,
        fixture.mint,
        fixture.owner,
        token::burn_amount_label(),
        BALANCE_FHE_TYPE,
        &[(fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT)],
    );
    assert_eq!(rand_events.len(), 1);
    assert_eq!(rand_events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(rand_events[0].fhe_type, BALANCE_FHE_TYPE);
    assert_eq!(rand_events[0].result, burn_record.handle);
    assert_eq!(burn_created_events.len(), 1);
    assert_eq!(burn_created_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(burn_created_events[0].mint, fixture.mint);
    assert_eq!(burn_created_events[0].owner, fixture.owner);
    assert_eq!(burn_created_events[0].token_account, fixture.alice_token);
    assert_eq!(
        burn_created_events[0].amount_kind,
        token::ConfidentialAmountKind::Burn
    );
    assert!(!burn_created_events[0].bounded);
    assert_eq!(burn_created_events[0].upper_bound, [0; 32]);
    assert_eq!(burn_created_events[0].handle, burn_record.handle);
    assert_eq!(burn_created_events[0].acl_record, burn_acl);
    assert_eq!(burn_created_events[0].nonce_sequence, 1);
}

#[test]
fn mollusk_create_random_bounded_amount_rejects_invalid_upper_bound() {
    let fixture = TokenMolluskFixture::new();
    let amount_acl = amount_acl_address(fixture.mint, fixture.owner, 0);
    let mut accounts = fixture.base_accounts();
    accounts.insert(amount_acl, system_account(0));
    let context = warped_context(accounts);
    let ix = create_random_bounded_amount_ix(
        &fixture,
        amount_acl,
        token::ConfidentialAmountKind::Transfer,
        amount_plaintext(3),
    );

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    let stored = read_token_account(&context, fixture.alice_token);
    assert_eq!(stored.next_amount_nonce_sequence, 0);
    assert_empty_system_account(&context, amount_acl);
}

struct TokenMolluskFixture {
    owner: Pubkey,
    bob_owner: Pubkey,
    mint: Pubkey,
    underlying_mint: Pubkey,
    compute_signer: Pubkey,
    total_supply_authority: Pubkey,
    host_config: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    user_usdc: Pubkey,
    vault_usdc: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    total_supply_initial: [u8; 32],
    alice_current_compute_acl: Pubkey,
    bob_current_compute_acl: Pubkey,
    total_supply_current_acl: Pubkey,
}

impl TokenMolluskFixture {
    fn new() -> Self {
        let owner = Pubkey::new_unique();
        let bob_owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let underlying_mint = Pubkey::new_unique();
        let compute_signer = token::compute_signer_address(mint).0;
        let total_supply_authority = token::total_supply_authority_address(mint).0;
        let host_config = host::host_config_address().0;
        let alice_token = token::token_account_address(mint, owner).0;
        let bob_token = token::token_account_address(mint, bob_owner).0;
        let user_usdc = Pubkey::new_unique();
        let vault_usdc = token::vault_token_account_address(mint, underlying_mint);
        let alice_initial = handle_for_chain(1, BALANCE_FHE_TYPE);
        let bob_initial = handle_for_chain(2, BALANCE_FHE_TYPE);
        let total_supply_initial = handle_for_chain(3, BALANCE_FHE_TYPE);
        let alice_current_compute_acl = token_balance_acl_address(mint, alice_token, 0);
        let bob_current_compute_acl = token_balance_acl_address(mint, bob_token, 0);
        let total_supply_current_acl =
            token_total_supply_acl_address(mint, total_supply_authority, 0);
        Self {
            owner,
            bob_owner,
            mint,
            underlying_mint,
            compute_signer,
            total_supply_authority,
            host_config,
            alice_token,
            bob_token,
            user_usdc,
            vault_usdc,
            alice_initial,
            bob_initial,
            total_supply_initial,
            alice_current_compute_acl,
            bob_current_compute_acl,
            total_supply_current_acl,
        }
    }

    fn context_with_input_amount(
        &self,
        amount_handle: [u8; 32],
    ) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
        self.context_with_input_amount_for_authority(amount_handle, self.owner)
    }

    fn context_with_input_amount_for_authority(
        &self,
        amount_handle: [u8; 32],
        amount_authority: Pubkey,
    ) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
        let amount_acl =
            amount_acl_address(self.mint, amount_authority, DEFAULT_INPUT_NONCE_SEQUENCE);
        let mut accounts = self.base_accounts();
        accounts.insert(
            amount_acl,
            acl_record_account(
                amount_handle,
                token::nonce_key(self.mint, amount_authority, token::transfer_amount_label()),
                DEFAULT_INPUT_NONCE_SEQUENCE,
                self.mint,
                amount_authority,
                token::transfer_amount_label(),
                &[host::AclSubjectEntry::compute(self.compute_signer)],
            ),
        );
        for account in SelfTransferOutputAccounts::canonical(self, 1).all_accounts() {
            accounts.entry(account).or_insert_with(|| system_account(0));
        }
        for account in DirectTransferOutputAccounts::canonical(self, 0, 1).all_accounts() {
            accounts.entry(account).or_insert_with(|| system_account(0));
        }
        warped_context(accounts)
    }

    /// Pre-seeds alice/bob balance lineages as host-owned (as if a prior wrap/init
    /// created them), so a transfer that reads/rotates them can run directly.
    fn seed_balance_lineages(
        &self,
        context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    ) {
        seed_account(
            context,
            token::balance_value_acl_address(self.mint, self.alice_token).0,
            balance_value_acl_account(
                self.mint,
                self.alice_token,
                self.alice_initial,
                &[self.owner, self.compute_signer],
            ),
        );
        seed_account(
            context,
            token::balance_value_acl_address(self.mint, self.bob_token).0,
            balance_value_acl_account(
                self.mint,
                self.bob_token,
                self.bob_initial,
                &[self.bob_owner, self.compute_signer],
            ),
        );
    }

    /// Seeds the mint's total-supply lineage host-owned at `total_supply_initial`
    /// (as initialize_mint would have), so wrap/burn can read+rotate it.
    fn seed_total_supply_lineage(
        &self,
        context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    ) {
        seed_account(
            context,
            token::total_supply_value_acl_address(self.mint, self.total_supply_authority).0,
            total_supply_value_acl_account(
                self.mint,
                self.total_supply_authority,
                self.total_supply_initial,
                &[self.compute_signer],
            ),
        );
    }

    /// Seeds alice's balance lineage and the total-supply lineage — the two
    /// lineages a wrap/burn reads and rotates.
    fn seed_wrap_lineages(&self, context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>) {
        seed_account(
            context,
            token::balance_value_acl_address(self.mint, self.alice_token).0,
            balance_value_acl_account(
                self.mint,
                self.alice_token,
                self.alice_initial,
                &[self.owner, self.compute_signer],
            ),
        );
        self.seed_total_supply_lineage(context);
    }

    fn context_with_wrap_accounts(&self) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
        let mut accounts = self.base_accounts();
        accounts.extend([
            (
                self.underlying_mint,
                spl_mint_account(self.owner, 6, 1_000_000_000),
            ),
            (
                self.user_usdc,
                spl_token_account(self.underlying_mint, self.owner, 1_000_000_000),
            ),
            (
                self.vault_usdc,
                spl_token_account(
                    self.underlying_mint,
                    token::vault_authority_address(self.mint).0,
                    0,
                ),
            ),
        ]);
        for account in WrapOutputAccounts::canonical(self, 1).all_accounts() {
            accounts.entry(account).or_insert_with(|| system_account(0));
        }
        warped_context(accounts)
    }

    fn context_with_wrap_and_burn_amount(
        &self,
        amount_handle: [u8; 32],
    ) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
        let mut accounts = self.base_accounts();
        accounts.extend([
            (
                self.underlying_mint,
                spl_mint_account(self.owner, 6, 1_000_000_000),
            ),
            (
                self.user_usdc,
                spl_token_account(self.underlying_mint, self.owner, 1_000_000_000),
            ),
            (
                self.vault_usdc,
                spl_token_account(
                    self.underlying_mint,
                    token::vault_authority_address(self.mint).0,
                    0,
                ),
            ),
            (
                burn_amount_acl_address(self.mint, self.owner, DEFAULT_INPUT_NONCE_SEQUENCE),
                acl_record_account(
                    amount_handle,
                    token::nonce_key(self.mint, self.owner, token::burn_amount_label()),
                    DEFAULT_INPUT_NONCE_SEQUENCE,
                    self.mint,
                    self.owner,
                    token::burn_amount_label(),
                    &[host::AclSubjectEntry::compute(self.compute_signer)],
                ),
            ),
        ]);
        for account in WrapOutputAccounts::canonical(self, 1).all_accounts() {
            accounts.entry(account).or_insert_with(|| system_account(0));
        }
        for account in BurnOutputAccounts::canonical(self, 0, 0).all_accounts() {
            accounts.entry(account).or_insert_with(|| system_account(0));
        }
        warped_context(accounts)
    }

    fn base_accounts(&self) -> HashMap<Pubkey, Account> {
        HashMap::from([
            (self.owner, system_account(5_000_000_000)),
            (self.bob_owner, system_account(5_000_000_000)),
            (self.mint, confidential_mint_account(self)),
            (self.compute_signer, system_account(0)),
            // Mint-scoped total-supply app authority PDA; the total-supply lineage
            // upsert / transient eval signs as this authority.
            (self.total_supply_authority, system_account(0)),
            (self.host_config, host_config_account(self.owner)),
            kms_context_account([0x11u8; 20]),
            (
                self.alice_token,
                confidential_token_account(
                    self.owner,
                    self.mint,
                    self.alice_initial,
                    self.alice_current_compute_acl,
                ),
            ),
            (
                self.bob_token,
                confidential_token_account(
                    self.bob_owner,
                    self.mint,
                    self.bob_initial,
                    self.bob_current_compute_acl,
                ),
            ),
            (
                self.alice_current_compute_acl,
                acl_record_account(
                    self.alice_initial,
                    token::balance_nonce_key(self.mint, self.alice_token),
                    0,
                    self.mint,
                    self.alice_token,
                    token::balance_label(),
                    &[
                        host::AclSubjectEntry::user(self.owner),
                        host::AclSubjectEntry::compute(self.compute_signer),
                    ],
                ),
            ),
            (
                self.bob_current_compute_acl,
                acl_record_account(
                    self.bob_initial,
                    token::balance_nonce_key(self.mint, self.bob_token),
                    0,
                    self.mint,
                    self.bob_token,
                    token::balance_label(),
                    &[
                        host::AclSubjectEntry::user(self.bob_owner),
                        host::AclSubjectEntry::compute(self.compute_signer),
                    ],
                ),
            ),
            (
                self.total_supply_current_acl,
                acl_record_account(
                    self.total_supply_initial,
                    token::total_supply_nonce_key(self.mint, self.total_supply_authority),
                    0,
                    self.mint,
                    self.total_supply_authority,
                    token::total_supply_label(),
                    &[host::AclSubjectEntry::compute(self.compute_signer)],
                ),
            ),
            (event_authority(host::id()), system_account(0)),
            (event_authority(token::id()), system_account(0)),
            // Balance/total-supply encrypted-value ACL lineage PDAs default to empty
            // (system-owned). Tests whose flow READS a lineage as a durable eval input
            // (wrap, burn, transfer, callbacks) seed them host-owned via
            // `seed_balance_lineages` / `seed_total_supply_lineage`.
            (
                token::balance_value_acl_address(self.mint, self.alice_token).0,
                system_account(0),
            ),
            (
                token::balance_value_acl_address(self.mint, self.bob_token).0,
                system_account(0),
            ),
            (
                token::total_supply_value_acl_address(self.mint, self.total_supply_authority).0,
                system_account(0),
            ),
        ])
    }
}

#[derive(Clone, Copy)]
struct SelfTransferOutputAccounts {
    alice: Pubkey,
    to_output: Pubkey,
    transferred: Pubkey,
}

impl SelfTransferOutputAccounts {
    fn canonical(fixture: &TokenMolluskFixture, amount_nonce_sequence: u64) -> Self {
        // Balances are encrypted-value ACL lineages (fixed PDA, rotated in place).
        // A self-transfer only mints the one-shot transferred-amount record.
        let alice = token::balance_value_acl_address(fixture.mint, fixture.alice_token).0;
        Self {
            alice,
            to_output: alice,
            transferred: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::transferred_amount_label(),
                amount_nonce_sequence,
            ),
        }
    }

    fn all_accounts(self) -> [Pubkey; 2] {
        [self.alice, self.transferred]
    }
}

#[derive(Clone, Copy)]
struct DirectTransferOutputAccounts {
    /// Sender balance encrypted-value ACL lineage (fixed PDA, rotated in place).
    from_output: Pubkey,
    /// Recipient balance encrypted-value ACL lineage (fixed PDA, rotated in place).
    to_output: Pubkey,
    /// One-shot transferred-amount ACL record (keyed by the sender's amount nonce).
    transferred: Pubkey,
}

impl DirectTransferOutputAccounts {
    fn canonical(fixture: &TokenMolluskFixture, amount_nonce_sequence: u64, _unused: u64) -> Self {
        Self {
            from_output: token::balance_value_acl_address(fixture.mint, fixture.alice_token).0,
            to_output: token::balance_value_acl_address(fixture.mint, fixture.bob_token).0,
            transferred: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::transferred_amount_label(),
                amount_nonce_sequence,
            ),
        }
    }

    fn all_accounts(self) -> [Pubkey; 3] {
        [self.from_output, self.to_output, self.transferred]
    }
}

#[derive(Clone, Copy)]
struct WrapOutputAccounts {
    balance: Pubkey,
    total_supply: Pubkey,
}

impl WrapOutputAccounts {
    fn canonical(fixture: &TokenMolluskFixture, _unused: u64) -> Self {
        // Balance and total supply are encrypted-value ACL lineages (fixed PDAs,
        // rotated in place), not per-rotation records.
        Self {
            balance: token::balance_value_acl_address(fixture.mint, fixture.alice_token).0,
            total_supply: token::total_supply_value_acl_address(
                fixture.mint,
                fixture.total_supply_authority,
            )
            .0,
        }
    }

    fn all_accounts(self) -> [Pubkey; 2] {
        [self.balance, self.total_supply]
    }
}

#[derive(Clone, Copy)]
struct BurnOutputAccounts {
    balance: Pubkey,
    burned: Pubkey,
    total_supply: Pubkey,
}

impl BurnOutputAccounts {
    fn canonical(fixture: &TokenMolluskFixture, amount_nonce_sequence: u64, _unused: u64) -> Self {
        // Balance and total supply are encrypted-value ACL lineages (fixed PDAs);
        // only the burned amount is a one-shot record keyed by the amount nonce.
        Self {
            balance: token::balance_value_acl_address(fixture.mint, fixture.alice_token).0,
            burned: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::burned_amount_label(),
                amount_nonce_sequence,
            ),
            total_supply: token::total_supply_value_acl_address(
                fixture.mint,
                fixture.total_supply_authority,
            )
            .0,
        }
    }

    fn all_accounts(self) -> [Pubkey; 3] {
        [self.balance, self.burned, self.total_supply]
    }
}

#[derive(Clone, Copy)]
struct CallbackSettlementOutputAccounts {
    settlement: Pubkey,
    to_output: Pubkey,
    refund: Pubkey,
    from_output: Pubkey,
    final_transferred: Pubkey,
}

impl CallbackSettlementOutputAccounts {
    /// `to_amount_nonce` is the recipient's amount nonce at prepare time (keys the
    /// refund record); `from_amount_nonce` is the sender's amount nonce at finalize
    /// time (keys the final-transferred record). Balances are fixed lineage PDAs.
    fn canonical(
        fixture: &TokenMolluskFixture,
        sent_handle: [u8; 32],
        from_amount_nonce: u64,
        to_amount_nonce: u64,
    ) -> Self {
        Self {
            settlement: token::transfer_callback_settlement_address(fixture.mint, sent_handle).0,
            to_output: token::balance_value_acl_address(fixture.mint, fixture.bob_token).0,
            refund: token_acl_address(
                fixture.mint,
                fixture.bob_token,
                token::callback_refund_amount_label(),
                to_amount_nonce,
            ),
            from_output: token::balance_value_acl_address(fixture.mint, fixture.alice_token).0,
            final_transferred: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::callback_final_transferred_label(),
                from_amount_nonce,
            ),
        }
    }

    #[allow(dead_code)]
    fn all_accounts(self) -> [Pubkey; 5] {
        [
            self.settlement,
            self.to_output,
            self.refund,
            self.from_output,
            self.final_transferred,
        ]
    }
}

fn self_transfer_ix(
    fixture: &TokenMolluskFixture,
    output: SelfTransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            owner: fixture.owner,
            payer: fixture.owner,
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            amount_compute_acl: amount_acl_address(
                fixture.mint,
                fixture.owner,
                DEFAULT_INPUT_NONCE_SEQUENCE,
            ),
            from_balance_value_acl: output.alice,
            transferred_amount_acl: output.transferred,
            to_balance_value_acl: output.to_output,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransfer { amount_handle },
    )
}

fn direct_transfer_ix(
    fixture: &TokenMolluskFixture,
    output: DirectTransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    direct_transfer_ix_with_payer(fixture, fixture.owner, output, amount_handle)
}

fn direct_transfer_ix_with_payer(
    fixture: &TokenMolluskFixture,
    payer: Pubkey,
    output: DirectTransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    direct_transfer_ix_with_acls(
        fixture,
        payer,
        amount_acl_address(fixture.mint, fixture.owner, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    )
}

fn direct_transfer_ix_with_acls(
    fixture: &TokenMolluskFixture,
    payer: Pubkey,
    amount_compute_acl: Pubkey,
    output: DirectTransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            owner: fixture.owner,
            payer,
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            amount_compute_acl,
            from_balance_value_acl: output.from_output,
            transferred_amount_acl: output.transferred,
            to_balance_value_acl: output.to_output,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransfer { amount_handle },
    )
}

#[allow(clippy::too_many_arguments)]
fn initialize_mint_ix(
    authority: Pubkey,
    mint: Pubkey,
    underlying_mint: Pubkey,
    compute_signer: Pubkey,
    total_supply_authority: Pubkey,
    total_supply_value_acl: Pubkey,
    host_config: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::InitializeMint {
            authority,
            mint,
            underlying_mint,
            compute_signer,
            total_supply_authority,
            total_supply_value_acl,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::InitializeMint {},
    )
}

#[allow(clippy::too_many_arguments)]
fn initialize_token_account_ix(
    owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    token_account: Pubkey,
    host_config: Pubkey,
    initial_balance: u64,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::InitializeTokenAccount {
            owner,
            mint,
            compute_signer,
            token_account,
            balance_value_acl: token::balance_value_acl_address(mint, token_account).0,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::InitializeTokenAccount { initial_balance },
    )
}

fn create_random_amount_ix(
    fixture: &TokenMolluskFixture,
    amount_acl_record: Pubkey,
    amount_kind: token::ConfidentialAmountKind,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CreateRandomAmount {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            amount_acl_record,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::CreateRandomAmount { amount_kind },
    )
}

fn create_random_bounded_amount_ix(
    fixture: &TokenMolluskFixture,
    amount_acl_record: Pubkey,
    amount_kind: token::ConfidentialAmountKind,
    upper_bound: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CreateRandomAmount {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            amount_acl_record,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::CreateRandomBoundedAmount {
            amount_kind,
            upper_bound,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn call_transfer_receiver_ix(
    fixture: &TokenMolluskFixture,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
    receiver_program: Pubkey,
    receiver_instruction_data: Vec<u8>,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialCallTransferReceiver {
            caller: fixture.owner,
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            sent_amount_acl,
            callback_success_acl,
            receiver_program,
            instructions_sysvar: sysvar::instructions::ID,
            hook_record: token::transfer_receiver_hook_address(fixture.mint, sent_handle).0,
            system_program: system_program::ID,
        },
        token::instruction::ConfidentialCallTransferReceiver {
            sent_handle,
            callback_success_handle,
            receiver_instruction_data,
        },
    )
}

fn accept_transfer_receiver_data(
    fixture: &TokenMolluskFixture,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
) -> Vec<u8> {
    receiver::instruction::AcceptConfidentialTransfer {
        mint: fixture.mint,
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle,
        sent_acl_record: sent_amount_acl,
        callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data()
}

fn wrap_usdc_ix(
    fixture: &TokenMolluskFixture,
    output: WrapOutputAccounts,
    amount: u64,
) -> Instruction {
    wrap_usdc_ix_with_vault(fixture, output, amount, fixture.vault_usdc)
}

fn wrap_usdc_ix_with_vault(
    fixture: &TokenMolluskFixture,
    output: WrapOutputAccounts,
    amount: u64,
    vault_usdc: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::WrapUsdc {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint,
            user_usdc: fixture.user_usdc,
            vault_usdc,
            vault_authority: token::vault_authority_address(fixture.mint).0,
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            total_supply_value_acl: output.total_supply,
            balance_value_acl: output.balance,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            token_program: spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::WrapUsdc { amount },
    )
}

fn disclose_balance_public_ix(
    fixture: &TokenMolluskFixture,
    owner: Pubkey,
    balance_value_acl: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseBalancePublic {
            owner,
            token_account: fixture.alice_token,
            balance_value_acl,
            zama_program: host::id(),
            system_program: system_program::ID,
        },
        token::instruction::DiscloseBalancePublic {},
    )
}

fn burn_ix(
    fixture: &TokenMolluskFixture,
    _balance_value_acl: Pubkey,
    _total_supply_value_acl: Pubkey,
    output: BurnOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    burn_ix_with_amount_acl(
        fixture,
        _balance_value_acl,
        _total_supply_value_acl,
        burn_amount_acl_address(fixture.mint, fixture.owner, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    )
}

fn burn_ix_with_amount_acl(
    fixture: &TokenMolluskFixture,
    _balance_value_acl: Pubkey,
    _total_supply_value_acl: Pubkey,
    amount_compute_acl: Pubkey,
    output: BurnOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialBurn {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            amount_compute_acl,
            balance_value_acl: output.balance,
            burned_amount_acl: output.burned,
            total_supply_value_acl: output.total_supply,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialBurn { amount_handle },
    )
}

fn request_disclose_amount_ix(
    fixture: &TokenMolluskFixture,
    amount_acl_record: Pubkey,
    amount_handle: [u8; 32],
) -> Instruction {
    request_disclose_amount_ix_with_nonce(
        fixture,
        amount_acl_record,
        amount_handle,
        request_nonce(1),
    )
}

fn request_disclose_amount_ix_with_nonce(
    fixture: &TokenMolluskFixture,
    amount_acl_record: Pubkey,
    amount_handle: [u8; 32],
    request_nonce: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RequestDiscloseAmount {
            requester: fixture.owner,
            mint: fixture.mint,
            amount_acl_record,
            amount_material_commitment: host::handle_material_address(amount_acl_record).0,
            disclosure_request: disclosure_request_address(
                fixture,
                fixture.owner,
                amount_handle,
                request_nonce,
            ),
            authority_permission_record: None,
            deny_subject_record: None,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RequestDiscloseAmount {
            amount_handle,
            request_nonce,
            expires_slot: DEFAULT_REQUEST_EXPIRES_SLOT,
        },
    )
}

// --- KMS EIP-712 disclose (secp256k1_recover) — #1494 Phase 3 cert-secp ---

const SECP_GATEWAY_CHAIN_ID: u64 = 31337;
const SECP_DECRYPTION_CONTRACT: [u8; 20] = [0xDEu8; 20];

/// Recovers the EVM address (keccak(pubkey)[12..]) for a KMS signing key.
fn secp_evm_address(key: &k256::ecdsa::SigningKey) -> [u8; 20] {
    let encoded = key.verifying_key().to_encoded_point(false);
    let hash = solana_program::keccak::hash(&encoded.as_bytes()[1..]).to_bytes();
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

/// 65-byte `[r || s || v]` recoverable signature over an EIP-712 digest.
fn secp_sign(key: &k256::ecdsa::SigningKey, digest: &[u8; 32]) -> [u8; 65] {
    let (signature, recovery_id) = key.sign_prehash_recoverable(digest).unwrap();
    let mut out = [0u8; 65];
    out[..64].copy_from_slice(&signature.to_bytes());
    out[64] = 27 + recovery_id.to_byte();
    out
}

fn decrypted_u64_bytes(value: u64) -> [u8; 32] {
    let mut decrypted = [0u8; 32];
    decrypted[24..].copy_from_slice(&value.to_be_bytes());
    decrypted
}

const KMS_CONTEXT_ID: u64 = 1;

/// Host config pointing at the active KMS context id + the gateway decryption contract.
fn kms_host_config_account(authority: Pubkey) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::HostConfig {
            admin: authority,
            chain_id: host::SOLANA_POC_CHAIN_ID,
            input_verifier_authority: authority,
            gateway_chain_id: SECP_GATEWAY_CHAIN_ID,
            input_verification_contract: [0u8; 20],
            coprocessor_signer: [0u8; 20],
            decryption_contract: SECP_DECRYPTION_CONTRACT,
            current_kms_context_id: KMS_CONTEXT_ID,
            material_authority: Pubkey::new_unique(),
            test_authority: authority,
            paused: false,
            mock_input_enabled: true,
            test_shims_enabled: true,
            grant_deny_list_enabled: false,
            updated_slot: 0,
            bump: host::host_config_address().1,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

/// KMS context PDA holding a single signer + threshold-1 at the test context id.
fn kms_context_account(signer: [u8; 20]) -> (Pubkey, Account) {
    kms_context_account_for(KMS_CONTEXT_ID, signer)
}

/// KMS context PDA for an arbitrary context id (used to model rotation).
fn kms_context_account_for(context_id: u64, signer: [u8; 20]) -> (Pubkey, Account) {
    let (pubkey, bump) = host::kms_context_address(context_id);
    (
        pubkey,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::KmsContext {
                context_id,
                signers: vec![signer],
                thresholds: host::KmsThresholds {
                    public_decryption: 1,
                    user_decryption: 1,
                    kms_gen: 1,
                    mpc: 1,
                },
                destroyed: false,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// KMS `PublicDecryptVerification` signature set over a single handle + cleartext.
fn kms_public_decrypt_signatures(
    key: &k256::ecdsa::SigningKey,
    handle: [u8; 32],
    cleartext_amount: u64,
) -> Vec<[u8; 65]> {
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"Decryption",
            b"1",
            SECP_GATEWAY_CHAIN_ID,
            &SECP_DECRYPTION_CONTRACT,
        ),
        &host::eip712::public_decrypt_struct_hash(
            &[handle],
            &decrypted_u64_bytes(cleartext_amount),
            &[],
        ),
    );
    vec![secp_sign(key, &digest)]
}

fn disclose_amount_secp_ix(
    fixture: &TokenMolluskFixture,
    amount_acl_record: Pubkey,
    amount_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseAmountSecp {
            mint: fixture.mint,
            amount_acl_record,
            amount_material_commitment: host::handle_material_address(amount_acl_record).0,
            disclosure_request: disclosure_request_address(
                fixture,
                fixture.owner,
                amount_handle,
                request_nonce(1),
            ),
            host_config: fixture.host_config,
            kms_context: host::kms_context_address(KMS_CONTEXT_ID).0,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseAmountSecp {
            amount_handle,
            cleartext_amount,
            signatures,
            extra_data: vec![],
        },
    )
}

#[test]
fn mollusk_disclose_amount_secp_accepts_real_kms_certificate_and_consumes_witness() {
    let fixture = TokenMolluskFixture::new();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
    let amount_handle = handle_for_chain(61, BALANCE_FHE_TYPE);
    let cleartext_amount = 41;
    let mut accounts = fixture.base_accounts();
    accounts.insert(fixture.host_config, kms_host_config_account(fixture.owner));
    let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&key));
    accounts.insert(kms_ctx, kms_ctx_account);
    let context = mollusk().with_context(accounts);
    let amount_acl = seed_disclosable_amount_acl(&context, &fixture, amount_handle);
    seed_material_commitment_for_acl(&context, amount_acl, 145);

    let request_result = process_transaction(
        &context,
        &[request_disclose_amount_ix(
            &fixture,
            amount_acl,
            amount_handle,
        )],
    );
    assert!(request_result.raw_result.is_ok());

    let signatures = kms_public_decrypt_signatures(&key, amount_handle, cleartext_amount);
    let result = process_transaction(
        &context,
        &[disclose_amount_secp_ix(
            &fixture,
            amount_acl,
            amount_handle,
            cleartext_amount,
            signatures.clone(),
        )],
    );
    assert!(result.raw_result.is_ok());
    let request =
        disclosure_request_address(&fixture, fixture.owner, amount_handle, request_nonce(1));
    assert_eq!(
        read_disclosure_request(&context, request).status,
        token::REQUEST_STATUS_CONSUMED
    );

    // Replaying the same cert against the now-CONSUMED witness must fail.
    let replay = process_transaction(
        &context,
        &[disclose_amount_secp_ix(
            &fixture,
            amount_acl,
            amount_handle,
            cleartext_amount,
            signatures,
        )],
    );
    assert!(
        replay.raw_result.is_err(),
        "consumed disclosure witness must not be replayable"
    );
}

/// Creates the burn-redemption request witness consumed by `redeem_burned_amount_secp`.
fn request_burn_redemption_ix(
    fixture: &TokenMolluskFixture,
    burned_amount_acl: Pubkey,
    burned_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RequestBurnRedemption {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint,
            destination_usdc: fixture.user_usdc,
            burned_amount_acl,
            burned_material_commitment: host::handle_material_address(burned_amount_acl).0,
            redemption_request: burn_redemption_request_address(
                fixture,
                burned_handle,
                request_nonce(1),
            ),
            authority_permission_record: None,
            deny_subject_record: None,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RequestBurnRedemption {
            burned_handle,
            request_nonce: request_nonce(1),
            expires_slot: DEFAULT_REQUEST_EXPIRES_SLOT,
        },
    )
}

/// Solana (secp256k1 EIP-712) counterpart of [`redeem_burned_amount_ix_with_vault`]: verifies
/// the KMS `PublicDecryptVerification` cert on-chain against the request-pinned KMS context.
fn redeem_burned_amount_secp_ix(
    fixture: &TokenMolluskFixture,
    burned_amount_acl: Pubkey,
    redemption_record: Pubkey,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
    vault_usdc: Pubkey,
    signatures: Vec<[u8; 65]>,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RedeemBurnedAmountSecp {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint,
            vault_usdc,
            destination_usdc: fixture.user_usdc,
            vault_authority: token::vault_authority_address(fixture.mint).0,
            burned_amount_acl,
            burned_material_commitment: host::handle_material_address(burned_amount_acl).0,
            redemption_request: burn_redemption_request_address(
                fixture,
                burned_handle,
                request_nonce(1),
            ),
            redemption_record,
            host_config: fixture.host_config,
            kms_context: host::kms_context_address(KMS_CONTEXT_ID).0,
            token_program: spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RedeemBurnedAmountSecp {
            burned_handle,
            cleartext_amount,
            signatures,
            extra_data: vec![],
        },
    )
}

fn wrap_and_burn_for_redeem(
    fixture: &TokenMolluskFixture,
    burn_amount_handle: [u8; 32],
) -> (
    WrapOutputAccounts,
    BurnOutputAccounts,
    mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    [u8; 32],
) {
    let wrap_output = WrapOutputAccounts::canonical(fixture, 1);
    let burn_output = BurnOutputAccounts::canonical(fixture, 0, 0);
    let context = fixture.context_with_wrap_and_burn_amount(burn_amount_handle);
    fixture.seed_wrap_lineages(&context);
    let wrap_ix = wrap_usdc_ix(fixture, wrap_output, 100_000_000);

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    let burn_ix = burn_ix(
        fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        burn_output,
        burn_amount_handle,
    );
    context.process_and_validate_instruction(&burn_ix, &[Check::success()]);

    let burned_handle = read_acl_record(&context, burn_output.burned).handle;
    (wrap_output, burn_output, context, burned_handle)
}

fn prepare_transfer_callback_ix(
    fixture: &TokenMolluskFixture,
    _to_balance_value_acl: Pubkey,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
    output: CallbackSettlementOutputAccounts,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialPrepareTransferCallback {
            payer: fixture.bob_owner,
            callback_authority: fixture.bob_owner,
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            sent_amount_acl,
            callback_success_acl,
            hook_record: token::transfer_receiver_hook_address(fixture.mint, sent_handle).0,
            settlement_record: output.settlement,
            to_balance_value_acl: output.to_output,
            refund_amount_acl: output.refund,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialPrepareTransferCallback {
            sent_handle,
            callback_success_handle,
        },
    )
}

fn finalize_transfer_callback_ix(
    fixture: &TokenMolluskFixture,
    _from_balance_value_acl: Pubkey,
    sent_amount_acl: Pubkey,
    output: CallbackSettlementOutputAccounts,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialFinalizeTransferCallback {
            payer: fixture.bob_owner,
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            sent_amount_acl,
            settlement_record: output.settlement,
            refund_amount_acl: output.refund,
            from_balance_value_acl: output.from_output,
            transferred_amount_acl: output.final_transferred,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialFinalizeTransferCallback {},
    )
}

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    let mut mollusk = Mollusk::new(&token::id(), "confidential_token");
    mollusk.add_program(&host::id(), "zama_host");
    mollusk.add_program(&receiver::id(), "confidential_token_receiver");
    mollusk_svm_programs_token::token::add_program(&mut mollusk);
    mollusk
}

/// A mollusk context with SlotHashes populated (slot >= 1) so fhe_eval's
/// `previous_bank_hash` lookup resolves instead of erroring 6053.
fn warped_context(
    accounts: HashMap<Pubkey, Account>,
) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
    let mut context = mollusk().with_context(accounts);
    context.mollusk.sysvars.warp_to_slot(10);
    context
}

fn anchor_ix<A, D>(program_id: Pubkey, accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

fn confidential_mint_account(fixture: &TokenMolluskFixture) -> Account {
    confidential_mint_account_with_compute_signer_and_extra(fixture, fixture.compute_signer, 0)
}

fn confidential_mint_account_with_compute_signer_and_extra(
    fixture: &TokenMolluskFixture,
    compute_signer: Pubkey,
    extra_bytes: usize,
) -> Account {
    let mut data = serialized_account(token::ConfidentialMint {
        authority: fixture.owner,
        acl_domain_key: fixture.mint,
        compute_signer,
        underlying_mint: fixture.underlying_mint,
        decimals: 6,
        total_supply_handle: fixture.total_supply_initial,
    });
    data.resize(data.len() + extra_bytes, 0);
    Account {
        lamports: 1_000_000_000,
        data,
        owner: token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn confidential_token_account(
    owner: Pubkey,
    mint: Pubkey,
    balance_handle: [u8; 32],
    _balance_acl_record: Pubkey,
) -> Account {
    confidential_token_account_with_bump_and_extra(
        owner,
        mint,
        balance_handle,
        _balance_acl_record,
        token::token_account_address(mint, owner).1,
        0,
    )
}

fn confidential_token_account_with_bump_and_extra(
    owner: Pubkey,
    mint: Pubkey,
    balance_handle: [u8; 32],
    _balance_acl_record: Pubkey,
    bump: u8,
    extra_bytes: usize,
) -> Account {
    let mut data = serialized_account(token::ConfidentialTokenAccount {
        owner,
        mint,
        balance_handle,
        next_amount_nonce_sequence: 0,
        bump,
    });
    data.resize(data.len() + extra_bytes, 0);
    Account {
        lamports: 1_000_000_000,
        data,
        owner: token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn spl_mint_account(authority: Pubkey, decimals: u8, supply: u64) -> Account {
    let mut data = vec![0; spl_token::state::Mint::LEN];
    spl_token::state::Mint::pack(
        spl_token::state::Mint {
            mint_authority: COption::Some(authority),
            supply,
            decimals,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        &mut data,
    )
    .expect("SPL mint should pack");
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn spl_token_account(mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
    let mut data = vec![0; spl_token::state::Account::LEN];
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        &mut data,
    )
    .expect("SPL token account should pack");
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn seed_noncanonical_vault_token_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    amount: u64,
) -> Pubkey {
    let vault = Pubkey::new_unique();
    seed_account(
        context,
        vault,
        spl_token_account(
            fixture.underlying_mint,
            token::vault_authority_address(fixture.mint).0,
            amount,
        ),
    );
    vault
}

fn transfer_receiver_hook_account(
    fixture: &TokenMolluskFixture,
    sent_handle: [u8; 32],
    sent_acl_record: Pubkey,
    callback_success_handle: [u8; 32],
    callback_success_acl_record: Pubkey,
) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(token::TransferReceiverHookCall {
            mint: fixture.mint,
            from_token_account: fixture.alice_token,
            to_token_account: fixture.bob_token,
            sent_handle,
            sent_acl_record,
            callback_success_handle,
            callback_success_acl_record,
            receiver_program: Pubkey::new_unique(),
            caller: fixture.owner,
            bump: token::transfer_receiver_hook_address(fixture.mint, sent_handle).1,
        }),
        owner: token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn host_config_account(authority: Pubkey) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::HostConfig {
            admin: authority,
            chain_id: host::SOLANA_POC_CHAIN_ID,
            input_verifier_authority: authority,
            gateway_chain_id: SECP_GATEWAY_CHAIN_ID,
            input_verification_contract: [0u8; 20],
            coprocessor_signer: [0u8; 20],
            decryption_contract: SECP_DECRYPTION_CONTRACT,
            // Request witnesses pin this context id; the secp disclose/redeem path verifies the
            // KMS cert against it.
            current_kms_context_id: KMS_CONTEXT_ID,
            material_authority: Pubkey::new_unique(),
            test_authority: authority,
            paused: false,
            mock_input_enabled: true,
            test_shims_enabled: true,
            grant_deny_list_enabled: false,
            updated_slot: 0,
            bump: host::host_config_address().1,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn acl_record_account(
    handle: [u8; 32],
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    entries: &[host::AclSubjectEntry],
) -> Account {
    let (_, bump) = host::acl_record_address(nonce_key, nonce_sequence);
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    let mut subject_roles = [0; host::MAX_ACL_SUBJECTS];
    for (index, entry) in entries.iter().enumerate() {
        subjects[index] = entry.pubkey;
        subject_roles[index] = entry.role_flags;
    }
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::AclRecord {
            handle,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            subjects,
            subject_roles,
            subject_count: entries.len() as u8,
            overflow_subject_count: 0,
            public_decrypt: false,
            material_commitment: Pubkey::default(),
            material_commitment_hash: [0; 32],
            material_key_id: [0; 32],
            created_slot: 0,
            bump,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

fn read_token_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::ConfidentialTokenAccount {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing token account")
        .clone();
    token::ConfidentialTokenAccount::try_deserialize(&mut account.data.as_slice())
        .expect("token account should deserialize")
}

fn read_confidential_mint(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::ConfidentialMint {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing confidential mint")
        .clone();
    assert_eq!(account.owner, token::id());
    token::ConfidentialMint::try_deserialize(&mut account.data.as_slice())
        .expect("confidential mint should deserialize")
}

fn read_spl_token_amount(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> u64 {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing SPL token account")
        .clone();
    assert_eq!(account.owner, spl_token::id());
    spl_token::state::Account::unpack(&account.data)
        .expect("SPL token account should unpack")
        .amount
}

fn read_acl_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> host::AclRecord {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing ACL account")
        .clone();
    assert_eq!(account.owner, host::id());
    host::AclRecord::try_deserialize(&mut account.data.as_slice())
        .expect("ACL account should deserialize")
}

fn read_transfer_callback_settlement(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::TransferCallbackSettlement {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing callback settlement")
        .clone();
    assert_eq!(account.owner, token::id());
    token::TransferCallbackSettlement::try_deserialize(&mut account.data.as_slice())
        .expect("callback settlement should deserialize")
}

fn read_transfer_receiver_hook_call(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::TransferReceiverHookCall {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing receiver hook marker")
        .clone();
    assert_eq!(account.owner, token::id());
    token::TransferReceiverHookCall::try_deserialize(&mut account.data.as_slice())
        .expect("receiver hook marker should deserialize")
}

fn acl_record_exists(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    let Some(account) = context.account_store.borrow().get(&address).cloned() else {
        return false;
    };
    account.owner == host::id()
        && host::AclRecord::try_deserialize(&mut account.data.as_slice()).is_ok()
}

fn acl_record_count(context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>) -> usize {
    context
        .account_store
        .borrow()
        .values()
        .filter(|account| {
            account.owner == host::id()
                && host::AclRecord::try_deserialize(&mut account.data.as_slice()).is_ok()
        })
        .count()
}

fn read_burn_redemption(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::BurnRedemption {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing burn redemption")
        .clone();
    assert_eq!(account.owner, token::id());
    token::BurnRedemption::try_deserialize(&mut account.data.as_slice())
        .expect("burn redemption should deserialize")
}

fn read_disclosure_request(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::DisclosureRequest {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing disclosure request")
        .clone();
    assert_eq!(account.owner, token::id());
    token::DisclosureRequest::try_deserialize(&mut account.data.as_slice())
        .expect("disclosure request should deserialize")
}

fn transfer_callback_settlement_exists(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    let Some(account) = context.account_store.borrow().get(&address).cloned() else {
        return false;
    };
    account.owner == token::id()
        && token::TransferCallbackSettlement::try_deserialize(&mut account.data.as_slice()).is_ok()
}

fn transfer_receiver_hook_call_exists(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    let Some(account) = context.account_store.borrow().get(&address).cloned() else {
        return false;
    };
    account.owner == token::id()
        && token::TransferReceiverHookCall::try_deserialize(&mut account.data.as_slice()).is_ok()
}

#[allow(clippy::too_many_arguments)]
fn assert_acl_record(
    record: &host::AclRecord,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    fhe_type: u8,
    subjects: &[(Pubkey, u8)],
) {
    assert_eq!(record.nonce_key, nonce_key);
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record.subject_count, subjects.len() as u8);
    assert!(!record.public_decrypt);
    assert_eq!(host::handle_fhe_type(record.handle), fhe_type);
    for (subject, role) in subjects {
        let index = record
            .inline_subject_index(*subject)
            .expect("subject should be stored inline");
        assert_eq!(record.subject_roles[index], *role);
    }
}

fn assert_empty_system_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing seeded system account")
        .clone();
    assert_eq!(account.owner, system_program::ID);
    assert!(account.data.is_empty());
    assert!(!account.executable);
}

fn assert_transfer_scratch_acl_records_absent(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    nonce_sequence: u64,
) {
    assert!(!acl_record_exists(
        context,
        token_acl_address(
            fixture.mint,
            fixture.alice_token,
            token::transfer_success_label(),
            nonce_sequence,
        ),
    ));
    assert!(!acl_record_exists(
        context,
        token_acl_address(
            fixture.mint,
            fixture.alice_token,
            token::debit_candidate_label(),
            nonce_sequence,
        ),
    ));
}

/// A failed transfer must leave both balances unchanged and not mint the one-shot
/// transferred-amount record. Balances are lineages (which may already exist), so
/// only the transferred-amount record is asserted absent.
fn assert_direct_transfer_failure_preserved_state(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    expected_alice_handle: [u8; 32],
    expected_alice_amount_nonce: u64,
    expected_bob_handle: [u8; 32],
    expected_bob_amount_nonce: u64,
    output: DirectTransferOutputAccounts,
) {
    let alice_token = read_token_account(context, fixture.alice_token);
    let bob_token = read_token_account(context, fixture.bob_token);
    assert_eq!(alice_token.balance_handle, expected_alice_handle);
    assert_eq!(
        alice_token.next_amount_nonce_sequence,
        expected_alice_amount_nonce
    );
    assert_eq!(bob_token.balance_handle, expected_bob_handle);
    assert_eq!(
        bob_token.next_amount_nonce_sequence,
        expected_bob_amount_nonce
    );
    assert!(!acl_record_exists(context, output.transferred));
}

fn decode_anchor_event<T>(data: &[u8]) -> Option<T>
where
    T: AnchorDeserialize + Discriminator,
{
    let event_prefix = anchor_event_prefix(T::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    T::deserialize(&mut &*payload).ok()
}

fn anchor_event_prefix(discriminator: &[u8]) -> Vec<u8> {
    anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(discriminator.iter().copied())
        .collect()
}

fn process_transaction(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    instructions: &[Instruction],
) -> TransactionResult {
    let mut account_map: HashMap<Pubkey, Account> = context
        .account_store
        .borrow()
        .iter()
        .map(|(pubkey, account)| (*pubkey, account.clone()))
        .collect();
    account_map.insert(ed25519_program::ID, precompile_account());
    let program_ids = instructions
        .iter()
        .map(|instruction| instruction.program_id)
        .collect::<HashSet<_>>();
    for meta in instructions
        .iter()
        .flat_map(|instruction| instruction.accounts.iter())
    {
        if meta.pubkey == sysvar::instructions::ID
            || meta.pubkey == system_program::ID
            || meta.pubkey == token::id()
            || meta.pubkey == host::id()
            || meta.pubkey == receiver::id()
            || meta.pubkey == spl_token::id()
            || meta.pubkey == ed25519_program::ID
            || program_ids.contains(&meta.pubkey)
        {
            continue;
        }
        account_map
            .entry(meta.pubkey)
            .or_insert_with(|| system_account(0));
    }
    let accounts: Vec<(Pubkey, Account)> = account_map.into_iter().collect();
    let result = context
        .mollusk
        .process_transaction_instructions(instructions, &accounts);
    if result.raw_result.is_ok() {
        let mut store = context.account_store.borrow_mut();
        for (pubkey, account) in result.resulting_accounts.iter() {
            store.insert(*pubkey, account.clone());
        }
    }
    result
}

fn seed_material_commitment_for_acl(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    acl_record_address: Pubkey,
    seed: u8,
) {
    let mut acl_record = read_acl_record(context, acl_record_address);
    let (material_commitment, bump) = host::handle_material_address(acl_record_address);
    let key_id = [seed; 32];
    let ciphertext_digest = [seed.wrapping_add(1); 32];
    let sns_ciphertext_digest = [seed.wrapping_add(2); 32];
    let coprocessor_set_digest = [seed.wrapping_add(3); 32];
    let material_commitment_hash = host::handle_material_commitment_hash(
        material_commitment,
        acl_record_address,
        key_id,
        ciphertext_digest,
        sns_ciphertext_digest,
        coprocessor_set_digest,
    );
    acl_record.material_commitment = material_commitment;
    acl_record.material_commitment_hash = material_commitment_hash;
    acl_record.material_key_id = key_id;
    seed_account(
        context,
        acl_record_address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(acl_record.clone()),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    );
    seed_account(
        context,
        material_commitment,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HandleMaterialCommitment {
                acl_record: acl_record_address,
                handle: acl_record.handle,
                key_id,
                ciphertext_digest,
                sns_ciphertext_digest,
                coprocessor_set_digest,
                material_commitment_hash,
                created_slot: 0,
                state: host::HANDLE_MATERIAL_STATE_COMMITTED,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    );
}

fn seed_disclosable_amount_acl(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    handle: [u8; 32],
) -> Pubkey {
    seed_amount_acl_with_subject_entries(
        context,
        fixture,
        fixture.alice_token,
        token::transferred_amount_label(),
        handle,
        &[
            host::AclSubjectEntry::user(fixture.owner),
            host::AclSubjectEntry::compute(fixture.compute_signer),
        ],
    )
}

fn seed_amount_acl_with_subject_entries(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    entries: &[host::AclSubjectEntry],
) -> Pubkey {
    let amount_acl = token_acl_address(
        fixture.mint,
        app_account,
        encrypted_value_label,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    seed_account(
        context,
        amount_acl,
        acl_record_account(
            handle,
            token::nonce_key(fixture.mint, app_account, encrypted_value_label),
            DEFAULT_INPUT_NONCE_SEQUENCE,
            fixture.mint,
            app_account,
            encrypted_value_label,
            entries,
        ),
    );
    amount_acl
}

fn precompile_account() -> Account {
    Account {
        lamports: 1,
        data: Vec::new(),
        owner: native_loader::ID,
        executable: true,
        rent_epoch: 0,
    }
}

fn seed_empty_system_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) {
    context
        .account_store
        .borrow_mut()
        .insert(address, system_account(0));
}

fn seed_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
    account: Account,
) {
    context.account_store.borrow_mut().insert(address, account);
}

fn seed_callback_success_acl(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    address: Pubkey,
    handle: [u8; 32],
) {
    seed_account(
        context,
        address,
        acl_record_account(
            handle,
            token::nonce_key(
                fixture.mint,
                fixture.bob_owner,
                token::callback_success_label(),
            ),
            DEFAULT_INPUT_NONCE_SEQUENCE,
            fixture.mint,
            fixture.bob_owner,
            token::callback_success_label(),
            &[host::AclSubjectEntry::compute(fixture.compute_signer)],
        ),
    );
}

fn disclosure_request_address(
    fixture: &TokenMolluskFixture,
    requester: Pubkey,
    handle: [u8; 32],
    request_nonce: [u8; 32],
) -> Pubkey {
    token::disclosure_request_address(fixture.mint, requester, handle, request_nonce).0
}

fn burn_redemption_request_address(
    fixture: &TokenMolluskFixture,
    burned_handle: [u8; 32],
    request_nonce: [u8; 32],
) -> Pubkey {
    token::burn_redemption_request_address(
        fixture.mint,
        fixture.owner,
        burned_handle,
        request_nonce,
    )
    .0
}

fn token_balance_acl_address(mint: Pubkey, token_account: Pubkey, nonce_sequence: u64) -> Pubkey {
    token_acl_address(mint, token_account, token::balance_label(), nonce_sequence)
}

fn token_total_supply_acl_address(
    mint: Pubkey,
    total_supply_authority: Pubkey,
    nonce_sequence: u64,
) -> Pubkey {
    token_acl_address(
        mint,
        total_supply_authority,
        token::total_supply_label(),
        nonce_sequence,
    )
}

fn amount_acl_address(mint: Pubkey, owner: Pubkey, nonce_sequence: u64) -> Pubkey {
    token_acl_address(mint, owner, token::transfer_amount_label(), nonce_sequence)
}

fn burn_amount_acl_address(mint: Pubkey, owner: Pubkey, nonce_sequence: u64) -> Pubkey {
    token_acl_address(mint, owner, token::burn_amount_label(), nonce_sequence)
}

fn callback_success_acl_address(
    mint: Pubkey,
    callback_authority: Pubkey,
    nonce_sequence: u64,
) -> Pubkey {
    token_acl_address(
        mint,
        callback_authority,
        token::callback_success_label(),
        nonce_sequence,
    )
}

fn callback_requested_refund_acl_address(
    fixture: &TokenMolluskFixture,
    nonce_sequence: u64,
) -> Pubkey {
    token_acl_address(
        fixture.mint,
        fixture.bob_token,
        callback_refund_request_label(),
        nonce_sequence,
    )
}

fn callback_refund_request_label() -> [u8; 32] {
    *b"callback_refund_request_________"
}

struct PredictedWrapHandles {
    amount: [u8; 32],
    balance: [u8; 32],
    total_supply: [u8; 32],
}

fn predicted_wrap_handles(
    fixture: &TokenMolluskFixture,
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    amount: u64,
    old_balance_handle: [u8; 32],
    old_total_supply_handle: [u8; 32],
    amount_nonce_sequence: u64,
    _unused: u64,
) -> PredictedWrapHandles {
    let mut amount_context = [0u8; 32];
    amount_context[24..].copy_from_slice(&amount.to_be_bytes());
    let previous_bank_hash = previous_bank_hash(context);
    let unix_timestamp = context.mollusk.sysvars.clock.unix_timestamp;

    // Balance leg: its own single-domain transient eval (trivial-encrypt then add).
    let balance_context_id = transfer_eval_context(
        b"wrap-balance",
        fixture.mint,
        fixture.alice_token,
        fixture.alice_token,
        amount_context,
        amount_nonce_sequence,
        amount_nonce_sequence,
    );
    let amount_handle = host::computed_eval_trivial_handle(
        amount_plaintext(amount),
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        balance_context_id,
        0,
    );
    let balance = host::computed_eval_handle(
        host::FheBinaryOpCode::Add,
        old_balance_handle,
        amount_handle,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        balance_context_id,
        1,
    );

    // Total-supply leg: a separate single-domain transient eval (mint-scoped).
    let supply_context_id = transfer_eval_context(
        b"wrap-total-supply",
        fixture.mint,
        fixture.total_supply_authority,
        fixture.total_supply_authority,
        amount_context,
        amount_nonce_sequence,
        amount_nonce_sequence,
    );
    let supply_amount_handle = host::computed_eval_trivial_handle(
        amount_plaintext(amount),
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        supply_context_id,
        0,
    );
    let total_supply = host::computed_eval_handle(
        host::FheBinaryOpCode::Add,
        old_total_supply_handle,
        supply_amount_handle,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        supply_context_id,
        1,
    );

    PredictedWrapHandles {
        amount: amount_handle,
        balance,
        total_supply,
    }
}

struct PredictedBurnHandles {
    success: [u8; 32],
    balance: [u8; 32],
    burned: [u8; 32],
    total_supply: [u8; 32],
}

fn predicted_burn_handles(
    fixture: &TokenMolluskFixture,
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    amount_handle: [u8; 32],
    old_balance_handle: [u8; 32],
    old_total_supply_handle: [u8; 32],
    amount_nonce_sequence: u64,
    _unused: u64,
) -> PredictedBurnHandles {
    let context_id = transfer_eval_context(
        b"burn-balance",
        fixture.mint,
        fixture.alice_token,
        fixture.alice_token,
        amount_handle,
        amount_nonce_sequence,
        amount_nonce_sequence,
    );
    let previous_bank_hash = previous_bank_hash(context);
    let unix_timestamp = context.mollusk.sysvars.clock.unix_timestamp;
    let success = host::computed_eval_handle(
        host::FheBinaryOpCode::Ge,
        old_balance_handle,
        amount_handle,
        false,
        0,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        0,
    );
    let debit_candidate = host::computed_eval_handle(
        host::FheBinaryOpCode::Sub,
        old_balance_handle,
        amount_handle,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        1,
    );
    // new_balance is a transient ternary output (no nonce binding).
    let balance = host::computed_eval_ternary_handle(
        host::FheTernaryOpCode::IfThenElse,
        success,
        debit_candidate,
        old_balance_handle,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        2,
    );
    // The burned amount is the one durable (nonce-bound) balance-leg output.
    let burned = host::computed_bound_eval_handle(
        host::FheBinaryOpCode::Sub,
        old_balance_handle,
        balance,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        3,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::burned_amount_label(),
        ),
        amount_nonce_sequence,
    );
    // Total supply rotates in its own single-domain transient eval over `burned`.
    let supply_context_id = transfer_eval_context(
        b"burn-total-supply",
        fixture.mint,
        fixture.total_supply_authority,
        fixture.alice_token,
        burned,
        amount_nonce_sequence,
        amount_nonce_sequence,
    );
    let total_supply = host::computed_eval_handle(
        host::FheBinaryOpCode::Sub,
        old_total_supply_handle,
        burned,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        supply_context_id,
        0,
    );

    PredictedBurnHandles {
        success,
        balance,
        burned,
        total_supply,
    }
}

fn predicted_transfer_sent_handle(
    fixture: &TokenMolluskFixture,
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    amount_handle: [u8; 32],
) -> [u8; 32] {
    let from = read_token_account(context, fixture.alice_token);
    // The transfer eval context is keyed by the sender's amount nonce for both legs.
    let amount_nonce_sequence = from.next_amount_nonce_sequence;
    let context_id = transfer_eval_context(
        b"combined",
        fixture.mint,
        fixture.alice_token,
        fixture.bob_token,
        amount_handle,
        amount_nonce_sequence,
        amount_nonce_sequence,
    );
    let previous_bank_hash = previous_bank_hash(context);
    let unix_timestamp = context.mollusk.sysvars.clock.unix_timestamp;
    let transfer_success_handle = host::computed_eval_handle(
        host::FheBinaryOpCode::Ge,
        from.balance_handle,
        amount_handle,
        false,
        0,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        0,
    );
    let debit_candidate_handle = host::computed_eval_handle(
        host::FheBinaryOpCode::Sub,
        from.balance_handle,
        amount_handle,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        1,
    );
    // new_from is a transient ternary output (no nonce binding).
    let new_from_handle = host::computed_eval_ternary_handle(
        host::FheTernaryOpCode::IfThenElse,
        transfer_success_handle,
        debit_candidate_handle,
        from.balance_handle,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        2,
    );
    // The transferred amount is the only durable (nonce-bound) output.
    host::computed_bound_eval_handle(
        host::FheBinaryOpCode::Sub,
        from.balance_handle,
        new_from_handle,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        3,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::transferred_amount_label(),
        ),
        amount_nonce_sequence,
    )
}

fn previous_bank_hash(context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>) -> [u8; 32] {
    context
        .mollusk
        .sysvars
        .clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            context
                .mollusk
                .sysvars
                .slot_hashes
                .get(&slot)
                .map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32])
}

fn transfer_eval_context(
    tag: &[u8],
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    amount_handle: [u8; 32],
    from_nonce_sequence: u64,
    to_nonce_sequence: u64,
) -> [u8; 32] {
    let from_sequence_bytes = from_nonce_sequence.to_be_bytes();
    let to_sequence_bytes = to_nonce_sequence.to_be_bytes();
    solana_sha256_hasher::hashv(&[
        b"confidential-token-transfer-eval-v1",
        tag,
        mint.as_ref(),
        from_token_account.as_ref(),
        to_token_account.as_ref(),
        &amount_handle,
        &from_sequence_bytes,
        &to_sequence_bytes,
    ])
    .to_bytes()
}

fn token_acl_address(
    mint: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    nonce_sequence: u64,
) -> Pubkey {
    host::acl_record_address(
        token::nonce_key(mint, app_account, encrypted_value_label),
        nonce_sequence,
    )
    .0
}

fn handle_for_chain(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[21] = 0;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn amount_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn system_account(lamports: u64) -> Account {
    Account {
        lamports,
        data: Vec::new(),
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

/// A host-owned encrypted-value ACL lineage account at `current_handle`, with no
/// history (`leaf_count == 0`). Pre-seeds a balance lineage so transfer/callback
/// tests can start from already-wrapped accounts without running a wrap first.
fn balance_value_acl_account(
    mint: Pubkey,
    token_account: Pubkey,
    current_handle: [u8; 32],
    subjects: &[Pubkey],
) -> Account {
    let bump = token::balance_value_acl_address(mint, token_account).1;
    let acl = host::EncryptedValueAcl {
        acl_domain_key: mint.to_bytes(),
        app_account: token_account.to_bytes(),
        encrypted_value_label: token::balance_label(),
        current_handle,
        subjects: subjects.iter().map(Pubkey::to_bytes).collect(),
        leaf_count: 0,
        peaks: Vec::new(),
        bump,
    };
    Account {
        lamports: 1_000_000_000,
        data: host::encode_account(&acl).expect("encode lineage"),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

/// A host-owned encrypted-value ACL lineage for a mint's total supply at
/// `current_handle`, no history. Mirrors what `initialize_mint` creates so flows
/// that read the total-supply lineage as a durable input (wrap, burn) can run.
fn total_supply_value_acl_account(
    mint: Pubkey,
    total_supply_authority: Pubkey,
    current_handle: [u8; 32],
    subjects: &[Pubkey],
) -> Account {
    let bump = token::total_supply_value_acl_address(mint, total_supply_authority).1;
    let acl = host::EncryptedValueAcl {
        acl_domain_key: mint.to_bytes(),
        app_account: total_supply_authority.to_bytes(),
        encrypted_value_label: token::total_supply_label(),
        current_handle,
        subjects: subjects.iter().map(Pubkey::to_bytes).collect(),
        leaf_count: 0,
        peaks: Vec::new(),
        bump,
    };
    Account {
        lamports: 1_000_000_000,
        data: host::encode_account(&acl).expect("encode lineage"),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

// ===========================================================================
// Encrypted-value ACL + MMR history — token integration e2e (fhevm-internal#1569).
//
// The token program creates/rotates the balance lineage at the balance-handle-
// replacement seam (wrap). `warp_to_slot` populates SlotHashes so fhe_eval's
// `previous_bank_hash` resolves (the eval path does not use the test-zero fallback).
// ===========================================================================

fn read_encrypted_value_acl(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::EncryptedValueAcl> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    host::decode_account(&account.data).ok()
}

// Decrypt authorization is an off-chain (KMS) check over the shared
// `zama_solana_acl` pure functions, not an on-chain instruction. These helpers
// decode the live lineage and run the same pure gates the KMS would (mirrors
// host_mollusk.rs).
fn ev_authorize_current(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    lineage: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<(), host::AclError> {
    let acl = read_encrypted_value_acl(context, lineage).expect("lineage exists");
    host::authorize_current(&acl, handle, subject.to_bytes())
}

fn ev_authorize_historical(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    lineage: Pubkey,
    encrypted_value: [u8; 32],
    subject: Pubkey,
    proof: &host::mmr::MmrProof,
) -> Result<(), host::AclError> {
    let acl = read_encrypted_value_acl(context, lineage).expect("lineage exists");
    host::authorize_historical(
        lineage.to_bytes(),
        &acl,
        encrypted_value,
        subject.to_bytes(),
        proof,
    )
}

#[test]
fn mollusk_token_wrap_creates_and_authorizes_balance_value_acl() {
    let fixture = TokenMolluskFixture::new();
    let mut context = fixture.context_with_wrap_accounts();
    context.mollusk.sysvars.warp_to_slot(10);

    let lineage = token::balance_value_acl_address(fixture.mint, fixture.alice_token).0;
    // The balance lineage exists (initialize_token_account created it) at the
    // initial handle, with no history yet.
    fixture.seed_wrap_lineages(&context);
    let before = read_encrypted_value_acl(&context, lineage).expect("lineage seeded");
    assert_eq!(before.current_handle, fixture.alice_initial);
    assert_eq!(before.leaf_count, 0);

    let amount = 100_000_000;
    let output = WrapOutputAccounts::canonical(&fixture, 1);
    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, output, amount),
        &[Check::success()],
    );

    // Wrap rotated the balance lineage: new handle bound, an access leaf appended
    // per durable subject (owner, compute_signer) for the old handle.
    let token_account = read_token_account(&context, fixture.alice_token);
    let acl = read_encrypted_value_acl(&context, lineage).expect("wrap rotated the lineage");
    assert_eq!(acl.current_handle, token_account.balance_handle);
    assert_ne!(acl.current_handle, fixture.alice_initial);
    assert_eq!(acl.leaf_count, 2);
    assert_eq!(acl.acl_domain_key, fixture.mint.to_bytes());
    assert_eq!(acl.app_account, fixture.alice_token.to_bytes());
    assert_eq!(acl.subjects.len(), 2);
    assert!(acl.is_subject(fixture.owner.to_bytes()));
    assert!(acl.is_subject(fixture.compute_signer.to_bytes()));

    // Current-decrypt gate authorizes the owner, rejects a non-subject.
    assert_eq!(
        ev_authorize_current(
            &context,
            lineage,
            token_account.balance_handle,
            fixture.owner
        ),
        Ok(())
    );
    let stranger = Pubkey::new_unique();
    assert_eq!(
        ev_authorize_current(&context, lineage, token_account.balance_handle, stranger),
        Err(host::AclError::SubjectMissing)
    );
}

/// Wrap ix variant that takes explicit current balance/supply ACL records, so a
/// second wrap can chain off the first wrap's outputs.
fn wrap_usdc_ix_chained(
    fixture: &TokenMolluskFixture,
    output: WrapOutputAccounts,
    amount: u64,
    _balance_value_acl: Pubkey,
    _total_supply_value_acl: Pubkey,
) -> Instruction {
    wrap_usdc_ix(fixture, output, amount)
}

#[test]
fn mollusk_token_double_wrap_rotates_lineage_with_history() {
    let fixture = TokenMolluskFixture::new();
    let mut context = fixture.context_with_wrap_accounts();
    context.mollusk.sysvars.warp_to_slot(10);

    fixture.seed_wrap_lineages(&context);
    let wrap1 = WrapOutputAccounts::canonical(&fixture, 1);
    let wrap2 = WrapOutputAccounts::canonical(&fixture, 2);
    {
        let mut store = context.account_store.borrow_mut();
        for account in wrap2.all_accounts() {
            store.entry(account).or_insert_with(|| system_account(0));
        }
    }
    let lineage = token::balance_value_acl_address(fixture.mint, fixture.alice_token).0;
    let h0 = fixture.alice_initial;

    // Wrap 1: rotates the seeded lineage H0->H1, appending an access leaf per
    // durable subject (owner, compute_signer) for the OLD handle H0 (leaves 0,1).
    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, wrap1, 50_000_000),
        &[Check::success()],
    );
    let acl1 = read_encrypted_value_acl(&context, lineage).expect("lineage rotated");
    let h1 = acl1.current_handle;
    assert_ne!(h0, h1);
    assert_eq!(acl1.leaf_count, 2);

    // Wrap 2: rotates the lineage H1->H2, appending an access leaf per durable
    // subject for the OLD handle H1 (leaves 2,3).
    context.process_and_validate_instruction(
        &wrap_usdc_ix_chained(
            &fixture,
            wrap2,
            50_000_000,
            wrap1.balance,
            wrap1.total_supply,
        ),
        &[Check::success()],
    );
    let acl2 = read_encrypted_value_acl(&context, lineage).expect("lineage rotated");
    let h2 = acl2.current_handle;
    assert_ne!(h1, h2);
    assert_eq!(acl2.leaf_count, 4);

    // Current decrypt: H2 authorized for owner; old H1 rejected by live state.
    assert_eq!(
        ev_authorize_current(&context, lineage, h2, fixture.owner),
        Ok(())
    );
    assert_eq!(
        ev_authorize_current(&context, lineage, h1, fixture.owner),
        Err(host::AclError::HandleMismatch)
    );

    // Historical decrypt of the old value H1 succeeds only with a valid MMR proof.
    // H1's access leaves were appended by wrap 2 at indices 2 (owner) and 3 (compute).
    let leaves = vec![
        host::historical_access_leaf_commitment(
            lineage.to_bytes(),
            0,
            h0,
            fixture.owner.to_bytes(),
        ),
        host::historical_access_leaf_commitment(
            lineage.to_bytes(),
            1,
            h0,
            fixture.compute_signer.to_bytes(),
        ),
        host::historical_access_leaf_commitment(
            lineage.to_bytes(),
            2,
            h1,
            fixture.owner.to_bytes(),
        ),
        host::historical_access_leaf_commitment(
            lineage.to_bytes(),
            3,
            h1,
            fixture.compute_signer.to_bytes(),
        ),
    ];
    let proof = host::mmr::mmr_build_proof(&leaves, 2).unwrap();
    assert_eq!(
        ev_authorize_historical(&context, lineage, h1, fixture.owner, &proof),
        Ok(())
    );
    // Wrong subject is rejected.
    assert_eq!(
        ev_authorize_historical(&context, lineage, h1, Pubkey::new_unique(), &proof),
        Err(host::AclError::HistoricalProofInvalid)
    );
}

/// The headline replacement invariant (fhevm-internal#1569): a balance/total_supply
/// rotation reuses one lineage account per value and mints NO per-rotation ACL
/// record, so the on-chain account population is flat across consecutive rotations.
/// The pre-refactor design minted a fresh keyed-nonce `AclRecord` for the balance
/// and the total supply on every wrap, growing the account set by two each time.
#[test]
fn mollusk_wrap_rotation_does_not_mint_new_acl_accounts() {
    let fixture = TokenMolluskFixture::new();
    let mut context = fixture.context_with_wrap_accounts();
    context.mollusk.sysvars.warp_to_slot(10);

    // Both lineages pre-exist (as initialize_token_account / initialize_mint leave
    // them); a wrap rotates them in place.
    fixture.seed_wrap_lineages(&context);
    let wrap1 = WrapOutputAccounts::canonical(&fixture, 1);
    let wrap2 = WrapOutputAccounts::canonical(&fixture, 2);
    {
        let mut store = context.account_store.borrow_mut();
        for account in wrap1.all_accounts().into_iter().chain(wrap2.all_accounts()) {
            store.entry(account).or_insert_with(|| system_account(0));
        }
    }

    let balance_lineage = token::balance_value_acl_address(fixture.mint, fixture.alice_token).0;
    let supply_lineage =
        token::total_supply_value_acl_address(fixture.mint, fixture.total_supply_authority).0;
    // Account-population snapshot: total live accounts, and the host-owned (ACL) subset.
    let snapshot =
        |context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>| -> (usize, usize) {
            let store = context.account_store.borrow();
            let total = store
                .values()
                .filter(|account| account.lamports > 0)
                .count();
            let host_owned = store
                .values()
                .filter(|account| account.owner == host::ID && account.lamports > 0)
                .count();
            (total, host_owned)
        };

    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, wrap1, 50_000_000),
        &[Check::success()],
    );
    let after_wrap1 = snapshot(&context);
    let balance_handle_1 = read_encrypted_value_acl(&context, balance_lineage)
        .expect("balance lineage")
        .current_handle;

    context.process_and_validate_instruction(
        &wrap_usdc_ix_chained(
            &fixture,
            wrap2,
            50_000_000,
            wrap1.balance,
            wrap1.total_supply,
        ),
        &[Check::success()],
    );
    let after_wrap2 = snapshot(&context);
    let balance_handle_2 = read_encrypted_value_acl(&context, balance_lineage)
        .expect("balance lineage")
        .current_handle;

    // The second rotation added ZERO accounts (total and host-owned counts flat) —
    // proof that no per-rotation AclRecord is minted.
    assert_eq!(
        after_wrap1, after_wrap2,
        "a balance/total_supply rotation must not grow the account population"
    );
    // ...yet the value did rotate: same lineage PDA, new handle.
    assert_ne!(balance_handle_1, balance_handle_2);
    // The only durable ACL accounts for these values are the two reused lineages.
    assert!(read_encrypted_value_acl(&context, balance_lineage).is_some());
    assert!(read_encrypted_value_acl(&context, supply_lineage).is_some());
}

/// End-to-end lifecycle across the REAL token + host programs: a balance lineage
/// is rotated three times (wrap, wrap, burn) and after every rotation the
/// off-chain `authorize_*` decisions (the exact pure fns the KMS calls) are run
/// against the on-chain lineage state the programs actually produced. This is the
/// on-chain→off-chain seam of the PoC: handles produced by program execution,
/// read back and authorized by the shared crate.
///
/// Proves, on program-produced state: current decrypt for the live handle; the
/// previous handle rejected by live state at EVERY rotation including the burn;
/// historical decrypt of any superseded handle only with a valid MMR proof (wrong
/// subject rejected); the oldest handle still provable after all three rotations
/// (the early MMR mountain survives); and MMR-absent (leaf_count==0) rejecting
/// proofs while current works.
#[test]
fn mollusk_token_lineage_lifecycle_authorizes_across_rotations_e2e() {
    let fixture = TokenMolluskFixture::new();
    let burn_amount_handle = handle_for_chain(52, BALANCE_FHE_TYPE);
    let mut context = fixture.context_with_wrap_and_burn_amount(burn_amount_handle);
    context.mollusk.sysvars.warp_to_slot(10);

    let wrap1 = WrapOutputAccounts::canonical(&fixture, 1);
    let wrap2 = WrapOutputAccounts::canonical(&fixture, 2);
    let burn_output = BurnOutputAccounts::canonical(&fixture, 0, 0);
    {
        let mut store = context.account_store.borrow_mut();
        for account in burn_output.all_accounts() {
            store.entry(account).or_insert_with(|| system_account(0));
        }
    }

    let balance = token::balance_value_acl_address(fixture.mint, fixture.alice_token).0;
    let owner = fixture.owner;
    let compute = fixture.compute_signer;
    let stranger = Pubkey::new_unique();
    let leaf = |idx: u64, old: [u8; 32], subject: Pubkey| {
        host::historical_access_leaf_commitment(balance.to_bytes(), idx, old, subject.to_bytes())
    };

    // Post-init state (exactly what initialize_token_account leaves; seeded here):
    // current decrypt works for the owner, but there is no history yet, so any
    // historical proof is rejected (leaf_count == 0).
    fixture.seed_wrap_lineages(&context);
    let h0 = fixture.alice_initial;
    assert_eq!(ev_authorize_current(&context, balance, h0, owner), Ok(()));
    assert_eq!(
        ev_authorize_current(&context, balance, h0, stranger),
        Err(host::AclError::SubjectMissing)
    );
    let empty = read_encrypted_value_acl(&context, balance).unwrap();
    assert_eq!(empty.leaf_count, 0);
    let empty_proof = host::mmr::mmr_build_proof(&[leaf(0, h0, owner)], 0).unwrap();
    assert!(ev_authorize_historical(&context, balance, h0, owner, &empty_proof).is_err());

    // Rotation 1 — wrap: H0 -> H1, appends access leaves 0 (owner) and 1 (compute)
    // for the OLD handle H0.
    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, wrap1, 50_000_000),
        &[Check::success()],
    );
    let h1 = read_encrypted_value_acl(&context, balance)
        .unwrap()
        .current_handle;
    assert_ne!(h0, h1);
    assert_eq!(ev_authorize_current(&context, balance, h1, owner), Ok(()));
    // Old value rejected by LIVE state (no proof) immediately after rotation.
    assert_eq!(
        ev_authorize_current(&context, balance, h0, owner),
        Err(host::AclError::HandleMismatch)
    );

    // Rotation 2 — wrap: H1 -> H2, appends leaves 2 (owner) and 3 (compute) for H1.
    context.process_and_validate_instruction(
        &wrap_usdc_ix_chained(
            &fixture,
            wrap2,
            50_000_000,
            wrap1.balance,
            wrap1.total_supply,
        ),
        &[Check::success()],
    );
    let h2 = read_encrypted_value_acl(&context, balance)
        .unwrap()
        .current_handle;
    assert_ne!(h1, h2);
    assert_eq!(ev_authorize_current(&context, balance, h2, owner), Ok(()));
    assert_eq!(
        ev_authorize_current(&context, balance, h1, owner),
        Err(host::AclError::HandleMismatch)
    );

    // Rotation 3 — burn: H2 -> H3, appends leaves 4 (owner) and 5 (compute) for H2.
    context.process_and_validate_instruction(
        &burn_ix(
            &fixture,
            balance,
            wrap2.total_supply,
            burn_output,
            burn_amount_handle,
        ),
        &[Check::success()],
    );
    let h3 = read_encrypted_value_acl(&context, balance)
        .unwrap()
        .current_handle;
    assert_ne!(h2, h3);

    // Final live state: only H3 currently decrypts; H2 (the pre-burn balance) is
    // revoked by live state — the burn path enforces revocation just like wraps.
    let final_acl = read_encrypted_value_acl(&context, balance).unwrap();
    assert_eq!(final_acl.leaf_count, 6);
    assert_eq!(ev_authorize_current(&context, balance, h3, owner), Ok(()));
    assert_eq!(
        ev_authorize_current(&context, balance, h2, owner),
        Err(host::AclError::HandleMismatch)
    );

    // Historical decrypt of every superseded handle, each provable only with the
    // valid MMR proof for its own access leaf, reconstructed from the full leaf set
    // (what an off-chain proof service would keep). Order mirrors the rotation order.
    let leaves = vec![
        leaf(0, h0, owner),
        leaf(1, h0, compute),
        leaf(2, h1, owner),
        leaf(3, h1, compute),
        leaf(4, h2, owner),
        leaf(5, h2, compute),
    ];
    let proof = |idx: u64| host::mmr::mmr_build_proof(&leaves, idx).unwrap();

    // The OLDEST handle H0 is still provable after all three rotations (its early
    // MMR mountain survives the later appends).
    assert_eq!(
        ev_authorize_historical(&context, balance, h0, owner, &proof(0)),
        Ok(())
    );
    assert_eq!(
        ev_authorize_historical(&context, balance, h0, compute, &proof(1)),
        Ok(())
    );
    assert_eq!(
        ev_authorize_historical(&context, balance, h1, owner, &proof(2)),
        Ok(())
    );
    // The pre-burn handle H2's access leaf was appended by the burn rotation.
    assert_eq!(
        ev_authorize_historical(&context, balance, h2, owner, &proof(4)),
        Ok(())
    );

    // Wrong subject for a real leaf is rejected (the subject is bound into the leaf).
    assert_eq!(
        ev_authorize_historical(&context, balance, h0, stranger, &proof(0)),
        Err(host::AclError::HistoricalProofInvalid)
    );
    // A proof for one handle's leaf does not authorize a different handle.
    assert_eq!(
        ev_authorize_historical(&context, balance, h1, owner, &proof(0)),
        Err(host::AclError::HistoricalProofInvalid)
    );
}
