use anchor_lang::{prelude::*, AccountDeserialize};
use zama_host::{
    cpi,
    cpi::accounts::{FheBinaryOp, TrivialEncryptAndBind},
    program::ZamaHost,
    AclSubjectEntry, FheBinaryOpCode,
};

use crate::ConfidentialTokenAccount;

pub struct BinaryOp<'a, 'info> {
    pub payer: &'a Signer<'info>,
    pub event_authority: &'a UncheckedAccount<'info>,
    pub zama_program: &'a Program<'info, ZamaHost>,
    pub compute_signer: &'a UncheckedAccount<'info>,
    pub app_account_authority: &'a Account<'info, ConfidentialTokenAccount>,
    pub lhs_acl_record: AccountInfo<'info>,
    pub op: FheBinaryOpCode,
    pub lhs: [u8; 32],
    pub rhs_acl_record: AccountInfo<'info>,
    pub rhs: [u8; 32],
    pub scalar: bool,
    pub output_acl_record: AccountInfo<'info>,
    pub output_fhe_type: u8,
    pub acl_domain_key: Pubkey,
    pub compute_signer_bump: u8,
    pub system_program: &'a Program<'info, System>,
    pub output_nonce_key: [u8; 32],
    pub output_nonce_sequence: u64,
    pub output_encrypted_value_label: [u8; 32],
    pub output_subjects: Vec<AclSubjectEntry>,
    pub output_public_decrypt: bool,
}

pub fn binary_op<'info>(request: BinaryOp<'_, 'info>) -> Result<[u8; 32]> {
    let compute_bump = [request.compute_signer_bump];
    let app_account_bump = [request.app_account_authority.bump];
    let compute_signer_seeds: &[&[u8]] = &[
        b"fhe-compute",
        request.acl_domain_key.as_ref(),
        &compute_bump,
    ];
    let app_account_seeds: &[&[u8]] = &[
        b"token-account",
        request.app_account_authority.mint.as_ref(),
        request.app_account_authority.owner.as_ref(),
        &app_account_bump,
    ];
    let signer_seeds: &[&[&[u8]]] = &[compute_signer_seeds, app_account_seeds];
    let output_acl_record_for_read = request.output_acl_record.clone();

    cpi::fhe_binary_op(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            FheBinaryOp {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                lhs_acl_record: request.lhs_acl_record,
                rhs_acl_record: request.rhs_acl_record,
                output_acl_record: request.output_acl_record,
                system_program: request.system_program.to_account_info(),
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        request.op,
        request.lhs,
        request.rhs,
        request.scalar,
        request.output_fhe_type,
        request.output_nonce_key,
        request.output_nonce_sequence,
        request.acl_domain_key,
        request.app_account_authority.key(),
        request.output_encrypted_value_label,
        request.output_subjects,
        request.output_public_decrypt,
    )?;

    output_handle(output_acl_record_for_read)
}

pub struct TrivialEncryptU64<'a, 'info> {
    pub payer: &'a Signer<'info>,
    pub event_authority: &'a UncheckedAccount<'info>,
    pub zama_program: &'a Program<'info, ZamaHost>,
    pub compute_signer: &'a UncheckedAccount<'info>,
    pub app_account_authority: &'a Account<'info, ConfidentialTokenAccount>,
    pub output_acl_record: AccountInfo<'info>,
    pub acl_domain_key: Pubkey,
    pub compute_signer_bump: u8,
    pub system_program: &'a Program<'info, System>,
    pub output_nonce_key: [u8; 32],
    pub output_nonce_sequence: u64,
    pub output_encrypted_value_label: [u8; 32],
    pub plaintext: u64,
    pub fhe_type: u8,
    pub output_subjects: Vec<AclSubjectEntry>,
    pub output_public_decrypt: bool,
}

pub fn trivial_encrypt_u64<'info>(request: TrivialEncryptU64<'_, 'info>) -> Result<[u8; 32]> {
    let compute_bump = [request.compute_signer_bump];
    let app_account_bump = [request.app_account_authority.bump];
    let compute_signer_seeds: &[&[u8]] = &[
        b"fhe-compute",
        request.acl_domain_key.as_ref(),
        &compute_bump,
    ];
    let app_account_seeds: &[&[u8]] = &[
        b"token-account",
        request.app_account_authority.mint.as_ref(),
        request.app_account_authority.owner.as_ref(),
        &app_account_bump,
    ];
    let signer_seeds: &[&[&[u8]]] = &[compute_signer_seeds, app_account_seeds];
    let output_acl_record_for_read = request.output_acl_record.clone();

    cpi::trivial_encrypt_and_bind(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            TrivialEncryptAndBind {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                output_acl_record: request.output_acl_record,
                system_program: request.system_program.to_account_info(),
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        u64_plaintext(request.plaintext),
        request.fhe_type,
        request.output_nonce_key,
        request.output_nonce_sequence,
        request.acl_domain_key,
        request.app_account_authority.key(),
        request.output_encrypted_value_label,
        request.output_subjects,
        request.output_public_decrypt,
    )?;

    output_handle(output_acl_record_for_read)
}

fn output_handle(output_acl_record: AccountInfo<'_>) -> Result<[u8; 32]> {
    let data = output_acl_record.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = zama_host::AclRecord::try_deserialize(&mut data_slice)?;
    Ok(record.handle)
}

fn u64_plaintext(value: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&value.to_be_bytes());
    plaintext
}
