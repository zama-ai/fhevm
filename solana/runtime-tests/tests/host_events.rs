use std::path::PathBuf;

use anchor_lang::{
    AccountDeserialize, AnchorDeserialize, Discriminator, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
use confidential_token as token;
use litesvm::{
    types::{TransactionMetadata, TransactionResult},
    LiteSVM,
};
use solana_sdk::{
    instruction::Instruction,
    message::{Message, VersionedMessage},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction, system_program,
    transaction::VersionedTransaction,
};
use zama_host as host;
use zama_host::{AclPermission, AclRecord, FheBinaryOpCode, FheBinaryOpEvent, TrivialEncryptEvent};

#[test]
fn trivial_encrypt_emits_anchor_cpi_event() {
    let program_id = host::id();
    let program_path = host_program_so_path();
    assert!(
        program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this runtime test",
        program_path.display()
    );

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(program_id, &program_path)
        .unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let ix = Instruction {
        program_id,
        accounts: host::accounts::EmitProtocolEvent {
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::TrivialEncrypt {
            subject: payer.pubkey(),
            plaintext: [7; 32],
            fhe_type: 5,
            result: [8; 32],
        }
        .data(),
    };

    let message =
        Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[&payer]).unwrap();

    let meta = svm.send_transaction(tx).unwrap();
    let self_cpi = meta
        .inner_instructions
        .iter()
        .flatten()
        .find(|ix| *ix.instruction.program_id(&account_keys) == program_id)
        .expect("expected emit_cpi! self-CPI instruction");

    let event_prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(TrivialEncryptEvent::DISCRIMINATOR.iter().copied())
        .collect::<Vec<_>>();
    assert!(
        self_cpi.instruction.data.starts_with(&event_prefix),
        "self-CPI data did not start with Anchor event prefix"
    );
}

#[test]
fn bind_acl_record_persists_handle_without_handle_derived_address() {
    let program_id = host::id();
    let program_path = host_program_so_path();
    assert!(
        program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this runtime test",
        program_path.display()
    );

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(program_id, &program_path)
        .unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let acl_nonce = 42;
    let scope_authority = Keypair::new();
    let scope = scope_authority.pubkey();
    let subject = payer.pubkey();
    let handle = [9; 32];
    let acl_record = acl_record_address(program_id, scope, subject, acl_nonce);

    let ix = Instruction {
        program_id,
        accounts: host::accounts::BindAclRecord {
            payer: payer.pubkey(),
            scope_authority: scope,
            acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::BindAclRecord {
            acl_nonce,
            scope,
            handle,
            subject,
            permission: AclPermission::Compute,
        }
        .data(),
    };

    send_with_signers(&mut svm, &payer.pubkey(), ix, &[&payer, &scope_authority]).unwrap();

    let account = svm
        .get_account(&acl_record)
        .expect("expected ACL record account");
    let mut data = account.data.as_slice();
    let record = AclRecord::try_deserialize(&mut data).unwrap();
    assert_eq!(record.acl_nonce, acl_nonce);
    assert_eq!(record.scope, scope);
    assert_eq!(record.handle, handle);
    assert_eq!(record.subject, subject);
    assert_eq!(record.permission, AclPermission::Compute);

    let assert_ix = Instruction {
        program_id,
        accounts: host::accounts::AssertAclRecord { acl_record }.to_account_metas(None),
        data: host::instruction::AssertAclRecord {
            scope,
            handle,
            subject,
            permission: AclPermission::Compute,
        }
        .data(),
    };
    send(&mut svm, &payer, assert_ix);
}

#[test]
fn bind_acl_record_rejects_scope_without_matching_authority() {
    let program_id = host::id();
    let program_path = host_program_so_path();
    assert!(
        program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this runtime test",
        program_path.display()
    );

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(program_id, &program_path)
        .unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let scope = Pubkey::new_unique();
    let subject = payer.pubkey();
    let acl_record = acl_record_address(program_id, scope, subject, 0);
    let ix = Instruction {
        program_id,
        accounts: host::accounts::BindAclRecord {
            payer: payer.pubkey(),
            scope_authority: payer.pubkey(),
            acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::BindAclRecord {
            acl_nonce: 0,
            scope,
            handle: [9; 32],
            subject,
            permission: AclPermission::Compute,
        }
        .data(),
    };

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn confidential_transfer_rotates_balance_handles_and_binds_output_acl() {
    let mut fixture = token_fixture();
    let amount_handle = [9; 32];
    authorize_input_compute_acl(&mut fixture, amount_handle);
    let new_alice = [3; 32];
    let new_bob = [4; 32];
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle, new_alice, new_bob);
    assert_eq!(transfer_ix.accounts.len(), 14);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let events = binary_op_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].version, 0);
    assert_eq!(events[0].op, FheBinaryOpCode::Sub);
    assert_eq!(events[0].subject, fixture.fhe_authority);
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, [9; 32]);
    assert!(!events[0].scalar);
    assert_eq!(events[0].result, new_alice);
    assert_eq!(events[1].version, 0);
    assert_eq!(events[1].op, FheBinaryOpCode::Add);
    assert_eq!(events[1].subject, fixture.fhe_authority);
    assert_eq!(events[1].lhs, fixture.bob_initial);
    assert_eq!(events[1].rhs, [9; 32]);
    assert!(!events[1].scalar);
    assert_eq!(events[1].result, new_bob);

    let alice_account = token_account(&fixture.svm, fixture.alice_token);
    let bob_account = token_account(&fixture.svm, fixture.bob_token);
    assert_eq!(alice_account.balance_handle, new_alice);
    assert_eq!(alice_account.next_acl_nonce, 2);
    assert_eq!(bob_account.balance_handle, new_bob);
    assert_eq!(bob_account.next_acl_nonce, 2);

    assert_acl(
        &fixture.svm,
        output.alice_owner,
        fixture.alice_token,
        1,
        new_alice,
        fixture.alice.pubkey(),
        AclPermission::UserDecrypt,
    );
    assert_acl(
        &fixture.svm,
        output.alice_compute,
        fixture.alice_token,
        1,
        new_alice,
        fixture.fhe_authority,
        AclPermission::Compute,
    );
    assert_acl(
        &fixture.svm,
        output.bob_owner,
        fixture.bob_token,
        1,
        new_bob,
        fixture.bob.pubkey(),
        AclPermission::UserDecrypt,
    );
    assert_acl(
        &fixture.svm,
        output.bob_compute,
        fixture.bob_token,
        1,
        new_bob,
        fixture.fhe_authority,
        AclPermission::Compute,
    );
}

#[test]
fn user_decrypt_model_uses_scope_as_app_context() {
    let mut fixture = token_fixture();
    let amount_handle = [9; 32];
    authorize_input_compute_acl(&mut fixture, amount_handle);
    let new_alice = [3; 32];
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle, new_alice, [4; 32]);
    send(&mut fixture.svm, &fixture.alice, transfer_ix);

    let request =
        signed_current_balance_user_decrypt_request(&fixture, fixture.alice_token, &fixture.alice);
    assert_eq!(request.handles[0].handle, new_alice);
    assert_eq!(request.handles[0].acl_record, output.alice_owner);

    assert!(kms_like_user_decrypt_check(&fixture.svm, &request));

    let mut wrong_context = request.clone();
    wrong_context.handles[0] = UserDecryptHandleEntry {
        app_context: fixture.bob_token,
        ..request.handles[0]
    };
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_context));

    let mut narrow_authorization = request.clone();
    narrow_authorization.authorization.allowed_accounts = vec![fixture.bob_token];
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &narrow_authorization
    ));

    let mut wrong_signature = request.clone();
    wrong_signature.signature = fixture
        .bob
        .sign_message(&authorization_payload_bytes(&wrong_signature.authorization));
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_signature));

    let mut wrong_acl_locator = request;
    wrong_acl_locator.handles[0].acl_record = output.bob_owner;
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &wrong_acl_locator
    ));
}

#[test]
fn wrap_usdc_escrows_spl_tokens_and_rotates_confidential_balance() {
    let mut fixture = token_fixture();
    let amount = 100_000_000;
    let amount_handle = [9; 32];
    authorize_input_compute_acl(&mut fixture, amount_handle);
    let new_alice = [5; 32];
    let output = wrap_output_accounts(&fixture, 1);
    let ix = wrap_usdc_ix(&fixture, output, amount, amount_handle, new_alice);

    let alice_usdc_before = spl_token_amount(&fixture.svm, fixture.alice_usdc);
    let vault_usdc_before = spl_token_amount(&fixture.svm, fixture.vault_usdc);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);

    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.alice_usdc),
        alice_usdc_before - amount
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.vault_usdc),
        vault_usdc_before + amount
    );

    let trivial_events = trivial_encrypt_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(trivial_events.len(), 1);
    assert_eq!(trivial_events[0].subject, fixture.fhe_authority);
    assert_eq!(trivial_events[0].plaintext, amount_plaintext(amount));
    assert_eq!(trivial_events[0].result, amount_handle);

    let events = binary_op_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].op, FheBinaryOpCode::Add);
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, amount_handle);
    assert_eq!(events[0].result, new_alice);

    let alice_account = token_account(&fixture.svm, fixture.alice_token);
    assert_eq!(alice_account.balance_handle, new_alice);
    assert_eq!(alice_account.next_acl_nonce, 2);

    assert_acl(
        &fixture.svm,
        output.owner,
        fixture.alice_token,
        1,
        new_alice,
        fixture.alice.pubkey(),
        AclPermission::UserDecrypt,
    );
    assert_acl(
        &fixture.svm,
        output.compute,
        fixture.alice_token,
        1,
        new_alice,
        fixture.fhe_authority,
        AclPermission::Compute,
    );
}

#[test]
fn confidential_transfer_budget_snapshot() {
    let mut fixture = token_fixture();
    authorize_input_compute_acl(&mut fixture, [9; 32]);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, [9; 32], [3; 32], [4; 32]);
    let top_level_metas = transfer_ix.accounts.len();
    let writable_metas = transfer_ix
        .accounts
        .iter()
        .filter(|account| account.is_writable)
        .count();
    let signer_metas = transfer_ix
        .accounts
        .iter()
        .filter(|account| account.is_signer)
        .count();

    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let inner_instructions = meta.inner_instructions.iter().flatten().count();
    let host_events = binary_op_events(&meta, &account_keys, fixture.host_program_id).len();
    let max_cpi_depth = max_cpi_depth(&meta);

    assert_eq!(top_level_metas, 14);
    assert_eq!(account_keys.len(), 15);
    assert_eq!(writable_metas, 7);
    assert_eq!(signer_metas, 1);
    assert_eq!(inner_instructions, 16);
    assert_eq!(host_events, 2);
    assert_eq!(created_acl_count(&fixture.svm, output), 4);
    assert!(
        meta.compute_units_consumed <= 150_000,
        "compute units: {}",
        meta.compute_units_consumed
    );
    assert_eq!(max_cpi_depth, 3);
}

#[test]
fn confidential_transfer_rejects_stale_current_acl() {
    let mut fixture = token_fixture();
    authorize_input_compute_acl(&mut fixture, [9; 32]);
    let first_output = transfer_output_accounts(&fixture, 1);
    let first_ix = transfer_ix(&fixture, first_output, [9; 32], [3; 32], [4; 32]);
    send(&mut fixture.svm, &fixture.alice, first_ix);

    authorize_input_compute_acl(&mut fixture, [8; 32]);
    let stale_ix = transfer_ix(
        &fixture,
        transfer_output_accounts(&fixture, 2),
        [8; 32],
        [5; 32],
        [6; 32],
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, stale_ix).is_err());
}

#[test]
fn confidential_transfer_rejects_wrong_acl_scope() {
    let mut fixture = token_fixture();
    authorize_input_compute_acl(&mut fixture, [9; 32]);
    let ix = transfer_ix_with_current_acl(
        &fixture,
        fixture.bob_current_compute_acl,
        fixture.bob_current_compute_acl,
        transfer_output_accounts(&fixture, 1),
        [9; 32],
        [3; 32],
        [4; 32],
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
}

#[test]
fn confidential_transfer_rejects_wrong_fhe_authority() {
    let mut fixture = token_fixture();
    authorize_input_compute_acl(&mut fixture, [9; 32]);
    let wrong_authority = Pubkey::new_unique();
    let wrong_authority_acl = acl_record_address(
        fixture.host_program_id,
        fixture.alice_token,
        wrong_authority,
        0,
    );
    authorize_balance_acl(
        &mut fixture.svm,
        &fixture.alice,
        fixture.token_program_id,
        fixture.host_program_id,
        wrong_authority_acl,
        fixture.mint.pubkey(),
        fixture.alice_token,
        wrong_authority,
        AclPermission::Compute,
    );

    let ix = transfer_ix_with_current_acl(
        &fixture,
        wrong_authority_acl,
        fixture.bob_current_compute_acl,
        transfer_output_accounts(&fixture, 1),
        [9; 32],
        [3; 32],
        [4; 32],
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
}

struct TokenFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    token_program_id: Pubkey,
    alice: Keypair,
    bob: Keypair,
    mint: Keypair,
    underlying_mint: Keypair,
    fhe_authority: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_usdc: Pubkey,
    vault_usdc: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    alice_current_compute_acl: Pubkey,
    bob_current_compute_acl: Pubkey,
}

#[derive(Clone, Copy)]
struct TransferOutputAccounts {
    alice_owner: Pubkey,
    alice_compute: Pubkey,
    bob_owner: Pubkey,
    bob_compute: Pubkey,
}

#[derive(Clone, Copy)]
struct WrapOutputAccounts {
    owner: Pubkey,
    compute: Pubkey,
}

#[derive(Clone)]
struct UserDecryptAuthorizationPayload {
    user: Pubkey,
    reencryption_public_key: [u8; 32],
    allowed_accounts: Vec<Pubkey>,
    start_timestamp: i64,
    duration_seconds: u64,
    extra_data: Vec<u8>,
}

#[derive(Clone)]
struct UserDecryptRequest {
    authorization: UserDecryptAuthorizationPayload,
    signature: Signature,
    handles: Vec<UserDecryptHandleEntry>,
}

#[derive(Clone, Copy)]
struct UserDecryptHandleEntry {
    handle: [u8; 32],
    app_context: Pubkey,
    owner: Pubkey,
    acl_record: Pubkey,
}

fn host_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/zama_host.so")
}

fn token_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/confidential_token.so")
}

fn token_fixture() -> TokenFixture {
    let host_program_id = host::id();
    let token_program_id = token::id();
    let host_program_path = host_program_so_path();
    let token_program_path = token_program_so_path();
    assert!(
        host_program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this runtime test",
        host_program_path.display()
    );
    assert!(
        token_program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this runtime test",
        token_program_path.display()
    );

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(host_program_id, &host_program_path)
        .unwrap();
    svm.add_program_from_file(token_program_id, &token_program_path)
        .unwrap();

    let alice = Keypair::new();
    let bob = Keypair::new();
    let mint = Keypair::new();
    let underlying_mint = Keypair::new();
    svm.airdrop(&alice.pubkey(), 2_000_000_000).unwrap();
    svm.airdrop(&bob.pubkey(), 1_000_000_000).unwrap();

    create_spl_mint(&mut svm, &alice, &underlying_mint, 6);
    let vault_authority = vault_authority_address(token_program_id, mint.pubkey());
    let alice_usdc = Keypair::new();
    let vault_usdc = Keypair::new();
    create_spl_token_account(
        &mut svm,
        &alice,
        &alice_usdc,
        underlying_mint.pubkey(),
        alice.pubkey(),
    );
    create_spl_token_account(
        &mut svm,
        &alice,
        &vault_usdc,
        underlying_mint.pubkey(),
        vault_authority,
    );
    mint_spl_to(
        &mut svm,
        &alice,
        underlying_mint.pubkey(),
        alice_usdc.pubkey(),
        1_000_000_000,
    );

    let fhe_authority = Pubkey::new_unique();
    send_with_signers(
        &mut svm,
        &alice.pubkey(),
        Instruction {
            program_id: token_program_id,
            accounts: token::accounts::InitializeMint {
                authority: alice.pubkey(),
                mint: mint.pubkey(),
                underlying_mint: underlying_mint.pubkey(),
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: token::instruction::InitializeMint { fhe_authority }.data(),
        },
        &[&alice, &mint],
    )
    .unwrap();

    let alice_token = token_account_address(token_program_id, mint.pubkey(), alice.pubkey());
    let bob_token = token_account_address(token_program_id, mint.pubkey(), bob.pubkey());
    let alice_initial = [1; 32];
    let bob_initial = [2; 32];

    send(
        &mut svm,
        &alice,
        Instruction {
            program_id: token_program_id,
            accounts: token::accounts::InitializeTokenAccount {
                owner: alice.pubkey(),
                mint: mint.pubkey(),
                token_account: alice_token,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: token::instruction::InitializeTokenAccount {
                balance_handle: alice_initial,
            }
            .data(),
        },
    );
    send(
        &mut svm,
        &bob,
        Instruction {
            program_id: token_program_id,
            accounts: token::accounts::InitializeTokenAccount {
                owner: bob.pubkey(),
                mint: mint.pubkey(),
                token_account: bob_token,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: token::instruction::InitializeTokenAccount {
                balance_handle: bob_initial,
            }
            .data(),
        },
    );

    let alice_current_compute_acl =
        acl_record_address(host_program_id, alice_token, fhe_authority, 0);
    let bob_current_compute_acl = acl_record_address(host_program_id, bob_token, fhe_authority, 0);
    authorize_balance_acl(
        &mut svm,
        &alice,
        token_program_id,
        host_program_id,
        alice_current_compute_acl,
        mint.pubkey(),
        alice_token,
        fhe_authority,
        AclPermission::Compute,
    );
    authorize_balance_acl(
        &mut svm,
        &bob,
        token_program_id,
        host_program_id,
        bob_current_compute_acl,
        mint.pubkey(),
        bob_token,
        fhe_authority,
        AclPermission::Compute,
    );

    TokenFixture {
        svm,
        host_program_id,
        token_program_id,
        alice,
        bob,
        mint,
        underlying_mint,
        fhe_authority,
        alice_token,
        bob_token,
        alice_usdc: alice_usdc.pubkey(),
        vault_usdc: vault_usdc.pubkey(),
        alice_initial,
        bob_initial,
        alice_current_compute_acl,
        bob_current_compute_acl,
    }
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn token_account_address(program_id: Pubkey, mint: Pubkey, owner: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"token-account", mint.as_ref(), owner.as_ref()],
        &program_id,
    )
    .0
}

fn vault_authority_address(program_id: Pubkey, mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"vault-authority", mint.as_ref()], &program_id).0
}

fn acl_record_address(
    program_id: Pubkey,
    scope: Pubkey,
    subject: Pubkey,
    acl_nonce: u64,
) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"acl",
            scope.as_ref(),
            subject.as_ref(),
            &acl_nonce.to_le_bytes(),
        ],
        &program_id,
    )
    .0
}

fn input_compute_acl_address(fixture: &TokenFixture, handle: [u8; 32]) -> Pubkey {
    acl_record_address(
        fixture.host_program_id,
        fixture.alice.pubkey(),
        fixture.fhe_authority,
        u64::from(handle[0]),
    )
}

fn transfer_output_accounts(fixture: &TokenFixture, acl_nonce: u64) -> TransferOutputAccounts {
    TransferOutputAccounts {
        alice_owner: acl_record_address(
            fixture.host_program_id,
            fixture.alice_token,
            fixture.alice.pubkey(),
            acl_nonce,
        ),
        alice_compute: acl_record_address(
            fixture.host_program_id,
            fixture.alice_token,
            fixture.fhe_authority,
            acl_nonce,
        ),
        bob_owner: acl_record_address(
            fixture.host_program_id,
            fixture.bob_token,
            fixture.bob.pubkey(),
            acl_nonce,
        ),
        bob_compute: acl_record_address(
            fixture.host_program_id,
            fixture.bob_token,
            fixture.fhe_authority,
            acl_nonce,
        ),
    }
}

fn authorize_input_compute_acl(fixture: &mut TokenFixture, handle: [u8; 32]) {
    let acl_record = input_compute_acl_address(fixture, handle);
    let ix = Instruction {
        program_id: fixture.host_program_id,
        accounts: host::accounts::BindAclRecord {
            payer: fixture.alice.pubkey(),
            scope_authority: fixture.alice.pubkey(),
            acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.host_program_id),
            program: fixture.host_program_id,
        }
        .to_account_metas(None),
        data: host::instruction::BindAclRecord {
            acl_nonce: u64::from(handle[0]),
            scope: fixture.alice.pubkey(),
            handle,
            subject: fixture.fhe_authority,
            permission: AclPermission::Compute,
        }
        .data(),
    };
    send(&mut fixture.svm, &fixture.alice, ix);
}

fn wrap_output_accounts(fixture: &TokenFixture, acl_nonce: u64) -> WrapOutputAccounts {
    WrapOutputAccounts {
        owner: acl_record_address(
            fixture.host_program_id,
            fixture.alice_token,
            fixture.alice.pubkey(),
            acl_nonce,
        ),
        compute: acl_record_address(
            fixture.host_program_id,
            fixture.alice_token,
            fixture.fhe_authority,
            acl_nonce,
        ),
    }
}

fn transfer_ix(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    new_from_handle: [u8; 32],
    new_to_handle: [u8; 32],
) -> Instruction {
    transfer_ix_with_current_acl(
        fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        output,
        amount_handle,
        new_from_handle,
        new_to_handle,
    )
}

fn transfer_ix_with_current_acl(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    new_from_handle: [u8; 32],
    new_to_handle: [u8; 32],
) -> Instruction {
    Instruction {
        program_id: fixture.token_program_id,
        accounts: token::accounts::ConfidentialTransfer {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            from_current_compute_acl,
            to_current_compute_acl,
            amount_compute_acl: input_compute_acl_address(fixture, amount_handle),
            from_owner_output_acl: output.alice_owner,
            from_compute_output_acl: output.alice_compute,
            to_owner_output_acl: output.bob_owner,
            to_compute_output_acl: output.bob_compute,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: token::instruction::ConfidentialTransfer {
            amount_handle,
            new_from_handle,
            new_to_handle,
        }
        .data(),
    }
}

fn wrap_usdc_ix(
    fixture: &TokenFixture,
    output: WrapOutputAccounts,
    amount: u64,
    amount_handle: [u8; 32],
    new_balance_handle: [u8; 32],
) -> Instruction {
    Instruction {
        program_id: fixture.token_program_id,
        accounts: token::accounts::WrapUsdc {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint.pubkey(),
            user_usdc: fixture.alice_usdc,
            vault_usdc: fixture.vault_usdc,
            vault_authority: vault_authority_address(
                fixture.token_program_id,
                fixture.mint.pubkey(),
            ),
            current_compute_acl: fixture.alice_current_compute_acl,
            amount_compute_acl: input_compute_acl_address(fixture, amount_handle),
            owner_output_acl: output.owner,
            compute_output_acl: output.compute,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            token_program: spl_token::id(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: token::instruction::WrapUsdc {
            amount,
            amount_handle,
            new_balance_handle,
        }
        .data(),
    }
}

fn authorize_balance_acl(
    svm: &mut LiteSVM,
    owner: &Keypair,
    token_program_id: Pubkey,
    host_program_id: Pubkey,
    acl_record: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    subject: Pubkey,
    permission: AclPermission,
) {
    send(
        svm,
        owner,
        Instruction {
            program_id: token_program_id,
            accounts: token::accounts::AuthorizeBalanceAcl {
                owner: owner.pubkey(),
                mint,
                token_account,
                acl_record,
                zama_event_authority: event_authority(host_program_id),
                zama_program: host_program_id,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: token::instruction::AuthorizeBalanceAcl {
                subject,
                permission,
            }
            .data(),
        },
    );
}

fn create_spl_mint(svm: &mut LiteSVM, payer: &Keypair, mint: &Keypair, decimals: u8) {
    let rent = svm.minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN);
    send_many_with_signers(
        svm,
        &payer.pubkey(),
        vec![
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint2(
                &spl_token::id(),
                &mint.pubkey(),
                &payer.pubkey(),
                None,
                decimals,
            )
            .unwrap(),
        ],
        &[payer, mint],
    )
    .unwrap();
}

fn create_spl_token_account(
    svm: &mut LiteSVM,
    payer: &Keypair,
    token_account: &Keypair,
    mint: Pubkey,
    owner: Pubkey,
) {
    let rent = svm.minimum_balance_for_rent_exemption(spl_token::state::Account::LEN);
    send_many_with_signers(
        svm,
        &payer.pubkey(),
        vec![
            system_instruction::create_account(
                &payer.pubkey(),
                &token_account.pubkey(),
                rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account3(
                &spl_token::id(),
                &token_account.pubkey(),
                &mint,
                &owner,
            )
            .unwrap(),
        ],
        &[payer, token_account],
    )
    .unwrap();
}

fn mint_spl_to(
    svm: &mut LiteSVM,
    mint_authority: &Keypair,
    mint: Pubkey,
    token_account: Pubkey,
    amount: u64,
) {
    send(
        svm,
        mint_authority,
        spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint,
            &token_account,
            &mint_authority.pubkey(),
            &[],
            amount,
        )
        .unwrap(),
    );
}

fn spl_token_amount(svm: &LiteSVM, address: Pubkey) -> u64 {
    let account = svm
        .get_account(&address)
        .expect("expected SPL token account");
    spl_token::state::Account::unpack(&account.data)
        .unwrap()
        .amount
}

fn amount_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn token_account(svm: &LiteSVM, address: Pubkey) -> token::ConfidentialTokenAccount {
    let account = svm
        .get_account(&address)
        .expect("expected confidential token account");
    let mut data = account.data.as_slice();
    token::ConfidentialTokenAccount::try_deserialize(&mut data).unwrap()
}

fn signed_current_balance_user_decrypt_request(
    fixture: &TokenFixture,
    token_account_address: Pubkey,
    signer: &Keypair,
) -> UserDecryptRequest {
    let account = token_account(&fixture.svm, token_account_address);
    let acl_nonce = account.next_acl_nonce.checked_sub(1).unwrap();
    let acl_record = acl_record_address(
        fixture.host_program_id,
        token_account_address,
        account.owner,
        acl_nonce,
    );
    let authorization = UserDecryptAuthorizationPayload {
        user: account.owner,
        reencryption_public_key: [7; 32],
        allowed_accounts: vec![token_account_address],
        start_timestamp: 1,
        duration_seconds: 300,
        extra_data: b"zama-solana-poc".to_vec(),
    };
    let signature = signer.sign_message(&authorization_payload_bytes(&authorization));

    UserDecryptRequest {
        authorization,
        signature,
        handles: vec![UserDecryptHandleEntry {
            handle: account.balance_handle,
            app_context: token_account_address,
            owner: account.owner,
            acl_record,
        }],
    }
}

fn authorization_payload_bytes(authorization: &UserDecryptAuthorizationPayload) -> Vec<u8> {
    let mut bytes = b"Zama Solana UserDecrypt v0".to_vec();
    bytes.extend_from_slice(authorization.user.as_ref());
    bytes.extend_from_slice(&authorization.reencryption_public_key);
    bytes.extend_from_slice(&(authorization.allowed_accounts.len() as u32).to_le_bytes());
    for account in &authorization.allowed_accounts {
        bytes.extend_from_slice(account.as_ref());
    }
    bytes.extend_from_slice(&authorization.start_timestamp.to_le_bytes());
    bytes.extend_from_slice(&authorization.duration_seconds.to_le_bytes());
    bytes.extend_from_slice(&(authorization.extra_data.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&authorization.extra_data);
    bytes
}

fn assert_acl(
    svm: &LiteSVM,
    address: Pubkey,
    scope: Pubkey,
    acl_nonce: u64,
    handle: [u8; 32],
    subject: Pubkey,
    permission: AclPermission,
) {
    let account = svm.get_account(&address).expect("expected ACL account");
    let mut data = account.data.as_slice();
    let record = AclRecord::try_deserialize(&mut data).unwrap();
    assert_eq!(record.acl_nonce, acl_nonce);
    assert_eq!(record.scope, scope);
    assert_eq!(record.handle, handle);
    assert_eq!(record.subject, subject);
    assert_eq!(record.permission, permission);
}

fn kms_like_user_decrypt_check(svm: &LiteSVM, request: &UserDecryptRequest) -> bool {
    let authorization = &request.authorization;
    let signed_payload = authorization_payload_bytes(authorization);
    if !request
        .signature
        .verify(authorization.user.as_ref(), &signed_payload)
        || authorization.reencryption_public_key == [0; 32]
        || authorization.duration_seconds == 0
        || authorization.extra_data.is_empty()
        || authorization.start_timestamp < 0
        || request.handles.is_empty()
    {
        return false;
    }
    request.handles.iter().all(|entry| {
        if authorization.user != entry.owner
            || !authorization.allowed_accounts.contains(&entry.app_context)
        {
            return false;
        }
        let Some(record) = read_acl_record(svm, entry.acl_record) else {
            return false;
        };
        record.scope == entry.app_context
            && record.handle == entry.handle
            && record.subject == authorization.user
            && record.permission == AclPermission::UserDecrypt
    })
}

fn read_acl_record(svm: &LiteSVM, address: Pubkey) -> Option<AclRecord> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    AclRecord::try_deserialize(&mut data).ok()
}

fn created_acl_count(svm: &LiteSVM, output: TransferOutputAccounts) -> usize {
    [
        output.alice_owner,
        output.alice_compute,
        output.bob_owner,
        output.bob_compute,
    ]
    .into_iter()
    .filter(|address| svm.get_account(address).is_some())
    .count()
}

fn binary_op_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<FheBinaryOpEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_binary_op_event(&ix.instruction.data))
        .collect()
}

fn trivial_encrypt_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<TrivialEncryptEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_trivial_encrypt_event(&ix.instruction.data))
        .collect()
}

fn max_cpi_depth(meta: &TransactionMetadata) -> u64 {
    meta.logs
        .iter()
        .filter_map(|log| {
            log.strip_suffix(']')?
                .rsplit_once(" invoke [")?
                .1
                .parse::<u64>()
                .ok()
        })
        .max()
        .unwrap_or(1)
}

fn decode_binary_op_event(data: &[u8]) -> Option<FheBinaryOpEvent> {
    let event_prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(FheBinaryOpEvent::DISCRIMINATOR.iter().copied())
        .collect::<Vec<_>>();
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheBinaryOpEvent::deserialize(&mut &*payload).ok()
}

fn decode_trivial_encrypt_event(data: &[u8]) -> Option<TrivialEncryptEvent> {
    let event_prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(TrivialEncryptEvent::DISCRIMINATOR.iter().copied())
        .collect::<Vec<_>>();
    let payload = data.strip_prefix(&event_prefix[..])?;
    TrivialEncryptEvent::deserialize(&mut &*payload).ok()
}

fn send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) {
    try_send(svm, payer, ix).unwrap();
}

fn send_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> (TransactionMetadata, Vec<Pubkey>) {
    let message =
        Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    (svm.send_transaction(tx).unwrap(), account_keys)
}

fn try_send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) -> TransactionResult {
    send_with_signers(svm, &payer.pubkey(), ix, &[payer])
}

fn send_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ix: Instruction,
    signers: &[&Keypair],
) -> TransactionResult {
    let message = Message::new_with_blockhash(&[ix], Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx)
}

fn send_many_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ixs: Vec<Instruction>,
    signers: &[&Keypair],
) -> TransactionResult {
    let message = Message::new_with_blockhash(&ixs, Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx)
}
