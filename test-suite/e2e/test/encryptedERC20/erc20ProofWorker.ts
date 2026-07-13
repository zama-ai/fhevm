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

async function runWorker(data: WorkerData) {
  const { contractAddress, transferAmount, tasks, sdkConfig } = data;
  const fhe = await FhevmSdk.create(sdkConfig);
  const results = [];
  for (const task of tasks) {
    const encryptedTransferAmount = await fhe.encryptUint64({
      value: transferAmount,
      contractAddress,
      userAddress: task.senderAddress,
    });
    results.push({
      index: task.index,
      recipientAddress: task.recipientAddress,
      amountHandle: toHex(encryptedTransferAmount.handles[0]),
      inputProof: toHex(encryptedTransferAmount.inputProof),
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
