use anchor_lang::prelude::*;

declare_id!("EMhXFu68v61bQV4GrF6ZhZhWNVbH6bHPnTdLtXK8meqn");

pub const EVENT_VERSION: u8 = 0;
pub const MAX_ACL_SUBJECTS: usize = 8;

#[program]
pub mod zama_host {
    use super::*;

    pub fn allow_handle(
        ctx: Context<EmitProtocolEvent>,
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
        require_keys_eq!(
            ctx.accounts.app_account_authority.key(),
            app_account,
            ZamaHostError::AppAccountAuthorityMismatch
        );
        require!(
            !subjects.is_empty() && subjects.len() <= MAX_ACL_SUBJECTS,
            ZamaHostError::AclSubjectCapacityExceeded
        );

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

    pub fn fhe_binary_op(
        ctx: Context<FheBinaryOp>,
        op: FheBinaryOpCode,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        result: [u8; 32],
    ) -> Result<()> {
        let subject = ctx.accounts.compute_subject.key();

        // Match the EVM executor boundary: no compute event is emitted until
        // the host program verifies that the compute subject can use the
        // operand handles.
        assert_record_allows_handle(&ctx.accounts.lhs_acl_record, lhs, subject)?;
        if !scalar {
            assert_record_allows_handle(&ctx.accounts.rhs_acl_record, rhs, subject)?;
        }

        // Future scalar and ternary ops must keep the EVM scalarByte rule:
        // bit i flags whether the i-th argument from the right is scalar.
        // Example for mulDiv(lhs, rhs, divisor):
        // enc x enc x scalar => 0x01, enc x scalar x scalar => 0x03.
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

    pub fn trivial_encrypt(
        ctx: Context<EmitProtocolEvent>,
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

    pub fn fhe_rand(
        ctx: Context<EmitProtocolEvent>,
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

    pub fn input_verified(
        ctx: Context<EmitProtocolEvent>,
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

#[derive(Accounts)]
#[event_cpi]
pub struct EmitProtocolEvent {}

#[derive(Accounts)]
#[instruction(nonce_key: [u8; 32], nonce_sequence: u64)]
#[event_cpi]
pub struct BindAclRecord<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub app_account_authority: Signer<'info>,
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
#[event_cpi]
pub struct FheBinaryOp<'info> {
    pub compute_subject: Signer<'info>,
    pub lhs_acl_record: Account<'info, AclRecord>,
    pub rhs_acl_record: Account<'info, AclRecord>,
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
    record: &Account<AclRecord>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<()> {
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
