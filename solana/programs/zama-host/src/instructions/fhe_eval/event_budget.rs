use super::*;

/// One compute event per step; the RFC-024 ACL lifecycle is event-free
/// (indexers reconstruct MMR leaves from instruction data).
pub(super) fn eval_event_capacity(args: &FheEvalArgs) -> usize {
    args.steps.len()
}

// Anchor self-CPI events allocate one CPI frame per event on the 32KiB bump heap.
// Frames with more events than this carry no on-chain event at all (there is no
// log fallback — the only event consumer, the relayer proof builder, never reads
// logs, and the indexer reconstructs from instruction data via Yellowstone gRPC).
const MAX_CPI_EVAL_EVENTS: usize = 8;

pub(super) fn should_emit_eval_events_as_cpi(event_count: usize) -> bool {
    event_count <= MAX_CPI_EVAL_EVENTS
}

/// A born-public durable output derives a block-entropy handle that lives in no
/// instruction argument, so the only way an off-chain proof builder can recover
/// it is the step's `emit_cpi!` event. A frame too large for CPI transport
/// carries no event, which would strand that lineage's public-decrypt proof — so
/// reject a born-public output in such a frame at write time (fail-closed).
/// Non-born-public durable outputs are unaffected: their handles reconstruct from
/// the `update_encrypted_value` arguments, needing no event.
pub(super) fn assert_born_public_frame_transportable(args: &FheEvalArgs) -> Result<()> {
    let has_born_public = args.steps.iter().any(step_output_is_born_public);
    require!(
        !has_born_public || should_emit_eval_events_as_cpi(eval_event_capacity(args)),
        ZamaHostError::FheEvalBornPublicFrameTooLarge
    );
    Ok(())
}

fn step_output_is_born_public(step: &FheEvalStep) -> bool {
    matches!(
        step_output(step),
        FheEvalOutput::AllowedDurable {
            make_public: true,
            ..
        }
    )
}

fn step_output(step: &FheEvalStep) -> &FheEvalOutput {
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
                    output: durable_output(3, false),
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
    fn born_public_output_within_cpi_cap_is_allowed() {
        let args = born_public_frame(MAX_CPI_EVAL_EVENTS);
        assert!(should_emit_eval_events_as_cpi(eval_event_capacity(&args)));
        assert_born_public_frame_transportable(&args).unwrap();
    }

    #[test]
    fn born_public_output_over_cpi_cap_is_rejected() {
        let args = born_public_frame(MAX_CPI_EVAL_EVENTS + 1);
        assert!(!should_emit_eval_events_as_cpi(eval_event_capacity(&args)));
        assert!(assert_born_public_frame_transportable(&args).is_err());
    }

    #[test]
    fn large_frame_without_born_public_is_allowed() {
        // A max-size frame is fine as long as no output is born public: regular
        // durable-output handles reconstruct from `update_encrypted_value` args.
        let args = FheEvalArgs {
            context_id: [1; 32],
            steps: (0..MAX_FHE_EVAL_OPS)
                .map(|index| FheEvalStep::TrivialEncrypt {
                    plaintext: [index as u8; 32],
                    fhe_type: 1,
                    output: durable_output(MAX_ACL_SUBJECTS, false),
                })
                .collect(),
        };

        assert!(!should_emit_eval_events_as_cpi(eval_event_capacity(&args)));
        assert_born_public_frame_transportable(&args).unwrap();
    }

    fn born_public_frame(step_count: usize) -> FheEvalArgs {
        FheEvalArgs {
            context_id: [1; 32],
            steps: (0..step_count)
                .map(|index| FheEvalStep::TrivialEncrypt {
                    plaintext: [index as u8; 32],
                    fhe_type: 1,
                    // Only the last step is born public; the guard must still fire
                    // on frame size, not on the born-public step's position.
                    output: durable_output(1, index + 1 == step_count),
                })
                .collect(),
        }
    }

    fn durable_output(subject_count: usize, make_public: bool) -> FheEvalOutput {
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
            make_public,
        }
    }
}
