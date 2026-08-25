export { deploy } from './deploy.js';
export { precomputeAddresses } from './addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL. Exposed because the next generation's
// update flow requires a live stack's ACL to already be owned by an `ACLOwner`.
export { setupACLOwner } from './aclOwner.js';
// Emergency pause / unpause of the ACL, driven through the standing `ACLOwner` by its admin.
export { pauseACL, unpauseACL } from './aclOwner.js';
// The `getVersion()` string every contract in this generation reports. Generated from the contracts
// themselves (internal/generateContractVersions.ts), so it cannot drift from what a deployed stack
// answers — which is what makes it usable as the expected value when verifying a deployment.
export { CONTRACT_VERSIONS } from './versions.js';

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
} from './types/public.js';
