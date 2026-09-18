export type { SolanaRuntimeConfig } from './internal/config.js';
export { setFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './internal/config.js';

export { createFhevmBaseClient } from './clients/createFhevmBaseClient.js';
export { createFhevmDecryptClient } from './clients/createFhevmDecryptClient.js';
export type { FhevmSolanaDecryptClient, SolanaDecryptValueParameters } from './clients/createFhevmDecryptClient.js';
export type {
  SolanaDecryptTrust,
  SolanaPermitDecryptActions,
  SolanaSignPermitParameters,
  SolanaUserDecryptEntry,
  SolanaUserDecryptParameters,
} from './clients/decorators/permitDecrypt.js';

export {
  solanaPermitWalletFromSecretKey,
  SolanaPermitError,
  SolanaPermitChannelError,
  SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE,
} from './permit/index.js';
export type { SolanaPermitWallet, SolanaSignedPermit, SolanaPermitWarning } from './permit/index.js';
export type { SolanaPermitSession } from './userDecrypt/execute.js';
export type { SolanaKmsSigner, SolanaGatewayEip712Domain } from './userDecrypt/response.js';
export { SolanaUserDecryptRunError } from './userDecrypt/session.js';
export { SolanaUserDecryptRequestError } from './userDecrypt/request.js';
export type {
  FhevmSolanaBaseClient,
  SolanaClientParameters,
  SolanaEncryptOptions,
} from './clients/createFhevmBaseClient.js';
export type { SolanaDecryptPublicValueParameters } from './actions/decryptPublicValue.js';

export {
  SOLANA_ENCRYPTED_STORE_SEED,
  decodeSolanaEncryptedStore,
  encryptedStoreHandle,
  fetchSolanaEncryptedStore,
  solanaEncryptedStoreAddress,
} from './encryptedStore.js';
export type { SolanaEncryptedStoreSeeds, SolanaEncryptedStore, SolanaRpc } from './encryptedStore.js';
export { createFhevmPublicDecryptClient } from './clients/createFhevmPublicDecryptClient.js';
export type { FhevmSolanaPublicDecryptClient } from './clients/createFhevmPublicDecryptClient.js';
export { createFhevmEncryptClient } from './clients/createFhevmEncryptClient.js';
export type { FhevmSolanaEncryptClient } from './clients/createFhevmEncryptClient.js';
export { clearSolanaEncryptionKeyCache } from './encryptionKeyCache.js';

export {
  buildPublicLeafProof,
  mmrBuildProof,
  mmrPeaksFromLeaves,
  reconstructSolanaStoreHistory,
  verifyHistoricalAccessProof,
  verifyPublicDecryptProof,
} from './proof.js';
export type { MmrProof, SolanaStoreHistoryEvent, SolanaReconstructedStoreHistory } from './proof.js';

export { buildSolanaPublicDecryptExtraData } from './actions/publicDecryptCertificate.js';
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
  SOLANA_WILDCARD_AUTHORITY,
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

export { createSolanaFheTransaction } from './fheTransaction.js';
export { solanaHostProgram } from './clients/createFhevmBaseClient.js';
export type { SolanaFheTransaction, SolanaFheTransactionAccounts } from './fheTransaction.js';

export { toSolanaZkProof } from '../core/coprocessor/SolanaZkProof-p.js';

export { assertHandleArrayEquals, bytes32HexToHandle } from '../core/handle/FhevmHandle.js';

export type { SolanaInputProof, SolanaEncryptValuesResult } from './clients/decorators/encrypt.js';

export type { SolanaEncryptValueParameters } from './clients/createFhevmEncryptClient.js';
