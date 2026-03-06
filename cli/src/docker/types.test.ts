import { describe, expect, test } from "bun:test";

import { DEFAULT_POLL_INTERVAL_MS, DEFAULT_TIMEOUTS, DOCKER_PROJECT } from "./types";

describe("docker types", () => {
  test("exposes default project and polling settings", () => {
    expect(DOCKER_PROJECT).toBe("fhevm");
    expect(DEFAULT_POLL_INTERVAL_MS).toBe(1_000);
  });

  test("uses spec timeout defaults", () => {
    expect(DEFAULT_TIMEOUTS.serviceMs).toBe(150_000);
    expect(DEFAULT_TIMEOUTS.relayerMs).toBe(120_000);
    expect(DEFAULT_TIMEOUTS.keyBootstrapMs).toBe(300_000);
    expect(DEFAULT_TIMEOUTS.oneShotMs).toBe(300_000);
  });
});
