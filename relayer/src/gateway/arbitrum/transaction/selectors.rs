// This selector is useful for reverts that should be retried. Due to node provider load balancers,
// we could have errors due to node inconsistent state during gas estimation which we want to retry
// (readiness check has passed on some state). It could nonetheless introduce some latency due to
// multiple retries if this never passes, and goes to a failure mode with termination.
// Should not be included in max_retry_exceeded metrics.

/// CiphertextMaterialNotFound(bytes32) - from gateway contracts CiphertextCommits.sol.
/// Retried during gas estimation because it represents a transient node inconsistency.
pub const SELECTOR_CIPHERTEXT_MATERIAL_NOT_READY: &str = "0x0666cbdf";
