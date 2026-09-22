// One permit session, from signed permit to verified plaintexts.
//
// The pieces are already built — the retry session, the response verification — and this module
// only fastens them together. Its one substantive rule is where the request inputs come from: the
// signed permit's own fields, including the KMS routing decoded out of the permit's extraData, plus
// the gateway domain from trust configuration. Configuration hands the routing in once, at permit
// creation; from then on the permit is the single source, and a verification that read the routing
// from configuration again could disagree with what the wallet actually signed. The domain is the
// exception by nature: the permit does not carry it, so it comes from the same trust configuration
// as the signer set and is bound into the link there.

import type { SolanaPermitFields, SolanaPermitWarning, SolanaSignedPermit } from '../permit/index.js';
import type {
  SolanaGatewayEip712Domain,
  SolanaKmsSigner,
  SolanaSigncryptedShare,
  SolanaTransportKeyPair,
  SolanaUserDecryptPlaintext,
  SolanaUserDecryptRequestInputs,
} from './response.js';
import type { SolanaUserDecryptHandleEntry } from './request.js';
import type { SolanaUserDecryptClock, SolanaUserDecryptTransport } from './session.js';
import { runSolanaUserDecrypt } from './session.js';
import { verifySolanaUserDecryptResponse } from './response.js';
import { encodeSolanaKmsRouting } from '../permit/index.js';

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
  /** The gateway domain the link is hashed under and node signatures verify against. */
  readonly gatewayEip712Domain: SolanaGatewayEip712Domain;
}

/**
 * The request inputs a permit's fields pin, for the given handles under the given gateway domain.
 *
 * Everything but the handles and the domain is the permit's own: the extra_data is the KMS routing
 * the wallet signed, re-encoded to the exact wire bytes the request carries, not read from
 * configuration — so the link this client computes and the link the KMS computes can only disagree
 * if the permit itself does, or if the configured domain is not the gateway's.
 *
 * @param fields - The signed permit's validated fields.
 * @param handles - The requested handles, in the order the request carries them.
 * @param gatewayEip712Domain - The gateway domain, from trust configuration.
 */
export function solanaUserDecryptRequestInputs(
  fields: SolanaPermitFields,
  handles: readonly Uint8Array[],
  gatewayEip712Domain: SolanaGatewayEip712Domain,
): SolanaUserDecryptRequestInputs {
  return {
    userPubkey: fields.userPubkey,
    hostChainId: fields.chainId,
    verifyingProgramId: fields.verifyingProgramId,
    handles,
    transportKey: fields.transportKey,
    gatewayEip712Domain,
    extraData: encodeSolanaKmsRouting(fields.kmsRouting),
  };
}

/**
 * Runs one user decryption end to end: the retry session to an answer, then its verification.
 *
 * @param run.session - The signed permit and its transport keypair.
 * @param run.entries - The handles to decrypt, in the order they will be requested.
 * @param run.transport - Submits a request and waits for its outcome.
 * @param run.clock - Used for the backoff between attempts.
 * @param run.attempts - Submission budget; the session's default applies when absent.
 * @param run.verification - The trust configuration verification runs under.
 * @throws SolanaUserDecryptRunError - When no attempt was answered.
 * @throws If the answer does not verify as this request's.
 */
export async function executeSolanaUserDecrypt(run: {
  readonly session: SolanaPermitSession;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
  readonly transport: SolanaUserDecryptTransport<readonly SolanaSigncryptedShare[]>;
  readonly clock: SolanaUserDecryptClock;
  readonly attempts?: number | undefined;
  readonly verification: SolanaUserDecryptVerification;
}): Promise<readonly SolanaUserDecryptPlaintext[]> {
  const { response: shares } = await runSolanaUserDecrypt({
    signedPermit: run.session.signedPermit,
    entries: run.entries,
    transport: run.transport,
    clock: run.clock,
    ...(run.attempts === undefined ? {} : { attempts: run.attempts }),
  });

  return verifySolanaUserDecryptResponse({
    request: solanaUserDecryptRequestInputs(
      run.session.signedPermit.fields,
      run.entries.map((entry) => entry.handle),
      run.verification.gatewayEip712Domain,
    ),
    shares,
    keyPair: run.session.keyPair,
    signers: run.verification.signers,
    fheParameter: run.verification.fheParameter,
  });
}
