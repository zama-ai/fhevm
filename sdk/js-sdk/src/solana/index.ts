export { setFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './internal/config.js';

export { createFhevmBaseClient } from './clients/createFhevmBaseClient.js';
export { createFhevmDecryptClient } from './clients/createFhevmDecryptClient.js';
export type { FhevmSolanaDecryptClient } from './clients/createFhevmDecryptClient.js';

export { solanaSignerFromSecretKey } from './signer.js';
export type { SolanaUserDecryptSigner } from './signer.js';

export type {
  SolanaUserDecryptParameters,
  SolanaUserDecryptResult,
  SolanaUserDecryptShare,
} from './actions/userDecrypt.js';
export type { SolanaDecryptActions } from './clients/decorators/decrypt.js';

export type { FhevmSolanaChain } from '../core/types/fhevmSolanaChain.js';
export { defineFhevmSolanaChain } from '../core/chains/utilsSolana.js';
