//! The single walk over an `fhe_execute` execution: resolve operands, assert operand
//! types, derive the produced handle, and hand each output to
//! [`super::ExecutionState`], which validates and mutates in one pass.
//!
//! A step that fails mid-execution reverts the whole transaction — the Solana
//! runtime discards every account write on error — so validating while
//! mutating needs no separate validate-only pass to stay atomic.

use super::*;

/// Per-execution slot entropy and rand anchor shared by every handle derivation.
pub(super) struct ExecutionHandleContext {
    pub derivation: HandleDerivationContext,
    /// Present exactly when the execution has a rand step (see [`computed_eval_rand_seed`]).
    pub rand: Option<RandContext>,
}

/// What a rand seed is anchored to: the consumed host nonce and the application identity.
pub(super) struct RandContext {
    pub nonce: u64,
    pub app: AppScope,
}

// Persistent and instruction-local outputs derive the identical handle, and identical
// computations collide by design: deterministic handles are content-addressed
// (op/operands/type + slot entropy, no salt), matching EVM `FHEVMExecutor`. Only
// rand seeds carry uniqueness — the consumed host nonce.
impl ExecutionHandleContext {
    fn binary_result(
        &self,
        op: FheBinaryOpCode,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        output_fhe_type: u8,
        mask: [u8; 32],
    ) -> [u8; 32] {
        computed_eval_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            mask,
            &self.derivation,
        )
    }

    fn ternary_result(
        &self,
        op: FheTernaryOpCode,
        control: [u8; 32],
        if_true: [u8; 32],
        if_false: [u8; 32],
        output_fhe_type: u8,
        mask: [u8; 32],
    ) -> [u8; 32] {
        computed_eval_ternary_handle(
            op,
            control,
            if_true,
            if_false,
            output_fhe_type,
            mask,
            &self.derivation,
        )
    }

    fn trivial_result(&self, plaintext: [u8; 32], fhe_type: u8) -> [u8; 32] {
        computed_eval_trivial_handle(plaintext, fhe_type, &self.derivation)
    }

    /// `fhe_execute` refuses a rand step without the nonce account before the walk starts, so
    /// the anchor is present whenever a rand step asks for its seed.
    pub(super) fn rand_seed(&self, op_index: u16) -> Result<[u8; 16]> {
        let rand = self
            .rand
            .as_ref()
            .ok_or(ZamaHostError::FheExecuteRandNonceMissing)?;
        Ok(computed_eval_rand_seed(
            rand.nonce,
            rand.app,
            op_index,
            &self.derivation,
        ))
    }

    fn unary_result(
        &self,
        op: FheUnaryOpCode,
        operand: [u8; 32],
        output_fhe_type: u8,
        mask: [u8; 32],
    ) -> [u8; 32] {
        computed_eval_unary_handle(op, operand, output_fhe_type, mask, &self.derivation)
    }

    fn mul_div_result(
        &self,
        factor1: [u8; 32],
        factor2: [u8; 32],
        scalar: bool,
        divisor: [u8; 32],
        output_fhe_type: u8,
        mask: [u8; 32],
    ) -> [u8; 32] {
        computed_eval_mul_div_handle(
            factor1,
            factor2,
            divisor,
            scalar,
            output_fhe_type,
            mask,
            &self.derivation,
        )
    }
}

/// Operand resolvers shared by every step shape. Defined here so the
/// match-on-step skeleton and the operand rules read together; the
/// account-access and mutation halves live with the state in [`super`].
impl ExecutionState<'_, '_, '_> {
    /// Resolves an operand that must be encrypted (rejects scalars).
    fn resolve_encrypted_operand(
        &mut self,
        operand: &FheExecuteOperand,
    ) -> Result<ResolvedOperand> {
        match operand {
            FheExecuteOperand::StateSlot {
                handle_index,
                state_index,
                key_index,
            } => {
                let handle = self.dictionary_bytes(*handle_index)?;
                let key = self.dictionary_bytes(*key_index)?;
                assert_handle_for_chain(handle, self.chain_id)?;
                require!(
                    self.table.state((*state_index).into())?.get(&key) == Some(handle),
                    ZamaHostError::PreviousStateMismatch
                );
                Ok(self.encrypted_operand(handle))
            }
            FheExecuteOperand::TransientResult {
                handle_index,
                consumer_state_index,
            } => {
                let handle = self.dictionary_bytes(*handle_index)?;
                assert_handle_for_chain(handle, self.chain_id)?;
                let consumer = self.table.account((*consumer_state_index).into())?.key();
                let depth = self
                    .transient_store
                    .authorized_depth(handle, consumer)
                    .ok_or(ZamaHostError::TransientAccountInvalid)?;
                Ok(ResolvedOperand {
                    handle,
                    scalar: false,
                    boundary: false,
                    depth,
                })
            }

            FheExecuteOperand::EarlierStep { producer_index } => {
                let result = self
                    .transient_store
                    .result(self.call_start + usize::from(*producer_index))
                    .ok_or(ZamaHostError::FheExecuteEarlierStepMissing)?;
                Ok(self.encrypted_operand(result.handle))
            }
            FheExecuteOperand::VerifiedInput { attestation } => {
                // EVM `fromExternal` parity: only the attested contract may consume the input.
                // Enforced here (the `msg.sender` analog) — not by constraining derived outputs.
                // The contract is the execution's application program, proven through the
                // persistent values it reads or writes; a copied attestation is useless to
                // anyone who cannot sign for that program's values.
                let program = self.app.program;
                require_keys_eq!(
                    Pubkey::new_from_array(attestation.contract_address),
                    program,
                    ZamaHostError::InputBindContractMismatch
                );
                self.resolve_verified_input_operand(attestation)
            }
            FheExecuteOperand::Scalar { .. } => {
                Err(error!(ZamaHostError::InvalidFheExecuteAccount))
            }
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

/// Drives the execution state over every execution step: resolve operands, assert
/// operand types, compute the produced handle, and accept the output.
pub(super) fn walk_steps<'info>(
    execution: &mut ExecutionState<'_, '_, 'info>,
    args: &FheExecuteArgs,
    handle_context: &ExecutionHandleContext,
) -> Result<()> {
    for (index, step) in args.steps.iter().enumerate() {
        let op_index = index as u16;
        match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
            } => {
                let lhs = execution.resolve_encrypted_operand(lhs)?;
                let rhs = execution.resolve_rhs_operand(rhs)?;
                assert_binary_operand_types(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                )?;
                let operands = [lhs, rhs];
                let result = handle_context.binary_result(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                    boundary_mask(&operands)?,
                );
                let pricing_type = if hcu::is_comparison(*op) {
                    handle_fhe_type(lhs.handle)
                } else {
                    *output_fhe_type
                };
                execution.accept_output(
                    result,
                    hcu::binary_op_hcu(*op, pricing_type, rhs.scalar)?,
                    &operands,
                )?;
            }
            FheExecuteStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
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
                let operands = [control, if_true, if_false];
                let result = handle_context.ternary_result(
                    *op,
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                    boundary_mask(&operands)?,
                );
                execution.accept_output(
                    result,
                    hcu::ternary_op_hcu(*op, *output_fhe_type)?,
                    &operands,
                )?;
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
            } => {
                assert_supported_fhe_type(*fhe_type)?;
                let result = handle_context.trivial_result(*plaintext, *fhe_type);
                execution.accept_output(result, hcu::trivial_encrypt_hcu(*fhe_type)?, &[])?;
            }
            FheExecuteStep::Rand { fhe_type } => {
                assert_supported_fhe_type(*fhe_type)?;
                let seed = handle_context.rand_seed(op_index)?;
                let result =
                    computed_rand_handle(seed, *fhe_type, handle_context.derivation.chain_id);
                execution.accept_output(result, hcu::rand_hcu(*fhe_type)?, &[])?;
            }
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
            } => {
                let operand = execution.resolve_encrypted_operand(operand)?;
                assert_unary_operand_type(*op, operand.handle, *output_fhe_type)?;
                let operands = [operand];
                let result = handle_context.unary_result(
                    *op,
                    operand.handle,
                    *output_fhe_type,
                    boundary_mask(&operands)?,
                );
                execution.accept_output(
                    result,
                    hcu::unary_op_hcu(*op, *output_fhe_type)?,
                    &operands,
                )?;
            }
            FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type,
            } => {
                assert_valid_bounded_rand_upper_bound(*upper_bound, *fhe_type)?;
                let seed = handle_context.rand_seed(op_index)?;
                let result = computed_rand_bounded_handle(
                    *upper_bound,
                    seed,
                    *fhe_type,
                    handle_context.derivation.chain_id,
                );
                execution.accept_output(result, hcu::rand_bounded_hcu(*fhe_type)?, &[])?;
            }
            FheExecuteStep::Sum { operands, fhe_type } => {
                assert_reduction_count(operands.len(), *fhe_type)?;
                let mut resolved: Vec<ResolvedOperand> = Vec::with_capacity(operands.len());
                for operand in operands {
                    resolved.push(execution.resolve_encrypted_operand(operand)?);
                }
                let operand_handles = resolved.iter().map(|r| &r.handle);
                assert_sum_operand_types(operand_handles.clone(), *fhe_type)?;
                let result = computed_eval_sum_handle(
                    operand_handles,
                    *fhe_type,
                    boundary_mask(&resolved)?,
                    &handle_context.derivation,
                );
                execution.accept_output(
                    result,
                    hcu::sum_hcu(*fhe_type, operands.len())?,
                    &resolved,
                )?;
            }
            FheExecuteStep::IsIn {
                value,
                set,
                fhe_type,
            } => {
                assert_reduction_count(set.len(), *fhe_type)?;
                let value_resolved = execution.resolve_encrypted_operand(value)?;
                let mut set_resolved: Vec<ResolvedOperand> = Vec::with_capacity(1 + set.len());
                set_resolved.push(value_resolved);
                for operand in set {
                    set_resolved.push(execution.resolve_encrypted_operand(operand)?);
                }
                let set_handles = set_resolved[1..].iter().map(|r| &r.handle);
                assert_is_in_operand_types(value_resolved.handle, set_handles.clone(), *fhe_type)?;
                let result = computed_eval_is_in_handle(
                    value_resolved.handle,
                    set_handles,
                    *fhe_type,
                    boundary_mask(&set_resolved)?,
                    &handle_context.derivation,
                );
                execution.accept_output(
                    result,
                    hcu::is_in_hcu(*fhe_type, set.len())?,
                    &set_resolved,
                )?;
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
            } => {
                let factor1 = execution.resolve_encrypted_operand(factor1)?;
                let factor2 = execution.resolve_rhs_operand(factor2)?;
                assert_mul_div_operand_types(
                    factor1.handle,
                    factor2.handle,
                    factor2.scalar,
                    *divisor,
                    *output_fhe_type,
                )?;
                let operands = [factor1, factor2];
                let result = handle_context.mul_div_result(
                    factor1.handle,
                    factor2.handle,
                    factor2.scalar,
                    *divisor,
                    *output_fhe_type,
                    boundary_mask(&operands)?,
                );
                execution.accept_output(
                    result,
                    hcu::mul_div_hcu(*output_fhe_type, factor2.scalar)?,
                    &operands,
                )?;
            }
        }
    }
    Ok(())
}
