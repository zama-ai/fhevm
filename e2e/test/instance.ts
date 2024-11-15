import dotenv from "dotenv";
import { clientKeyDecryptor, createInstance as createFhevmInstance, getCiphertextCallParams } from "fhevmjs/node";
import { readFileSync } from "fs";
import * as fs from "fs";
import { ethers, ethers as hethers, network } from "hardhat";
import { homedir } from "os";
import path from "path";

import { createEncryptedInputMocked, reencryptRequestMocked } from "./fhevmjsMocked";
import type { Signers } from "./signers";
import { FhevmInstances } from "./types";

const parsedEnv = dotenv.parse(fs.readFileSync(".env"));

const FHE_CLIENT_KEY_PATH = process.env.FHE_CLIENT_KEY_PATH;

let clientKey: Uint8Array | undefined;

export const createInstances = async (accounts: Signers): Promise<FhevmInstances> => {
  // Create instance
  const instances: FhevmInstances = {} as FhevmInstances;

  await Promise.all(
    Object.keys(accounts).map(async (k) => {
      instances[k as keyof FhevmInstances] = await createInstance();
    }),
  );

  return instances;
};

export const createInstance = async () => {
  const instance = await createFhevmInstance({
    networkUrl: network.config.url,
    gatewayUrl: parsedEnv.GATEWAY_URL,
    aclContractAddress: parsedEnv.ACL_CONTRACT_ADDRESS,
    kmsContractAddress: parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS,
  });
  return instance;
};

const getCiphertext = async (handle: bigint, ethers: typeof hethers): Promise<string> => {
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
 * In production, decryption is only possible via an asyncronous on-chain call to the Gateway.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bool}
 */
export const decryptBool = async (handle: bigint): Promise<boolean> => {
  if (network.name === "hardhat") {
    await awaitCoprocessor();
    return (await getClearText(handle)) === "1";
  } else {
    return getDecryptor().decryptBool(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Gateway.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt4 = async (handle: bigint): Promise<bigint> => {
  if (network.name === "hardhat") {
    await awaitCoprocessor();
    return BigInt(await getClearText(handle));
  } else {
    return getDecryptor().decrypt4(await getCiphertext(handle, ethers));
  }
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It cannot be used in production code, since it requires the FHE private key for decryption.
 * In production, decryption is only possible via an asyncronous on-chain call to the Gateway.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt8 = async (handle: bigint): Promise<bigint> => {
  if (network.name === "hardhat") {
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
 * In production, decryption is only possible via an asyncronous on-chain call to the Gateway.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt16 = async (handle: bigint): Promise<bigint> => {
  if (network.name === "hardhat") {
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
 * In production, decryption is only possible via an asyncronous on-chain call to the Gateway.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt32 = async (handle: bigint): Promise<bigint> => {
  if (network.name === "hardhat") {
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
 * In production, decryption is only possible via an asyncronous on-chain call to the Gateway.
 *
 * @param {bigint} a handle to decrypt
 * @returns {bigint}
 */
export const decrypt64 = async (handle: bigint): Promise<bigint> => {
  if (network.name === "hardhat") {
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
 * In production, decryption is only possible via an asyncronous on-chain call to the Gateway.
 *
 * @param {bigint} a handle to decrypt
 * @returns {string}
 */
export const decryptAddress = async (handle: bigint): Promise<string> => {
  if (network.name === "hardhat") {
    await awaitCoprocessor();
    const bigintAdd = BigInt(await getClearText(handle));
    const handleStr = "0x" + bigintAdd.toString(16).padStart(40, "0");
    return handleStr;
  } else {
    return getDecryptor().decryptAddress(await getCiphertext(handle, ethers));
  }
};
