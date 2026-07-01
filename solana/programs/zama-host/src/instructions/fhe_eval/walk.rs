//! Single operand-resolution + step-walk path shared by the admission
//! (validate-only) and execution (mutate) phases of [`super::fhe_eval`].
//!
//! Both phases parse the same plan and resolve the same operands; only the
//! side effects differ (admission tracks planned mutations in memory, execution
//! performs them). The two phases plug their differences into
//! [`EvalStepVisitor`] so the match-on-step skeleton and the operand resolvers
//! exist exactly once.

use super::handles::{
    expected_binary_eval_result, expected_is_in_eval_result, expected_mul_div_eval_result,
    expected_rand_bounded_eval_seed, expected_rand_eval_seed, expected_sum_eval_result,
    expected_ternary_eval_result, expected_trivial_eval_result, expected_unary_eval_result,
    EvalHandleContext,
};
use super::*;

/// Operands resolved identically by both phases (durable input role checks and
/// transient producer lookups), parameterized by the phase-specific account
/// access and transient-session handling.
pub(super) trait EvalStepVisitor {
    /// Subject required to hold the input role on durable ACL records.
    fn subject(&self) -> Pubkey;
    /// Transient values produced by earlier steps in this frame.
    fn produced(&self) -> &[ProducedValue];

    /// Resolves a durable encrypted input, fetching its ACL record (and optional
    /// overflow permission witness) the way this phase fetches accounts.
    fn resolve_durable_operand(
        &mut self,
        handle: [u8; 32],
        acl_record_index: u16,
        permission_index: Option<u16>,
    ) -> Result<ResolvedOperand>;

    /// Resolves an external input verified in-frame via the coprocessor attestation. Admission
    /// resolves it structurally (the handle is known from the operand data); execution re-runs the
    /// secp256k1 attestation authoritatively. Instruction-local — no account, no PDA.
    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand>;

    /// Records the per-op event for the produced handle. Admission ignores it;
    /// execution buffers it for transport.
    fn record_op_event(&mut self, event: EvalEvent);

    /// Validates and applies a produced output (instruction-local or durable).
    /// Admission validates and plans; execution validates and mutates.
    fn accept_output<'info>(
        &mut self,
        ctx: &Context<'info, FheEval<'info>>,
        result: [u8; 32],
        output: &FheEvalOutput,
        output_public_decrypt_allowed: bool,
        enforce_public_decrypt_role_propagation: bool,
        verified_input: Option<VerifiedInputBinding>,
    ) -> Result<()>;

    /// Resolves an operand that must be encrypted (rejects scalars).
    fn resolve_encrypted_operand(&mut self, operand: &FheEvalOperand) -> Result<ResolvedOperand> {
        match operand {
            FheEvalOperand::AllowedDurable {
                handle,
                acl_record_index,
                permission_index,
            } => self.resolve_durable_operand(*handle, *acl_record_index, *permission_index),
            FheEvalOperand::AllowedLocal { producer_index } => self
                .produced()
                .get(*producer_index as usize)
                .map(ResolvedOperand::from_produced)
                .ok_or_else(|| error!(ZamaHostError::FheEvalAllowedLocalMissing)),
            FheEvalOperand::VerifiedInput { attestation } => {
                self.resolve_verified_input_operand(attestation)
            }
            FheEvalOperand::Scalar(_) => Err(error!(ZamaHostError::InvalidFheEvalAccount)),
        }
    }

    /// Resolves a binary left-hand operand, which may not be a scalar.
    fn resolve_lhs_operand(&mut self, operand: &FheEvalOperand) -> Result<ResolvedOperand> {
        match operand {
            FheEvalOperand::Scalar(_) => Err(error!(ZamaHostError::InvalidFheEvalAccount)),
            _ => self.resolve_encrypted_operand(operand),
        }
    }

    /// Resolves a binary right-hand operand, which may be a scalar.
    fn resolve_rhs_operand(&mut self, operand: &FheEvalOperand) -> Result<ResolvedOperand> {
        match operand {
            FheEvalOperand::Scalar(value) => Ok(ResolvedOperand::scalar(*value)),
            _ => self.resolve_encrypted_operand(operand),
        }
    }
}

/// Drives a visitor over every plan step: resolve operands, assert operand
/// types, compute the produced handle, record its event, and accept the output.
pub(super) fn walk_eval_frame<'info, V: EvalStepVisitor>(
    visitor: &mut V,
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
    handle_context: &EvalHandleContext<'_>,
) -> Result<()> {
    // HCU metering: pure pass over the plan, enforcing the per-frame total + in-frame depth caps
    // against the canonical host_config limits (0 = unlimited). Runs in both the admission and
    // execution phases (both call this walk), so they compute and trip identically; a trip
    // in admission — which runs first — reverts before execution mutates any account.
    let host_config = &ctx.accounts.host_config;
    super::hcu::meter_eval_plan(
        &args.steps,
        host_config.max_hcu_per_tx,
        host_config.max_hcu_depth_per_tx,
    )?;

    let subject = visitor.subject();
    for (index, step) in args.steps.iter().enumerate() {
        let op_index = index as u16;
        match step {
            FheEvalStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                output,
            } => {
                let lhs = visitor.resolve_lhs_operand(lhs)?;
                let rhs = visitor.resolve_rhs_operand(rhs)?;
                let verified_input = combine_verified_input_binding(&[&lhs, &rhs])?;
                assert_binary_operand_types(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                )?;
                let result = expected_binary_eval_result(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                    handle_context,
                    op_index,
                    output,
                );
                visitor.record_op_event(EvalEvent::Binary(FheBinaryOpEvent {
                    version: EVENT_VERSION,
                    op: *op,
                    subject: subject.to_bytes(),
                    lhs: lhs.handle,
                    rhs: rhs.handle,
                    scalar: rhs.scalar,
                    result,
                }));
                visitor.accept_output(
                    ctx,
                    result,
                    output,
                    inputs_allow_public_decrypt(&lhs, &rhs),
                    true,
                    verified_input,
                )?;
            }
            FheEvalStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                output,
            } => {
                let control = visitor.resolve_encrypted_operand(control)?;
                let if_true = visitor.resolve_encrypted_operand(if_true)?;
                let if_false = visitor.resolve_encrypted_operand(if_false)?;
                let verified_input =
                    combine_verified_input_binding(&[&control, &if_true, &if_false])?;
                assert_ternary_operand_types(
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                )?;
                let result = expected_ternary_eval_result(
                    *op,
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                    handle_context,
                    op_index,
                    output,
                );
                visitor.record_op_event(EvalEvent::Ternary(FheTernaryOpEvent {
                    version: EVENT_VERSION,
                    op: *op,
                    subject: subject.to_bytes(),
                    control: control.handle,
                    if_true: if_true.handle,
                    if_false: if_false.handle,
                    result,
                }));
                visitor.accept_output(
                    ctx,
                    result,
                    output,
                    inputs3_allow_public_decrypt(&control, &if_true, &if_false),
                    true,
                    verified_input,
                )?;
            }
            FheEvalStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                output,
            } => {
                assert_supported_fhe_type(*fhe_type)?;
                let result = expected_trivial_eval_result(
                    *plaintext,
                    *fhe_type,
                    handle_context,
                    op_index,
                    output,
                );
                visitor.record_op_event(EvalEvent::Trivial(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    plaintext: *plaintext,
                    fhe_type: *fhe_type,
                    result,
                }));
                visitor.accept_output(ctx, result, output, false, false, None)?;
            }
            FheEvalStep::Rand { fhe_type, output } => {
                assert_supported_rand_type(*fhe_type)?;
                let seed = expected_rand_eval_seed(handle_context, op_index, output);
                let result = computed_rand_handle(seed, *fhe_type, handle_context.chain_id);
                visitor.record_op_event(EvalEvent::Rand(FheRandEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    seed,
                    fhe_type: *fhe_type,
                    result,
                }));
                visitor.accept_output(ctx, result, output, false, false, None)?;
            }
            FheEvalStep::Unary {
                op,
                operand,
                output_fhe_type,
                output,
            } => {
                let operand = visitor.resolve_encrypted_operand(operand)?;
                let verified_input = combine_verified_input_binding(&[&operand])?;
                assert_unary_operand_type(*op, operand.handle, *output_fhe_type)?;
                let result = expected_unary_eval_result(
                    *op,
                    operand.handle,
                    *output_fhe_type,
                    handle_context,
                    op_index,
                    output,
                );
                visitor.record_op_event(EvalEvent::Unary(FheUnaryOpEvent {
                    version: EVENT_VERSION,
                    op: *op,
                    subject: subject.to_bytes(),
                    operand: operand.handle,
                    result,
                }));
                visitor.accept_output(
                    ctx,
                    result,
                    output,
                    operand.public_decrypt_allowed,
                    true,
                    verified_input,
                )?;
            }
            FheEvalStep::RandBounded {
                upper_bound,
                fhe_type,
                output,
            } => {
                assert_supported_bounded_rand_type(*fhe_type)?;
                assert_valid_bounded_rand_upper_bound(*upper_bound, *fhe_type)?;
                let seed =
                    expected_rand_bounded_eval_seed(*upper_bound, handle_context, op_index, output);
                let result = computed_rand_bounded_handle(
                    *upper_bound,
                    seed,
                    *fhe_type,
                    handle_context.chain_id,
                );
                visitor.record_op_event(EvalEvent::RandBounded(FheRandBoundedEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    upper_bound: *upper_bound,
                    seed,
                    fhe_type: *fhe_type,
                    result,
                }));
                visitor.accept_output(ctx, result, output, false, false, None)?;
            }
            FheEvalStep::Sum {
                operands,
                fhe_type,
                output,
            } => {
                let mut resolved: Vec<ResolvedOperand> = Vec::with_capacity(operands.len());
                for operand in operands {
                    resolved.push(visitor.resolve_encrypted_operand(operand)?);
                }
                assert_sum_operand_types(operands, *fhe_type)?;
                let resolved_refs: Vec<&ResolvedOperand> = resolved.iter().collect();
                let verified_input = combine_verified_input_binding(resolved_refs.as_slice())?;
                let operand_handles: Vec<[u8; 32]> = resolved.iter().map(|r| r.handle).collect();
                let public_decrypt = resolved.iter().all(|r| r.public_decrypt_allowed);
                let result = expected_sum_eval_result(
                    &operand_handles,
                    *fhe_type,
                    handle_context,
                    op_index,
                    output,
                );
                visitor.record_op_event(EvalEvent::Sum(FheSumEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    operands: operand_handles,
                    fhe_type: *fhe_type,
                    result,
                }));
                visitor.accept_output(ctx, result, output, public_decrypt, true, verified_input)?;
            }
            FheEvalStep::IsIn {
                value,
                set,
                fhe_type,
                output,
            } => {
                let value_resolved = visitor.resolve_encrypted_operand(value)?;
                let mut set_resolved: Vec<ResolvedOperand> = Vec::with_capacity(set.len());
                for operand in set {
                    set_resolved.push(visitor.resolve_encrypted_operand(operand)?);
                }
                assert_is_in_operand_types(set, *fhe_type)?;
                let mut all_refs: Vec<&ResolvedOperand> =
                    Vec::with_capacity(1 + set_resolved.len());
                all_refs.push(&value_resolved);
                all_refs.extend(set_resolved.iter());
                let verified_input = combine_verified_input_binding(all_refs.as_slice())?;
                let set_handles: Vec<[u8; 32]> = set_resolved.iter().map(|r| r.handle).collect();
                let public_decrypt = value_resolved.public_decrypt_allowed
                    && set_resolved.iter().all(|r| r.public_decrypt_allowed);
                let result = expected_is_in_eval_result(
                    value_resolved.handle,
                    &set_handles,
                    *fhe_type,
                    handle_context,
                    op_index,
                    output,
                );
                visitor.record_op_event(EvalEvent::IsIn(FheIsInEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    value: value_resolved.handle,
                    set: set_handles,
                    fhe_type: *fhe_type,
                    result,
                }));
                visitor.accept_output(ctx, result, output, public_decrypt, true, verified_input)?;
            }
            FheEvalStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
                output,
            } => {
                let factor1 = visitor.resolve_lhs_operand(factor1)?;
                let factor2 = visitor.resolve_rhs_operand(factor2)?;
                let verified_input = combine_verified_input_binding(&[&factor1, &factor2])?;
                assert_mul_div_operand_types(*output_fhe_type)?;
                let result = expected_mul_div_eval_result(
                    factor1.handle,
                    factor2.handle,
                    factor2.scalar,
                    *divisor,
                    *output_fhe_type,
                    handle_context,
                    op_index,
                    output,
                );
                visitor.record_op_event(EvalEvent::MulDiv(FheMulDivEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    factor1: factor1.handle,
                    factor2: factor2.handle,
                    divisor: *divisor,
                    scalar: factor2.scalar,
                    result,
                }));
                visitor.accept_output(
                    ctx,
                    result,
                    output,
                    inputs_allow_public_decrypt(&factor1, &factor2),
                    true,
                    verified_input,
                )?;
            }
        }
    }
    Ok(())
}
