import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import type { Auth } from '../sdk/types';

type WorkerTask = {
  index: number;
  senderAddress: string;
  recipientAddress: string;
};

type WorkerData = {
  contractAddress: string;
  transferAmount: number;
  tasks: WorkerTask[];
  /** Milliseconds between successive requests; omit to go as fast as possible. */
  paceMs?: number;
  /** Shared wall-clock origin so paced workers interleave into one rate. */
  startAtMs?: number;
  sdkConfig: {
    verifyingContractAddressDecryption: string;
    verifyingContractAddressInputVerification: string;
    kmsContractAddress: string;
    inputVerifierContractAddress: string;
    aclContractAddress: string;
    protocolConfigAddress?: string;
    relayerUrl: string;
    rpcUrl: string;
    gatewayChainId: number;
    chainId: number;
    auth?: Auth;
    numberOfThreads?: number;
  };
};

const toHex = (value: Uint8Array | string) =>
  typeof value === 'string' ? value : `0x${Buffer.from(value).toString('hex')}`;

const sendAndExit = (message: unknown, exitCode: number) => {
  if (process.send) {
    const exitAfterSend = (): void => {
      process.exit(exitCode);
    };
    process.send(message, undefined, undefined, exitAfterSend);
    return;
  }
  process.exit(exitCode);
};

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

async function runWorker(data: WorkerData) {
  const { contractAddress, transferAmount, tasks, sdkConfig, paceMs, startAtMs } = data;
  const fhe = await FhevmSdk.create(sdkConfig);
  const results = [];

  // When pacing, every worker shares one wall-clock origin so the aggregate
  // request rate across workers is the configured rate rather than a function
  // of how long each worker took to boot its WASM pool.
  const origin = startAtMs ?? Date.now();

  for (let i = 0; i < tasks.length; i += 1) {
    const task = tasks[i];
    if (paceMs) {
      const dueAt = origin + i * paceMs;
      const wait = dueAt - Date.now();
      if (wait > 0) await sleep(wait);
    }
    // Spans client-side proving plus the gateway round-trip that triggers
    // coprocessor verification: this is the latency a submitter actually sees.
    const requestedAtMs = Date.now();
    const encryptedTransferAmount = await fhe.encryptUint64({
      value: transferAmount,
      contractAddress,
      userAddress: task.senderAddress,
    });
    const respondedAtMs = Date.now();
    results.push({
      index: task.index,
      recipientAddress: task.recipientAddress,
      amountHandle: toHex(encryptedTransferAmount.handles[0]),
      inputProof: toHex(encryptedTransferAmount.inputProof),
      requestedAtMs,
      respondedAtMs,
    });
  }
  sendAndExit({ ok: true, results }, 0);
}

process.on('message', (data: WorkerData) => {
  runWorker(data).catch((error) => {
    sendAndExit(
      {
        ok: false,
        error: error instanceof Error ? error.stack || error.message : String(error),
      },
      1,
    );
  });
});
