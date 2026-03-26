// This selectors are useful for revert that should be retried. Due to node provider load balancers,
// we could have error due to node inconsistent state during gas estimation which we want to retry (readiness check has passed on some state).
// It could nonetheless introduce some latency due to multiple retries if this never pass, and goes to a failure mode with termination.
// Should not be included in max_retry_exceeded metrics.

/// CiphertextMaterialNotFound(bytes32) - from gateway contracts CiphertextCommits.sol
pub const SELECTOR_CIPHERTEXT_MATERIAL_NOT_READY: &str = "0x0666cbdf";

/// PublicDecryptNotAllowed(bytes32) - for ACL readiness check on public decrypt revert.
pub const SELECTOR_PUBLIC_DECRYPT_NOT_ALLOWED: &str = "0x4331a85d";

/// AccountNotAllowedToUseCiphertext(bytes32,address) - for ACL readiness check on user decrypt revert.
pub const SELECTOR_ACCOUNT_NOT_ALLOWED_TO_USE_CIPHERTEXT: &str = "0x160a2b4b";
