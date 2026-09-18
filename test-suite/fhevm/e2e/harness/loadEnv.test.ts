import { describe, expect, test } from "bun:test";

import { loadEnv, resolveEnv } from "./loadEnv";

describe("loadEnv", () => {
  test("defaults reproduce the local clean-e2e stack", () => {
    const env = loadEnv({});
    expect(env.source).toBe("local");
    expect(env.rpcUrl).toBe("http://127.0.0.1:8899");
    expect(env.relayerUrl).toBe("http://127.0.0.1:3000");
    expect(env.chainId).toBe(72057594037940281n);
    expect(env.aclProgram).toMatch(/^0x[0-9a-f]{64}$/);
    expect(env.capabilities).toEqual({ faucet: true, freshMints: true, fastSlots: true });
    expect(env.roots.deployerKeypairPath).toContain(".config/solana/id.json");
    expect(env.coprocessorDbPsql).toEqual(["docker", "exec", "coprocessor-and-kms-db", "psql", "-U", "postgres", "-d", "coprocessor"]);
  });

  test("environment variables override defaults", () => {
    const env = loadEnv({
      SOLANA_RPC_URL: "http://10.0.0.1:8899",
      SOLANA_DEPLOYER_KEYPAIR: "/tmp/custom.json",
    });
    expect(env.rpcUrl).toBe("http://10.0.0.1:8899");
    expect(env.roots.deployerKeypairPath).toBe("/tmp/custom.json");
  });

  test("rejects a non-Solana (low-bit) chain id", () => {
    expect(() => resolveEnv({ chainId: "12345" })).toThrow(/not a Solana type-byte chain id/);
  });

  test("rejects a malformed ACL program identity", () => {
    expect(() => resolveEnv({ aclProgram: "0xdeadbeef" })).toThrow(/32-byte hex/);
  });

  test("rejects a non-decimal user-decrypt context id", () => {
    expect(() => resolveEnv({ userDecryptContextId: "0x01" })).toThrow(/unsigned decimal/);
  });

  test("SOLANA_E2E_SOURCE=devnet: no faucet, no fast slots, small transfers from the deployer", () => {
    const env = loadEnv({ SOLANA_E2E_SOURCE: "devnet", COPROCESSOR_DB_PSQL: "kubectl exec -n ns db-0 -- psql -U zama -d e2e" });
    expect(env.source).toBe("devnet");
    expect(env.network).toBe("devnet");
    expect(env.capabilities).toEqual({ faucet: false, freshMints: true, fastSlots: false });
    expect(env.funding.primarySol).toBeLessThan(1);
    expect(env.coprocessorDbPsql).toEqual(["kubectl", "exec", "-n", "ns", "db-0", "--", "psql", "-U", "zama", "-d", "e2e"]);
    expect(() => loadEnv({ SOLANA_E2E_SOURCE: "mainnet" })).toThrow(/SOLANA_E2E_SOURCE/);
  });

  test("a demo-config seeded on devnet keeps pre-seeded mints and loses the faucet", () => {
    const env = resolveEnv({}, "demo-config", "devnet");
    expect(env.capabilities).toEqual({ faucet: false, freshMints: false, fastSlots: false });
    expect(env.funding).toEqual(resolveEnv({}, "devnet").funding);
  });

  test("the demo-config source labels its provenance and pre-seeds mints (no fresh mints)", () => {
    const env = resolveEnv({ relayerUrl: "http://127.0.0.1:3000" }, "demo-config");
    expect(env.source).toBe("demo-config");
    // Still a local validator: it can fund + advance slots, but mints are pre-seeded, not created.
    expect(env.capabilities).toEqual({ faucet: true, freshMints: false, fastSlots: true });
  });
});
