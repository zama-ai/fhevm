export { deploy } from './deploy.js';
export { updateV13ToV14 } from './upgrade.js';
// Places the stack at CALLER-CHOSEN addresses on a dev node. Test harnesses need this: a contract under
// test compiles ZamaConfig's addresses into itself, so the stack must meet it there — and `deploy`, being
// CREATE-based, can only land on nonce-derived addresses.
export { deployAt } from './deployAt.js';
export { precomputeAddresses } from './addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL — the prerequisite for `updateV13ToV14`.
export { setupACLOwner } from './aclOwner.js';
// Emergency pause / unpause of the ACL, driven through the standing `ACLOwner` by its admin.
export { pauseACL, unpauseACL } from './aclOwner.js';
// Retire a past KMS context via the standing `ACLOwner`. (No rotation helper yet: v14 made defining a
// new context a multi-party ceremony — see the note in `kmsContext.ts`.)
export { destroyKmsContext } from './kmsContext.js';
// Package defaults and node-params builders. Unlike v13, consumers genuinely need these: `deployAt`
// takes a REQUIRED config, `updateV13ToV14`'s migration wants v14 `KmsNodeParams`, and until the
// rotation helper lands (see `kmsContext.ts`) a context switch is driven by hand from these blocks.
export {
  DEFAULT_BOOTSTRAP_CONFIG_V14,
  DEFAULT_KMS_SOFTWARE_VERSION,
  DEFAULT_KMS_THRESHOLDS,
  DEFAULT_PCR_VALUES,
  generateFromExistingDefaultKmsNodes,
  nextDefaultKmsSignerWindow,
} from './constants.js';

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
