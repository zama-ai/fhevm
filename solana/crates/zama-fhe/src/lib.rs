//! App-facing helpers for preparing `zama-host` FHE evaluation requests.
//!
//! This crate targets the role-aware host ABI. It lets app code work with durable
//! account pubkeys, then lowers those references to the indexed
//! `remaining_accounts` shape expected by [`FheEvalArgs`].
//!
//! Scope (unreleased preview): this is a **client-side request builder** only. It
//! produces an [`EvalPlan`] (the validated [`FheEvalArgs`] plus the ordered
//! account list) and performs every check it can detect before the host program
//! runs — see [`EvalBuildError`]. It does **not** yet perform the CPI itself; the
//! caller assembles the `AccountInfo`s in `remaining_accounts` order, derives the
//! compute-signer seeds, and invokes `zama_host::cpi::fhe_eval`. A future
//! `'info`-generic `execute()` driver that owns that CPI plumbing is the tracked
//! next step (DESIGN_DECISIONS.md "Transient-session SDK ergonomics").
//!
//! The eval batch is binary-only ([`FheEvalOp`] wraps a single [`FheBinaryOpCode`]);
//! ternary `if_then_else` and trivial-encrypt/rand are separate top-level host
//! instructions, not part of an [`FheEvalArgs`] frame.

use anchor_lang::prelude::Pubkey;

pub use zama_host::{
    acl_nonce_key, acl_record_address, AclSubjectEntry, FheBinaryOpCode, FheEvalArgs, FheEvalOp,
    FheEvalOperand, FheEvalOutput, TransientCapabilityGrant, MAX_FHE_EVAL_OPS,
};

/// Result type used by the builder helpers.
pub type Result<T> = std::result::Result<T, EvalBuildError>;

/// Builder failures that can be detected before invoking the host program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalBuildError {
    /// More accounts were referenced than fit in the host's `u16` indices.
    TooManyRemainingAccounts,
    /// A transient operand referenced an operation that has not been produced.
    InvalidTransientReference,
    /// More ops were added than the host accepts (`MAX_FHE_EVAL_OPS`).
    TooManyOps,
    /// `finish` was called with no ops; the host rejects empty eval frames.
    EmptyOps,
    /// `finish` was called with an all-zero `context_id`; the host rejects it.
    EmptyContextId,
    /// A scalar was supplied as the left-hand operand. The host invariant is
    /// scalar-RHS-only: the left operand must be an encrypted handle.
    ScalarLhsOperand,
}

/// Durable host operand identified by account pubkeys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurableOperand {
    pub handle: [u8; 32],
    pub acl_record: Pubkey,
    pub permission: Option<Pubkey>,
}

/// Sealed one-shot transient session operand identified by account pubkey.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransientSessionOperand {
    pub handle: [u8; 32],
    pub session: Pubkey,
    pub capability_index: u16,
}

/// Operand shape exposed to app programs and client builders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Durable(DurableOperand),
    Transient { producer_index: u16 },
    TransientSession(TransientSessionOperand),
    Scalar([u8; 32]),
}

impl Operand {
    pub fn durable(handle: [u8; 32], acl_record: Pubkey) -> Self {
        Self::Durable(DurableOperand {
            handle,
            acl_record,
            permission: None,
        })
    }

    pub fn durable_with_permission(
        handle: [u8; 32],
        acl_record: Pubkey,
        permission: Pubkey,
    ) -> Self {
        Self::Durable(DurableOperand {
            handle,
            acl_record,
            permission: Some(permission),
        })
    }

    pub fn transient(producer_index: u16) -> Self {
        Self::Transient { producer_index }
    }

    pub fn transient_session(handle: [u8; 32], session: Pubkey, capability_index: u16) -> Self {
        Self::TransientSession(TransientSessionOperand {
            handle,
            session,
            capability_index,
        })
    }

    pub fn scalar(value: [u8; 32]) -> Self {
        Self::Scalar(value)
    }

    /// Builds a `u64` scalar operand using the host's plaintext convention:
    /// big-endian in the low 8 bytes (`bytes[24..32]`), the rest zero.
    pub fn scalar_u64(value: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[24..].copy_from_slice(&value.to_be_bytes());
        Self::Scalar(bytes)
    }

    /// Builds a `u128` scalar operand using the host's plaintext convention:
    /// big-endian in the low 16 bytes (`bytes[16..32]`), the rest zero.
    pub fn scalar_u128(value: u128) -> Self {
        let mut bytes = [0u8; 32];
        bytes[16..].copy_from_slice(&value.to_be_bytes());
        Self::Scalar(bytes)
    }
}

/// Durable output metadata keyed by the app-selected ACL record pubkey.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableOutput {
    pub output_acl_record: Pubkey,
    pub output_nonce_key: [u8; 32],
    pub output_nonce_sequence: u64,
    pub output_acl_domain_key: Pubkey,
    pub output_app_account: Pubkey,
    pub output_encrypted_value_label: [u8; 32],
    pub output_subjects: Vec<AclSubjectEntry>,
    pub output_public_decrypt: bool,
}

/// Output policy exposed by the builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    Transient,
    TransientSession {
        session: Pubkey,
        capability: TransientCapabilityGrant,
    },
    Durable(DurableOutput),
}

/// Lowered eval request plus the account order expected by `remaining_accounts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalPlan {
    pub args: FheEvalArgs,
    pub remaining_accounts: Vec<Pubkey>,
}

/// Pubkey-oriented builder for `FheEvalArgs`.
#[derive(Debug, Clone)]
pub struct EvalBuilder {
    context_id: [u8; 32],
    ops: Vec<FheEvalOp>,
    remaining_accounts: Vec<Pubkey>,
}

impl EvalBuilder {
    pub fn new(context_id: [u8; 32]) -> Self {
        Self {
            context_id,
            ops: Vec::new(),
            remaining_accounts: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        lhs: Operand,
        rhs: Operand,
        output_fhe_type: u8,
        result: [u8; 32],
        output: Output,
    ) -> Result<Operand> {
        self.binary_op(
            FheBinaryOpCode::Add,
            lhs,
            rhs,
            output_fhe_type,
            result,
            output,
        )
    }

    pub fn sub(
        &mut self,
        lhs: Operand,
        rhs: Operand,
        output_fhe_type: u8,
        result: [u8; 32],
        output: Output,
    ) -> Result<Operand> {
        self.binary_op(
            FheBinaryOpCode::Sub,
            lhs,
            rhs,
            output_fhe_type,
            result,
            output,
        )
    }

    pub fn ge(
        &mut self,
        lhs: Operand,
        rhs: Operand,
        result: [u8; 32],
        output: Output,
    ) -> Result<Operand> {
        self.binary_op(FheBinaryOpCode::Ge, lhs, rhs, 0, result, output)
    }

    pub fn binary_op(
        &mut self,
        op: FheBinaryOpCode,
        lhs: Operand,
        rhs: Operand,
        output_fhe_type: u8,
        result: [u8; 32],
        output: Output,
    ) -> Result<Operand> {
        // The host requires the left operand to be an encrypted handle; only the
        // RHS may be a plaintext scalar. Catch this before the CPI.
        if matches!(lhs, Operand::Scalar(_)) {
            return Err(EvalBuildError::ScalarLhsOperand);
        }
        if self.ops.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        let op_index = u16::try_from(self.ops.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let lhs = self.lower_operand(lhs)?;
        let rhs = self.lower_operand(rhs)?;
        let output = self.lower_output(output)?;
        self.ops.push(FheEvalOp {
            op,
            lhs,
            rhs,
            output_fhe_type,
            result,
            output,
        });
        Ok(Operand::transient(op_index))
    }

    /// Validates the accumulated frame and lowers it to an [`EvalPlan`].
    ///
    /// Mirrors the host admission checks (`context_id != 0`, non-empty ops,
    /// `ops.len() <= MAX_FHE_EVAL_OPS`) so a malformed frame fails locally
    /// instead of on-chain.
    pub fn finish(self) -> Result<EvalPlan> {
        if self.context_id == [0u8; 32] {
            return Err(EvalBuildError::EmptyContextId);
        }
        if self.ops.is_empty() {
            return Err(EvalBuildError::EmptyOps);
        }
        if self.ops.len() > MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        Ok(EvalPlan {
            args: FheEvalArgs {
                context_id: self.context_id,
                ops: self.ops,
            },
            remaining_accounts: self.remaining_accounts,
        })
    }

    fn lower_operand(&mut self, operand: Operand) -> Result<FheEvalOperand> {
        match operand {
            Operand::Durable(durable) => {
                let acl_record_index = self.account_index(durable.acl_record)?;
                let permission_index = durable
                    .permission
                    .map(|permission| self.account_index(permission))
                    .transpose()?;
                Ok(FheEvalOperand::Durable {
                    handle: durable.handle,
                    acl_record_index,
                    permission_index,
                })
            }
            Operand::Transient { producer_index } => {
                if producer_index as usize >= self.ops.len() {
                    return Err(EvalBuildError::InvalidTransientReference);
                }
                Ok(FheEvalOperand::Transient { producer_index })
            }
            Operand::TransientSession(session) => Ok(FheEvalOperand::TransientSession {
                handle: session.handle,
                session_index: self.account_index(session.session)?,
                capability_index: session.capability_index,
            }),
            Operand::Scalar(value) => Ok(FheEvalOperand::Scalar(value)),
        }
    }

    fn lower_output(&mut self, output: Output) -> Result<FheEvalOutput> {
        match output {
            Output::Transient => Ok(FheEvalOutput::Transient),
            Output::TransientSession {
                session,
                capability,
            } => Ok(FheEvalOutput::TransientSession {
                session_index: self.account_index(session)?,
                capability,
            }),
            Output::Durable(output) => Ok(FheEvalOutput::Durable {
                output_acl_record_index: self.account_index(output.output_acl_record)?,
                output_nonce_key: output.output_nonce_key,
                output_nonce_sequence: output.output_nonce_sequence,
                output_acl_domain_key: output.output_acl_domain_key,
                output_app_account: output.output_app_account,
                output_encrypted_value_label: output.output_encrypted_value_label,
                output_subjects: output.output_subjects,
                output_public_decrypt: output.output_public_decrypt,
            }),
        }
    }

    fn account_index(&mut self, pubkey: Pubkey) -> Result<u16> {
        if let Some(index) = self
            .remaining_accounts
            .iter()
            .position(|candidate| *candidate == pubkey)
        {
            return u16::try_from(index).map_err(|_| EvalBuildError::TooManyRemainingAccounts);
        }
        let index = u16::try_from(self.remaining_accounts.len())
            .map_err(|_| EvalBuildError::TooManyRemainingAccounts)?;
        self.remaining_accounts.push(pubkey);
        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handle(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    #[test]
    fn lowers_pubkey_operands_to_stable_remaining_account_indices() {
        let acl_record = Pubkey::new_unique();
        let output_acl_record = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(handle(9));
        let produced = builder
            .add(
                Operand::durable(handle(1), acl_record),
                Operand::scalar(handle(2)),
                5,
                handle(3),
                Output::Durable(DurableOutput {
                    output_acl_record,
                    output_nonce_key: handle(4),
                    output_nonce_sequence: 7,
                    output_acl_domain_key: Pubkey::new_unique(),
                    output_app_account: Pubkey::new_unique(),
                    output_encrypted_value_label: handle(5),
                    output_subjects: Vec::new(),
                    output_public_decrypt: false,
                }),
            )
            .unwrap();
        builder
            .sub(
                produced,
                Operand::durable(handle(1), acl_record),
                5,
                handle(6),
                Output::Transient,
            )
            .unwrap();

        let plan = builder.finish().unwrap();
        assert_eq!(plan.remaining_accounts, vec![acl_record, output_acl_record]);
        match &plan.args.ops[0].lhs {
            FheEvalOperand::Durable {
                acl_record_index, ..
            } => assert_eq!(*acl_record_index, 0),
            other => panic!("unexpected operand: {other:?}"),
        }
        match &plan.args.ops[0].output {
            FheEvalOutput::Durable {
                output_acl_record_index,
                ..
            } => assert_eq!(*output_acl_record_index, 1),
            other => panic!("unexpected output: {other:?}"),
        }
        match &plan.args.ops[1].rhs {
            FheEvalOperand::Durable {
                acl_record_index, ..
            } => assert_eq!(*acl_record_index, 0),
            other => panic!("unexpected operand: {other:?}"),
        }
    }

    #[test]
    fn rejects_forward_transient_references() {
        let mut builder = EvalBuilder::new(handle(9));
        let error = builder
            .add(
                Operand::transient(0),
                Operand::scalar(handle(2)),
                5,
                handle(3),
                Output::Transient,
            )
            .unwrap_err();
        assert_eq!(error, EvalBuildError::InvalidTransientReference);
    }

    #[test]
    fn lowers_transient_session_operand_to_remaining_account_index() {
        let session = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(handle(9));
        builder
            .add(
                Operand::transient_session(handle(1), session, 0),
                Operand::scalar(handle(2)),
                5,
                handle(3),
                Output::Transient,
            )
            .unwrap();

        let plan = builder.finish().unwrap();
        assert_eq!(plan.remaining_accounts, vec![session]);
        match &plan.args.ops[0].lhs {
            FheEvalOperand::TransientSession {
                handle: operand_handle,
                session_index,
                capability_index,
            } => {
                assert_eq!(*operand_handle, handle(1));
                assert_eq!(*session_index, 0);
                assert_eq!(*capability_index, 0);
            }
            other => panic!("unexpected operand: {other:?}"),
        }
    }

    #[test]
    fn lowers_durable_operand_permission_to_remaining_account_index() {
        let acl_record = Pubkey::new_unique();
        let permission = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(handle(9));
        builder
            .add(
                Operand::durable_with_permission(handle(1), acl_record, permission),
                Operand::scalar_u64(2),
                5,
                handle(3),
                Output::Transient,
            )
            .unwrap();

        let plan = builder.finish().unwrap();
        assert_eq!(plan.remaining_accounts, vec![acl_record, permission]);
        match &plan.args.ops[0].lhs {
            FheEvalOperand::Durable {
                handle: operand_handle,
                acl_record_index,
                permission_index,
            } => {
                assert_eq!(*operand_handle, handle(1));
                assert_eq!(*acl_record_index, 0);
                assert_eq!(*permission_index, Some(1));
            }
            other => panic!("unexpected operand: {other:?}"),
        }
    }

    #[test]
    fn lowers_transient_session_output_to_remaining_account_index() {
        let acl_record = Pubkey::new_unique();
        let session = Pubkey::new_unique();
        let capability = TransientCapabilityGrant {
            subject: Pubkey::new_unique(),
            receiver_program: Pubkey::new_unique(),
            acl_domain_key: Pubkey::new_unique(),
            app_account: Pubkey::new_unique(),
            role_flags: zama_host::ACL_ROLE_USE,
            max_uses: 1,
            durable_output_allowed: false,
            public_decrypt_allowed: false,
        };
        let mut builder = EvalBuilder::new(handle(9));
        builder
            .add(
                Operand::durable(handle(1), acl_record),
                Operand::scalar_u64(2),
                5,
                handle(3),
                Output::TransientSession {
                    session,
                    capability,
                },
            )
            .unwrap();

        let plan = builder.finish().unwrap();
        assert_eq!(plan.remaining_accounts, vec![acl_record, session]);
        match &plan.args.ops[0].output {
            FheEvalOutput::TransientSession {
                session_index,
                capability: lowered_capability,
            } => {
                assert_eq!(*session_index, 1);
                assert_eq!(*lowered_capability, capability);
            }
            other => panic!("unexpected output: {other:?}"),
        }
    }

    #[test]
    fn rejects_scalar_left_operand() {
        let mut builder = EvalBuilder::new(handle(9));
        let error = builder
            .add(
                Operand::scalar_u64(7),
                Operand::durable(handle(1), Pubkey::new_unique()),
                5,
                handle(3),
                Output::Transient,
            )
            .unwrap_err();
        assert_eq!(error, EvalBuildError::ScalarLhsOperand);
    }

    #[test]
    fn finish_rejects_empty_context_id_and_empty_ops() {
        // Empty ops.
        assert_eq!(
            EvalBuilder::new(handle(9)).finish().unwrap_err(),
            EvalBuildError::EmptyOps
        );

        // Zero context id (even with a valid op).
        let mut builder = EvalBuilder::new([0u8; 32]);
        builder
            .add(
                Operand::durable(handle(1), Pubkey::new_unique()),
                Operand::scalar_u64(2),
                5,
                handle(3),
                Output::Transient,
            )
            .unwrap();
        assert_eq!(
            builder.finish().unwrap_err(),
            EvalBuildError::EmptyContextId
        );
    }

    #[test]
    fn rejects_more_than_max_ops() {
        let acl_record = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(handle(9));
        for index in 0..MAX_FHE_EVAL_OPS {
            builder
                .add(
                    Operand::durable(handle(1), acl_record),
                    Operand::scalar_u64(index as u64),
                    5,
                    handle(3),
                    Output::Transient,
                )
                .unwrap();
        }
        let error = builder
            .add(
                Operand::durable(handle(1), acl_record),
                Operand::scalar_u64(99),
                5,
                handle(3),
                Output::Transient,
            )
            .unwrap_err();
        assert_eq!(error, EvalBuildError::TooManyOps);
    }

    #[test]
    fn scalar_u64_uses_big_endian_low_bytes() {
        match Operand::scalar_u64(0x0102_0304_0506_0708) {
            Operand::Scalar(bytes) => {
                let mut expected = [0u8; 32];
                expected[24..].copy_from_slice(&0x0102_0304_0506_0708u64.to_be_bytes());
                assert_eq!(bytes, expected);
            }
            other => panic!("unexpected operand: {other:?}"),
        }
    }
}
