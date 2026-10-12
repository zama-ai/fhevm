// The decryption relayer's synchronous routes: one POST, one answer.
//
// `relayer-http` fans a request out to the KMS Connectors and answers once a threshold of them did,
// in the same HTTP exchange: `{ status: "succeeded", requestId, result }` on success, and
// `{ code, message, requestId }` with an HTTP status otherwise. There is no job to poll and no
// Retry-After to serve; whether to submit again is the retry loop's decision, made from the code.
//
// A response that is neither envelope did not come from the relayer's handlers (an edge proxy, a
// wrong URL): it is infrastructure, not a refusal the retry loop could classify, so it surfaces as
// an error.

import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { RelayerCommonOptions } from '../../core/types/relayer.js';
import type { SolanaDecryptionDeadline } from './deadline.js';
import type { SolanaDecryptionOutcome } from './retry.js';
import { setAuth } from '../../core/base/auth.js';
import { normalizeHeaders } from '../../core/base/fetch.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from '../../core/modules/relayer/module/relayerUrl.js';
import { sdkName, version } from '../../core/_version.js';

/**
 * The URL of a decryption route on the chain's decryption relayer.
 *
 * @param chain - Names the decryption relayer.
 * @param route - The route.
 * @param hasAuth - Whether credentials travel with the request, which requires HTTPS.
 * @throws InvalidUrlError - If the base URL does not parse.
 */
export function solanaDecryptionUrl(
  chain: FhevmSolanaChain,
  route: 'v4/user-decrypt' | 'v4/public-decrypt',
  hasAuth: boolean,
): string {
  return buildRelayerUrlString(validateRelayerBaseUrl(chain.fhevm.decryptionRelayerUrl, hasAuth), route);
}

/**
 * Posts one request body and reads the relayer's answer, within the operation's deadline.
 *
 * @param request.url - The route's full URL.
 * @param request.body - The request body, serialized as it is.
 * @param request.deadline - Ends the request with the operation's timeout or abort error.
 * @param request.options - The caller's credentials and extra headers.
 * @returns The `result` of a succeeded envelope, or the code and message of an error body.
 * @throws If the request does not reach the relayer, or the answer is neither envelope.
 */
export function postSolanaDecryption<TResult>(request: {
  readonly url: string;
  readonly body: unknown;
  readonly deadline: SolanaDecryptionDeadline;
  readonly options?: Pick<RelayerCommonOptions, 'auth' | 'headers'> | undefined;
}): Promise<SolanaDecryptionOutcome<TResult>> {
  return request.deadline.bound((signal) => post<TResult>(request.url, request.body, request.options, signal));
}

async function post<TResult>(
  url: string,
  body: unknown,
  options: Pick<RelayerCommonOptions, 'auth' | 'headers'> | undefined,
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
