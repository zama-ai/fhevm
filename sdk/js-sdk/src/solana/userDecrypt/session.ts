// One permit, as many requests as it takes.
//
// The permit is signed once and the request is disposable: every attempt of the retry loop
// (`decryption/retry.ts`) submits the same bytes under the same signature. The wallet is not among
// this module's inputs at all — it takes a permit that is already signed, so no path through here
// can produce a second prompt.

import type { SolanaDecryptionClock, SolanaDecryptionOutcome } from '../decryption/retry.js';
import type { SolanaUserDecryptHandleEntry, SolanaUserDecryptRequestJson } from './request.js';
import type { SolanaSignedPermit } from '../permit/index.js';
import { retrySolanaDecryption } from '../decryption/retry.js';
import { buildSolanaUserDecryptRequest } from './request.js';

/**
 * Submitting one request and waiting for its outcome.
 *
 * HTTP and timeouts live behind this port, and so does the shape of the answer: what comes back is
 * handed to the response verification, which owns that type.
 */
export interface SolanaUserDecryptTransport<TResponse> {
  submit(request: SolanaUserDecryptRequestJson): Promise<SolanaDecryptionOutcome<TResponse>>;
}

/**
 * Runs one user decryption to an answer, or to a rejection it cannot repair.
 *
 * The request is built once, from the entries and the permit alone: a request that can never be
 * submitted — too many handles, over the bit budget, a foreign chain, a malformed identity field —
 * is refused before anything is spent on it.
 *
 * @param run.signedPermit - The permit and its one signature.
 * @param run.entries - The handles to decrypt, in the order they will be requested.
 * @param run.transport - Submits a request and waits for its outcome.
 * @param run.clock - Used for the backoff between attempts.
 * @param run.attempts - Attempt budget; the retry loop's default applies when absent.
 * @throws SolanaUserDecryptRequestError - When the request is refused before the network.
 * @throws SolanaDecryptionRunError - When the budget runs out, or the rejection cannot be repaired.
 */
export async function runSolanaUserDecrypt<TResponse>(run: {
  readonly signedPermit: SolanaSignedPermit;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
  readonly transport: SolanaUserDecryptTransport<TResponse>;
  readonly clock: SolanaDecryptionClock;
  readonly attempts?: number | undefined;
}): Promise<TResponse> {
  const body = buildSolanaUserDecryptRequest({ signedPermit: run.signedPermit, entries: run.entries });
  return retrySolanaDecryption({ submit: () => run.transport.submit(body), clock: run.clock, attempts: run.attempts });
}
