import { type ContractTransactionReceipt, ethers as EthersT } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

export const mineNBlocks = async (hre: HardhatRuntimeEnvironment, n: number): Promise<void> => {
  for (let index = 0; index < n; index++) {
    await hre.ethers.provider.send('evm_mine');
  }
};

export const waitNBlocks = async (hre: HardhatRuntimeEnvironment, numBlocks: number): Promise<void> => {
  const isDevelopment = hre.fhevm.isDevelopment;
  const currentBlock = await hre.ethers.provider.getBlockNumber();
  if (isDevelopment) {
    await produceDummyTransactions(hre, numBlocks);
  } else {
    await waitForBlock(hre, BigInt(currentBlock + numBlocks));
  }
};

export const produceDummyTransactions = async (hre: HardhatRuntimeEnvironment, blockCount: number): Promise<void> => {
  let counter = blockCount;
  while (counter >= 0) {
    counter--;
    const [signer] = await hre.ethers.getSigners();
    const tx = {
      to: EthersT.ZeroAddress,
      value: 0n,
    };
    const receipt = await signer.sendTransaction(tx);
    await receipt.wait();
  }
};

const waitForBlock = (hre: HardhatRuntimeEnvironment, blockNumber: bigint): Promise<bigint> => {
  return new Promise((resolve, reject) => {
    let targetReached = false;
    const rejectWithError = (reason: unknown): void => {
      reject(reason instanceof Error ? reason : new Error(String(reason)));
    };
    const waitBlock = (currentBlock: number): void => {
      if (targetReached || blockNumber > BigInt(currentBlock)) return;
      targetReached = true;
      void hre.ethers.provider
        .off('block', waitBlock)
        .then(() => {
          resolve(blockNumber);
        })
        .catch(rejectWithError);
    };
    void hre.ethers.provider.on('block', waitBlock).catch(rejectWithError);
  });
};

// `tx.wait()` is typed `| null` for the zero-confirmation case, which these tests never ask for.
export function requireReceipt(receipt: ContractTransactionReceipt | null): ContractTransactionReceipt {
  if (receipt === null) {
    throw new Error('Expected a transaction receipt');
  }
  return receipt;
}
