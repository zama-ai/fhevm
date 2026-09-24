import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { toBufferBE } from 'bigint-buffer';
import { Typed } from 'ethers';
import type { BytesLike, ContractMethodArgs, Signer } from 'ethers';
import { ethers, network } from 'hardhat';
import hre from 'hardhat';

import type { Counter } from '../typechain-types';
import { TypedContractMethod } from '../typechain-types/common';
import { getSigners } from './signers';
import type { FhevmInstance } from './types';

export async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function checkIsHardhatSigner(signer: HardhatEthersSigner) {
  const signers: HardhatEthersSigner[] = await hre.ethers.getSigners();
  if (signers.findIndex((s) => s.address === signer.address) === -1) {
    throw new Error(
      `The provided address (${signer.address}) is not the address of a valid hardhat signer.
      Please use addresses listed via the 'npx hardhat get-accounts --network hardhat' command.`,
    );
  }
}

export const waitForBlock = (blockNumber: bigint | number) => {
  if (network.name === 'hardhat') {
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
          await ethers.provider.off('block', waitBlock);
          resolve(blockNumber);
        }
      };
      ethers.provider.on('block', waitBlock).catch((err) => {
        reject(err);
      });
    });
  }
};

export const waitNBlocks = async (Nblocks: number) => {
  const currentBlock = await ethers.provider.getBlockNumber();
  if (network.name === 'hardhat') {
    await produceDummyTransactions(Nblocks);
  }
  await waitForBlock(currentBlock + Nblocks);
};

export const waitForBalance = async (address: string): Promise<void> => {
  return new Promise((resolve, reject) => {
    const checkBalance = async () => {
      const balance = await ethers.provider.getBalance(address);
      if (balance > 0) {
        await ethers.provider.off('block', checkBalance);
        resolve();
      }
    };
    ethers.provider.on('block', checkBalance).catch((err) => {
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
  const contract = await contractFactory.connect(signers.dave).deploy();
  await contract.waitForDeployment();

  return contract as unknown as Counter;
}

export const mineNBlocks = async (n: number) => {
  for (let index = 0; index < n; index++) {
    await ethers.provider.send('evm_mine');
  }
};

export const bigIntToBytes64 = (value: bigint) => {
  return new Uint8Array(toBufferBE(value, 64));
};

export const bigIntToBytes128 = (value: bigint) => {
  return new Uint8Array(toBufferBE(value, 128));
};

export const bigIntToBytes256 = (value: bigint) => {
  return new Uint8Array(toBufferBE(value, 256));
};

export const userDecryptSingleHandle = async (
  handle: BytesLike,
  contractAddress: string,
  instance: FhevmInstance,
  signer: Signer,
  privateKey: string,
  publicKey: string,
): Promise<bigint> => {
  return instance.userDecryptSingleHandle({ handle, contractAddress, signer, keypair: { publicKey, privateKey } });
};

// `delegate` performs a user decrypt on behalf of `delegator`
export const delegatedUserDecryptSingleHandle = async (params: {
  instance: FhevmInstance;
  handle: BytesLike;
  signer: Signer;
  contractAddress: string;
  delegatorAddress: string;
  kmsPrivateKey: string;
  kmsPublicKey: string;
}): Promise<bigint> => {
  return params.instance.delegatedUserDecryptSingleHandle({
    handle: params.handle,
    contractAddress: params.contractAddress,
    delegatorAddress: params.delegatorAddress,
    signer: params.signer,
    keypair: { publicKey: params.kmsPublicKey, privateKey: params.kmsPrivateKey },
  });
};
