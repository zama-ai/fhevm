import { createInstance as createFhevmInstance, MainnetConfig, SepoliaConfig } from '@zama-fhe/relayer-sdk/node';
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

const kmsAdd =
  process.env.KMS_VERIFIER_CONTRACT_ADDRESS ??
  vars.get('KMS_VERIFIER_CONTRACT_ADDRESS', defaults?.kmsContractAddress ?? '');
if (!kmsAdd) throw new Error('Missing required env var KMS_VERIFIER_CONTRACT_ADDRESS');

const aclAdd =
  process.env.ACL_CONTRACT_ADDRESS ?? vars.get('ACL_CONTRACT_ADDRESS', defaults?.aclContractAddress ?? '');
if (!aclAdd) throw new Error('Missing required env var ACL_CONTRACT_ADDRESS');

const inputAdd =
  process.env.INPUT_VERIFIER_CONTRACT_ADDRESS ??
  vars.get('INPUT_VERIFIER_CONTRACT_ADDRESS', defaults?.inputVerifierContractAddress ?? '');
if (!inputAdd) throw new Error('Missing required env var INPUT_VERIFIER_CONTRACT_ADDRESS');

const gatewayChainRaw =
  process.env.CHAIN_ID_GATEWAY ?? vars.get('CHAIN_ID_GATEWAY', defaults?.gatewayChainId?.toString() ?? '');
const gatewayChainID = gatewayChainRaw ? Number(gatewayChainRaw) : undefined;
if (!Number.isFinite(gatewayChainID)) throw new Error('Missing required env var CHAIN_ID_GATEWAY');

const hostChainRaw =
  process.env.CHAIN_ID_HOST ?? vars.get('CHAIN_ID_HOST', defaults?.chainId?.toString() ?? '');
const hostChainID = hostChainRaw ? Number(hostChainRaw) : undefined;
if (!Number.isFinite(hostChainID)) throw new Error('Missing required env var CHAIN_ID_HOST');

const verifyingContractAddressDecryption =
  process.env.DECRYPTION_ADDRESS ??
  vars.get('DECRYPTION_ADDRESS', defaults?.verifyingContractAddressDecryption ?? '');
if (!verifyingContractAddressDecryption) {
  throw new Error('Missing required env var DECRYPTION_ADDRESS');
}

const verifyingContractAddressInputVerification =
  process.env.INPUT_VERIFICATION_ADDRESS ??
  vars.get(
    'INPUT_VERIFICATION_ADDRESS',
    defaults?.verifyingContractAddressInputVerification ?? '',
  );
if (!verifyingContractAddressInputVerification) {
  throw new Error('Missing required env var INPUT_VERIFICATION_ADDRESS');
}

const relayerUrl = process.env.RELAYER_URL ?? vars.get('RELAYER_URL', defaults?.relayerUrl ?? '');
if (!relayerUrl) throw new Error('Missing required env var RELAYER_URL');

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
