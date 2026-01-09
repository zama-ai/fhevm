# Changelog

All notable changes to gateway-contracts will be documented in this file.

## [Unreleased] - Gateway V2 Migration (Phase 1A)

### Deleted V1 Contracts
- `contracts/InputVerification.sol`
- `contracts/Decryption.sol`
- `contracts/CiphertextCommits.sol`
- `contracts/MultichainACL.sol`

### Deleted V1 Interfaces
- `contracts/interfaces/IInputVerification.sol`
- `contracts/interfaces/IDecryption.sol`
- `contracts/interfaces/ICiphertextCommits.sol`
- `contracts/interfaces/IMultichainACL.sol`

### Deleted V1 Mocks
- `contracts/mocks/InputVerificationMock.sol`
- `contracts/mocks/DecryptionMock.sol`
- `contracts/mocks/CiphertextCommitsMock.sol`
- `contracts/mocks/MultichainACLMock.sol`

### Deleted V1 Example Contracts
- `contracts/examples/InputVerificationV2Example.sol`
- `contracts/examples/DecryptionV2Example.sol`
- `contracts/examples/CiphertextCommitsV2Example.sol`
- `contracts/examples/MultichainACLV2Example.sol`

### Deleted Shared Utilities
- `contracts/shared/MultichainACLChecks.sol`

### Added V2 Contracts
- `contracts/DecryptionRegistry.sol` - New contract for decryption request registration (no response handling)
- `contracts/interfaces/IDecryptionRegistry.sol` - Interface for DecryptionRegistry

### Modified Contracts
- `contracts/shared/Structs.sol` - Added `apiUrl` field to `KmsNode` and `Coprocessor` structs
- `contracts/GatewayConfig.sol`:
  - Removed imports of deleted V1 contracts
  - Updated `pauseAllGatewayContracts()` and `unpauseAllGatewayContracts()` (no longer call V1 contracts)
  - Added `getKmsNodesWithApis()` function
  - Added `getCoprocessorsWithApis()` function
- `contracts/interfaces/IGatewayConfig.sol` - Added new getter function signatures
- `contracts/shared/KMSRequestCounters.sol` - Updated comment reference to DecryptionRegistry

### Updated Hardhat Tasks
- `tasks/pauseContracts.ts` - Removed V1 pause/unpause tasks (pauseInputVerification, pauseDecryption, etc.)
- `tasks/upgradeContracts.ts` - Removed V1 upgrade tasks (upgradeMultichainACL, upgradeCiphertextCommits, upgradeDecryption, upgradeInputVerification)
- `tasks/blockExplorerVerify.ts` - Removed V1 verification tasks
- `tasks/deployment/contracts.ts` - Removed V1 deployment tasks (deployInputVerification, deployCiphertextCommits, deployMultichainACL, deployDecryption)
- `tasks/deployment/empty_proxies.ts` - Removed V1 proxy deployment

### Deleted Test Files
- `test/CiphertextCommits.ts`
- `test/Decryption.ts`
- `test/InputVerification.ts`
- `test/MultichainACL.ts`
- `test/tasks/pausing.ts`

### Updated Test Files
- `test/mocks/mocks.ts` - Removed V1 mock tests, kept GatewayConfigMock and KMSGenerationMock tests
- `test/GatewayConfig.ts` - Removed V1 contract references, updated pause tests
- `test/utils/contracts.ts` - Removed V1 contract loading

### Regenerated Rust Bindings
- Deleted V1 binding files (decryption.rs, ciphertext_commits.rs, etc.)
- Updated remaining bindings with new struct fields

### Build Verification
- `forge build` ✅
- `cargo build` (rust_bindings) ✅
- `npx tsc --noEmit` ✅
