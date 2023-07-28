import { Signer } from 'ethers';
import fhevmjs, { FhevmInstance } from 'fhevmjs';
import { ethers as hethers } from 'hardhat';

import { FhevmInstances, Signers } from './types';

let publicKey: string;
let chainId: number;

export const createInstances = async (
  contractAddress: string,
  ethers: typeof hethers,
  accounts: Signers,
): Promise<FhevmInstances> => {
  if (!publicKey || !chainId) {
    // 1. Get chain id
    const provider = ethers.provider;

    const network = await provider.getNetwork();
    chainId = +network.chainId.toString(); // Need to be a number

    // Get blockchain public key
    publicKey = await provider.call({
      to: '0x0000000000000000000000000000000000000044',
    });
  }

  // Create instance
  const instances: FhevmInstances = {} as FhevmInstances;
  await Promise.all(
    Object.keys(accounts).map(async (k) => {
      const instance = await fhevmjs.createInstance({ chainId, publicKey });
      await generateToken(contractAddress, accounts[k as keyof Signers], instance);
      instances[k as keyof FhevmInstances] = instance;
    }),
  );

  return instances;
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
