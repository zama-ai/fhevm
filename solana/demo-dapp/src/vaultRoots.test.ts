import { describe, expect, test } from "vitest";

import type { DemoConfig } from "./demoSession";
import { vaultRoots } from "./vaultRoots";

const config = {
  programs: { batcher: "batcher-program", token: "token-program", vault: "vault-program", host: "host-program" },
  batchers: {
    deposit: { batcher: "deposit-batcher", lookupTable: "deposit-table" },
    redeem: { batcher: "redeem-batcher", lookupTable: "redeem-table" },
  },
  mints: {
    joinUnderlying: "usdc",
    payoutUnderlying: "shares",
    joinConfidential: "cusdc",
    payoutConfidential: "cshares",
  },
  vault: "vault",
  hostConfig: "host-config",
  kmsContext: "kms-context",
} as unknown as DemoConfig;

describe("vaultRoots", () => {
  test("keeps the deposit direction", () => {
    expect(vaultRoots(config, "deposit")).toMatchObject({
      batcher: "deposit-batcher",
      joinConfidentialMint: "cusdc",
      payoutConfidentialMint: "cshares",
      joinUnderlyingMint: "usdc",
      payoutUnderlyingMint: "shares",
    });
  });

  test("swaps both confidential and underlying roots for redemption", () => {
    expect(vaultRoots(config, "redeem")).toMatchObject({
      batcher: "redeem-batcher",
      joinConfidentialMint: "cshares",
      payoutConfidentialMint: "cusdc",
      joinUnderlyingMint: "shares",
      payoutUnderlyingMint: "usdc",
    });
  });
});
