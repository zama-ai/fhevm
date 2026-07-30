import { resolveNetworkConfig, type FheValueType } from "@cli-fhevm-sdk/toolkit";
import { availableParallelism } from "node:os";
import { fileURLToPath } from "node:url";
import { Piscina } from "piscina";
import type { Hex } from "viem";

import { poolDir, type LoadTestEnv } from "../env";
import { logger } from "../shared/logger";
import { isoNow } from "../shared/time";
import { PoolStore } from "./store";
import type { InputProofPoolItem, PoolMeta } from "./types";
import type { InputProofWorkerTask } from "./workers/input-proof-worker";

export const INPUT_PROOF_POOL = "input-proof";

/** Default user address mirrors the toolkit's pure input-proof flow. */
const DEFAULT_USER_ADDRESS: Hex = "0x0000000000000000000000000000000000000002";

export type GenerateInputProofPoolOptions = Readonly<{
  count: number;
  valueTypes?: readonly FheValueType[];
  userAddress?: Hex;
  /** Worker threads; defaults to piscina's core-count heuristic. */
  threads?: number;
  onProgress?: (done: number, total: number) => void;
  signal?: AbortSignal;
}>;

/**
 * Generates `count` unique input-proof payloads into the pool, appending to
 * any existing items (indices continue). Payloads are single-use; consumption
 * is tracked by the pool cursor, never by deleting items.
 */
export const generateInputProofPool = async (
  env: LoadTestEnv,
  options: GenerateInputProofPoolOptions,
): Promise<PoolStore<InputProofPoolItem>> => {
  options.signal?.throwIfAborted();
  const dir = poolDir(env, INPUT_PROOF_POOL);
  const networkConfig = resolveNetworkConfig(env.network);
  const contractAddress = env.contractAddress ?? networkConfig.fheTestAddress;

  const existing = await PoolStore.openIfExists<InputProofPoolItem>(dir);
  const meta: PoolMeta = existing?.meta ?? {
    kind: "input-proof-payloads",
    flow: "input-proof",
    network: env.network,
    contractChainId: env.contractChainId,
    contractAddress,
    relayerUrl: env.relayerUrl,
    createdAt: isoNow(),
    count: 0,
  };
  const store =
    existing ?? (await PoolStore.create<InputProofPoolItem>(dir, meta));

  // TFHE runs single-threaded inside each worker (toolkit sets the SDK's
  // `singleThread: true`, so no nested wasm thread pools); parallelism is
  // one wasm instance per piscina thread. Piscina's default oversubscribes
  // (~1.5x cores) — wrong for CPU-bound proof generation and each thread
  // pays its own wasm + key-material memory, so default to cores - 1.
  const piscina = new Piscina({
    filename: fileURLToPath(new URL("./workers/input-proof-worker.ts", import.meta.url)),
    maxThreads: options.threads ?? Math.max(1, availableParallelism() - 1),
  });
  const writer = await store.itemsWriter();
  // The writer rebases under the cross-process lock; its reservation is the
  // only safe source of indices when another producer may have appended.
  const startIndex = writer.startIndex;
  const valueTypes = options.valueTypes ?? ["uint64"];
  let done = 0;

  try {
    const tasks = Array.from({ length: options.count }, (_, offset) => {
      const task: InputProofWorkerTask = {
        index: startIndex + offset,
        network: env.network,
        relayerUrl: env.relayerUrl,
        rpcUrl: env.rpcUrl,
        contractAddress: env.contractAddress,
        userAddress: options.userAddress ?? DEFAULT_USER_ADDRESS,
        contractChainId: env.contractChainId,
        valueTypes,
      };
      return piscina.run(task).then(async (item: InputProofPoolItem) => {
        options.signal?.throwIfAborted();
        await writer.write(item);
        done += 1;
        options.onProgress?.(done, options.count);
      });
    });
    await Promise.all(tasks);
  } finally {
    try {
      await writer.close();
    } finally {
      await piscina.destroy();
    }
  }

  await store.updateCount(startIndex + done);
  logger.success(
    `Input-proof pool now holds ${(startIndex + done).toString()} payload(s) at ${dir}`,
  );
  return PoolStore.open<InputProofPoolItem>(dir);
};
