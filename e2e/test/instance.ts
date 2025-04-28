import {
  clientKeyDecryptor,
  createInstance as createFhevmInstance,
  getCiphertextCallParams,
} from "@fhevm/sdk";
import { readFileSync } from "fs";
import { ethers, ethers as hethers, network } from "hardhat";
import { homedir } from "os";
import path from "path";

import type { Signers } from "./signers";
import { FhevmInstances } from "./types";

const FHE_CLIENT_KEY_PATH = process.env.FHE_CLIENT_KEY_PATH;

let clientKey: Uint8Array | undefined;

const kmsAdd = process.env.KMS_VERIFIER_CONTRACT_ADDRESS;
const aclAdd = process.env.ACL_CONTRACT_ADDRESS;
const gatewayChainID = +process.env.CHAIN_ID_GATEWAY!;
const verifyingContract = process.env.DECRYPTION_ADDRESS!;
const relayerUrl = process.env.RELAYER_URL!;

export const createInstances = async (
  accounts: Signers
): Promise<FhevmInstances> => {
  // Create instance
  const instances: FhevmInstances = {} as FhevmInstances;
  await Promise.all(
    Object.keys(accounts).map(async (k) => {
      instances[k as keyof FhevmInstances] = await createInstance();
    })
  );
  return instances;
};

export const createInstance = async () => {
  console.log("relayer url given to create instance", relayerUrl);
  const instance = await createFhevmInstance({
    verifyingContractAddress: verifyingContract,
    kmsContractAddress: kmsAdd,
    aclContractAddress: aclAdd,
    network: network.config.url,
    relayerUrl: relayerUrl,
    gatewayChainId: gatewayChainID,
  });
  return instance;
};

const getCiphertext = async (
  handle: string,
  ethers: typeof hethers
): Promise<string> => {
  return ethers.provider.call(getCiphertextCallParams(handle));
};

const getDecryptor = () => {
  if (clientKey == null) {
    if (FHE_CLIENT_KEY_PATH) {
      clientKey = readFileSync(FHE_CLIENT_KEY_PATH);
    } else {
      const home = homedir();
      const clientKeyPath = path.join(home, "network-fhe-keys/cks");
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
  return getDecryptor().decryptBool(await getCiphertext(handle, ethers));
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
  return getDecryptor().decrypt8(await getCiphertext(handle, ethers));
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
  return getDecryptor().decrypt16(await getCiphertext(handle, ethers));
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
  return getDecryptor().decrypt32(await getCiphertext(handle, ethers));
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
  return getDecryptor().decrypt64(await getCiphertext(handle, ethers));
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
  return getDecryptor().decrypt128(await getCiphertext(handle, ethers));
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
  return getDecryptor().decrypt256(await getCiphertext(handle, ethers));
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
  return getDecryptor().decryptAddress(await getCiphertext(handle, ethers));
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
  return getDecryptor().decryptEbytes64(await getCiphertext(handle, ethers));
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
  return getDecryptor().decryptEbytes128(await getCiphertext(handle, ethers));
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
  return getDecryptor().decryptEbytes256(await getCiphertext(handle, ethers));
};
