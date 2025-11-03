import {
  FhevmInstance,
  clientKeyDecryptor,
  createEIP712,
  createInstance as createFhevmInstance,
  generateKeypair,
  getCiphertextCallParams,
} from '@zama-fhe/relayer-sdk/node';
import dotenv from 'dotenv';
import type { ethers as EthersT } from 'ethers';
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
const abiAcl = [
  'function delegateForUserDecryption(address,address,uint64)',
  'function revokeDelegationForUserDecryption(address,address)',
];

const parsedEnv = dotenv.parse(fs.readFileSync('./fhevmTemp/addresses/.env.host'));
const kmsAdd = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
const aclAdd = parsedEnv.ACL_CONTRACT_ADDRESS;
const inputVerificationAdd = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
const gatewayChainID = +process.env.CHAIN_ID_GATEWAY!;
const hostChainId = Number(network.config.chainId);
const verifyingContractAddressDecryption = process.env.DECRYPTION_ADDRESS!;

const getKMSSigners = async (): Promise<string[]> => {
  const kmsContract = new ethers.Contract(kmsAdd, abiKmsVerifier, ethers.provider);
  const signers: string[] = await kmsContract.getKmsSigners();
  return signers;
};

export const delegateUserDecryption = async (
  delegator: EthersT.Signer,
  delegate: string,
  contractAddress: string,
  expirationDate: bigint,
): Promise<EthersT.TransactionResponse> => {
  const aclContract = new ethers.Contract(aclAdd, abiAcl, delegator);
  return aclContract.delegateForUserDecryption(delegate, contractAddress, expirationDate);
};

export const revokeUserDecryptionDelegation = async (
  delegator: EthersT.Signer,
  delegate: string,
  contractAddress: string,
): Promise<EthersT.TransactionResponse> => {
  const aclContract = new ethers.Contract(aclAdd, abiAcl, delegator);
  return aclContract.revokeDelegationForUserDecryption(delegate, contractAddress);
};

const createInstanceMocked = async (): FhevmInstance => {
  const kmsSigners = await getKMSSigners();

  const instance: FhevmInstance = {
    userDecrypt: userDecryptRequestMocked(
      kmsSigners,
      gatewayChainID,
      hostChainId,
      verifyingContractAddressDecryption,
      aclAdd,
      'http://localhost:3000',
      ethers.provider,
    ),
    createEncryptedInput: createEncryptedInputMocked,
    getPublicKey: () => '0xFFAA44433',
    generateKeypair: generateKeypair,
    createEIP712: createEIP712(verifyingContractAddressDecryption, network.config.chainId!),
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
    verifyingContractAddressDecryption,
    verifyingContractAddressInputVerification: ethers.ZeroAddress,
    kmsContractAddress: kmsAdd,
    aclContractAddress: aclAdd,
    inputVerifierContractAddress: inputVerificationAdd,
    network: (network.config as any).url,
    relayerUrl: relayerUrl,
    gatewayChainId: gatewayChainID || 54321,
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
