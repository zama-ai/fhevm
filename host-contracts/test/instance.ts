import type { FhevmInstance } from '@zama-fhe/relayer-sdk/node';
import type { ethers as EthersT } from 'ethers';
import { ethers } from 'hardhat';

import {
  assertNetwork,
  awaitCoprocessor,
  createInstanceMocked,
  getClearText,
  getEnvFhevmMockConfig,
  getTxHCUFromTxReceipt,
} from './fhevmjsMocked';
import type { Signers } from './signers';
import { FhevmInstances } from './types';

const abiAcl = [
  'function delegateForUserDecryption(address,address,uint64)',
  'function revokeDelegationForUserDecryption(address,address)',
];

export const createInstances = async (accounts: Signers): Promise<FhevmInstances> => {
  assertNetwork('hardhat');

  const instances: FhevmInstances = {} as FhevmInstances;
  await Promise.all(
    Object.keys(accounts).map(async (k) => {
      instances[k as keyof FhevmInstances] = await createInstanceMocked(getEnvFhevmMockConfig());
    }),
  );

  return instances;
};

export const createInstance = async (): Promise<FhevmInstance> => {
  assertNetwork('hardhat');
  const instance = await createInstanceMocked(getEnvFhevmMockConfig());
  return instance;
};

export const delegateUserDecryption = async (
  delegator: EthersT.Signer,
  delegate: string,
  contractAddress: string,
  expirationDate: bigint,
): Promise<EthersT.TransactionResponse> => {
  const aclContract = new ethers.Contract(getEnvFhevmMockConfig().aclContractAddress, abiAcl, delegator);
  return aclContract.delegateForUserDecryption(delegate, contractAddress, expirationDate);
};

export const revokeUserDecryptionDelegation = async (
  delegator: EthersT.Signer,
  delegate: string,
  contractAddress: string,
): Promise<EthersT.TransactionResponse> => {
  const aclContract = new ethers.Contract(getEnvFhevmMockConfig().aclContractAddress, abiAcl, delegator);
  return aclContract.revokeDelegationForUserDecryption(delegate, contractAddress);
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  return (await getClearText(handle)) === '1';
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  return BigInt(await getClearText(handle));
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  return BigInt(await getClearText(handle));
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  return BigInt(await getClearText(handle));
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  return BigInt(await getClearText(handle));
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  return BigInt(await getClearText(handle));
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  return BigInt(await getClearText(handle));
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
  assertNetwork('hardhat');
  await awaitCoprocessor();
  const bigintAdd = BigInt(await getClearText(handle));
  const handleStr = '0x' + bigintAdd.toString(16).padStart(40, '0');
  return handleStr;
};

export { getTxHCUFromTxReceipt, awaitCoprocessor };
