// The relayer transport: the v3 user-decrypt job, driven by the core async request.
//
// Submitting and waiting live in `RelayerAsyncRequest` — the POST/GET job loop, the Retry-After
// waits, the global timeout, and the 429, which it waits out at the wire with the server's own
// delay and the same bytes. That last point is why this transport never produces the `overloaded`
// rejection: by the time an outcome reaches the port, every wait the relayer asked for has already
// been served, closer to the wire than the session's backoff could serve it.
//
// What belongs here is translation, in both directions: the request body goes through verbatim
// (it is already the relayer's wire shape), and the class's outcomes come back as the port's — an
// answer's shares, a refusal or a failed job as its machine-readable label, a completed job with
// nothing in it as `unanswered`. An error that is not a relayer answer at all (an edge proxy, a
// network fault the inner retries did not repair) is not translated: it is infrastructure, not a
// rejection the session could classify, and it surfaces as the error it is.

import type { RelayerUserDecryptOptions, RelayerUserDecryptProgressArgs } from '../../core/types/relayer.js';
import type { FetchUserDecryptResult } from '../../core/types/relayer.js';
import type { SolanaSigncryptedShare } from './response.js';
import type { SolanaUserDecryptRejection } from './failure.js';
import type { SolanaUserDecryptRequestJson } from './request.js';
import type { SolanaUserDecryptTransport, SolanaUserDecryptClock } from './session.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from '../../core/modules/relayer/module/relayerUrl.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { RelayerResponseApiError } from '../../core/errors/RelayerResponseApiError.js';
import { abortableSleep } from '../../core/base/timeout.js';
import { RelayerTimeoutError } from '../../core/errors/RelayerTimeoutError.js';

/**
 * Builds the transport and retry clock for one user-decrypt operation.
 * One deadline bounds all its requests and backoff; create a new transport for the next operation.
 *
 * One `RelayerAsyncRequest` per submission: the class is single-run by design, and the session's
 * retry loop decides whether there is another submission at all.
 *
 * @param config.relayerUrl - The relayer base URL; the v3 user-decrypt path is appended here.
 * @param config.options - Core relayer options (auth, timeout, abort signal, progress callback).
 * @throws InvalidUrlError - If the base URL does not parse, at construction rather than mid-session.
 */
export function createSolanaUserDecryptRelayerTransport(config: {
  readonly relayerUrl: string;
  readonly options?: RelayerUserDecryptOptions | undefined;
}): SolanaUserDecryptTransport<readonly SolanaSigncryptedShare[]> &
  SolanaUserDecryptClock & { throwIfAbortedOrExpired(): void } {
  const baseUrl = validateRelayerBaseUrl(config.relayerUrl, config.options?.auth !== undefined);
  const url = buildRelayerUrlString(baseUrl, 'v3/user-decrypt');
  const timeout = config.options?.timeout ?? RelayerAsyncRequest.DEFAULT_GLOBAL_REQUEST_TIMEOUT_MS;
  if (!Number.isSafeInteger(timeout) || timeout < 1 || timeout > 2_147_483_647) {
    throw new RangeError('timeout must be an integer from 1 to 2147483647 milliseconds');
  }
  const deadline = Date.now() + timeout;
  let lastProgress: RelayerUserDecryptProgressArgs | undefined;
  const expire = (): never => {
    const progress = {
      url,
      operation: 'USER_DECRYPT' as const,
      retryCount: 0,
      step: 0,
      totalSteps: 0,
      ...lastProgress,
      type: 'timeout' as const,
    };
    const onProgress = config.options?.onProgress;
    if (onProgress) {
      queueMicrotask(() => {
        onProgress(progress);
      });
    }

    throw new RelayerTimeoutError({ operation: 'USER_DECRYPT', url, timeoutMs: timeout });
  };
  const remaining = (): number => {
    config.options?.signal?.throwIfAborted();
    const milliseconds = deadline - Date.now();
    return milliseconds > 0 ? milliseconds : expire();
  };

  return {
    throwIfAbortedOrExpired(): void {
      remaining();
    },
    async delay(seconds: number): Promise<void> {
      const milliseconds = remaining();
      await abortableSleep(Math.min(seconds * 1000, milliseconds), config.options?.signal);
      remaining();
    },
    async submit(request: SolanaUserDecryptRequestJson) {
      // Whether the request became a job: the one fact that splits a refusal (the relayer never
      // accepted it) from a failure (the job it became did not produce an answer). The class keeps
      // its job id private, but it announces the acceptance through the progress callback.
      let queued = false;

      const relayerRequest = new RelayerAsyncRequest({
        relayerOperation: 'USER_DECRYPT',
        url,
        payload: request as unknown as Record<string, unknown>,
        options: {
          ...config.options,
          timeout: remaining(),
          onProgress: (progress: RelayerUserDecryptProgressArgs) => {
            lastProgress = progress;
            if (progress.type === 'queued') {
              queued = true;
            }
            config.options?.onProgress?.(progress);
          },
        },
      });

      try {
        const shares = (await relayerRequest.run()) as FetchUserDecryptResult;
        // A completed job carrying nothing is not an answer to verify: it is what a Connector
        // refusal looks like from here, and the session repairs it by resolving the evidence again.
        if (shares.length === 0) {
          return { ok: false, rejection: { kind: 'unanswered' } };
        }
        return {
          ok: true,
          response: shares.map((share) => ({
            signature: share.signature,
            payload: share.payload,
            extraData: share.extraData,
          })),
        };
      } catch (error) {
        return { ok: false, rejection: rejectionFrom(error, queued) };
      }
    },
  };
}

/**
 * Translates a thrown relayer outcome into the port's rejection, or rethrows what is not one.
 *
 * @param error - Whatever the core request threw.
 * @param queued - Whether the request had become a job before the error.
 */
function rejectionFrom(error: unknown, queued: boolean): SolanaUserDecryptRejection {
  if (error instanceof RelayerResponseApiError) {
    const { label, message } = error.relayerApiError;
    return queued ? { kind: 'failed', label, message } : { kind: 'refused', label, message };
  }
  throw error;
}
