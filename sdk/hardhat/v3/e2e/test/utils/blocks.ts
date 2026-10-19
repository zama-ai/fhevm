import type { NetworkConnection } from 'hardhat/types/network';

// Lets a coprocessor-side change (a revoked delegation, say) propagate: N blocks of dummy transactions
// on a development chain, a wait for the chain to reach the block anywhere else.
export async function waitNBlocks(connection: NetworkConnection, numBlocks: number): Promise<void> {
  const currentBlock = await connection.ethers.provider.getBlockNumber();
  if (connection.fhevm.isDevelopment) {
    await produceDummyTransactions(connection, numBlocks);
  } else {
    await waitForBlock(connection, currentBlock + numBlocks);
  }
}

async function produceDummyTransactions(connection: NetworkConnection, blockCount: number): Promise<void> {
  const [signer] = await connection.ethers.getSigners();
  if (signer === undefined) throw new Error('no signer to produce dummy transactions with');
  for (let i = 0; i <= blockCount; i++) {
    const tx = await signer.sendTransaction({ to: connection.ethers.ZeroAddress, value: 0n });
    await tx.wait();
  }
}

function waitForBlock(connection: NetworkConnection, blockNumber: number): Promise<void> {
  return new Promise((resolve, reject) => {
    const rejectWithError = (reason: unknown): void => {
      reject(reason instanceof Error ? reason : new Error(String(reason)));
    };
    const onBlock = (currentBlock: number): void => {
      if (currentBlock < blockNumber) return;
      void connection.ethers.provider
        .off('block', onBlock)
        .then(() => {
          resolve();
        })
        .catch(rejectWithError);
    };
    void connection.ethers.provider.on('block', onBlock).catch(rejectWithError);
  });
}
