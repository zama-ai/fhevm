//! Token-local FHE helper functions.
//!
//! The confidential token program keeps raw ZamaHost CPI assembly in this
//! module so business logic can call named operations (`add`, `sub`,
//! `trivial_encrypt_u64`) and receive the host-verified output handle.

use anchor_lang::{prelude::*, AccountDeserialize};
use zama_host::{
    cpi,
    cpi::accounts::{
        AllowForDecryption as HostAllowForDecryption, FheBinaryOpAndBindOutput, FheRandAndBind,
        FheRandBoundedAndBind, FheTernaryOpAndBindOutput, TrivialEncryptAndBind,
    },
    program::ZamaHost,
    AclSubjectEntry, FheBinaryOpCode, FheTernaryOpCode, HostConfig,
};

use crate::{ConfidentialTokenAccount, ConfidentialTokenError};

/// Inputs required for a binary FHE operation that also binds durable ACL state.
pub struct BinaryOp<'a, 'info> {
    /// Transaction payer and rent payer for the output ACL record.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Program-controlled compute signer PDA.
    pub compute_signer: &'a UncheckedAccount<'info>,
    /// Token account that signs as the app account authority.
    pub app_account_authority: &'a Account<'info, ConfidentialTokenAccount>,
    /// ACL record proving access to the left-hand operand.
    pub lhs_acl_record: AccountInfo<'info>,
    /// Left-hand operand handle.
    pub lhs: [u8; 32],
    /// ACL record proving access to the right-hand operand when it is encrypted.
    pub rhs_acl_record: AccountInfo<'info>,
    /// Right-hand operand handle or scalar bytes.
    pub rhs: [u8; 32],
    /// Whether `rhs` is plaintext scalar bytes.
    pub scalar: bool,
    /// Canonical output ACL record to initialize.
    pub output_acl_record: AccountInfo<'info>,
    /// FHE type byte for the output handle.
    pub output_fhe_type: u8,
    /// ACL domain for this token app, currently the confidential mint.
    pub acl_domain_key: Pubkey,
    /// Bump for the compute signer PDA.
    pub compute_signer_bump: u8,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// App-level encrypted field label for the output.
    pub output_encrypted_value_label: [u8; 32],
    /// Initial subjects for the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public-decrypt flag for the output ACL record.
    pub output_public_decrypt: bool,
}

/// Inputs required for a binary FHE operation authorized by a non-token PDA.
pub struct BinaryOpWithAppPda<'a, 'info> {
    /// Transaction payer and rent payer for the output ACL record.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Program-controlled compute signer PDA.
    pub compute_signer: &'a UncheckedAccount<'info>,
    /// PDA that signs as the app account authority.
    pub app_account_authority: &'a UncheckedAccount<'info>,
    /// Seeds for `app_account_authority`, excluding the compute signer seeds.
    pub app_signer_seeds: &'a [&'a [u8]],
    /// App account recorded in the output ACL.
    pub output_app_account: Pubkey,
    /// ACL record proving access to the left-hand operand.
    pub lhs_acl_record: AccountInfo<'info>,
    /// Left-hand operand handle.
    pub lhs: [u8; 32],
    /// ACL record proving access to the right-hand operand when it is encrypted.
    pub rhs_acl_record: AccountInfo<'info>,
    /// Right-hand operand handle or scalar bytes.
    pub rhs: [u8; 32],
    /// Whether `rhs` is plaintext scalar bytes.
    pub scalar: bool,
    /// Canonical output ACL record to initialize.
    pub output_acl_record: AccountInfo<'info>,
    /// FHE type byte for the output handle.
    pub output_fhe_type: u8,
    /// ACL domain for this token app, currently the confidential mint.
    pub acl_domain_key: Pubkey,
    /// Bump for the compute signer PDA.
    pub compute_signer_bump: u8,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// App-level encrypted field label for the output.
    pub output_encrypted_value_label: [u8; 32],
    /// Initial subjects for the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public-decrypt flag for the output ACL record.
    pub output_public_decrypt: bool,
}

/// Performs a host-verified FHE addition and returns the output handle.
pub fn add<'info>(request: BinaryOp<'_, 'info>) -> Result<[u8; 32]> {
    binary_op(FheBinaryOpCode::Add, request)
}

/// Performs a host-verified FHE addition authorized by a non-token PDA.
pub fn add_with_app_pda<'info>(request: BinaryOpWithAppPda<'_, 'info>) -> Result<[u8; 32]> {
    binary_op_with_app_pda(FheBinaryOpCode::Add, request)
}

/// Performs a host-verified FHE subtraction and returns the output handle.
pub fn sub<'info>(request: BinaryOp<'_, 'info>) -> Result<[u8; 32]> {
    binary_op(FheBinaryOpCode::Sub, request)
}

/// Performs a host-verified FHE subtraction authorized by a non-token PDA.
pub fn sub_with_app_pda<'info>(request: BinaryOpWithAppPda<'_, 'info>) -> Result<[u8; 32]> {
    binary_op_with_app_pda(FheBinaryOpCode::Sub, request)
}

/// Performs a host-verified FHE greater-than-or-equal comparison.
pub fn ge<'info>(request: BinaryOp<'_, 'info>) -> Result<[u8; 32]> {
    binary_op(FheBinaryOpCode::Ge, request)
}

fn binary_op<'info>(op: FheBinaryOpCode, request: BinaryOp<'_, 'info>) -> Result<[u8; 32]> {
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
    let result = zama_host::computed_bound_binary_handle_for_current_slot_with_chain_id(
        op,
        request.lhs,
        request.rhs,
        request.scalar,
        request.output_fhe_type,
        request.host_config.chain_id,
        request.output_nonce_key,
        request.output_nonce_sequence,
    )?;

    cpi::fhe_binary_op_and_bind_output(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            FheBinaryOpAndBindOutput {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                host_config: request.host_config.to_account_info(),
                lhs_acl_record: request.lhs_acl_record,
                lhs_permission_record: None,
                rhs_acl_record: request.rhs_acl_record,
                rhs_permission_record: None,
                output_acl_record: request.output_acl_record,
                system_program: request.system_program.to_account_info(),
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        op,
        request.lhs,
        request.rhs,
        request.scalar,
        request.output_fhe_type,
        result,
        request.output_nonce_key,
        request.output_nonce_sequence,
        request.acl_domain_key,
        request.app_account_authority.key(),
        request.output_encrypted_value_label,
        request.output_subjects,
        request.output_public_decrypt,
    )?;

    Ok(result)
}

fn binary_op_with_app_pda<'info>(
    op: FheBinaryOpCode,
    request: BinaryOpWithAppPda<'_, 'info>,
) -> Result<[u8; 32]> {
    let compute_bump = [request.compute_signer_bump];
    let compute_signer_seeds: &[&[u8]] = &[
        b"fhe-compute",
        request.acl_domain_key.as_ref(),
        &compute_bump,
    ];
    let signer_seeds: &[&[&[u8]]] = &[compute_signer_seeds, request.app_signer_seeds];
    let result = zama_host::computed_bound_binary_handle_for_current_slot_with_chain_id(
        op,
        request.lhs,
        request.rhs,
        request.scalar,
        request.output_fhe_type,
        request.host_config.chain_id,
        request.output_nonce_key,
        request.output_nonce_sequence,
    )?;

    cpi::fhe_binary_op_and_bind_output(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            FheBinaryOpAndBindOutput {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                host_config: request.host_config.to_account_info(),
                lhs_acl_record: request.lhs_acl_record,
                lhs_permission_record: None,
                rhs_acl_record: request.rhs_acl_record,
                rhs_permission_record: None,
                output_acl_record: request.output_acl_record,
                system_program: request.system_program.to_account_info(),
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        op,
        request.lhs,
        request.rhs,
        request.scalar,
        request.output_fhe_type,
        result,
        request.output_nonce_key,
        request.output_nonce_sequence,
        request.acl_domain_key,
        request.output_app_account,
        request.output_encrypted_value_label,
        request.output_subjects,
        request.output_public_decrypt,
    )?;

    Ok(result)
}

/// Inputs required for a ternary FHE operation that also binds durable ACL state.
pub struct TernaryOp<'a, 'info> {
    /// Transaction payer and rent payer for the output ACL record.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Program-controlled compute signer PDA.
    pub compute_signer: &'a UncheckedAccount<'info>,
    /// Token account that signs as the app account authority.
    pub app_account_authority: &'a Account<'info, ConfidentialTokenAccount>,
    /// ACL record proving access to the encrypted control.
    pub control_acl_record: AccountInfo<'info>,
    /// Encrypted control handle.
    pub control: [u8; 32],
    /// ACL record proving access to the true-branch handle.
    pub if_true_acl_record: AccountInfo<'info>,
    /// True-branch handle.
    pub if_true: [u8; 32],
    /// ACL record proving access to the false-branch handle.
    pub if_false_acl_record: AccountInfo<'info>,
    /// False-branch handle.
    pub if_false: [u8; 32],
    /// Canonical output ACL record to initialize.
    pub output_acl_record: AccountInfo<'info>,
    /// FHE type byte for the output handle.
    pub output_fhe_type: u8,
    /// ACL domain for this token app, currently the confidential mint.
    pub acl_domain_key: Pubkey,
    /// Bump for the compute signer PDA.
    pub compute_signer_bump: u8,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// App-level encrypted field label for the output.
    pub output_encrypted_value_label: [u8; 32],
    /// Initial subjects for the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public-decrypt flag for the output ACL record.
    pub output_public_decrypt: bool,
}

/// Performs a host-verified encrypted conditional select.
pub fn if_then_else<'info>(request: TernaryOp<'_, 'info>) -> Result<[u8; 32]> {
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
    let result = zama_host::computed_bound_ternary_handle_for_current_slot_with_chain_id(
        FheTernaryOpCode::IfThenElse,
        request.control,
        request.if_true,
        request.if_false,
        request.output_fhe_type,
        request.host_config.chain_id,
        request.output_nonce_key,
        request.output_nonce_sequence,
    )?;

    cpi::fhe_ternary_op_and_bind_output(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            FheTernaryOpAndBindOutput {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                host_config: request.host_config.to_account_info(),
                control_acl_record: request.control_acl_record,
                control_permission_record: None,
                if_true_acl_record: request.if_true_acl_record,
                if_true_permission_record: None,
                if_false_acl_record: request.if_false_acl_record,
                if_false_permission_record: None,
                output_acl_record: request.output_acl_record,
                system_program: request.system_program.to_account_info(),
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        FheTernaryOpCode::IfThenElse,
        request.control,
        request.if_true,
        request.if_false,
        request.output_fhe_type,
        result,
        request.output_nonce_key,
        request.output_nonce_sequence,
        request.acl_domain_key,
        request.app_account_authority.key(),
        request.output_encrypted_value_label,
        request.output_subjects,
        request.output_public_decrypt,
    )?;

    Ok(result)
}

/// Inputs required for trivial encryption of a `u64` plus ACL record birth.
pub struct TrivialEncryptU64<'a, 'info> {
    /// Transaction payer and rent payer for the output ACL record.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Program-controlled compute signer PDA.
    pub compute_signer: &'a UncheckedAccount<'info>,
    /// Token account that signs as the app account authority.
    pub app_account_authority: &'a Account<'info, ConfidentialTokenAccount>,
    /// Canonical output ACL record to initialize.
    pub output_acl_record: AccountInfo<'info>,
    /// ACL domain for this token app, currently the confidential mint.
    pub acl_domain_key: Pubkey,
    /// Bump for the compute signer PDA.
    pub compute_signer_bump: u8,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// App-level encrypted field label for the output.
    pub output_encrypted_value_label: [u8; 32],
    /// Clear `u64` encoded into the trivial-encrypt event.
    pub plaintext: u64,
    /// FHE type byte for the output handle.
    pub fhe_type: u8,
    /// Initial subjects for the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public-decrypt flag for the output ACL record.
    pub output_public_decrypt: bool,
}

/// Inputs required for trivial encryption authorized by a non-token PDA.
pub struct TrivialEncryptU64WithAppPda<'a, 'info> {
    /// Transaction payer and rent payer for the output ACL record.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Program-controlled compute signer PDA.
    pub compute_signer: &'a UncheckedAccount<'info>,
    /// PDA that signs as the app account authority.
    pub app_account_authority: &'a UncheckedAccount<'info>,
    /// Seeds for `app_account_authority`, excluding the compute signer seeds.
    pub app_signer_seeds: &'a [&'a [u8]],
    /// App account recorded in the output ACL.
    pub output_app_account: Pubkey,
    /// Canonical output ACL record to initialize.
    pub output_acl_record: AccountInfo<'info>,
    /// ACL domain for this token app, currently the confidential mint.
    pub acl_domain_key: Pubkey,
    /// Bump for the compute signer PDA.
    pub compute_signer_bump: u8,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// App-level encrypted field label for the output.
    pub output_encrypted_value_label: [u8; 32],
    /// Clear `u64` encoded into the trivial-encrypt event.
    pub plaintext: u64,
    /// FHE type byte for the output handle.
    pub fhe_type: u8,
    /// Initial subjects for the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public-decrypt flag for the output ACL record.
    pub output_public_decrypt: bool,
}

/// Performs host-owned trivial encryption and returns the created handle.
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
                host_config: request.host_config.to_account_info(),
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

/// Performs host-owned trivial encryption authorized by a non-token PDA.
pub fn trivial_encrypt_u64_with_app_pda<'info>(
    request: TrivialEncryptU64WithAppPda<'_, 'info>,
) -> Result<[u8; 32]> {
    let compute_bump = [request.compute_signer_bump];
    let compute_signer_seeds: &[&[u8]] = &[
        b"fhe-compute",
        request.acl_domain_key.as_ref(),
        &compute_bump,
    ];
    let signer_seeds: &[&[&[u8]]] = &[compute_signer_seeds, request.app_signer_seeds];
    let output_acl_record_for_read = request.output_acl_record.clone();

    cpi::trivial_encrypt_and_bind(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            TrivialEncryptAndBind {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                host_config: request.host_config.to_account_info(),
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
        request.output_app_account,
        request.output_encrypted_value_label,
        request.output_subjects,
        request.output_public_decrypt,
    )?;

    output_handle(output_acl_record_for_read)
}

/// Inputs required for random `euint64` amount creation plus ACL record birth.
pub struct RandU64<'a, 'info> {
    /// Transaction payer and rent payer for the output ACL record.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Program-controlled compute signer PDA.
    pub compute_signer: &'a UncheckedAccount<'info>,
    /// Owner signer recorded as the app account authority for this amount.
    pub app_account_authority: &'a Signer<'info>,
    /// Canonical output ACL record to initialize.
    pub output_acl_record: AccountInfo<'info>,
    /// ACL domain for this token app, currently the confidential mint.
    pub acl_domain_key: Pubkey,
    /// Bump for the compute signer PDA.
    pub compute_signer_bump: u8,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// App-level encrypted field label for the output.
    pub output_encrypted_value_label: [u8; 32],
    /// Initial subjects for the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public-decrypt flag for the output ACL record.
    pub output_public_decrypt: bool,
}

/// Performs host-owned random amount creation and returns the created handle.
pub fn rand_u64<'info>(request: RandU64<'_, 'info>) -> Result<[u8; 32]> {
    let compute_bump = [request.compute_signer_bump];
    let compute_signer_seeds: &[&[u8]] = &[
        b"fhe-compute",
        request.acl_domain_key.as_ref(),
        &compute_bump,
    ];
    let signer_seeds: &[&[&[u8]]] = &[compute_signer_seeds];
    let output_acl_record_for_read = request.output_acl_record.clone();

    cpi::fhe_rand_and_bind(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            FheRandAndBind {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                host_config: request.host_config.to_account_info(),
                output_acl_record: request.output_acl_record,
                system_program: request.system_program.to_account_info(),
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        crate::BALANCE_FHE_TYPE,
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

/// Performs host-owned bounded random amount creation and returns the created handle.
pub fn rand_bounded_u64<'info>(
    request: RandU64<'_, 'info>,
    upper_bound: [u8; 32],
) -> Result<[u8; 32]> {
    let compute_bump = [request.compute_signer_bump];
    let compute_signer_seeds: &[&[u8]] = &[
        b"fhe-compute",
        request.acl_domain_key.as_ref(),
        &compute_bump,
    ];
    let signer_seeds: &[&[&[u8]]] = &[compute_signer_seeds];
    let output_acl_record_for_read = request.output_acl_record.clone();

    cpi::fhe_rand_bounded_and_bind(
        CpiContext::new_with_signer(
            request.zama_program.key(),
            FheRandBoundedAndBind {
                payer: request.payer.to_account_info(),
                compute_subject: request.compute_signer.to_account_info(),
                app_account_authority: request.app_account_authority.to_account_info(),
                host_config: request.host_config.to_account_info(),
                output_acl_record: request.output_acl_record,
                system_program: request.system_program.to_account_info(),
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        upper_bound,
        crate::BALANCE_FHE_TYPE,
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

/// Inputs required to mark a host handle publicly decryptable.
pub struct AllowPublicDecrypt<'a, 'info> {
    /// Subject that already has `ACL_ROLE_PUBLIC_DECRYPT` on the ACL record.
    pub authority: &'a Signer<'info>,
    /// Optional overflow permission witness when the authority is not inline.
    pub authority_permission_record: Option<AccountInfo<'info>>,
    /// ACL record whose public-decrypt flag is updated.
    pub acl_record: AccountInfo<'info>,
    /// ZamaHost config account.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Optional deny-list witness when grant deny-lists are enabled.
    pub deny_subject_record: Option<AccountInfo<'info>>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Handle stored in `acl_record`.
    pub handle: [u8; 32],
}

/// Delegates public-decrypt authorization to ZamaHost.
pub fn allow_public_decrypt<'info>(request: AllowPublicDecrypt<'_, 'info>) -> Result<()> {
    cpi::allow_for_decryption(
        CpiContext::new(
            request.zama_program.key(),
            HostAllowForDecryption {
                authority: request.authority.to_account_info(),
                authority_permission_record: request.authority_permission_record,
                acl_record: request.acl_record,
                host_config: request.host_config.to_account_info(),
                deny_subject_record: request.deny_subject_record,
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
        ),
        request.handle,
    )
}

fn output_handle(output_acl_record: AccountInfo<'_>) -> Result<[u8; 32]> {
    require!(
        output_acl_record.data_len() == 8 + zama_host::AclRecord::SPACE,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    let data = output_acl_record.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = zama_host::AclRecord::try_deserialize(&mut data_slice)?;
    require!(
        zama_host::acl_record_subject_slots_are_canonical(&record),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    Ok(record.handle)
}

fn u64_plaintext(value: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&value.to_be_bytes());
    plaintext
}
