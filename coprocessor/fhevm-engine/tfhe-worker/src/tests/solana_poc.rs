use std::path::PathBuf;

use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::token::spl_token;
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheCiphertexts};
use host_listener::{
    database::tfhe_event_propagate::Handle,
    solana_adapter::{
        decode_anchor_cpi_event, insert_solana_events, solana_transaction_id, SolanaBlockMeta,
        SolanaHostEvent,
    },
};
use litesvm::{types::TransactionMetadata, LiteSVM};
use serial_test::serial;
use solana_sdk::{
    instruction::Instruction,
    message::{Message, VersionedMessage},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction, system_program,
    transaction::VersionedTransaction,
};
use tfhe::prelude::FheTryEncrypt;
use time::{Date, Month, PrimitiveDateTime, Time};
use zama_host::{AclPermission, AclRecord};

use crate::tests::{
    event_helpers::{decrypt_handles, setup_event_harness, wait_until_computed},
    utils::latest_db_key,
};

use confidential_token as token;
use zama_host as host;

const FAST_REAL_FHE_TYPE: u8 = 2;
type SeededCiphertext = ([u8; 32], i16, Vec<u8>);

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_confidential_transfer_with_real_ciphertexts_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut fixture = token_fixture();

    let amount_handle = typed_fast_handle(0x09);
    let new_alice_handle = typed_fast_handle(0x0a);
    let new_bob_handle = typed_fast_handle(0x0b);

    seed_real_fast_ciphertexts(
        &harness.pool,
        &[
            (fixture.alice_initial, 125),
            (fixture.bob_initial, 20),
            (amount_handle, 100),
        ],
    )
    .await?;

    authorize_input_compute_acl(&mut fixture, amount_handle);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(
        &fixture,
        output,
        amount_handle,
        new_alice_handle,
        new_bob_handle,
    );
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let host_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&host_events), 2);
    assert_eq!(count_acl_events(&host_events), 4);

    let transaction_id = solana_transaction_id(signature.as_ref());
    let block = SolanaBlockMeta {
        block_number: 1,
        block_timestamp: PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 11)?,
            Time::MIDNIGHT,
        ),
    };
    let mut db_tx = harness.listener_db.new_transaction().await?;
    let stats = insert_solana_events(
        &harness.listener_db,
        &mut db_tx,
        host_events,
        transaction_id,
        block,
    )
    .await?;
    db_tx.commit().await?;

    assert_eq!(stats.tfhe_events, 2);
    assert_eq!(stats.acl_events, 4);

    wait_until_computed(&harness.app).await?;
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.alice,
            vec![fixture.alice_token],
            vec![UserDecryptHandleEntry {
                handle: new_alice_handle,
                app_context: fixture.alice_token,
                owner: fixture.alice.pubkey(),
                acl_record: output.alice_owner,
            }],
        ),
    ));
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.bob,
            vec![fixture.bob_token],
            vec![UserDecryptHandleEntry {
                handle: new_bob_handle,
                app_context: fixture.bob_token,
                owner: fixture.bob.pubkey(),
                acl_record: output.bob_owner,
            }],
        ),
    ));
    let decrypted = decrypt_handles(
        &harness.pool,
        &[Handle::from(new_alice_handle), Handle::from(new_bob_handle)],
    )
    .await?;

    assert_eq!(decrypted[0].output_type, FAST_REAL_FHE_TYPE as i16);
    assert_eq!(decrypted[0].value, "25");
    assert_eq!(decrypted[1].output_type, FAST_REAL_FHE_TYPE as i16);
    assert_eq!(decrypted[1].value, "120");

    Ok(())
}

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_trivial_encrypt_then_confidential_transfer_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut fixture = token_fixture();

    let amount_handle = typed_fast_handle(0x19);
    let new_alice_handle = typed_fast_handle(0x1a);
    let new_bob_handle = typed_fast_handle(0x1b);

    let initial_ixs = vec![
        trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.fhe_authority,
            125,
            fixture.alice_initial,
        ),
        allow_handle_ix(
            fixture.host_program_id,
            fixture.alice_initial,
            fixture.fhe_authority,
            AclPermission::Compute,
        ),
        trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.fhe_authority,
            20,
            fixture.bob_initial,
        ),
        allow_handle_ix(
            fixture.host_program_id,
            fixture.bob_initial,
            fixture.fhe_authority,
            AclPermission::Compute,
        ),
        trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.fhe_authority,
            100,
            amount_handle,
        ),
        allow_handle_ix(
            fixture.host_program_id,
            amount_handle,
            fixture.fhe_authority,
            AclPermission::Compute,
        ),
    ];
    let (meta, account_keys, signature) =
        send_many_with_meta(&mut fixture.svm, &fixture.alice, initial_ixs);
    let initial_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&initial_events), 3);
    assert_eq!(count_acl_events(&initial_events), 3);

    insert_host_events(&harness.listener_db, initial_events, signature, 1).await?;
    wait_until_computed(&harness.app).await?;

    authorize_input_compute_acl(&mut fixture, amount_handle);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(
        &fixture,
        output,
        amount_handle,
        new_alice_handle,
        new_bob_handle,
    );
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let transfer_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&transfer_events), 2);
    assert_eq!(count_acl_events(&transfer_events), 4);

    insert_host_events(&harness.listener_db, transfer_events, signature, 2).await?;
    wait_until_computed(&harness.app).await?;
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.alice,
            vec![fixture.alice_token],
            vec![UserDecryptHandleEntry {
                handle: new_alice_handle,
                app_context: fixture.alice_token,
                owner: fixture.alice.pubkey(),
                acl_record: output.alice_owner,
            }],
        ),
    ));
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.bob,
            vec![fixture.bob_token],
            vec![UserDecryptHandleEntry {
                handle: new_bob_handle,
                app_context: fixture.bob_token,
                owner: fixture.bob.pubkey(),
                acl_record: output.bob_owner,
            }],
        ),
    ));
    let decrypted = decrypt_handles(
        &harness.pool,
        &[Handle::from(new_alice_handle), Handle::from(new_bob_handle)],
    )
    .await?;

    assert_eq!(decrypted[0].output_type, FAST_REAL_FHE_TYPE as i16);
    assert_eq!(decrypted[0].value, "25");
    assert_eq!(decrypted[1].output_type, FAST_REAL_FHE_TYPE as i16);
    assert_eq!(decrypted[1].value, "120");

    Ok(())
}

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_fhe_rand_creates_ciphertext_and_decrypts() -> Result<(), Box<dyn std::error::Error>>
{
    let harness = setup_event_harness().await?;
    let mut fixture = host_fixture();
    let rand_handle = typed_fast_handle(0x29);

    let ixs = vec![
        fhe_rand_ix(
            fixture.host_program_id,
            fixture.payer.pubkey(),
            [7_u8; 16],
            rand_handle,
        ),
        allow_handle_ix(
            fixture.host_program_id,
            rand_handle,
            fixture.payer.pubkey(),
            AclPermission::UserDecrypt,
        ),
    ];
    let (meta, account_keys, signature) =
        send_many_with_meta(&mut fixture.svm, &fixture.payer, ixs);
    let events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&events), 1);
    assert_eq!(count_acl_events(&events), 1);

    insert_host_events(&harness.listener_db, events, signature, 1).await?;
    wait_until_computed(&harness.app).await?;

    let decrypted = decrypt_handles(&harness.pool, &[Handle::from(rand_handle)]).await?;
    assert_eq!(decrypted[0].output_type, FAST_REAL_FHE_TYPE as i16);
    let value = decrypted[0].value.parse::<u16>()?;
    assert!(value <= u8::MAX as u16);

    Ok(())
}

#[test]
#[ignore = "requires built Solana PoC programs; validates user-decrypt ACL semantics without running the worker"]
fn solana_user_decrypt_acl_invariants_match_evm_semantics() {
    let mut fixture = token_fixture();
    let amount_handle = typed_fast_handle(0x39);
    authorize_input_compute_acl(&mut fixture, amount_handle);
    let new_alice_handle = typed_fast_handle(0x3a);
    let new_bob_handle = typed_fast_handle(0x3b);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(
        &fixture,
        output,
        amount_handle,
        new_alice_handle,
        new_bob_handle,
    );
    send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);

    let valid = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.alice_token],
        vec![UserDecryptHandleEntry {
            handle: new_alice_handle,
            app_context: fixture.alice_token,
            owner: fixture.alice.pubkey(),
            acl_record: output.alice_owner,
        }],
    );
    assert!(kms_like_user_decrypt_check(&fixture.svm, &valid));

    let missing_user_acl = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.alice_token],
        vec![UserDecryptHandleEntry {
            handle: new_alice_handle,
            app_context: fixture.alice_token,
            owner: fixture.alice.pubkey(),
            acl_record: output.alice_compute,
        }],
    );
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &missing_user_acl
    ));

    let missing_app_scope = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.bob_token],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &missing_app_scope
    ));

    let wrong_app_context = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.bob_token],
        vec![UserDecryptHandleEntry {
            app_context: fixture.bob_token,
            ..valid.handles[0]
        }],
    );
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &wrong_app_context
    ));

    let wrong_owner = signed_user_decrypt_request(
        &fixture.bob,
        vec![fixture.alice_token],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_owner));

    let user_equals_app_context = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.alice.pubkey()],
        vec![UserDecryptHandleEntry {
            app_context: fixture.alice.pubkey(),
            ..valid.handles[0]
        }],
    );
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &user_equals_app_context
    ));
}

struct TokenFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    token_program_id: Pubkey,
    alice: Keypair,
    bob: Keypair,
    mint: Keypair,
    fhe_authority: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    alice_current_compute_acl: Pubkey,
    bob_current_compute_acl: Pubkey,
}

struct HostFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    payer: Keypair,
}

#[derive(Clone, Copy)]
struct TransferOutputAccounts {
    alice_owner: Pubkey,
    alice_compute: Pubkey,
    bob_owner: Pubkey,
    bob_compute: Pubkey,
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

fn host_fixture() -> HostFixture {
    let host_program_id = host::id();
    let host_program_path = host_program_so_path();
    assert!(
        host_program_path.exists(),
        "missing {}; run `cd solana && NO_DNA=1 anchor build` before this test",
        host_program_path.display()
    );

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(host_program_id, &host_program_path)
        .unwrap();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    HostFixture {
        svm,
        host_program_id,
        payer,
    }
}

fn token_fixture() -> TokenFixture {
    let host_program_id = host::id();
    let token_program_id = token::id();
    let host_program_path = host_program_so_path();
    let token_program_path = token_program_so_path();
    assert!(
        host_program_path.exists(),
        "missing {}; run `cd solana && NO_DNA=1 anchor build` before this test",
        host_program_path.display()
    );
    assert!(
        token_program_path.exists(),
        "missing {}; run `cd solana && NO_DNA=1 anchor build` before this test",
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
    );

    let alice_token = token_account_address(token_program_id, mint.pubkey(), alice.pubkey());
    let bob_token = token_account_address(token_program_id, mint.pubkey(), bob.pubkey());
    let alice_initial = typed_fast_handle(0x01);
    let bob_initial = typed_fast_handle(0x02);

    initialize_token_account(
        &mut svm,
        &alice,
        token_program_id,
        mint.pubkey(),
        alice_token,
        alice_initial,
    );
    initialize_token_account(
        &mut svm,
        &bob,
        token_program_id,
        mint.pubkey(),
        bob_token,
        bob_initial,
    );

    let alice_current_compute_acl =
        acl_record_address(host_program_id, alice_token, fhe_authority, 0);
    let bob_current_compute_acl = acl_record_address(host_program_id, bob_token, fhe_authority, 0);
    authorize_balance_acl(
        BalanceAclTarget {
            token_program_id,
            host_program_id,
            mint: mint.pubkey(),
            token_account: alice_token,
            acl_record: alice_current_compute_acl,
            subject: fhe_authority,
        },
        &mut svm,
        &alice,
    );
    authorize_balance_acl(
        BalanceAclTarget {
            token_program_id,
            host_program_id,
            mint: mint.pubkey(),
            token_account: bob_token,
            acl_record: bob_current_compute_acl,
            subject: fhe_authority,
        },
        &mut svm,
        &bob,
    );

    TokenFixture {
        svm,
        host_program_id,
        token_program_id,
        alice,
        bob,
        mint,
        fhe_authority,
        alice_token,
        bob_token,
        alice_initial,
        bob_initial,
        alice_current_compute_acl,
        bob_current_compute_acl,
    }
}

fn initialize_token_account(
    svm: &mut LiteSVM,
    owner: &Keypair,
    program_id: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    balance_handle: [u8; 32],
) {
    send(
        svm,
        owner,
        Instruction {
            program_id,
            accounts: token::accounts::InitializeTokenAccount {
                owner: owner.pubkey(),
                mint,
                token_account,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: token::instruction::InitializeTokenAccount { balance_handle }.data(),
        },
    );
}

struct BalanceAclTarget {
    token_program_id: Pubkey,
    host_program_id: Pubkey,
    acl_record: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    subject: Pubkey,
}

fn authorize_balance_acl(target: BalanceAclTarget, svm: &mut LiteSVM, owner: &Keypair) {
    send(
        svm,
        owner,
        Instruction {
            program_id: target.token_program_id,
            accounts: token::accounts::AuthorizeBalanceAcl {
                owner: owner.pubkey(),
                mint: target.mint,
                token_account: target.token_account,
                acl_record: target.acl_record,
                zama_event_authority: event_authority(target.host_program_id),
                zama_program: target.host_program_id,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: token::instruction::AuthorizeBalanceAcl {
                subject: target.subject,
                permission: AclPermission::Compute,
            }
            .data(),
        },
    );
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

fn input_compute_acl_address(fixture: &TokenFixture, handle: [u8; 32]) -> Pubkey {
    acl_record_address(
        fixture.host_program_id,
        fixture.alice.pubkey(),
        fixture.fhe_authority,
        u64::from(handle[0]),
    )
}

fn authorize_input_compute_acl(fixture: &mut TokenFixture, handle: [u8; 32]) {
    let acl_record = input_compute_acl_address(fixture, handle);
    send(
        &mut fixture.svm,
        &fixture.alice,
        Instruction {
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
        },
    );
}

fn transfer_ix(
    fixture: &TokenFixture,
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
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.bob_current_compute_acl,
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

fn trivial_encrypt_ix(
    program_id: Pubkey,
    subject: Pubkey,
    value: u64,
    result: [u8; 32],
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::EmitProtocolEvent {
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::TrivialEncrypt {
            subject,
            plaintext: amount_to_plaintext(value),
            fhe_type: FAST_REAL_FHE_TYPE,
            result,
        }
        .data(),
    }
}

fn allow_handle_ix(
    program_id: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
    permission: AclPermission,
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::EmitProtocolEvent {
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::AllowHandle {
            handle,
            subject,
            permission,
        }
        .data(),
    }
}

fn fhe_rand_ix(
    program_id: Pubkey,
    subject: Pubkey,
    seed: [u8; 16],
    result: [u8; 32],
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::EmitProtocolEvent {
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::FheRand {
            subject,
            seed,
            fhe_type: FAST_REAL_FHE_TYPE,
            result,
        }
        .data(),
    }
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
    );
}

async fn seed_real_fast_ciphertexts(
    pool: &sqlx::PgPool,
    values: &[([u8; 32], u8)],
) -> Result<(), Box<dyn std::error::Error>> {
    let (key, _) = latest_db_key(pool).await;
    let values = values.to_vec();
    let ciphertexts =
        tokio::task::spawn_blocking(move || -> Result<Vec<SeededCiphertext>, String> {
            let client_key = key.cks.expect("test key must include a client key");
            #[cfg(not(feature = "gpu"))]
            tfhe::set_server_key(key.sks);
            #[cfg(feature = "gpu")]
            tfhe::set_server_key(key.csks.decompress());

            values
                .into_iter()
                .map(|(handle, value)| {
                    let ciphertext = tfhe::FheUint8::try_encrypt(value, &client_key)
                        .map_err(|err| err.to_string())?;
                    let supported = SupportedFheCiphertexts::FheUint8(ciphertext);
                    let ty = supported.type_num();
                    let compressed = supported.compress().map_err(|err| err.to_string())?;
                    Ok((handle, ty, compressed))
                })
                .collect()
        })
        .await?
        .map_err(std::io::Error::other)?;

    for (handle, ty, ciphertext) in ciphertexts {
        sqlx::query(
            r#"
                INSERT INTO ciphertexts(handle, ciphertext, ciphertext_version, ciphertext_type)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (handle, ciphertext_version) DO UPDATE
                SET ciphertext = EXCLUDED.ciphertext,
                    ciphertext_type = EXCLUDED.ciphertext_type
            "#,
        )
        .bind(handle.to_vec())
        .bind(ciphertext)
        .bind(current_ciphertext_version())
        .bind(ty)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn insert_host_events(
    listener_db: &host_listener::database::tfhe_event_propagate::Database,
    host_events: Vec<SolanaHostEvent>,
    signature: Signature,
    block_number: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let transaction_id = solana_transaction_id(signature.as_ref());
    let block = SolanaBlockMeta {
        block_number,
        block_timestamp: PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 11)?,
            Time::MIDNIGHT,
        ),
    };
    let mut db_tx = listener_db.new_transaction().await?;
    insert_solana_events(listener_db, &mut db_tx, host_events, transaction_id, block).await?;
    db_tx.commit().await?;
    Ok(())
}

fn host_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<SolanaHostEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_anchor_cpi_event(&ix.instruction.data))
        .collect()
}

fn count_tfhe_events(events: &[SolanaHostEvent]) -> usize {
    events
        .iter()
        .filter(|event| {
            matches!(
                event,
                SolanaHostEvent::FheBinaryOp(_)
                    | SolanaHostEvent::TrivialEncrypt(_)
                    | SolanaHostEvent::FheRand(_)
            )
        })
        .count()
}

fn count_acl_events(events: &[SolanaHostEvent]) -> usize {
    events
        .iter()
        .filter(|event| matches!(event, SolanaHostEvent::AclAllowed(_)))
        .count()
}

fn signed_user_decrypt_request(
    signer: &Keypair,
    allowed_accounts: Vec<Pubkey>,
    handles: Vec<UserDecryptHandleEntry>,
) -> UserDecryptRequest {
    let authorization = UserDecryptAuthorizationPayload {
        user: signer.pubkey(),
        reencryption_public_key: [7; 32],
        allowed_accounts,
        start_timestamp: 1,
        duration_seconds: 300,
        extra_data: b"zama-solana-poc".to_vec(),
    };
    let signature = signer.sign_message(&authorization_payload_bytes(&authorization));

    UserDecryptRequest {
        authorization,
        signature,
        handles,
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
            || authorization.user == entry.app_context
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

fn typed_fast_handle(seed: u8) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[30] = FAST_REAL_FHE_TYPE;
    handle
}

fn amount_to_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn host_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../solana/target/deploy/zama_host.so")
}

fn token_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../solana/target/deploy/confidential_token.so")
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

fn send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) {
    send_with_signers(svm, &payer.pubkey(), ix, &[payer]);
}

fn send_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> (TransactionMetadata, Vec<Pubkey>, Signature) {
    let message =
        Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    let signature = tx.signatures[0];
    (svm.send_transaction(tx).unwrap(), account_keys, signature)
}

fn send_many_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ixs: Vec<Instruction>,
) -> (TransactionMetadata, Vec<Pubkey>, Signature) {
    let message = Message::new_with_blockhash(&ixs, Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    let signature = tx.signatures[0];
    (svm.send_transaction(tx).unwrap(), account_keys, signature)
}

fn send_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ix: Instruction,
    signers: &[&Keypair],
) -> TransactionMetadata {
    let message = Message::new_with_blockhash(&[ix], Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx).unwrap()
}

fn send_many_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ixs: Vec<Instruction>,
    signers: &[&Keypair],
) -> TransactionMetadata {
    let message = Message::new_with_blockhash(&ixs, Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx).unwrap()
}
