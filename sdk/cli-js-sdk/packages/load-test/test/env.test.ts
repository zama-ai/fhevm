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

describe("resolveEnv A/B target validation", () => {
  it("rejects --relayer-b-api-prefix without a relayer B URL", () => {
    expect(() => resolveEnv({ relayerBApiPrefix: "/v2" })).toThrow(
      /--relayer-b-api-prefix .* requires --relayer-b/,
    );
  });

  it("rejects --relayer-b-config without a relayer B URL", () => {
    expect(() => resolveEnv({ relayerBConfigPath: "/tmp/relayer-b.json" })).toThrow(
      /--relayer-b-config .* requires --relayer-b/,
    );
  });

  it("rejects LOAD_TEST_RELAYER_B_API_PREFIX without LOAD_TEST_RELAYER_B_URL", () => {
    process.env.LOAD_TEST_RELAYER_B_API_PREFIX = "/v2";
    expect(() => resolveEnv({})).toThrow(/requires --relayer-b/);
  });

  it("allows relayer B options when a relayer B URL is configured", () => {
    const env = resolveEnv({
      relayerBUrl: "https://candidate.example",
      relayerBApiPrefix: "/v2",
      relayerBConfigPath: "/tmp/relayer-b.json",
    });
    expect(env.relayerBUrl).toBe("https://candidate.example");
    expect(env.relayerBApiPrefix).toBe("/v2");
    expect(env.relayerBConfigPath).toBe("/tmp/relayer-b.json");
  });

  it("rejects relayer B resolving to the same origin as relayer A", () => {
    expect(() => resolveEnv({
      relayerUrl: "https://relayer.example",
      relayerBUrl: "https://relayer.example/v2",
    })).toThrow(/must not resolve to the same origin/);
  });

  it("allows relayer B on a distinct origin", () => {
    const env = resolveEnv({
      relayerUrl: "https://relayer.example",
      relayerBUrl: "https://candidate.example",
    });
    expect(env.relayerBUrl).toBe("https://candidate.example");
  });
});
