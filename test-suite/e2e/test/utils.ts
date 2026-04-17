import type { FheTypeInfo } from '@fhevm/solidity/lib-js/common';
import { ALL_FHE_TYPE_INFOS } from '@fhevm/solidity/lib-js/fheTypeInfos';
import { ALL_OPERATORS_PRICES } from '@fhevm/solidity/lib-js/operatorsPrices';
import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { toBufferBE } from 'bigint-buffer';
import { ContractMethodArgs, Log, TransactionReceipt, Typed } from 'ethers';
import { Signer } from 'ethers';
import { ethers, network } from 'hardhat';
import hre from 'hardhat';

import { coprocessorAddress } from './instance';
import { TypedContractMethod } from '../types/common';

const delegatedUserDecryptRetryMs = Number(process.env.RELAYER_SDK_DELEGATED_USER_DECRYPT_RETRY_MS) || 2_000;
const delegatedUserDecryptTimeoutMs =
  Number(process.env.RELAYER_SDK_DELEGATED_USER_DECRYPT_TIMEOUT_MS) || 10 * 60 * 1000;

const readinessTimedOut = 'readiness_check_timed_out';
const readinessTimedOutMessage = 'Ciphertext not ready for decryption on the gateway chain';

const isDelegatedDecryptNotReady = (error: unknown) => {
  if (error instanceof Error && error.message.includes(readinessTimedOutMessage)) {
    return true;
  }
  if (typeof error !== 'object' || error === null || !('relayerApiError' in error)) {
    return false;
  }
  const relayerApiError = (error as { relayerApiError?: { label?: string; message?: string } }).relayerApiError;
  return (
    relayerApiError?.label === readinessTimedOut ||
    relayerApiError?.message?.includes(readinessTimedOutMessage) === true
  );
};

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

export const mineNBlocks = async (n: number) => {
  for (let index = 0; index < n; index++) {
    await ethers.provider.send('evm_mine');
  }
};

export const waitForPendingTransactions = async (txHashes: string[], timeoutMs = 30_000): Promise<void> => {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    const pendingBlock = await ethers.provider.send('eth_getBlockByNumber', ['pending', false]);
    const pendingTxHashes = new Set<string>(pendingBlock?.transactions ?? []);
    if (txHashes.every((txHash) => pendingTxHashes.has(txHash))) {
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
  throw new Error(`Timed out waiting for pending txs: ${txHashes.join(', ')}`);
};

export const waitForTransactionReceipt = async (txHash: string, timeoutMs = 30_000): Promise<TransactionReceipt> => {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    const receipt = await ethers.provider.getTransactionReceipt(txHash);
    if (receipt) {
      return receipt;
    }
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
  throw new Error(`Timed out waiting for receipt: ${txHash}`);
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
  const startTimeStamp = Math.floor(Date.now() / 1000);
  const durationDays = 10; // Relayer-sdk expects numbers from now on
  const contractAddresses = [contractAddress];

  // Build the extraData field
  const extraData = await instance.getExtraData();

  // Use the new createEIP712 function
  const eip712 = instance.createEIP712(publicKey, contractAddresses, startTimeStamp, durationDays, extraData);

  // Update the signing to match the new primaryType
  const signature = await signer.signTypedData(
    eip712.domain,
    {
      UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
    },
    eip712.message,
  );

  const signerAddress = await signer.getAddress();
  const deadline = Date.now() + delegatedUserDecryptTimeoutMs;
  let result;
  while (true) {
    try {
      result = await instance.userDecrypt(
        HandleContractPairs,
        privateKey,
        publicKey,
        signature.replace('0x', ''),
        contractAddresses,
        signerAddress,
        startTimeStamp,
        durationDays,
        extraData,
      );
      break;
    } catch (error) {
      if (!isDelegatedDecryptNotReady(error) || Date.now() + delegatedUserDecryptRetryMs > deadline) {
        throw error;
      }
      await new Promise((resolve) => setTimeout(resolve, delegatedUserDecryptRetryMs));
    }
  }

  const decryptedValue = result[handle];
  return decryptedValue;
};

export const delegatedUserDecryptSingleHandle = async (
  instance: any,
  handle: string,
  contractAddress: string,
  delegatorAddress: string,
  delegateAddress: string,
  signer: Signer,
  delegatePrivateKey: string,
  delegatePublicKey: string,
): Promise<bigint | boolean | string> => {
  const handleContractPairs = [
    {
      handle,
      contractAddress,
    },
  ];
  const startTimeStamp = Math.floor(Date.now() / 1000);
  const durationDays = 10;
  const contractAddresses = [contractAddress];

  // Build the extraData field
  const extraData = await instance.getExtraData();

  // The `delegate` creates a EIP712 with the `delegator` address
  const eip712 = instance.createDelegatedUserDecryptEIP712(
    delegatePublicKey,
    contractAddresses,
    delegatorAddress,
    startTimeStamp,
    durationDays,
    extraData,
  );

  // Update the signing to match the new primaryType
  const delegateSignature = await signer.signTypedData(
    eip712.domain,
    {
      DelegatedUserDecryptRequestVerification: eip712.types.DelegatedUserDecryptRequestVerification,
    },
    eip712.message,
  );

  const deadline = Date.now() + delegatedUserDecryptTimeoutMs;
  let result;
  while (true) {
    try {
      result = await instance.delegatedUserDecrypt(
        handleContractPairs,
        delegatePrivateKey,
        delegatePublicKey,
        delegateSignature.replace('0x', ''),
        contractAddresses,
        delegatorAddress,
        delegateAddress,
        startTimeStamp,
        durationDays,
        extraData,
      );
      break;
    } catch (error) {
      if (!isDelegatedDecryptNotReady(error) || Date.now() + delegatedUserDecryptRetryMs > deadline) {
        throw error;
      }
      await new Promise((resolve) => setTimeout(resolve, delegatedUserDecryptRetryMs));
    }
  }

  return result[handle];
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
  'event VerifyInput(address indexed caller, bytes32 inputHandle, address userAddress, bytes inputProof, uint8 inputType, bytes32 result)',
  'event Cast(address indexed caller, bytes32 ct, uint8 toType, bytes32 result)',
  'event TrivialEncrypt(address indexed caller, uint256 pt, uint8 toType, bytes32 result)',
  'event TrivialEncryptBytes(address indexed caller, bytes pt, uint8 toType, bytes32 result)',
  'event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result)',
  'event FheRand(address indexed caller, uint8 randType, bytes16 seed, bytes32 result)',
  'event FheRandBounded(address indexed caller, uint256 upperBound, uint8 randType, bytes16 seed, bytes32 result)',
];

export function getTxHCUFromTxReceipt(
  receipt: TransactionReceipt,
  FheTypes: FheTypeInfo[] = ALL_FHE_TYPE_INFOS,
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

  const contract = new ethers.Contract(coprocessorAddress, abi, ethers.provider);
  const relevantLogs = receipt.logs.filter((log: Log) => {
    if (log.address.toLowerCase() !== coprocessorAddress.toLowerCase()) {
      return false;
    }
    try {
      const parsedLog = contract.interface.parseLog({
        topics: log.topics,
        data: log.data,
      });
      return abi.some((item) => item.startsWith(`event ${parsedLog!.name}`) && parsedLog!.name !== 'VerifyInput');
    } catch {
      return false;
    }
  });

  const FHELogs = relevantLogs.map((log: Log) => {
    const parsedLog = contract.interface.parseLog({
      topics: log.topics,
      data: log.data,
    });
    return {
      name: parsedLog!.name,
      args: parsedLog!.args,
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        hcuConsumed = (ALL_OPERATORS_PRICES['trivialEncrypt'].types as Record<string, number>)[type];
        totalHCUConsumed += hcuConsumed;
        handleResult = ethers.toBeHex(event.args[3], 32);
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        break;

      case 'TrivialEncryptBytes':
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        hcuConsumed = (ALL_OPERATORS_PRICES['trivialEncrypt'].types as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheAdd'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheAdd'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheSub'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheSub'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheMul'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheMul'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheDiv'].scalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheRem'].scalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheBitAnd'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheBitAnd'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheBitOr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheBitOr'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheBitXor'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheBitXor'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheShl'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheShl'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheShr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheShr'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheRotl'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheRotl'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheRotr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheRotr'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheEq'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheEq'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheEq'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheEq'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheNe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheNe'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheNe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheNe'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheGe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheGe'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheGt'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheGt'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheLe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheLe'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheLt'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheLt'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheMax'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheMax'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheMin'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES['fheMin'].nonScalar as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        hcuConsumed = (ALL_OPERATORS_PRICES['cast'].types as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        hcuConsumed = (ALL_OPERATORS_PRICES['fheNot'].types as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        hcuConsumed = (ALL_OPERATORS_PRICES['fheNeg'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(ethers.toBeHex(event.args[2], 32));
        break;

      case 'FheIfThenElse':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }

        hcuConsumed = (ALL_OPERATORS_PRICES['ifThenElse'].types as Record<string, number>)[type];
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
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        hcuConsumed = (ALL_OPERATORS_PRICES['fheRand'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRandBounded':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheTypeInfo index: ${typeIndex}`);
        }
        hcuConsumed = (ALL_OPERATORS_PRICES['fheRandBounded'].types as Record<string, number>)[type];
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
