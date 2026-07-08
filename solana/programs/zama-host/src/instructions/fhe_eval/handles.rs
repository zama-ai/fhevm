use super::*;

pub(super) struct EvalHandleContext<'a> {
    pub chain_id: u64,
    pub previous_bank_hash: &'a [u8; 32],
    pub unix_timestamp: i64,
    pub context_id: &'a [u8; 32],
}

#[allow(clippy::too_many_arguments)]
pub(super) fn expected_binary_eval_result(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    handle_context: &EvalHandleContext<'_>,
    op_index: u16,
    binding: Option<&OutputBinding>,
) -> [u8; 32] {
    match binding {
        None => computed_eval_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
        ),
        Some(binding) => computed_bound_eval_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
            binding.value_key,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn expected_ternary_eval_result(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    output_fhe_type: u8,
    handle_context: &EvalHandleContext<'_>,
    op_index: u16,
    binding: Option<&OutputBinding>,
) -> [u8; 32] {
    match binding {
        None => computed_eval_ternary_handle(
            op,
            control,
            if_true,
            if_false,
            output_fhe_type,
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
        ),
        Some(binding) => computed_bound_eval_ternary_handle(
            op,
            control,
            if_true,
            if_false,
            output_fhe_type,
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
            binding.value_key,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn expected_trivial_eval_result(
    plaintext: [u8; 32],
    fhe_type: u8,
    handle_context: &EvalHandleContext<'_>,
    op_index: u16,
    binding: Option<&OutputBinding>,
) -> [u8; 32] {
    match binding {
        None => computed_eval_trivial_handle(
            plaintext,
            fhe_type,
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
        ),
        Some(binding) => computed_bound_eval_trivial_handle(
            plaintext,
            fhe_type,
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
            binding.value_key,
        ),
    }
}

pub(super) fn expected_rand_eval_seed(
    handle_context: &EvalHandleContext<'_>,
    op_index: u16,
    binding: Option<&OutputBinding>,
) -> [u8; 16] {
    match binding {
        None => computed_eval_rand_seed(
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
        ),
        Some(binding) => computed_bound_eval_rand_seed(
            handle_context.chain_id,
            *handle_context.previous_bank_hash,
            handle_context.unix_timestamp,
            *handle_context.context_id,
            op_index,
            binding.value_key,
        ),
    }
}
