import { registerAs } from '@nestjs/config';

export type EtherProvider = 'Etherscan' | 'Unknown';
export const SEPOLIA_CHAIN_ID = '11155111';
type ChainId = typeof SEPOLIA_CHAIN_ID;

export interface EtherConfig {
  provider: EtherProvider;
  apiEndpoint: string;
  rpcEndpoint: string;
  apiKey?: string;
}

const configs: Record<
  ChainId,
  Omit<EtherConfig, 'apiKey'> & { apiKey: () => string | undefined }
> = {
  '11155111': {
    provider: 'Etherscan',
    apiEndpoint: 'https://api-sepolia.etherscan.io/api',
    rpcEndpoint: 'https://rpc.sepolia.org',
    apiKey: () => process.env.ETHERSCAN_SEPOLIA_APIKEY,
  },
};

export default registerAs('ether', () => {
  const chainId = process.env.CHAIN_ID ?? '';
  const config: Omit<EtherConfig, 'apiKey'> & {
    apiKey: () => string | undefined;
  } =
    chainId in configs
      ? configs[chainId as ChainId]
      : {
          provider: 'Unknown',
          apiEndpoint: '',
          rpcEndpoint: '',
          apiKey: () => undefined,
        };
  return {
    ...config,
    apiKey: config.apiKey(),
  } satisfies EtherConfig;
});
