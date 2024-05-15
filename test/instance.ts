import { toBufferBE } from 'bigint-buffer';
import { Signer } from 'ethers';
import fhevmjs, { FhevmInstance, clientKeyDecryptor, getCiphertextCallParams, getPublicKeyCallParams } from 'fhevmjs';
import { readFileSync } from 'fs';
import { ethers, ethers as hethers } from 'hardhat';
import { homedir } from 'os';
import path from 'path';

import type { Signers } from './signers';
import { FhevmInstances } from './types';

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
  // 1. Get chain id
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

  const instance = await fhevmjs.createInstance({ chainId, publicKey });

  if (HARDHAT_NETWORK === 'hardhat') {
    instance.encryptBool = createUintToUint8ArrayFunction(1);
    instance.encrypt4 = createUintToUint8ArrayFunction(4);
    instance.encrypt8 = createUintToUint8ArrayFunction(8);
    instance.encrypt16 = createUintToUint8ArrayFunction(16);
    instance.encrypt32 = createUintToUint8ArrayFunction(32);
    instance.encrypt64 = createUintToUint8ArrayFunction(64);
    instance.encryptAddress = createUintToUint8ArrayFunction(160);
    instance.decrypt = (_, hexadecimalString) => BigInt(hexadecimalString);
    instance.decryptAddress = (_, hexadecimalString) => ethers.getAddress(hexadecimalString.slice(26, 66));
  }
  await generatePublicKey(contractAddress, account, instance);

  return instance;
};

const generatePublicKey = async (contractAddress: string, signer: Signer, instance: FhevmInstance) => {
  // Generate token to decrypt
  const generatedToken = instance.generatePublicKey({
    verifyingContract: contractAddress,
  });
  // Sign the public key
  const signature = await signer.signTypedData(
    generatedToken.eip712.domain,
    { Reencrypt: generatedToken.eip712.types.Reencrypt }, // Need to remove EIP712Domain from types
    generatedToken.eip712.message,
  );
  instance.setSignature(contractAddress, signature);
};

function createUintToUint8ArrayFunction(numBits: number) {
  const numBytes = Math.ceil(numBits / 8);
  return function (uint: number | bigint | boolean) {
    const buffer = toBufferBE(BigInt(uint), numBytes);
    return buffer;
  };
}

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
