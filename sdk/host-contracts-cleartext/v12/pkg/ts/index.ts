export { deploy } from './deploy.js';
export { precomputeAddresses } from './addresses.js';
// The deterministic-deployment counterpart: addresses that depend on nothing but the factory, the salt
// inputs and the init code — no deployer nonce, so they survive any transaction that moves it.
export { precomputeCreate2Addresses, CREATE2_FACTORY, CREATE2_ROLES } from './create2Addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL. Exposed because the next generation's
// update flow requires a live stack's ACL to already be owned by an `ACLOwner`.
export { setupACLOwner } from './aclOwner.js';
// Emergency pause / unpause of the ACL, driven through the standing `ACLOwner` by its admin.
export { pauseACL, unpauseACL } from './aclOwner.js';
// The `getVersion()` string every contract in this generation reports. Generated from the contracts
// themselves (internal/generateContractVersions.ts), so it cannot drift from what a deployed stack
// answers — which is what makes it usable as the expected value when verifying a deployment.
export { CONTRACT_VERSIONS } from './versions.js';
// Full-integrity verification of a deployed or upgraded stack. `snapshotStack` is taken BEFORE an
// upgrade; `verify` compares against it, so "nothing else changed" is checkable rather than assumed.
export { verify, snapshotStack, DEFAULT_MAY_CHANGE } from './verify.js';

export type {
  // Abstract adapter interfaces (consumers implement these over their web3 lib).
  AbstractEthereumProvider,
  AbstractEthereumUtils,
  AbstractEthereumSigner,
  // Parameter / return shapes referenced by the adapter interfaces above.
  DeployParameters,
  DeployReturnType,
  EncodeCallParameters,
  // Address sets.
  FhevmAddresses,
  CleartextAddresses,
  // `deploy` config + its result.
  BootstrapConfig,
  Deployed,
  // Per-contract bootstrap init configs.
  InputVerifierInitConfig,
  KMSVerifierInitConfig,
  HCULimitInitConfig,
  // `verify` / `snapshotStack`.
  AbstractEthereumHistory,
  PartialStack,
  SnapshotParameters,
  StackSnapshot,
  VerifyCheck,
  VerifyExpectations,
  VerifyParameters,
  VerifyReport,
  // `precomputeCreate2Addresses`.
  Create2Parameters,
  Create2Addresses,
} from './types/public.js';
