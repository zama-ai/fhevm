use super::*;

pub(super) fn eval_event_capacity(args: &FheEvalArgs) -> usize {
    args.steps
        .iter()
        .map(|step| 1 + durable_output_event_capacity(eval_step_output(step)))
        .sum()
}

fn eval_step_output(step: &FheEvalStep) -> &FheEvalOutput {
    match step {
        FheEvalStep::Binary { output, .. }
        | FheEvalStep::Ternary { output, .. }
        | FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. }
        | FheEvalStep::Unary { output, .. }
        | FheEvalStep::RandBounded { output, .. }
        | FheEvalStep::Sum { output, .. }
        | FheEvalStep::IsIn { output, .. }
        | FheEvalStep::MulDiv { output, .. } => output,
    }
}

fn durable_output_event_capacity(output: &FheEvalOutput) -> usize {
    match output {
        FheEvalOutput::AllowedDurable {
            output_subjects, ..
        } => 1 + output_subjects.len() * 2,
        FheEvalOutput::AllowedLocal => 0,
    }
}

// Anchor self-CPI events allocate one CPI frame per event. Large composed eval
// frames keep the same event payloads but use log transport to avoid heap OOM.
const MAX_CPI_EVAL_EVENTS: usize = 8;
const MAX_FHE_EVAL_EVENT_LOG_BYTES: usize = 8 * 1024;
const PROGRAM_DATA_LOG_PREFIX_BYTES: usize = "Program data: ".len();
const ANCHOR_EVENT_DISCRIMINATOR_BYTES: usize = 8;
const EVENT_VERSION_BYTES: usize = 1;
const EVENT_ENUM_BYTES: usize = 1;
const EVENT_BOOL_BYTES: usize = 1;
const EVENT_U8_BYTES: usize = 1;
const EVENT_U64_BYTES: usize = 8;
const EVENT_PUBKEY_BYTES: usize = 32;
const EVENT_HANDLE_BYTES: usize = 32;
const EVENT_SEED_BYTES: usize = 16;
const FHE_BINARY_OP_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_ENUM_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_BOOL_BYTES
    + EVENT_HANDLE_BYTES;
const FHE_TERNARY_OP_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_ENUM_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_HANDLE_BYTES;
const TRIVIAL_ENCRYPT_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_U8_BYTES
    + EVENT_HANDLE_BYTES;
const FHE_RAND_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_SEED_BYTES
    + EVENT_U8_BYTES
    + EVENT_HANDLE_BYTES;
const FHE_UNARY_OP_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_ENUM_BYTES   // op
    + EVENT_PUBKEY_BYTES // subject
    + EVENT_HANDLE_BYTES // operand
    + EVENT_HANDLE_BYTES; // result

const FHE_RAND_BOUNDED_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_PUBKEY_BYTES  // subject
    + EVENT_HANDLE_BYTES  // upper_bound
    + EVENT_SEED_BYTES    // seed
    + EVENT_U8_BYTES      // fhe_type
    + EVENT_HANDLE_BYTES; // result
fn fhe_sum_event_bytes(operand_count: usize) -> usize {
    EVENT_VERSION_BYTES
        + EVENT_PUBKEY_BYTES           // subject
        + 4 + operand_count * EVENT_HANDLE_BYTES // Vec<[u8;32]> (4-byte length prefix)
        + EVENT_U8_BYTES               // fhe_type
        + EVENT_HANDLE_BYTES           // result
}

fn fhe_is_in_event_bytes(set_count: usize) -> usize {
    EVENT_VERSION_BYTES
        + EVENT_PUBKEY_BYTES           // subject
        + EVENT_HANDLE_BYTES           // value
        + 4 + set_count * EVENT_HANDLE_BYTES // Vec<[u8;32]>
        + EVENT_U8_BYTES               // fhe_type
        + EVENT_HANDLE_BYTES           // result
}

const FHE_MUL_DIV_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_PUBKEY_BYTES  // subject
    + EVENT_HANDLE_BYTES  // factor1
    + EVENT_HANDLE_BYTES  // factor2
    + EVENT_HANDLE_BYTES  // divisor
    + EVENT_BOOL_BYTES    // scalar
    + EVENT_HANDLE_BYTES; // result

const ACL_RECORD_BOUND_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_U64_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_U8_BYTES
    + EVENT_BOOL_BYTES
    + EVENT_U64_BYTES;
const ACL_ALLOWED_EVENT_BYTES: usize =
    EVENT_VERSION_BYTES + EVENT_HANDLE_BYTES + EVENT_PUBKEY_BYTES;
const ACL_SUBJECT_ALLOWED_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_U8_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_U8_BYTES
    + EVENT_U64_BYTES;

pub(super) fn should_emit_eval_events_as_cpi(event_count: usize) -> bool {
    event_count <= MAX_CPI_EVAL_EVENTS
}

pub(super) fn assert_eval_log_budget(args: &FheEvalArgs) -> Result<()> {
    require!(
        estimated_eval_program_data_log_bytes(args) <= MAX_FHE_EVAL_EVENT_LOG_BYTES,
        ZamaHostError::FheEvalEventLogBudgetExceeded
    );
    Ok(())
}

fn estimated_eval_program_data_log_bytes(args: &FheEvalArgs) -> usize {
    let log_transport = !should_emit_eval_events_as_cpi(eval_event_capacity(args));
    args.steps
        .iter()
        .map(|step| eval_step_program_data_log_bytes(step, log_transport))
        .sum()
}

fn eval_step_program_data_log_bytes(step: &FheEvalStep, log_transport: bool) -> usize {
    let mut bytes = 0;
    if log_transport {
        bytes += anchor_program_data_log_bytes(eval_step_event_payload_bytes(step));
    }

    if let FheEvalOutput::AllowedDurable {
        output_subjects, ..
    } = eval_step_output(step)
    {
        bytes += anchor_program_data_log_bytes(ACL_RECORD_BOUND_EVENT_BYTES);
        bytes +=
            output_subjects.len() * anchor_program_data_log_bytes(ACL_SUBJECT_ALLOWED_EVENT_BYTES);
        if log_transport {
            bytes += output_subjects.len() * anchor_program_data_log_bytes(ACL_ALLOWED_EVENT_BYTES);
        }
    }

    bytes
}

fn eval_step_event_payload_bytes(step: &FheEvalStep) -> usize {
    match step {
        FheEvalStep::Binary { .. } => FHE_BINARY_OP_EVENT_BYTES,
        FheEvalStep::Ternary { .. } => FHE_TERNARY_OP_EVENT_BYTES,
        FheEvalStep::TrivialEncrypt { .. } => TRIVIAL_ENCRYPT_EVENT_BYTES,
        FheEvalStep::Rand { .. } => FHE_RAND_EVENT_BYTES,
        FheEvalStep::Unary { .. } => FHE_UNARY_OP_EVENT_BYTES,
        FheEvalStep::RandBounded { .. } => FHE_RAND_BOUNDED_EVENT_BYTES,
        FheEvalStep::Sum { operands, .. } => fhe_sum_event_bytes(operands.len()),
        FheEvalStep::IsIn { set, .. } => fhe_is_in_event_bytes(set.len()),
        FheEvalStep::MulDiv { .. } => FHE_MUL_DIV_EVENT_BYTES,
    }
}

fn anchor_program_data_log_bytes(payload_bytes: usize) -> usize {
    PROGRAM_DATA_LOG_PREFIX_BYTES
        + base64_encoded_len(ANCHOR_EVENT_DISCRIMINATOR_BYTES + payload_bytes)
}

fn base64_encoded_len(bytes: usize) -> usize {
    bytes.div_ceil(3) * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_event_capacity_counts_all_deferred_events() {
        let args = FheEvalArgs {
            context_id: [1; 32],
            steps: vec![
                FheEvalStep::TrivialEncrypt {
                    plaintext: [0; 32],
                    fhe_type: 1,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::TrivialEncrypt {
                    plaintext: [1; 32],
                    fhe_type: 1,
                    output: durable_output_with_subjects(3),
                },
            ],
        };

        assert_eq!(eval_event_capacity(&args), 1 + 1 + 1 + 3 * 2);
    }

    #[test]
    fn eval_event_transport_threshold_is_event_count_based() {
        assert!(should_emit_eval_events_as_cpi(MAX_CPI_EVAL_EVENTS));
        assert!(!should_emit_eval_events_as_cpi(MAX_CPI_EVAL_EVENTS + 1));
        assert_eq!(
            durable_output_event_capacity(&durable_output_with_subjects(MAX_ACL_SUBJECTS)),
            1 + MAX_ACL_SUBJECTS * 2
        );
        assert!(!should_emit_eval_events_as_cpi(
            MAX_FHE_EVAL_OPS * (1 + 1 + MAX_ACL_SUBJECTS * 2)
        ));
    }

    #[test]
    fn eval_event_log_budget_counts_rich_acl_logs_for_cpi_frames() {
        let args = FheEvalArgs {
            context_id: [1; 32],
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: [0; 32],
                fhe_type: 1,
                output: durable_output_with_subjects(1),
            }],
        };

        assert!(should_emit_eval_events_as_cpi(eval_event_capacity(&args)));
        assert_eq!(
            estimated_eval_program_data_log_bytes(&args),
            anchor_program_data_log_bytes(ACL_RECORD_BOUND_EVENT_BYTES)
                + anchor_program_data_log_bytes(ACL_SUBJECT_ALLOWED_EVENT_BYTES)
        );
        assert_eval_log_budget(&args).unwrap();
    }

    #[test]
    fn eval_event_log_budget_allows_threshold_replay_shape() {
        let args = FheEvalArgs {
            context_id: [1; 32],
            steps: vec![FheEvalStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheEvalOperand::AllowedDurable {
                    handle: [1; 32],
                    acl_record_index: 0,
                    permission_index: None,
                },
                rhs: FheEvalOperand::AllowedDurable {
                    handle: [2; 32],
                    acl_record_index: 1,
                    permission_index: None,
                },
                output_fhe_type: 5,
                output: durable_output_with_subjects(4),
            }],
        };

        assert!(!should_emit_eval_events_as_cpi(eval_event_capacity(&args)));
        assert!(estimated_eval_program_data_log_bytes(&args) < MAX_FHE_EVAL_EVENT_LOG_BYTES);
        assert_eval_log_budget(&args).unwrap();
    }

    #[test]
    fn eval_event_log_budget_rejects_worst_case_durable_frame() {
        let args = FheEvalArgs {
            context_id: [1; 32],
            steps: (0..MAX_FHE_EVAL_OPS)
                .map(|index| FheEvalStep::TrivialEncrypt {
                    plaintext: [index as u8; 32],
                    fhe_type: 1,
                    output: durable_output_with_subjects(MAX_ACL_SUBJECTS),
                })
                .collect(),
        };

        assert!(estimated_eval_program_data_log_bytes(&args) > MAX_FHE_EVAL_EVENT_LOG_BYTES);
        assert!(assert_eval_log_budget(&args).is_err());
    }

    fn durable_output_with_subjects(subject_count: usize) -> FheEvalOutput {
        FheEvalOutput::AllowedDurable {
            output_acl_record_index: 0,
            output_app_account_authority_index: None,
            output_nonce_key: [0; 32],
            output_nonce_sequence: 0,
            output_acl_domain_key: Pubkey::new_unique(),
            output_app_account: Pubkey::new_unique(),
            output_encrypted_value_label: [0; 32],
            output_subjects: (0..subject_count)
                .map(|_| AclSubjectEntry {
                    pubkey: Pubkey::new_unique(),
                    role_flags: ACL_ROLE_USE,
                })
                .collect(),
            output_public_decrypt: false,
        }
    }
}
