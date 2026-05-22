use anchor_lang::{AccountDeserialize, AccountSerialize, prelude::*};
use confidential_token as token;
use litesvm::LiteSVM;
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signer};
use zama_host as host;

use crate::util::{set_previous_slot_hash, DEFAULT_TEST_PREVIOUS_BANK_HASH};pub fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

pub fn token_account_address(program_id: Pubkey, mint: Pubkey, owner: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"token-account", mint.as_ref(), owner.as_ref()],
        &program_id,
    )
    .0
}

pub fn vault_authority_address(program_id: Pubkey, mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"vault-authority", mint.as_ref()], &program_id).0
}

pub fn acl_record_address(program_id: Pubkey, nonce_key: [u8; 32], nonce_sequence: u64) -> Pubkey {
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

pub fn balance_acl_record_address(
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

pub fn transfer_amount_acl_address(
    fixture: &crate::fixture::TokenFixture,
    nonce_sequence: u64,
) -> Pubkey {
    acl_record_address(
        fixture.host_program_id,
        token::transfer_amount_nonce_key(fixture.mint.pubkey(), fixture.alice_token),
        nonce_sequence,
    )
}

pub fn read_acl_record(svm: &litesvm::LiteSVM, address: Pubkey) -> Option<host::AclRecord> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    host::AclRecord::try_deserialize(&mut data).ok()
}

pub fn record_subjects(record: &host::AclRecord) -> Vec<Pubkey> {
    record.subjects[..record.subject_count as usize].to_vec()
}

pub fn assert_acl_record(
    svm: &LiteSVM,
    address: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    nonce_sequence: u64,
    handle: [u8; 32],
    subjects: &[Pubkey],
) {
    let record = read_acl_record(svm, address).expect("expected ACL account");
    assert_eq!(record.handle, handle);
    assert_eq!(
        record.nonce_key,
        token::nonce_key(acl_domain_key, app_account, encrypted_value_label)
    );
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record_subjects(&record), subjects);
}

pub fn assert_balance_acl(
    svm: &LiteSVM,
    address: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    nonce_sequence: u64,
    handle: [u8; 32],
    subjects: &[Pubkey],
) {
    assert_acl_record(
        svm,
        address,
        acl_domain_key,
        app_account,
        token::balance_label(),
        nonce_sequence,
        handle,
        subjects,
    );
}

pub fn seed_authorizing_acl_record(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    authority: Pubkey,
) -> Pubkey {
    let (address, bump) = Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &program_id,
    );
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    subjects[0] = authority;
    svm.set_account(
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_acl_record(host::AclRecord {
                handle,
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
                subject_count: 1,
                public_decrypt: false,
                bump,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    set_previous_slot_hash(svm, DEFAULT_TEST_PREVIOUS_BANK_HASH);
    address
}

fn serialized_acl_record(record: host::AclRecord) -> Vec<u8> {
    let mut data = Vec::new();
    record.try_serialize(&mut data).unwrap();
    data
}

pub fn created_acl_count(svm: &LiteSVM, addresses: &[Pubkey]) -> usize {
    addresses
        .iter()
        .filter(|address| svm.get_account(address).is_some())
        .count()
}
