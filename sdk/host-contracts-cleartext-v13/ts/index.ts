export { deploy } from './deploy.js';
export { updateV12ToV13 } from './upgrade.js';
export { precomputeAddresses } from './addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL — the prerequisite for `updateV12ToV13`.
export { setupACLOwner } from './aclOwner.js';

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
  FhevmAddressesV12,
  FhevmAddressesV13,
  CleartextAddresses,
  // `deploy` config + shared result of `deploy` / `updateV12ToV13`.
  BootstrapConfigV13,
  DeployedV13,
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
