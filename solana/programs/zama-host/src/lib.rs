// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use solana_sha256_hasher::hashv;
use solana_sysvar::slot_hashes::PodSlotHashes;

declare_id!("EMhXFu68v61bQV4GrF6ZhZhWNVbH6bHPnTdLtXK8meqn");

pub const EVENT_VERSION: u8 = 0;
pub const MAX_ACL_SUBJECTS: usize = 8;
pub const MAX_FRAME_STEPS: usize = 16;
pub const MAX_FRAME_ACTIONS: usize = 16;
pub const MAX_FRAME_RESULTS: usize = 16;
pub const MAX_FRAME_TRANSIENT_ALLOWS: usize = 32;
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
    /// worker tests. Protocol flows should create ACL state through authenticated
    /// handle-producing instructions such as `trivial_encrypt_and_bind` or an
    /// input verifier.
    /// Input flows may use `mock_input_verified_and_bind` only as a temporary mock
    /// short-circuit until the verifier/transciphering boundary exists.
    pub fn test_emit_acl_allowed(
        ctx: Context<TestEmitProtocolEvent>,
        handle: [u8; 32],
        subject: Pubkey,
    ) -> Result<()> {
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle,
            subject: subject.to_bytes(),
        });
        Ok(())
    }

    pub fn allow_acl_subjects(
        ctx: Context<AllowAclSubjects>,
        handle: [u8; 32],
        subjects: Vec<AclSubjectEntry>,
    ) -> Result<()> {
        let authority = ctx.accounts.authority.key();
        assert_canonical_acl_record(
            &ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.acl_record,
        )?;
        assert_record_allows_handle(&ctx.accounts.acl_record, handle, authority)?;
        extend_acl_subjects(&mut ctx.accounts.acl_record, &subjects)?;

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
        steps: Vec<FheFrameStep>,
        actions: Vec<FheFrameAction>,
    ) -> Result<()> {
        require!(
            steps.len() <= MAX_FRAME_STEPS && actions.len() <= MAX_FRAME_ACTIONS,
            ZamaHostError::FrameLimitExceeded
        );

        let subject = ctx.accounts.compute_subject.key();
        let clock = Clock::get()?;
        let previous_bank_hash = previous_bank_hash(clock.slot)?;
        let mut frame = ExecutionFrame::new(subject);

        for step in steps {
            execute_frame_step(
                &mut frame,
                ctx.remaining_accounts,
                step,
                previous_bank_hash,
                clock.unix_timestamp,
            )?;
        }

        let payer = ctx.accounts.payer.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        for action in actions {
            apply_frame_action(
                &mut frame,
                payer.clone(),
                system_program.clone(),
                ctx.remaining_accounts,
                action,
            )?;
        }

        for event in frame.events {
            match event {
                FrameEvent::BinaryOp(event) => emit_cpi!(event),
                FrameEvent::TrivialEncrypt(event) => emit_cpi!(event),
                FrameEvent::AclAllowed(event) => emit_cpi!(event),
            }
        }

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
        assert_output_acl_metadata(
            output_nonce_key,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            &output_subjects,
        )?;

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
            subject: subject.to_bytes(),
            plaintext,
            fhe_type,
            result,
        });
        for output_subject in output_subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: result,
                subject: output_subject.pubkey.to_bytes(),
            });
        }
        Ok(())
    }

    /// Mock input-verification short-circuit for Solana input birth.
    ///
    /// This creates an ACL record for a caller-supplied input handle and emits
    /// the same event shape the worker currently consumes. It does not verify a
    /// ZKPoK, ciphertext preimage, or transciphering proof. The final input path
    /// must replace this with a real verifier authority before it is treated as
    /// protocol logic.
    pub fn mock_input_verified_and_bind(
        ctx: Context<MockInputVerifiedAndBind>,
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
        assert_output_acl_metadata(
            output_nonce_key,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            &output_subjects,
        )?;

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
            user: user.to_bytes(),
            acl_domain_key: output_acl_domain_key.to_bytes(),
        });
        for output_subject in output_subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: input_handle,
                subject: output_subject.pubkey.to_bytes(),
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
        result: [u8; 32],
    ) -> Result<()> {
        let subject = ctx.accounts.compute_subject.key();

        // Match the EVM executor boundary: no compute event is emitted until
        // the host program verifies that the compute subject can use the
        // operand handles.
        assert_canonical_acl_record(
            &ctx.accounts.lhs_acl_record.to_account_info(),
            &ctx.accounts.lhs_acl_record,
        )?;
        assert_record_allows_handle(&ctx.accounts.lhs_acl_record, lhs, subject)?;
        if !scalar {
            assert_unchecked_acl_record_allows_handle(
                &ctx.accounts.rhs_acl_record.to_account_info(),
                rhs,
                subject,
            )?;
        }

        let clock = Clock::get()?;
        let previous_bank_hash = previous_bank_hash(clock.slot)?;
        let expected_result = computed_binary_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            SOLANA_POC_CHAIN_ID,
            previous_bank_hash,
            clock.unix_timestamp,
        );
        require!(
            result == expected_result,
            ZamaHostError::ComputedHandleMismatch
        );

        // Future scalar and ternary ops must keep the EVM scalarByte rule:
        // bit i flags whether the i-th argument from the right is scalar.
        // Example for mulDiv(lhs, rhs, divisor):
        // enc x enc x scalar => 0x01, enc x scalar x scalar => 0x03.
        emit_cpi!(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op,
            subject: subject.to_bytes(),
            lhs,
            rhs,
            scalar,
            result,
        });
        Ok(())
    }

    pub fn fhe_binary_op_and_bind_output(
        ctx: Context<FheBinaryOpAndBindOutput>,
        op: FheBinaryOpCode,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        output_fhe_type: u8,
        result: [u8; 32],
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        let subject = ctx.accounts.compute_subject.key();
        assert_output_acl_metadata(
            output_nonce_key,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            &output_subjects,
        )?;

        assert_canonical_acl_record(
            &ctx.accounts.lhs_acl_record.to_account_info(),
            &ctx.accounts.lhs_acl_record,
        )?;
        assert_record_allows_handle(&ctx.accounts.lhs_acl_record, lhs, subject)?;
        if !scalar {
            assert_unchecked_acl_record_allows_handle(
                &ctx.accounts.rhs_acl_record.to_account_info(),
                rhs,
                subject,
            )?;
        }

        let clock = Clock::get()?;
        let previous_bank_hash = previous_bank_hash(clock.slot)?;
        let expected_result = computed_bound_binary_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            SOLANA_POC_CHAIN_ID,
            previous_bank_hash,
            clock.unix_timestamp,
            output_nonce_key,
            output_nonce_sequence,
        );
        require!(
            result == expected_result,
            ZamaHostError::ComputedHandleMismatch
        );

        emit_cpi!(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op,
            subject: subject.to_bytes(),
            lhs,
            rhs,
            scalar,
            result,
        });

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

        for output_subject in output_subjects {
            emit_cpi!(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: result,
                subject: output_subject.pubkey.to_bytes(),
            });
        }
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
            subject: subject.to_bytes(),
            plaintext,
            fhe_type,
            result,
        });
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
            subject: subject.to_bytes(),
            seed,
            fhe_type,
            result,
        });
        Ok(())
    }

    /// Test-only event shim.
    ///
    /// This only emits an input-verification event. The PoC ACL-bearing stand-in
    /// is `mock_input_verified_and_bind`; the final version should require the real
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
            user: user.to_bytes(),
            acl_domain_key: acl_domain_key.to_bytes(),
        });
        Ok(())
    }
}

/// Accounts for test-only event shims that bypass protocol state writes.
#[derive(Accounts)]
#[event_cpi]
pub struct TestEmitProtocolEvent {}

#[derive(Accounts)]
#[event_cpi]
pub struct AllowAclSubjects<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub acl_record: Account<'info, AclRecord>,
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

/// Accounts for the mock input short-circuit.
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
pub struct MockInputVerifiedAndBind<'info> {
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
#[event_cpi]
pub struct ExecuteFrame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub compute_subject: Signer<'info>,
    pub app_account_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8
)]
#[event_cpi]
pub struct FheBinaryOp<'info> {
    pub compute_subject: Signer<'info>,
    pub lhs_acl_record: Account<'info, AclRecord>,
    /// CHECK: encrypted RHS operands are deserialized and ACL-checked in the
    /// instruction body; scalar RHS operands deliberately skip this account.
    pub rhs_acl_record: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    result: [u8; 32],
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct FheBinaryOpAndBindOutput<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub compute_subject: Signer<'info>,
    pub app_account_authority: Signer<'info>,
    pub lhs_acl_record: Account<'info, AclRecord>,
    /// CHECK: encrypted RHS operands are deserialized and ACL-checked in the
    /// instruction body; scalar RHS operands deliberately skip this account.
    pub rhs_acl_record: UncheckedAccount<'info>,
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
    #[msg("computed handle does not match host formula")]
    ComputedHandleMismatch,
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

#[derive(Clone, Copy)]
struct FrameResult {
    handle: [u8; 32],
}

struct ExecutionFrame {
    subject: Pubkey,
    results: Vec<FrameResult>,
    transient_allows: Vec<([u8; 32], Pubkey)>,
    events: Vec<FrameEvent>,
}

enum FrameEvent {
    BinaryOp(FheBinaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
    AclAllowed(AclAllowedEvent),
}

#[derive(Clone, Copy)]
struct ResolvedOperand {
    value: [u8; 32],
    is_scalar: bool,
}

impl ExecutionFrame {
    fn new(subject: Pubkey) -> Self {
        Self {
            subject,
            results: Vec::new(),
            transient_allows: Vec::new(),
            events: Vec::new(),
        }
    }

    fn push_result(&mut self, result: FrameResult) -> Result<()> {
        require!(
            self.results.len() < MAX_FRAME_RESULTS,
            ZamaHostError::FrameLimitExceeded
        );
        self.results.push(result);
        self.allow_transient(result.handle)
    }

    fn allow_transient(&mut self, handle: [u8; 32]) -> Result<()> {
        require!(
            self.transient_allows.len() < MAX_FRAME_TRANSIENT_ALLOWS,
            ZamaHostError::FrameLimitExceeded
        );
        self.transient_allows.push((handle, self.subject));
        Ok(())
    }

    fn transient_allows(&self, handle: [u8; 32], subject: Pubkey) -> bool {
        self.transient_allows
            .iter()
            .any(|(allowed_handle, allowed_subject)| {
                *allowed_handle == handle && *allowed_subject == subject
            })
    }
}

fn execute_frame_step<'info>(
    frame: &mut ExecutionFrame,
    remaining_accounts: &[AccountInfo<'info>],
    step: FheFrameStep,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> Result<()> {
    match step {
        FheFrameStep::Operation {
            opcode,
            operands,
            scalar_byte,
            output_fhe_type,
        } => execute_operation_step(
            frame,
            remaining_accounts,
            opcode,
            operands,
            scalar_byte,
            output_fhe_type,
            previous_bank_hash,
            unix_timestamp,
        ),
        FheFrameStep::TrivialEncrypt {
            plaintext,
            fhe_type,
        } => {
            let result = computed_trivial_handle(
                plaintext,
                fhe_type,
                SOLANA_POC_CHAIN_ID,
                previous_bank_hash,
                unix_timestamp,
            );
            frame.push_result(FrameResult { handle: result })?;
            frame
                .events
                .push(FrameEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: frame.subject.to_bytes(),
                    plaintext,
                    fhe_type,
                    result,
                }));
            Ok(())
        }
    }
}

fn execute_operation_step<'info>(
    frame: &mut ExecutionFrame,
    remaining_accounts: &[AccountInfo<'info>],
    opcode: FheOpcode,
    operands: Vec<FheOperand>,
    scalar_byte: u8,
    output_fhe_type: u8,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> Result<()> {
    let binary_op = FheBinaryOpCode::try_from(opcode)?;
    require!(operands.len() == 2, ZamaHostError::InvalidFrameOperands);
    let lhs = resolve_operand(frame, remaining_accounts, &operands[0])?;
    let rhs = resolve_operand(frame, remaining_accounts, &operands[1])?;
    require!(!lhs.is_scalar, ZamaHostError::InvalidFrameOperands);
    let scalar = rhs.is_scalar;
    require!(
        scalar_byte == u8::from(scalar),
        ZamaHostError::InvalidFrameOperands
    );

    let result = computed_binary_handle(
        binary_op,
        lhs.value,
        rhs.value,
        scalar,
        output_fhe_type,
        SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
    );
    frame.push_result(FrameResult { handle: result })?;
    frame.events.push(FrameEvent::BinaryOp(FheBinaryOpEvent {
        version: EVENT_VERSION,
        op: binary_op,
        subject: frame.subject.to_bytes(),
        lhs: lhs.value,
        rhs: rhs.value,
        scalar,
        result,
    }));
    Ok(())
}

fn apply_frame_action<'info>(
    frame: &mut ExecutionFrame,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    remaining_accounts: &[AccountInfo<'info>],
    action: FheFrameAction,
) -> Result<()> {
    match action {
        FheFrameAction::Allow {
            source,
            output_acl_record_index,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            subjects,
            public_decrypt,
        } => {
            let source = resolve_operand(frame, remaining_accounts, &source)?;
            require!(!source.is_scalar, ZamaHostError::InvalidFrameOperands);
            require!(
                is_allowed(frame, remaining_accounts, source.value, frame.subject)?,
                ZamaHostError::AclSubjectMismatch
            );
            assert_output_acl_metadata(
                nonce_key,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                &subjects,
            )?;
            let output_acl_record = account_at(remaining_accounts, output_acl_record_index)?;
            create_acl_record_account(
                &payer,
                &output_acl_record,
                &system_program,
                nonce_key,
                nonce_sequence,
            )?;
            write_acl_record_data(
                &output_acl_record,
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                source.value,
                &subjects,
                public_decrypt,
            )?;
            for subject in subjects {
                frame.events.push(FrameEvent::AclAllowed(AclAllowedEvent {
                    version: EVENT_VERSION,
                    handle: source.value,
                    subject: subject.pubkey.to_bytes(),
                }));
            }
            Ok(())
        }
        FheFrameAction::AllowForDecryption {
            source,
            acl_record_index,
        } => {
            let source = resolve_operand(frame, remaining_accounts, &source)?;
            require!(!source.is_scalar, ZamaHostError::InvalidFrameOperands);
            require!(
                is_allowed(frame, remaining_accounts, source.value, frame.subject)?,
                ZamaHostError::AclSubjectMismatch
            );
            let acl_record_info = account_at(remaining_accounts, acl_record_index)?;
            let mut record = deserialize_acl_record(&acl_record_info)?;
            assert_canonical_acl_record_data(acl_record_info.key(), &record)?;
            require!(
                record.handle == source.value,
                ZamaHostError::AclHandleMismatch
            );
            record.public_decrypt = true;
            serialize_acl_record(&acl_record_info, &record)?;
            frame.events.push(FrameEvent::AclAllowed(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: source.value,
                subject: frame.subject.to_bytes(),
            }));
            Ok(())
        }
    }
}

fn resolve_operand<'info>(
    frame: &ExecutionFrame,
    remaining_accounts: &[AccountInfo<'info>],
    operand: &FheOperand,
) -> Result<ResolvedOperand> {
    match operand {
        FheOperand::AclRecord {
            handle,
            account_index,
        } => {
            let record_info = account_at(remaining_accounts, *account_index)?;
            let record = deserialize_acl_record(&record_info)?;
            assert_canonical_acl_record_data(record_info.key(), &record)?;
            assert_record_allows_handle(&record, *handle, frame.subject)?;
            Ok(ResolvedOperand {
                value: *handle,
                is_scalar: false,
            })
        }
        FheOperand::PreviousResult { index } => {
            let result = frame
                .results
                .get(*index as usize)
                .ok_or_else(|| error!(ZamaHostError::FrameResultIndexOutOfRange))?;
            require!(
                frame.transient_allows(result.handle, frame.subject),
                ZamaHostError::AclSubjectMismatch
            );
            Ok(ResolvedOperand {
                value: result.handle,
                is_scalar: false,
            })
        }
        FheOperand::Scalar { value, .. } => Ok(ResolvedOperand {
            value: *value,
            is_scalar: true,
        }),
    }
}

fn is_allowed<'info>(
    frame: &ExecutionFrame,
    remaining_accounts: &[AccountInfo<'info>],
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<bool> {
    if frame.transient_allows(handle, subject) {
        return Ok(true);
    }
    for account in remaining_accounts {
        if *account.owner != crate::ID || account.data_is_empty() {
            continue;
        }
        let record = deserialize_acl_record(account)?;
        if assert_canonical_acl_record_data(account.key(), &record).is_ok()
            && record.handle == handle
            && record_allows(&record, subject)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn account_at<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    index: u8,
) -> Result<AccountInfo<'info>> {
    remaining_accounts
        .get(index as usize)
        .cloned()
        .ok_or_else(|| error!(ZamaHostError::FrameAccountIndexOutOfRange))
}

fn create_acl_record_account<'info>(
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

fn write_acl_record_data<'info>(
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

fn deserialize_acl_record<'info>(record_info: &AccountInfo<'info>) -> Result<AclRecord> {
    require_keys_eq!(
        *record_info.owner,
        crate::ID,
        ZamaHostError::AclRecordPdaMismatch
    );
    let data = record_info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    AclRecord::try_deserialize(&mut data_slice)
}

fn serialize_acl_record<'info>(record_info: &AccountInfo<'info>, record: &AclRecord) -> Result<()> {
    let mut data = record_info.try_borrow_mut_data()?;
    let mut data_slice: &mut [u8] = &mut data;
    record.try_serialize(&mut data_slice)?;
    Ok(())
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
    record.bump = bump;
    write_acl_record_fields(
        record,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        subjects,
        public_decrypt,
    );
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

fn extend_acl_subjects(
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

fn assert_output_acl_metadata(
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
    assert_canonical_acl_record_data(record_info.key(), record)
}

fn assert_canonical_acl_record_data(record_key: Pubkey, record: &AclRecord) -> Result<()> {
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

fn assert_unchecked_acl_record_allows_handle(
    record_info: &AccountInfo,
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        *record_info.owner,
        crate::ID,
        ZamaHostError::AclRecordPdaMismatch
    );
    let data = record_info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = AclRecord::try_deserialize(&mut data_slice)?;
    assert_canonical_acl_record_data(record_info.key(), &record)?;
    assert_record_allows_handle(&record, handle, subject)
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

pub fn computed_binary_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
    ))
}

pub fn computed_bound_binary_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_bound_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    ))
}

pub fn computed_trivial_handle_for_current_slot(
    plaintext: [u8; 32],
    fhe_type: u8,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_trivial_handle(
        plaintext,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
    ))
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

pub fn computed_bound_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &hashv(&[
            b"FHE_bound_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
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

pub fn record_allows(record: &AclRecord, subject: Pubkey) -> bool {
    record.subjects[..record.subject_count as usize].contains(&subject)
}
