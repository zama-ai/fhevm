/**
 * Creates an Error with name 'AbortError' for consistency with fetch abort behavior.
 * Preserves the original abort reason as the error's cause.
 */
function createAbortError(reason?: unknown): Error {
  const error = new Error('Aborted', { cause: reason });
  error.name = 'AbortError';
  return error;
}

/**
 * Returns a Promise that resolves after the specified delay, but can be aborted.
 *
 * @throws {Error} An error with name 'AbortError' if the signal is aborted
 */
export function abortableSleep(
  ms: number,
  signal?: AbortSignal,
): Promise<void> {
  // Check if already aborted before creating the Promise
  signal?.throwIfAborted();

  return new Promise((resolve, reject) => {
    const timeoutId = setTimeout(resolve, ms);

    signal?.addEventListener(
      'abort',
      () => {
        clearTimeout(timeoutId);
        reject(createAbortError(signal.reason));
      },
      { once: true },
    );
  });
}
