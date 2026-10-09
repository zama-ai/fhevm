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
    expect(env.capabilities).toEqual({ faucet: true, freshMints: true, fastSlots: true, protocolServices: true });
    expect(env.roots.deployerKeypairPath).toContain(".config/solana/id.json");
    expect(env.coprocessorDbPsql).toEqual(["docker", "exec", "coprocessor-and-kms-db", "psql", "-U", "postgres", "-d", "coprocessor"]);
    expect(env.merkleDbPsql).toEqual(["docker", "exec", "solana-merkle-db", "psql", "-U", "postgres", "-d", "solana_merkle"]);
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

  test("SOLANA_E2E_SOURCE=devnet: no faucet, no fast slots, small transfers from the deployer", () => {
    const env = loadEnv({
      SOLANA_E2E_SOURCE: "devnet",
      COPROCESSOR_DB_PSQL: "kubectl exec -n ns db-0 -- psql -U zama -d e2e",
      MERKLE_DB_PSQL: "kubectl exec -n ns db-0 -- psql -U zama -d solana_merkle",
    });
    expect(env.source).toBe("devnet");
    expect(env.network).toBe("devnet");
    expect(env.capabilities).toEqual({ faucet: false, freshMints: true, fastSlots: false, protocolServices: true });
    expect(env.funding.primarySol).toBeLessThan(1);
    expect(env.coprocessorDbPsql).toEqual(["kubectl", "exec", "-n", "ns", "db-0", "--", "psql", "-U", "zama", "-d", "e2e"]);
    expect(env.merkleDbPsql).toEqual(["kubectl", "exec", "-n", "ns", "db-0", "--", "psql", "-U", "zama", "-d", "solana_merkle"]);
    expect(() => loadEnv({ SOLANA_E2E_SOURCE: "mainnet" })).toThrow(/SOLANA_E2E_SOURCE/);
  });

  test("a demo-config seeded on devnet keeps pre-seeded mints and loses the faucet", () => {
    const env = resolveEnv({}, "demo-config", "devnet");
    expect(env.capabilities).toEqual({ faucet: false, freshMints: false, fastSlots: false, protocolServices: true });
    expect(env.funding).toEqual(resolveEnv({}, "devnet").funding);
  });

  test("the demo-config source labels its provenance and pre-seeds mints (no fresh mints)", () => {
    const env = resolveEnv({ relayerUrl: "http://127.0.0.1:3000" }, "demo-config");
    expect(env.source).toBe("demo-config");
    // Still a local validator: it can fund + advance slots, but mints are pre-seeded, not created.
    expect(env.capabilities).toEqual({ faucet: true, freshMints: false, fastSlots: true, protocolServices: true });
  });

  test("SOLANA_E2E_SOURCE=cleartext: the cleartext stack's validator and wallet, no protocol services", () => {
    const env = loadEnv({ SOLANA_E2E_SOURCE: "cleartext" });
    expect(env.source).toBe("cleartext");
    expect(env.network).toBe("localnet");
    expect(env.rpcUrl).toBe("http://127.0.0.1:28899");
    expect(env.wsUrl).toBe("ws://127.0.0.1:28900");
    expect(env.roots.deployerKeypairPath).toEndWith("solana-cleartext/deployer.json");
    expect(env.capabilities).toEqual({ faucet: true, freshMints: true, fastSlots: true, protocolServices: false });
    expect(loadEnv({ SOLANA_E2E_SOURCE: "cleartext", SOLANA_RPC_URL: "http://10.0.0.1:1" }).rpcUrl).toBe("http://10.0.0.1:1");
  });
});
