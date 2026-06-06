use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn expected_binary_eval_result(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output: &FheEvalOutput,
) -> [u8; 32] {
    match output {
        FheEvalOutput::Transient | FheEvalOutput::TransientSession { .. } => computed_eval_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            chain_id,
            previous_bank_hash,
            unix_timestamp,
            context_id,
            op_index,
        ),
        FheEvalOutput::Durable {
            output_nonce_key,
            output_nonce_sequence,
            ..
        } => computed_bound_eval_handle(
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            chain_id,
            previous_bank_hash,
            unix_timestamp,
            context_id,
            op_index,
            *output_nonce_key,
            *output_nonce_sequence,
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
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output: &FheEvalOutput,
) -> [u8; 32] {
    match output {
        FheEvalOutput::Transient | FheEvalOutput::TransientSession { .. } => {
            computed_eval_ternary_handle(
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                chain_id,
                previous_bank_hash,
                unix_timestamp,
                context_id,
                op_index,
            )
        }
        FheEvalOutput::Durable {
            output_nonce_key,
            output_nonce_sequence,
            ..
        } => computed_bound_eval_ternary_handle(
            op,
            control,
            if_true,
            if_false,
            output_fhe_type,
            chain_id,
            previous_bank_hash,
            unix_timestamp,
            context_id,
            op_index,
            *output_nonce_key,
            *output_nonce_sequence,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn expected_trivial_eval_result(
    plaintext: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output: &FheEvalOutput,
) -> [u8; 32] {
    match output {
        FheEvalOutput::Transient | FheEvalOutput::TransientSession { .. } => {
            computed_eval_trivial_handle(
                plaintext,
                fhe_type,
                chain_id,
                previous_bank_hash,
                unix_timestamp,
                context_id,
                op_index,
            )
        }
        FheEvalOutput::Durable {
            output_nonce_key,
            output_nonce_sequence,
            ..
        } => computed_trivial_handle(
            plaintext,
            fhe_type,
            chain_id,
            previous_bank_hash,
            unix_timestamp,
            *output_nonce_key,
            *output_nonce_sequence,
        ),
    }
}

pub(super) fn expected_rand_eval_seed(
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output: &FheEvalOutput,
) -> [u8; 16] {
    match output {
        FheEvalOutput::Transient | FheEvalOutput::TransientSession { .. } => {
            computed_eval_rand_seed(
                chain_id,
                previous_bank_hash,
                unix_timestamp,
                context_id,
                op_index,
            )
        }
        FheEvalOutput::Durable {
            output_nonce_key,
            output_nonce_sequence,
            ..
        } => computed_rand_seed(
            chain_id,
            previous_bank_hash,
            unix_timestamp,
            *output_nonce_key,
            *output_nonce_sequence,
        ),
    }
}
