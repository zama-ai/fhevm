export { deploy } from './deploy.js';
export { updateV12ToV13 } from './upgrade.js';
export { precomputeAddresses } from './addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL — the prerequisite for `updateV12ToV13`.
export { setupACLOwner } from './aclOwner.js';
// Emergency pause / unpause of the ACL, driven through the standing `ACLOwner` by its admin.
export { pauseACL, unpauseACL } from './aclOwner.js';
// Rotate / retire KMS contexts via the standing `ACLOwner`.
export { defineNewKmsContext, destroyKmsContext } from './kmsContext.js';
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
  // Address sets. `FhevmAddresses` is this package's own; the suffixed one is the PREVIOUS generation,
  // and exists only because `updateV12ToV13` takes it as input.
  FhevmAddresses,
  CleartextAddresses,
  FhevmAddressesV12,
  // `deploy` config + shared result of `deploy` / `updateV12ToV13`.
  BootstrapConfig,
  Deployed,
  // Per-contract bootstrap init configs.
  ProtocolConfigInitConfig,
  InputVerifierInitConfig,
  KMSVerifierInitConfig,
  HCULimitInitConfig,
  // Shared on-chain structs.
  KmsNode,
  KmsThresholds,
  // `updateV12ToV13` migration config.
  UpdateV12ToV13MigrationConfig,
} from './types/public.js';
