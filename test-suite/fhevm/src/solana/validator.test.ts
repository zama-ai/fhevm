import { describe, expect, test } from "bun:test";

import { isLifecycleLedgerPath, renderGeyserConfig, validatorStartArgs } from "./validator";

describe("renderGeyserConfig", () => {
  test("substitutes the plugin cdylib path into the committed template shape", () => {
    const rendered = renderGeyserConfig('{"libpath": "@LIBPATH@"}', "/tmp/libyellowstone.so");
    expect(rendered).toBe('{"libpath": "/tmp/libyellowstone.so"}');
  });
});

describe("validatorStartArgs", () => {
  test("pins loopback bind, the sbpf deployment feature deactivation, and the geyser config", () => {
    const args = validatorStartArgs({ ledgerDir: "/tmp/ledger", geyserConfigPath: "/tmp/geyser.json" });
    expect(args[0]).toBe("solana-test-validator");
    // agave 4.x advertises --bind-address as the gossip IP and rejects 0.0.0.0; loopback is safe
    // because the RPC/pubsub listeners bind 0.0.0.0 regardless.
    expect(args).toContain("--bind-address");
    expect(args[args.indexOf("--bind-address") + 1]).toBe("127.0.0.1");
    // Matches mainnet, which has no disable_sbpf_v0_v1_v2_deployment feature account.
    expect(args[args.indexOf("--deactivate-feature") + 1]).toBe("B8JJXCy5amZyWG9r7EnUYLwzXSXTxG7GZ1qZ1qggo83g");
    expect(args[args.indexOf("--ledger") + 1]).toBe("/tmp/ledger");
    expect(args[args.indexOf("--geyser-plugin-config") + 1]).toBe("/tmp/geyser.json");
    expect(args).toContain("--reset");
  });
});

describe("isLifecycleLedgerPath", () => {
  test("accepts only the short owned per-boot ledger shape", () => {
    expect(isLifecycleLedgerPath(`/tmp/fhevm-demo-501/${"a".repeat(24)}.ledger`, 501)).toBe(true);
    expect(isLifecycleLedgerPath(`/tmp/fhevm-demo-502/${"a".repeat(24)}.ledger`, 501)).toBe(false);
    expect(isLifecycleLedgerPath("/tmp/fhevm-demo-501/short.ledger", 501)).toBe(false);
    expect(isLifecycleLedgerPath(`/home/fhevm-demo-501/${"a".repeat(24)}.ledger`, 501)).toBe(false);
    expect(isLifecycleLedgerPath(`/tmp/fhevm-demo-501/${"a".repeat(24)}.ledger/../x`, 501)).toBe(false);
  });
});
