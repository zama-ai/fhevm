import { describe, expect, test } from "bun:test";

import {
  parseBalanceState,
  parseTransferWorkerResult,
  runSolanaTwoHolderTransfer,
  solanaUserDecryptContext,
  type BalanceState,
  type TwoHolderDependencies,
  type TwoHolderScenario,
} from "./two-holder-transfer";

const hex32 = (byte: string) => `0x${byte.repeat(64)}`;
const alice = { owner: "1".repeat(32), keypairPath: "/alice.json", secretKey: hex32("1") };
const bob = { owner: "2".repeat(32), keypairPath: "/bob.json", secretKey: hex32("2") };
const scenario: TwoHolderScenario = {
  mint: "3".repeat(32),
  computeSigner: "4".repeat(32),
  alice,
  bob,
};
const balance = (owner: string, handleByte: string): BalanceState => ({
  version: 1,
  mint: scenario.mint,
  owner,
  tokenAccount: owner === alice.owner ? "5".repeat(32) : "6".repeat(32),
  encryptedValueAccount: owner === alice.owner ? "7".repeat(32) : "8".repeat(32),
  aclValueKey: hex32("a"),
  currentHandle: hex32(handleByte),
  chainId: "9223372036854788153",
});

describe("solana-two-holder-transfer", () => {
  test("proves initial balances, transfers once, then proves both latest balances", async () => {
    const events: string[] = [];
    let reads = 0;
    const dependencies: TwoHolderDependencies = {
      provision: async () => scenario,
      readBalance: async (_scenario, holder) => {
        const final = reads++ >= 2;
        events.push(`read:${holder.owner}:${final ? "final" : "initial"}`);
        return balance(holder.owner, holder === alice ? (final ? "3" : "1") : final ? "4" : "2");
      },
      waitForHandle: async (handle) => {
        events.push(`wait:${handle.at(-1)}`);
      },
      transfer: async () => {
        events.push("transfer");
      },
      decrypt: async (_scenario, holder, _state, expected) => {
        events.push(`decrypt:${holder.owner}:${expected}`);
        return expected;
      },
      cleanup: async () => {
        events.push("cleanup");
      },
    };

    await runSolanaTwoHolderTransfer(dependencies);

    expect(events.filter((event) => event === "transfer")).toHaveLength(1);
    expect(events).toEqual([
      `read:${alice.owner}:initial`,
      `read:${bob.owner}:initial`,
      "wait:1",
      "wait:2",
      `decrypt:${alice.owner}:1000`,
      `decrypt:${bob.owner}:0`,
      "transfer",
      `read:${alice.owner}:final`,
      `read:${bob.owner}:final`,
      "wait:3",
      "wait:4",
      `decrypt:${alice.owner}:600`,
      `decrypt:${bob.owner}:400`,
      "cleanup",
    ]);
  });

  test("rejects a stale post-transfer handle and still cleans up", async () => {
    let cleaned = false;
    let reads = 0;
    const dependencies: TwoHolderDependencies = {
      provision: async () => scenario,
      readBalance: async (_scenario, holder) => balance(holder.owner, reads++ < 2 ? (holder === alice ? "1" : "2") : "1"),
      waitForHandle: async () => undefined,
      transfer: async () => undefined,
      decrypt: async (_scenario, _holder, _state, expected) => expected,
      cleanup: async () => {
        cleaned = true;
      },
    };
    await expect(runSolanaTwoHolderTransfer(dependencies)).rejects.toThrow("did not rotate both current balance handles");
    expect(cleaned).toBe(true);
  });

  test("strictly validates versioned balance probe identity and high-bit chain id", () => {
    const state = balance(alice.owner, "1");
    expect(parseBalanceState(`${JSON.stringify(state)}\n`, scenario.mint, alice.owner)).toEqual(state);
    expect(() => parseBalanceState(JSON.stringify({ ...state, version: 2 }), scenario.mint, alice.owner)).toThrow(
      "identity or version mismatch",
    );
    expect(() => parseBalanceState(JSON.stringify({ ...state, chainId: "12345" }), scenario.mint, alice.owner)).toThrow(
      "invalid Solana chainId",
    );
    expect(() => parseBalanceState(JSON.stringify({ ...state, extra: true }), scenario.mint, alice.owner)).toThrow(
      "identity or version mismatch",
    );
  });

  test("encodes a decimal user-decrypt context as bytes32", () => {
    expect(solanaUserDecryptContext("1")).toBe(`0x${"0".repeat(63)}1`);
    expect(() => solanaUserDecryptContext("0x01")).toThrow("unsigned decimal integer");
    expect(() => solanaUserDecryptContext((1n << 256n).toString())).toThrow("fit in 32 bytes");
  });

  test("strictly validates the one-shot SDK worker result", () => {
    const result = { version: 1, signature: "9".repeat(88), inputHandle: hex32("1") };
    expect(() => parseTransferWorkerResult(JSON.stringify(result))).not.toThrow();
    expect(() => parseTransferWorkerResult(JSON.stringify({ ...result, extra: true }))).toThrow(
      "malformed versioned JSON",
    );
  });
});
