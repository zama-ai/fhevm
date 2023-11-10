import { Signer } from 'ethers';
import fhevmjs, { FhevmInstance } from 'fhevmjs';
import { ethers as hethers } from 'hardhat';

import { FHE_LIB_ADDRESS } from './generated';
import type { Signers } from './signers';
import { FhevmInstances } from './types';

let publicKey: string;
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
  if (!publicKey || !chainId) {
    // 1. Get chain id
    const provider = ethers.provider;

    const network = await provider.getNetwork();
    chainId = +network.chainId.toString(); // Need to be a number

    // Get blockchain public key
    const ret = await provider.call({
      to: FHE_LIB_ADDRESS,
      // first four bytes of keccak256('fhePubKey(bytes1)') + 1 byte for library
      data: '0xd9d47bb001',
    });
    const decoded = ethers.AbiCoder.defaultAbiCoder().decode(['bytes'], ret);
    publicKey = decoded[0];
  }
  const instance = await fhevmjs.createInstance({ chainId, publicKey });
  await generateToken(contractAddress, account, instance);
  return instance;
};

const generateToken = async (contractAddress: string, signer: Signer, instance: FhevmInstance) => {
  // Generate token to decrypt
  const generatedToken = instance.generateToken({
    verifyingContract: contractAddress,
  });

  // Sign the public key
  const signature = await signer.signTypedData(
    generatedToken.token.domain,
    { Reencrypt: generatedToken.token.types.Reencrypt }, // Need to remove EIP712Domain from types
    generatedToken.token.message,
  );
  instance.setTokenSignature(contractAddress, signature);
};
