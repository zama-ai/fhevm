/**
 * Interruption primitives shared by the runner and every layer above it.
 *
 * A user interruption is signaled exactly one of two ways: the controlling
 * `AbortSignal` has fired, or the surfaced error is (or equals) the runner's
 * distinctive `RunInterruptedError` abort reason. Classifying by
 * `error.name === "AbortError"` is unsafe above `executeRun`: undici's
 * operational `RequestAbortedError` also reports `name === "AbortError"` even
 * when no signal ever fired, so a name check would mask real failures as
 * clean interruptions. Keeping this in a dependency-free leaf module lets the
 * CLI import it without eagerly loading the whole runner.
 */
export class RunInterruptedError extends Error {
  constructor() {
    super("Load-test run was interrupted before execution could start.");
    this.name = "RunInterruptedError";
  }
}

/**
 * True only when the error genuinely originates from a user interruption:
 * either the runner surfaced its distinctive `RunInterruptedError`, or the
 * controlling signal aborted AND the error is that signal's own abort reason
 * (what `throwIfAborted`, fetch, and undici propagate on a user abort). Never
 * keys off `error.name`, and — mirroring `executeRun` — a bare aborted signal
 * is not enough: an unrelated failure that merely coincides with an abort
 * stays a failure rather than being disguised as a clean interruption.
 */
export const isUserInterruption = (error: unknown, signal?: AbortSignal): boolean =>
  error instanceof RunInterruptedError ||
  (signal?.aborted === true && error === signal.reason);
