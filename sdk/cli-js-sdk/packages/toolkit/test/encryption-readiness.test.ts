import { describe, expect, it, vi } from "vitest";

import { encryptValues } from "../src/fhevm/encryption";

const parameters = {
  contractAddress: "0x0000000000000000000000000000000000000001",
  userAddress: "0x0000000000000000000000000000000000000002",
  values: [{ type: "uint64", value: 42n }],
} as const;

describe("encryptValues readiness", () => {
  it("does not start encryption before the SDK client is ready", async () => {
    let resolveReady!: () => void;
    const ready = new Promise<void>((resolve) => {
      resolveReady = resolve;
    });
    const encrypt = vi.fn().mockResolvedValue({
      encryptedValues: ["0x01"],
      inputProof: "0x02",
    });

    const encrypting = encryptValues(
      { ready, encryptValues: encrypt } as never,
      parameters,
    );
    await Promise.resolve();
    expect(encrypt).not.toHaveBeenCalled();

    resolveReady();
    await encrypting;
    expect(encrypt).toHaveBeenCalledTimes(1);
  });

  it("propagates readiness failure before encryption starts", async () => {
    const readinessError = new Error("encrypt runtime unavailable");
    const encrypt = vi.fn();

    await expect(
      encryptValues(
        { ready: Promise.reject(readinessError), encryptValues: encrypt } as never,
        parameters,
      ),
    ).rejects.toBe(readinessError);
    expect(encrypt).not.toHaveBeenCalled();
  });
});
