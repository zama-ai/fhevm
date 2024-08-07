import { Signer } from 'ethers';
import fhevmjs, { FhevmInstance, getPublicKeyCallParams } from 'fhevmjs';
import { ethers as hethers } from 'hardhat';

let publicKey: string | undefined;
let chainId: number;

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
