use super::*;

/// One compute event per step; the RFC-024 ACL lifecycle is event-free
/// (indexers reconstruct MMR leaves from instruction data).
pub(super) fn eval_event_capacity(args: &FheEvalArgs) -> usize {
    args.steps.len()
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
const FHE_RAND_BOUNDED_EVENT_BYTES: usize = EVENT_VERSION_BYTES
    + EVENT_PUBKEY_BYTES
    + EVENT_HANDLE_BYTES
    + EVENT_SEED_BYTES
    + EVENT_U8_BYTES
    + EVENT_HANDLE_BYTES;

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
    if log_transport {
        anchor_program_data_log_bytes(eval_step_event_payload_bytes(step))
    } else {
        0
    }
}

fn eval_step_event_payload_bytes(step: &FheEvalStep) -> usize {
    match step {
        FheEvalStep::Binary { .. } => FHE_BINARY_OP_EVENT_BYTES,
        FheEvalStep::Ternary { .. } => FHE_TERNARY_OP_EVENT_BYTES,
        FheEvalStep::TrivialEncrypt { .. } => TRIVIAL_ENCRYPT_EVENT_BYTES,
        FheEvalStep::Rand { .. } => FHE_RAND_EVENT_BYTES,
        FheEvalStep::RandBounded { .. } => FHE_RAND_BOUNDED_EVENT_BYTES,
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
    fn eval_event_capacity_is_one_compute_event_per_step() {
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

        assert_eq!(eval_event_capacity(&args), 2);
    }

    #[test]
    fn eval_event_transport_threshold_is_event_count_based() {
        assert!(should_emit_eval_events_as_cpi(MAX_CPI_EVAL_EVENTS));
        assert!(!should_emit_eval_events_as_cpi(MAX_CPI_EVAL_EVENTS + 1));
        assert!(!should_emit_eval_events_as_cpi(MAX_FHE_EVAL_OPS * 2));
    }

    #[test]
    fn eval_event_log_budget_is_zero_for_cpi_frames() {
        let args = FheEvalArgs {
            context_id: [1; 32],
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: [0; 32],
                fhe_type: 1,
                output: durable_output_with_subjects(1),
            }],
        };

        assert!(should_emit_eval_events_as_cpi(eval_event_capacity(&args)));
        // No ACL lifecycle events remain, so CPI-transported frames log nothing.
        assert_eq!(estimated_eval_program_data_log_bytes(&args), 0);
        assert_eval_log_budget(&args).unwrap();
    }

    #[test]
    fn eval_event_log_budget_allows_worst_case_durable_frame() {
        // The worst-case frame (max steps, max subjects) now fits the log
        // budget: only compute events remain and per-step payloads are small.
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

        assert!(!should_emit_eval_events_as_cpi(eval_event_capacity(&args)));
        assert!(estimated_eval_program_data_log_bytes(&args) < MAX_FHE_EVAL_EVENT_LOG_BYTES);
        assert_eval_log_budget(&args).unwrap();
    }

    fn durable_output_with_subjects(subject_count: usize) -> FheEvalOutput {
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index: 0,
            output_app_account_authority_index: None,
            output_acl_domain_key: Pubkey::new_unique(),
            output_app_account: Pubkey::new_unique(),
            output_encrypted_value_label: [0; 32],
            output_subjects: (0..subject_count)
                .map(|_| AclSubjectEntry {
                    pubkey: Pubkey::new_unique(),
                })
                .collect(),
            previous_handle: None,
            previous_subjects: None,
        }
    }
}
