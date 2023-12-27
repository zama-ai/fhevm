import { ContractMethodArgs, Typed } from 'ethers';
import { ethers } from 'hardhat';

import type { Counter } from '../types';
import { TypedContractMethod } from '../types/common';
import { getSigners } from './signers';

async function getCurrentBlockNumber() {
  return ethers.provider.getBlockNumber();
}

export const waitForBlock = (blockNumber: bigint) => {
  return new Promise((resolve, reject) => {
    const intervalId = setInterval(async () => {
      try {
        const currentBlock = await ethers.provider.getBlockNumber();
        if (BigInt(currentBlock) >= blockNumber) {
          clearInterval(intervalId);
          resolve(currentBlock);
        }
      } catch (error) {
        clearInterval(intervalId);
        reject(error);
      }
    }, 50); // Check every 50 milliseconds
  });
};

export const waitForBalance = (address: string) => {
  return new Promise<void>((resolve, reject) => {
    const intervalId = setInterval(async () => {
      try {
        const balance = await ethers.provider.getBalance(address);
        if (balance >= 0) {
          clearInterval(intervalId);
          resolve();
        }
      } catch (error) {
        clearInterval(intervalId);
        reject(error);
      }
    }, 50); // Check every 50 milliseconds
  });
};

export const createTransaction = async <A extends [...{ [I in keyof A]-?: A[I] | Typed }]>(
  method: TypedContractMethod<A>,
  ...params: A
) => {
  const gasLimit = await method.estimateGas(...params);
  const updatedParams: ContractMethodArgs<A> = [
    ...params,
    { gasLimit: Math.min(Math.round(+gasLimit.toString() * 1.2), 10000000) },
  ];
  return method(...updatedParams);
};

export const produceDummyTransactions = async (blockCount: number) => {
  const contract = await deployCounterContract();
  let counter = blockCount;
  while (counter > 0) {
    counter--;
    const tx = await contract.increment();
    const _ = await tx.wait();
  }
};

async function deployCounterContract(): Promise<Counter> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('Counter');
  const contract = await contractFactory.connect(signers.eve).deploy();
  await contract.waitForDeployment();

  return contract;
}

export const mineNBlocks = async (n: number) => {
  for (let index = 0; index < n; index++) {
    await ethers.provider.send('evm_mine');
  }
};
