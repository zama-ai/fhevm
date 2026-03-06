import { describe, expect, test } from "bun:test";

import { ExitCode, exitWithError, formatError, formatErrorJson } from "./errors";

describe("errors", () => {
  test("formats full error context", () => {
    const formatted = formatError({
      exitCode: ExitCode.DOCKER,
      message: "container failed",
      step: "Step 7: Coprocessor",
      service: "coprocessor-tfhe-worker",
      logLines: ["line1", "line2"],
      logHint: "fhevm-cli logs coprocessor-tfhe-worker",
    });

    expect(formatted).toContain("Error [exit 3]: container failed");
    expect(formatted).toContain("Step: Step 7: Coprocessor");
    expect(formatted).toContain("Service: coprocessor-tfhe-worker");
    expect(formatted).toContain("Last 20 log lines:");
    expect(formatted).toContain("line2");
    expect(formatted).toContain("Hint: fhevm-cli logs coprocessor-tfhe-worker");
  });

  test("formats minimal error context", () => {
    expect(formatError({ exitCode: ExitCode.GENERAL, message: "boom" })).toBe("Error [exit 1]: boom");
  });

  test("defaults hint from service", () => {
    const formatted = formatError({
      exitCode: ExitCode.CONFIG,
      message: "bad config",
      service: "gateway-node",
    });

    expect(formatted).toContain("Hint: fhevm-cli logs gateway-node");
  });

  test("formats JSON error context", () => {
    const raw = formatErrorJson({
      exitCode: ExitCode.DOCKER,
      message: "container failed",
      step: "Step 7: Coprocessor",
      service: "coprocessor-tfhe-worker",
      logLines: ["line1", "line2"],
      logHint: "fhevm-cli logs coprocessor-tfhe-worker",
    });
    const parsed = JSON.parse(raw) as Record<string, unknown>;

    expect(parsed.error).toBe(true);
    expect(parsed.exitCode).toBe(ExitCode.DOCKER);
    expect(parsed.message).toBe("container failed");
    expect(parsed.step).toBe("Step 7: Coprocessor");
    expect(parsed.service).toBe("coprocessor-tfhe-worker");
    expect(parsed.logLines).toEqual(["line1", "line2"]);
    expect(parsed.hint).toBe("fhevm-cli logs coprocessor-tfhe-worker");
  });

  test("formats JSON error with null optional fields", () => {
    const parsed = JSON.parse(formatErrorJson({ exitCode: ExitCode.GENERAL, message: "boom" })) as Record<
      string,
      unknown
    >;

    expect(parsed.step).toBeNull();
    expect(parsed.service).toBeNull();
    expect(parsed.logLines).toBeNull();
    expect(parsed.hint).toBeNull();
  });

  test("exitWithError emits JSON when requested", () => {
    const stderr: string[] = [];
    const originalError = console.error;
    const originalExit = process.exit;

    console.error = (...args: unknown[]) => {
      stderr.push(args.map((arg) => String(arg)).join(" "));
    };
    (process as { exit: (code?: number) => never }).exit = ((code?: number) => {
      throw new Error(`exit:${String(code)}`);
    }) as typeof process.exit;

    try {
      expect(() =>
        exitWithError(
          {
            exitCode: ExitCode.CONFIG,
            message: "bad config",
            service: "gateway-node",
          },
          { json: true },
        ),
      ).toThrow("exit:2");

      const parsed = JSON.parse(stderr[0] ?? "") as Record<string, unknown>;
      expect(parsed.message).toBe("bad config");
      expect(parsed.hint).toBe("fhevm-cli logs gateway-node");
    } finally {
      console.error = originalError;
      process.exit = originalExit;
    }
  });

  test("exposes expected exit codes", () => {
    expect(ExitCode.SUCCESS).toBe(0);
    expect(ExitCode.GENERAL).toBe(1);
    expect(ExitCode.CONFIG).toBe(2);
    expect(ExitCode.DOCKER).toBe(3);
    expect(ExitCode.TEST_FAILURE).toBe(10);
  });
});
