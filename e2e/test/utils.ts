import { toBufferLE } from "bigint-buffer";
import { ContractMethodArgs, Typed } from "ethers";
import { ethers, network } from "hardhat";

import { TypedContractMethod } from "../types/common";

export const waitForBlock = (blockNumber: bigint | number) => {
  if (network.name === "hardhat") {
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
  } else {
    return new Promise((resolve, reject) => {
      const waitBlock = async (currentBlock: number) => {
        if (blockNumber <= BigInt(currentBlock)) {
          await ethers.provider.off("block", waitBlock);
          resolve(blockNumber);
        }
      };
      ethers.provider.on("block", waitBlock).catch((err) => {
        reject(err);
      });
    });
  }
};

export const waitNBlocks = async (Nblocks: number) => {
  const currentBlock = await ethers.provider.getBlockNumber();
  if (network.name === "hardhat") {
    await produceDummyTransactions(Nblocks);
  }
  await waitForBlock(currentBlock + Nblocks);
};

export const waitForBalance = async (address: string): Promise<void> => {
  return new Promise((resolve, reject) => {
    const checkBalance = async () => {
      const balance = await ethers.provider.getBalance(address);
      if (balance > 0) {
        await ethers.provider.off("block", checkBalance);
        resolve();
      }
    };
    ethers.provider.on("block", checkBalance).catch((err) => {
      reject(err);
    });
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
  let counter = blockCount;
  while (counter >= 0) {
    counter--;
    const [signer] = await ethers.getSigners();
    const nullAddress = "0x0000000000000000000000000000000000000000";
    const tx = {
      to: nullAddress,
      value: 0n,
    };
    const receipt = await signer.sendTransaction(tx);
    await receipt.wait();
  }
};

export const mineNBlocks = async (n: number) => {
  for (let index = 0; index < n; index++) {
    await ethers.provider.send("evm_mine");
  }
};

export const bigIntToBytes = (value: bigint) => {
  const byteArrayLength = Math.ceil(value.toString(2).length / 8);
  return new Uint8Array(toBufferLE(value, byteArrayLength));
};
