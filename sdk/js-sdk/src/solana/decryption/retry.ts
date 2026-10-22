// One request, as many submissions as it takes.
//
// A decryption request is disposable and its authorization is not: a user decryption cites the one
// signature the wallet gave, and a public decryption needs none. So a rate-limited relayer, a
// service that was briefly unreachable and a leaf the coprocessors had not yet indexed when the
// Connector asked are all answered by submitting the same bytes again. No path here re-prompts a
// wallet.
//
// It stops on its own. A bounded number of attempts means a client that cannot be authorized fails
// with the reason it last saw, rather than sitting in a loop that looks like a slow success. And it
// waits before every retry, with the wait doubling per attempt made: a bounded budget spent in a
// burst is a client hammering a service that needed time.
//
// What to do about a refusal is read off its code. The relayer answers every failure with one code,
// either its own or the one most of the Connectors answered with, and sends no retry flag yet, so
// the action each code implies is this module's table.

/** What the decryption service answered instead of a result: its machine-readable code and message. */
export type SolanaDecryptionRejection = { readonly code: string; readonly message?: string | undefined };

/** What one submission got back: an answer, or a reason there is none. */
export type SolanaDecryptionOutcome<TResponse> =
  | { readonly ok: true; readonly response: TResponse }
  | { readonly ok: false; readonly rejection: SolanaDecryptionRejection };

/** Waiting, as an injected capability, so a test does not have to spend the time. */
export interface SolanaDecryptionClock {
  delay(seconds: number): Promise<void>;
}

/**
 * What to do about a rejection: submit the same bytes again after the backoff, or stop because
 * nothing this layer can do repairs it.
 */
export type SolanaDecryptionRecovery = 'retry-unchanged' | 'give-up';

/**
 * The action each code implies: the Connector's codes (`ErrorCode` in `kms-connector-api`) and the
 * relayer's own (`relayer-http`'s `error.rs`).
 *
 * Split by whether submitting the same bytes again could ever succeed. A code about the request —
 * its shape, its signature, its KMS context — will say the same thing forever. A code about the
 * moment — rate limits, overload, an unindexed leaf, a grant not yet visible — is the same request
 * meeting a later state.
 *
 * - `acl_denied` is retried: an allow or a delegation the Connector cannot see yet is the one
 *   authorization a later attempt can find, and on this path the client's retry is the only one.
 * - `auth_expired` is retried: it refuses the relayer's own `FhevmSig` header, and the relayer signs
 *   a fresh one for the next submission.
 * - `user_signature_rejected` and `user_decryption_expired` are not: an expired or revoked permit
 *   stays so. The one case that passes later, a start ahead of the host clock, is prevented where
 *   the start is chosen (`permit/start.ts`).
 * - `internal` and `unknown` are not: a relayer bug and a code this table predates are no reason to
 *   spend the budget, and an unknown code treated as retryable is a client hammering the service
 *   over something no delay repairs.
 */
export const SOLANA_DECRYPTION_CODE_ACTIONS: Readonly<Record<string, SolanaDecryptionRecovery>> = {
  // The Connector's codes.
  malformed: 'give-up',
  unsupported_attestation_type: 'give-up',
  sender_authentication_failed: 'give-up',
  auth_expired: 'retry-unchanged',
  rate_limited: 'retry-unchanged',
  overloaded: 'retry-unchanged',
  acl_denied: 'retry-unchanged',
  user_signature_rejected: 'give-up',
  user_decryption_expired: 'give-up',
  ciphertext_not_found: 'retry-unchanged',
  copro_consensus_failed: 'retry-unchanged',
  kms_context_invalid: 'retry-unchanged',
  kms_context_destroyed: 'give-up',
  unprocessable: 'give-up',
  upstream_transient: 'retry-unchanged',
  timeout: 'retry-unchanged',
  unknown: 'give-up',
  // The relayer's own: a wrong route or method is a misconfigured URL.
  not_found: 'give-up',
  method_not_allowed: 'give-up',
  internal: 'give-up',
  shutting_down: 'retry-unchanged',
};

/**
 * Picks the action for one rejection; a code not in the table is given up on.
 *
 * @param rejection - What the service answered.
 */
export function classifySolanaDecryptionRejection(rejection: SolanaDecryptionRejection): SolanaDecryptionRecovery {
  return SOLANA_DECRYPTION_CODE_ACTIONS[rejection.code] === 'retry-unchanged' ? 'retry-unchanged' : 'give-up';
}

/** How many submissions one call may make before it reports the last rejection. */
export const SOLANA_DECRYPTION_DEFAULT_ATTEMPTS = 4;

/** The wait before the first resubmission; it doubles with each attempt made. */
export const SOLANA_DECRYPTION_RETRY_SECONDS = 2;

/** A decryption that ended without an answer. */
export class SolanaDecryptionRunError extends Error {
  /** What the last attempt saw. */
  readonly rejection: SolanaDecryptionRejection;
  /** How many submissions were made. */
  readonly attempts: number;

  constructor(rejection: SolanaDecryptionRejection, attempts: number) {
    const detail = rejection.message === undefined ? '' : `: ${rejection.message}`;
    super(`the decryption request was not answered after ${attempts} attempt(s): ${rejection.code}${detail}`);
    this.name = 'SolanaDecryptionRunError';
    this.rejection = rejection;
    this.attempts = attempts;
  }
}

/**
 * Submits one request until it is answered, refused for good, or out of attempts.
 *
 * @param run.submit - Submits the request once; every call sends the same bytes.
 * @param run.clock - Used for the backoff between attempts.
 * @param run.attempts - Attempt budget; defaults to {@link SOLANA_DECRYPTION_DEFAULT_ATTEMPTS}.
 * @throws SolanaDecryptionRunError - When the budget runs out, or the rejection cannot be repaired.
 */
export async function retrySolanaDecryption<TResponse>(run: {
  readonly submit: () => Promise<SolanaDecryptionOutcome<TResponse>>;
  readonly clock: SolanaDecryptionClock;
  readonly attempts?: number | undefined;
}): Promise<TResponse> {
  const budget = run.attempts ?? SOLANA_DECRYPTION_DEFAULT_ATTEMPTS;
  if (!Number.isSafeInteger(budget) || budget < 1) {
    throw new RangeError('attempts must be a positive safe integer');
  }

  for (let attempts = 1; ; attempts += 1) {
    const outcome = await run.submit();
    if (outcome.ok) {
      return outcome.response;
    }
    // An unrepairable rejection and a spent budget end the same way: with the rejection last seen
    // and the count of what it cost to see it.
    if (classifySolanaDecryptionRejection(outcome.rejection) === 'give-up' || attempts >= budget) {
      throw new SolanaDecryptionRunError(outcome.rejection, attempts);
    }
    // A fault that survived a retry is not one immediate resubmission away.
    await run.clock.delay(SOLANA_DECRYPTION_RETRY_SECONDS * 2 ** (attempts - 1));
  }
}
