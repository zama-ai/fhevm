use anchor_lang::prelude::*;
use anchor_lang::solana_program::{hash::hashv, sysvar::slot_hashes::PodSlotHashes};

declare_id!("EMhXFu68v61bQV4GrF6ZhZhWNVbH6bHPnTdLtXK8meqn");

pub const EVENT_VERSION: u8 = 0;
pub const MAX_ACL_SUBJECTS: usize = 8;
pub const SOLANA_POC_CHAIN_ID: u64 = 12345;

const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
const COMPUTED_HANDLE_MARKER: u8 = 0xff;
const HANDLE_VERSION: u8 = 0;

#[program]
pub mod zama_host {
    use super::*;

    /// Test-only event shim.
    ///
    /// This bypasses ACL record verification and exists only to feed listener /
    /// worker tests. Protocol flows should create ACL state through
    /// `bind_acl_record`, `trivial_encrypt_and_bind`, or `fhe_binary_op`.
    /// Input flows may use `poc_input_verified_and_bind` only as a temporary PoC
    /// short-circuit until the verifier/transciphering boundary exists.
    pub fn test_emit_acl_allowed(
        ctx: Context<TestEmitProtocolEvent>,
        handle: [u8; 32],
        subject: Pubkey,
        permission: AclPermission,
    ) -> Result<()> {
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle,
            subject,
            permission,
        });
        drop(ctx);
        Ok(())
    }

    pub fn bind_acl_record(
        ctx: Context<BindAclRecord>,
        nonce_key: [u8; 32],
        nonce_sequence: u64,
        acl_domain_key: Pubkey,
        app_account: Pubkey,
        encrypted_value_label: [u8; 32],
        handle: [u8; 32],
        subjects: Vec<AclSubjectEntry>,
        public_decrypt: bool,
    ) -> Result<()> {
        let authority = ctx.accounts.authority.key();
        assert_nonce_key_matches_fields(
            nonce_key,
            acl_domain_key,
            app_account,
            encrypted_value_label,
        )?;
        require_keys_eq!(
            ctx.accounts.app_account_authority.key(),
            app_account,
            ZamaHostError::AppAccountAuthorityMismatch
        );
        require!(
            !subjects.is_empty() && subjects.len() <= MAX_ACL_SUBJECTS,
            ZamaHostError::AclSubjectCapacityExceeded
        );
        assert_canonical_acl_record(
            &ctx.accounts.authorizing_acl_record.to_account_info(),
            &ctx.accounts.authorizing_acl_record,
        )?;
        assert_record_allows_handle(&ctx.accounts.authorizing_acl_record, handle, authority)?;

        write_acl_record(
            &mut ctx.accounts.acl_record,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            &subjects,
            public_decrypt,
            ctx.bumps.acl_record,
        );

        for subject in subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle,
                subject: subject.pubkey,
                permission: subject.permission,
            });
        }
        Ok(())
    }

    pub fn assert_acl_record(
        ctx: Context<AssertAclRecord>,
        nonce_key: [u8; 32],
        nonce_sequence: u64,
        acl_domain_key: Pubkey,
        app_account: Pubkey,
        encrypted_value_label: [u8; 32],
        handle: [u8; 32],
        subject: Pubkey,
    ) -> Result<()> {
        assert_record(
            &ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.acl_record,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject,
        )?;
        Ok(())
    }

    pub fn allow_for_decryption(ctx: Context<AllowForDecryption>, handle: [u8; 32]) -> Result<()> {
        let subject = ctx.accounts.authority.key();
        assert_canonical_acl_record(
            &ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.acl_record,
        )?;
        assert_record_allows_handle(&ctx.accounts.acl_record, handle, subject)?;
        ctx.accounts.acl_record.public_decrypt = true;
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle,
            subject,
            permission: AclPermission::PublicDecrypt,
        });
        Ok(())
    }

    pub fn trivial_encrypt_and_bind(
        ctx: Context<TrivialEncryptAndBind>,
        plaintext: [u8; 32],
        fhe_type: u8,
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        let subject = ctx.accounts.compute_subject.key();
        require_keys_eq!(
            ctx.accounts.app_account_authority.key(),
            output_app_account,
            ZamaHostError::AppAccountAuthorityMismatch
        );
        assert_nonce_key_matches_fields(
            output_nonce_key,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
        )?;
        require!(
            !output_subjects.is_empty() && output_subjects.len() <= MAX_ACL_SUBJECTS,
            ZamaHostError::AclSubjectCapacityExceeded
        );

        let clock = Clock::get()?;
        let previous_bank_hash = previous_bank_hash(clock.slot)?;
        let result = computed_trivial_handle(
            plaintext,
            fhe_type,
            SOLANA_POC_CHAIN_ID,
            previous_bank_hash,
            clock.unix_timestamp,
        );

        write_acl_record(
            &mut ctx.accounts.output_acl_record,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            result,
            &output_subjects,
            output_public_decrypt,
            ctx.bumps.output_acl_record,
        );

        emit_cpi!(TrivialEncryptEvent {
            version: EVENT_VERSION,
            subject,
            plaintext,
            fhe_type,
            result,
        });
        for output_subject in output_subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: result,
                subject: output_subject.pubkey,
                permission: output_subject.permission,
            });
        }
        Ok(())
    }

    /// PoC short-circuit for Solana input birth.
    ///
    /// This creates an ACL record for a caller-supplied input handle and emits
    /// the same event shape the worker currently consumes. It does not verify a
    /// ZKPoK, ciphertext preimage, or transciphering proof. The final input path
    /// must replace this with a real verifier authority before it is treated as
    /// protocol logic.
    pub fn poc_input_verified_and_bind(
        ctx: Context<PocInputVerifiedAndBind>,
        input_handle: [u8; 32],
        user: Pubkey,
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.app_account_authority.key(),
            output_app_account,
            ZamaHostError::AppAccountAuthorityMismatch
        );
        assert_nonce_key_matches_fields(
            output_nonce_key,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
        )?;
        require!(
            !output_subjects.is_empty() && output_subjects.len() <= MAX_ACL_SUBJECTS,
            ZamaHostError::AclSubjectCapacityExceeded
        );

        write_acl_record(
            &mut ctx.accounts.output_acl_record,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            input_handle,
            &output_subjects,
            output_public_decrypt,
            ctx.bumps.output_acl_record,
        );

        emit_cpi!(InputVerifiedEvent {
            version: EVENT_VERSION,
            input_handle,
            result_handle: input_handle,
            user,
            acl_domain_key: output_acl_domain_key,
        });
        for output_subject in output_subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: input_handle,
                subject: output_subject.pubkey,
                permission: output_subject.permission,
            });
        }
        Ok(())
    }

    pub fn fhe_binary_op(
        ctx: Context<FheBinaryOp>,
        op: FheBinaryOpCode,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        output_fhe_type: u8,
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        let subject = ctx.accounts.compute_subject.key();
        require_keys_eq!(
            ctx.accounts.app_account_authority.key(),
            output_app_account,
            ZamaHostError::AppAccountAuthorityMismatch
        );
        assert_nonce_key_matches_fields(
            output_nonce_key,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
        )?;
        require!(
            !output_subjects.is_empty() && output_subjects.len() <= MAX_ACL_SUBJECTS,
            ZamaHostError::AclSubjectCapacityExceeded
        );

        // Match the EVM executor boundary: no compute event is emitted until
        // the host program verifies that the compute subject can use the
        // operand handles.
        assert_canonical_acl_record(
            &ctx.accounts.lhs_acl_record.to_account_info(),
            &ctx.accounts.lhs_acl_record,
        )?;
        assert_record_allows_handle(&ctx.accounts.lhs_acl_record, lhs, subject)?;
        if !scalar {
            assert_canonical_acl_record(
                &ctx.accounts.rhs_acl_record.to_account_info(),
                &ctx.accounts.rhs_acl_record,
            )?;
            assert_record_allows_handle(&ctx.accounts.rhs_acl_record, rhs, subject)?;
        }

        let clock = Clock::get()?;
        let previous_bank_hash = previous_bank_hash(clock.slot)?;
        let result = computed_binary_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            SOLANA_POC_CHAIN_ID,
            previous_bank_hash,
            clock.unix_timestamp,
        );

        write_acl_record(
            &mut ctx.accounts.output_acl_record,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            result,
            &output_subjects,
            output_public_decrypt,
            ctx.bumps.output_acl_record,
        );

        // Future scalar and ternary ops must keep the EVM scalarByte rule:
        // bit i flags whether the i-th argument from the right is scalar.
        // Example for mulDiv(lhs, rhs, divisor):
        // enc x enc x scalar => 0x01, enc x scalar x scalar => 0x03.
        for output_subject in output_subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: result,
                subject: output_subject.pubkey,
                permission: output_subject.permission,
            });
        }
        emit_cpi!(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op,
            subject,
            lhs,
            rhs,
            scalar,
            result,
        });
        Ok(())
    }

    /// Test-only event shim.
    ///
    /// This emits a caller-chosen trivial-encrypt result handle without creating
    /// an ACL record. App flows should use `trivial_encrypt_and_bind` when they
    /// need a host-born trivial encryption handle with durable ACL state.
    pub fn test_emit_trivial_encrypt(
        ctx: Context<TestEmitProtocolEvent>,
        subject: Pubkey,
        plaintext: [u8; 32],
        fhe_type: u8,
        result: [u8; 32],
    ) -> Result<()> {
        emit_cpi!(TrivialEncryptEvent {
            version: EVENT_VERSION,
            subject,
            plaintext,
            fhe_type,
            result,
        });
        drop(ctx);
        Ok(())
    }

    /// Test-only event shim.
    ///
    /// This emits a caller-chosen random result handle. It is useful for worker
    /// tests and should not be treated as the final random-handle birth API.
    pub fn test_emit_fhe_rand(
        ctx: Context<TestEmitProtocolEvent>,
        subject: Pubkey,
        seed: [u8; 16],
        fhe_type: u8,
        result: [u8; 32],
    ) -> Result<()> {
        emit_cpi!(FheRandEvent {
            version: EVENT_VERSION,
            subject,
            seed,
            fhe_type,
            result,
        });
        drop(ctx);
        Ok(())
    }

    /// Test-only event shim.
    ///
    /// This only emits an input-verification event. The PoC ACL-bearing stand-in
    /// is `poc_input_verified_and_bind`; the final version should require the real
    /// InputVerifier/transciphering boundary.
    pub fn test_emit_input_verified(
        ctx: Context<TestEmitProtocolEvent>,
        input_handle: [u8; 32],
        result_handle: [u8; 32],
        user: Pubkey,
        acl_domain_key: Pubkey,
    ) -> Result<()> {
        emit_cpi!(InputVerifiedEvent {
            version: EVENT_VERSION,
            input_handle,
            result_handle,
            user,
            acl_domain_key,
        });
        drop(ctx);
        Ok(())
    }
}

/// Accounts for test-only event shims that bypass protocol state writes.
#[derive(Accounts)]
#[event_cpi]
pub struct TestEmitProtocolEvent {}

#[derive(Accounts)]
#[instruction(nonce_key: [u8; 32], nonce_sequence: u64)]
#[event_cpi]
pub struct BindAclRecord<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    pub app_account_authority: Signer<'info>,
    pub authorizing_acl_record: Account<'info, AclRecord>,
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [b"acl-record", nonce_key.as_ref(), &nonce_sequence.to_le_bytes()],
        bump
    )]
    // PoC records are born Bound through Anchor `init`. A future predeclared-account
    // flow should add an explicit Empty -> Bound state machine.
    pub acl_record: Account<'info, AclRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    plaintext: [u8; 32],
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct TrivialEncryptAndBind<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub compute_subject: Signer<'info>,
    pub app_account_authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [b"acl-record", output_nonce_key.as_ref(), &output_nonce_sequence.to_le_bytes()],
        bump
    )]
    pub output_acl_record: Account<'info, AclRecord>,
    pub system_program: Program<'info, System>,
}

/// Accounts for the PoC input short-circuit.
///
/// `app_account_authority` only proves that the app account accepted this input
/// binding. It is not a cryptographic input verifier.
#[derive(Accounts)]
#[instruction(
    input_handle: [u8; 32],
    user: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct PocInputVerifiedAndBind<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub app_account_authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [b"acl-record", output_nonce_key.as_ref(), &output_nonce_sequence.to_le_bytes()],
        bump
    )]
    pub output_acl_record: Account<'info, AclRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssertAclRecord<'info> {
    pub acl_record: Account<'info, AclRecord>,
}

#[derive(Accounts)]
#[event_cpi]
pub struct AllowForDecryption<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub acl_record: Account<'info, AclRecord>,
}

#[derive(Accounts)]
#[instruction(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct FheBinaryOp<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub compute_subject: Signer<'info>,
    pub app_account_authority: Signer<'info>,
    pub lhs_acl_record: Account<'info, AclRecord>,
    pub rhs_acl_record: Account<'info, AclRecord>,
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [b"acl-record", output_nonce_key.as_ref(), &output_nonce_sequence.to_le_bytes()],
        bump
    )]
    pub output_acl_record: Account<'info, AclRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AclSubjectEntry {
    pub pubkey: Pubkey,
    pub permission: AclPermission,
}

#[account]
pub struct AclRecord {
    pub handle: [u8; 32],
    pub nonce_key: [u8; 32],
    pub nonce_sequence: u64,
    pub acl_domain_key: Pubkey,
    pub app_account: Pubkey,
    pub encrypted_value_label: [u8; 32],
    pub subjects: [Pubkey; MAX_ACL_SUBJECTS],
    pub subject_count: u8,
    pub public_decrypt: bool,
    pub bump: u8,
}

impl AclRecord {
    pub const SPACE: usize = 32 + 32 + 8 + 32 + 32 + 32 + (32 * MAX_ACL_SUBJECTS) + 1 + 1 + 1;
}

#[event]
pub struct FheBinaryOpEvent {
    pub version: u8,
    pub op: FheBinaryOpCode,
    pub subject: Pubkey,
    pub lhs: [u8; 32],
    pub rhs: [u8; 32],
    pub scalar: bool,
    pub result: [u8; 32],
}

#[event]
pub struct TrivialEncryptEvent {
    pub version: u8,
    pub subject: Pubkey,
    pub plaintext: [u8; 32],
    pub fhe_type: u8,
    pub result: [u8; 32],
}

#[event]
pub struct FheRandEvent {
    pub version: u8,
    pub subject: Pubkey,
    pub seed: [u8; 16],
    pub fhe_type: u8,
    pub result: [u8; 32],
}

#[event]
pub struct AclAllowedEvent {
    pub version: u8,
    pub handle: [u8; 32],
    pub subject: Pubkey,
    pub permission: AclPermission,
}

#[event]
pub struct InputVerifiedEvent {
    pub version: u8,
    pub input_handle: [u8; 32],
    pub result_handle: [u8; 32],
    pub user: Pubkey,
    pub acl_domain_key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheBinaryOpCode {
    Add,
    Sub,
}

impl FheBinaryOpCode {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclPermission {
    Compute,
    UserDecrypt,
    PublicDecrypt,
}

#[error_code]
pub enum ZamaHostError {
    #[msg("ACL app account authority does not match app account")]
    AppAccountAuthorityMismatch,
    #[msg("ACL record nonce key does not match")]
    AclNonceKeyMismatch,
    #[msg("ACL record address is not the canonical PDA for its nonce key")]
    AclRecordPdaMismatch,
    #[msg("ACL record nonce sequence does not match")]
    AclNonceSequenceMismatch,
    #[msg("ACL record domain key does not match")]
    AclDomainKeyMismatch,
    #[msg("ACL record app account does not match")]
    AclAppAccountMismatch,
    #[msg("ACL record encrypted value label does not match")]
    AclEncryptedValueLabelMismatch,
    #[msg("ACL record handle does not match")]
    AclHandleMismatch,
    #[msg("ACL record subject is not allowed")]
    AclSubjectMismatch,
    #[msg("ACL record has too many subjects")]
    AclSubjectCapacityExceeded,
    #[msg("previous bank hash is not available")]
    PreviousBankHashUnavailable,
}

fn write_acl_record(
    record: &mut Account<AclRecord>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[AclSubjectEntry],
    public_decrypt: bool,
    bump: u8,
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
    record.bump = bump;

    for (index, subject) in subjects.iter().enumerate() {
        record.subjects[index] = subject.pubkey;
    }
}

fn assert_record(
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

fn assert_canonical_acl_record(
    record_info: &AccountInfo,
    record: &Account<AclRecord>,
) -> Result<()> {
    assert_nonce_key_matches_fields(
        record.nonce_key,
        record.acl_domain_key,
        record.app_account,
        record.encrypted_value_label,
    )?;

    let expected = Pubkey::create_program_address(
        &[
            b"acl-record",
            record.nonce_key.as_ref(),
            &record.nonce_sequence.to_le_bytes(),
            &[record.bump],
        ],
        &crate::ID,
    )
    .map_err(|_| error!(ZamaHostError::AclRecordPdaMismatch))?;
    require_keys_eq!(
        record_info.key(),
        expected,
        ZamaHostError::AclRecordPdaMismatch
    );
    Ok(())
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

pub fn computed_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let scalar_byte = [u8::from(scalar)];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &op_byte,
        &lhs,
        &rhs,
        &scalar_byte,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

pub fn computed_trivial_handle(
    plaintext: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[2],
        &plaintext,
        &fhe_type_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

fn previous_bank_hash(current_slot: u64) -> Result<[u8; 32]> {
    let Some(previous_slot) = current_slot.checked_sub(1) else {
        return Ok([0; 32]);
    };
    let slot_hashes =
        PodSlotHashes::fetch().map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;
    if let Some(hash) = slot_hashes
        .get(&previous_slot)
        .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?
    {
        return Ok(hash.to_bytes());
    }

    // LiteSVM starts from an empty slot-hash history in these PoC tests.
    // Real cluster execution should take the branch above.
    Ok([0; 32])
}

fn assert_record_allows_handle(
    record: &Account<AclRecord>,
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

pub fn record_allows(record: &AclRecord, subject: Pubkey) -> bool {
    record.subjects[..record.subject_count as usize].contains(&subject)
}
