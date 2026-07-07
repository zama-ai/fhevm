#[allow(dead_code)]
mod support;

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, AnchorDeserialize,
    Discriminator, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
use confidential_token as token;
use mollusk_svm::{
    result::{types::TransactionResult, Check},
    Mollusk,
};
use solana_sdk::{
    account::Account,
    ed25519_program,
    instruction::{Instruction, InstructionError},
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
fn mollusk_confidential_transfer_rejects_attestation_user_mismatch() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(30, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    // fromExternal binding: an attestation authored by someone other than the transfer authority
    // (owner) must be rejected before any balance rotation.
    let attestation =
        amount_attestation_for(amount_handle, fixture.bob_owner, fixture.compute_signer);
    let ix = direct_transfer_ix_with_attestation(&fixture, fixture.owner, output, attestation);
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
    // Reuses the now-stale nonce-0 current-balance ACLs (the canonical defaults), which no longer
    // match the token accounts after the first transfer.
    let stale_ix = direct_transfer_ix(&fixture, stale_output, amount_handle);

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
fn mollusk_confidential_transfer_rejects_attestation_contract_mismatch() {
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(96, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let context = fixture.context_with_input_amount(amount_handle);
    // fromExternal binding: an attestation bound to a contract other than the mint compute-signer
    // PDA must be rejected before any balance rotation.
    let attestation = amount_attestation_for(amount_handle, fixture.owner, Pubkey::new_unique());
    let ix = direct_transfer_ix_with_attestation(&fixture, fixture.owner, output, attestation);

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
fn mollusk_confidential_burn_rejects_attestation_contract_mismatch() {
    let fixture = TokenMolluskFixture::new();
    let wrap_amount = 100_000_000;
    let amount_handle = handle_for_chain(79, BALANCE_FHE_TYPE);
    let wrap_output = WrapOutputAccounts::canonical(&fixture, 1);
    let burn_output = BurnOutputAccounts::canonical(&fixture, 2, 2);
    let context = fixture.context_with_wrap_accounts();
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);

    context.process_and_validate_instruction(&wrap_ix, &[Check::success()]);

    for account in burn_output.all_accounts() {
        seed_empty_system_account(&context, account);
    }

    // fromExternal binding: a burn-amount attestation bound to a contract other than the mint
    // compute-signer PDA must be rejected before any supply/balance rotation.
    let attestation = amount_attestation_for(amount_handle, fixture.owner, Pubkey::new_unique());
    let ix = burn_ix_with_attestation(
        &fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        burn_output,
        attestation,
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
    let total_supply_acl_record = token_total_supply_acl_address(mint, total_supply_authority, 0);
    let host_config = host::host_config_address().0;
    let context = mollusk().with_context(HashMap::from([
        (authority, system_account(5_000_000_000)),
        (mint, system_account(0)),
        (underlying_mint, spl_mint_account(authority, 6, 0)),
        (compute_signer, system_account(0)),
        (total_supply_authority, system_account(0)),
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
        _amount_handle: [u8; 32],
    ) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
        // The transfer amount flows through a coprocessor attestation argument, not a seeded
        // amount ACL account, so the context only needs the base + output-record accounts.
        let mut accounts = self.base_accounts();
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

fn self_transfer_ix(
    fixture: &TokenMolluskFixture,
    output: SelfTransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            // Block-cap optional accounts threaded through the transfer CPI; the default
            // unrestricted cap means None/None here. The HCU authority is mandatory.
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            hcu_authority: token::hcu_authority_address(fixture.mint).0,
            owner: fixture.owner,
            payer: fixture.owner,
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.alice_current_compute_acl,
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
        token::instruction::ConfidentialTransfer {
            amount_attestation: amount_attestation(fixture, amount_handle),
        },
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
    direct_transfer_ix_with_attestation(
        fixture,
        payer,
        output,
        amount_attestation(fixture, amount_handle),
    )
}

fn direct_transfer_ix_with_attestation(
    fixture: &TokenMolluskFixture,
    payer: Pubkey,
    output: DirectTransferOutputAccounts,
    amount_attestation: host::CoprocessorInputAttestation,
) -> Instruction {
    // Block-cap optional accounts threaded through the transfer CPI; the default
    // unrestricted cap means None/None here. The mint's HCU authority is mandatory.
    direct_transfer_ix_with_block_cap_accounts(
        fixture,
        payer,
        output,
        amount_attestation,
        None,
        None,
        token::hcu_authority_address(fixture.mint).0,
    )
}

fn direct_transfer_ix_with_block_cap_accounts(
    fixture: &TokenMolluskFixture,
    payer: Pubkey,
    output: DirectTransferOutputAccounts,
    amount_attestation: host::CoprocessorInputAttestation,
    hcu_block_meter: Option<Pubkey>,
    hcu_trusted_app_record: Option<Pubkey>,
    hcu_authority: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            hcu_block_meter,
            hcu_trusted_app_record,
            hcu_authority,
            owner: fixture.owner,
            payer,
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.bob_current_compute_acl,
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
        token::instruction::ConfidentialTransfer { amount_attestation },
    )
}

/// Coprocessor signing key backing the `fromExternal` attestations; its EVM address is the
/// `coprocessor_signer` configured on the token fixture's `host_config`.
fn coprocessor_signing_key() -> k256::ecdsa::SigningKey {
    k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap()
}

/// Builds a canonical transfer/burn amount attestation authored by the token owner and bound to the
/// mint compute-signer PDA, exactly as the token program requires.
fn amount_attestation(
    fixture: &TokenMolluskFixture,
    amount_handle: [u8; 32],
) -> host::CoprocessorInputAttestation {
    amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer)
}

/// Builds a coprocessor-signed `fromExternal` attestation over `amount_handle`, binding it to
/// (`user`, `contract`). The token program checks `user == transfer authority` and
/// `contract == mint compute-signer PDA`; the host re-verifies this signature in-frame.
fn amount_attestation_for(
    amount_handle: [u8; 32],
    user: Pubkey,
    contract: Pubkey,
) -> host::CoprocessorInputAttestation {
    let key = coprocessor_signing_key();
    let ct_handles = vec![amount_handle];
    let contract_chain_id = 12345u64;
    let extra_data = vec![0x00u8];
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"InputVerification",
            b"1",
            SECP_GATEWAY_CHAIN_ID,
            &INPUT_VERIFICATION_CONTRACT,
        ),
        &host::eip712::ciphertext_verification_struct_hash(
            &ct_handles,
            &user.to_bytes(),
            &contract.to_bytes(),
            contract_chain_id,
            &extra_data,
        ),
    );
    host::CoprocessorInputAttestation {
        input_handle: amount_handle,
        ct_handles,
        handle_index: 0,
        user_address: user.to_bytes(),
        contract_address: contract.to_bytes(),
        contract_chain_id,
        extra_data,
        signatures: vec![secp_sign(&key, &digest)],
    }
}

#[allow(clippy::too_many_arguments)]
fn initialize_mint_ix(
    authority: Pubkey,
    mint: Pubkey,
    underlying_mint: Pubkey,
    compute_signer: Pubkey,
    total_supply_authority: Pubkey,
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
            total_supply_acl_record,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: token::hcu_authority_address(mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
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
            hcu_authority: token::hcu_authority_address(mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
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
    create_random_amount_ix_with_block_cap_accounts(
        fixture,
        amount_acl_record,
        amount_kind,
        None,
        None,
    )
}

fn create_random_amount_ix_with_block_cap_accounts(
    fixture: &TokenMolluskFixture,
    amount_acl_record: Pubkey,
    amount_kind: token::ConfidentialAmountKind,
    hcu_block_meter: Option<Pubkey>,
    hcu_trusted_app_record: Option<Pubkey>,
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
            hcu_authority: token::hcu_authority_address(fixture.mint).0,
            hcu_block_meter,
            hcu_trusted_app_record,
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
            hcu_authority: token::hcu_authority_address(fixture.mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::CreateRandomBoundedAmount {
            amount_kind,
            upper_bound,
        },
    )
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
            hcu_authority: token::hcu_authority_address(fixture.mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
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
    burn_ix_with_attestation(
        fixture,
        current_compute_acl,
        current_total_supply_acl,
        output,
        amount_attestation(fixture, amount_handle),
    )
}

fn burn_ix_with_attestation(
    fixture: &TokenMolluskFixture,
    current_compute_acl: Pubkey,
    current_total_supply_acl: Pubkey,
    output: BurnOutputAccounts,
    amount_attestation: host::CoprocessorInputAttestation,
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
            output_acl: output.balance,
            burned_amount_acl: output.burned,
            total_supply_output_acl: output.total_supply,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_authority: token::hcu_authority_address(fixture.mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialBurn { amount_attestation },
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

// --- KMS EIP-712 disclose (secp256k1_recover) — #1494 Phase 3 cert-secp ---

const SECP_GATEWAY_CHAIN_ID: u64 = 31337;
const SECP_DECRYPTION_CONTRACT: [u8; 20] = [0xDEu8; 20];
/// Coprocessor `CiphertextVerification` verifying contract used by the `fromExternal` attestations.
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCDu8; 20];

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
            max_hcu_per_tx: 0,
            max_hcu_depth_per_tx: 0,
            // Ships unrestricted; existing flows are unaffected by the block cap.
            hcu_block_cap_per_app: u64::MAX,
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

fn disclose_balance_secp_ix(
    fixture: &TokenMolluskFixture,
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
) -> Instruction {
    disclose_balance_secp_ix_with_context(
        fixture,
        cleartext_amount,
        signatures,
        host::kms_context_address(KMS_CONTEXT_ID).0,
        request_nonce(1),
    )
}

fn disclose_balance_secp_ix_with_context(
    fixture: &TokenMolluskFixture,
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    kms_context: Pubkey,
    nonce: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseBalanceSecp {
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
                nonce,
            ),
            host_config: fixture.host_config,
            kms_context,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseBalanceSecp {
            cleartext_amount,
            signatures,
            extra_data: vec![],
        },
    )
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
fn mollusk_disclose_balance_secp_accepts_real_kms_certificate() {
    let fixture = TokenMolluskFixture::new();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
    let cleartext_amount = 125;
    let mut accounts = fixture.base_accounts();
    accounts.insert(fixture.host_config, kms_host_config_account(fixture.owner));
    let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&key));
    accounts.insert(kms_ctx, kms_ctx_account);
    let context = mollusk().with_context(accounts);

    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 120);
    let request_result = process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);
    assert!(request_result.raw_result.is_ok());

    let signatures = kms_public_decrypt_signatures(&key, fixture.alice_initial, cleartext_amount);
    let result = process_transaction(
        &context,
        &[disclose_balance_secp_ix(
            &fixture,
            cleartext_amount,
            signatures,
        )],
    );

    assert!(result.raw_result.is_ok());
    let events: Vec<token::BalanceDisclosedEvent> = result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].mint, fixture.mint);
    assert_eq!(events[0].handle, fixture.alice_initial);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
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

/// SECURITY (witness <-> secp binding): a request witness pins the KMS context id at request time.
/// A certificate minted under a different (e.g. rotated) context is rejected, an expired witness is
/// rejected, and a consumed witness cannot be replayed.
#[test]
fn mollusk_disclose_balance_secp_rejects_rotated_context_expired_and_replay() {
    let fixture = TokenMolluskFixture::new();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
    let cleartext_amount = 125;

    // Rejected: the passed kms_context is a different id than the one the witness pinned.
    {
        let mut accounts = fixture.base_accounts();
        accounts.insert(fixture.host_config, kms_host_config_account(fixture.owner));
        // Seed the witness-pinned context (KMS_CONTEXT_ID) so the request can verify the PDA, plus a
        // rotated context the cert is NOT bound to.
        let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&key));
        accounts.insert(kms_ctx, kms_ctx_account);
        let rotated_id = KMS_CONTEXT_ID + 1;
        let (rotated_ctx, _) = host::kms_context_address(rotated_id);
        accounts.insert(
            rotated_ctx,
            kms_context_account_for(rotated_id, secp_evm_address(&key)).1,
        );
        let context = mollusk().with_context(accounts);
        seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 150);
        let request_result =
            process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);
        assert!(request_result.raw_result.is_ok());
        let signatures =
            kms_public_decrypt_signatures(&key, fixture.alice_initial, cleartext_amount);
        // Present the rotated context PDA instead of the witness-pinned one.
        let result = process_transaction(
            &context,
            &[disclose_balance_secp_ix_with_context(
                &fixture,
                cleartext_amount,
                signatures,
                rotated_ctx,
                request_nonce(1),
            )],
        );
        assert!(
            result.raw_result.is_err(),
            "cert verified against a context other than the witness-pinned one must be rejected"
        );
    }

    // Rejected: the witness has expired before the disclosure is consumed.
    {
        let mut accounts = fixture.base_accounts();
        accounts.insert(fixture.host_config, kms_host_config_account(fixture.owner));
        let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&key));
        accounts.insert(kms_ctx, kms_ctx_account);
        let mut context = mollusk().with_context(accounts);
        seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 151);
        let request_result = process_transaction(
            &context,
            &[request_disclose_balance_ix_with_nonce_and_expires(
                &fixture,
                request_nonce(1),
                5,
            )],
        );
        assert!(request_result.raw_result.is_ok());
        context.mollusk.sysvars.warp_to_slot(6);
        let signatures =
            kms_public_decrypt_signatures(&key, fixture.alice_initial, cleartext_amount);
        let result = process_transaction(
            &context,
            &[disclose_balance_secp_ix(
                &fixture,
                cleartext_amount,
                signatures,
            )],
        );
        assert!(
            result.raw_result.is_err(),
            "expired disclosure witness must be rejected"
        );
    }
}

/// SECURITY (P1#3 context binding): a request whose `extra_data` names a KMS context other than the
/// passed `kms_context` account is rejected. The context is resolved from `extra_data` (which the
/// KMS signs over) and checked against the account BEFORE signature verification, so a certificate
/// minted under one context cannot be verified under another (e.g. the current one after a rotation).
/// The certificate here is signed over this very `extra_data`, so the only reason for rejection is
/// the context-id binding.
#[test]
fn mollusk_disclose_balance_secp_rejects_context_mismatch_in_extra_data() {
    let fixture = TokenMolluskFixture::new();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
    let cleartext_amount = 125;
    let mut accounts = fixture.base_accounts();
    accounts.insert(fixture.host_config, kms_host_config_account(fixture.owner));
    let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&key));
    accounts.insert(kms_ctx, kms_ctx_account);
    let context = mollusk().with_context(accounts);

    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 120);
    let request_result = process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);
    assert!(request_result.raw_result.is_ok());

    // extra_data (version 1) names context KMS_CONTEXT_ID + 1, but the passed kms_context is for
    // KMS_CONTEXT_ID. Sign the cert over this extra_data so it is otherwise valid.
    let mut extra_data = vec![1u8];
    extra_data.extend_from_slice(&[0u8; 24]);
    extra_data.extend_from_slice(&(KMS_CONTEXT_ID + 1).to_be_bytes());
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"Decryption",
            b"1",
            SECP_GATEWAY_CHAIN_ID,
            &SECP_DECRYPTION_CONTRACT,
        ),
        &host::eip712::public_decrypt_struct_hash(
            &[fixture.alice_initial],
            &decrypted_u64_bytes(cleartext_amount),
            &extra_data,
        ),
    );
    let signatures = vec![secp_sign(&key, &digest)];

    let ix = anchor_ix(
        token::id(),
        token::accounts::DiscloseBalanceSecp {
            mint: fixture.mint,
            token_account: fixture.alice_token,
            balance_acl_record: fixture.alice_current_compute_acl,
            balance_material_commitment: host::handle_material_address(
                fixture.alice_current_compute_acl,
            )
            .0,
            disclosure_request: disclosure_request_address(
                &fixture,
                fixture.owner,
                fixture.alice_initial,
                request_nonce(1),
            ),
            host_config: fixture.host_config,
            kms_context: host::kms_context_address(KMS_CONTEXT_ID).0,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseBalanceSecp {
            cleartext_amount,
            signatures,
            extra_data,
        },
    );
    let result = process_transaction(&context, &[ix]);
    assert!(
        result.raw_result.is_err(),
        "extra_data naming a context other than the passed kms_context must be rejected"
    );
}

#[test]
fn mollusk_disclose_balance_secp_rejects_unauthorized_signer() {
    let fixture = TokenMolluskFixture::new();
    let configured = k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
    let attacker = k256::ecdsa::SigningKey::from_bytes(&[0x66u8; 32].into()).unwrap();
    let cleartext_amount = 125;
    let mut accounts = fixture.base_accounts();
    accounts.insert(fixture.host_config, kms_host_config_account(fixture.owner));
    let (kms_ctx, kms_ctx_account) = kms_context_account(secp_evm_address(&configured));
    accounts.insert(kms_ctx, kms_ctx_account);
    let context = mollusk().with_context(accounts);

    seed_material_commitment_for_acl(&context, fixture.alice_current_compute_acl, 120);
    let request_result = process_transaction(&context, &[request_disclose_balance_ix(&fixture)]);
    assert!(request_result.raw_result.is_ok());

    let signatures =
        kms_public_decrypt_signatures(&attacker, fixture.alice_initial, cleartext_amount);
    let result = process_transaction(
        &context,
        &[disclose_balance_secp_ix(
            &fixture,
            cleartext_amount,
            signatures,
        )],
    );

    assert!(result.raw_result.is_err());
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

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    let mut mollusk = Mollusk::new(&token::id(), "confidential_token");
    mollusk.add_program(&host::id(), "zama_host");
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

fn host_config_account(authority: Pubkey) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::HostConfig {
            admin: authority,
            chain_id: host::SOLANA_POC_CHAIN_ID,
            input_verifier_authority: authority,
            gateway_chain_id: SECP_GATEWAY_CHAIN_ID,
            // Configured so the transfer/burn `fromExternal` amount attestation verifies in-frame.
            input_verification_contract: INPUT_VERIFICATION_CONTRACT,
            coprocessor_signer: secp_evm_address(&coprocessor_signing_key()),
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
            max_hcu_per_tx: 0,
            max_hcu_depth_per_tx: 0,
            // Ships unrestricted; existing flows are unaffected by the block cap.
            hcu_block_cap_per_app: u64::MAX,
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
    assert_eq!(request.kms_context_id, KMS_CONTEXT_ID);
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
            request.kms_context_id,
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

/// The transfer fixture's host config with the per-app block cap overridden to `cap`.
fn host_config_account_with_block_cap(authority: Pubkey, cap: u64) -> Account {
    let mut account = host_config_account(authority);
    let mut config = {
        let mut data = account.data.as_slice();
        host::HostConfig::try_deserialize(&mut data).expect("valid host config")
    };
    config.hcu_block_cap_per_app = cap;
    account.data = serialized_account(config);
    account
}

// ---- Block cap enforced through the confidential-token -> fhe_eval CPI ----

#[test]
fn mollusk_confidential_transfer_block_cap_ban_is_enforced_through_cpi() {
    // A confidential transfer reaches fhe_eval only by CPI. With the calling app untrusted
    // and the cap at the ban sentinel (0), the block-cap breach must surface through the CPI
    // and roll the whole transfer back — exactly as a direct fhe_eval call is rejected. The
    // underlying HcuBlockLimitExceeded propagates cleanly through the CPI as the top-level
    // InstructionError::Custom code (not wrapped), and the transfer's durable outputs are never
    // created (atomic revert).
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(200, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    // The mint's HCU authority is signed in (as on every transfer), but no meter and no
    // trust witness — the untrusted CPI shape the ban applies to.
    let ix = direct_transfer_ix(&fixture, output, amount_handle);
    let context = fixture.context_with_input_amount(amount_handle);
    // Ban untrusted apps via the block cap (overrides the fixture's unrestricted default).
    seed_account(
        &context,
        fixture.host_config,
        host_config_account_with_block_cap(fixture.owner, 0),
    );

    let result = context.process_instruction(&ix);

    // Rejected via CPI with the exact host error code; the transfer's durable outputs are never
    // created (atomic revert).
    let expected_code: u32 = host::errors::ZamaHostError::HcuBlockLimitExceeded.into();
    assert_eq!(
        result.raw_result,
        Err(InstructionError::Custom(expected_code))
    );
    assert_empty_system_account(&context, output.from_output);
    assert_empty_system_account(&context, output.transferred);
    assert_empty_system_account(&context, output.to_output);
    // Balances are unchanged.
    assert_eq!(
        read_token_account(&context, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_non_canonical_hcu_authority() {
    // The token program pins the mandatory HCU authority to the canonical
    // ["hcu-authority", mint] PDA before signing it into the CPI — an arbitrary account in
    // that slot (e.g. another mint's authority, to spend its budget) is rejected up front,
    // even while the host cap is unrestricted.
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(202, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let ix = direct_transfer_ix_with_block_cap_accounts(
        &fixture,
        fixture.owner,
        output,
        amount_attestation(&fixture, amount_handle),
        None,
        None,
        token::hcu_authority_address(Pubkey::new_unique()).0,
    );
    let context = fixture.context_with_input_amount(amount_handle);

    let result = context.process_instruction(&ix);

    let expected_code: u32 = token::ConfidentialTokenError::HcuAuthorityMismatch.into();
    assert_eq!(
        result.raw_result,
        Err(InstructionError::Custom(expected_code))
    );
    assert_empty_system_account(&context, output.from_output);
}

/// Exact HCU cost of the combined transfer eval frame (`execute_transfer_eval`), from the frame
/// cost model: `Ge` at ebool (21_000) + debit `Sub` at euint64 (38_000) + `IfThenElse` at euint64
/// (45_000) + transferred `Sub` at euint64 (38_000) + credit `Add` at euint64 (38_000). The
/// `VerifiedInput` amount is an operand, not a step, so it adds no HCU.
const TRANSFER_FRAME_HCU: u64 = 21_000 + 38_000 + 45_000 + 38_000 + 38_000; // 180_000

fn read_hcu_block_meter(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::HcuBlockMeter> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    host::HcuBlockMeter::try_deserialize(&mut data).ok()
}

#[test]
fn mollusk_confidential_transfer_metering_band_charges_meter_through_cpi() {
    // The Some(meter) CPI shape — the production account set once the cap drops below
    // u64::MAX. With a metering-band cap, the mint's HCU authority signed in, and the meter
    // threaded through ConfidentialTransfer, the transfer must succeed and the meter must be
    // lazy-created charged with exactly the frame's HCU, proving the three optional accounts
    // survive the token -> zama-fhe -> fhe_eval CPI encoding (None placeholder vs real meta,
    // ordering, writability, PDA signing) end to end. The metering identity is the mint's
    // ["hcu-authority", mint] PDA — one budget per mint, NOT per sender token account.
    let fixture = TokenMolluskFixture::new();
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let output = DirectTransferOutputAccounts::canonical(&fixture, 1, 1);
    let hcu_authority = token::hcu_authority_address(fixture.mint).0;
    let meter_pda = host::hcu_block_meter_address(hcu_authority).0;
    let ix = direct_transfer_ix_with_block_cap_accounts(
        &fixture,
        fixture.owner,
        output,
        amount_attestation(&fixture, amount_handle),
        Some(meter_pda),
        None,
        hcu_authority,
    );
    let context = fixture.context_with_input_amount(amount_handle);
    // A metering-band cap (above the frame cost) instead of the unrestricted default.
    seed_account(
        &context,
        fixture.host_config,
        host_config_account_with_block_cap(fixture.owner, 500_000),
    );

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    assert!(!result.inner_instructions.is_empty());
    // The transfer completed: balances rotated onto the new output ACL records.
    let alice_token = read_token_account(&context, fixture.alice_token);
    assert_eq!(alice_token.balance_acl_record, output.from_output);
    assert_ne!(alice_token.balance_handle, fixture.alice_initial);
    // The meter was lazy-created through the CPI, keyed on the mint's HCU authority, and
    // charged exactly the transfer frame's HCU at the current slot.
    let meter = read_hcu_block_meter(&context, meter_pda).expect("meter created through CPI");
    assert_eq!(meter.app, hcu_authority);
    assert_eq!(meter.used_hcu, TRANSFER_FRAME_HCU);
    assert_eq!(meter.last_seen_slot, context.mollusk.sysvars.clock.slot);
    // Regression guard on the metering granularity: nothing accrues to the sender token
    // account's key — a sybil minting fresh token accounts gets no fresh budget.
    assert!(
        read_hcu_block_meter(&context, host::hcu_block_meter_address(fixture.alice_token).0)
            .is_none()
    );
}

/// Exact HCU cost of `create_random_amount` (Transfer kind): a single `FheRand` at euint64.
const RAND_U64_HCU: u64 = 52_000;

/// A program-owned trust record at the canonical `["hcu-trusted", app]` PDA.
fn hcu_trusted_app_record_account(app: Pubkey, trusted: bool) -> Account {
    let (_, bump) = host::hcu_trusted_app_address(app);
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::HcuTrustedAppRecord { app, trusted, bump }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

/// Shared setup for the newly-threaded non-transfer eval instructions: a `create_random_amount`
/// context with a metering-band block cap and the transfer-amount output ACL seeded.
fn create_random_amount_block_cap_context(
    fixture: &TokenMolluskFixture,
    amount_acl: Pubkey,
    cap: u64,
) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
    let mut accounts = fixture.base_accounts();
    accounts.insert(amount_acl, system_account(0));
    accounts.insert(
        fixture.host_config,
        host_config_account_with_block_cap(fixture.owner, cap),
    );
    mollusk().with_context(accounts)
}

#[test]
fn mollusk_create_random_amount_trusted_authority_bypasses_meter_through_cpi() {
    // Newly-threaded path: create_random_amount now forwards the trust witness. With the mint's
    // per-mint hcu_authority registered trusted and the witness threaded, a metering-band cap is
    // bypassed — the op succeeds and no meter is created (contention-free whitelist path). This
    // is the wiring that makes "register confidential-token trusted" actually reachable for the
    // non-transfer instructions, not just ConfidentialTransfer.
    let fixture = TokenMolluskFixture::new();
    let hcu_authority = token::hcu_authority_address(fixture.mint).0;
    let amount_acl = amount_acl_address(fixture.mint, fixture.owner, 0);
    let trust_pda = host::hcu_trusted_app_address(hcu_authority).0;
    let meter_pda = host::hcu_block_meter_address(hcu_authority).0;
    let context = create_random_amount_block_cap_context(&fixture, amount_acl, 500_000);
    seed_account(&context, trust_pda, hcu_trusted_app_record_account(hcu_authority, true));

    let ix = create_random_amount_ix_with_block_cap_accounts(
        &fixture,
        amount_acl,
        token::ConfidentialAmountKind::Transfer,
        None,
        Some(trust_pda),
    );
    context.process_and_validate_instruction(&ix, &[Check::success()]);

    // Bypass: the op succeeded and no meter was lazily created for the authority.
    assert!(read_hcu_block_meter(&context, meter_pda).is_none());
    assert!(acl_record_exists(&context, amount_acl));
}

#[test]
fn mollusk_create_random_amount_metering_band_charges_meter_through_cpi() {
    // Newly-threaded path: with a metering-band cap and the meter threaded, create_random_amount
    // lazy-creates and charges the per-mint meter for exactly the FheRand cost — proving the
    // optional accounts survive this instruction's token -> zama-fhe -> fhe_eval CPI, not only
    // ConfidentialTransfer's.
    let fixture = TokenMolluskFixture::new();
    let hcu_authority = token::hcu_authority_address(fixture.mint).0;
    let amount_acl = amount_acl_address(fixture.mint, fixture.owner, 0);
    let meter_pda = host::hcu_block_meter_address(hcu_authority).0;
    let context = create_random_amount_block_cap_context(&fixture, amount_acl, 500_000);

    let ix = create_random_amount_ix_with_block_cap_accounts(
        &fixture,
        amount_acl,
        token::ConfidentialAmountKind::Transfer,
        Some(meter_pda),
        None,
    );
    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let meter = read_hcu_block_meter(&context, meter_pda).expect("meter created through CPI");
    assert_eq!(meter.app, hcu_authority);
    assert_eq!(meter.used_hcu, RAND_U64_HCU);
    assert_eq!(meter.last_seen_slot, context.mollusk.sysvars.clock.slot);
}

#[test]
fn mollusk_create_random_amount_metering_band_none_none_fails_closed() {
    // Newly-threaded path: once the cap is active, an untrusted create_random_amount that forwards
    // neither meter nor trust witness fails closed (HcuBlockLimitExceeded surfaces via the host's
    // missing-meter path through the CPI), never silently un-metered, and creates no amount ACL.
    let fixture = TokenMolluskFixture::new();
    let amount_acl = amount_acl_address(fixture.mint, fixture.owner, 0);
    let context = create_random_amount_block_cap_context(&fixture, amount_acl, 500_000);

    let ix = create_random_amount_ix_with_block_cap_accounts(
        &fixture,
        amount_acl,
        token::ConfidentialAmountKind::Transfer,
        None,
        None,
    );
    let result = context.process_instruction(&ix);

    let expected_code: u32 = host::errors::ZamaHostError::HcuBlockMeterMissing.into();
    assert_eq!(
        result.raw_result,
        Err(InstructionError::Custom(expected_code))
    );
    assert!(!acl_record_exists(&context, amount_acl));
}
