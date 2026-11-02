import type { FetchPadSharesParameters, FetchPadSharesReturnType, RelayerClientWithRuntime } from '../types.js';
import type { PadDecryptResponse } from '../types.js';
import type { RelayerAsyncRequestState, RelayerPadDecryptSucceeded } from '../../../types/relayer-p.js';
import { sdkName, version } from '../../../_version.js';
import { setAuth } from '../../../base/auth.js';
import { InvalidPropertyError } from '../../../base/errors/InvalidPropertyError.js';
import { normalizeHeaders } from '../../../base/fetch.js';
import { RelayerAbortError } from '../../../errors/RelayerAbortError.js';
import { RelayerFetchError } from '../../../errors/RelayerFetchError.js';
import { RelayerResponseInvalidBodyError } from '../../../errors/RelayerResponseInvalidBodyError.js';
import { RelayerResponseStatusError } from '../../../errors/RelayerResponseStatusError.js';
import { RelayerTimeoutError } from '../../../errors/RelayerTimeoutError.js';
import { assertIsRelayerPadDecryptSucceeded } from './guards/RelayerPadDecryptSucceeded.js';
import {
  appendRelayerQuery,
  buildRelayerUrlString,
  normalizeRelayerQuery,
  validateRelayerBaseUrl,
} from './relayerUrl.js';

////////////////////////////////////////////////////////////////////////////////
// fetchPadShares (RFC-030 pad decryption, relayer-http)
////////////////////////////////////////////////////////////////////////////////

/** relayer-http's synchronous route: one POST, answered with every KMS node's response. */
const PAD_DECRYPT_ROUTE = 'v4/pad-decrypt';

/**
 * Posts an RFC-030 pad decryption request to relayer-http (`POST /v4/pad-decrypt`) and returns one
 * {@link PadDecryptResponse} per KMS node that answered, as the node sent it. Nothing is verified here: the caller
 * checks the signatures and reconstructs the pads (`reconstructPads`).
 *
 * The route is synchronous: no job id, no polling. `options.timeout` bounds the whole call, `options.signal` aborts
 * it. A network failure surfaces at once as a {@link RelayerFetchError} (relayer-http already retries its KMS calls
 * within its own deadline), a non-200 as a {@link RelayerResponseStatusError} carrying relayer-http's `code`,
 * `message` and `requestId`.
 */
export async function fetchPadShares(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPadSharesParameters,
): Promise<FetchPadSharesReturnType> {
  const { options, payload } = parameters;
  const message = payload.kmsDecryptEip712Message;

  // The KMS connector's pad envelope (`kms-connector-api` `PadDecryptionRequest`): each pad's label and length
  // around its witness; integers that may exceed 2^53 are decimal strings.
  const body = {
    attestationType: 'eip712-unified-user-decrypt-v1',
    payload: {
      witnessVersion: payload.witnessVersion,
      prfId: payload.prfId,
      chainId: payload.chainId.toString(),
      pads: payload.witnesses.map((w) => ({
        label: w.label,
        witness: {
          submitter: w.submitter,
          consumer: w.consumer,
          expirationTimestamp: w.expirationTimestamp.toString(),
          decryptors: w.decryptors,
          nonce: w.nonce,
        },
        padLength: w.padLength,
      })),
      userAddress: message.userAddress,
      publicKey: message.publicKey,
      allowedContracts: message.allowedContracts,
      requestValidity: {
        startTimestamp: message.startTimestamp,
        durationSeconds: message.durationSeconds,
      },
      extraData: message.extraData,
    },
    signature: payload.kmsDecryptEip712Signature,
  };

  const hasAuth: boolean = options?.auth !== undefined;
  const relayerBaseUrl: URL = validateRelayerBaseUrl(relayerClient.chain.fhevm.relayerUrl, hasAuth);
  const url = buildRelayerUrlString(relayerBaseUrl, PAD_DECRYPT_ROUTE);
  const query = normalizeRelayerQuery(options?.query);
  const headers = normalizeHeaders(options?.headers);

  const started = Date.now();
  const errorParams = (): {
    fetchMethod: 'POST';
    url: string;
    operation: 'PAD_DECRYPT';
    retryCount: number;
    elapsed: number;
    state: RelayerAsyncRequestState;
  } => ({
    fetchMethod: 'POST',
    url,
    operation: 'PAD_DECRYPT',
    retryCount: 0,
    elapsed: Date.now() - started,
    state: { ...IDLE_STATE },
  });

  // One controller for the external signal and the timeout, so `fetch` sees a single signal.
  const controller = new AbortController();
  const external = options?.signal;
  const onExternalAbort = (): void => {
    controller.abort(external?.reason);
  };
  if (external?.aborted === true) {
    throw new RelayerAbortError({ url, operation: 'PAD_DECRYPT' });
  }
  external?.addEventListener('abort', onExternalAbort);
  // An object, not a `let`: the flag is set from the timer callback.
  const timeout = { fired: false };
  const timer =
    options?.timeout !== undefined
      ? setTimeout(() => {
          timeout.fired = true;
          controller.abort();
        }, options.timeout)
      : undefined;

  let response: Response;
  try {
    response = await fetch(
      appendRelayerQuery(url, query),
      setAuth(
        {
          method: 'POST',
          headers: {
            ...headers,
            'content-type': 'application/json',
            'zama-sdk-version': version,
            'zama-sdk-name': sdkName,
          },
          body: JSON.stringify(body),
          signal: controller.signal,
        } satisfies RequestInit,
        options?.auth,
      ),
    );
  } catch (cause) {
    if (timeout.fired) {
      throw new RelayerTimeoutError({ url, operation: 'PAD_DECRYPT', timeoutMs: options?.timeout ?? 0 });
    }
    if (controller.signal.aborted) {
      throw new RelayerAbortError({ url, operation: 'PAD_DECRYPT' });
    }
    throw new RelayerFetchError({ ...errorParams(), message: 'Fetch POST failed.', cause });
  } finally {
    clearTimeout(timer);
    external?.removeEventListener('abort', onExternalAbort);
  }

  const text = await response.text().catch(() => '');

  if (response.status !== 200) {
    const responseMessage = _readRelayerHttpError(text);
    throw new RelayerResponseStatusError({
      ...errorParams(),
      status: response.status,
      ...(responseMessage !== undefined ? { responseMessage } : {}),
    });
  }

  let json: unknown;
  try {
    json = JSON.parse(text);
  } catch (cause) {
    throw new RelayerFetchError({ ...errorParams(), message: 'Response body is not JSON.', cause });
  }
  const succeeded = _assertSucceeded(json, (cause) => {
    throw new RelayerResponseInvalidBodyError({ ...errorParams(), status: response.status, cause, bodyJson: text });
  });

  return Object.freeze(
    succeeded.result.responses.map(
      (r): PadDecryptResponse =>
        Object.freeze({ requestId: r.decryptionId, encryptedPayload: r.encryptedPayload, signature: r.signature }),
    ),
  );
}

////////////////////////////////////////////////////////////////////////////////

/** `json` as the success body; `onInvalid` turns a schema mismatch into the caller's error. */
function _assertSucceeded(
  json: unknown,
  onInvalid: (cause: InvalidPropertyError) => never,
): RelayerPadDecryptSucceeded {
  try {
    assertIsRelayerPadDecryptSucceeded(json, 'body', {});
    return json;
  } catch (cause) {
    if (cause instanceof InvalidPropertyError) {
      onInvalid(cause);
    }
    throw cause;
  }
}

/** The state an error reports for a request that never queued (there is no job on a synchronous route). */
const IDLE_STATE: RelayerAsyncRequestState = Object.freeze({
  aborted: false,
  canceled: false,
  failed: true,
  fetching: false,
  running: false,
  succeeded: false,
  terminated: true,
  timeout: false,
});

/**
 * relayer-http's error body, `{ "code", "message", "requestId" }`, as one line: `[code] message (requestId …)`.
 * Anything else (an edge proxy's page, an empty body) surfaces nothing.
 */
function _readRelayerHttpError(text: string): string | undefined {
  try {
    const json: unknown = JSON.parse(text);
    if (typeof json !== 'object' || json === null) {
      return undefined;
    }
    const { code, message, requestId } = json as Record<string, unknown>;
    if (typeof code !== 'string' && typeof message !== 'string') {
      return undefined;
    }
    const parts = [
      typeof code === 'string' ? `[${code}]` : undefined,
      typeof message === 'string' ? message : undefined,
      typeof requestId === 'string' ? `(requestId ${requestId})` : undefined,
    ];
    return parts.filter((p) => p !== undefined).join(' ');
  } catch {
    return undefined;
  }
}
