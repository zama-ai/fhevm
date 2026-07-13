/** Milliseconds since an arbitrary epoch; monotonic, safe for latency math. */
export const monotonicNowMs = (): number => performance.now();

/** Wall-clock epoch milliseconds, for report timestamps and correlation. */
export const epochNowMs = (): number => Date.now();

/** ISO timestamp for report fields and run directory names. */
export const isoNow = (): string => new Date().toISOString();

/**
 * Sleeps for `ms`, resolving early (without throwing) when `signal` aborts.
 */
export const sleep = (ms: number, signal?: AbortSignal): Promise<void> =>
  new Promise((resolve) => {
    if (ms <= 0 || signal?.aborted) {
      resolve();
      return;
    }
    const timer = setTimeout(() => {
      signal?.removeEventListener("abort", onAbort);
      resolve();
    }, ms);
    const onAbort = (): void => {
      clearTimeout(timer);
      resolve();
    };
    signal?.addEventListener("abort", onAbort, { once: true });
  });

/** Clamps a number into `[min, max]`. */
export const clamp = (value: number, min: number, max: number): number =>
  Math.min(max, Math.max(min, value));
