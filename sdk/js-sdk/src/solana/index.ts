export { setFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './internal/config.js';

export { createFhevmBaseClient } from './clients/createFhevmBaseClient.js';
export { createFhevmDecryptClient } from './clients/createFhevmDecryptClient.js';
export type { FhevmSolanaDecryptClient } from './clients/createFhevmDecryptClient.js';
export { createFhevmEncryptClient } from './clients/createFhevmEncryptClient.js';
export type { FhevmSolanaEncryptClient } from './clients/createFhevmEncryptClient.js';

export { solanaSignerFromSecretKey } from './signer.js';
export type { SolanaUserDecryptSigner } from './signer.js';

export type { SolanaUserDecryptParameters, SolanaUserDecryptResult } from './actions/userDecrypt.js';
export type { SolanaDecryptActions } from './clients/decorators/decrypt.js';

export { fetchSolanaDecryptProof } from './proof.js';
export type { SolanaDecryptProof } from './proof.js';

export type {
  SolanaEncryptInputParameters,
  SolanaEncryptInputResult,
  SolanaEncryptInputValue,
} from './actions/encryptInput.js';
export type { SolanaEncryptActions } from './clients/decorators/encrypt.js';
export type { SolanaZkProof, SolanaZkProofLike } from '../core/types/zkProof-p.js';

export type { FhevmSolanaChain } from '../core/types/fhevmSolanaChain.js';
export { defineFhevmSolanaChain } from '../core/chains/utilsSolana.js';
