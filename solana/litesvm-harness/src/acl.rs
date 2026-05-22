use anchor_lang::prelude::*;
use confidential_token as token;
use solana_sdk::signature::Signer;
use zama_host as host;

pub fn event_authority(program_id: Pubkey) -> Pubkey {
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
