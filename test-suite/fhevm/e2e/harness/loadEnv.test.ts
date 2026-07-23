import { describe, expect, test } from "bun:test";

import { loadEnv, resolveEnv } from "./loadEnv";

describe("loadEnv", () => {
  test("defaults reproduce the local clean-e2e stack", () => {
    const env = loadEnv({});
    expect(env.source).toBe("local");
    expect(env.rpcUrl).toBe("http://127.0.0.1:8899");
    expect(env.relayerUrl).toBe("http://127.0.0.1:3000");
    expect(env.proofServiceUrl).toBe("http://127.0.0.1:8088");
    expect(env.chainId).toBe(9223372036854788153n);
    expect(env.aclProgram).toMatch(/^0x[0-9a-f]{64}$/);
    expect(env.capabilities).toEqual({ faucet: true, freshMints: true, fastSlots: true });
    expect(env.roots.deployerKeypairPath).toContain(".config/solana/id.json");
  });

  test("environment variables override defaults", () => {
    const env = loadEnv({
      SOLANA_RPC_URL: "http://10.0.0.1:8899",
      PROOF_SERVICE_URL: "http://10.0.0.1:8088",
      SOLANA_DEPLOYER_KEYPAIR: "/tmp/custom.json",
    });
    expect(env.rpcUrl).toBe("http://10.0.0.1:8899");
    expect(env.proofServiceUrl).toBe("http://10.0.0.1:8088");
    expect(env.roots.deployerKeypairPath).toBe("/tmp/custom.json");
  });

  test("rejects a non-Solana (low-bit) chain id", () => {
    expect(() => resolveEnv({ chainId: "12345" })).toThrow(/not a Solana high-bit chain id/);
  });

  test("rejects a malformed ACL program identity", () => {
    expect(() => resolveEnv({ aclProgram: "0xdeadbeef" })).toThrow(/32-byte hex/);
  });

  test("rejects a non-decimal user-decrypt context id", () => {
    expect(() => resolveEnv({ userDecryptContextId: "0x01" })).toThrow(/unsigned decimal/);
  });
});
