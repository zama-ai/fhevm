import { Signer } from 'ethers';
import fhevmjs, { FhevmInstance } from 'fhevmjs';
import { ethers as hethers } from 'hardhat';

export const FHE_LIB_ADDRESS = '0x000000000000000000000000000000000000005d';

let publicKey: string;
let chainId: number;

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
