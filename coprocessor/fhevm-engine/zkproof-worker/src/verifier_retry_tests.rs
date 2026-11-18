use super::{is_retryable_initial_verification_error, ExecutionError};

#[test]
fn retry_policy_distinguishes_transient_rerandomization_from_invalid_proofs() {
    assert!(is_retryable_initial_verification_error(
        &ExecutionError::TransientVerificationFailure("missing rerandomization key".into())
    ));
    assert!(!is_retryable_initial_verification_error(
        &ExecutionError::InvalidProof(7, "invalid proof".into())
    ));
}
