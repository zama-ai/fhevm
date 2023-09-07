import { ethers } from 'hardhat';

export const waitForBlock = (blockNumber: bigint) => {
  return new Promise((resolve, reject) => {
    const waitBlock = async (currentBlock: number) => {
      if (blockNumber <= BigInt(currentBlock)) {
        console.log(`Block ${currentBlock} reached!`);
        await ethers.provider.off('block', waitBlock);
        resolve(blockNumber);
      } else {
        console.log(`Current block: ${currentBlock}, waiting for ${blockNumber}...`);
      }
    };
    ethers.provider.on('block', waitBlock).catch((err) => {
      reject(err);
    });
  });
};
