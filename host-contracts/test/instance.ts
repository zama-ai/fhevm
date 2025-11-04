import { createEIP712, createInstance as createFhevmInstance, generateKeypair } from '@zama-fhe/relayer-sdk/node';
import type { RelayerEncryptedInput } from '@zama-fhe/relayer-sdk/node';
import dotenv from 'dotenv';
import * as fs from 'fs';
import { ethers, network } from 'hardhat';

import { awaitCoprocessor, getClearText } from './coprocessorUtils';
import { createEncryptedInputMocked, userDecryptRequestMocked } from './fhevmjsMocked';
import type { Signers } from './signers';
import { FhevmInstances } from './types';

const abiKmsVerifier = ['function getKmsSigners() view returns (address[])'];

const kmsAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).KMS_VERIFIER_CONTRACT_ADDRESS;
const aclAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).ACL_CONTRACT_ADDRESS;
const gatewayChainID = +process.env.CHAIN_ID_GATEWAY!;
const hostChainId = Number(network.config.chainId);
const verifyingContract = process.env.DECRYPTION_ADDRESS!;

const getKMSSigners = async (): Promise<string[]> => {
  const kmsContract = new ethers.Contract(kmsAdd, abiKmsVerifier, ethers.provider);
  const signers: string[] = await kmsContract.getKmsSigners();
  return signers;
};

const createInstanceMocked = async () => {
  const kmsSigners = await getKMSSigners();

  const instance = {
    userDecrypt: userDecryptRequestMocked(
      kmsSigners,
      gatewayChainID,
      hostChainId,
      verifyingContract,
      aclAdd,
      'http://localhost:3000',
      ethers.provider,
    ),
    createEncryptedInput: createEncryptedInputMocked,
    getPublicKey: () => '0xFFAA44433',
    generateKeypair: generateKeypair,
    createEIP712: createEIP712(verifyingContract, network.config.chainId),
  };
  return instance;
};

export const createInstances = async (accounts: Signers): Promise<FhevmInstances> => {
  // Create instance
  const instances: FhevmInstances = {} as FhevmInstances;
  if (network.name === 'hardhat') {
    await Promise.all(
      Object.keys(accounts).map(async (k) => {
        instances[k as keyof FhevmInstances] = await createInstanceMocked();
      }),
    );
  } else {
    await Promise.all(
      Object.keys(accounts).map(async (k) => {
        instances[k as keyof FhevmInstances] = await createInstance();
      }),
    );
  }
  return instances;
};

export const createInstance = async () => {
  const relayerUrl = 'http://localhost:3000';
  const instance = await createFhevmInstance({
    verifyingContractAddress: verifyingContract,
    kmsContractAddress: kmsAdd,
    aclContractAddress: aclAdd,
    network: network.config.url,
    relayerUrl: relayerUrl,
    gatewayChainId: gatewayChainID || '54321',
  });
  return instance;
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bool}
 */
export const decryptBool = async (handle: string): Promise<boolean> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return (await getClearText(handle)) === '1';
  } else {
    throw new Error(`decryptBool is not supported on network ${network.name}`);
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt8 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    throw new Error(`decrypt8 is not supported on network ${network.name}`);
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt16 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    throw new Error(`decrypt16 is not supported on network ${network.name}`);
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt32 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    throw new Error(`decrypt32 is not supported on network ${network.name}`);
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt64 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    throw new Error(`decrypt64 is not supported on network ${network.name}`);
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt128 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    throw new Error(`decrypt128 is not supported on network ${network.name}`);
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt256 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    throw new Error(`decrypt256 is not supported on network ${network.name}`);
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {string}
 */
export const decryptAddress = async (handle: string): Promise<string> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    const bigintAdd = BigInt(await getClearText(handle));
    const handleStr = '0x' + bigintAdd.toString(16).padStart(40, '0');
    return handleStr;
  } else {
    throw new Error(`decryptAddress is not supported on network ${network.name}`);
  }
};

export function getTotalBits(input: RelayerEncryptedInput) {
  let bits = input.getBits();
  let total = 0;
  for (let i = 0; i < bits.length; ++i) {
    total += bits[i];
  }
}
