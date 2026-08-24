// One permit session, from signed permit to verified plaintexts.
//
// The pieces are already built — the retry session, the response verification — and this module
// only fastens them together. Its one substantive rule is where the link inputs come from: the
// signed permit's own fields, including the KMS routing decoded out of the permit's extraData.
// Configuration hands the routing in once, at permit creation; from then on the permit is the
// single source, and a verification that read the routing from configuration again could disagree
// with what the wallet actually signed.

import type { SolanaAccessEvidenceSource, SolanaHandleRequest } from './evidence.js';
import type { SolanaPermitFields, SolanaPermitWarning, SolanaSignedPermit } from '../permit/index.js';
import type {
  SolanaGatewayEip712Domain,
  SolanaKmsSigner,
  SolanaSigncryptedShare,
  SolanaTransportKeyPair,
  SolanaUserDecryptLinkInputs,
  SolanaUserDecryptPlaintext,
} from './response.js';
import type { SolanaUserDecryptClock, SolanaUserDecryptTransport } from './session.js';
import { runSolanaUserDecrypt } from './session.js';
import { verifySolanaUserDecryptResponse } from './response.js';

/**
 * One signed permit and everything created alongside it. The reusable object of the whole path:
 * every request of the session cites its one signature, and its transport keypair is what the
 * responses are de-signcrypted under.
 */
export interface SolanaPermitSession {
  readonly signedPermit: SolanaSignedPermit;
  /** The ML-KEM pair the permit commits to. The secret key never leaves the client. */
  readonly keyPair: SolanaTransportKeyPair;
  /** Advisory findings from permit creation; nothing here blocks signing. */
  readonly warnings: readonly SolanaPermitWarning[];
}

/** What response verification must be told beyond the permit: the trust configuration. */
export interface SolanaUserDecryptVerification {
  readonly signers: readonly SolanaKmsSigner[];
  readonly fheParameter: string;
  readonly gatewayEip712Domain?: SolanaGatewayEip712Domain | undefined;
}

/**
 * The link inputs a permit's fields pin, for the given handles.
 *
 * Everything but the handles is the permit's own: the KMS routing ids come from the extraData the
 * wallet signed, not from configuration — so the link this client computes and the link the KMS
 * computes can only disagree if the permit itself does.
 *
 * @param fields - The signed permit's validated fields.
 * @param handles - The requested handles, in the order the request carries them.
 */
export function solanaUserDecryptLinkInputs(
  fields: SolanaPermitFields,
  handles: readonly Uint8Array[],
): SolanaUserDecryptLinkInputs {
  return {
    userPubkey: fields.userPubkey,
    hostChainId: fields.chainId,
    verifyingProgramId: fields.verifyingProgramId,
    kmsContextId: fields.kmsRouting.kmsContextId,
    kmsEpochId: fields.kmsRouting.kmsEpochId,
    handles,
    transportKey: fields.transportKey,
  };
}

/**
 * Runs one user decryption end to end: the retry session to an answer, then its verification.
 *
 * @param run.session - The signed permit and its transport keypair.
 * @param run.requests - The handles to decrypt, in the order they will be requested.
 * @param run.evidence - Where per-handle evidence comes from.
 * @param run.transport - Submits a request and waits for its outcome.
 * @param run.clock - Used for the backoff between attempts.
 * @param run.attempts - Submission budget; the session's default applies when absent.
 * @param run.verification - The trust configuration verification runs under.
 * @throws SolanaUserDecryptRunError - When no attempt was answered.
 * @throws If the answer does not verify as this request's.
 */
export async function executeSolanaUserDecrypt(run: {
  readonly session: SolanaPermitSession;
  readonly requests: readonly SolanaHandleRequest[];
  readonly evidence: SolanaAccessEvidenceSource;
  readonly transport: SolanaUserDecryptTransport<readonly SolanaSigncryptedShare[]>;
  readonly clock: SolanaUserDecryptClock;
  readonly attempts?: number | undefined;
  readonly verification: SolanaUserDecryptVerification;
}): Promise<readonly SolanaUserDecryptPlaintext[]> {
  const { response: shares } = await runSolanaUserDecrypt({
    signedPermit: run.session.signedPermit,
    requests: run.requests,
    evidence: run.evidence,
    transport: run.transport,
    clock: run.clock,
    ...(run.attempts === undefined ? {} : { attempts: run.attempts }),
  });

  return verifySolanaUserDecryptResponse({
    link: solanaUserDecryptLinkInputs(
      run.session.signedPermit.fields,
      run.requests.map((request) => request.handle),
    ),
    shares,
    keyPair: run.session.keyPair,
    signers: run.verification.signers,
    fheParameter: run.verification.fheParameter,
    gatewayEip712Domain: run.verification.gatewayEip712Domain,
  });
}
