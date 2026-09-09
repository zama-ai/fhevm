// One permit, as many requests as it takes.
//
// The permit is signed once and the request is disposable. This runner is where that asymmetry pays
// off: an overloaded relayer, a service that was briefly unreachable and a leaf the coprocessors had
// not yet indexed when the Connector asked are all answered by submitting the same bytes again under
// the same signature. The wallet is not among this module's inputs at all — it takes a permit that is
// already signed, so no path through here can produce a second prompt.
//
// It stops on its own. A bounded number of attempts means a client that cannot be authorized fails
// with the reason it last saw, rather than sitting in a loop that looks like a slow success. And it
// waits before every retry, with the wait doubling per attempt made: a bounded budget spent in a
// burst is a client hammering a service that needed time.

import type { SolanaUserDecryptRejection } from './failure.js';
import type { SolanaUserDecryptHandleEntry, SolanaUserDecryptRequestJson } from './request.js';
import type { SolanaSignedPermit } from '../permit/index.js';
import { classifySolanaUserDecryptRejection } from './failure.js';
import { buildSolanaUserDecryptRequest } from './request.js';

/** What the transport got back: an answer, or a reason there is none. */
export type SolanaUserDecryptTransportOutcome<TResponse> =
  | { readonly ok: true; readonly response: TResponse }
  | { readonly ok: false; readonly rejection: SolanaUserDecryptRejection };

/**
 * Submitting one request and waiting for its outcome.
 *
 * Polling, timeouts and HTTP live behind this port, and so does the shape of the answer: what comes
 * back is handed to the response verification, which owns that type. This runner only distinguishes
 * an answer from the absence of one.
 */
export interface SolanaUserDecryptTransport<TResponse> {
  submit(request: SolanaUserDecryptRequestJson): Promise<SolanaUserDecryptTransportOutcome<TResponse>>;
}

/** Waiting, as an injected capability, so a test does not have to spend the time. */
export interface SolanaUserDecryptClock {
  delay(seconds: number): Promise<void>;
}

/** How many submissions one call may make before it reports the last rejection. */
export const SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS = 4;

/** A run that ended without an answer. */
export class SolanaUserDecryptRunError extends Error {
  /** What the last attempt saw. */
  readonly rejection: SolanaUserDecryptRejection;
  /** How many submissions were made. */
  readonly attempts: number;

  constructor(rejection: SolanaUserDecryptRejection, attempts: number) {
    // A labeled rejection names its reason in the message: "failed" alone tells the reader
    // nothing, while the label (e.g. the relayer's `not_allowed_on_host_acl`) is the diagnosis.
    const cause =
      'label' in rejection
        ? `${rejection.kind} [${rejection.label}]${rejection.message === undefined ? '' : `: ${rejection.message}`}`
        : rejection.kind;
    super(`the user-decryption request was not answered after ${attempts} attempt(s): ${cause}`);
    this.name = 'SolanaUserDecryptRunError';
    this.rejection = rejection;
    this.attempts = attempts;
  }
}

/**
 * Runs one user decryption to an answer, or to a rejection it cannot repair.
 *
 * The request is built once, from the entries and the permit alone: a request that can never be
 * submitted — too many handles, over the bit budget, a foreign chain, a malformed identity field —
 * is refused before anything is spent on it. Each attempt then submits the same bytes. What happens
 * next is the classification's decision: submit again later, or stop. The permit's signature is
 * carried unchanged through all of it: no path here re-prompts the wallet.
 *
 * @param run.signedPermit - The permit and its one signature.
 * @param run.entries - The handles to decrypt, in the order they will be requested.
 * @param run.transport - Submits a request and waits for its outcome.
 * @param run.clock - Used for the backoff between attempts.
 * @param run.attempts - Attempt budget; defaults to {@link SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS}.
 * @throws SolanaUserDecryptRequestError - When the request is refused before the network.
 * @throws SolanaUserDecryptRunError - When the budget runs out, or the rejection cannot be repaired.
 */
export async function runSolanaUserDecrypt<TResponse>(run: {
  readonly signedPermit: SolanaSignedPermit;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
  readonly transport: SolanaUserDecryptTransport<TResponse>;
  readonly clock: SolanaUserDecryptClock;
  readonly attempts?: number;
}): Promise<{ readonly response: TResponse; readonly attempts: number }> {
  const body = buildSolanaUserDecryptRequest({ signedPermit: run.signedPermit, entries: run.entries });

  const budget = Math.max(1, run.attempts ?? SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS);
  let attempts = 0;

  for (;;) {
    attempts += 1;
    const outcome = await run.transport.submit(body);
    if (outcome.ok) {
      return { response: outcome.response, attempts };
    }

    // An unrepairable rejection and a spent budget end the same way: with the rejection last seen
    // and the count of what it cost to see it.
    const recovery = classifySolanaUserDecryptRejection(outcome.rejection);
    if (recovery.action === 'give-up' || attempts >= budget) {
      throw new SolanaUserDecryptRunError(outcome.rejection, attempts);
    }

    // The wait before the next submission doubles with each attempt already made: a fault that
    // survived a retry is not one immediate resubmission away.
    await run.clock.delay(recovery.afterSeconds * 2 ** (attempts - 1));
  }
}
