import {
  createClientContext,
  createRandomValue,
  serializeValue,
  type ClientContext,
  type FheValueType,
  type NetworkName,
} from "@cli-fhevm-sdk/toolkit";
import { generateZkProof } from "@fhevm/sdk/actions/encrypt";
import type { Hex } from "viem";

import type { InputProofPoolItem } from "../types";

/**
 * Piscina worker generating one input-proof payload per task. TFHE encryption
 * and ZK proof generation are CPU-bound; running them here keeps the
 * generation pool off the main thread. The SDK runtime (wasm + fetched key
 * material) is initialized once per thread and reused.
 */

export type InputProofWorkerTask = Readonly<{
  index: number;
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
  contractAddress?: Hex;
  userAddress: Hex;
  contractChainId: number;
  valueTypes: readonly FheValueType[];
}>;

type InputProofWorkerDependencies = Readonly<{
  createContext: typeof createClientContext;
  generateProof: typeof generateZkProof;
}>;

const defaultDependencies: InputProofWorkerDependencies = {
  createContext: createClientContext,
  generateProof: generateZkProof,
};

/** Creates one worker-local handler with an SDK context cache. */
export const createInputProofWorker = (
  dependencies: InputProofWorkerDependencies = defaultDependencies,
): ((task: InputProofWorkerTask) => Promise<InputProofPoolItem>) => {
  let cachedContext: ClientContext | undefined;
  let cachedKey: string | undefined;

  const contextFor = (task: InputProofWorkerTask): ClientContext => {
    const key = JSON.stringify([task.network, task.relayerUrl, task.rpcUrl, task.contractAddress]);
    if (!cachedContext || cachedKey !== key) {
      cachedContext = dependencies.createContext({
        network: task.network,
        relayerUrl: task.relayerUrl,
        rpcUrl: task.rpcUrl,
        contractAddress: task.contractAddress,
      });
      cachedKey = key;
    }
    return cachedContext;
  };

  return async (task: InputProofWorkerTask): Promise<InputProofPoolItem> => {
    const context = contextFor(task);
    await context.fhevm.ready;

    const values = task.valueTypes.map((type) => createRandomValue(type));
    const proof = await dependencies.generateProof(context.fhevm, {
      contractAddress: context.contractAddress,
      userAddress: task.userAddress,
      values,
    });

    return {
      index: task.index,
      contractChainId: task.contractChainId,
      contractAddress: context.contractAddress,
      userAddress: task.userAddress,
      ciphertextWithInputVerification: Buffer.from(proof.ciphertextWithZkProof).toString("hex"),
      extraData: proof.getExtraData(),
      expectedHandles: proof.getInputHandles().map((handle) => handle.bytes32Hex),
      values: values.map((value) => ({
        type: value.type,
        value: serializeValue(value).value,
      })),
    };
  };
};

export default createInputProofWorker();
