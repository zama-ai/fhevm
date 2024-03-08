import { toBufferBE } from 'bigint-buffer';
import { Signer } from 'ethers';
import fhevmjs, { FhevmInstance, getPublicKeyCallParams } from 'fhevmjs';
import { ethers as hethers } from 'hardhat';

import type { Signers } from './signers';
import { FhevmInstances } from './types';

const HARDHAT_NETWORK = process.env.HARDHAT_NETWORK;

let publicKey: string | undefined;
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
    instance.decrypt = (_, hexadecimalString) => BigInt(hexadecimalString);
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
