// until — a generic polling helper for the Solana e2e scenario layer.
//
// Zero protocol knowledge: it only repeats a caller-supplied async probe until it yields a truthy
// value or a deadline passes. Scenarios use it to gate on stack readiness (relayer accepting POSTs,
// proof-service reporting ready) instead of hand-rolling sleep loops.

export type UntilOptions = {
  /** Overall deadline. Default 120s — long enough for a freshly (re)started relayer/proof-service. */
  readonly timeoutMs?: number;
  /** Delay between attempts. Default 2s, matching the bash readiness loops it replaces. */
  readonly intervalMs?: number;
  /** Human-readable subject for the timeout error (e.g. "relayer readiness"). */
  readonly description?: string;
};

const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * Polls `condition` until it resolves to a truthy value, then returns that value. A probe that
 * throws is treated as "not ready yet" (swallowed) until the deadline, at which point the last
 * error — or a plain timeout — is thrown. `false`, `undefined`, and `null` all mean "keep waiting".
 */
export async function until<T>(
  condition: () => Promise<T | false | undefined | null>,
  options: UntilOptions = {},
): Promise<T> {
  const timeoutMs = options.timeoutMs ?? 120_000;
  const intervalMs = options.intervalMs ?? 2_000;
  const subject = options.description ?? "condition";
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  for (;;) {
    try {
      const result = await condition();
      if (result !== false && result !== undefined && result !== null) return result;
      lastError = undefined;
    } catch (error) {
      lastError = error;
    }
    if (Date.now() >= deadline) {
      const suffix = lastError ? `; last error: ${lastError instanceof Error ? lastError.message : String(lastError)}` : "";
      throw new Error(`until(${subject}) timed out after ${timeoutMs}ms${suffix}`);
    }
    await sleep(intervalMs);
  }
}
