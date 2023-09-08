import { ContractMethodArgs, Typed } from 'ethers';
import { ethers } from 'hardhat';

import { TypedContractMethod } from '../types/common';

export const waitForBlock = (blockNumber: bigint) => {
  return new Promise((resolve, reject) => {
    const waitBlock = async (currentBlock: number) => {
      // console.log(`Block ${currentBlock} reached! Waiting ${blockNumber}...`);
      if (blockNumber <= BigInt(currentBlock)) {
        // console.log(`Block ${currentBlock} reached!`);
        await ethers.provider.off('block', waitBlock);
        resolve(blockNumber);
      }
    };
    ethers.provider.on('block', waitBlock).catch((err) => {
      reject(err);
    });
  });
};

export const createTransaction = async <A extends [...{ [I in keyof A]-?: A[I] | Typed }]>(
  method: TypedContractMethod<A>,
  ...params: A
) => {
  const gasLimit = await method.estimateGas(...params);
  const updatedParams: ContractMethodArgs<A> = [...params, { gasLimit: Math.round(+gasLimit.toString() * 1.2) }];
  return method(...updatedParams);
};
