#[allow(dead_code)]
mod support;

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, AnchorDeserialize,
    AnchorSerialize, Discriminator, InstructionData, ToAccountMetas,
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
    signature::Keypair,
    signature::Signer,
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
    let output = SelfTransferOutputAccounts::canonical(&fixture, 1);
    let ix = self_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    assert!(result.inner_instructions.is_empty());
    let token_account = read_token_account(&context, fixture.alice_token);
    assert_eq!(token_account.balance_handle, fixture.alice_initial);
    assert_eq!(
        token_account.balance_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(token_account.next_balance_nonce_sequence, 1);
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
    assert_eq!(
        token_account.balance_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(token_account.next_balance_nonce_sequence, 1);
    assert_empty_system_account(&context, output.alice);
    assert_empty_system_account(&context, output.to_output);
}

#[test]
fn mollusk_confidential_transfer_rotates_accounts_and_acl_records() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let ix = direct_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
    let acl_records_before = acl_record_count(&context);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    assert!(!result.inner_instructions.is_empty());
    assert_transfer_created_exactly_three_acl_records(&context, acl_records_before, output);
    assert_transfer_scratch_acl_records_absent(&context, &fixture, 1);
    let alice_token = read_token_account(&context, fixture.alice_token);
    let bob_token = read_token_account(&context, fixture.bob_token);
    let from_output_acl = read_acl_record(&context, output.from_output);
    let transferred_acl = read_acl_record(&context, output.transferred);
    let to_output_acl = read_acl_record(&context, output.to_output);

    assert_eq!(alice_token.balance_acl_record, output.from_output);
    assert_eq!(alice_token.balance_handle, from_output_acl.handle);
    assert_ne!(alice_token.balance_handle, fixture.alice_initial);
    assert_eq!(alice_token.next_balance_nonce_sequence, 2);
    assert_eq!(bob_token.balance_acl_record, output.to_output);
    assert_eq!(bob_token.balance_handle, to_output_acl.handle);
    assert_ne!(bob_token.balance_handle, fixture.bob_initial);
    assert_eq!(bob_token.next_balance_nonce_sequence, 2);

    assert_acl_record(
        &from_output_acl,
        token::balance_nonce_key(fixture.mint, fixture.alice_token),
        1,
        fixture.mint,
        fixture.alice_token,
        token::balance_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert_acl_record(
        &transferred_acl,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::transferred_amount_label(),
        ),
        1,
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
    assert_acl_record(
        &to_output_acl,
        token::balance_nonce_key(fixture.mint, fixture.bob_token),
        1,
        fixture.mint,
        fixture.bob_token,
        token::balance_label(),
        BALANCE_FHE_TYPE,
        &[
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
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let ix = direct_transfer_ix_with_payer(&fixture, payer, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
    seed_account(&context, payer, system_account(5_000_000_000));
    let acl_records_before = acl_record_count(&context);

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    assert_transfer_created_exactly_three_acl_records(&context, acl_records_before, output);
    let alice_token = read_token_account(&context, fixture.alice_token);
    let transferred_acl = read_acl_record(&context, output.transferred);
    assert_eq!(alice_token.balance_acl_record, output.from_output);
    assert_eq!(transferred_acl.app_account, fixture.alice_token);
    assert!(transferred_acl.inline_subject_index(payer).is_none());
}

#[test]
fn mollusk_confidential_transfer_rejects_payer_scoped_amount_acl() {
    let fixture = TokenMolluskFixture::new();
    let payer = Pubkey::new_unique();
    let amount_handle = handle_for_chain(30, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let ix = direct_transfer_ix_with_acls(
        &fixture,
        payer,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        amount_acl_address(fixture.mint, payer, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    );
    let context = fixture.context_with_input_amount_for_authority(amount_handle, payer);
    seed_account(&context, payer, system_account(5_000_000_000));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_direct_transfer_failure_preserved_state(
        &context,
        &fixture,
        fixture.alice_initial,
        fixture.alice_current_compute_acl,
        1,
        fixture.bob_initial,
        fixture.bob_current_compute_acl,
        1,
        output,
    );
}

#[test]
fn mollusk_confidential_transfer_replays_token_event_cpis() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(22, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let ix = direct_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let from_output_acl = read_acl_record(&context, output.from_output);
    let transferred_acl = read_acl_record(&context, output.transferred);
    let to_output_acl = read_acl_record(&context, output.to_output);
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
    assert_eq!(
        balance_events[0].old_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(balance_events[0].new_handle, from_output_acl.handle);
    assert_eq!(balance_events[0].new_acl_record, output.from_output);
    assert_eq!(
        balance_events[1].reason,
        token::BalanceHandleUpdateReason::TransferCredit
    );
    assert_eq!(balance_events[1].version, token::APP_EVENT_VERSION);
    assert_eq!(balance_events[1].mint, fixture.mint);
    assert_eq!(balance_events[1].owner, fixture.bob_owner);
    assert_eq!(balance_events[1].token_account, fixture.bob_token);
    assert_eq!(balance_events[1].old_handle, fixture.bob_initial);
    assert_eq!(
        balance_events[1].old_acl_record,
        fixture.bob_current_compute_acl
    );
    assert_eq!(balance_events[1].new_handle, to_output_acl.handle);
    assert_eq!(balance_events[1].new_acl_record, output.to_output);
}

#[test]
fn mollusk_confidential_transfer_rejects_stale_current_acl_without_creating_outputs() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(95, BALANCE_FHE_TYPE);
    let first_output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let stale_output = DirectTransferOutputAccounts::canonical(&fixture, 2, 2);
    let context = fixture.context_with_input_amount(amount_handle);
    let first_ix = direct_transfer_ix(&fixture, first_output, amount_handle);

    context.process_and_validate_instruction(&first_ix, &[Check::success()]);
    for account in stale_output.all_accounts() {
        seed_empty_system_account(&context, account);
    }
    let alice_after_first = read_token_account(&context, fixture.alice_token);
    let bob_after_first = read_token_account(&context, fixture.bob_token);
    let stale_ix = direct_transfer_ix_with_acls(
        &fixture,
        fixture.owner,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        amount_acl_address(fixture.mint, fixture.owner, DEFAULT_INPUT_NONCE_SEQUENCE),
        stale_output,
        amount_handle,
    );

    let result = context.process_instruction(&stale_ix);

    assert!(result.raw_result.is_err());
    assert_direct_transfer_failure_preserved_state(
        &context,
        &fixture,
        alice_after_first.balance_handle,
        alice_after_first.balance_acl_record,
        alice_after_first.next_balance_nonce_sequence,
        bob_after_first.balance_handle,
        bob_after_first.balance_acl_record,
        bob_after_first.next_balance_nonce_sequence,
        stale_output,
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_wrong_amount_acl_label_without_creating_outputs() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(96, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
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
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
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
        fixture.alice_current_compute_acl,
        1,
        fixture.bob_initial,
        fixture.bob_current_compute_acl,
        1,
        output,
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_output_acl_for_wrong_token_account_without_creating_outputs(
) {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(97, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let wrong_output = DirectTransferOutputAccounts {
        from_output: output.to_output,
        to_output: output.from_output,
        ..output
    };
    let ix = direct_transfer_ix(&fixture, wrong_output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_direct_transfer_failure_preserved_state(
        &context,
        &fixture,
        fixture.alice_initial,
        fixture.alice_current_compute_acl,
        1,
        fixture.bob_initial,
        fixture.bob_current_compute_acl,
        1,
        wrong_output,
    );
}

#[test]
fn mollusk_request_disclose_balance_marks_current_balance_public_decrypt() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 120);
    let ix = request_disclose_balance_ix(&fixture);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let record = read_acl_record(&context, fixture.alice_current_compute_acl);
    assert_eq!(record.handle, fixture.alice_initial);
    assert_eq!(record.app_account, fixture.alice_token);
    assert!(record.public_decrypt);

    let public_events: Vec<host::PublicDecryptAllowedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let token_events: Vec<token::BalanceDisclosureRequestedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(public_events.len(), 1);
    assert_eq!(
        public_events[0].acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(public_events[0].handle, fixture.alice_initial);
    assert_eq!(public_events[0].authority, fixture.owner.to_bytes());

    assert_eq!(token_events.len(), 1);
    assert_eq!(token_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(token_events[0].mint, fixture.mint);
    assert_eq!(token_events[0].owner, fixture.owner);
    assert_eq!(token_events[0].token_account, fixture.alice_token);
    assert_eq!(token_events[0].handle, fixture.alice_initial);
    assert_eq!(
        token_events[0].acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(
        token_events[0].request,
        disclosure_request_address(
            &fixture,
            fixture.owner,
            fixture.alice_initial,
            request_nonce(1)
        )
    );
    assert_disclosure_request(
        &context,
        token_events[0].request,
        &fixture,
        token::DISCLOSURE_REQUEST_MODE_BALANCE,
        fixture.alice_token,
        fixture.alice_token,
        fixture.alice_initial,
        fixture.alice_current_compute_acl,
        request_nonce(1),
    );
}

#[test]
fn mollusk_request_disclose_balance_is_idempotent_without_duplicate_host_event() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 120);

    let first_result = process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);

    assert!(first_result.raw_result.is_ok());
    assert!(read_acl_record(&context, fixture.alice_current_compute_acl).public_decrypt);
    let first_public_events: Vec<host::PublicDecryptAllowedEvent> = first_result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(first_public_events.len(), 1);
    assert_eq!(
        first_public_events[0].acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(first_public_events[0].handle, fixture.alice_initial);
    assert_eq!(first_public_events[0].authority, fixture.owner.to_bytes());

    let second_result = process_transaction(
        &context,
        &[request_disclose_balance_ix_with_nonce(
            &fixture,
            request_nonce(2),
        )],
    );

    assert!(second_result.raw_result.is_ok());
    assert!(read_acl_record(&context, fixture.alice_current_compute_acl).public_decrypt);
    let second_public_events: Vec<host::PublicDecryptAllowedEvent> = second_result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let second_token_events: Vec<token::BalanceDisclosureRequestedEvent> = second_result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert!(second_public_events.is_empty());
    assert_eq!(second_token_events.len(), 1);
    assert_eq!(second_token_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(second_token_events[0].mint, fixture.mint);
    assert_eq!(second_token_events[0].owner, fixture.owner);
    assert_eq!(second_token_events[0].token_account, fixture.alice_token);
    assert_eq!(second_token_events[0].handle, fixture.alice_initial);
    assert_eq!(
        second_token_events[0].acl_record,
        fixture.alice_current_compute_acl
    );
}

#[test]
fn mollusk_disclose_balance_accepts_kms_signed_cleartext() {
    let fixture = TokenMolluskFixture::new();
    let mut context = mollusk().with_context(fixture.base_accounts());
    let cleartext_amount = 125;
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 120);
    let request_ix = request_disclose_balance_ix(&fixture);

    let request_result = process_transaction(&context, &[request_ix]);
    assert!(request_result.raw_result.is_ok());

    let result = process_transaction(
        &context,
        &[
            disclosure_ed25519_ix(&context, &fixture, fixture.alice_initial, cleartext_amount),
            disclose_balance_ix(&fixture, cleartext_amount),
        ],
    );

    assert!(result.raw_result.is_ok());
    let events: Vec<token::BalanceDisclosedEvent> = result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(events[0].mint, fixture.mint);
    assert_eq!(events[0].owner, fixture.owner);
    assert_eq!(events[0].token_account, fixture.alice_token);
    assert_eq!(events[0].handle, fixture.alice_initial);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
    let request = disclosure_request_address(
        &fixture,
        fixture.owner,
        fixture.alice_initial,
        request_nonce(1),
    );
    assert_eq!(
        read_disclosure_request(&context, request).status,
        token::REQUEST_STATUS_CONSUMED
    );
    let owner_before = account_lamports(&context, fixture.owner);
    context
        .mollusk
        .sysvars
        .warp_to_slot(DEFAULT_REQUEST_EXPIRES_SLOT + 1);
    let close_result = context.process_and_validate_instruction(
        &close_consumed_disclosure_request_ix(&fixture, request),
        &[Check::success()],
    );
    assert!(close_result.raw_result.is_ok());
    assert!(!disclosure_request_exists(&context, request));
    assert!(account_lamports(&context, fixture.owner) > owner_before);
}

#[test]
fn mollusk_disclose_balance_accepts_requested_handle_after_balance_rotation() {
    let fixture = TokenMolluskFixture::new();
    let context = fixture.context_with_wrap_accounts();
    let cleartext_amount = 125;
    let wrap_amount = 100_000_000;
    let wrap_output = WrapOutputAccounts::canonical(&fixture, 1);
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 120);
    let request_ix = request_disclose_balance_ix(&fixture);

    let request_result = process_transaction(&context, &[request_ix]);
    assert!(request_result.raw_result.is_ok());
    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, wrap_output, wrap_amount),
        &[Check::success()],
    );
    let token_account = read_token_account(&context, fixture.alice_token);
    assert_eq!(token_account.balance_acl_record, wrap_output.balance);
    assert_ne!(token_account.balance_handle, fixture.alice_initial);

    let result = process_transaction(
        &context,
        &[
            disclosure_ed25519_ix(&context, &fixture, fixture.alice_initial, cleartext_amount),
            disclose_balance_ix(&fixture, cleartext_amount),
        ],
    );

    assert!(result.raw_result.is_ok());
    let events: Vec<token::BalanceDisclosedEvent> = result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].handle, fixture.alice_initial);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
}

#[test]
fn mollusk_disclose_amount_accepts_kms_signed_cleartext() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(23, BALANCE_FHE_TYPE);
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_acl = seed_disclosable_amount_acl(&context, &fixture, amount_handle);
    let cleartext_amount = 77;
    seed_material_commitment_for_acl(&context, amount_acl, 121);

    let request_result = process_transaction(
        &context,
        &[request_disclose_amount_ix(
            &fixture,
            amount_acl,
            amount_handle,
        )],
    );
    assert!(request_result.raw_result.is_ok());

    let result = process_transaction(
        &context,
        &[
            disclosure_ed25519_ix(&context, &fixture, amount_handle, cleartext_amount),
            disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount),
        ],
    );

    assert!(result.raw_result.is_ok());
    let request_record = read_acl_record(&context, amount_acl);
    assert!(request_record.public_decrypt);
    let events: Vec<token::AmountDisclosedEvent> = result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(events[0].mint, fixture.mint);
    assert_eq!(events[0].handle, amount_handle);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
    let request =
        disclosure_request_address(&fixture, fixture.owner, amount_handle, request_nonce(1));
    assert_eq!(
        read_disclosure_request(&context, request).status,
        token::REQUEST_STATUS_CONSUMED
    );
}

#[test]
fn mollusk_close_expired_disclosure_request_returns_rent_to_requester() {
    let fixture = TokenMolluskFixture::new();
    let mut context = mollusk().with_context(fixture.base_accounts());
    let request = disclosure_request_address(
        &fixture,
        fixture.owner,
        fixture.alice_initial,
        request_nonce(8),
    );
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 128);
    let request_result = process_transaction(
        &context,
        &[request_disclose_balance_ix_with_nonce_and_expires(
            &fixture,
            request_nonce(8),
            5,
        )],
    );
    assert!(request_result.raw_result.is_ok());
    assert!(disclosure_request_exists(&context, request));
    let owner_before = account_lamports(&context, fixture.owner);

    context.mollusk.sysvars.warp_to_slot(6);
    let result = context.process_and_validate_instruction(
        &close_expired_disclosure_request_ix(&fixture, request),
        &[Check::success()],
    );

    assert!(result.raw_result.is_ok());
    assert!(!disclosure_request_exists(&context, request));
    assert!(account_lamports(&context, fixture.owner) > owner_before);
}

#[test]
fn mollusk_request_disclose_balance_rejects_wrong_owner() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 122);
    let ix = request_disclose_balance_ix_with_owner(&fixture, fixture.bob_owner);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, fixture.alice_current_compute_acl);
    assert!(!record.public_decrypt);
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
fn mollusk_disclose_amount_rejects_without_public_decrypt_release() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(25, BALANCE_FHE_TYPE);
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_acl = seed_disclosable_amount_acl(&context, &fixture, amount_handle);
    let cleartext_amount = 88;
    seed_material_commitment_for_acl(&context, amount_acl, 124);
    let ed25519_ix = ed25519_verify_ix(&fixture.verifier, b"request-not-created");
    let disclose_ix = disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount);

    let result = process_transaction(&context, &[ed25519_ix, disclose_ix]);

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, amount_acl);
    assert!(!record.public_decrypt);
}

#[test]
fn mollusk_disclose_balance_rejects_missing_material_commitment() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let request_ix = request_disclose_balance_ix(&fixture);

    let result = process_transaction(&context, &[request_ix]);

    assert!(result.raw_result.is_err());
    assert!(!read_acl_record(&context, fixture.alice_current_compute_acl).public_decrypt);
    assert!(!material_commitment_exists(
        &context,
        host::handle_material_address(fixture.alice_current_compute_acl).0,
    ));
}

#[test]
fn mollusk_disclose_balance_rejects_without_public_decrypt_release() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let cleartext_amount = 125;
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 125);
    let ed25519_ix = ed25519_verify_ix(&fixture.verifier, b"request-not-created");
    let disclose_ix = disclose_balance_ix(&fixture, cleartext_amount);

    let result = process_transaction(&context, &[ed25519_ix, disclose_ix]);

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, fixture.alice_current_compute_acl);
    assert!(!record.public_decrypt);
}

#[test]
fn mollusk_disclose_balance_rejects_mismatched_kms_cleartext() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let signed_amount = 124;
    let claimed_amount = 125;
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 126);
    let request_result = process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);
    assert!(request_result.raw_result.is_ok());
    let ed25519_ix =
        disclosure_ed25519_ix(&context, &fixture, fixture.alice_initial, signed_amount);
    let disclose_ix = disclose_balance_ix(&fixture, claimed_amount);

    let result = process_transaction(&context, &[ed25519_ix, disclose_ix]);

    assert!(result.raw_result.is_err());
}

#[test]
fn mollusk_disclose_balance_rejects_wrong_kms_verifier() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let cleartext_amount = 125;
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 127);
    let request_result = process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);
    assert!(request_result.raw_result.is_ok());
    let wrong_verifier = Keypair::new();
    let request = read_disclosure_request(
        &context,
        disclosure_request_address(
            &fixture,
            fixture.owner,
            fixture.alice_initial,
            request_nonce(1),
        ),
    );
    let ed25519_ix = ed25519_verify_ix(
        &wrong_verifier,
        &disclosure_proof_message_for_request(
            &request,
            request.acl_record,
            fixture.alice_initial,
            cleartext_amount,
            token::id(),
            host::id(),
        ),
    );
    let disclose_ix = disclose_balance_ix(&fixture, cleartext_amount);

    let result = process_transaction(&context, &[ed25519_ix, disclose_ix]);

    assert!(result.raw_result.is_err());
}

#[test]
fn mollusk_request_disclose_balance_rejects_deny_witness_when_deny_list_disabled() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 132);
    let deny_subject_record = host::deny_subject_address(fixture.owner).0;
    seed_empty_system_account(&context, deny_subject_record);
    let ix = request_disclose_balance_ix_with_deny_record(&fixture, Some(deny_subject_record));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, fixture.alice_current_compute_acl);
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
fn mollusk_disclose_amount_rejects_acl_record_with_wrong_stored_bump() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(27, BALANCE_FHE_TYPE);
    let cleartext_amount = 125;
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_acl = seed_disclosable_amount_acl(&context, &fixture, amount_handle);
    seed_material_commitment_for_acl(&context, amount_acl, 128);

    let request_result = process_transaction(
        &context,
        &[request_disclose_amount_ix(
            &fixture,
            amount_acl,
            amount_handle,
        )],
    );
    assert!(request_result.raw_result.is_ok());
    mutate_acl_record(&context, amount_acl, |record| {
        record.bump = record.bump.wrapping_add(1);
    });

    let result = process_transaction(
        &context,
        &[
            disclosure_ed25519_ix(&context, &fixture, amount_handle, cleartext_amount),
            disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount),
        ],
    );

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, amount_acl);
    assert!(record.public_decrypt);
    assert_ne!(
        record.bump,
        host::acl_record_address(record.nonce_key, record.nonce_sequence).1
    );
}

#[test]
fn mollusk_disclose_amount_rejects_acl_record_with_noncanonical_nonce_sequence() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(28, BALANCE_FHE_TYPE);
    let cleartext_amount = 125;
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_acl = seed_disclosable_amount_acl(&context, &fixture, amount_handle);
    seed_material_commitment_for_acl(&context, amount_acl, 129);

    let request_result = process_transaction(
        &context,
        &[request_disclose_amount_ix(
            &fixture,
            amount_acl,
            amount_handle,
        )],
    );
    assert!(request_result.raw_result.is_ok());
    mutate_acl_record(&context, amount_acl, |record| {
        record.nonce_sequence = record.nonce_sequence.wrapping_add(1);
    });

    let result = process_transaction(
        &context,
        &[
            disclosure_ed25519_ix(&context, &fixture, amount_handle, cleartext_amount),
            disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount),
        ],
    );

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, amount_acl);
    assert!(record.public_decrypt);
    assert_ne!(
        amount_acl,
        host::acl_record_address(record.nonce_key, record.nonce_sequence).0
    );
}

#[test]
fn mollusk_disclose_balance_rejects_unlinked_unsealed_material_commitment() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let material_commitment =
        seed_unsealed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 130);
    let request_ix = request_disclose_balance_ix(&fixture);

    let result = process_transaction(&context, &[request_ix]);

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, fixture.alice_current_compute_acl);
    assert!(!record.public_decrypt);
    assert_eq!(record.material_commitment, Pubkey::default());
    assert!(material_commitment_exists(&context, material_commitment));
}

#[test]
fn mollusk_disclose_balance_rejects_oversized_material_commitment() {
    let fixture = TokenMolluskFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let cleartext_amount = 125;
    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 131);
    let request_result = process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);
    assert!(request_result.raw_result.is_ok());
    let material_commitment = host::handle_material_address(fixture.alice_current_compute_acl).0;
    let material_len = account_data_len(&context, material_commitment);
    extend_account_data(&context, material_commitment, 1);

    let result = process_transaction(
        &context,
        &[
            disclosure_ed25519_ix(&context, &fixture, fixture.alice_initial, cleartext_amount),
            disclose_balance_ix(&fixture, cleartext_amount),
        ],
    );

    assert!(result.raw_result.is_err());
    assert_eq!(
        account_data_len(&context, material_commitment),
        material_len + 1
    );
    let record = read_acl_record(&context, fixture.alice_current_compute_acl);
    assert!(record.public_decrypt);
    assert_eq!(record.material_commitment, material_commitment);
}

#[test]
fn mollusk_transfer_receiver_hook_records_same_transaction_callback_metadata() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(43, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(44, 0);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let context = fixture.context_with_input_amount(amount_handle);
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
    assert_eq!(balance_events[0].new_acl_record, output.from_output);
    assert_eq!(balance_events[1].new_acl_record, output.to_output);
}

#[test]
fn mollusk_transfer_receiver_hook_is_one_shot_per_sent_handle() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(51, BALANCE_FHE_TYPE);
    let callback_success_handle = handle_for_chain(52, 0);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let context = fixture.context_with_input_amount(amount_handle);
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
    let transfer_output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let transfer_ix = direct_transfer_ix(&fixture, transfer_output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
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
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let transfer_ix = direct_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
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
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let context = fixture.context_with_input_amount(amount_handle);
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
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let context = fixture.context_with_input_amount(amount_handle);
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
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let context = fixture.context_with_input_amount(amount_handle);
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
    let transfer_output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let transfer_ix = direct_transfer_ix(&fixture, transfer_output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);

    context.process_and_validate_instruction(&transfer_ix, &[Check::success()]);

    let sent_acl = read_acl_record(&context, transfer_output.transferred);
    let sent_handle = sent_acl.handle;
    let callback_success_acl = callback_success_acl_address(
        fixture.mint,
        fixture.bob_owner,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let output = CallbackSettlementOutputAccounts::canonical(&fixture, sent_handle, 2, 2);
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
    for account in output.all_accounts() {
        seed_empty_system_account(&context, account);
    }

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
    let bob_output_acl = read_acl_record(&context, output.to_output);
    let prepared = read_transfer_callback_settlement(&context, output.settlement);
    let prepare_balance_events: Vec<token::BalanceHandleUpdatedEvent> = prepare_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();

    assert_eq!(bob_token.balance_acl_record, output.to_output);
    assert_eq!(bob_token.balance_handle, bob_output_acl.handle);
    assert_eq!(bob_token.next_balance_nonce_sequence, 3);
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
    assert_eq!(prepared.to_balance_handle, bob_output_acl.handle);
    assert_eq!(prepared.to_balance_acl_record, output.to_output);
    assert!(!acl_record_exists(
        &context,
        callback_requested_refund_acl_address(&fixture, 2)
    ));
    assert_acl_record(
        &refund_acl,
        token::nonce_key(
            fixture.mint,
            fixture.bob_token,
            token::callback_refund_amount_label(),
        ),
        2,
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
    assert_acl_record(
        &bob_output_acl,
        token::balance_nonce_key(fixture.mint, fixture.bob_token),
        2,
        fixture.mint,
        fixture.bob_token,
        token::balance_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.bob_owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert_eq!(prepare_balance_events.len(), 1);
    assert_eq!(
        prepare_balance_events[0].reason,
        token::BalanceHandleUpdateReason::TransferCallbackRefundDebit
    );
    assert_eq!(
        prepare_balance_events[0].old_acl_record,
        transfer_output.to_output
    );
    assert_eq!(prepare_balance_events[0].new_acl_record, output.to_output);
    assert_eq!(prepare_balance_events[0].new_handle, bob_output_acl.handle);

    let finalize_ix = finalize_transfer_callback_ix(
        &fixture,
        transfer_output.from_output,
        transfer_output.transferred,
        output,
    );
    let finalize_result =
        context.process_and_validate_instruction(&finalize_ix, &[Check::success()]);

    let alice_token = read_token_account(&context, fixture.alice_token);
    let alice_output_acl = read_acl_record(&context, output.from_output);
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

    assert_eq!(alice_token.balance_acl_record, output.from_output);
    assert_eq!(alice_token.balance_handle, alice_output_acl.handle);
    assert_eq!(alice_token.next_balance_nonce_sequence, 3);
    assert_eq!(finalized.status, token::CALLBACK_SETTLEMENT_FINALIZED);
    assert_eq!(finalized.from_balance_handle, alice_output_acl.handle);
    assert_eq!(finalized.from_balance_acl_record, output.from_output);
    assert_eq!(finalized.transferred_handle, final_transferred_acl.handle);
    assert_eq!(finalized.transferred_acl_record, output.final_transferred);
    assert_acl_record(
        &alice_output_acl,
        token::balance_nonce_key(fixture.mint, fixture.alice_token),
        2,
        fixture.mint,
        fixture.alice_token,
        token::balance_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert_acl_record(
        &final_transferred_acl,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::callback_final_transferred_label(),
        ),
        2,
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
        finalize_balance_events[0].old_acl_record,
        transfer_output.from_output
    );
    assert_eq!(
        finalize_balance_events[0].new_acl_record,
        output.from_output
    );
    assert_eq!(
        finalize_balance_events[0].new_handle,
        alice_output_acl.handle
    );
}

#[test]
fn mollusk_wrap_usdc_escrows_spl_tokens_and_rotates_confidential_balance() {
    let fixture = TokenMolluskFixture::new();
    let amount = 100_000_000;
    let output = WrapOutputAccounts::canonical(&fixture, 1);
    let ix = wrap_usdc_ix(&fixture, output, amount);
    let context = fixture.context_with_wrap_accounts();

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
    let balance_acl = read_acl_record(&context, output.balance);
    let total_supply_acl = read_acl_record(&context, output.total_supply);
    let legacy_amount_acl = token_acl_address(
        fixture.mint,
        fixture.alice_token,
        token::wrap_amount_label(),
        1,
    );
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

    assert_eq!(token_account.balance_acl_record, output.balance);
    assert_eq!(token_account.balance_handle, balance_acl.handle);
    assert_eq!(token_account.next_balance_nonce_sequence, 2);
    assert_eq!(mint_account.total_supply_acl_record, output.total_supply);
    assert_eq!(mint_account.total_supply_handle, total_supply_acl.handle);
    assert_eq!(mint_account.next_total_supply_nonce_sequence, 2);
    assert!(!acl_record_exists(&context, legacy_amount_acl));

    assert_acl_record(
        &balance_acl,
        token::balance_nonce_key(fixture.mint, fixture.alice_token),
        1,
        fixture.mint,
        fixture.alice_token,
        token::balance_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert_acl_record(
        &total_supply_acl,
        token::total_supply_nonce_key(fixture.mint, fixture.total_supply_authority),
        1,
        fixture.mint,
        fixture.total_supply_authority,
        token::total_supply_label(),
        BALANCE_FHE_TYPE,
        &[(fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT)],
    );

    let wrap_handles = predicted_wrap_handles(
        &fixture,
        &context,
        amount,
        fixture.alice_initial,
        fixture.total_supply_initial,
        1,
        1,
    );
    assert_eq!(wrap_handles.balance, balance_acl.handle);
    assert_eq!(wrap_handles.total_supply, total_supply_acl.handle);

    assert_eq!(balance_events.len(), 1);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::Wrap
    );
    assert_eq!(balance_events[0].old_handle, fixture.alice_initial);
    assert_eq!(
        balance_events[0].old_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(balance_events[0].new_handle, balance_acl.handle);
    assert_eq!(balance_events[0].new_acl_record, output.balance);
    assert_eq!(supply_events.len(), 1);
    assert_eq!(
        supply_events[0].reason,
        token::TotalSupplyUpdateReason::Wrap
    );
    assert_eq!(supply_events[0].old_handle, fixture.total_supply_initial);
    assert_eq!(
        supply_events[0].old_acl_record,
        fixture.total_supply_current_acl
    );
    assert_eq!(supply_events[0].new_handle, total_supply_acl.handle);
    assert_eq!(supply_events[0].new_acl_record, output.total_supply);
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
        token_account_after.balance_acl_record,
        token_account_before.balance_acl_record
    );
    assert_eq!(
        token_account_after.next_balance_nonce_sequence,
        token_account_before.next_balance_nonce_sequence
    );
    let mint_account_after = read_confidential_mint(&context, fixture.mint);
    assert_eq!(
        mint_account_after.total_supply_handle,
        mint_account_before.total_supply_handle
    );
    assert_eq!(
        mint_account_after.total_supply_acl_record,
        mint_account_before.total_supply_acl_record
    );
    assert_eq!(
        mint_account_after.next_total_supply_nonce_sequence,
        mint_account_before.next_total_supply_nonce_sequence
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
    let burn_output = BurnOutputAccounts::canonical(&fixture, 2, 2);
    let context = fixture.context_with_wrap_and_burn_amount(burn_amount_handle);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    let wrapped_balance_acl = read_acl_record(&context, wrap_output.balance);
    let wrapped_supply_acl = read_acl_record(&context, wrap_output.total_supply);
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
    let balance_acl = read_acl_record(&context, burn_output.balance);
    let burned_acl = read_acl_record(&context, burn_output.burned);
    let total_supply_acl = read_acl_record(&context, burn_output.total_supply);
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
    assert_eq!(token_account.balance_acl_record, burn_output.balance);
    assert_eq!(token_account.balance_handle, balance_acl.handle);
    assert_eq!(token_account.next_balance_nonce_sequence, 3);
    assert_eq!(
        mint_account.total_supply_acl_record,
        burn_output.total_supply
    );
    assert_eq!(mint_account.total_supply_handle, total_supply_acl.handle);
    assert_eq!(mint_account.next_total_supply_nonce_sequence, 3);

    assert_acl_record(
        &balance_acl,
        token::balance_nonce_key(fixture.mint, fixture.alice_token),
        2,
        fixture.mint,
        fixture.alice_token,
        token::balance_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert_acl_record(
        &burned_acl,
        token::nonce_key(
            fixture.mint,
            fixture.alice_token,
            token::burned_amount_label(),
        ),
        2,
        fixture.mint,
        fixture.alice_token,
        token::burned_amount_label(),
        BALANCE_FHE_TYPE,
        &[
            (fixture.owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );
    assert_acl_record(
        &total_supply_acl,
        token::total_supply_nonce_key(fixture.mint, fixture.total_supply_authority),
        2,
        fixture.mint,
        fixture.total_supply_authority,
        token::total_supply_label(),
        BALANCE_FHE_TYPE,
        &[(fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT)],
    );

    let burn_handles = predicted_burn_handles(
        &fixture,
        &context,
        burn_amount_handle,
        wrapped_balance_acl.handle,
        wrapped_supply_acl.handle,
        2,
        2,
    );
    assert_eq!(burn_handles.balance, balance_acl.handle);
    assert_eq!(burn_handles.burned, burned_acl.handle);
    assert_eq!(burn_handles.total_supply, total_supply_acl.handle);

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
    assert_eq!(balance_events[0].old_handle, wrapped_balance_acl.handle);
    assert_eq!(balance_events[0].old_acl_record, wrap_output.balance);
    assert_eq!(balance_events[0].new_handle, balance_acl.handle);
    assert_eq!(balance_events[0].new_acl_record, burn_output.balance);
    assert_eq!(supply_events.len(), 1);
    assert_eq!(
        supply_events[0].reason,
        token::TotalSupplyUpdateReason::Burn
    );
    assert_eq!(supply_events[0].old_handle, wrapped_supply_acl.handle);
    assert_eq!(supply_events[0].old_acl_record, wrap_output.total_supply);
    assert_eq!(supply_events[0].new_handle, total_supply_acl.handle);
    assert_eq!(supply_events[0].new_acl_record, burn_output.total_supply);
}

#[test]
fn mollusk_confidential_burn_over_balance_burns_zero_without_underflow() {
    let fixture = TokenMolluskFixture::new();
    let wrap_amount = 100_000_000;
    let burn_amount_handle = handle_for_chain(78, BALANCE_FHE_TYPE);
    let wrap_output = WrapOutputAccounts::canonical(&fixture, 1);
    let burn_output = BurnOutputAccounts::canonical(&fixture, 2, 2);
    let context = fixture.context_with_wrap_and_burn_amount(burn_amount_handle);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.total_supply_initial, TypedClearValue::uint64(0));

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    let wrapped_balance_acl = read_acl_record(&context, wrap_output.balance);
    let wrapped_supply_acl = read_acl_record(&context, wrap_output.total_supply);
    let wrap_handles = predicted_wrap_handles(
        &fixture,
        &context,
        wrap_amount,
        fixture.alice_initial,
        fixture.total_supply_initial,
        1,
        1,
    );
    assert_eq!(wrap_handles.balance, wrapped_balance_acl.handle);
    assert_eq!(wrap_handles.total_supply, wrapped_supply_acl.handle);
    cleartext.seed_cleartext(
        wrapped_balance_acl.handle,
        TypedClearValue::uint64(100_000_125),
    );
    cleartext.seed_cleartext(
        wrapped_supply_acl.handle,
        TypedClearValue::uint64(100_000_000),
    );
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
    let balance_acl = read_acl_record(&context, burn_output.balance);
    let burned_acl = read_acl_record(&context, burn_output.burned);
    let total_supply_acl = read_acl_record(&context, burn_output.total_supply);

    assert_eq!(token_account.balance_acl_record, burn_output.balance);
    assert_eq!(token_account.balance_handle, balance_acl.handle);
    assert_eq!(token_account.next_balance_nonce_sequence, 3);
    assert_eq!(
        mint_account.total_supply_acl_record,
        burn_output.total_supply
    );
    assert_eq!(mint_account.total_supply_handle, total_supply_acl.handle);
    assert_eq!(mint_account.next_total_supply_nonce_sequence, 3);
    let burn_handles = predicted_burn_handles(
        &fixture,
        &context,
        burn_amount_handle,
        wrapped_balance_acl.handle,
        wrapped_supply_acl.handle,
        2,
        2,
    );
    assert_eq!(burn_handles.balance, balance_acl.handle);
    assert_eq!(burn_handles.burned, burned_acl.handle);
    assert_eq!(burn_handles.total_supply, total_supply_acl.handle);
    cleartext.seed_cleartext(
        burn_handles.success,
        TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        },
    );
    cleartext.seed_cleartext(balance_acl.handle, TypedClearValue::uint64(100_000_125));
    cleartext.seed_cleartext(burned_acl.handle, TypedClearValue::uint64(0));
    cleartext.seed_cleartext(
        total_supply_acl.handle,
        TypedClearValue::uint64(100_000_000),
    );
    assert_eq!(
        cleartext.decrypt_cleartext(burn_handles.success),
        Some(TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        })
    );
    assert_eq!(
        cleartext.decrypt_cleartext(balance_acl.handle),
        Some(TypedClearValue::uint64(100_000_125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(burned_acl.handle),
        Some(TypedClearValue::uint64(0))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(total_supply_acl.handle),
        Some(TypedClearValue::uint64(100_000_000))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(wrapped_balance_acl.handle),
        Some(TypedClearValue::uint64(100_000_125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(wrapped_supply_acl.handle),
        Some(TypedClearValue::uint64(100_000_000))
    );
}

#[test]
fn mollusk_confidential_burn_rejects_transfer_amount_acl_label() {
    let fixture = TokenMolluskFixture::new();
    let wrap_amount = 100_000_000;
    let amount_handle = handle_for_chain(79, BALANCE_FHE_TYPE);
    let wrap_output = WrapOutputAccounts::canonical(&fixture, 1);
    let burn_output = BurnOutputAccounts::canonical(&fixture, 2, 2);
    let context = fixture.context_with_wrap_accounts();
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

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
    for account in burn_output.all_accounts() {
        seed_empty_system_account(&context, account);
    }

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
    assert_eq!(
        read_token_account(&context, fixture.alice_token).balance_acl_record,
        wrap_output.balance
    );
    assert_eq!(
        read_confidential_mint(&context, fixture.mint).total_supply_acl_record,
        wrap_output.total_supply
    );
    for account in burn_output.all_accounts() {
        assert_empty_system_account(&context, account);
    }
}

#[test]
fn mollusk_redeem_burned_amount_releases_vault_once_with_kms_certificate() {
    let fixture = TokenMolluskFixture::new();
    let cleartext_amount = 9;
    let burn_amount_handle = handle_for_chain(53, BALANCE_FHE_TYPE);
    let (_wrap_output, burn_output, mut context, burned_handle) =
        wrap_and_burn_for_redeem(&fixture, burn_amount_handle);

    seed_material_commitment_for_acl(&context, burn_output.burned, 140);
    let release_ix = request_burn_redemption_ix(&fixture, burn_output.burned, burned_handle);
    context.process_and_validate_instruction(&release_ix, &[Check::success()]);

    let released_burned_acl = read_acl_record(&context, burn_output.burned);
    assert!(released_burned_acl.public_decrypt);
    assert_eq!(released_burned_acl.handle, burned_handle);

    let redemption_record = token::burn_redemption_address(fixture.mint, burned_handle).0;
    let vault_before = read_spl_token_amount(&context, fixture.vault_usdc);
    let destination_before = read_spl_token_amount(&context, fixture.user_usdc);
    let ed25519_ix = redemption_ed25519_ix(&context, &fixture, burned_handle, cleartext_amount);
    let replay_ed25519_ix = ed25519_ix.clone();
    let redeem_ix = redeem_burned_amount_ix(
        &fixture,
        burn_output.burned,
        redemption_record,
        burned_handle,
        cleartext_amount,
    );

    let redeem_result = process_transaction(&context, &[ed25519_ix, redeem_ix]);

    assert!(
        redeem_result.raw_result.is_ok(),
        "redeem transaction failed: raw={:?} program={:?}",
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
    assert_eq!(redemption.mint, fixture.mint);
    assert_eq!(redemption.owner, fixture.owner);
    assert_eq!(redemption.token_account, fixture.alice_token);
    assert_eq!(redemption.burned_handle, burned_handle);
    assert_eq!(redemption.burned_acl_record, burn_output.burned);
    assert_eq!(redemption.cleartext_amount, cleartext_amount);

    let redeem_events: Vec<token::BurnRedeemedEvent> = redeem_result
        .inner_instructions
        .iter()
        .flatten()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(redeem_events.len(), 1);
    assert_eq!(redeem_events[0].version, token::APP_EVENT_VERSION);
    assert_eq!(redeem_events[0].mint, fixture.mint);
    assert_eq!(redeem_events[0].owner, fixture.owner);
    assert_eq!(redeem_events[0].token_account, fixture.alice_token);
    assert_eq!(redeem_events[0].burned_handle, burned_handle);
    assert_eq!(redeem_events[0].burned_acl_record, burn_output.burned);
    assert_eq!(redeem_events[0].destination_usdc, fixture.user_usdc);
    assert_eq!(redeem_events[0].cleartext_amount, cleartext_amount);
    let redemption_request =
        burn_redemption_request_address(&fixture, burned_handle, request_nonce(1));
    assert_eq!(
        read_burn_redemption_request(&context, redemption_request).status,
        token::REQUEST_STATUS_CONSUMED
    );

    let vault_after_redeem = read_spl_token_amount(&context, fixture.vault_usdc);
    let destination_after_redeem = read_spl_token_amount(&context, fixture.user_usdc);
    let replay_redeem_ix = redeem_burned_amount_ix(
        &fixture,
        burn_output.burned,
        redemption_record,
        burned_handle,
        cleartext_amount,
    );
    let replay_result = process_transaction(&context, &[replay_ed25519_ix, replay_redeem_ix]);

    assert!(replay_result.raw_result.is_err());
    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        vault_after_redeem
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.user_usdc),
        destination_after_redeem
    );

    let owner_before = account_lamports(&context, fixture.owner);
    context
        .mollusk
        .sysvars
        .warp_to_slot(DEFAULT_REQUEST_EXPIRES_SLOT + 1);
    let close_result = context.process_and_validate_instruction(
        &close_consumed_burn_redemption_request_ix(&fixture, redemption_request),
        &[Check::success()],
    );
    assert!(close_result.raw_result.is_ok());
    assert!(!burn_redemption_request_exists(
        &context,
        redemption_request
    ));
    assert!(account_lamports(&context, fixture.owner) > owner_before);
}

#[test]
fn mollusk_close_expired_burn_redemption_request_returns_rent_to_owner() {
    let fixture = TokenMolluskFixture::new();
    let burn_amount_handle = handle_for_chain(58, BALANCE_FHE_TYPE);
    let (_wrap_output, burn_output, mut context, burned_handle) =
        wrap_and_burn_for_redeem(&fixture, burn_amount_handle);
    seed_material_commitment_for_acl(&context, burn_output.burned, 144);
    let request = burn_redemption_request_address(&fixture, burned_handle, request_nonce(8));
    let request_ix = request_burn_redemption_ix_with_nonce_and_expires(
        &fixture,
        burn_output.burned,
        burned_handle,
        request_nonce(8),
        5,
    );

    let request_result = process_transaction(&context, &[request_ix]);
    assert!(request_result.raw_result.is_ok());
    assert!(burn_redemption_request_exists(&context, request));
    let owner_before = account_lamports(&context, fixture.owner);

    context.mollusk.sysvars.warp_to_slot(6);
    let result = context.process_and_validate_instruction(
        &close_expired_burn_redemption_request_ix(&fixture, request),
        &[Check::success()],
    );

    assert!(result.raw_result.is_ok());
    assert!(!burn_redemption_request_exists(&context, request));
    assert!(account_lamports(&context, fixture.owner) > owner_before);
}

#[test]
fn mollusk_redeem_burned_amount_rejects_without_public_decrypt_release() {
    let fixture = TokenMolluskFixture::new();
    let cleartext_amount = 9;
    let burn_amount_handle = handle_for_chain(54, BALANCE_FHE_TYPE);
    let (_wrap_output, burn_output, context, burned_handle) =
        wrap_and_burn_for_redeem(&fixture, burn_amount_handle);

    seed_material_commitment_for_acl(&context, burn_output.burned, 141);

    let burned_acl = read_acl_record(&context, burn_output.burned);
    assert!(!burned_acl.public_decrypt);
    assert_eq!(burned_acl.handle, burned_handle);

    let redemption_record = token::burn_redemption_address(fixture.mint, burned_handle).0;
    let vault_before = read_spl_token_amount(&context, fixture.vault_usdc);
    let destination_before = read_spl_token_amount(&context, fixture.user_usdc);
    let ed25519_ix = ed25519_verify_ix(&fixture.verifier, b"request-not-created");
    let redeem_ix = redeem_burned_amount_ix(
        &fixture,
        burn_output.burned,
        redemption_record,
        burned_handle,
        cleartext_amount,
    );

    let result = process_transaction(&context, &[ed25519_ix, redeem_ix]);

    assert!(result.raw_result.is_err());
    assert!(!burn_redemption_exists(&context, redemption_record));
    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        vault_before
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.user_usdc),
        destination_before
    );
}

#[test]
fn mollusk_redeem_burned_amount_rejects_noncanonical_vault_account() {
    let fixture = TokenMolluskFixture::new();
    let cleartext_amount = 9;
    let burn_amount_handle = handle_for_chain(55, BALANCE_FHE_TYPE);
    let (_wrap_output, burn_output, context, burned_handle) =
        wrap_and_burn_for_redeem(&fixture, burn_amount_handle);

    seed_material_commitment_for_acl(&context, burn_output.burned, 142);
    let release_ix = request_burn_redemption_ix(&fixture, burn_output.burned, burned_handle);
    context.process_and_validate_instruction(&release_ix, &[Check::success()]);
    let noncanonical_vault = seed_noncanonical_vault_token_account(&context, &fixture, 100);

    let redemption_record = token::burn_redemption_address(fixture.mint, burned_handle).0;
    let canonical_vault_before = read_spl_token_amount(&context, fixture.vault_usdc);
    let noncanonical_vault_before = read_spl_token_amount(&context, noncanonical_vault);
    let destination_before = read_spl_token_amount(&context, fixture.user_usdc);
    let ed25519_ix = redemption_ed25519_ix(&context, &fixture, burned_handle, cleartext_amount);
    let redeem_ix = redeem_burned_amount_ix_with_vault(
        &fixture,
        burn_output.burned,
        redemption_record,
        burned_handle,
        cleartext_amount,
        noncanonical_vault,
    );

    let result = process_transaction(&context, &[ed25519_ix, redeem_ix]);

    assert!(result.raw_result.is_err());
    assert!(!burn_redemption_exists(&context, redemption_record));
    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        canonical_vault_before
    );
    assert_eq!(
        read_spl_token_amount(&context, noncanonical_vault),
        noncanonical_vault_before
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.user_usdc),
        destination_before
    );
}

#[test]
fn mollusk_redeem_burned_amount_rejects_mismatched_kms_cleartext() {
    let fixture = TokenMolluskFixture::new();
    let signed_amount = 8;
    let claimed_amount = 9;
    let burn_amount_handle = handle_for_chain(56, BALANCE_FHE_TYPE);
    let (_wrap_output, burn_output, context, burned_handle) =
        wrap_and_burn_for_redeem(&fixture, burn_amount_handle);

    seed_material_commitment_for_acl(&context, burn_output.burned, 143);
    let release_ix = request_burn_redemption_ix(&fixture, burn_output.burned, burned_handle);
    context.process_and_validate_instruction(&release_ix, &[Check::success()]);

    let redemption_record = token::burn_redemption_address(fixture.mint, burned_handle).0;
    let vault_before = read_spl_token_amount(&context, fixture.vault_usdc);
    let destination_before = read_spl_token_amount(&context, fixture.user_usdc);
    let ed25519_ix = redemption_ed25519_ix(&context, &fixture, burned_handle, signed_amount);
    let redeem_ix = redeem_burned_amount_ix(
        &fixture,
        burn_output.burned,
        redemption_record,
        burned_handle,
        claimed_amount,
    );

    let result = process_transaction(&context, &[ed25519_ix, redeem_ix]);

    assert!(result.raw_result.is_err());
    assert!(!burn_redemption_exists(&context, redemption_record));
    assert_eq!(
        read_spl_token_amount(&context, fixture.vault_usdc),
        vault_before
    );
    assert_eq!(
        read_spl_token_amount(&context, fixture.user_usdc),
        destination_before
    );
}

#[test]
fn mollusk_confidential_token_account_rejects_wrong_bump_or_length() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(96, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
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
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
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
    let disclosure_verifier_set =
        host::verifier_set_address(host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE, mint, 1).0;
    let redemption_verifier_set =
        host::verifier_set_address(host::VERIFIER_SET_KIND_TOKEN_REDEMPTION, mint, 1).0;
    let verifier = Keypair::new();
    let total_supply_acl_record = token_total_supply_acl_address(mint, total_supply_authority, 0);
    let host_config = host::host_config_address().0;
    let context = mollusk().with_context(HashMap::from([
        (authority, system_account(5_000_000_000)),
        (mint, system_account(0)),
        (underlying_mint, spl_mint_account(authority, 6, 0)),
        (compute_signer, system_account(0)),
        (total_supply_authority, system_account(0)),
        (
            disclosure_verifier_set,
            verifier_set_account(
                authority,
                host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE,
                mint,
                verifier.pubkey(),
            ),
        ),
        (
            redemption_verifier_set,
            verifier_set_account(
                authority,
                host::VERIFIER_SET_KIND_TOKEN_REDEMPTION,
                mint,
                verifier.pubkey(),
            ),
        ),
        (total_supply_acl_record, system_account(0)),
        (host_config, host_config_account(authority)),
        (event_authority(host::id()), system_account(0)),
        (event_authority(token::id()), system_account(0)),
    ]));
    let ix = initialize_mint_ix(
        authority,
        mint,
        underlying_mint,
        compute_signer,
        total_supply_authority,
        disclosure_verifier_set,
        redemption_verifier_set,
        total_supply_acl_record,
        host_config,
    );

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_confidential_mint(&context, mint);
    let supply_acl = read_acl_record(&context, total_supply_acl_record);
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
    assert_eq!(stored.disclosure_verifier_set, disclosure_verifier_set);
    assert_eq!(stored.redemption_verifier_set, redemption_verifier_set);
    assert_eq!(stored.decimals, 6);
    assert_eq!(stored.total_supply_handle, supply_acl.handle);
    assert_eq!(stored.total_supply_acl_record, total_supply_acl_record);
    assert_eq!(stored.next_total_supply_nonce_sequence, 1);
    assert_acl_record(
        &supply_acl,
        token::total_supply_nonce_key(mint, total_supply_authority),
        0,
        mint,
        total_supply_authority,
        token::total_supply_label(),
        BALANCE_FHE_TYPE,
        &[(compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT)],
    );

    assert_eq!(trivial_events.len(), 1);
    assert_eq!(trivial_events[0].subject, compute_signer.to_bytes());
    assert_eq!(trivial_events[0].plaintext, amount_plaintext(0));
    assert_eq!(trivial_events[0].fhe_type, BALANCE_FHE_TYPE);
    assert_eq!(trivial_events[0].result, supply_acl.handle);
    assert_eq!(supply_events.len(), 1);
    assert_eq!(supply_events[0].mint, mint);
    assert_eq!(supply_events[0].old_handle, [0; 32]);
    assert_eq!(supply_events[0].old_acl_record, Pubkey::default());
    assert_eq!(supply_events[0].new_handle, supply_acl.handle);
    assert_eq!(supply_events[0].new_acl_record, total_supply_acl_record);
    assert_eq!(
        supply_events[0].reason,
        token::TotalSupplyUpdateReason::Initialize
    );
}

#[test]
fn mollusk_initialize_mint_rejects_invalid_verifier_set_accounts() {
    let authority = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let underlying_mint = Pubkey::new_unique();
    let compute_signer = token::compute_signer_address(mint).0;
    let total_supply_authority = token::total_supply_authority_address(mint).0;
    let disclosure_verifier_set =
        host::verifier_set_address(host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE, mint, 1).0;
    let total_supply_acl_record = token_total_supply_acl_address(mint, total_supply_authority, 0);
    let host_config = host::host_config_address().0;
    let context = mollusk().with_context(HashMap::from([
        (authority, system_account(5_000_000_000)),
        (mint, system_account(0)),
        (underlying_mint, spl_mint_account(authority, 6, 0)),
        (compute_signer, system_account(0)),
        (total_supply_authority, system_account(0)),
        (disclosure_verifier_set, system_account(1)),
        (total_supply_acl_record, system_account(0)),
        (host_config, host_config_account(authority)),
        (event_authority(host::id()), system_account(0)),
        (event_authority(token::id()), system_account(0)),
    ]));
    let ix = initialize_mint_ix(
        authority,
        mint,
        underlying_mint,
        compute_signer,
        total_supply_authority,
        disclosure_verifier_set,
        Pubkey::default(),
        total_supply_acl_record,
        host_config,
    );

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_empty_system_account(&context, mint);
    assert_empty_system_account(&context, total_supply_acl_record);
}

#[test]
fn mollusk_migrate_legacy_mint_preserves_state_and_sets_split_verifiers() {
    let fixture = TokenMolluskFixture::new();
    let legacy_verifier = Pubkey::new_unique();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.mint,
        legacy_confidential_mint_account(&fixture, legacy_verifier),
    );
    let context = mollusk().with_context(accounts);
    let ix = migrate_mint_verifier_sets_ix(&fixture, fixture.owner);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_confidential_mint(&context, fixture.mint);
    assert_eq!(stored.authority, fixture.owner);
    assert_eq!(stored.acl_domain_key, fixture.mint);
    assert_eq!(stored.compute_signer, fixture.compute_signer);
    assert_eq!(stored.underlying_mint, fixture.underlying_mint);
    assert_eq!(
        stored.disclosure_verifier_set,
        fixture.disclosure_verifier_set
    );
    assert_eq!(
        stored.redemption_verifier_set,
        fixture.redemption_verifier_set
    );
    assert_eq!(stored.decimals, 6);
    assert_eq!(stored.total_supply_handle, fixture.total_supply_initial);
    assert_eq!(
        stored.total_supply_acl_record,
        fixture.total_supply_current_acl
    );
    assert_eq!(stored.next_total_supply_nonce_sequence, 1);
    let migrated_events: Vec<token::ConfidentialMintMigratedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(migrated_events.len(), 1);
    assert_eq!(migrated_events[0].mint, fixture.mint);
    assert_eq!(migrated_events[0].authority, fixture.owner);
    assert_eq!(
        migrated_events[0].legacy_kms_verifier_authority,
        legacy_verifier
    );
    assert_eq!(
        migrated_events[0].disclosure_verifier_set,
        fixture.disclosure_verifier_set
    );
    assert_eq!(
        migrated_events[0].redemption_verifier_set,
        fixture.redemption_verifier_set
    );
}

#[test]
fn mollusk_initialize_token_account_creates_initial_balance_acl() {
    let fixture = TokenMolluskFixture::new();
    let owner = Pubkey::new_unique();
    let (token_account, token_bump) = token::token_account_address(fixture.mint, owner);
    let acl_record = token_balance_acl_address(fixture.mint, token_account, 0);
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(acl_record, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(
        owner,
        fixture.mint,
        fixture.compute_signer,
        token_account,
        acl_record,
        fixture.host_config,
        0,
    );

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_token_account(&context, token_account);
    let balance_acl = read_acl_record(&context, acl_record);
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
    assert_eq!(stored.balance_handle, balance_acl.handle);
    assert_eq!(stored.balance_acl_record, acl_record);
    assert_eq!(stored.next_balance_nonce_sequence, 1);
    assert_eq!(stored.next_amount_nonce_sequence, 0);
    assert_eq!(stored.bump, token_bump);
    assert_acl_record(
        &balance_acl,
        token::balance_nonce_key(fixture.mint, token_account),
        0,
        fixture.mint,
        token_account,
        token::balance_label(),
        BALANCE_FHE_TYPE,
        &[
            (owner, host::ACL_ROLE_USER),
            (fixture.compute_signer, host::ACL_ROLE_COMPUTE_SUBJECT),
        ],
    );

    assert_eq!(trivial_events.len(), 1);
    assert_eq!(trivial_events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(trivial_events[0].plaintext, amount_plaintext(0));
    assert_eq!(trivial_events[0].fhe_type, BALANCE_FHE_TYPE);
    assert_eq!(trivial_events[0].result, balance_acl.handle);
    assert_eq!(balance_events.len(), 1);
    assert_eq!(balance_events[0].mint, fixture.mint);
    assert_eq!(balance_events[0].owner, owner);
    assert_eq!(balance_events[0].token_account, token_account);
    assert_eq!(balance_events[0].old_handle, [0; 32]);
    assert_eq!(balance_events[0].old_acl_record, Pubkey::default());
    assert_eq!(balance_events[0].new_handle, balance_acl.handle);
    assert_eq!(balance_events[0].new_acl_record, acl_record);
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
    let acl_record = token_balance_acl_address(fixture.mint, token_account, 0);
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(acl_record, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(
        owner,
        fixture.mint,
        fixture.compute_signer,
        token_account,
        acl_record,
        fixture.host_config,
        1,
    );

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert_empty_system_account(&context, token_account);
    assert_empty_system_account(&context, acl_record);
}

#[test]
fn mollusk_create_random_amount_advances_nonce_and_emits_events() {
    let fixture = TokenMolluskFixture::new();
    let transfer_acl = amount_acl_address(fixture.mint, fixture.owner, 0);
    let burn_acl = burn_amount_acl_address(fixture.mint, fixture.owner, 1);
    let mut accounts = fixture.base_accounts();
    accounts.insert(transfer_acl, system_account(0));
    accounts.insert(burn_acl, system_account(0));
    let context = mollusk().with_context(accounts);
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
    let context = mollusk().with_context(accounts);
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
    disclosure_verifier_set: Pubkey,
    redemption_verifier_set: Pubkey,
    verifier: Keypair,
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
        let verifier = Keypair::new();
        let disclosure_verifier_set =
            host::verifier_set_address(host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE, mint, 1).0;
        let redemption_verifier_set =
            host::verifier_set_address(host::VERIFIER_SET_KIND_TOKEN_REDEMPTION, mint, 1).0;
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
            disclosure_verifier_set,
            redemption_verifier_set,
            verifier,
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
        for account in DirectTransferOutputAccounts::canonical(self, 1, 1).all_accounts() {
            accounts.entry(account).or_insert_with(|| system_account(0));
        }
        mollusk().with_context(accounts)
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
        mollusk().with_context(accounts)
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
        for account in BurnOutputAccounts::canonical(self, 2, 2).all_accounts() {
            accounts.entry(account).or_insert_with(|| system_account(0));
        }
        mollusk().with_context(accounts)
    }

    fn base_accounts(&self) -> HashMap<Pubkey, Account> {
        HashMap::from([
            (self.owner, system_account(5_000_000_000)),
            (self.bob_owner, system_account(5_000_000_000)),
            (self.mint, confidential_mint_account(self)),
            (self.compute_signer, system_account(0)),
            (self.host_config, host_config_account(self.owner)),
            (
                self.disclosure_verifier_set,
                verifier_set_account(
                    self.owner,
                    host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE,
                    self.mint,
                    self.verifier.pubkey(),
                ),
            ),
            (
                self.redemption_verifier_set,
                verifier_set_account(
                    self.owner,
                    host::VERIFIER_SET_KIND_TOKEN_REDEMPTION,
                    self.mint,
                    self.verifier.pubkey(),
                ),
            ),
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
    fn canonical(fixture: &TokenMolluskFixture, nonce_sequence: u64) -> Self {
        let alice = token_balance_acl_address(fixture.mint, fixture.alice_token, nonce_sequence);
        Self {
            alice,
            to_output: alice,
            transferred: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::transferred_amount_label(),
                nonce_sequence,
            ),
        }
    }

    fn all_accounts(self) -> [Pubkey; 3] {
        [self.alice, self.to_output, self.transferred]
    }
}

#[derive(Clone, Copy)]
struct DirectTransferOutputAccounts {
    from_output: Pubkey,
    to_output: Pubkey,
    transferred: Pubkey,
}

impl DirectTransferOutputAccounts {
    fn canonical(
        fixture: &TokenMolluskFixture,
        from_nonce_sequence: u64,
        to_nonce_sequence: u64,
    ) -> Self {
        Self {
            from_output: token_balance_acl_address(
                fixture.mint,
                fixture.alice_token,
                from_nonce_sequence,
            ),
            to_output: token_balance_acl_address(
                fixture.mint,
                fixture.bob_token,
                to_nonce_sequence,
            ),
            transferred: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::transferred_amount_label(),
                from_nonce_sequence,
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
    fn canonical(fixture: &TokenMolluskFixture, nonce_sequence: u64) -> Self {
        Self {
            balance: token_balance_acl_address(fixture.mint, fixture.alice_token, nonce_sequence),
            total_supply: token_total_supply_acl_address(
                fixture.mint,
                fixture.total_supply_authority,
                nonce_sequence,
            ),
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
    fn canonical(
        fixture: &TokenMolluskFixture,
        balance_nonce_sequence: u64,
        total_supply_nonce_sequence: u64,
    ) -> Self {
        Self {
            balance: token_balance_acl_address(
                fixture.mint,
                fixture.alice_token,
                balance_nonce_sequence,
            ),
            burned: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::burned_amount_label(),
                balance_nonce_sequence,
            ),
            total_supply: token_total_supply_acl_address(
                fixture.mint,
                fixture.total_supply_authority,
                total_supply_nonce_sequence,
            ),
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
    fn canonical(
        fixture: &TokenMolluskFixture,
        sent_handle: [u8; 32],
        from_nonce_sequence: u64,
        to_nonce_sequence: u64,
    ) -> Self {
        Self {
            settlement: token::transfer_callback_settlement_address(fixture.mint, sent_handle).0,
            to_output: token_balance_acl_address(
                fixture.mint,
                fixture.bob_token,
                to_nonce_sequence,
            ),
            refund: token_acl_address(
                fixture.mint,
                fixture.bob_token,
                token::callback_refund_amount_label(),
                to_nonce_sequence,
            ),
            from_output: token_balance_acl_address(
                fixture.mint,
                fixture.alice_token,
                from_nonce_sequence,
            ),
            final_transferred: token_acl_address(
                fixture.mint,
                fixture.alice_token,
                token::callback_final_transferred_label(),
                from_nonce_sequence,
            ),
        }
    }

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
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.alice_current_compute_acl,
            amount_compute_acl: amount_acl_address(
                fixture.mint,
                fixture.owner,
                DEFAULT_INPUT_NONCE_SEQUENCE,
            ),
            from_output_acl: output.alice,
            transferred_amount_acl: output.transferred,
            to_output_acl: output.to_output,
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
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        amount_acl_address(fixture.mint, fixture.owner, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    )
}

fn direct_transfer_ix_with_acls(
    fixture: &TokenMolluskFixture,
    payer: Pubkey,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
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
            from_current_compute_acl,
            to_current_compute_acl,
            amount_compute_acl,
            from_output_acl: output.from_output,
            transferred_amount_acl: output.transferred,
            to_output_acl: output.to_output,
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
    disclosure_verifier_set: Pubkey,
    redemption_verifier_set: Pubkey,
    total_supply_acl_record: Pubkey,
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
            disclosure_verifier_set,
            redemption_verifier_set,
            total_supply_acl_record,
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

fn migrate_mint_verifier_sets_ix(fixture: &TokenMolluskFixture, payer: Pubkey) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::MigrateMintVerifierSets {
            payer,
            authority: fixture.owner,
            mint: fixture.mint,
            disclosure_verifier_set: fixture.disclosure_verifier_set,
            redemption_verifier_set: fixture.redemption_verifier_set,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::MigrateMintVerifierSets {},
    )
}

#[allow(clippy::too_many_arguments)]
fn initialize_token_account_ix(
    owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    token_account: Pubkey,
    acl_record: Pubkey,
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
            acl_record,
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
            current_compute_acl: fixture.alice_current_compute_acl,
            current_total_supply_acl: fixture.total_supply_current_acl,
            output_acl: output.balance,
            total_supply_output_acl: output.total_supply,
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

fn burn_ix(
    fixture: &TokenMolluskFixture,
    current_compute_acl: Pubkey,
    current_total_supply_acl: Pubkey,
    output: BurnOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    burn_ix_with_amount_acl(
        fixture,
        current_compute_acl,
        current_total_supply_acl,
        burn_amount_acl_address(fixture.mint, fixture.owner, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    )
}

fn burn_ix_with_amount_acl(
    fixture: &TokenMolluskFixture,
    current_compute_acl: Pubkey,
    current_total_supply_acl: Pubkey,
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
            current_compute_acl,
            current_total_supply_acl,
            amount_compute_acl,
            output_acl: output.balance,
            burned_amount_acl: output.burned,
            total_supply_output_acl: output.total_supply,
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
            disclosure_verifier_set: fixture.disclosure_verifier_set,
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

fn request_disclose_balance_ix(fixture: &TokenMolluskFixture) -> Instruction {
    request_disclose_balance_ix_with_nonce(fixture, request_nonce(1))
}

fn request_disclose_balance_ix_with_nonce(
    fixture: &TokenMolluskFixture,
    request_nonce: [u8; 32],
) -> Instruction {
    request_disclose_balance_ix_with_nonce_and_expires(
        fixture,
        request_nonce,
        DEFAULT_REQUEST_EXPIRES_SLOT,
    )
}

fn request_disclose_balance_ix_with_nonce_and_expires(
    fixture: &TokenMolluskFixture,
    request_nonce: [u8; 32],
    expires_slot: u64,
) -> Instruction {
    request_disclose_balance_ix_with_owner_and_deny_record_and_nonce(
        fixture,
        fixture.owner,
        None,
        request_nonce,
        expires_slot,
    )
}

fn request_disclose_balance_ix_with_deny_record(
    fixture: &TokenMolluskFixture,
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    request_disclose_balance_ix_with_owner_and_deny_record(
        fixture,
        fixture.owner,
        deny_subject_record,
    )
}

fn request_disclose_balance_ix_with_owner(
    fixture: &TokenMolluskFixture,
    owner: Pubkey,
) -> Instruction {
    request_disclose_balance_ix_with_owner_and_deny_record(fixture, owner, None)
}

fn request_disclose_balance_ix_with_owner_and_deny_record(
    fixture: &TokenMolluskFixture,
    owner: Pubkey,
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    request_disclose_balance_ix_with_owner_and_deny_record_and_nonce(
        fixture,
        owner,
        deny_subject_record,
        request_nonce(1),
        DEFAULT_REQUEST_EXPIRES_SLOT,
    )
}

fn request_disclose_balance_ix_with_owner_and_deny_record_and_nonce(
    fixture: &TokenMolluskFixture,
    owner: Pubkey,
    deny_subject_record: Option<Pubkey>,
    request_nonce: [u8; 32],
    expires_slot: u64,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RequestDiscloseBalance {
            owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            balance_acl_record: fixture.alice_current_compute_acl,
            balance_material_commitment: host::handle_material_address(
                fixture.alice_current_compute_acl,
            )
            .0,
            disclosure_request: disclosure_request_address(
                fixture,
                owner,
                fixture.alice_initial,
                request_nonce,
            ),
            disclosure_verifier_set: fixture.disclosure_verifier_set,
            authority_permission_record: None,
            deny_subject_record,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RequestDiscloseBalance {
            request_nonce,
            expires_slot,
        },
    )
}

fn disclose_balance_ix(fixture: &TokenMolluskFixture, cleartext_amount: u64) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseBalance {
            mint: fixture.mint,
            token_account: fixture.alice_token,
            balance_acl_record: fixture.alice_current_compute_acl,
            balance_material_commitment: host::handle_material_address(
                fixture.alice_current_compute_acl,
            )
            .0,
            disclosure_request: disclosure_request_address(
                fixture,
                fixture.owner,
                fixture.alice_initial,
                request_nonce(1),
            ),
            disclosure_verifier_set: fixture.disclosure_verifier_set,
            host_config: fixture.host_config,
            instructions_sysvar: sysvar::instructions::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseBalance { cleartext_amount },
    )
}

fn disclose_amount_ix(
    fixture: &TokenMolluskFixture,
    amount_acl_record: Pubkey,
    amount_handle: [u8; 32],
    cleartext_amount: u64,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseAmount {
            mint: fixture.mint,
            amount_acl_record,
            amount_material_commitment: host::handle_material_address(amount_acl_record).0,
            disclosure_request: disclosure_request_address(
                fixture,
                fixture.owner,
                amount_handle,
                request_nonce(1),
            ),
            disclosure_verifier_set: fixture.disclosure_verifier_set,
            host_config: fixture.host_config,
            instructions_sysvar: sysvar::instructions::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseAmount {
            amount_handle,
            cleartext_amount,
        },
    )
}

fn request_burn_redemption_ix(
    fixture: &TokenMolluskFixture,
    burned_amount_acl: Pubkey,
    burned_handle: [u8; 32],
) -> Instruction {
    request_burn_redemption_ix_with_nonce_and_expires(
        fixture,
        burned_amount_acl,
        burned_handle,
        request_nonce(1),
        DEFAULT_REQUEST_EXPIRES_SLOT,
    )
}

fn request_burn_redemption_ix_with_nonce_and_expires(
    fixture: &TokenMolluskFixture,
    burned_amount_acl: Pubkey,
    burned_handle: [u8; 32],
    request_nonce: [u8; 32],
    expires_slot: u64,
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
                request_nonce,
            ),
            redemption_verifier_set: fixture.redemption_verifier_set,
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
            request_nonce,
            expires_slot,
        },
    )
}

fn close_expired_disclosure_request_ix(
    fixture: &TokenMolluskFixture,
    disclosure_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CloseExpiredDisclosureRequest {
            requester: fixture.owner,
            disclosure_request,
        },
        token::instruction::CloseExpiredDisclosureRequest {},
    )
}

fn close_consumed_disclosure_request_ix(
    fixture: &TokenMolluskFixture,
    disclosure_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CloseConsumedDisclosureRequest {
            requester: fixture.owner,
            disclosure_request,
        },
        token::instruction::CloseConsumedDisclosureRequest {},
    )
}

fn close_expired_burn_redemption_request_ix(
    fixture: &TokenMolluskFixture,
    redemption_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CloseExpiredBurnRedemptionRequest {
            owner: fixture.owner,
            redemption_request,
        },
        token::instruction::CloseExpiredBurnRedemptionRequest {},
    )
}

fn close_consumed_burn_redemption_request_ix(
    fixture: &TokenMolluskFixture,
    redemption_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CloseConsumedBurnRedemptionRequest {
            owner: fixture.owner,
            redemption_request,
        },
        token::instruction::CloseConsumedBurnRedemptionRequest {},
    )
}

fn redeem_burned_amount_ix(
    fixture: &TokenMolluskFixture,
    burned_amount_acl: Pubkey,
    redemption_record: Pubkey,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
) -> Instruction {
    redeem_burned_amount_ix_with_vault(
        fixture,
        burned_amount_acl,
        redemption_record,
        burned_handle,
        cleartext_amount,
        fixture.vault_usdc,
    )
}

fn redeem_burned_amount_ix_with_vault(
    fixture: &TokenMolluskFixture,
    burned_amount_acl: Pubkey,
    redemption_record: Pubkey,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
    vault_usdc: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RedeemBurnedAmount {
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
            redemption_verifier_set: fixture.redemption_verifier_set,
            redemption_record,
            instructions_sysvar: sysvar::instructions::ID,
            host_config: fixture.host_config,
            token_program: spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RedeemBurnedAmount {
            burned_handle,
            cleartext_amount,
        },
    )
}

fn disclosure_ed25519_ix(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    handle: [u8; 32],
    cleartext_amount: u64,
) -> Instruction {
    let request = read_disclosure_request(
        context,
        disclosure_request_address(fixture, fixture.owner, handle, request_nonce(1)),
    );
    ed25519_verify_ix(
        &fixture.verifier,
        &disclosure_proof_message_for_request(
            &request,
            request.acl_record,
            handle,
            cleartext_amount,
            token::id(),
            host::id(),
        ),
    )
}

fn redemption_ed25519_ix(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
) -> Instruction {
    let request = read_burn_redemption_request(
        context,
        burn_redemption_request_address(fixture, burned_handle, request_nonce(1)),
    );
    ed25519_verify_ix(
        &fixture.verifier,
        &token::redemption_proof_message_v2(
            token::id(),
            host::id(),
            request.host_config,
            request.chain_id,
            request.mint,
            request.verifier_set,
            request.verifier_set_version,
            burn_redemption_request_address(fixture, burned_handle, request_nonce(1)),
            request.request_hash,
            request.burned_acl_record,
            request.material_commitment_hash,
            request.material_key_id,
            burned_handle,
            cleartext_amount,
            request.owner,
            request.token_account,
            request.underlying_mint,
            request.destination_owner,
            request.destination_account,
        ),
    )
}

fn ed25519_verify_ix(authority: &Keypair, message: &[u8]) -> Instruction {
    const PUBKEY_SIZE: usize = 32;
    const SIGNATURE_SIZE: usize = 64;
    const OFFSETS_SIZE: usize = 14;
    const OFFSETS_START: usize = 2;
    const DATA_START: usize = OFFSETS_START + OFFSETS_SIZE;

    let public_key_offset = DATA_START;
    let signature_offset = public_key_offset + PUBKEY_SIZE;
    let message_data_offset = signature_offset + SIGNATURE_SIZE;
    let signature = authority.sign_message(message);

    let mut data = Vec::with_capacity(message_data_offset + message.len());
    data.push(1);
    data.push(0);
    data.extend_from_slice(&(signature_offset as u16).to_le_bytes());
    data.extend_from_slice(&u16::MAX.to_le_bytes());
    data.extend_from_slice(&(public_key_offset as u16).to_le_bytes());
    data.extend_from_slice(&u16::MAX.to_le_bytes());
    data.extend_from_slice(&(message_data_offset as u16).to_le_bytes());
    data.extend_from_slice(&(message.len() as u16).to_le_bytes());
    data.extend_from_slice(&u16::MAX.to_le_bytes());
    data.extend_from_slice(authority.pubkey().as_ref());
    data.extend_from_slice(signature.as_ref());
    data.extend_from_slice(message);

    Instruction {
        program_id: ed25519_program::ID,
        accounts: Vec::new(),
        data,
    }
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
    let burn_output = BurnOutputAccounts::canonical(fixture, 2, 2);
    let context = fixture.context_with_wrap_and_burn_amount(burn_amount_handle);
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
    to_current_compute_acl: Pubkey,
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
            to_current_compute_acl,
            sent_amount_acl,
            callback_success_acl,
            hook_record: token::transfer_receiver_hook_address(fixture.mint, sent_handle).0,
            settlement_record: output.settlement,
            to_output_acl: output.to_output,
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
    from_current_compute_acl: Pubkey,
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
            from_current_compute_acl,
            sent_amount_acl,
            settlement_record: output.settlement,
            refund_amount_acl: output.refund,
            from_output_acl: output.from_output,
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
        disclosure_verifier_set: fixture.disclosure_verifier_set,
        redemption_verifier_set: fixture.redemption_verifier_set,
        decimals: 6,
        total_supply_handle: fixture.total_supply_initial,
        total_supply_acl_record: fixture.total_supply_current_acl,
        next_total_supply_nonce_sequence: 1,
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

fn legacy_confidential_mint_account(
    fixture: &TokenMolluskFixture,
    legacy_kms_verifier_authority: Pubkey,
) -> Account {
    let mut data = token::ConfidentialMint::DISCRIMINATOR.to_vec();
    token::LegacyConfidentialMintV1 {
        authority: fixture.owner,
        acl_domain_key: fixture.mint,
        compute_signer: fixture.compute_signer,
        underlying_mint: fixture.underlying_mint,
        kms_verifier_authority: legacy_kms_verifier_authority,
        decimals: 6,
        total_supply_handle: fixture.total_supply_initial,
        total_supply_acl_record: fixture.total_supply_current_acl,
        next_total_supply_nonce_sequence: 1,
    }
    .serialize(&mut data)
    .expect("legacy mint should serialize");
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
    balance_acl_record: Pubkey,
) -> Account {
    confidential_token_account_with_bump_and_extra(
        owner,
        mint,
        balance_handle,
        balance_acl_record,
        token::token_account_address(mint, owner).1,
        0,
    )
}

fn confidential_token_account_with_bump_and_extra(
    owner: Pubkey,
    mint: Pubkey,
    balance_handle: [u8; 32],
    balance_acl_record: Pubkey,
    bump: u8,
    extra_bytes: usize,
) -> Account {
    let mut data = serialized_account(token::ConfidentialTokenAccount {
        owner,
        mint,
        balance_handle,
        balance_acl_record,
        next_balance_nonce_sequence: 1,
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

fn verifier_set_account(admin: Pubkey, kind: u8, scope: Pubkey, signer: Pubkey) -> Account {
    let mut signers = [Pubkey::default(); host::MAX_VERIFIER_SET_SIGNERS];
    signers[0] = signer;
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::VerifierSet {
            admin,
            kind,
            scope,
            version: 1,
            threshold: 1,
            signer_count: 1,
            signers,
            state: host::VERIFIER_SET_STATE_ACTIVE,
            created_slot: 0,
            updated_slot: 0,
            bump: host::verifier_set_address(kind, scope, 1).1,
        }),
        owner: host::id(),
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
            input_verifier_set: authority,
            input_verifier_set_version: 1,
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

fn read_burn_redemption_request(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::BurnRedemptionRequest {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing burn redemption request")
        .clone();
    assert_eq!(account.owner, token::id());
    token::BurnRedemptionRequest::try_deserialize(&mut account.data.as_slice())
        .expect("burn redemption request should deserialize")
}

fn burn_redemption_exists(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    let Some(account) = context.account_store.borrow().get(&address).cloned() else {
        return false;
    };
    account.owner == token::id()
        && token::BurnRedemption::try_deserialize(&mut account.data.as_slice()).is_ok()
}

fn disclosure_request_exists(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    let Some(account) = context.account_store.borrow().get(&address).cloned() else {
        return false;
    };
    account.owner == token::id()
        && token::DisclosureRequest::try_deserialize(&mut account.data.as_slice()).is_ok()
}

fn burn_redemption_request_exists(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    let Some(account) = context.account_store.borrow().get(&address).cloned() else {
        return false;
    };
    account.owner == token::id()
        && token::BurnRedemptionRequest::try_deserialize(&mut account.data.as_slice()).is_ok()
}

fn account_lamports(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> u64 {
    context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing account")
        .lamports
}

#[allow(clippy::too_many_arguments)]
fn assert_disclosure_request(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
    fixture: &TokenMolluskFixture,
    mode: u8,
    token_account: Pubkey,
    app_account: Pubkey,
    handle: [u8; 32],
    acl_record: Pubkey,
    request_nonce: [u8; 32],
) {
    let request = read_disclosure_request(context, address);
    let material = read_material_commitment(context, host::handle_material_address(acl_record).0);
    assert_eq!(request.mint, fixture.mint);
    assert_eq!(request.requester, fixture.owner);
    assert_eq!(request.token_account, token_account);
    assert_eq!(request.app_account, app_account);
    assert_eq!(request.handle, handle);
    assert_eq!(request.acl_record, acl_record);
    assert_eq!(
        request.material_commitment,
        host::handle_material_address(acl_record).0
    );
    assert_eq!(
        request.material_commitment_hash,
        material.material_commitment_hash
    );
    assert_eq!(request.material_key_id, material.key_id);
    assert_eq!(request.host_config, fixture.host_config);
    assert_eq!(request.verifier_set, fixture.disclosure_verifier_set);
    assert_eq!(request.verifier_set_version, 1);
    assert_eq!(request.request_nonce, request_nonce);
    assert_eq!(request.chain_id, host::SOLANA_POC_CHAIN_ID);
    assert_eq!(request.expires_slot, DEFAULT_REQUEST_EXPIRES_SLOT);
    assert_eq!(request.mode, mode);
    assert_eq!(request.status, token::REQUEST_STATUS_PENDING);
    assert_eq!(
        request.request_hash,
        token::disclosure_request_hash(
            token::id(),
            address,
            request.mint,
            request.requester,
            request.token_account,
            request.app_account,
            request.handle,
            request.acl_record,
            request.material_commitment,
            request.material_commitment_hash,
            request.material_key_id,
            request.host_config,
            request.verifier_set,
            request.verifier_set_version,
            request.request_nonce,
            request.chain_id,
            request.expires_slot,
            request.mode,
        )
    );
}

fn read_material_commitment(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> host::HandleMaterialCommitment {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing material commitment")
        .clone();
    assert_eq!(account.owner, host::id());
    host::HandleMaterialCommitment::try_deserialize(&mut account.data.as_slice())
        .expect("material commitment should deserialize")
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

fn assert_transfer_created_exactly_three_acl_records(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    acl_records_before: usize,
    output: DirectTransferOutputAccounts,
) {
    assert_eq!(acl_record_count(context), acl_records_before + 3);
    for account in output.all_accounts() {
        assert_rent_funded_acl_record_account(context, account);
    }
}

fn assert_rent_funded_acl_record_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing output ACL account")
        .clone();
    let account_size = 8 + host::AclRecord::SPACE;
    let expected_lamports = context.mollusk.sysvars.rent.minimum_balance(account_size);
    assert_eq!(account.owner, host::id());
    assert_eq!(account.data.len(), account_size);
    assert_eq!(account.lamports, expected_lamports);
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

#[allow(clippy::too_many_arguments)]
fn assert_direct_transfer_failure_preserved_state(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &TokenMolluskFixture,
    expected_alice_handle: [u8; 32],
    expected_alice_acl: Pubkey,
    expected_alice_nonce: u64,
    expected_bob_handle: [u8; 32],
    expected_bob_acl: Pubkey,
    expected_bob_nonce: u64,
    output: DirectTransferOutputAccounts,
) {
    let alice_token = read_token_account(context, fixture.alice_token);
    let bob_token = read_token_account(context, fixture.bob_token);
    assert_eq!(alice_token.balance_handle, expected_alice_handle);
    assert_eq!(alice_token.balance_acl_record, expected_alice_acl);
    assert_eq!(
        alice_token.next_balance_nonce_sequence,
        expected_alice_nonce
    );
    assert_eq!(bob_token.balance_handle, expected_bob_handle);
    assert_eq!(bob_token.balance_acl_record, expected_bob_acl);
    assert_eq!(bob_token.next_balance_nonce_sequence, expected_bob_nonce);
    for account in output.all_accounts() {
        assert_empty_system_account(context, account);
    }
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

fn mutate_acl_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
    mutate: impl FnOnce(&mut host::AclRecord),
) {
    let mut record = read_acl_record(context, address);
    mutate(&mut record);
    seed_account(
        context,
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(record),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    );
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

fn seed_unsealed_material_commitment_for_acl(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    acl_record_address: Pubkey,
    seed: u8,
) -> Pubkey {
    let acl_record = read_acl_record(context, acl_record_address);
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
    material_commitment
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

fn material_commitment_exists(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    let Some(account) = context.account_store.borrow().get(&address).cloned() else {
        return false;
    };
    account.owner == host::id()
        && host::HandleMaterialCommitment::try_deserialize(&mut account.data.as_slice()).is_ok()
}

fn account_data_len(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> usize {
    context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing account")
        .data
        .len()
}

fn extend_account_data(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
    extra_bytes: usize,
) {
    let mut store = context.account_store.borrow_mut();
    let account = store.get_mut(&address).expect("missing account");
    let new_len = account.data.len() + extra_bytes;
    account.data.resize(new_len, 0);
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

fn disclosure_proof_message_for_request(
    request: &token::DisclosureRequest,
    acl_record: Pubkey,
    handle: [u8; 32],
    cleartext_amount: u64,
    token_program_id: Pubkey,
    host_program_id: Pubkey,
) -> Vec<u8> {
    token::disclosure_proof_message_v2(
        token_program_id,
        host_program_id,
        request.host_config,
        request.chain_id,
        request.mint,
        request.mode,
        request.verifier_set,
        request.verifier_set_version,
        token::disclosure_request_address(
            request.mint,
            request.requester,
            request.handle,
            request.request_nonce,
        )
        .0,
        request.request_hash,
        acl_record,
        request.material_commitment_hash,
        request.material_key_id,
        handle,
        cleartext_amount,
    )
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
    balance_nonce_sequence: u64,
    total_supply_nonce_sequence: u64,
) -> PredictedWrapHandles {
    let mut amount_context = [0u8; 32];
    amount_context[24..].copy_from_slice(&amount.to_be_bytes());
    let context_id = transfer_eval_context(
        b"wrap-balance",
        fixture.mint,
        fixture.alice_token,
        fixture.alice_token,
        amount_context,
        balance_nonce_sequence,
        total_supply_nonce_sequence,
    );
    let previous_bank_hash = previous_bank_hash(context);
    let unix_timestamp = context.mollusk.sysvars.clock.unix_timestamp;
    let amount_handle = host::computed_eval_trivial_handle(
        amount_plaintext(amount),
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        0,
    );
    let balance = host::computed_bound_eval_handle(
        host::FheBinaryOpCode::Add,
        old_balance_handle,
        amount_handle,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        1,
        token::balance_nonce_key(fixture.mint, fixture.alice_token),
        balance_nonce_sequence,
    );
    let total_supply = host::computed_bound_eval_handle(
        host::FheBinaryOpCode::Add,
        old_total_supply_handle,
        amount_handle,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        2,
        token::total_supply_nonce_key(fixture.mint, fixture.total_supply_authority),
        total_supply_nonce_sequence,
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
    balance_nonce_sequence: u64,
    total_supply_nonce_sequence: u64,
) -> PredictedBurnHandles {
    let context_id = transfer_eval_context(
        b"burn-balance",
        fixture.mint,
        fixture.alice_token,
        fixture.alice_token,
        amount_handle,
        balance_nonce_sequence,
        total_supply_nonce_sequence,
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
    let balance = host::computed_bound_eval_ternary_handle(
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
        token::balance_nonce_key(fixture.mint, fixture.alice_token),
        balance_nonce_sequence,
    );
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
        balance_nonce_sequence,
    );
    let total_supply = host::computed_bound_eval_handle(
        host::FheBinaryOpCode::Sub,
        old_total_supply_handle,
        burned,
        false,
        BALANCE_FHE_TYPE,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        4,
        token::total_supply_nonce_key(fixture.mint, fixture.total_supply_authority),
        total_supply_nonce_sequence,
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
    let to = read_token_account(context, fixture.bob_token);
    let from_nonce_sequence = from.next_balance_nonce_sequence;
    let to_nonce_sequence = to.next_balance_nonce_sequence;
    let context_id = transfer_eval_context(
        b"combined",
        fixture.mint,
        fixture.alice_token,
        fixture.bob_token,
        amount_handle,
        from_nonce_sequence,
        to_nonce_sequence,
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
    let new_from_handle = host::computed_bound_eval_ternary_handle(
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
        token::balance_nonce_key(fixture.mint, fixture.alice_token),
        from_nonce_sequence,
    );
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
        from_nonce_sequence,
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
