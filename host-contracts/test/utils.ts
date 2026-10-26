import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { toBufferBE } from 'bigint-buffer';
import { ContractMethodArgs, Typed } from 'ethers';
import { Signer } from 'ethers';
import { ethers, network } from 'hardhat';
import hre from 'hardhat';

import type { Counter } from '../types';
import { TypedContractMethod } from '../types/common';
import type { ClearValues, FhevmInstance } from './legacyRelayerSdk/types';
import { ENCRYPTION_TYPES, isEuintFheTypeId } from './legacyRelayerSdk/types';
import { getSigners } from './signers';

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
  const contract = await _deployCounterContract();
  let counter = blockCount;
  while (counter > 0) {
    counter--;
    const tx = await contract.increment();
    const _ = await tx.wait();
  }
};

async function _deployCounterContract(): Promise<Counter> {
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

// Throws if handle is not a 32-byte handle (hex string or bytes)
function _assertIsHandle32(handle: string | Uint8Array): void {
  if (typeof handle === 'string' ? !ethers.isHexString(handle, 32) : handle.length !== 32) {
    const shown = typeof handle === 'string' ? handle : ethers.hexlify(handle);
    throw new Error(`Invalid handle ${shown}, expected 32 bytes`);
  }
}

// Throws if handle is not a 32-byte handle (hex string or bytes) whose FheTypeId is a euintxxx
function _assertIsEuintHandle(handle: string | Uint8Array): void {
  _assertIsHandle32(handle);
  const fheTypeId = ethers.getBytes(handle)[30];
  if (!isEuintFheTypeId(fheTypeId)) {
    throw new Error(`Handle ${ethers.hexlify(handle)} is not a euint (FheTypeId=${fheTypeId})`);
  }
}

// Throws if handle is not a 32-byte handle (hex string or bytes) whose FheTypeId is `fheTypeId`
function _assertIsHandleOfFheTypeId(handle: string | Uint8Array, fheTypeId: number, typeName: string): void {
  _assertIsHandle32(handle);
  const handleFheTypeId = ethers.getBytes(handle)[30];
  if (handleFheTypeId !== fheTypeId) {
    throw new Error(`Handle ${ethers.hexlify(handle)} is not an ${typeName} (FheTypeId=${handleFheTypeId})`);
  }
}

// User-decrypts a single handle, without any FheTypeId check. The mock returns every clear value as a bigint.
async function _userDecryptSingleHandleAsBigInt(
  handle: string | Uint8Array,
  contractAddress: string,
  instance: FhevmInstance,
  signer: Signer,
  privateKey: string,
  publicKey: string,
): Promise<bigint> {
  const handleContractPairs = [{ handle, contractAddress }];
  const startTimeStamp = Math.floor(Date.now() / 1000).toString();
  const durationDays = '10'; // String for consistency
  const contractAddresses = [contractAddress];

  const eip712 = instance.createEIP712(publicKey, contractAddresses, startTimeStamp, durationDays);
  const signature = await signer.signTypedData(
    eip712.domain,
    { UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification },
    eip712.message,
  );

  const decryptedValues: ClearValues = await instance.userDecrypt(
    handleContractPairs,
    privateKey,
    publicKey,
    signature.replace('0x', ''),
    contractAddresses,
    await signer.getAddress(),
    startTimeStamp,
    durationDays,
  );

  const entries = Object.entries(decryptedValues);
  if (entries.length !== 1) {
    throw new Error(`Expected a single decrypted value, got ${entries.length}`);
  }

  const [decryptedHandle, decryptedValue] = entries[0];
  // ethers.hexlify returns lowercase 0x-prefixed hex for both strings and bytes
  const handleHex = ethers.hexlify(handle);
  if (ethers.hexlify(decryptedHandle) !== handleHex) {
    throw new Error(`Expected decrypted value for handle ${handleHex}, got ${decryptedHandle}`);
  }
  if (typeof decryptedValue !== 'bigint') {
    throw new Error(`Expected a bigint decrypted value, got ${typeof decryptedValue}`);
  }

  return decryptedValue;
}

// Returns the clear value of a euintxxx handle as a bigint
export const userDecryptSingleEuint = async (
  handle: string | Uint8Array,
  contractAddress: string,
  instance: FhevmInstance,
  signer: Signer,
  privateKey: string,
  publicKey: string,
): Promise<bigint> => {
  _assertIsEuintHandle(handle);

  return _userDecryptSingleHandleAsBigInt(handle, contractAddress, instance, signer, privateKey, publicKey);
};

// Returns the clear value as a boolean, as the relayer SDK does for an ebool
export const userDecryptSingleEbool = async (
  handle: string | Uint8Array,
  contractAddress: string,
  instance: FhevmInstance,
  signer: Signer,
  privateKey: string,
  publicKey: string,
): Promise<boolean> => {
  _assertIsHandleOfFheTypeId(handle, ENCRYPTION_TYPES[2], 'ebool');

  const value = await _userDecryptSingleHandleAsBigInt(
    handle,
    contractAddress,
    instance,
    signer,
    privateKey,
    publicKey,
  );
  if (value !== 0n && value !== 1n) {
    throw new Error(`Expected an ebool clear value of 0 or 1, got ${value}`);
  }
  return value === 1n;
};

// Returns the clear value as a checksummed address, as the relayer SDK does for an eaddress
export const userDecryptSingleEaddress = async (
  handle: string | Uint8Array,
  contractAddress: string,
  instance: FhevmInstance,
  signer: Signer,
  privateKey: string,
  publicKey: string,
): Promise<string> => {
  _assertIsHandleOfFheTypeId(handle, ENCRYPTION_TYPES[160], 'eaddress');

  const value = await _userDecryptSingleHandleAsBigInt(
    handle,
    contractAddress,
    instance,
    signer,
    privateKey,
    publicKey,
  );
  if (value >= 2n ** 160n) {
    throw new Error(`Expected an eaddress clear value below 2^160, got ${value}`);
  }
  return ethers.getAddress('0x' + value.toString(16).padStart(40, '0'));
};
