import { describe, expect, test } from "bun:test";

import { ExitCode } from "../errors";

import { validateUpOptions } from "./up";

describe("up command", () => {
  test("rejects --resume with --from", () => {
    expect(() => validateUpOptions({ resume: true, from: "7" })).toThrowError(
      expect.objectContaining({
        exitCode: ExitCode.CONFIG,
        message: "--resume and --from are mutually exclusive",
      }),
    );
  });

  test("rejects coprocessor local mode when using multi-coprocessor topology", () => {
    expect(() =>
      validateUpOptions({
        local: ["coprocessor"],
        numCoprocessors: 3,
      }),
    ).toThrowError(
      expect.objectContaining({
        exitCode: ExitCode.CONFIG,
        message: "--local for coprocessor services is not supported with --coprocessors > 1",
      }),
    );
  });

  test("allows non-coprocessor local components with multi-coprocessor topology", () => {
    expect(() =>
      validateUpOptions({
        local: ["kms-worker"],
        numCoprocessors: 3,
      }),
    ).not.toThrow();
  });

  test("rejects unknown --local component names", () => {
    expect(() => validateUpOptions({ local: ["invalid-thing"] })).toThrowError(
      expect.objectContaining({
        exitCode: ExitCode.CONFIG,
        message: expect.stringContaining("unknown --local component 'invalid-thing'"),
      }),
    );
  });

  test("rejects --threshold greater than --coprocessors", () => {
    expect(() => validateUpOptions({ threshold: 3, numCoprocessors: 2 })).toThrowError(
      expect.objectContaining({
        exitCode: ExitCode.CONFIG,
        message: "--threshold (3) cannot exceed --coprocessors (2)",
      }),
    );
  });

  test("rejects --threshold greater than default coprocessors (1)", () => {
    expect(() => validateUpOptions({ threshold: 2 })).toThrowError(
      expect.objectContaining({
        exitCode: ExitCode.CONFIG,
        message: "--threshold (2) cannot exceed --coprocessors (1)",
      }),
    );
  });

  test("allows valid --threshold with matching --coprocessors", () => {
    expect(() => validateUpOptions({ threshold: 2, numCoprocessors: 3 })).not.toThrow();
  });
});
