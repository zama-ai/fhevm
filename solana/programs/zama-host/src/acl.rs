use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use solana_sha256_hasher::hashv;

use crate::{AclRecord, AclSubjectEntry, ZamaHostError, MAX_ACL_SUBJECTS};

pub fn acl_nonce_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    hashv(&[
        b"zama-acl-nonce-key-v1",
        acl_domain_key.as_ref(),
        app_account.as_ref(),
        &encrypted_value_label,
    ])
    .to_bytes()
}

pub fn record_allows(record: &AclRecord, subject: Pubkey) -> bool {
    record.subjects[..record.subject_count as usize].contains(&subject)
}

pub(crate) fn assert_record_allows_handle(
    record: &AclRecord,
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<()> {
    require!(record.handle == handle, ZamaHostError::AclHandleMismatch);
    require!(
        record_allows(record, subject),
        ZamaHostError::AclSubjectMismatch
    );
    Ok(())
}

pub(crate) fn assert_canonical_acl_record(
    record_info: &AccountInfo,
    record: &Account<AclRecord>,
) -> Result<()> {
    assert_canonical_acl_record_data(record_info.key(), record)
}

pub(crate) fn assert_canonical_acl_record_data(
    record_key: Pubkey,
    record: &AclRecord,
) -> Result<()> {
    assert_nonce_key_matches_fields(
        record.nonce_key,
        record.acl_domain_key,
        record.app_account,
        record.encrypted_value_label,
    )?;

    let (expected, expected_bump) = Pubkey::find_program_address(
        &[
            b"acl-record",
            record.nonce_key.as_ref(),
            &record.nonce_sequence.to_le_bytes(),
        ],
        &crate::ID,
    );
    require_keys_eq!(record_key, expected, ZamaHostError::AclRecordPdaMismatch);
    require!(
        record.bump == expected_bump,
        ZamaHostError::AclRecordPdaMismatch
    );
    Ok(())
}

pub(crate) fn assert_record(
    record_info: &AccountInfo,
    record: &Account<AclRecord>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<()> {
    assert_nonce_key_matches_fields(
        nonce_key,
        acl_domain_key,
        app_account,
        encrypted_value_label,
    )?;
    assert_canonical_acl_record(record_info, record)?;
    require!(
        record.nonce_key == nonce_key,
        ZamaHostError::AclNonceKeyMismatch
    );
    require!(
        record.nonce_sequence == nonce_sequence,
        ZamaHostError::AclNonceSequenceMismatch
    );
    require_keys_eq!(
        record.acl_domain_key,
        acl_domain_key,
        ZamaHostError::AclDomainKeyMismatch
    );
    require_keys_eq!(
        record.app_account,
        app_account,
        ZamaHostError::AclAppAccountMismatch
    );
    require!(
        record.encrypted_value_label == encrypted_value_label,
        ZamaHostError::AclEncryptedValueLabelMismatch
    );
    assert_record_allows_handle(record, handle, subject)
}

pub(crate) fn extend_acl_subjects(
    record: &mut Account<AclRecord>,
    subjects: &[AclSubjectEntry],
) -> Result<()> {
    require!(
        !subjects.is_empty(),
        ZamaHostError::AclSubjectCapacityExceeded
    );

    let mut subject_count = record.subject_count as usize;
    for subject in subjects {
        if record.subjects[..subject_count].contains(&subject.pubkey) {
            continue;
        }
        require!(
            subject_count < MAX_ACL_SUBJECTS,
            ZamaHostError::AclSubjectCapacityExceeded
        );
        record.subjects[subject_count] = subject.pubkey;
        subject_count += 1;
    }
    record.subject_count = subject_count as u8;
    Ok(())
}

pub(crate) fn assert_output_acl_metadata(
    nonce_key: [u8; 32],
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    subjects: &[AclSubjectEntry],
) -> Result<()> {
    assert_nonce_key_matches_fields(
        nonce_key,
        acl_domain_key,
        app_account,
        encrypted_value_label,
    )?;
    require!(
        !subjects.is_empty() && subjects.len() <= MAX_ACL_SUBJECTS,
        ZamaHostError::AclSubjectCapacityExceeded
    );
    Ok(())
}

pub(crate) fn create_acl_record_account<'info>(
    payer: &AccountInfo<'info>,
    output_acl_record: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
) -> Result<()> {
    require!(
        output_acl_record.data_is_empty() && output_acl_record.lamports() == 0,
        ZamaHostError::FrameOutputAccountAlreadyInitialized
    );
    let (expected, bump) = Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &crate::ID,
    );
    require_keys_eq!(
        output_acl_record.key(),
        expected,
        ZamaHostError::AclRecordPdaMismatch
    );
    let space = 8 + AclRecord::SPACE;
    let lamports = Rent::get()?.minimum_balance(space);
    let nonce_sequence_bytes = nonce_sequence.to_le_bytes();
    let seeds: &[&[u8]] = &[
        b"acl-record",
        nonce_key.as_ref(),
        &nonce_sequence_bytes,
        &[bump],
    ];
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            output_acl_record.key,
            lamports,
            space as u64,
            &crate::ID,
        ),
        &[
            payer.clone(),
            output_acl_record.clone(),
            system_program.clone(),
        ],
        &[seeds],
    )?;
    Ok(())
}

pub(crate) fn write_acl_record_data<'info>(
    record_info: &AccountInfo<'info>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[AclSubjectEntry],
    public_decrypt: bool,
) -> Result<()> {
    let (_, bump) = Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &crate::ID,
    );
    let mut record = AclRecord {
        handle: [0; 32],
        nonce_key: [0; 32],
        nonce_sequence: 0,
        acl_domain_key: Pubkey::default(),
        app_account: Pubkey::default(),
        encrypted_value_label: [0; 32],
        subjects: [Pubkey::default(); MAX_ACL_SUBJECTS],
        subject_count: 0,
        public_decrypt: false,
        bump,
    };
    write_acl_record_fields(
        &mut record,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        subjects,
        public_decrypt,
    );
    serialize_acl_record(record_info, &record)
}

pub(crate) fn deserialize_acl_record<'info>(record_info: &AccountInfo<'info>) -> Result<AclRecord> {
    require_keys_eq!(
        *record_info.owner,
        crate::ID,
        ZamaHostError::AclRecordPdaMismatch
    );
    let data = record_info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    AclRecord::try_deserialize(&mut data_slice)
}

pub(crate) fn serialize_acl_record<'info>(
    record_info: &AccountInfo<'info>,
    record: &AclRecord,
) -> Result<()> {
    let mut data = record_info.try_borrow_mut_data()?;
    let mut data_slice: &mut [u8] = &mut data;
    record.try_serialize(&mut data_slice)?;
    Ok(())
}

fn write_acl_record_fields(
    record: &mut AclRecord,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[AclSubjectEntry],
    public_decrypt: bool,
) {
    record.handle = handle;
    record.nonce_key = nonce_key;
    record.nonce_sequence = nonce_sequence;
    record.acl_domain_key = acl_domain_key;
    record.app_account = app_account;
    record.encrypted_value_label = encrypted_value_label;
    record.subjects = [Pubkey::default(); MAX_ACL_SUBJECTS];
    record.subject_count = subjects.len() as u8;
    record.public_decrypt = public_decrypt;

    for (index, subject) in subjects.iter().enumerate() {
        record.subjects[index] = subject.pubkey;
    }
}

fn assert_nonce_key_matches_fields(
    nonce_key: [u8; 32],
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> Result<()> {
    require!(
        nonce_key == acl_nonce_key(acl_domain_key, app_account, encrypted_value_label),
        ZamaHostError::AclNonceKeyMismatch
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acl_nonce_key_is_stable() {
        let domain = Pubkey::new_unique();
        let app = Pubkey::new_unique();
        let label = [7_u8; 32];
        let first = acl_nonce_key(domain, app, label);
        let second = acl_nonce_key(domain, app, label);
        assert_eq!(first, second);
    }

    #[test]
    fn record_allows_respects_subject_count() {
        let owner = Pubkey::new_unique();
        let compute = Pubkey::new_unique();
        let record = AclRecord {
            handle: [0; 32],
            nonce_key: [0; 32],
            nonce_sequence: 0,
            acl_domain_key: Pubkey::default(),
            app_account: Pubkey::default(),
            encrypted_value_label: [0; 32],
            subjects: [
                owner,
                compute,
                Pubkey::default(),
                Pubkey::default(),
                Pubkey::default(),
                Pubkey::default(),
                Pubkey::default(),
                Pubkey::default(),
            ],
            subject_count: 2,
            public_decrypt: false,
            bump: 0,
        };
        assert!(record_allows(&record, owner));
        assert!(record_allows(&record, compute));
        assert!(!record_allows(&record, Pubkey::new_unique()));
    }
}
