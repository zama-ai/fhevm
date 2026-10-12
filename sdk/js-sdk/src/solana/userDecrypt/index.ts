// Assembling a Solana user-decryption request from a signed permit.

export {
  MAX_SOLANA_DECRYPT_HANDLES,
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

export { executeSolanaUserDecrypt, solanaUserDecryptRequestInputs } from './execute.js';
export type { SolanaPermitSession, SolanaUserDecryptVerification } from './execute.js';

export {
  generateSolanaTransportKeyPair,
  verifySolanaUserDecryptPlaintexts,
  verifySolanaUserDecryptResponse,
} from './response.js';
export type {
  SolanaGatewayEip712Domain,
  SolanaKmsSigner,
  SolanaSigncryptedShare,
  SolanaTransportKeyPair,
  SolanaUserDecryptPlaintext,
  SolanaUserDecryptRequestInputs,
} from './response.js';
