// Assembling a Solana user-decryption request from a signed permit.

export {
  MAX_SOLANA_USER_DECRYPT_HANDLES,
  SOLANA_SRFC38_ATTESTATION_TYPE,
  SolanaUserDecryptRequestError,
  admitSolanaUserDecryptRequest,
  buildSolanaUserDecryptRequest,
  solanaUserDecryptRequestBits,
} from './request.js';
export type {
  SolanaUserDecryptHandleEntry,
  SolanaUserDecryptHandleJson,
  SolanaUserDecryptPayloadJson,
  SolanaUserDecryptRequestFailure,
  SolanaUserDecryptRequestJson,
} from './request.js';

export { executeSolanaUserDecrypt, solanaUserDecryptLinkInputs } from './execute.js';
export type { SolanaPermitSession, SolanaUserDecryptVerification } from './execute.js';

export {
  SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS,
  SOLANA_USER_DECRYPT_LABEL_ACTIONS,
  classifySolanaUserDecryptRejection,
} from './failure.js';
export type { SolanaUserDecryptRecovery, SolanaUserDecryptRejection } from './failure.js';

export {
  generateSolanaTransportKeyPair,
  solanaUserDecryptLink,
  solanaUserDecryptRequestHalf,
  verifySolanaUserDecryptPlaintexts,
  verifySolanaUserDecryptResponse,
} from './response.js';
export type {
  SolanaGatewayEip712Domain,
  SolanaKmsSigner,
  SolanaSigncryptedShare,
  SolanaTransportKeyPair,
  SolanaUserDecryptLinkInputs,
  SolanaUserDecryptPlaintext,
} from './response.js';

export { createSolanaUserDecryptRelayerTransport } from './relayerTransport.js';

export { SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS, SolanaUserDecryptRunError, runSolanaUserDecrypt } from './session.js';
export type {
  SolanaUserDecryptClock,
  SolanaUserDecryptTransport,
  SolanaUserDecryptTransportOutcome,
} from './session.js';
