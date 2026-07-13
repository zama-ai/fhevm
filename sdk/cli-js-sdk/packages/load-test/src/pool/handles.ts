import {
  createFreshDecryptValues,
  createWalletContextForAccount,
  describeValue,
  encryptValues,
  ensureUserDecryptionDelegation,
  getSetEncryptedFunctionName,
  resolveNetworkConfig,
  serializeValue,
  simulateSetEncryptedValue,
  type ContractWriteRequest,
  type EncryptValue,
  type FheValueType,
  type WalletContext,
} from "@cli-fhevm-sdk/toolkit";
import { availableParallelism } from "node:os";
import { prepareTransactionRequest } from "viem/actions";
import {
  encodeFunctionData,
  formatEther,
  keccak256,
  parseEther,
  type Address,
  type Hex,
  type TransactionRequest,
} from "viem";

import {
  delegateLaneIndex,
  laneAccount,
  laneIndices,
  poolDir,
  type LoadTestEnv,
} from "../env";
import type { FlowKind } from "../relayer/types";
import { logger } from "../shared/logger";
import { isoNow, monotonicNowMs, sleep } from "../shared/time";
import { PoolStore, type PoolItemsWriter } from "./store";
import type { FheHandlePoolItem, PoolMeta } from "./types";

export const HANDLE_POOLS = {
  "public-decrypt": "public-decrypt-handles",
  "user-decrypt": "user-decrypt-handles",
  "delegated-user-decrypt": "delegated-user-decrypt-handles",
} as const;

export type HandlePoolFlow = keyof typeof HANDLE_POOLS;

/** HD address index of the delegate account for delegated pools. */
export const DELEGATE_ACCOUNT_INDEX = 10_000;

/** Minimum lane balance; each handle costs one FHETest setter transaction. */
const MIN_LANE_BALANCE = parseEther("0.001");

export type CreateHandlePoolOptions = Readonly<{
  flow: HandlePoolFlow;
  count: number;
  valueTypes?: readonly FheValueType[];
  /** Parallel wallet lanes (HD accounts); each runs its txs sequentially. */
  lanes?: number;
  /** Local value/encryption preparation concurrency per wallet lane. */
  encryptConcurrency?: number | "auto";
  delegationDurationDays?: number;
  onProgress?: (done: number, total: number) => void;
  signal?: AbortSignal;
}>;

const earliestExpiration = (
  expirations: Readonly<Record<string, string>>,
): string | undefined => {
  const values = Object.values(expirations).map(BigInt);
  if (values.length === 0) return undefined;
  return values.reduce((earliest, value) => value < earliest ? value : earliest).toString();
};

export const mergeDelegationExpirations = (
  committed: Readonly<Record<string, string>> | undefined,
  refreshed: Readonly<Record<string, string>>,
): Record<string, string> => {
  const merged = { ...committed };
  for (const [ownerIndex, expiration] of Object.entries(refreshed)) {
    const previous = merged[ownerIndex];
    if (previous === undefined || BigInt(expiration) > BigInt(previous)) {
      merged[ownerIndex] = expiration;
    }
  }
  return merged;
};

type PreparedHandle = Readonly<{
  index: number;
  type: FheValueType;
  value: EncryptValue;
  handle: Hex;
  request: ContractWriteRequest;
}>;

type BroadcastHandle = Readonly<{
  prepared: PreparedHandle;
  transactionHash: Hex;
}>;

type ReceiptOutcome =
  | Readonly<{ ok: true; broadcast: BroadcastHandle }>
  | Readonly<{ ok: false; error: unknown }>;

const RECEIPT_POLL_INTERVAL_MS = 2_000;
const RECEIPT_TIMEOUT_MS = 30 * 60 * 1000;
const SEND_RETRY_INTERVAL_MS = 2_000;
const SEND_TIMEOUT_MS = 10 * 60 * 1000;

const toTransactionRequest = (request: ContractWriteRequest): TransactionRequest => {
  const write = request as ContractWriteRequest & {
    address?: Address;
    abi?: Parameters<typeof encodeFunctionData>[0]["abi"];
    functionName?: string;
    args?: readonly unknown[];
  };
  if (!write.address || !write.abi || !write.functionName) {
    throw new Error("Contract write request is missing address, ABI, or function name.");
  }
  return {
    account: request.account,
    to: write.address,
    data: encodeFunctionData({
      abi: write.abi,
      functionName: write.functionName,
      args: write.args,
    } as never),
  } as TransactionRequest;
};

const walletContextFor = (env: LoadTestEnv, index: number): WalletContext =>
  createWalletContextForAccount(
    {
      network: env.network,
      relayerUrl: env.relayerUrl,
      rpcUrl: env.rpcUrl,
      contractAddress: env.contractAddress,
    },
    laneAccount(index),
  );

const assertFunded = async (context: WalletContext, label: string): Promise<void> => {
  const balance = await context.publicClient.getBalance({
    address: context.account.address,
  });
  if (balance < MIN_LANE_BALANCE) {
    throw new Error(
      `${label} (${context.account.address}) holds ${formatEther(balance)} ETH; ` +
        `fund it with at least ${formatEther(MIN_LANE_BALANCE)} ETH to create handles.`,
    );
  }
};

const createLimiter = (concurrency: number): (<T>(task: () => Promise<T>) => Promise<T>) => {
  let active = 0;
  const queue: Array<() => void> = [];

  const runNext = (): void => {
    active -= 1;
    queue.shift()?.();
  };

  return <T>(task: () => Promise<T>): Promise<T> =>
    new Promise((resolve, reject) => {
      const run = (): void => {
        active += 1;
        Promise.resolve().then(task).then(resolve, reject).finally(runNext);
      };
      if (active < concurrency) run();
      else queue.push(run);
    });
};

const resolveEncryptConcurrency = (
  option: CreateHandlePoolOptions["encryptConcurrency"],
  count: number,
): number => {
  if (count <= 1) return 1;
  if (option === undefined) return 1;
  if (option === "auto") return Math.max(1, Math.min(availableParallelism(), count));
  return Math.max(1, Math.min(option, count));
};

const prepareHandle = async (options: {
  context: WalletContext;
  index: number;
  type: FheValueType;
  makePublic: boolean;
  onProgress?: (message: string) => void;
}): Promise<PreparedHandle> => {
  const values = createFreshDecryptValues(options.type);
  const value = values[0];
  if (!value) throw new Error("No value to encrypt.");
  options.onProgress?.(`[${options.index.toString()}] value: ${describeValue(value)}`);

  const encrypted = await encryptValues(options.context.fhevm, {
    contractAddress: options.context.contractAddress,
    userAddress: options.context.account.address,
    values,
    onProgress: options.onProgress,
    progressLabel: `[${options.index.toString()}] encrypting`,
  });

  const encryptedValue = encrypted.encryptedValues[0];
  if (!encryptedValue) throw new Error("FHEVM SDK did not return a handle.");
  options.onProgress?.(`[${options.index.toString()}] encrypted handle: ${encryptedValue}`);
  options.onProgress?.(
    `[${options.index.toString()}] simulating FHETest.${getSetEncryptedFunctionName(options.type)}`,
  );

  const request = await simulateSetEncryptedValue(
    {
      account: options.context.account,
      contractAddress: options.context.contractAddress,
      publicClient: options.context.publicClient,
    },
    {
      encryptedValue,
      inputProof: encrypted.inputProof,
      value,
      makePublic: options.makePublic,
    },
  );

  return {
    index: options.index,
    type: options.type,
    value,
    handle: encryptedValue,
    request,
  };
};

const broadcastPreparedHandle = async (options: {
  context: WalletContext;
  prepared: PreparedHandle;
  nonce: number;
  onProgress?: (message: string) => void;
}): Promise<BroadcastHandle> => {
  const signTransaction = options.context.account.signTransaction;
  if (!signTransaction) {
    throw new Error("Handle pool creation requires a local account that can sign transactions.");
  }
  const transactionRequest = toTransactionRequest(options.prepared.request);
  const request = await prepareTransactionRequest(options.context.walletClient, {
    ...transactionRequest,
    nonce: options.nonce,
  } as never);
  const signedTransaction = await signTransaction(request as never);
  const transactionHash = keccak256(signedTransaction);
  options.onProgress?.(
    `[${options.prepared.index.toString()}] Sending transaction nonce ${options.nonce.toString()}: ${transactionHash}`,
  );
  const started = monotonicNowMs();
  for (;;) {
    try {
      await options.context.walletClient.sendRawTransaction({
        serializedTransaction: signedTransaction,
      });
      options.onProgress?.(
        `[${options.prepared.index.toString()}] Broadcast transaction: ${transactionHash}`,
      );
      return { prepared: options.prepared, transactionHash };
    } catch (error) {
      if (isAlreadyKnown(error)) {
        options.onProgress?.(
          `[${options.prepared.index.toString()}] Transaction already known: ${transactionHash}`,
        );
        return { prepared: options.prepared, transactionHash };
      }
      if (!isRequestTimeout(error)) throw error;
      const receipt = await getReceiptIfAvailable(options.context, transactionHash);
      if (receipt) {
        options.onProgress?.(
          `[${options.prepared.index.toString()}] Transaction mined after send timeout: ${transactionHash}`,
        );
        return { prepared: options.prepared, transactionHash };
      }
      if (monotonicNowMs() - started > SEND_TIMEOUT_MS) {
        throw new Error(`Timed out broadcasting transaction: ${transactionHash}`);
      }
      options.onProgress?.(
        `[${options.prepared.index.toString()}] Send timed out; retrying transaction ${transactionHash}`,
      );
      await sleep(SEND_RETRY_INTERVAL_MS);
    }
  }
};

const errorText = (error: unknown): string =>
  error instanceof Error ? `${error.name} ${error.message}` : String(error);

const isRequestTimeout = (error: unknown): boolean => {
  const text = errorText(error).toLowerCase();
  return text.includes("timed out") || text.includes("took too long");
};

const isAlreadyKnown = (error: unknown): boolean => {
  const text = errorText(error).toLowerCase();
  return (
    text.includes("already known") ||
    text.includes("already imported") ||
    text.includes("known transaction")
  );
};

const getReceiptIfAvailable = async (
  context: WalletContext,
  transactionHash: Hex,
) => {
  try {
    return await context.publicClient.getTransactionReceipt({ hash: transactionHash });
  } catch (error) {
    if (isReceiptNotFound(error)) return undefined;
    throw error;
  }
};

const isReceiptNotFound = (error: unknown): boolean =>
  error instanceof Error &&
  (error.name === "TransactionReceiptNotFoundError" ||
    error.message.includes("Transaction receipt with hash") ||
    error.message.includes("could not be found"));

const waitForStoredHandle = async (options: {
  context: WalletContext;
  broadcast: BroadcastHandle;
  onProgress?: (message: string) => void;
}): Promise<BroadcastHandle> => {
  const started = monotonicNowMs();
  options.onProgress?.(
    `[${options.broadcast.prepared.index.toString()}] Waiting for transaction receipt: ${options.broadcast.transactionHash}`,
  );
  let receipt;
  for (;;) {
    try {
      receipt = await options.context.publicClient.getTransactionReceipt({
        hash: options.broadcast.transactionHash,
      });
      break;
    } catch (error) {
      if (!isReceiptNotFound(error)) throw error;
      if (monotonicNowMs() - started > RECEIPT_TIMEOUT_MS) {
        throw new Error(
          `Timed out waiting for transaction receipt: ${options.broadcast.transactionHash}`,
        );
      }
      await sleep(RECEIPT_POLL_INTERVAL_MS);
    }
  }
  if (receipt.status !== "success") {
    throw new Error(`Transaction reverted: ${options.broadcast.transactionHash}`);
  }

  // The encrypted input handle is the value stored by FHETest:
  // setE* calls FHE.fromExternal(input, proof), then stores unwrap(enc).
  // Reading getHandleOf(account,type) here is unsafe once later transactions
  // for the same account/type have been broadcast because the mapping is
  // overwritten by the latest mined setter.
  options.onProgress?.(
    `[${options.broadcast.prepared.index.toString()}] stored ${options.broadcast.prepared.handle}`,
  );

  return options.broadcast;
};

const reflectReceipt = async (
  wait: Promise<BroadcastHandle>,
): Promise<ReceiptOutcome> => {
  try {
    return { ok: true, broadcast: await wait };
  } catch (error) {
    return { ok: false, error };
  }
};

const writeStoredHandle = async (options: {
  writer: PoolItemsWriter<FheHandlePoolItem>;
  ownerIndex: number;
  ownerAddress: Hex;
  makePublic: boolean;
  broadcast: BroadcastHandle;
}): Promise<void> => {
  const item = options.broadcast.prepared;
  await options.writer.write({
    index: item.index,
    type: item.type,
    value: serializeValue(item.value).value,
    handle: item.handle,
    ownerIndex: options.ownerIndex,
    ownerAddress: options.ownerAddress,
    isPublic: options.makePublic,
    transactionHash: options.broadcast.transactionHash,
  });
};

const createLaneHandles = async (options: {
  context: WalletContext;
  ownerIndex: number;
  startIndex: number;
  count: number;
  valueTypes: readonly FheValueType[];
  makePublic: boolean;
  encryptConcurrency: number;
  writer: PoolItemsWriter<FheHandlePoolItem>;
  onProgress?: (message: string) => void;
  reportProgress: () => void;
  signal?: AbortSignal;
}): Promise<void> => {
  const limit = createLimiter(options.encryptConcurrency);
  const prepared = Array.from({ length: options.count }, (_, offset) => {
    const index = options.startIndex + offset;
    const type = options.valueTypes[index % options.valueTypes.length] ?? "uint64";
    return limit(() =>
      prepareHandle({
        context: options.context,
        index,
        type,
        makePublic: options.makePublic,
        onProgress: options.onProgress,
      }),
    );
  });

  const nonceStart = await options.context.publicClient.getTransactionCount({
    address: options.context.account.address,
    blockTag: "pending",
  });
  const broadcasts: Array<Promise<ReceiptOutcome>> = [];
  let broadcastError: unknown;
  try {
    for (let offset = 0; offset < options.count; offset += 1) {
      options.signal?.throwIfAborted();
      const item = await prepared[offset];
      options.signal?.throwIfAborted();
      if (!item) throw new Error("Prepared handle missing.");
      const broadcast = await broadcastPreparedHandle({
        context: options.context,
        prepared: item,
        nonce: nonceStart + offset,
        onProgress: options.onProgress,
      });
      broadcasts.push(
        reflectReceipt(
          waitForStoredHandle({
            context: options.context,
            broadcast,
            onProgress: options.onProgress,
          }).then(async (stored) => {
            await writeStoredHandle({
              writer: options.writer,
              ownerIndex: options.ownerIndex,
              ownerAddress: options.context.account.address,
              makePublic: options.makePublic,
              broadcast: stored,
            });
            options.reportProgress();
            return stored;
          }),
        ),
      );
    }
  } catch (error) {
    broadcastError = error;
  }

  let receiptError: unknown;
  for (let offset = 0; offset < broadcasts.length; offset += 1) {
    const stored = await broadcasts[offset];
    if (!stored) throw new Error("Receipt outcome missing.");
    if (!stored.ok) receiptError ??= stored.error;
  }
  if (broadcastError) throw broadcastError;
  if (receiptError) throw receiptError;
};

/**
 * Creates `count` FHETest handles with known plaintexts, split across wallet
 * lanes. FHETest's per-(account, type) mapping is overwritten by each setter,
 * but previous handles stay decryptable: ACL grants and the public-decrypt
 * flag are applied per handle at set time, so the pool records every handle
 * as it is created.
 *
 * Delegated pools additionally ensure an ACL user-decryption delegation from
 * every owner lane to the shared delegate account.
 */
export const createHandlePool = async (
  env: LoadTestEnv,
  options: CreateHandlePoolOptions,
): Promise<PoolStore<FheHandlePoolItem>> => {
  options.signal?.throwIfAborted();
  const dir = poolDir(env, HANDLE_POOLS[options.flow]);
  const networkConfig = resolveNetworkConfig(env.network);
  const contractAddress = env.contractAddress ?? networkConfig.fheTestAddress;
  const indices = laneIndices(options.lanes ?? 4);
  const valueTypes = options.valueTypes ?? ["uint64"];
  const makePublic = options.flow === "public-decrypt";
  const delegated = options.flow === "delegated-user-decrypt";
  const encryptConcurrency = resolveEncryptConcurrency(
    options.encryptConcurrency,
    options.count,
  );

  const delegateIndex = delegated ? delegateLaneIndex(DELEGATE_ACCOUNT_INDEX) : undefined;
  const delegateAddress =
    delegateIndex !== undefined ? laneAccount(delegateIndex).address : undefined;
  if (delegated && delegateAddress) {
    for (const ownerIndex of indices) {
      if (laneAccount(ownerIndex).address.toLowerCase() === delegateAddress.toLowerCase()) {
        throw new Error("Delegator and delegate accounts must differ.");
      }
    }
  }

  const existing = await PoolStore.openIfExists<FheHandlePoolItem>(dir);
  const initialCount = existing?.meta.count ?? 0;
  const meta: PoolMeta = {
    kind: "fhe-handles",
    flow: options.flow as FlowKind,
    network: env.network,
    contractChainId: env.contractChainId,
    contractAddress,
    createdAt: existing?.meta.createdAt ?? isoNow(),
    count: initialCount,
    ownerIndices: [...new Set([...(existing?.meta.ownerIndices ?? []), ...indices])],
    ...(delegated
      ? {
          delegateIndex,
          delegateAddress,
          delegationExpiration: existing?.meta.delegationExpiration,
          delegationExpirations: existing?.meta.delegationExpirations,
        }
      : {}),
  };
  const store = existing ?? (await PoolStore.create<FheHandlePoolItem>(dir, meta));

  const contexts = indices.map((index) => ({
    index,
    context: walletContextFor(env, index),
  }));
  const delegationContexts = delegated
    ? (meta.ownerIndices ?? indices).map((index) => ({
        index,
        context: walletContextFor(env, index),
      }))
    : contexts;
  for (const lane of delegationContexts) {
    options.signal?.throwIfAborted();
    await assertFunded(lane.context, `Lane ${lane.index.toString()}`);
  }

  const delegationExpirations: Record<string, string> = {
    ...(existing?.meta.delegationExpirations ?? {}),
  };
  if (delegated && delegateAddress) {
    const durationDays = options.delegationDurationDays ?? 30;
    for (const lane of delegationContexts) {
      options.signal?.throwIfAborted();
      const delegation = await ensureUserDecryptionDelegation(lane.context, {
        delegatorContext: lane.context,
        delegatorAddress: lane.context.account.address,
        delegateAddress,
        durationDays,
        onProgress: (message) => logger.info(message),
      });
      delegationExpirations[lane.index.toString()] = delegation.expirationDate;
    }
  }

  const writer = await store.itemsWriter((latest) => {
    const mergedExpirations = mergeDelegationExpirations(
      latest.delegationExpirations,
      delegationExpirations,
    );
    return {
      ...latest,
      ownerIndices: [...new Set([...(latest.ownerIndices ?? []), ...indices])],
      ...(delegated
        ? {
            delegateIndex,
            delegateAddress,
            delegationExpirations: mergedExpirations,
            delegationExpiration: earliestExpiration(mergedExpirations),
          }
        : {}),
    };
  });
  // Rebased while holding the writer lock; avoids stale indices across
  // concurrent producer processes.
  const startIndex = writer.startIndex;
  let done = 0;
  const reportProgress = (): void => {
    done += 1;
    options.onProgress?.(done, options.count);
  };

  // Distribute items across lanes; each lane broadcasts sequentially with
  // explicit nonces, then waits for receipts without blocking later broadcasts.
  if (encryptConcurrency > 1) {
    logger.info(
      `Preparing encrypted handle inputs with concurrency ${encryptConcurrency.toString()} per lane; broadcasts use sequential nonces per lane.`,
    );
  }
  const perLane = Math.ceil(options.count / contexts.length);
  let assigned = 0;
  const settled = await Promise.allSettled(
    contexts.map((lane, laneNumber) => {
      const laneCount = Math.min(perLane, options.count - laneNumber * perLane);
      if (laneCount <= 0) return Promise.resolve();
      const laneStart = startIndex + assigned;
      assigned += laneCount;
      return createLaneHandles({
        context: lane.context,
        ownerIndex: lane.index,
        startIndex: laneStart,
        count: laneCount,
        valueTypes,
        makePublic,
        encryptConcurrency: Math.min(encryptConcurrency, laneCount),
        writer,
        onProgress: (message) => logger.info(message),
        reportProgress,
        signal: options.signal,
      });
    }),
  );
  const generationError = settled.find(
    (result): result is PromiseRejectedResult => result.status === "rejected",
  )?.reason;
  try {
    await writer.close();
  } catch (error) {
    if (generationError) {
      throw new AggregateError(
        [generationError, error],
        "Handle generation and pool commit both failed",
      );
    }
    throw error;
  }
  if (generationError) throw generationError;
  logger.success(
    `${options.flow} handle pool now holds ${(startIndex + done).toString()} handle(s) at ${dir}`,
  );
  return PoolStore.open<FheHandlePoolItem>(dir);
};

/** Refreshes every owner delegation in an existing delegated handle pool. */
export const refreshDelegatedHandlePool = async (
  env: LoadTestEnv,
  options: Readonly<{
    requiredValidUntil: bigint;
    onProgress?: (message: string) => void;
    signal?: AbortSignal;
  }>,
): Promise<void> => {
  const dir = poolDir(env, HANDLE_POOLS["delegated-user-decrypt"]);
  const store = await PoolStore.openIfExists<FheHandlePoolItem>(dir);
  if (!store) throw new Error(`No delegated-user-decrypt handle pool at ${dir}.`);
  const items = await store.loadItems();
  const meta = store.meta;
  if (meta.flow !== "delegated-user-decrypt" || meta.delegateIndex === undefined) {
    throw new Error(`Pool at ${dir} is not a delegated-user-decrypt pool.`);
  }
  const delegate = laneAccount(meta.delegateIndex);
  if (!meta.delegateAddress || delegate.address.toLowerCase() !== meta.delegateAddress.toLowerCase()) {
    throw new Error("Delegated pool delegate identity does not match configured credentials.");
  }

  const owners = new Map<number, string>();
  for (const item of items) {
    const previous = owners.get(item.ownerIndex);
    if (previous && previous.toLowerCase() !== item.ownerAddress.toLowerCase()) {
      throw new Error(`Owner lane ${item.ownerIndex.toString()} has conflicting addresses in the pool.`);
    }
    owners.set(item.ownerIndex, item.ownerAddress);
  }
  const now = BigInt(Math.floor(Date.now() / 1000));
  const secondsNeeded = options.requiredValidUntil > now
    ? options.requiredValidUntil - now
    : 1n;
  // Delegation API uses whole days from the latest block timestamp. One extra
  // day covers wall-clock/block skew at exact day boundaries.
  const durationDays = Number((secondsNeeded + 86_399n) / 86_400n) + 1;
  const delegationExpirations: Record<string, string> = {};
  for (const ownerIndex of meta.ownerIndices ?? []) {
    options.signal?.throwIfAborted();
    const context = walletContextFor(env, ownerIndex);
    const recordedOwnerAddress = owners.get(ownerIndex);
    if (
      recordedOwnerAddress &&
      context.account.address.toLowerCase() !== recordedOwnerAddress.toLowerCase()
    ) {
      throw new Error(
        `Pool owner ${recordedOwnerAddress} does not match lane ${ownerIndex.toString()} account ${context.account.address}.`,
      );
    }
    const delegation = await ensureUserDecryptionDelegation(context, {
      delegatorContext: context,
      delegatorAddress: context.account.address,
      delegateAddress: delegate.address,
      durationDays,
      onProgress: options.onProgress,
    });
    if (BigInt(delegation.expirationDate) < options.requiredValidUntil) {
      throw new Error(
        `Delegation refresh for owner lane ${ownerIndex.toString()} did not reach required expiration ${options.requiredValidUntil.toString()}.`,
      );
    }
    delegationExpirations[ownerIndex.toString()] = delegation.expirationDate;
  }

  await store.updateMeta((latest) => {
    const mergedExpirations = mergeDelegationExpirations(
      latest.delegationExpirations,
      delegationExpirations,
    );
    return {
      ...latest,
      delegationExpirations: mergedExpirations,
      delegationExpiration: earliestExpiration(mergedExpirations),
    };
  });
};
