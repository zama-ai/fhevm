import dotenv from 'dotenv';
import type { ethers as EthersT } from 'ethers';
import * as fs from 'node:fs';

import { getRequiredEnvVar } from '../tasks/env.js';
import { awaitCoprocessor, coprocDbReadCleartext } from './coprocessorUtils.js';
import { createEncryptedInputMocked, userDecryptRequestMocked } from './fhevmjsMocked.js';
import { chainId as networkChainId, ethers, isInProcessNetwork, networkName } from './hardhat.js';
import { createEIP712Mocked } from './legacyRelayerSdk/eip712.js';
import type { FhevmInstance, PublicDecryptResults, PublicParams } from './legacyRelayerSdk/types.js';
import type { Signers } from './signers.js';
import type { FhevmInstances } from './types.js';
import { HOST_ADDRESSES_ENV_PATH } from './utils.js';

const abiAcl = [
  'function delegateForUserDecryption(address,address,uint64)',
  'function revokeDelegationForUserDecryption(address,address)',
];

const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
const aclAdd = parsedEnv.ACL_CONTRACT_ADDRESS;

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

// eslint-disable-next-line @typescript-eslint/require-await
const createInstanceMocked = async (): Promise<FhevmInstance> => {
  const chainId = networkChainId;
  if (chainId === undefined) {
    throw new Error(`No chainId configured for network "${networkName}" in hardhat.config.ts`);
  }
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
    createEIP712: createEIP712Mocked(getRequiredEnvVar('DECRYPTION_ADDRESS'), chainId),
    publicDecrypt: function (_handles: Array<string | Uint8Array>): Promise<PublicDecryptResults> {
      throw new Error('Function not implemented in mock mode.');
    },
    getPublicParams: function (_bits: keyof PublicParams): { publicParams: Uint8Array; publicParamsId: string } | null {
      throw new Error('Function not implemented in mock mode.');
    },
  };
  return instance;
};

export const createInstances = async (accounts: Signers): Promise<FhevmInstances> => {
  // Create instance
  const instances: FhevmInstances = {} as FhevmInstances;
  if (isInProcessNetwork) {
    await Promise.all(
      Object.keys(accounts).map(async (k) => {
        instances[k as keyof FhevmInstances] = await createInstanceMocked();
      }),
    );
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    return (await coprocDbReadCleartext(handle)) === '1';
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    return BigInt(await coprocDbReadCleartext(handle));
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    return BigInt(await coprocDbReadCleartext(handle));
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    return BigInt(await coprocDbReadCleartext(handle));
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    return BigInt(await coprocDbReadCleartext(handle));
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    return BigInt(await coprocDbReadCleartext(handle));
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    return BigInt(await coprocDbReadCleartext(handle));
  } else {
    throw new Error(`network ${networkName} is not supported`);
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
  if (isInProcessNetwork) {
    await awaitCoprocessor();
    const bigintAdd = BigInt(await coprocDbReadCleartext(handle));
    const handleStr = ('0x' + bigintAdd.toString(16).padStart(40, '0')) as `0x${string}`;
    return handleStr;
  } else {
    throw new Error(`network ${networkName} is not supported`);
  }
};
