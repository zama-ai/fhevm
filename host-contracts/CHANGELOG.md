# Changelog

All notable changes to host-contracts will be documented in this file.

## [Unreleased] - Gateway V2 Migration (Phase 1B)

### Deleted V1 Contracts
- `contracts/KMSVerifier.sol` - Replaced by KMSVerifierV2

### Deleted V1 Tests
- `test/kmsVerifier/kmsVerifier.t.sol` - Replaced by kmsVerifierV2.t.sol

### Added V2 Contracts
- `contracts/KMSVerifierV2.sol` - New KMS verifier with epoch grace period support:
  - Grace period state machine for seamless key rotations
  - Dual epoch storage (current + previous signers/threshold)
  - `isValidSigner()` checks both epochs during grace period
  - `getEffectiveThreshold()` returns minimum during transition
  - Epoch ID tracking and grace period configuration
  - Backward-compatible `isSigner()` function

### Added V2 Tests
- `test/kmsVerifier/kmsVerifierV2.t.sol` - 18 tests covering:
  - Grace period behavior and state transitions
  - Signature validation during and after grace period
  - Epoch tracking and context switches
  - Access control for admin functions

### Updated Hardhat Tasks
- `tasks/taskDeploy.ts` - `task:deployKMSVerifier` now deploys KMSVerifierV2
- `tasks/taskUtils.ts` - `task:getKmsSigners` now uses KMSVerifierV2
- `tasks/upgradeContracts.ts` - `task:upgradeKMSVerifier` now expects KMSVerifierV2 artifacts

### Updated Test Utilities
- `fhevm-foundry/HostContractsDeployerTestUtils.sol` - `_deployKMSVerifier()` now deploys KMSVerifierV2
- `test/fhevm-foundry/TestHostContractsDeployerTestUtils.t.sol` - Updated to test KMSVerifierV2

### Build Verification
- `forge build` ✅
- `forge test` (294 tests passed) ✅
