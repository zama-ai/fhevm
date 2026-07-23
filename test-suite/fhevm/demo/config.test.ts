import { afterEach, describe, expect, test } from "bun:test";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { address, getAddressDecoder } from "@solana/kit";

import {
  depositRoots,
  parseDemoConfig,
  readDemoConfig,
  redeemRoots,
  writeDemoConfig,
  type SolanaDemoConfig,
} from "./config";

const addr = (fill: number): string => getAddressDecoder().decode(new Uint8Array(32).fill(fill));

const sampleConfig = (): SolanaDemoConfig => ({
  source: "demo-config",
  chainId: "9223372036854788153",
  rpcUrl: "http://127.0.0.1:8899",
  wsUrl: "ws://127.0.0.1:8900",
  relayerUrl: "http://127.0.0.1:3000",
  proofServiceUrl: "http://127.0.0.1:8088",
  gatewayRpcUrl: "http://127.0.0.1:8546",
  aclProgram: `0x${"ab".repeat(32)}`,
  userDecryptContextId: "42",
  authorityFundingLamports: "100000000",
  programs: { batcher: address(addr(30)), token: address(addr(31)), vault: address(addr(32)), host: address(addr(33)) },
  hostConfig: address(addr(8)),
  kmsContext: address(addr(9)),
  vault: address(addr(10)),
  mints: {
    joinUnderlying: address(addr(5)),
    payoutUnderlying: address(addr(14)),
    joinConfidential: address(addr(4)),
    payoutConfidential: address(addr(13)),
  },
  batchers: {
    deposit: { batcher: address(addr(2)), lookupTable: address(addr(200)) },
    redeem: { batcher: address(addr(3)), lookupTable: address(addr(201)) },
  },
  mintAuthority: address(addr(50)),
  personas: { keeper: address(addr(60)), alice: address(addr(61)), bob: address(addr(62)) },
});

const tmpFiles: string[] = [];
afterEach(async () => {
  for (const f of tmpFiles.splice(0)) await fs.rm(f, { force: true });
});

describe("demo-config parse", () => {
  test("round-trips through disk unchanged", async () => {
    const config = sampleConfig();
    const file = path.join(await fs.mkdtemp(path.join(os.tmpdir(), "demo-config-")), "solana-demo.json");
    tmpFiles.push(file);
    await writeDemoConfig(config, file);
    expect(await readDemoConfig(file)).toEqual(config);
  });

  test("rejects a malformed field with the field named", () => {
    const broken = { ...sampleConfig(), hostConfig: "not-base58!!" } as unknown;
    expect(() => parseDemoConfig(broken)).toThrow(/hostConfig/);
  });

  test("rejects a chain id that is not decimal", () => {
    const broken = { ...sampleConfig(), chainId: "0xdeadbeef" } as unknown;
    expect(() => parseDemoConfig(broken)).toThrow(/chainId/);
  });

  test("rejects a missing nested object with its path", () => {
    const { batchers: _drop, ...rest } = sampleConfig();
    expect(() => parseDemoConfig(rest as unknown)).toThrow(/batchers/);
  });
});

describe("direction roots projection", () => {
  test("deposit joins with cUSDC and pays out cShares", () => {
    const config = sampleConfig();
    const roots = depositRoots(config);
    expect(roots.batcher).toBe(config.batchers.deposit.batcher);
    expect(roots.joinConfidentialMint).toBe(config.mints.joinConfidential);
    expect(roots.payoutConfidentialMint).toBe(config.mints.payoutConfidential);
    expect(roots.joinUnderlyingMint).toBe(config.mints.joinUnderlying);
    expect(roots.payoutUnderlyingMint).toBe(config.mints.payoutUnderlying);
  });

  test("redeem is the deposit direction with join/payout swapped (review focus #6)", () => {
    const config = sampleConfig();
    const deposit = depositRoots(config);
    const redeem = redeemRoots(config);
    // Same batcher program + vault + host/kms context; different batcher instance.
    expect(redeem.batcherProgram).toBe(deposit.batcherProgram);
    expect(redeem.vault).toBe(deposit.vault);
    expect(redeem.batcher).toBe(config.batchers.redeem.batcher);
    expect(redeem.batcher).not.toBe(deposit.batcher);
    // The two directions mirror each other exactly on the mint pairs.
    expect(redeem.joinConfidentialMint).toBe(deposit.payoutConfidentialMint);
    expect(redeem.payoutConfidentialMint).toBe(deposit.joinConfidentialMint);
    expect(redeem.joinUnderlyingMint).toBe(deposit.payoutUnderlyingMint);
    expect(redeem.payoutUnderlyingMint).toBe(deposit.joinUnderlyingMint);
  });
});
