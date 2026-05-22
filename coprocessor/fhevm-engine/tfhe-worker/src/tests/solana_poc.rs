use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheCiphertexts};
use host_listener::{
    database::tfhe_event_propagate::Handle,
    solana_adapter::{
        decode_anchor_cpi_event, insert_solana_events, solana_transaction_id, SolanaBlockMeta,
        SolanaHostEvent,
    },
};
use litesvm::types::TransactionMetadata;
use serial_test::serial;
use solana_sdk::signature::{Signature, Signer};
use tfhe::prelude::FheTryEncrypt;
use time::{Date, Month, PrimitiveDateTime, Time};
use zama_solana_litesvm_harness::{
    authorize_transfer_amount, collect_cpi_events, kms_like_user_decrypt_check, read_acl_record,
    run_transfer_scenario, send_with_meta, send_with_meta_and_signature,
    signed_user_decrypt_request_with_domains, token_fixture, transfer_ix, transfer_output_accounts,
    TransferExpect, TransferSetup, UserDecryptHandleEntry, BALANCE_FHE_TYPE,
    DEFAULT_INPUT_NONCE_SEQUENCE,
};

use crate::tests::{
    event_helpers::{decrypt_handles, setup_event_harness, wait_until_computed},
    solana_semantic::assert_transfer_worker,
    utils::latest_db_key,
};

type SeededCiphertext = ([u8; 32], i16, Vec<u8>);

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_confidential_transfer_with_real_ciphertexts_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = token_fixture();
    let setup = TransferSetup {
        amount: 100,
        output_nonce_sequence: 1,
        amount_nonce_sequence: DEFAULT_INPUT_NONCE_SEQUENCE,
    };
    let scenario = run_transfer_scenario(&mut fixture, setup);

    let host_events = collect_listener_host_events(
        &scenario.meta,
        &scenario.account_keys,
        scenario.host_program_id,
    );
    assert_eq!(count_tfhe_events(&host_events), 2);
    assert_eq!(count_acl_events(&host_events), 4);

    assert_transfer_worker(
        &scenario,
        125,
        20,
        setup.amount,
        TransferExpect {
            alice: 25,
            bob: 120,
        },
    )
    .await?;

    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request_with_domains(
            &fixture.alice,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: scenario.new_alice_handle,
                owner: fixture.alice.pubkey(),
                acl_record: scenario.output.alice,
            }],
        ),
    ));
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request_with_domains(
            &fixture.bob,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: scenario.new_bob_handle,
                owner: fixture.bob.pubkey(),
                acl_record: scenario.output.bob,
            }],
        ),
    ));

    Ok(())
}

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_trivial_encrypt_then_confidential_transfer_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut fixture = token_fixture();

    let amount_handle = authorize_transfer_amount(&mut fixture, 100, DEFAULT_INPUT_NONCE_SEQUENCE);

    seed_real_fast_ciphertexts(
        &harness.pool,
        &[
            (fixture.alice_initial, 125),
            (fixture.bob_initial, 20),
            (amount_handle, 100),
        ],
    )
    .await?;

    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta_and_signature(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;
    let new_bob_handle = read_acl_record(&fixture.svm, output.bob)
        .expect("expected Bob output ACL")
        .handle;
    let transfer_events =
        collect_listener_host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&transfer_events), 2);
    assert_eq!(count_acl_events(&transfer_events), 4);

    insert_host_events(&harness.listener_db, transfer_events, signature, 2).await?;
    wait_until_computed(&harness.app).await?;
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request_with_domains(
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
        &signed_user_decrypt_request_with_domains(
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

    assert_eq!(decrypted[0].output_type, BALANCE_FHE_TYPE as i16);
    assert_eq!(decrypted[0].value, "25");
    assert_eq!(decrypted[1].output_type, BALANCE_FHE_TYPE as i16);
    assert_eq!(decrypted[1].value, "120");

    Ok(())
}

#[tokio::test]
#[serial(db)]
#[ignore = "rand birth is not implemented in execute_frame yet"]
async fn solana_fhe_rand_creates_ciphertext_and_decrypts() -> Result<(), Box<dyn std::error::Error>>
{
    let _harness = setup_event_harness().await?;
    Err("execute_frame does not implement Rand yet".into())
}

#[test]
#[ignore = "requires built Solana PoC programs; validates user-decrypt ACL semantics without running the worker"]
fn solana_user_decrypt_acl_invariants_match_evm_semantics() {
    let mut fixture = token_fixture();
    let amount_handle = authorize_transfer_amount(&mut fixture, 100, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;

    let valid = signed_user_decrypt_request_with_domains(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: new_alice_handle,
            owner: fixture.alice.pubkey(),
            acl_record: output.alice,
        }],
    );
    assert!(kms_like_user_decrypt_check(&fixture.svm, &valid));

    let missing_user_acl = signed_user_decrypt_request_with_domains(
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

    let missing_domain = signed_user_decrypt_request_with_domains(
        &fixture.alice,
        vec![solana_sdk::pubkey::Pubkey::new_unique()],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &missing_domain));

    let wrong_owner = signed_user_decrypt_request_with_domains(
        &fixture.bob,
        vec![fixture.mint.pubkey()],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_owner));

    let wrong_handle = signed_user_decrypt_request_with_domains(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: [0x7f; 32],
            ..valid.handles[0]
        }],
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_handle));
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
                    let ciphertext = tfhe::FheUint64::try_encrypt(value as u64, &client_key)
                        .map_err(|err| err.to_string())?;
                    let supported = SupportedFheCiphertexts::FheUint64(ciphertext);
                    let ty = supported.type_num();
                    assert_eq!(ty, BALANCE_FHE_TYPE as i16);
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

fn collect_listener_host_events(
    meta: &TransactionMetadata,
    account_keys: &[solana_sdk::pubkey::Pubkey],
    program_id: solana_sdk::pubkey::Pubkey,
) -> Vec<SolanaHostEvent> {
    collect_cpi_events(meta, account_keys, program_id, decode_anchor_cpi_event)
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
