import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  createWalletContext: vi.fn(),
  ensureDelegation: vi.fn(),
}));

vi.mock("@cli-fhevm-sdk/toolkit", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@cli-fhevm-sdk/toolkit")>();
  return {
    ...actual,
    createWalletContextForAccount: mocks.createWalletContext,
    ensureUserDecryptionDelegation: mocks.ensureDelegation,
  };
});

import type { LoadTestEnv } from "../src/env";
import { createHandlePool } from "../src/pool/handles";

const MNEMONIC = "test test test test test test test test test test test junk";
let dir: string;
let previousMnemonic: string | undefined;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-handle-ready-"));
  previousMnemonic = process.env.MNEMONIC;
  process.env.MNEMONIC = MNEMONIC;
  vi.clearAllMocks();
});

afterEach(async () => {
  if (previousMnemonic === undefined) delete process.env.MNEMONIC;
  else process.env.MNEMONIC = previousMnemonic;
  await rm(dir, { recursive: true, force: true });
});

describe("delegated handle-pool readiness", () => {
  it("fails before ACL delegation mutation when encryption readiness fails", async () => {
    const readinessError = new Error("handle encrypt runtime unavailable");
    let rejectReady!: (reason?: unknown) => void;
    const ready = new Promise<void>((_resolve, reject) => {
      rejectReady = reject;
    });
    const getBalance = vi.fn().mockResolvedValue(10n ** 18n);
    mocks.createWalletContext.mockImplementation((_options, account) => ({
      account,
      fhevm: { ready },
      publicClient: { getBalance },
    }));
    const env: LoadTestEnv = {
      network: "testnet",
      relayerUrl: "https://relayer.example",
      contractChainId: 11_155_111,
      dataDir: dir,
    };

    const creating = createHandlePool(env, {
      flow: "delegated-user-decrypt",
      count: 1,
      lanes: 1,
    });
    await vi.waitFor(() => expect(getBalance).toHaveBeenCalledTimes(1));
    await Promise.resolve();
    rejectReady(readinessError);

    await expect(creating).rejects.toBe(readinessError);
    expect(mocks.ensureDelegation).not.toHaveBeenCalled();
  });
});
