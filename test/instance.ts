import { Signer } from 'ethers';
import fhevmjs, { clientKeyDecryptor, getCiphertextCallParams, getPublicKeyCallParams } from 'fhevmjs';
import { readFileSync } from 'fs';
import { ethers, ethers as hethers } from 'hardhat';
import { homedir } from 'os';
import path from 'path';

import type { Signers } from './signers';
import { FhevmInstances } from './types';

const hre = require('hardhat');

const HARDHAT_NETWORK = process.env.HARDHAT_NETWORK;
const FHE_CLIENT_KEY_PATH = process.env.FHE_CLIENT_KEY_PATH;

let publicKey: string | undefined;
let clientKey: Uint8Array | undefined;
let chainId: number;

export const createInstances = async (
  contractAddress: string,
  ethers: typeof hethers,
  accounts: Signers,
): Promise<FhevmInstances> => {
  // Create instance
  const instances: FhevmInstances = {} as FhevmInstances;
  await Promise.all(
    Object.keys(accounts).map(async (k) => {
      instances[k as keyof FhevmInstances] = await createInstance(
        contractAddress,
        accounts[k as keyof Signers],
        ethers,
      );
    }),
  );

  return instances;
};

export const createInstance = async (contractAddress: string, account: Signer, ethers: typeof hethers) => {
  const provider = ethers.provider;
  const network = await provider.getNetwork();
  chainId = +network.chainId.toString(); // Need to be a number
  try {
    // Get blockchain public key
    const ret = await provider.call(getPublicKeyCallParams());
    const decoded = ethers.AbiCoder.defaultAbiCoder().decode(['bytes'], ret);
    publicKey = decoded[0];
  } catch (e) {
    publicKey = undefined;
  }
  const instance = await fhevmjs.createInstance({ chainId, publicKey, networkUrl: hre.network.config.url });
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
      const clientKeyPath = path.join(home, 'network-fhe-keys/cks');
      clientKey = readFileSync(clientKeyPath);
    }
  }
  return clientKeyDecryptor(clientKey);
};

export const decryptBool = async (handle: bigint): Promise<boolean> => {
  return getDecryptor().decryptBool(await getCiphertext(handle, ethers));
};

export const decrypt4 = async (handle: bigint): Promise<number> => {
  return getDecryptor().decrypt4(await getCiphertext(handle, ethers));
};

export const decrypt8 = async (handle: bigint): Promise<number> => {
  return getDecryptor().decrypt8(await getCiphertext(handle, ethers));
};

export const decrypt16 = async (handle: bigint): Promise<number> => {
  return getDecryptor().decrypt16(await getCiphertext(handle, ethers));
};

export const decrypt32 = async (handle: bigint): Promise<number> => {
  return getDecryptor().decrypt32(await getCiphertext(handle, ethers));
};

export const decrypt64 = async (handle: bigint): Promise<bigint> => {
  return getDecryptor().decrypt64(await getCiphertext(handle, ethers));
};

export const decryptAddress = async (handle: bigint): Promise<string> => {
  return getDecryptor().decryptAddress(await getCiphertext(handle, ethers));
};
