import dotenv from 'dotenv';
import type { ethers as EthersT } from 'ethers';
import * as fs from 'fs';
import { ethers, network } from 'hardhat';

import { awaitCoprocessor, getClearText } from './coprocessorUtils';
import { createEncryptedInputMocked, userDecryptRequestMocked } from './fhevmjsMocked';
import type { FhevmInstances } from './types';
import type { Signers } from './signers';
import type { FhevmInstance, PublicDecryptResults, PublicParams } from './legacyRelayerSdk/types';
import { createEIP712Mocked } from './legacyRelayerSdk/eip712';

const abiAcl = [
  'function delegateForUserDecryption(address,address,uint64)',
  'function revokeDelegationForUserDecryption(address,address)',
];

const parsedEnv = dotenv.parse(fs.readFileSync('./fhevmTemp/addresses/.env.host'));
const aclAdd = parsedEnv.ACL_CONTRACT_ADDRESS;
const verifyingContractAddressDecryption = process.env.DECRYPTION_ADDRESS!;

export const delegateUserDecryption = async (
  delegator: EthersT.Signer,
  delegate: string,
  contractAddress: string,
  expirationDate: bigint,
): Promise<unknown> => {
  const aclContract = new ethers.Contract(aclAdd, abiAcl, delegator);
  return aclContract.delegateForUserDecryption(delegate, contractAddress, expirationDate);
};

export const revokeUserDecryptionDelegation = async (
  delegator: EthersT.Signer,
  delegate: string,
  contractAddress: string,
): Promise<unknown> => {
  const aclContract = new ethers.Contract(aclAdd, abiAcl, delegator);
  return aclContract.revokeDelegationForUserDecryption(delegate, contractAddress);
};

const createInstanceMocked = async (): Promise<FhevmInstance> => {
  const instance: FhevmInstance = {
    userDecrypt: userDecryptRequestMocked,
    createEncryptedInput: createEncryptedInputMocked,
    getPublicKey: () => {
      throw new Error('Function not implemented in mock mode.');
    },
    generateKeypair: () => ({
      publicKey: ethers.hexlify(ethers.randomBytes(32)).slice(2),
      privateKey: ethers.hexlify(ethers.randomBytes(32)).slice(2),
    }),
    createEIP712: createEIP712Mocked(verifyingContractAddressDecryption, network.config.chainId!),
    publicDecrypt: function (handles: (string | Uint8Array)[]): Promise<PublicDecryptResults> {
      throw new Error('Function not implemented in mock mode.');
    },
    getPublicParams: function (bits: keyof PublicParams): { publicParams: Uint8Array; publicParamsId: string } | null {
      throw new Error('Function not implemented in mock mode.');
    },
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
    throw new Error(`network.name ${network.name} is not supported`);
  }
  return instances;
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
    throw new Error(`network.name ${network.name} is not supported`);
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
    throw new Error(`network.name ${network.name} is not supported`);
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
    throw new Error(`network.name ${network.name} is not supported`);
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
    throw new Error(`network.name ${network.name} is not supported`);
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
    throw new Error(`network.name ${network.name} is not supported`);
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
    throw new Error(`network.name ${network.name} is not supported`);
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
    throw new Error(`network.name ${network.name} is not supported`);
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
    const handleStr = ('0x' + bigintAdd.toString(16).padStart(40, '0')) as `0x${string}`;
    return handleStr;
  } else {
    throw new Error(`network.name ${network.name} is not supported`);
  }
};
