# Signer Governance Scenario

Focus: signer-set and threshold governance state machines for `InputVerifier` and `KMSVerifier`.
Signer candidate-domain assumptions are centralized in each handler (`candidateDomainSize`/`candidateAt`) and consumed by shared checks.

## Actions
- `defineNewContext`
- `setThreshold`
- `verifyInput` / `verifyDecryptionEIP712KMSSignatures`
- `cleanTransientStorage` (`InputVerifier` scenario)

## Model state
- Shared signer candidate domain exported by handlers.
- Violation flags: unexpected context mutation, unauthorized governance success.

## Invariants (IDs)
- `SG-INPUT-001`: threshold always in `[1, signers.length]`.
- `SG-INPUT-002`: `InputVerifier` signer array and `isSigner` mapping stay consistent both ways.
- `SG-INPUT-003`: verification/cleanup flows do not mutate signer governance state.
- `SG-INPUT-004`: only ACL owner can mutate signer governance (`defineNewContext`, `setThreshold`).
- `SG-KMS-001`: threshold always in `[1, signers.length]`.
- `SG-KMS-002`: `KMSVerifier` signer array and `isSigner` mapping stay consistent both ways.
- `SG-KMS-003`: decryption verification flow does not mutate signer governance state.
- `SG-KMS-004`: only ACL owner can mutate signer governance (`defineNewContext`, `setThreshold`).

## Known non-goals
- No cryptographic correctness checks for signature/proof contents.
