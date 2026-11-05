/**
 * Returns a Promise that resolves after the specified delay, but can be aborted.
 *
 * @throws The signal's abort reason itself (any value, unchanged), whether the signal
 * is already aborted or aborts during the sleep, as `fetch` does.
 */
export function abortableSleep(ms: number, signal?: AbortSignal): Promise<void> {
  // Check if already aborted before creating the Promise
  signal?.throwIfAborted();

  return new Promise((resolve, reject) => {
    const onAbort = (): void => {
      clearTimeout(timeoutId);
      // The reason is caller-chosen (any value) and must pass through unchanged, like fetch and throwIfAborted().
      // eslint-disable-next-line @typescript-eslint/prefer-promise-reject-errors
      reject(signal?.reason);
    };
    const timeoutId = setTimeout(() => {
      signal?.removeEventListener('abort', onAbort);
      resolve();
    }, ms);

    signal?.addEventListener('abort', onAbort, { once: true });
  });
}
