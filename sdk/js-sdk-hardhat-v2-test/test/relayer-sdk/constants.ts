import type { FhevmInstanceConfig } from './types.js';

////////////////////////////////////////////////////////////////////////////////
// MainnetConfig
////////////////////////////////////////////////////////////////////////////////

export const MainnetRelayerBaseUrl = 'https://relayer.mainnet.zama.org';
export const MainnetRelayerUrlV1 = `${MainnetRelayerBaseUrl}/v1`;
export const MainnetRelayerUrlV2 = `${MainnetRelayerBaseUrl}/v2`;

export const MainnetConfigBase: Omit<FhevmInstanceConfig, 'relayerUrl' | 'network'> = {
  aclContractAddress: '0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6',
  kmsContractAddress: '0x77627828a55156b04Ac0DC0eb30467f1a552BB03',
  inputVerifierContractAddress: '0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2',
  verifyingContractAddressDecryption: '0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24',
  verifyingContractAddressInputVerification: '0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287',
  chainId: 1,
  gatewayChainId: 261131,
} as const;
Object.freeze(MainnetConfigBase);

export const MainnetConfig: Omit<FhevmInstanceConfig, 'network'> = {
  ...MainnetConfigBase,
  relayerUrl: MainnetRelayerBaseUrl,
} as const;
Object.freeze(MainnetConfig);

export const MainnetConfigV1: Omit<FhevmInstanceConfig, 'network'> = {
  ...MainnetConfigBase,
  relayerUrl: MainnetRelayerUrlV1,
} as const;
Object.freeze(MainnetConfigV1);

export const MainnetConfigV2: Omit<FhevmInstanceConfig, 'network'> = {
  ...MainnetConfigBase,
  relayerUrl: MainnetRelayerUrlV2,
} as const;
Object.freeze(MainnetConfigV2);

////////////////////////////////////////////////////////////////////////////////
// SepoliaConfig
////////////////////////////////////////////////////////////////////////////////

export const SepoliaRelayerBaseUrl = 'https://relayer.testnet.zama.org';
export const SepoliaRelayerUrlV1 = `${SepoliaRelayerBaseUrl}/v1`;
export const SepoliaRelayerUrlV2 = `${SepoliaRelayerBaseUrl}/v2`;

export const SepoliaConfigBase: Omit<FhevmInstanceConfig, 'relayerUrl' | 'network'> = {
  aclContractAddress: '0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D',
  kmsContractAddress: '0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A',
  inputVerifierContractAddress: '0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0',
  verifyingContractAddressDecryption: '0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478',
  verifyingContractAddressInputVerification: '0x483b9dE06E4E4C7D35CCf5837A1668487406D955',
  chainId: 11155111,
  gatewayChainId: 10901,
} as const;
Object.freeze(SepoliaConfigBase);

export const SepoliaConfig: Omit<FhevmInstanceConfig, 'network'> = {
  ...SepoliaConfigBase,
  relayerUrl: SepoliaRelayerBaseUrl,
} as const;
Object.freeze(SepoliaConfig);

export const SepoliaConfigV1: Omit<FhevmInstanceConfig, 'network'> = {
  ...SepoliaConfigBase,
  relayerUrl: SepoliaRelayerUrlV1,
} as const;
Object.freeze(SepoliaConfigV1);

export const SepoliaConfigV2: Omit<FhevmInstanceConfig, 'network'> = {
  ...SepoliaConfigBase,
  relayerUrl: SepoliaRelayerUrlV2,
} as const;
Object.freeze(SepoliaConfigV2);
