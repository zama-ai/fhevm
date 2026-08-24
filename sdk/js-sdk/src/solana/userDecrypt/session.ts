// One permit, as many requests as it takes.
//
// The permit is signed once and the request is disposable. This runner is where that asymmetry pays
// off: a stale proof, a handle an update has replaced, an overloaded relayer and a service that was briefly
// unreachable are all answered by building another request from the same signature. The wallet is not
// among this module's inputs at all — it takes a permit that is already signed, so no path through
// here can produce a second prompt.
//
// It stops on its own. A bounded number of attempts means a client that cannot be authorized fails
// with the reason it last saw, rather than sitting in a loop that looks like a slow success. And it
// waits before every repair, with the wait doubling per attempt made: a bounded budget spent in a
// burst is a client hammering a service that needed time.

import type { SolanaAccessEvidenceSource, SolanaHandleRequest } from './evidence.js';
import type { SolanaUserDecryptRejection } from './failure.js';
import type { SolanaUserDecryptRequestJson } from './request.js';
import type { SolanaSignedPermit } from '../permit/index.js';
import { SolanaAccessEvidenceIntegrityError, resolveSolanaAccessEvidence } from './evidence.js';
import { SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS, classifySolanaUserDecryptRejection } from './failure.js';
import {
  SolanaUserDecryptRequestError,
  admitSolanaUserDecryptRequest,
  buildSolanaUserDecryptRequest,
} from './request.js';

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
  /** How many attempts were spent — submissions, plus evidence resolutions that failed. */
  readonly attempts: number;

  constructor(rejection: SolanaUserDecryptRejection, attempts: number) {
    super(`the user-decryption request was not answered after ${attempts} attempt(s): ${rejection.kind}`);
    this.name = 'SolanaUserDecryptRunError';
    this.rejection = rejection;
    this.attempts = attempts;
  }
}

/**
 * Runs one user decryption to an answer, or to a rejection it cannot repair.
 *
 * The request is admitted first, from the handles and the permit alone: a request that can never be
 * submitted — too many handles, over the bit budget, a foreign chain, a malformed identity field —
 * is refused before any evidence is fetched for it.
 *
 * Each attempt then resolves the evidence, builds the request and submits it. What happens next is
 * the classification's decision: resolve again (the evidence moved — a rebuilt proof, or a handle
 * now reachable only as historical), submit the same bytes, or stop. A resolution that fails
 * transiently — an RPC read fault, a proof-service fault, the account-read/proof snapshot race — is
 * an attempt like any other: it spends from the same budget and waits the same backoff. What is not
 * retried is evidence wrong on its face — a malformed or refuted proof would come back the same on
 * every retry. Either repair waits first, and the wait doubles with each attempt made. The permit's
 * signature is carried unchanged through all of it: no path here re-prompts the wallet.
 *
 * @param run.signedPermit - The permit and its one signature.
 * @param run.requests - The handles to decrypt, in the order they will be requested.
 * @param run.evidence - Where per-handle evidence comes from; consulted once per attempt.
 * @param run.transport - Submits a request and waits for its outcome.
 * @param run.clock - Used for the backoff between attempts.
 * @param run.attempts - Attempt budget; defaults to {@link SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS}.
 * @throws SolanaUserDecryptRequestError - When the request is refused before the network.
 * @throws SolanaUserDecryptRunError - When the budget runs out, or the rejection cannot be repaired.
 * @throws The last resolution fault, when the budget runs out before evidence ever resolved.
 */
export async function runSolanaUserDecrypt<TResponse>(run: {
  readonly signedPermit: SolanaSignedPermit;
  readonly requests: readonly SolanaHandleRequest[];
  readonly evidence: SolanaAccessEvidenceSource;
  readonly transport: SolanaUserDecryptTransport<TResponse>;
  readonly clock: SolanaUserDecryptClock;
  readonly attempts?: number;
}): Promise<{ readonly response: TResponse; readonly attempts: number }> {
  admitSolanaUserDecryptRequest({ chainId: run.signedPermit.fields.chainId, requests: run.requests });

  const budget = Math.max(1, run.attempts ?? SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS);
  let attempts = 0;

  // The body in flight. Cleared by resolve-again — the next attempt rebuilds it from fresh
  // evidence — and kept by retry-unchanged, so "the same bytes" is the same object, not a rebuild
  // that happens to come out equal.
  let body: SolanaUserDecryptRequestJson | undefined;

  for (;;) {
    if (body === undefined) {
      try {
        const entries = await resolveSolanaAccessEvidence(run.evidence, run.requests);
        body = buildSolanaUserDecryptRequest({ signedPermit: run.signedPermit, entries });
      } catch (error) {
        // Evidence wrong on its face is not a moment to wait out: the builder refusing a proof and
        // a source answering with malformed bytes would both come back the same on every retry.
        if (error instanceof SolanaUserDecryptRequestError || error instanceof SolanaAccessEvidenceIntegrityError) {
          throw error;
        }
        // Everything else is the evidence services meeting a bad moment — an RPC fault, a
        // proof-service fault, the account-read/proof snapshot race — and is retried on the same
        // budget and backoff as a failed submission. No wallet is involved: the signature is
        // already in hand, so another resolution costs no prompt.
        attempts += 1;
        if (attempts >= budget) {
          throw error;
        }
        await run.clock.delay(SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS * 2 ** (attempts - 1));
        continue;
      }
    }

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
    // survived a retry is not one immediate resubmission away. The resolve-again branch waits too —
    // fresh evidence comes from the same services whose fault is being waited out, and an instant
    // re-resolution is the burst the backoff exists to prevent.
    const growth = 2 ** (attempts - 1);
    switch (recovery.action) {
      case 'retry-unchanged':
        await run.clock.delay(recovery.afterSeconds * growth);
        break;
      case 'resolve-again':
        await run.clock.delay(SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS * growth);
        body = undefined;
        break;
    }
  }
}
