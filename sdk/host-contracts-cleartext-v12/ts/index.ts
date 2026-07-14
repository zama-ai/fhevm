export { deploy } from './deploy.js';
export { precomputeAddresses } from './addresses.js';
// Installs a standing `ACLOwner` over an EOA-owned ACL. Exposed because the v12→v13 upgrade flow
// (in the v13 package) requires the live v12 stack's ACL to be owned by an `ACLOwner`.
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
  CleartextAddresses,
  // `deploy` config + result.
  BootstrapConfigV12,
  DeployedV12,
  // Per-contract bootstrap init configs.
  EIP712VerifierInitConfig,
  HCULimitInitConfig,
} from './types/public.js';
