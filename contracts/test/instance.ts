import {
  clientKeyDecryptor,
  createEIP712,
  createInstance as createFhevmInstance,
  generateKeypair,
  getCiphertextCallParams,
} from '@fhevm/sdk';
import dotenv from 'dotenv';
import { readFileSync } from 'fs';
import * as fs from 'fs';
import { ethers, ethers as hethers, network } from 'hardhat';
import { homedir } from 'os';
import path from 'path';

import { awaitCoprocessor, getClearText } from './coprocessorUtils';
import { createEncryptedInputMocked, userDecryptRequestMocked } from './fhevmjsMocked';
import type { Signers } from './signers';
import { FhevmInstances } from './types';

const FHE_CLIENT_KEY_PATH = process.env.FHE_CLIENT_KEY_PATH;

let clientKey: Uint8Array | undefined;

const abiKmsVerifier = ['function getKmsSigners() view returns (address[])'];

const kmsAdd = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
const aclAdd = dotenv.parse(fs.readFileSync('addresses/.env.acl')).ACL_CONTRACT_ADDRESS;
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
    createEIP712: createEIP712(gatewayChainID, verifyingContract, network.config.chainId),
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

const getCiphertext = async (handle: string, ethers: typeof hethers): Promise<string> => {
  return ethers.provider.call(getCiphertextCallParams(handle));
};

const getDecryptor = () => {
  if (clientKey == null) {
    if (FHE_CLIENT_KEY_PATH) {
      clientKey = readFileSync(FHE_CLIENT_KEY_PATH);
    } else {
      const home = homedir();
      const clientKeyPath = path.join(home, 'network-fhe-keys/cks');
      clientKey = readFileSync(clientKeyPath);
    }
  }
  return clientKeyDecryptor(clientKey);
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bool}
 */
export const decryptBool = async (handle: string): Promise<boolean> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return (await getClearText(handle)) === '1';
  } else {
    return getDecryptor().decryptBool(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt8 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decrypt8(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt16 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decrypt16(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt32 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decrypt32(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt64 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decrypt64(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt128 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decrypt128(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt256 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decrypt256(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {string}
 */
export const decryptAddress = async (handle: string): Promise<string> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    const bigintAdd = BigInt(await getClearText(handle));
    const handleStr = '0x' + bigintAdd.toString(16).padStart(40, '0');
    return handleStr;
  } else {
    return getDecryptor().decryptAddress(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decryptEbytes64 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decryptEbytes64(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decryptEbytes128 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decryptEbytes128(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Decryption Oracle.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decryptEbytes256 = async (handle: string): Promise<bigint> => {
  if (network.name === 'hardhat') {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decryptEbytes256(await getCiphertext(handle, ethers));
  }
};
