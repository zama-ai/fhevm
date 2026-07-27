import { address } from "@solana/kit";
import { beforeEach, describe, expect, test, vi } from "vitest";

import type { DemoSession } from "./demoSession";
import {
  reconcileDepositTransaction,
  reconcileSavedDeposit,
  type StoredDeposit,
  usdcToBaseUnits,
} from "./deposit";

const user = address("SysvarRent111111111111111111111111111111111");
const batch = address("SysvarC1ock11111111111111111111111111111111");
const session = {
  config: { chainId: "2147483648" },
  signer: { address: user },
} as unknown as DemoSession;
const activeDepositKey = `fhevm-solana-demo:active-deposit:${session.config.chainId}:${user}`;

const storedDeposit = (transaction = true): StoredDeposit => ({
  batchIndex: 8n,
  batch,
  amountBaseUnits: 100_000_000n,
  ...(transaction
    ? {
        transaction: {
          signature: "signature",
          blockhash: "blockhash",
          lastValidBlockHeight: "100",
        },
      }
    : {}),
});

const rpcWith = (
  account: unknown,
  status: unknown,
  blockHeight = 90n,
): Parameters<typeof reconcileSavedDeposit>[0] =>
  ({
    getAccountInfo: vi.fn(() => ({ send: vi.fn().mockResolvedValue({ value: account }) })),
    getSignatureStatuses: vi.fn(() => ({
      send: vi.fn().mockResolvedValue({ value: [status] }),
    })),
    getBlockHeight: vi.fn(() => ({ send: vi.fn().mockResolvedValue(blockHeight) })),
  }) as unknown as Parameters<typeof reconcileSavedDeposit>[0];

beforeEach(() => {
  const values = new Map<string, string>();
  Object.defineProperty(globalThis, "localStorage", {
    configurable: true,
    value: {
      clear: () => values.clear(),
      getItem: (key: string) => values.get(key) ?? null,
      key: (index: number) => [...values.keys()][index] ?? null,
      get length() {
        return values.size;
      },
      removeItem: (key: string) => values.delete(key),
      setItem: (key: string, value: string) => values.set(key, value),
    } satisfies Storage,
  });
});

describe("usdcToBaseUnits", () => {
  test("converts the six-decimal demo asset exactly", () => {
    expect(usdcToBaseUnits(100)).toBe(100_000_000n);
    expect(usdcToBaseUnits(0.000001)).toBe(1n);
  });

  test("rejects zero and amounts above the funded demo balance", () => {
    expect(() => usdcToBaseUnits(0)).toThrow("between 0 and 1,000");
    expect(() => usdcToBaseUnits(1_000.000001)).toThrow("between 0 and 1,000");
  });
});

describe("deposit join recovery", () => {
  test("keeps a successful signature pending until its join record is visible", async () => {
    const rpc = rpcWith(null, { err: null, confirmationStatus: "confirmed" });
    await expect(reconcileDepositTransaction(rpc, storedDeposit().transaction!)).resolves.toBe("pending");
    expect(rpc.getBlockHeight).not.toHaveBeenCalled();
  });

  test("retries failed and expired signatures", async () => {
    const failedRpc = rpcWith(null, { err: { InstructionError: [0, "Custom"] } });
    await expect(reconcileDepositTransaction(failedRpc, storedDeposit().transaction!)).resolves.toBe("retry");

    const expiredRpc = rpcWith(null, null, 101n);
    await expect(reconcileDepositTransaction(expiredRpc, storedDeposit().transaction!)).resolves.toBe("retry");
  });

  test("waits for an unexpired signature that is not visible yet", async () => {
    const rpc = rpcWith(null, null, 100n);
    await expect(reconcileDepositTransaction(rpc, storedDeposit().transaction!)).resolves.toBe("pending");
  });

  test("promotes a visible join record and removes only transaction metadata", async () => {
    localStorage.setItem(activeDepositKey, "old journal");
    const saved = storedDeposit();
    await expect(reconcileSavedDeposit(rpcWith({}, null), session, saved)).resolves.toEqual({
      batchIndex: saved.batchIndex,
      batch: saved.batch,
      amountBaseUnits: saved.amountBaseUnits,
    });
    expect(JSON.parse(localStorage.getItem(activeDepositKey)!)).toEqual({
      batchIndex: "8",
      batch,
      amountBaseUnits: "100000000",
    });
  });

  test("clears a failed signed join so a new proof may be built", async () => {
    localStorage.setItem(activeDepositKey, "old journal");
    await expect(
      reconcileSavedDeposit(rpcWith(null, { err: { InstructionError: [0, "Custom"] } }), session, storedDeposit()),
    ).resolves.toBeNull();
    expect(localStorage.getItem(activeDepositKey)).toBeNull();
  });

  test("preserves the signed join journal on transient RPC errors", async () => {
    const journal = JSON.stringify({ durable: true });
    localStorage.setItem(activeDepositKey, journal);
    const rpc = rpcWith(null, null);
    vi.mocked(rpc.getSignatureStatuses).mockReturnValue({
      send: vi.fn().mockRejectedValue(new Error("RPC unavailable")),
    } as unknown as ReturnType<typeof rpc.getSignatureStatuses>);

    await expect(reconcileSavedDeposit(rpc, session, storedDeposit())).rejects.toThrow("RPC unavailable");
    expect(localStorage.getItem(activeDepositKey)).toBe(journal);
  });

  test("preserves the journal when the join-record lookup fails transiently", async () => {
    const journal = JSON.stringify({ durable: true });
    localStorage.setItem(activeDepositKey, journal);
    const rpc = rpcWith(null, null);
    vi.mocked(rpc.getAccountInfo).mockReturnValue({
      send: vi.fn().mockRejectedValue(new Error("account lookup unavailable")),
    } as unknown as ReturnType<typeof rpc.getAccountInfo>);

    await expect(reconcileSavedDeposit(rpc, session, storedDeposit())).rejects.toThrow(
      "account lookup unavailable",
    );
    expect(localStorage.getItem(activeDepositKey)).toBe(journal);
  });
});
