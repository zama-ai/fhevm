import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  ensureDelegation: vi.fn(),
  createWalletContext: vi.fn(),
}));

vi.mock("@cli-fhevm-sdk/toolkit", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@cli-fhevm-sdk/toolkit")>();
  return {
    ...actual,
    ensureUserDecryptionDelegation: mocks.ensureDelegation,
    createWalletContextForAccount: mocks.createWalletContext,
  };
});

import { laneAccount, poolDir, type LoadTestEnv } from "../src/env";
import {
  DELEGATE_ACCOUNT_INDEX,
  HANDLE_POOLS,
  mergeDelegationExpirations,
  refreshDelegatedHandlePool,
} from "../src/pool/handles";
import { PoolStore } from "../src/pool/store";
import type { FheHandlePoolItem, PoolMeta } from "../src/pool/types";
import { createBuiltinScenario } from "../src/scenario/builtin";
import { planPools } from "../src/suite/requirements";

const MNEMONIC = "test test test test test test test test test test test junk";
let dir: string;
let previousMnemonic: string | undefined;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-delegation-refresh-"));
  previousMnemonic = process.env.MNEMONIC;
  process.env.MNEMONIC = MNEMONIC;
  mocks.ensureDelegation.mockReset().mockResolvedValue({ expirationDate: "5000" });
  mocks.createWalletContext.mockReset().mockImplementation((_options, account) => ({ account }));
});

afterEach(async () => {
  if (previousMnemonic === undefined) delete process.env.MNEMONIC;
  else process.env.MNEMONIC = previousMnemonic;
  await rm(dir, { recursive: true, force: true });
});

describe("refreshDelegatedHandlePool", () => {
  it("does not let a stale producer downgrade expirations or lose a concurrent lane", async () => {
    const pool = await PoolStore.create<FheHandlePoolItem>(join(dir, "delegated-producer"), {
      kind: "fhe-handles",
      flow: "delegated-user-decrypt",
      network: "testnet",
      contractChainId: 11_155_111,
      contractAddress: "0x0000000000000000000000000000000000000001",
      createdAt: "2026-01-01T00:00:00.000Z",
      count: 0,
      ownerIndices: [0],
      delegateIndex: DELEGATE_ACCOUNT_INDEX,
      delegateAddress: laneAccount(DELEGATE_ACCOUNT_INDEX).address,
      delegationExpirations: { "0": "5000" },
      delegationExpiration: "5000",
    });
    // This is the delegation result a producer obtained before the concurrent
    // metadata commit, but will publish only after rebasing under the lock.
    const staleProducerExpirations = { "0": "6000" };
    await pool.updateMeta((latest) => ({
      ...latest,
      ownerIndices: [...(latest.ownerIndices ?? []), 1],
      delegationExpirations: { "0": "7000", "1": "7000" },
      delegationExpiration: "7000",
    }));

    const staleWriter = await pool.itemsWriter((latest) => {
      const merged = mergeDelegationExpirations(
        latest.delegationExpirations,
        staleProducerExpirations,
      );
      return {
        ...latest,
        ownerIndices: [...new Set([...(latest.ownerIndices ?? []), 0])],
        delegationExpirations: merged,
        delegationExpiration: Object.values(merged)
          .map(BigInt)
          .reduce((earliest, value) => value < earliest ? value : earliest)
          .toString(),
      };
    });
    await staleWriter.close();

    const reopened = await PoolStore.open<FheHandlePoolItem>(pool.dir);
    expect(reopened.meta.ownerIndices).toEqual([0, 1]);
    expect(reopened.meta.delegationExpirations).toEqual({ "0": "7000", "1": "7000" });
    expect(reopened.meta.delegationExpiration).toBe("7000");
  });

  it("refreshes every declared owner lane when item count is below lane count", async () => {
    const env: LoadTestEnv = {
      network: "testnet",
      relayerUrl: "https://relayer.example",
      contractChainId: 11_155_111,
      dataDir: dir,
    };
    const delegate = laneAccount(DELEGATE_ACCOUNT_INDEX);
    const owner = laneAccount(0);
    const meta: PoolMeta = {
      kind: "fhe-handles",
      flow: "delegated-user-decrypt",
      network: "testnet",
      contractChainId: env.contractChainId,
      contractAddress: "0x0000000000000000000000000000000000000001",
      createdAt: "2026-01-01T00:00:00.000Z",
      count: 0,
      ownerIndices: [0, 1, 2, 3],
      delegateIndex: DELEGATE_ACCOUNT_INDEX,
      delegateAddress: delegate.address,
      delegationExpirations: { "0": "5000", "1": "5000", "2": "5000" },
      delegationExpiration: "5000",
    };
    const store = await PoolStore.create<FheHandlePoolItem>(
      poolDir(env, HANDLE_POOLS["delegated-user-decrypt"]),
      meta,
    );
    const writer = await store.itemsWriter();
    await writer.write({
      index: 0,
      type: "uint64",
      value: "42",
      handle: `0x${"01".repeat(32)}`,
      ownerIndex: 0,
      ownerAddress: owner.address,
      isPublic: false,
      transactionHash: `0x${"02".repeat(32)}`,
    });
    await writer.close();

    const scenario = createBuiltinScenario("open-steady", {
      flow: "delegated-user-decrypt",
      rps: 1,
      durationSec: 1,
    });
    const plan = await planPools(env, [scenario], { nowSeconds: 1_000n });
    expect(plan.find((item) => item.flow === "delegated-user-decrypt")?.refreshRequired)
      .toBe(true);

    await refreshDelegatedHandlePool(env, { requiredValidUntil: 4_000n });

    expect(mocks.ensureDelegation).toHaveBeenCalledTimes(4);
    expect((await PoolStore.open<FheHandlePoolItem>(store.dir)).meta.delegationExpirations)
      .toEqual({ "0": "5000", "1": "5000", "2": "5000", "3": "5000" });
  });

  it("does not let a stale concurrent refresh downgrade a committed expiration", async () => {
    const env: LoadTestEnv = {
      network: "testnet",
      relayerUrl: "https://relayer.example",
      contractChainId: 11_155_111,
      dataDir: dir,
    };
    const delegate = laneAccount(DELEGATE_ACCOUNT_INDEX);
    await PoolStore.create<FheHandlePoolItem>(
      poolDir(env, HANDLE_POOLS["delegated-user-decrypt"]),
      {
        kind: "fhe-handles",
        flow: "delegated-user-decrypt",
        network: "testnet",
        contractChainId: env.contractChainId,
        contractAddress: "0x0000000000000000000000000000000000000001",
        createdAt: "2026-01-01T00:00:00.000Z",
        count: 0,
        ownerIndices: [0],
        delegateIndex: DELEGATE_ACCOUNT_INDEX,
        delegateAddress: delegate.address,
        delegationExpirations: { "0": "4500" },
        delegationExpiration: "4500",
      },
    );

    let resolveNewer!: (value: { expirationDate: string }) => void;
    let resolveStale!: (value: { expirationDate: string }) => void;
    mocks.ensureDelegation
      .mockImplementationOnce(() => new Promise((resolve) => { resolveNewer = resolve; }))
      .mockImplementationOnce(() => new Promise((resolve) => { resolveStale = resolve; }));

    const newerRefresh = refreshDelegatedHandlePool(env, { requiredValidUntil: 4_000n });
    const staleRefresh = refreshDelegatedHandlePool(env, { requiredValidUntil: 4_000n });
    await vi.waitFor(() => expect(mocks.ensureDelegation).toHaveBeenCalledTimes(2));

    const concurrentStore = await PoolStore.open<FheHandlePoolItem>(
      poolDir(env, HANDLE_POOLS["delegated-user-decrypt"]),
    );
    await concurrentStore.updateMeta((latest) => ({
      ...latest,
      ownerIndices: [...(latest.ownerIndices ?? []), 1],
      delegationExpirations: {
        ...(latest.delegationExpirations ?? {}),
        "1": "7000",
      },
      delegationExpiration: "4500",
    }));

    resolveNewer({ expirationDate: "6000" });
    await newerRefresh;
    resolveStale({ expirationDate: "5000" });
    await staleRefresh;

    const reopened = await PoolStore.open<FheHandlePoolItem>(
      poolDir(env, HANDLE_POOLS["delegated-user-decrypt"]),
    );
    expect(reopened.meta.ownerIndices).toEqual([0, 1]);
    expect(reopened.meta.delegationExpirations).toEqual({ "0": "6000", "1": "7000" });
    expect(reopened.meta.delegationExpiration).toBe("6000");
  });
});
