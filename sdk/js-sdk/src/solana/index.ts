export { setFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './internal/config.js';

export { createFhevmBaseClient } from './clients/createFhevmBaseClient.js';
export { createFhevmDecryptClient } from './clients/createFhevmDecryptClient.js';
export type { FhevmSolanaDecryptClient } from './clients/createFhevmDecryptClient.js';
export { createFhevmPublicDecryptClient } from './clients/createFhevmPublicDecryptClient.js';
export type { FhevmSolanaPublicDecryptClient } from './clients/createFhevmPublicDecryptClient.js';
export { createFhevmEncryptClient } from './clients/createFhevmEncryptClient.js';
export type { FhevmSolanaEncryptClient } from './clients/createFhevmEncryptClient.js';

export { solanaSignerFromSecretKey } from './signer.js';
export type { SolanaUserDecryptSigner } from './signer.js';
export { buildSolanaUserDecryptMmrProofExtraData } from '../core/coprocessor/SolanaUserDecrypt-p.js';
export {
  bytesToHex as solanaProofBytesToHex,
  hexToBytes as solanaProofHexToBytes,
  verifyPublicDecryptProof,
} from './proof.js';
export type { MmrProof } from './proof.js';

export type { SolanaUserDecryptParameters, SolanaUserDecryptResult } from './actions/userDecrypt.js';
export type {
  SolanaPublicDecryptCertificateClaim,
  SolanaPublicDecryptCertificateParameters,
} from './actions/publicDecryptCertificate.js';
export type { SolanaDecryptActions } from './clients/decorators/decrypt.js';
export type { SolanaPublicDecryptActions } from './clients/decorators/publicDecrypt.js';

export type {
  SolanaEncryptInputParameters,
  SolanaEncryptInputResult,
  SolanaEncryptInputValue,
} from './actions/encryptInput.js';
export type { SolanaSubmitInputProofParameters, SolanaSubmitInputProofResult } from './actions/submitInputProof.js';
export type { SolanaConfidentialTransferParameters } from './actions/confidentialTransfer.js';
export type { SolanaEncryptActions } from './clients/decorators/encrypt.js';
export type { SolanaZkProof, SolanaZkProofLike } from '../core/types/zkProof-p.js';

export type { FhevmSolanaChain } from '../core/types/fhevmSolanaChain.js';
export { defineFhevmSolanaChain } from '../core/chains/utilsSolana.js';
