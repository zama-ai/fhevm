import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { toBufferBE } from 'bigint-buffer';
import { ContractMethodArgs, Typed } from 'ethers';
import { Signer } from 'ethers';
import { ethers, network } from 'hardhat';

import type { Counter } from '../types';
import { TypedContractMethod } from '../types/common';
import operatorsPrices from './operatorsPrices.json';
import { getSigners } from './signers';
import { ALL_FHE_TYPES } from './types';

const hre = require('hardhat');

const coprocAddress = process.env.FHEVM_EXECUTOR_CONTRACT_ADDRESS;

export async function checkIsHardhatSigner(signer: HardhatEthersSigner) {
  const signers = await hre.ethers.getSigners();
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

  return contract;
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
  handle: string,
  contractAddress: string,
  instance: any,
  signer: Signer,
  privateKey: string,
  publicKey: string,
): Promise<bigint | boolean | string> => {
  const HandleContractPairs = [
    {
      handle: handle,
      contractAddress: contractAddress,
    },
  ];
  const startTimeStamp = Math.floor(Date.now() / 1000).toString();
  const durationDays = '10'; // String for consistency
  const contractAddresses = [contractAddress];

  // Use the new createEIP712 function
  const eip712 = instance.createEIP712(publicKey, contractAddresses, startTimeStamp, durationDays);

  // Update the signing to match the new primaryType
  const signature = await signer.signTypedData(
    eip712.domain,
    {
      UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
    },
    eip712.message,
  );

  const result = await instance.userDecrypt(
    HandleContractPairs,
    privateKey,
    publicKey,
    signature.replace('0x', ''),
    contractAddresses,
    signer.address,
    startTimeStamp,
    durationDays,
  );

  const decryptedValue = result[handle];
  return decryptedValue;
};

const abi = [
  'event FheAdd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheSub(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheMul(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheDiv(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheRem(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheBitAnd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheBitOr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheBitXor(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheShl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheShr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheRotl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheRotr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheEq(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheEqBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result)',
  'event FheNe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheNeBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result)',
  'event FheGe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheGt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheLe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheLt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheMin(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheMax(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheNeg(address indexed caller, bytes32 ct, bytes32 result)',
  'event FheNot(address indexed caller, bytes32 ct, bytes32 result)',
  'event VerifyCiphertext(address indexed caller, bytes32 inputHandle, address userAddress, bytes inputProof, uint8 inputType, bytes32 result)',
  'event Cast(address indexed caller, bytes32 ct, uint8 toType, bytes32 result)',
  'event TrivialEncrypt(address indexed caller, uint256 pt, uint8 toType, bytes32 result)',
  'event TrivialEncryptBytes(address indexed caller, bytes pt, uint8 toType, bytes32 result)',
  'event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result)',
  'event FheRand(address indexed caller, uint8 randType, bytes16 seed, bytes32 result)',
  'event FheRandBounded(address indexed caller, uint256 upperBound, uint8 randType, bytes16 seed, bytes32 result)',
];

export function getTxHCUFromTxReceipt(
  receipt: ethers.TransactionReceipt,
  FheTypes: FheType[] = ALL_FHE_TYPES,
): {
  globalTxHCU: number;
  maxTxHCUDepth: number;
  HCUDepthPerHandle: Record<string, number>;
} {
  if (receipt.status === 0) {
    throw new Error('Transaction reverted');
  }

  function readFromHCUMap(handle: string): number {
    if (hcuMap[handle] === undefined) {
      return 0;
    }
    return hcuMap[handle];
  }

  let hcuMap: Record<string, number> = {};
  let handleSet: Set<string> = new Set();

  const contract = new ethers.Contract(coprocAddress, abi, ethers.provider);
  const relevantLogs = receipt.logs.filter((log: ethers.Log) => {
    if (log.address.toLowerCase() !== coprocAddress.toLowerCase()) {
      return false;
    }
    try {
      const parsedLog = contract.interface.parseLog({
        topics: log.topics,
        data: log.data,
      });
      return abi.some((item) => item.startsWith(`event ${parsedLog.name}`) && parsedLog.name !== 'VerifyCiphertext');
    } catch {
      return false;
    }
  });

  const FHELogs = relevantLogs.map((log: ethers.Log) => {
    const parsedLog = contract.interface.parseLog({
      topics: log.topics,
      data: log.data,
    });
    return {
      name: parsedLog.name,
      args: parsedLog.args,
    };
  });

  let totalHCUConsumed = 0;

  for (const event of FHELogs) {
    let type: string | undefined;
    let typeIndex: number;
    let handle: string;
    let handleResult: string;
    let hcuConsumed: number;

    switch (event.name) {
      case 'TrivialEncrypt':
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['trivialEncrypt'].types as Record<string, number>)[type];
        totalHCUConsumed += hcuConsumed;
        handleResult = ethers.toBeHex(event.args[3], 32);
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        break;

      case 'TrivialEncryptBytes':
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['trivialEncrypt'].types as Record<string, number>)[type];
        totalHCUConsumed += hcuConsumed;
        handleResult = ethers.toBeHex(event.args[3], 32);
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        break;

      case 'FheAdd':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheAdd'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheAdd'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheSub':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheSub'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheSub'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheMul':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheMul'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheMul'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheDiv':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheDiv'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          throw new Error('Non-scalar div not implemented yet');
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRem':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheRem'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          throw new Error('Non-scalar rem not implemented yet');
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheBitAnd':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheBitAnd'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheBitAnd'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheBitOr':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheBitOr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheBitOr'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheBitXor':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheBitXor'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheBitXor'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheShl':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheShl'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheShl'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheShr':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheShr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheShr'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRotl':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheRotl'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheRotl'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRotr':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheRotr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheRotr'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheEq':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheEq'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheEq'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheEqBytes':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheEq'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheEq'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheNe':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheNe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheNe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheNeBytes':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheNe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheNe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheGe':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheGe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheGe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheGt':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheGt'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheGt'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheLe':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheLe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheLe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheLt':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheLt'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheLt'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheMax':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheMax'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheMax'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheMin':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheMin'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheMin'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'Cast':
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        handleResult = ethers.toBeHex(event.args[3], 32);

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['cast'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(handleResult);
        break;

      case 'FheNot':
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        handleResult = ethers.toBeHex(event.args[2], 32);
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheNot'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(ethers.toBeHex(event.args[2], 32));
        break;

      case 'FheNeg':
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        handleResult = ethers.toBeHex(event.args[2], 32);
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheNeg'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(ethers.toBeHex(event.args[2], 32));
        break;

      case 'FheIfThenElse':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['ifThenElse'].types as Record<string, number>)[type];
        hcuMap[handleResult] =
          hcuConsumed +
          Math.max(
            readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
            readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            readFromHCUMap(ethers.toBeHex(event.args[3], 32)),
          );
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRand':
        handleResult = ethers.toBeHex(event.args[3], 32);
        typeIndex = parseInt(event.args[1]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheRand'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRandBounded':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheRandBounded'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
    }
  }

  let maxDepthHCU = 0;

  handleSet.forEach((handle) => {
    const hcu = hcuMap[handle];
    if (hcu > maxDepthHCU) {
      maxDepthHCU = hcu;
    }
  });

  return {
    globalTxHCU: totalHCUConsumed,
    maxTxHCUDepth: maxDepthHCU,
    HCUDepthPerHandle: hcuMap,
  };
}