//! The single walk over an `fhe_execute` batch: resolve operands, assert operand
//! types, derive the produced handle, and hand each output to
//! [`super::EvalExecutionState`], which validates and mutates in one pass.
//!
//! A step that fails mid-batch reverts the whole transaction — the Solana
//! runtime discards every account write on error — so validating while
//! mutating needs no separate validate-only pass to stay atomic.

use super::*;

/// Per-batch slot entropy, identity, and rand anchor shared by every handle derivation.
pub(super) struct EvalHandleContext<'a> {
    pub derivation: HandleDerivationContext,
    /// Signed caller identity folded into rand seeds (never into deterministic handles).
    pub compute_subject: Pubkey,
    /// The batch's persistent-write anchor: every persistent output's live account
    /// identity, current handle, and MMR leaf count in wire order
    /// (see [`computed_eval_rand_seed`]).
    pub persistent_anchor_bytes: &'a [u8],
}

// Persistent and instruction-local outputs derive the identical handle, and identical
// computations collide by design: deterministic handles are content-addressed
// (op/operands/type + slot entropy, no salt), matching EVM `FHEVMExecutor`. Only
// rand seeds carry uniqueness — signer identity plus the persistent-write anchor.
impl EvalHandleContext<'_> {
    fn binary_result(
        &self,
        op: FheBinaryOpCode,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        output_fhe_type: u8,
    ) -> [u8; 32] {
        computed_eval_handle(op, lhs, rhs, scalar, output_fhe_type, &self.derivation)
    }

    fn ternary_result(
        &self,
        op: FheTernaryOpCode,
        control: [u8; 32],
        if_true: [u8; 32],
        if_false: [u8; 32],
        output_fhe_type: u8,
    ) -> [u8; 32] {
        computed_eval_ternary_handle(
            op,
            control,
            if_true,
            if_false,
            output_fhe_type,
            &self.derivation,
        )
    }

    fn trivial_result(&self, plaintext: [u8; 32], fhe_type: u8) -> [u8; 32] {
        computed_eval_trivial_handle(plaintext, fhe_type, &self.derivation)
    }

    pub(super) fn rand_seed(&self, op_index: u16) -> [u8; 16] {
        computed_eval_rand_seed(
            self.compute_subject,
            self.persistent_anchor_bytes,
            op_index,
            &self.derivation,
        )
    }

    fn unary_result(&self, op: FheUnaryOpCode, operand: [u8; 32], output_fhe_type: u8) -> [u8; 32] {
        computed_eval_unary_handle(op, operand, output_fhe_type, &self.derivation)
    }

    fn sum_result(&self, operand_handles: &[[u8; 32]], fhe_type: u8) -> [u8; 32] {
        computed_eval_sum_handle(operand_handles, fhe_type, &self.derivation)
    }

    fn is_in_result(
        &self,
        value_handle: [u8; 32],
        set_handles: &[[u8; 32]],
        fhe_type: u8,
    ) -> [u8; 32] {
        computed_eval_is_in_handle(value_handle, set_handles, fhe_type, &self.derivation)
    }

    fn mul_div_result(
        &self,
        factor1: [u8; 32],
        factor2: [u8; 32],
        scalar: bool,
        divisor: [u8; 32],
        output_fhe_type: u8,
    ) -> [u8; 32] {
        computed_eval_mul_div_handle(
            factor1,
            factor2,
            divisor,
            scalar,
            output_fhe_type,
            &self.derivation,
        )
    }
}

/// Operand resolvers shared by every step shape. Defined here so the
/// match-on-step skeleton and the operand rules read together; the
/// account-access and mutation halves live with the state in [`super`].
impl EvalExecutionState<'_, '_, '_> {
    /// Resolves an operand that must be encrypted (rejects scalars).
    fn resolve_encrypted_operand(
        &mut self,
        operand: &FheExecuteOperand,
    ) -> Result<ResolvedOperand> {
        match operand {
            FheExecuteOperand::AllowedPersistent {
                handle_index,
                encrypted_value_index,
            } => {
                let handle = self.dictionary_bytes(*handle_index)?;
                self.resolve_persistent_operand(handle, u16::from(*encrypted_value_index))
            }
            FheExecuteOperand::AllowedLocal { producer_index } => self
                .produced
                .get(*producer_index as usize)
                .map(ResolvedOperand::from_produced)
                .ok_or_else(|| error!(ZamaHostError::FheExecuteAllowedLocalMissing)),
            FheExecuteOperand::VerifiedInput { attestation } => {
                // EVM `fromExternal` parity: only the attested contract may consume the input.
                // Enforced here (the `msg.sender` analog) — not by constraining derived outputs.
                // `subject` is the eval's `compute_subject`; a copied attestation is useless
                // unless the caller can sign as `contract_address`.
                require_keys_eq!(
                    Pubkey::new_from_array(attestation.contract_address),
                    self.subject,
                    ZamaHostError::InputBindContractMismatch
                );
                self.resolve_verified_input_operand(attestation)
            }
            FheExecuteOperand::Scalar { .. } => {
                Err(error!(ZamaHostError::InvalidFheExecuteAccount))
            }
        }
    }

    /// Resolves a binary left-hand operand, which may not be a scalar.
    fn resolve_lhs_operand(&mut self, operand: &FheExecuteOperand) -> Result<ResolvedOperand> {
        match operand {
            FheExecuteOperand::Scalar { .. } => {
                Err(error!(ZamaHostError::InvalidFheExecuteAccount))
            }
            _ => self.resolve_encrypted_operand(operand),
        }
    }

    /// Resolves a binary right-hand operand, which may be a scalar.
    fn resolve_rhs_operand(&mut self, operand: &FheExecuteOperand) -> Result<ResolvedOperand> {
        match operand {
            FheExecuteOperand::Scalar { value_index } => Ok(ResolvedOperand::scalar(
                self.dictionary_bytes(*value_index)?,
            )),
            _ => self.resolve_encrypted_operand(operand),
        }
    }
}

/// Drives the execution state over every batch step: resolve operands, assert
/// operand types, compute the produced handle, and accept the output.
pub(super) fn walk_batch<'info>(
    execution: &mut EvalExecutionState<'_, '_, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    args: &FheExecuteArgs,
    handle_context: &EvalHandleContext<'_>,
) -> Result<()> {
    for (index, step) in args.steps.iter().enumerate() {
        let op_index = index as u16;
        match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                output,
            } => {
                let lhs = execution.resolve_lhs_operand(lhs)?;
                let rhs = execution.resolve_rhs_operand(rhs)?;
                assert_binary_operand_types(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                )?;
                let result = handle_context.binary_result(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                );
                execution.accept_output(
                    ctx,
                    op_index,
                    result,
                    output,
                    inputs_allow_public_decrypt(&lhs, &rhs),
                )?;
            }
            FheExecuteStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                output,
            } => {
                let control = execution.resolve_encrypted_operand(control)?;
                let if_true = execution.resolve_encrypted_operand(if_true)?;
                let if_false = execution.resolve_encrypted_operand(if_false)?;
                assert_ternary_operand_types(
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                )?;
                let result = handle_context.ternary_result(
                    *op,
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                );
                execution.accept_output(
                    ctx,
                    op_index,
                    result,
                    output,
                    inputs3_allow_public_decrypt(&control, &if_true, &if_false),
                )?;
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                output,
            } => {
                assert_supported_fhe_type(*fhe_type)?;
                let result = handle_context.trivial_result(*plaintext, *fhe_type);
                execution.accept_output(ctx, op_index, result, output, false)?;
            }
            FheExecuteStep::Rand { fhe_type, output } => {
                assert_supported_rand_type(*fhe_type)?;
                let seed = handle_context.rand_seed(op_index);
                let result =
                    computed_rand_handle(seed, *fhe_type, handle_context.derivation.chain_id);
                execution.accept_output(ctx, op_index, result, output, false)?;
            }
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
                output,
            } => {
                let operand = execution.resolve_encrypted_operand(operand)?;
                assert_unary_operand_type(*op, operand.handle, *output_fhe_type)?;
                let result = handle_context.unary_result(*op, operand.handle, *output_fhe_type);
                execution.accept_output(
                    ctx,
                    op_index,
                    result,
                    output,
                    operand.public_decrypt_allowed,
                )?;
            }
            FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type,
                output,
            } => {
                assert_valid_bounded_rand_upper_bound(*upper_bound, *fhe_type)?;
                let seed = handle_context.rand_seed(op_index);
                let result = computed_rand_bounded_handle(
                    *upper_bound,
                    seed,
                    *fhe_type,
                    handle_context.derivation.chain_id,
                );
                execution.accept_output(ctx, op_index, result, output, false)?;
            }
            FheExecuteStep::Sum {
                operands,
                fhe_type,
                output,
            } => {
                let mut resolved: Vec<ResolvedOperand> = Vec::with_capacity(operands.len());
                for operand in operands {
                    resolved.push(execution.resolve_encrypted_operand(operand)?);
                }
                let operand_handles: Vec<[u8; 32]> = resolved.iter().map(|r| r.handle).collect();
                assert_sum_operand_types(&operand_handles, *fhe_type)?;
                let public_decrypt = resolved.iter().all(|r| r.public_decrypt_allowed);
                let result = handle_context.sum_result(&operand_handles, *fhe_type);
                execution.accept_output(ctx, op_index, result, output, public_decrypt)?;
            }
            FheExecuteStep::IsIn {
                value,
                set,
                fhe_type,
                output,
            } => {
                let value_resolved = execution.resolve_encrypted_operand(value)?;
                let mut set_resolved: Vec<ResolvedOperand> = Vec::with_capacity(set.len());
                for operand in set {
                    set_resolved.push(execution.resolve_encrypted_operand(operand)?);
                }
                let set_handles: Vec<[u8; 32]> = set_resolved.iter().map(|r| r.handle).collect();
                assert_is_in_operand_types(value_resolved.handle, &set_handles, *fhe_type)?;
                let public_decrypt = value_resolved.public_decrypt_allowed
                    && set_resolved.iter().all(|r| r.public_decrypt_allowed);
                let result =
                    handle_context.is_in_result(value_resolved.handle, &set_handles, *fhe_type);
                execution.accept_output(ctx, op_index, result, output, public_decrypt)?;
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
                output,
            } => {
                let factor1 = execution.resolve_lhs_operand(factor1)?;
                let factor2 = execution.resolve_rhs_operand(factor2)?;
                assert_mul_div_operand_types(
                    factor1.handle,
                    factor2.handle,
                    factor2.scalar,
                    *divisor,
                    *output_fhe_type,
                )?;
                let result = handle_context.mul_div_result(
                    factor1.handle,
                    factor2.handle,
                    factor2.scalar,
                    *divisor,
                    *output_fhe_type,
                );
                execution.accept_output(
                    ctx,
                    op_index,
                    result,
                    output,
                    inputs_allow_public_decrypt(&factor1, &factor2),
                )?;
            }
        }
    }
    Ok(())
}
