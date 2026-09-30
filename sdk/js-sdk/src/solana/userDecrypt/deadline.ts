// One deadline and one abort signal over a whole user decryption: every submission and every
// backoff between them, whichever backend answers.

import type { RelayerUserDecryptOptions } from '../../core/types/relayer.js';
import type { SolanaUserDecryptClock } from './session.js';
import { abortableSleep } from '../../core/base/timeout.js';
import { RelayerAbortError } from '../../core/errors/RelayerAbortError.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { RelayerTimeoutError } from '../../core/errors/RelayerTimeoutError.js';

/** The retry clock of one user decryption, bounded by the caller's timeout and abort signal. */
export type SolanaUserDecryptDeadline = SolanaUserDecryptClock & {
  /** Milliseconds left; throws the abort or timeout error when the operation may not continue. */
  remaining(): number;
  throwIfAbortedOrExpired(): void;
  /** Records that a request already reported the abort, so it is not reported twice. */
  noteAbortReported(): void;
};

/**
 * Starts the deadline of one user decryption.
 *
 * @param config.url - Where the operation is sent, named by its errors.
 * @param config.options - The caller's timeout, abort signal and progress callback.
 * @throws RangeError - If the timeout is not a positive 32-bit millisecond count.
 */
export function createSolanaUserDecryptDeadline(config: {
  readonly url: string;
  readonly options?: RelayerUserDecryptOptions | undefined;
}): SolanaUserDecryptDeadline {
  const { url } = config;
  const timeout = config.options?.timeout ?? RelayerAsyncRequest.DEFAULT_GLOBAL_REQUEST_TIMEOUT_MS;
  if (!Number.isSafeInteger(timeout) || timeout < 1 || timeout > 2_147_483_647) {
    throw new RangeError('timeout must be an integer from 1 to 2147483647 milliseconds');
  }
  const deadline = Date.now() + timeout;
  const expire = (): never => {
    const progress = {
      url,
      operation: 'USER_DECRYPT' as const,
      retryCount: 0,
      step: 0,
      totalSteps: 0,
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
  let abortReported = false;
  const abort = (): never => {
    if (!abortReported) {
      abortReported = true;
      const onProgress = config.options?.onProgress;
      if (onProgress) {
        queueMicrotask(() => {
          onProgress({
            type: 'abort',
            operation: 'USER_DECRYPT',
            url,
            retryCount: 0,
            step: 0,
            totalSteps: 0,
          });
        });
      }
    }
    throw new RelayerAbortError({ operation: 'USER_DECRYPT', url });
  };
  const remaining = (): number => {
    if (config.options?.signal?.aborted === true) abort();
    const milliseconds = deadline - Date.now();
    return milliseconds > 0 ? milliseconds : expire();
  };

  return {
    remaining,
    throwIfAbortedOrExpired(): void {
      remaining();
    },
    noteAbortReported(): void {
      abortReported = true;
    },
    async delay(seconds: number): Promise<void> {
      const milliseconds = remaining();
      try {
        await abortableSleep(Math.min(seconds * 1000, milliseconds), config.options?.signal);
      } catch (error) {
        if (config.options?.signal?.aborted === true) abort();
        throw error;
      }
      remaining();
    },
  };
}
