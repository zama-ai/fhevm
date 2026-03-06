import { describe, expect, test } from "bun:test";

import { exec, execOk } from "./shell";

describe("shell", () => {
  test("exec captures stdout", async () => {
    const result = await exec(["echo", "hello"]);

    expect(result.exitCode).toBe(0);
    expect(result.stdout).toBe("hello");
    expect(result.stderr).toBe("");
  });

  test("exec handles non-zero exits", async () => {
    const result = await exec(["sh", "-c", "exit 1"]);
    expect(result.exitCode).toBe(1);
  });

  test("exec forwards environment variables", async () => {
    const result = await exec(["sh", "-c", "printf %s \"$FHEVM_TEST_ENV\""], {
      env: { FHEVM_TEST_ENV: "ok" },
    });
    expect(result.exitCode).toBe(0);
    expect(result.stdout).toBe("ok");
  });

  test("execOk reflects command success", async () => {
    expect(await execOk(["sh", "-c", "exit 0"])).toBe(true);
    expect(await execOk(["sh", "-c", "exit 2"])).toBe(false);
  });

  test("exec timeout kills command", async () => {
    const result = await exec(["sh", "-c", "sleep 2"], { timeoutMs: 100 });
    expect(result.exitCode).not.toBe(0);
  });

  test("exec timeout escalates to SIGKILL when SIGTERM is ignored", async () => {
    const started = Date.now();
    const result = await exec(["sh", "-c", "trap '' TERM; while :; do :; done"], { timeoutMs: 100 });

    expect(result.exitCode).not.toBe(0);
    expect(Date.now() - started).toBeLessThan(3_000);
  });
});
