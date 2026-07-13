import { afterEach, beforeEach, describe, expect, it } from "vitest";

import { resolveEnv } from "../src/env";

const originalEnv = { ...process.env };

beforeEach(() => {
  delete process.env.LOAD_TEST_NETWORK;
});

afterEach(() => {
  process.env = { ...originalEnv };
});

describe("resolveEnv network precedence", () => {
  it("falls back to DEFAULT_NETWORK when neither an override nor LOAD_TEST_NETWORK is set", () => {
    expect(resolveEnv({}).network).toBe("testnet");
  });

  it("honors LOAD_TEST_NETWORK when no explicit override is given", () => {
    process.env.LOAD_TEST_NETWORK = "devnet";
    expect(resolveEnv({}).network).toBe("devnet");
  });

  it("prefers an explicit override (e.g. --network) over LOAD_TEST_NETWORK", () => {
    process.env.LOAD_TEST_NETWORK = "devnet";
    expect(resolveEnv({ network: "mainnet" }).network).toBe("mainnet");
  });
});
