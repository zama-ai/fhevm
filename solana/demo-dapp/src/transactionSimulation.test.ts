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

  test("surfaces the RPC error and program logs", () => {
    expect(() =>
      assertSimulationSucceeded("Shield transaction", {
        err: { InstructionError: [1, { Custom: 6_001n }] },
        logs: ["Program log: rejected", "Program failed"],
      }),
    ).toThrow(
      'Shield transaction failed local simulation: {"InstructionError":[1,{"Custom":"6001"}]}\n' +
        "Program log: rejected\nProgram failed",
    );
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
