// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

use anchor_lang::prelude::*;

mod acl;
mod frame;
mod handles;

pub use acl::{acl_nonce_key, record_allows};
pub use handles::{computed_binary_handle, computed_trivial_handle};

declare_id!("EMhXFu68v61bQV4GrF6ZhZhWNVbH6bHPnTdLtXK8meqn");

pub const EVENT_VERSION: u8 = 0;
pub const MAX_ACL_SUBJECTS: usize = 8;
pub const MAX_FRAME_STEPS: usize = 16;
pub const MAX_FRAME_ACTIONS: usize = 16;
pub const MAX_FRAME_RESULTS: usize = 16;
pub const MAX_FRAME_TRANSIENT_ALLOWS: usize = 32;
pub const SOLANA_POC_CHAIN_ID: u64 = 12345;

#[program]
pub mod zama_host {
    use super::*;

    pub fn allow_acl_subjects(
        ctx: Context<AllowAclSubjects>,
        handle: [u8; 32],
        subjects: Vec<AclSubjectEntry>,
    ) -> Result<()> {
        let authority = ctx.accounts.authority.key();
        acl::assert_canonical_acl_record(
            &ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.acl_record,
        )?;
        acl::assert_record_allows_handle(&ctx.accounts.acl_record, handle, authority)?;
        acl::extend_acl_subjects(&mut ctx.accounts.acl_record, &subjects)?;

        for subject in subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle,
                subject: subject.pubkey.to_bytes(),
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
        acl::assert_record(
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
        acl::assert_canonical_acl_record(
            &ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.acl_record,
        )?;
        acl::assert_record_allows_handle(&ctx.accounts.acl_record, handle, subject)?;
        // EVM parity: any subject allowed on the handle may mark it as allowed
        // for decryption. `allowTransient` can satisfy this check on EVM, so
        // the Solana transient-allow design must preserve that same behavior.
        ctx.accounts.acl_record.public_decrypt = true;
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle,
            subject: subject.to_bytes(),
        });
        Ok(())
    }

    pub fn execute_frame<'info>(
        ctx: Context<'info, ExecuteFrame<'info>>,
        authorized_app_accounts: Vec<Pubkey>,
        steps: Vec<FheFrameStep>,
        actions: Vec<FheFrameAction>,
    ) -> Result<()> {
        require!(
            steps.len() <= MAX_FRAME_STEPS && actions.len() <= MAX_FRAME_ACTIONS,
            ZamaHostError::FrameLimitExceeded
        );
        require!(
            authorized_app_accounts.len() <= MAX_FRAME_ACTIONS,
            ZamaHostError::FrameLimitExceeded
        );

        let subject = ctx.accounts.compute_subject.key();
        let clock = Clock::get()?;
        let previous_bank_hash = handles::previous_bank_hash(clock.slot)?;
        let events = frame::execute(
            subject,
            &authorized_app_accounts,
            ctx.remaining_accounts,
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            steps,
            actions,
            previous_bank_hash,
            clock.unix_timestamp,
        )?;

        for event in events {
            match event {
                frame::FrameEvent::BinaryOp(event) => emit_cpi!(event),
                frame::FrameEvent::TrivialEncrypt(event) => emit_cpi!(event),
                frame::FrameEvent::AclAllowed(event) => emit_cpi!(event),
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[event_cpi]
pub struct AllowAclSubjects<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub acl_record: Account<'info, AclRecord>,
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
pub struct ExecuteFrame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub compute_subject: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AclSubjectEntry {
    pub pubkey: Pubkey,
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
    pub subject: [u8; 32],
    pub lhs: [u8; 32],
    pub rhs: [u8; 32],
    pub scalar: bool,
    pub result: [u8; 32],
}

#[event]
pub struct TrivialEncryptEvent {
    pub version: u8,
    pub subject: [u8; 32],
    pub plaintext: [u8; 32],
    pub fhe_type: u8,
    pub result: [u8; 32],
}

#[event]
pub struct FheRandEvent {
    pub version: u8,
    pub subject: [u8; 32],
    pub seed: [u8; 16],
    pub fhe_type: u8,
    pub result: [u8; 32],
}

#[event]
pub struct AclAllowedEvent {
    pub version: u8,
    pub handle: [u8; 32],
    pub subject: [u8; 32],
}

#[event]
pub struct InputVerifiedEvent {
    pub version: u8,
    pub input_handle: [u8; 32],
    pub result_handle: [u8; 32],
    pub user: [u8; 32],
    pub acl_domain_key: [u8; 32],
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
pub enum FheOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Rotl,
    Rotr,
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
    Min,
    Max,
    Neg,
    Not,
    VerifyInput,
    Cast,
    TrivialEncrypt,
    IfThenElse,
    Rand,
    RandBounded,
    Sum,
    IsIn,
    MulDiv,
}

impl FheOpcode {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Div => 3,
            Self::Rem => 4,
            Self::BitAnd => 5,
            Self::BitOr => 6,
            Self::BitXor => 7,
            Self::Shl => 8,
            Self::Shr => 9,
            Self::Rotl => 10,
            Self::Rotr => 11,
            Self::Eq => 12,
            Self::Ne => 13,
            Self::Ge => 14,
            Self::Gt => 15,
            Self::Le => 16,
            Self::Lt => 17,
            Self::Min => 18,
            Self::Max => 19,
            Self::Neg => 20,
            Self::Not => 21,
            Self::VerifyInput => 22,
            Self::Cast => 23,
            Self::TrivialEncrypt => 24,
            Self::IfThenElse => 25,
            Self::Rand => 26,
            Self::RandBounded => 27,
            Self::Sum => 28,
            Self::IsIn => 29,
            Self::MulDiv => 30,
        }
    }
}

impl TryFrom<FheOpcode> for FheBinaryOpCode {
    type Error = Error;

    fn try_from(value: FheOpcode) -> Result<Self> {
        match value {
            FheOpcode::Add => Ok(Self::Add),
            FheOpcode::Sub => Ok(Self::Sub),
            _ => err!(ZamaHostError::UnsupportedFrameOpcode),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheOperand {
    AclRecord { handle: [u8; 32], account_index: u8 },
    PreviousResult { index: u8 },
    Scalar { value: [u8; 32], fhe_type: u8 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheFrameStep {
    Operation {
        opcode: FheOpcode,
        operands: Vec<FheOperand>,
        scalar_byte: u8,
        output_fhe_type: u8,
    },
    TrivialEncrypt {
        plaintext: [u8; 32],
        fhe_type: u8,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheFrameAction {
    Allow {
        source: FheOperand,
        output_acl_record_index: u8,
        nonce_key: [u8; 32],
        nonce_sequence: u64,
        acl_domain_key: Pubkey,
        app_account: Pubkey,
        encrypted_value_label: [u8; 32],
        subjects: Vec<AclSubjectEntry>,
        public_decrypt: bool,
    },
    AllowForDecryption {
        source: FheOperand,
        acl_record_index: u8,
    },
}

#[error_code]
pub enum ZamaHostError {
    #[msg("Allow app account is not in authorized_app_accounts")]
    UnauthorizedAppAccount,
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
    #[msg("frame exceeds configured limit")]
    FrameLimitExceeded,
    #[msg("frame account index is out of range")]
    FrameAccountIndexOutOfRange,
    #[msg("frame result index is out of range")]
    FrameResultIndexOutOfRange,
    #[msg("frame opcode is not supported by this PoC")]
    UnsupportedFrameOpcode,
    #[msg("frame step has invalid operands")]
    InvalidFrameOperands,
    #[msg("frame action targets an initialized account")]
    FrameOutputAccountAlreadyInitialized,
}
