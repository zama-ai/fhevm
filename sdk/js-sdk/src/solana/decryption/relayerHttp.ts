// The decryption relayer's synchronous routes: one POST, one answer.
//
// `relayer-http` fans a request out to the KMS Connectors and answers once a threshold of them did,
// in the same HTTP exchange: `{ status: "succeeded", requestId, result }` on success, and
// `{ code, message, requestId }` with an HTTP status otherwise.
//
// A response that is neither envelope did not come from the relayer's handlers (an edge proxy, a
// wrong URL): it is infrastructure, not a refusal the retry loop could classify, so it surfaces as
// an error.

import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaDecryptionDeadline, SolanaDecryptionOperation, SolanaDecryptionOptions } from './deadline.js';
import type { SolanaDecryptionOutcome } from './retry.js';
import { setAuth } from '../../core/base/auth.js';
import { normalizeHeaders } from '../../core/base/fetch.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from '../../core/modules/relayer/module/relayerUrl.js';
import { sdkName, version } from '../../core/_version.js';
import { createSolanaDecryptionDeadline } from './deadline.js';

const ROUTES = {
  USER_DECRYPT: 'v4/user-decrypt',
  PUBLIC_DECRYPT: 'v4/public-decrypt',
} as const satisfies Record<SolanaDecryptionOperation, string>;

/** One decryption on the chain's decryption relayer: its deadline, and a submission bounded by it. */
export type SolanaDecryptionRelayer = {
  readonly deadline: SolanaDecryptionDeadline;
  /**
   * Posts the body once and reads the answer.
   *
   * @returns The `result` of a succeeded envelope, or the code and message of an error body.
   * @throws If the request does not reach the relayer, or the answer is neither envelope.
   */
  submit<TResult>(body: unknown): Promise<SolanaDecryptionOutcome<TResult>>;
};

/**
 * The URL of an operation's route on the chain's decryption relayer.
 *
 * @param chain - Names the decryption relayer.
 * @param operation - Selects the route.
 * @param hasAuth - Whether credentials travel with the request, which requires HTTPS.
 * @throws InvalidUrlError - If the base URL does not parse.
 */
export function solanaDecryptionUrl(
  chain: FhevmSolanaChain,
  operation: SolanaDecryptionOperation,
  hasAuth: boolean,
): string {
  return buildRelayerUrlString(validateRelayerBaseUrl(chain.fhevm.decryptionRelayerUrl, hasAuth), ROUTES[operation]);
}

/**
 * Starts one decryption on the chain's decryption relayer.
 *
 * @param chain - Names the decryption relayer.
 * @param operation - Selects the route, and names the deadline's errors and progress events.
 * @param options - The caller's credentials, headers, timeout, abort signal and progress callback.
 *   Credentials require an HTTPS URL.
 * @throws InvalidUrlError - If the base URL does not parse.
 */
export function solanaDecryptionRelayer<O extends SolanaDecryptionOperation>(
  chain: FhevmSolanaChain,
  operation: O,
  options: SolanaDecryptionOptions<O> | undefined,
): SolanaDecryptionRelayer {
  const url = solanaDecryptionUrl(chain, operation, options?.auth !== undefined);
  const deadline = createSolanaDecryptionDeadline({ operation, url, options });
  return {
    deadline,
    submit: <TResult>(body: unknown) => deadline.bound((signal) => post<TResult>(url, body, options, signal)),
  };
}

async function post<TResult>(
  url: string,
  body: unknown,
  options: Pick<SolanaDecryptionOptions<SolanaDecryptionOperation>, 'auth' | 'headers'> | undefined,
  signal: AbortSignal,
): Promise<SolanaDecryptionOutcome<TResult>> {
  const init = setAuth(
    {
      method: 'POST',
      headers: {
        ...normalizeHeaders(options?.headers),
        'content-type': 'application/json',
        'zama-sdk-version': version,
        'zama-sdk-name': sdkName,
      },
      body: JSON.stringify(body),
      signal,
    },
    options?.auth,
  );
  const response = await fetch(url, init);
  const text = await response.text();
  let envelope: unknown;
  try {
    envelope = JSON.parse(text);
  } catch {
    envelope = undefined;
  }

  if (response.ok) {
    if (isRecord(envelope) && envelope.status === 'succeeded' && 'result' in envelope) {
      return { ok: true, response: envelope.result as TResult };
    }
    throw new Error(`${url} answered HTTP ${response.status} without a succeeded envelope`);
  }
  if (isRecord(envelope) && typeof envelope.code === 'string') {
    const message = typeof envelope.message === 'string' ? envelope.message : undefined;
    return { ok: false, rejection: { code: envelope.code, message } };
  }
  throw new Error(`${url} answered HTTP ${response.status} without an error body`);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}
