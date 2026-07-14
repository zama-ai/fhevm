export { deploy } from './deploy.js';
export { precomputeAddresses } from './addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL. Exposed because a future vN→v14 upgrade flow
// requires the live stack's ACL to already be owned by an `ACLOwner`.
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
  CleartextAddresses,
  // `deploy` config + result.
  BootstrapConfigV14,
  DeployedV14,
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
