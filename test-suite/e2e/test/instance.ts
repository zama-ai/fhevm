import { network } from 'hardhat';
import { vars } from 'hardhat/config';

import { FhevmSdk } from './sdk/fhevm-sdk/sdk';
import type { Signers } from './signers';
import { FhevmInstances } from './types';

const MainnetConfig = {
  aclContractAddress: '0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6',
  kmsContractAddress: '0x77627828a55156b04Ac0DC0eb30467f1a552BB03',
  inputVerifierContractAddress: '0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2',
  verifyingContractAddressDecryption: '0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24',
  verifyingContractAddressInputVerification: '0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287',
  chainId: 1,
  gatewayChainId: 261131,
  relayerUrl: 'https://relayer.mainnet.zama.org/v2',
} as const;

const SepoliaConfig = {
  aclContractAddress: '0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D',
  kmsContractAddress: '0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A',
  inputVerifierContractAddress: '0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0',
  verifyingContractAddressDecryption: '0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478',
  verifyingContractAddressInputVerification: '0x483b9dE06E4E4C7D35CCf5837A1668487406D955',
  chainId: 11155111,
  gatewayChainId: 10901,
  relayerUrl: 'https://relayer.testnet.zama.org/v2',
} as const;

const defaults = (() => {
  const chainId = network.config.chainId;
  if (network.name === 'sepolia' || chainId === 11155111) return SepoliaConfig;
  if (network.name === 'mainnet' || chainId === 1) return MainnetConfig;
  return undefined;
})();

const requireEnv = (value: string | undefined, name: string): string => {
  if (!value) throw new Error(`${name} is required`);
  return value;
};

const kmsVerifierAddress = requireEnv(
  process.env.KMS_VERIFIER_CONTRACT_ADDRESS || defaults?.kmsContractAddress,
  'KMS_VERIFIER_CONTRACT_ADDRESS',
);

const aclAddress = requireEnv(process.env.ACL_CONTRACT_ADDRESS || defaults?.aclContractAddress, 'ACL_CONTRACT_ADDRESS');

// Coprocessor/executor address defaults (not in SDK, values from ZamaConfig.sol)
const coprocessorDefaults: Record<string, string> = {
  sepolia: '0x92C920834Ec8941d2C77D188936E1f7A6f49c127',
  mainnet: '0xD82385dADa1ae3E969447f20A3164F6213100e75',
};
const coprocessorAddress = requireEnv(
  process.env.FHEVM_EXECUTOR_CONTRACT_ADDRESS || coprocessorDefaults[network.name],
  'FHEVM_EXECUTOR_CONTRACT_ADDRESS',
);

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
// Auth is optional since internal smoke tests don't go through Kong
const apiKey = process.env.ZAMA_FHEVM_API_KEY ?? vars.get('ZAMA_FHEVM_API_KEY', '');
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
  const cfg = {
    verifyingContractAddressDecryption,
    verifyingContractAddressInputVerification,
    kmsContractAddress: kmsVerifierAddress,
    inputVerifierContractAddress: inputAdd,
    aclContractAddress: aclAddress,
    rpcUrl: (network.config as { url: string }).url,
    relayerUrl,
    gatewayChainId: gatewayChainID,
    chainId: hostChainID,
    ...(auth ? { auth } : {}),
  };
  return FhevmSdk.create(cfg);
};

// Export coprocessor config addresses for smoke tests
export { aclAddress, coprocessorAddress, kmsVerifierAddress };
