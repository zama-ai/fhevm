export { deploy } from './deploy.js';
export { updateV13ToV14 } from './upgrade.js';
// Places the stack at CALLER-CHOSEN addresses on a dev node. Test harnesses need this: a contract under
// test compiles ZamaConfig's addresses into itself, so the stack must meet it there — and `deploy`, being
// CREATE-based, can only land on nonce-derived addresses.
export { deployAt } from './deployAt.js';
export { precomputeAddresses } from './addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL — the prerequisite for `updateV13ToV14`.
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
  FhevmAddressesV14,
  FixedAddressesV14,
  CleartextAddresses,
  // `deploy` config + result.
  BootstrapConfigV14,
  DeployedV14,
  // `updateV13ToV14` migration values.
  UpdateV13ToV14MigrationConfig,
  // Per-contract bootstrap init configs.
  ProtocolConfigInitConfig,
  InputVerifierInitConfig,
  KMSVerifierInitConfig,
  HCULimitInitConfig,
  // Shared on-chain structs.
  KmsNodeParams,
  PcrValues,
  KmsThresholds,
} from './types/public.js';
