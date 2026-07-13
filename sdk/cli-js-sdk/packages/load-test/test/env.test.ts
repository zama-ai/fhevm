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
      /--relayer-b-api-prefix .* requires --relayer-b-url/,
    );
  });

  it("rejects --relayer-b-config without a relayer B URL", () => {
    expect(() => resolveEnv({ relayerBConfigPath: "/tmp/relayer-b.json" })).toThrow(
      /--relayer-b-config .* requires --relayer-b-url/,
    );
  });

  it("rejects LOAD_TEST_RELAYER_B_API_PREFIX without LOAD_TEST_RELAYER_B_URL", () => {
    process.env.LOAD_TEST_RELAYER_B_API_PREFIX = "/v2";
    expect(() => resolveEnv({})).toThrow(/requires --relayer-b-url/);
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

  it("rejects relayer B that is an identical target (same URL and API prefix)", () => {
    expect(() => resolveEnv({
      relayerUrl: "https://relayer.example",
      relayerBUrl: "https://relayer.example/v2",
    })).toThrow(/same target as the primary relayer/);
  });

  it("allows relayer B on a distinct origin", () => {
    const env = resolveEnv({
      relayerUrl: "https://relayer.example",
      relayerBUrl: "https://candidate.example",
    });
    expect(env.relayerBUrl).toBe("https://candidate.example");
  });

  it("accepts a path-routed B on the same host with a distinct API prefix", () => {
    // One gateway host serving A and B under different API prefixes is a
    // legitimate deployment and must not be rejected.
    const env = resolveEnv({
      relayerUrl: "https://gateway.example",
      relayerApiPrefix: "/v1",
      relayerBUrl: "https://gateway.example",
      relayerBApiPrefix: "/v2",
    });
    expect(env.relayerBUrl).toBe("https://gateway.example");
    expect(env.relayerApiPrefix).toBe("/v1");
    expect(env.relayerBApiPrefix).toBe("/v2");
  });

  it("accepts a path-routed B on the same host under a distinct base path", () => {
    const env = resolveEnv({
      relayerUrl: "https://gateway.example/tenant-a",
      relayerBUrl: "https://gateway.example/tenant-b",
    });
    expect(env.relayerBUrl).toBe("https://gateway.example/tenant-b");
  });

  it("rejects relayer B with an identical URL and the same effective prefix", () => {
    expect(() => resolveEnv({
      relayerUrl: "https://gateway.example",
      relayerApiPrefix: "/v2",
      relayerBUrl: "https://gateway.example",
      // B prefix falls back to A's (/v2), so this is the same target.
    })).toThrow(/same target as the primary relayer/);
  });
});
