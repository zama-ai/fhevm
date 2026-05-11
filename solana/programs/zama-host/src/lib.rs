use anchor_lang::prelude::*;

declare_id!("EMhXFu68v61bQV4GrF6ZhZhWNVbH6bHPnTdLtXK8meqn");

pub const EVENT_VERSION: u8 = 0;

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
        acl_nonce: u64,
        scope: Pubkey,
        handle: [u8; 32],
        subject: Pubkey,
        permission: AclPermission,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.scope_authority.key(),
            scope,
            ZamaHostError::ScopeAuthorityMismatch
        );
        write_acl_record(
            &mut ctx.accounts.acl_record,
            acl_nonce,
            scope,
            handle,
            subject,
            permission,
            ctx.bumps.acl_record,
        );

        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle,
            subject,
            permission,
        });
        Ok(())
    }

    pub fn assert_acl_record(
        ctx: Context<AssertAclRecord>,
        scope: Pubkey,
        handle: [u8; 32],
        subject: Pubkey,
        permission: AclPermission,
    ) -> Result<()> {
        assert_record(&ctx.accounts.acl_record, scope, handle, subject, permission)?;
        Ok(())
    }

    pub fn fhe_binary_op(
        ctx: Context<EmitProtocolEvent>,
        op: FheBinaryOpCode,
        subject: Pubkey,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        result: [u8; 32],
    ) -> Result<()> {
        emit_cpi!(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op,
            subject,
            lhs,
            rhs,
            scalar,
            result,
        });
        drop(ctx);
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
        app_context: Pubkey,
    ) -> Result<()> {
        emit_cpi!(InputVerifiedEvent {
            version: EVENT_VERSION,
            input_handle,
            result_handle,
            user,
            app_context,
        });
        drop(ctx);
        Ok(())
    }
}

#[derive(Accounts)]
#[event_cpi]
pub struct EmitProtocolEvent {}

#[derive(Accounts)]
#[instruction(acl_nonce: u64, scope: Pubkey, _handle: [u8; 32], subject: Pubkey)]
#[event_cpi]
pub struct BindAclRecord<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub scope_authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [b"acl", scope.as_ref(), subject.as_ref(), &acl_nonce.to_le_bytes()],
        bump
    )]
    pub acl_record: Account<'info, AclRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssertAclRecord<'info> {
    pub acl_record: Account<'info, AclRecord>,
}

#[account]
pub struct AclRecord {
    pub acl_nonce: u64,
    pub scope: Pubkey,
    pub handle: [u8; 32],
    pub subject: Pubkey,
    pub permission: AclPermission,
    pub bump: u8,
}

impl AclRecord {
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 1 + 1;
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
    pub app_context: Pubkey,
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
    #[msg("ACL scope authority does not match scope")]
    ScopeAuthorityMismatch,
    #[msg("ACL record scope does not match")]
    AclScopeMismatch,
    #[msg("ACL record handle does not match")]
    AclHandleMismatch,
    #[msg("ACL record subject does not match")]
    AclSubjectMismatch,
    #[msg("ACL record permission does not match")]
    AclPermissionMismatch,
}

fn write_acl_record(
    record: &mut Account<AclRecord>,
    acl_nonce: u64,
    scope: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
    permission: AclPermission,
    bump: u8,
) {
    record.acl_nonce = acl_nonce;
    record.scope = scope;
    record.handle = handle;
    record.subject = subject;
    record.permission = permission;
    record.bump = bump;
}

fn assert_record(
    record: &Account<AclRecord>,
    scope: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
    permission: AclPermission,
) -> Result<()> {
    require_keys_eq!(record.scope, scope, ZamaHostError::AclScopeMismatch);
    require!(record.handle == handle, ZamaHostError::AclHandleMismatch);
    require_keys_eq!(record.subject, subject, ZamaHostError::AclSubjectMismatch);
    require!(
        record.permission == permission,
        ZamaHostError::AclPermissionMismatch
    );
    Ok(())
}
