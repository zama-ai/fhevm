import type { ClientContext } from "@cli-fhevm-sdk/toolkit";
import type { generateZkProof } from "@fhevm/sdk/actions/encrypt";
import { describe, expect, it, vi } from "vitest";

import {
  createInputProofWorker,
  type InputProofWorkerTask,
} from "../src/pool/workers/input-proof-worker";

const CONTRACT_ADDRESS = "0x0000000000000000000000000000000000000001" as const;
const USER_ADDRESS = "0x0000000000000000000000000000000000000002" as const;
const HANDLE = `0x${"ab".repeat(32)}` as const;

const task = (overrides: Partial<InputProofWorkerTask> = {}): InputProofWorkerTask => ({
  index: 7,
  network: "testnet",
  relayerUrl: "https://relayer.example.com",
  rpcUrl: "https://rpc.example.com",
  contractAddress: CONTRACT_ADDRESS,
  userAddress: USER_ADDRESS,
  contractChainId: 11_155_111,
  valueTypes: ["uint64"],
  ...overrides,
});

const deferred = () => {
  let resolve!: () => void;
  const promise = new Promise<void>((done) => {
    resolve = done;
  });
  return { promise, resolve };
};

const context = (ready: Promise<void>, contractAddress = CONTRACT_ADDRESS): ClientContext =>
  ({
    contractAddress,
    fhevm: { ready },
  }) as unknown as ClientContext;

const proof = {
  ciphertextWithZkProof: Uint8Array.from([0xde, 0xad, 0xbe, 0xef]),
  getExtraData: () => "0x00",
  getInputHandles: () => [{ bytes32Hex: HANDLE }],
};

describe("input-proof worker", () => {
  it("awaits SDK readiness before generating and serializes the proof", async () => {
    const ready = deferred();
    const createContext = vi.fn(() => context(ready.promise));
    const generateProof = vi.fn(async () => proof) as unknown as typeof generateZkProof;
    const worker = createInputProofWorker({ createContext, generateProof });

    const pending = worker(task());
    await vi.waitFor(() => expect(createContext).toHaveBeenCalledOnce());
    expect(generateProof).not.toHaveBeenCalled();

    ready.resolve();
    const item = await pending;

    expect(generateProof).toHaveBeenCalledOnce();
    expect(item).toMatchObject({
      index: 7,
      contractChainId: 11_155_111,
      contractAddress: CONTRACT_ADDRESS,
      userAddress: USER_ADDRESS,
      ciphertextWithInputVerification: "deadbeef",
      extraData: "0x00",
      expectedHandles: [HANDLE],
      values: [{ type: "uint64", value: expect.any(String) }],
    });
  });

  it("reuses the worker-local context for equivalent tasks", async () => {
    const createContext = vi.fn(() => context(Promise.resolve()));
    const generateProof = vi.fn(async () => proof) as unknown as typeof generateZkProof;
    const worker = createInputProofWorker({ createContext, generateProof });

    await worker(task({ index: 1 }));
    await worker(task({ index: 2 }));

    expect(createContext).toHaveBeenCalledOnce();
    expect(generateProof).toHaveBeenCalledTimes(2);
  });

  it("creates and awaits a new context when connection settings change", async () => {
    const firstReady = Promise.resolve();
    const secondReady = deferred();
    const createContext = vi
      .fn()
      .mockReturnValueOnce(context(firstReady))
      .mockReturnValueOnce(context(secondReady.promise));
    const generateProof = vi.fn(async () => proof) as unknown as typeof generateZkProof;
    const worker = createInputProofWorker({ createContext, generateProof });

    await worker(task({ index: 1 }));
    const pending = worker(task({ index: 2, relayerUrl: "https://candidate.example.com" }));
    await vi.waitFor(() => expect(createContext).toHaveBeenCalledTimes(2));
    expect(generateProof).toHaveBeenCalledTimes(1);

    secondReady.resolve();
    await pending;

    expect(generateProof).toHaveBeenCalledTimes(2);
  });
});
