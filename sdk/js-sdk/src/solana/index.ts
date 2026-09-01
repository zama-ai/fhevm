export { setFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './internal/config.js';

export { createFhevmBaseClient } from './clients/createFhevmBaseClient.js';
export { createFhevmDecryptClient } from './clients/createFhevmDecryptClient.js';
export type { FhevmSolanaDecryptClient, FhevmSolanaPermitDecryptClient } from './clients/createFhevmDecryptClient.js';
export type {
  SolanaDecryptTrust,
  SolanaPermitDecryptActions,
  SolanaSignPermitParameters,
  SolanaUserDecryptEntry,
  SolanaUserDecryptParameters,
} from './clients/decorators/permitDecrypt.js';

// The permit and user-decrypt modules are curated surfaces of their own; they travel whole.
export * from './permit/index.js';
export * from './userDecrypt/index.js';

export {
  SOLANA_ENCRYPTED_VALUE_SEED,
  decodeSolanaEncryptedValueState,
  fetchSolanaEncryptedValueState,
  solanaEncryptedValueAccountAddress,
} from './encryptedValueAccount.js';
export type { SolanaEncryptedValueState, SolanaRpc } from './encryptedValueAccount.js';
export { createFhevmPublicDecryptClient } from './clients/createFhevmPublicDecryptClient.js';
export type { FhevmSolanaPublicDecryptClient } from './clients/createFhevmPublicDecryptClient.js';
export { createFhevmEncryptClient } from './clients/createFhevmEncryptClient.js';
export type { FhevmSolanaEncryptClient } from './clients/createFhevmEncryptClient.js';
export { clearSolanaEncryptionKeyCache } from './encryptionKeyCache.js';

export {
  bytesToHex as solanaProofBytesToHex,
  hexToBytes as solanaProofHexToBytes,
  verifyPublicDecryptProof,
} from './proof.js';
export type { MmrProof } from './proof.js';

export { buildSolanaPublicDecryptMmrProofExtraData } from './actions/publicDecryptCertificate.js';
export type {
  SolanaPublicDecryptCertificateClaim,
  SolanaPublicDecryptCertificateParameters,
} from './actions/publicDecryptCertificate.js';
export {
  buildVerifyPublicDecryptInstruction,
  verifyPublicDecryptArgsFromClaim,
} from './actions/verifyPublicDecrypt.js';
export type {
  SolanaVerifyPublicDecryptAccounts,
  SolanaVerifyPublicDecryptArgs,
} from './actions/verifyPublicDecrypt.js';
export {
  SOLANA_USER_DECRYPTION_DELEGATION_SEED,
  SOLANA_WILDCARD_AUTHORITY_WARNING,
  SOLANA_WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
  buildDelegateForUserDecryptionInstruction,
  buildRevokeDelegationForUserDecryptionInstruction,
  decodeSolanaUserDecryptionDelegation,
  fetchSolanaUserDecryptionDelegation,
  isSolanaUserDecryptionDelegationLiveAt,
  solanaDelegationWarnings,
  solanaUserDecryptionDelegationAddress,
} from './actions/userDecryptionDelegation.js';
export type {
  SolanaDelegateForUserDecryptionParameters,
  SolanaDelegationWarning,
  SolanaRevokeDelegationForUserDecryptionParameters,
  SolanaUserDecryptionDelegationRecord,
  SolanaSignerOrAddress,
  SolanaUserDecryptionDelegationRows,
  SolanaUserDecryptionDelegationTuple,
  SolanaZamaHostAddressConfig,
} from './actions/userDecryptionDelegation.js';
export {
  SOLANA_PERMIT_INVALIDATION_SEED,
  buildRevokePermitsInstruction,
  solanaPermitInvalidationAddress,
} from './actions/revokePermits.js';
export type { SolanaDecryptActions } from './clients/decorators/decrypt.js';
export type { SolanaPublicDecryptActions } from './clients/decorators/publicDecrypt.js';

export type {
  SolanaEncryptInputParameters,
  SolanaEncryptInputResult,
  SolanaEncryptInputValue,
} from './actions/encryptInput.js';
export type { SolanaSubmitInputProofParameters, SolanaSubmitInputProofResult } from './actions/submitInputProof.js';
export type { SolanaEncryptActions } from './clients/decorators/encrypt.js';
export type { SolanaZkProof, SolanaZkProofLike } from '../core/types/zkProof-p.js';

export type { FhevmSolanaChain } from '../core/types/fhevmSolanaChain.js';
export { defineFhevmSolanaChain } from '../core/chains/utilsSolana.js';
