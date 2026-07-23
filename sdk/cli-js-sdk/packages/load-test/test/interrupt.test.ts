import { describe, expect, it } from "vitest";

import { isUserInterruption, RunInterruptedError } from "../src/runner/interrupt";

const abortError = (message: string): Error => {
  // undici's RequestAbortedError shape: operational, but name === "AbortError".
  const error = new Error(message);
  error.name = "AbortError";
  return error;
};

describe("isUserInterruption", () => {
  it("treats a RunInterruptedError as a user interruption regardless of signal", () => {
    expect(isUserInterruption(new RunInterruptedError())).toBe(true);
    expect(isUserInterruption(new RunInterruptedError(), new AbortController().signal)).toBe(true);
  });

  it("classifies the signal's own abort reason as an interruption", () => {
    const controller = new AbortController();
    const reason = new Error("stopped");
    controller.abort(reason);
    expect(isUserInterruption(reason, controller.signal)).toBe(true);
  });

  it("does NOT classify an operational AbortError as an interruption when no signal fired", () => {
    expect(isUserInterruption(abortError("other side closed"))).toBe(false);
    expect(isUserInterruption(abortError("other side closed"), new AbortController().signal)).toBe(
      false,
    );
  });

  it("does NOT disguise an unrelated failure that merely coincides with an abort", () => {
    const controller = new AbortController();
    controller.abort();
    // Aborted signal, but the surfaced error is not the abort reason.
    expect(isUserInterruption(abortError("undici socket"), controller.signal)).toBe(false);
    expect(isUserInterruption(new Error("planning exploded"), controller.signal)).toBe(false);
  });
});
