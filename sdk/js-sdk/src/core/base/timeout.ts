import { isRecordStringProperty } from './string.js';

/**
 * Creates an Error for an aborted sleep. Preserves the original abort reason as
 * the error's cause, and reuses the reason's own `name` when it has one (e.g. a
 * `DOMException('…', 'TimeoutError')` passed to `controller.abort(reason)`) so a
 * caller can tell a timeout from a plain cancellation; defaults to 'AbortError'
 * otherwise, for consistency with fetch's own abort behavior.
 */
function createAbortError(reason?: unknown): Error {
  const name = isRecordStringProperty(reason, 'name') ? reason.name : 'AbortError';
  const error = new Error('Aborted', { cause: reason });
  error.name = name;
  return error;
}

/**
 * Returns a Promise that resolves after the specified delay, but can be aborted.
 *
 * @throws {Error} An error whose `name` is the abort reason's own `name` if it has
 * one (e.g. 'TimeoutError'), or 'AbortError' otherwise, if the signal is aborted
 */
export function abortableSleep(ms: number, signal?: AbortSignal): Promise<void> {
  // Check if already aborted before creating the Promise
  signal?.throwIfAborted();

  return new Promise((resolve, reject) => {
    const onAbort = (): void => {
      clearTimeout(timeoutId);
      reject(createAbortError(signal?.reason));
    };
    const timeoutId = setTimeout(() => {
      signal?.removeEventListener('abort', onAbort);
      resolve();
    }, ms);

    signal?.addEventListener('abort', onAbort, { once: true });
  });
}
