import { describe, expect, test } from "bun:test";

import { until } from "./until";

describe("until", () => {
  test("returns the first truthy probe value", async () => {
    let calls = 0;
    const value = await until(
      async () => {
        calls += 1;
        return calls >= 3 ? `ready-${calls}` : false;
      },
      { intervalMs: 1, timeoutMs: 1_000 },
    );
    expect(value).toBe("ready-3");
    expect(calls).toBe(3);
  });

  test("treats a throwing probe as not-ready and recovers", async () => {
    let calls = 0;
    const value = await until(
      async () => {
        calls += 1;
        if (calls < 2) throw new Error("boom");
        return 42;
      },
      { intervalMs: 1, timeoutMs: 1_000 },
    );
    expect(value).toBe(42);
  });

  test("times out with the description and the last error", async () => {
    await expect(
      until(async () => Promise.reject(new Error("still-warming-up")), {
        description: "relayer readiness",
        intervalMs: 1,
        timeoutMs: 20,
      }),
    ).rejects.toThrow(/until\(relayer readiness\) timed out.*still-warming-up/);
  });

  test("treats 0 as a ready value (only false/undefined/null mean wait)", async () => {
    let calls = 0;
    const value = await until(
      async () => {
        calls += 1;
        // 0 and "" are falsy but are legitimate results; only false/undefined/null mean "wait".
        return calls === 1 ? undefined : calls === 2 ? 0 : "done";
      },
      { intervalMs: 1, timeoutMs: 1_000 },
    );
    expect(value).toBe(0);
  });
});
