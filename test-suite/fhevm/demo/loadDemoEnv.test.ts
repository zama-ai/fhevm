import { afterEach, describe, expect, test } from "bun:test";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { getAddressDecoder } from "@solana/kit";

import { writeDemoConfig, type SolanaDemoConfig } from "./config";
import { loadDemoEnv } from "./loadDemoEnv";

const addr = (fill: number): string => getAddressDecoder().decode(new Uint8Array(32).fill(fill));

const sampleConfig = (): SolanaDemoConfig => ({
  source: "demo-config",
  chainId: "9223372036854788153",
  rpcUrl: "http://127.0.0.1:9999",
  wsUrl: "ws://127.0.0.1:9998",
  relayerUrl: "http://127.0.0.1:3001",
  proofServiceUrl: "http://127.0.0.1:8089",
  gatewayRpcUrl: "http://127.0.0.1:8547",
  aclProgram: `0x${"cd".repeat(32)}`,
  userDecryptContextId: "7",
  authorityFundingLamports: "100000000",
  programs: { batcher: addr(30), token: addr(31), vault: addr(32), host: addr(33) } as SolanaDemoConfig["programs"],
  hostConfig: addr(8) as SolanaDemoConfig["hostConfig"],
  kmsContext: addr(9) as SolanaDemoConfig["kmsContext"],
  vault: addr(10) as SolanaDemoConfig["vault"],
  mints: {
    joinUnderlying: addr(5),
    payoutUnderlying: addr(14),
    joinConfidential: addr(4),
    payoutConfidential: addr(13),
  } as SolanaDemoConfig["mints"],
  batchers: {
    deposit: { batcher: addr(2), lookupTable: addr(200) },
    redeem: { batcher: addr(3), lookupTable: addr(201) },
  } as SolanaDemoConfig["batchers"],
  mintAuthority: addr(50) as SolanaDemoConfig["mintAuthority"],
  personas: { keeper: addr(60), alice: addr(61), bob: addr(62) } as SolanaDemoConfig["personas"],
});

const tmp: string[] = [];
afterEach(async () => {
  for (const f of tmp.splice(0)) await fs.rm(f, { force: true });
});

describe("loadDemoEnv", () => {
  test("projects the demo-config onto a demo-config-sourced TestEnv", async () => {
    const cfg = sampleConfig();
    const file = path.join(await fs.mkdtemp(path.join(os.tmpdir(), "demo-env-")), "solana-demo.json");
    tmp.push(file);
    await writeDemoConfig(cfg, file);

    const { env, config } = await loadDemoEnv(file);
    expect(env.source).toBe("demo-config");
    expect(env.rpcUrl).toBe(cfg.rpcUrl);
    expect(env.proofServiceUrl).toBe(cfg.proofServiceUrl);
    expect(env.gatewayRpcUrl).toBe(cfg.gatewayRpcUrl);
    expect(env.chainId).toBe(BigInt(cfg.chainId));
    expect(env.aclProgram).toBe(cfg.aclProgram);
    expect(env.capabilities.freshMints).toBe(false);
    // The vault roots stay in the config object, never leaking into TestEnv.
    expect(config.batchers.deposit.batcher).toBe(cfg.batchers.deposit.batcher);
    expect("vault" in env).toBe(false);
  });
});
