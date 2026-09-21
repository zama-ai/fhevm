import type { Transaction } from "@solana/kit";
import { describe, expect, test, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  encode: vi.fn(() => "wire-transaction"),
}));

vi.mock("@solana/kit", () => ({
  createSolanaRpc: vi.fn(),
  getBase64EncodedWireTransaction: mocks.encode,
}));

import {
  assertSimulationSucceeded,
  simulateSignedTransactionLocally,
  simulateUnsignedTransactionLocally,
} from "./transactionSimulation";

describe("assertSimulationSucceeded", () => {
  test("accepts a successful simulation", () => {
    expect(() => assertSimulationSucceeded("Shield transaction", { err: null })).not.toThrow();
  });

  test("surfaces host IDL errors instead of the raw simulation payload", () => {
    expect(() =>
      assertSimulationSucceeded("Shield transaction", {
        err: { InstructionError: [1, { Custom: 6080 }] },
        logs: [
          "Program log: AnchorError caused by account: transient_store. Error Code: TransientStoreNotOpened. Error Number: 6080. Error Message: transient store must be opened for this transaction and closed last.",
        ],
      }),
    ).toThrow(/TransientStoreNotOpened \(6080\)/);
  });

  test("surfaces the RPC error and program logs", () => {
    expect(() =>
      assertSimulationSucceeded("Shield transaction", {
        err: { InstructionError: [1, { Custom: 6_001n }] },
        logs: ["Program log: rejected", "Program failed"],
      }),
    ).toThrow(/HostConfigPaused \(6001\)/);
  });

  test("reports failures that have no logs", () => {
    expect(() =>
      assertSimulationSucceeded("Claim transaction", {
        err: "BlockhashNotFound",
        logs: null,
      }),
    ).toThrow('Claim transaction failed local simulation: "BlockhashNotFound"');
  });
});

describe("local transaction simulation", () => {
  const transaction = {} as Transaction;

  test.each([
    ["unsigned", simulateUnsignedTransactionLocally, false],
    ["signed", simulateSignedTransactionLocally, true],
  ] as const)("simulates a %s transaction with the expected signature policy", async (_kind, simulate, sigVerify) => {
    const send = vi.fn().mockResolvedValue({ value: { err: null } });
    const simulateTransaction = vi.fn(() => ({ send }));
    const rpc = { simulateTransaction };

    await simulate(rpc as never, transaction, "Transaction");

    expect(mocks.encode).toHaveBeenCalledWith(transaction);
    expect(simulateTransaction).toHaveBeenCalledWith("wire-transaction", {
      commitment: "confirmed",
      encoding: "base64",
      sigVerify,
    });
    expect(send).toHaveBeenCalledOnce();
  });
});
