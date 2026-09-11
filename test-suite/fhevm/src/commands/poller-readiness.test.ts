import { expect, test } from "bun:test";
import { waitForPollerCatchup } from "./poller-readiness";

test("waits for an uninitialized or lagging poller on every operator and chain", async () => {
  const targets = Array.from({ length: 6 }, (_, i) => ({ name: `poller-${i}`, block: 100n }));
  let iteration = 0;
  await waitForPollerCatchup(targets, {
    running: async () => true,
    progress: async (name) => name !== "poller-5" || iteration === 2 ? 100n : iteration === 0 ? null : 99n,
    sleep: async () => { iteration++; },
    now: () => iteration * 5_000,
  });
  expect(iteration).toBe(2);
});

test("reports the pending poller and its progress on timeout", async () => {
  let elapsed = 0;
  await expect(waitForPollerCatchup([{ name: "poller-chain-b", block: 100n }], {
    running: async () => true,
    progress: async () => 90n,
    sleep: async (ms) => { elapsed += ms; },
    now: () => elapsed,
  }, 5_000)).rejects.toThrow("Poller catchup timed out: poller-chain-b=90/100");
});

test("rejects a stopped poller even if its stored cursor reached the target", async () => {
  await expect(waitForPollerCatchup([{ name: "poller", block: 100n }], {
    running: async () => false,
    progress: async () => 100n,
    sleep: async () => {},
    now: () => 0,
  })).rejects.toThrow("Poller stopped during catchup: poller");
});
