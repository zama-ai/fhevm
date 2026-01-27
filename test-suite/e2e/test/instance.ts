import { MainnetConfig, SepoliaConfig, createInstance as createFhevmInstance } from '@zama-fhe/relayer-sdk/node';
import { network } from 'hardhat';
import { vars } from 'hardhat/config';

import type { Signers } from './signers';
import { FhevmInstances } from './types';

const defaults = (() => {
  const chainId = network.config.chainId;
  if (network.name === 'sepolia' || chainId === 11155111) return SepoliaConfig;
  if (network.name === 'mainnet' || chainId === 1) return MainnetConfig;
  return undefined;
})();

const kmsAdd = process.env.KMS_VERIFIER_CONTRACT_ADDRESS || defaults?.kmsContractAddress;
if (!kmsAdd) throw new Error('KMS_VERIFIER_CONTRACT_ADDRESS is required');

const aclAdd = process.env.ACL_CONTRACT_ADDRESS || defaults?.aclContractAddress;
if (!aclAdd) throw new Error('ACL_CONTRACT_ADDRESS is required');

const inputAdd = process.env.INPUT_VERIFIER_CONTRACT_ADDRESS || defaults?.inputVerifierContractAddress;
if (!inputAdd) throw new Error('INPUT_VERIFIER_CONTRACT_ADDRESS is required');

const gatewayChainID = Number(process.env.CHAIN_ID_GATEWAY) || defaults?.gatewayChainId;
if (!gatewayChainID) throw new Error('CHAIN_ID_GATEWAY is required');

const hostChainID = Number(process.env.CHAIN_ID_HOST) || defaults?.chainId;
if (!hostChainID) throw new Error('CHAIN_ID_HOST is required');

const verifyingContractAddressDecryption =
  process.env.DECRYPTION_ADDRESS || defaults?.verifyingContractAddressDecryption;
if (!verifyingContractAddressDecryption) throw new Error('DECRYPTION_ADDRESS is required');

const verifyingContractAddressInputVerification =
  process.env.INPUT_VERIFICATION_ADDRESS || defaults?.verifyingContractAddressInputVerification;
if (!verifyingContractAddressInputVerification) throw new Error('INPUT_VERIFICATION_ADDRESS is required');

const relayerUrl = process.env.RELAYER_URL || defaults?.relayerUrl;
if (!relayerUrl) throw new Error('RELAYER_URL is required');

// API key is a secret - support hardhat vars for secure storage
const apiKey = process.env.ZAMA_FHEVM_API_KEY ?? vars.get('ZAMA_FHEVM_API_KEY', '');
const isMainnet = network.name === 'mainnet' || network.config.chainId === 1;
if (isMainnet && !apiKey) {
  throw new Error('ZAMA_FHEVM_API_KEY is required for mainnet');
}
const auth = apiKey ? { __type: 'ApiKeyHeader' as const, value: apiKey } : undefined;

export const createInstances = async (accounts: Signers): Promise<FhevmInstances> => {
  const instances: FhevmInstances = {} as FhevmInstances;
  await Promise.all(
    Object.keys(accounts).map(async (k) => {
      instances[k as keyof FhevmInstances] = await createInstance();
    }),
  );
  return instances;
};

export const createInstance = async () => {
  return createFhevmInstance({
    verifyingContractAddressDecryption,
    verifyingContractAddressInputVerification,
    kmsContractAddress: kmsAdd,
    inputVerifierContractAddress: inputAdd,
    aclContractAddress: aclAdd,
    network: network.config.url,
    relayerUrl,
    gatewayChainId: gatewayChainID,
    chainId: hostChainID,
    ...(auth ? { auth } : {}),
  });
};
