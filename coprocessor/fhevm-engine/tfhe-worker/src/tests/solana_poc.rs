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
use zama_host::{AclPermission, AclRecord, AclSubjectEntry};

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
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;
    let new_bob_handle = read_acl_record(&fixture.svm, output.bob)
        .expect("expected Bob output ACL")
        .handle;
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
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_alice_handle,
                owner: fixture.alice.pubkey(),
                acl_record: output.alice,
            }],
        ),
    ));
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.bob,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_bob_handle,
                owner: fixture.bob.pubkey(),
                acl_record: output.bob,
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

    let initial_ixs = vec![
        trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.compute_signer,
            125,
            fixture.alice_initial,
        ),
        allow_handle_ix(
            fixture.host_program_id,
            fixture.alice_initial,
            fixture.compute_signer,
            AclPermission::Compute,
        ),
        trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.compute_signer,
            20,
            fixture.bob_initial,
        ),
        allow_handle_ix(
            fixture.host_program_id,
            fixture.bob_initial,
            fixture.compute_signer,
            AclPermission::Compute,
        ),
        trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.compute_signer,
            100,
            amount_handle,
        ),
        allow_handle_ix(
            fixture.host_program_id,
            amount_handle,
            fixture.compute_signer,
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
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;
    let new_bob_handle = read_acl_record(&fixture.svm, output.bob)
        .expect("expected Bob output ACL")
        .handle;
    let transfer_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&transfer_events), 2);
    assert_eq!(count_acl_events(&transfer_events), 4);

    insert_host_events(&harness.listener_db, transfer_events, signature, 2).await?;
    wait_until_computed(&harness.app).await?;
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.alice,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_alice_handle,
                owner: fixture.alice.pubkey(),
                acl_record: output.alice,
            }],
        ),
    ));
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.bob,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_bob_handle,
                owner: fixture.bob.pubkey(),
                acl_record: output.bob,
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
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;

    let valid = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: new_alice_handle,
            owner: fixture.alice.pubkey(),
            acl_record: output.alice,
        }],
    );
    assert!(kms_like_user_decrypt_check(&fixture.svm, &valid));

    let missing_user_acl = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: new_alice_handle,
            owner: fixture.alice.pubkey(),
            acl_record: output.bob,
        }],
    );
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &missing_user_acl
    ));

    let missing_domain = signed_user_decrypt_request(
        &fixture.alice,
        vec![Pubkey::new_unique()],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &missing_domain));

    let wrong_owner = signed_user_decrypt_request(
        &fixture.bob,
        vec![fixture.mint.pubkey()],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_owner));

    let wrong_handle = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: typed_fast_handle(0x7f),
            ..valid.handles[0]
        }],
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_handle));
}

struct TokenFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    token_program_id: Pubkey,
    alice: Keypair,
    bob: Keypair,
    mint: Keypair,
    compute_signer: Pubkey,
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
    alice: Pubkey,
    bob: Pubkey,
}

#[derive(Clone)]
struct UserDecryptAuthorizationPayload {
    user: Pubkey,
    reencryption_public_key: [u8; 32],
    allowed_acl_domain_keys: Vec<Pubkey>,
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
    owner: Pubkey,
    acl_record: Pubkey,
}

fn host_fixture() -> HostFixture {
    let host_program_id = host::id();
    let host_program_path = host_program_so_path();
    assert!(
        host_program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this test",
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
        "missing {}; run `cd solana && anchor build` before this test",
        host_program_path.display()
    );
    assert!(
        token_program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this test",
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
            data: token::instruction::InitializeMint {}.data(),
        },
        &[&alice, &mint],
    );

    let compute_signer = token::compute_signer_address(mint.pubkey()).0;
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
        balance_acl_record_address(host_program_id, mint.pubkey(), alice_token, 0);
    let bob_current_compute_acl =
        balance_acl_record_address(host_program_id, mint.pubkey(), bob_token, 0);
    authorize_balance_acl(
        BalanceAclTarget {
            token_program_id,
            host_program_id,
            mint: mint.pubkey(),
            token_account: alice_token,
            acl_record: alice_current_compute_acl,
            compute_signer,
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
            compute_signer,
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
        compute_signer,
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
    compute_signer: Pubkey,
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
                compute_signer: target.compute_signer,
                token_account: target.token_account,
                acl_record: target.acl_record,
                zama_event_authority: event_authority(target.host_program_id),
                zama_program: target.host_program_id,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: token::instruction::AuthorizeBalanceAcl {}.data(),
        },
    );
}

fn transfer_output_accounts(fixture: &TokenFixture, nonce_sequence: u64) -> TransferOutputAccounts {
    TransferOutputAccounts {
        alice: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            nonce_sequence,
        ),
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            nonce_sequence,
        ),
    }
}

fn input_compute_acl_address(fixture: &TokenFixture, handle: [u8; 32]) -> Pubkey {
    acl_record_address(
        fixture.host_program_id,
        token::nonce_key(
            fixture.alice.pubkey(),
            fixture.alice.pubkey(),
            label("input"),
        ),
        u64::from(handle[0]),
    )
}

fn authorize_input_compute_acl(fixture: &mut TokenFixture, handle: [u8; 32]) {
    // Temporary PoC stand-in for the future Solana InputVerifier/transient
    // authorization path. It lets host::fhe_binary_op enforce both operands.
    let acl_record = input_compute_acl_address(fixture, handle);
    let acl_domain_key = fixture.alice.pubkey();
    let app_account = fixture.alice.pubkey();
    let encrypted_value_label = label("input");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = u64::from(handle[0]);
    send(
        &mut fixture.svm,
        &fixture.alice,
        Instruction {
            program_id: fixture.host_program_id,
            accounts: host::accounts::BindAclRecord {
                payer: fixture.alice.pubkey(),
                app_account_authority: app_account,
                acl_record,
                system_program: system_program::ID,
                event_authority: event_authority(fixture.host_program_id),
                program: fixture.host_program_id,
            }
            .to_account_metas(None),
            data: host::instruction::BindAclRecord {
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                handle,
                subjects: vec![AclSubjectEntry {
                    pubkey: fixture.compute_signer,
                    permission: AclPermission::Compute,
                }],
                public_decrypt: false,
            }
            .data(),
        },
    );
}

fn transfer_ix(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    Instruction {
        program_id: fixture.token_program_id,
        accounts: token::accounts::ConfidentialTransfer {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.bob_current_compute_acl,
            amount_compute_acl: input_compute_acl_address(fixture, amount_handle),
            from_output_acl: output.alice,
            to_output_acl: output.bob,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: token::instruction::ConfidentialTransfer { amount_handle }.data(),
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
    allowed_acl_domain_keys: Vec<Pubkey>,
    handles: Vec<UserDecryptHandleEntry>,
) -> UserDecryptRequest {
    let authorization = UserDecryptAuthorizationPayload {
        user: signer.pubkey(),
        reencryption_public_key: [7; 32],
        allowed_acl_domain_keys,
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
    bytes.extend_from_slice(&(authorization.allowed_acl_domain_keys.len() as u32).to_le_bytes());
    for account in &authorization.allowed_acl_domain_keys {
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
        if authorization.user != entry.owner {
            return false;
        }

        let Some(raw_account) = svm.get_account(&entry.acl_record) else {
            return false;
        };
        if raw_account.owner != host::id() {
            return false;
        }
        let mut data = raw_account.data.as_slice();
        let Ok(record) = AclRecord::try_deserialize(&mut data) else {
            return false;
        };
        let expected_nonce_key = token::nonce_key(
            record.acl_domain_key,
            record.app_account,
            record.encrypted_value_label,
        );
        let expected_acl_record =
            acl_record_address(host::id(), expected_nonce_key, record.nonce_sequence);

        authorization
            .allowed_acl_domain_keys
            .contains(&record.acl_domain_key)
            && record.handle == entry.handle
            && record.nonce_key == expected_nonce_key
            && entry.acl_record == expected_acl_record
            && record_subjects(&record).contains(&authorization.user)
    })
}

fn record_subjects(record: &AclRecord) -> Vec<Pubkey> {
    record.subjects[..record.subject_count as usize].to_vec()
}

fn read_acl_record(svm: &LiteSVM, address: Pubkey) -> Option<AclRecord> {
    let raw_account = svm.get_account(&address)?;
    let mut data = raw_account.data.as_slice();
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

fn acl_record_address(program_id: Pubkey, nonce_key: [u8; 32], nonce_sequence: u64) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &program_id,
    )
    .0
}

fn balance_acl_record_address(
    program_id: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    nonce_sequence: u64,
) -> Pubkey {
    acl_record_address(
        program_id,
        token::balance_nonce_key(acl_domain_key, app_account),
        nonce_sequence,
    )
}

fn label(name: &str) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let bytes = name.as_bytes();
    assert!(bytes.len() <= out.len());
    out[..bytes.len()].copy_from_slice(bytes);
    out
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
