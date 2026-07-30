import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({ sendAndWait: vi.fn() }));

vi.mock("../src/shared/transactions", () => ({
  sendAndWait: mocks.sendAndWait,
}));

import { ensureUserDecryptionDelegation } from "../src/acl/delegation";

const address = (suffix: string) =>
  `0x${suffix.padStart(40, "0")}` as `0x${string}`;

describe("ensureUserDecryptionDelegation", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.sendAndWait.mockResolvedValue(`0x${"ab".repeat(32)}`);
  });

  it("extends an active delegation that does not cover the requested duration", async () => {
    const publicClient = {
      readContract: vi.fn().mockResolvedValue(1_100n),
      getBlock: vi.fn().mockResolvedValue({ timestamp: 1_000n }),
      simulateContract: vi.fn().mockResolvedValue({ request: { data: "0x" } }),
    };
    const context = {
      chain: { fhevm: { contracts: { acl: { address: address("1") } } } },
      contractAddress: address("2"),
      account: { address: address("3") },
      publicClient,
      walletClient: {},
    } as never;

    const result = await ensureUserDecryptionDelegation(context, {
      delegatorContext: context,
      delegatorAddress: address("3"),
      delegateAddress: address("4"),
      durationDays: 1,
    });

    expect(publicClient.simulateContract).toHaveBeenCalledWith(
      expect.objectContaining({ args: [address("4"), address("2"), 87_400n] }),
    );
    expect(result.expirationDate).toBe("87400");
  });

  it("rejects fractional durations before chain access", async () => {
    const publicClient = { readContract: vi.fn() };
    const context = { publicClient } as never;
    await expect(ensureUserDecryptionDelegation(context, {
      delegatorAddress: address("3"),
      delegateAddress: address("4"),
      durationDays: 0.5,
    })).rejects.toThrow("positive safe integer");
    expect(publicClient.readContract).not.toHaveBeenCalled();
  });
});
