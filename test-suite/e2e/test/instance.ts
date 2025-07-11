import { createInstance as createFhevmInstance } from '@zama-fhe/relayer-sdk';
import { network } from 'hardhat';

import type { Signers } from './signers';
import { FhevmInstances } from './types';

const kmsAdd = process.env.KMS_VERIFIER_CONTRACT_ADDRESS;
const aclAdd = process.env.ACL_CONTRACT_ADDRESS;
const inputAdd = process.env.INPUT_VERIFIER_CONTRACT_ADDRESS;
const gatewayChainID = +process.env.CHAIN_ID_GATEWAY!;
const verifyingContractAddressDecryption = process.env.DECRYPTION_ADDRESS!;
const verifyingContractAddressInputVerification = process.env.INPUT_VERIFICATION_ADDRESS!;
const relayerUrl = process.env.RELAYER_URL!;
const apiKey: string | undefined = process.env.RELAYER_API_KEY;

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
  console.log('relayer url given to create instance', relayerUrl);
  console.log('network', network.name, network.config.url);
  const instance = await createFhevmInstance({
    verifyingContractAddressDecryption: verifyingContractAddressDecryption,
    verifyingContractAddressInputVerification: verifyingContractAddressInputVerification,
    kmsContractAddress: kmsAdd,
    inputVerifierContractAddress: inputAdd,
    aclContractAddress: aclAdd,
    network: network.config.url,
    relayerUrl: relayerUrl,
    gatewayChainId: gatewayChainID,
    apiKey: apiKey,
  });
  return instance;
};
